use crate::Rect;
use skia_bindings as sb;
use std::{
    cmp::{max, min},
    ops::Range,
};

pub use sb::skia_textlayout_Affinity as Affinity;
variant_name!(Affinity::Downstream);
pub use sb::skia_textlayout_RectHeightStyle as RectHeightStyle;
variant_name!(RectHeightStyle::IncludeLineSpacingBottom);
pub use sb::skia_textlayout_RectWidthStyle as RectWidthStyle;
variant_name!(RectWidthStyle::Max);
pub use sb::skia_textlayout_TextAlign as TextAlign;
variant_name!(TextAlign::End);
pub use sb::skia_textlayout_TextDirection as TextDirection;
variant_name!(TextDirection::LTR);

pub use sb::skia_textlayout_PositionWithAffinity as PositionWithAffinity;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct TextBox {
    pub rect: Rect,
    pub direct: TextDirection,
}

native_transmutable!(sb::skia_textlayout_TextBox, TextBox, text_box_layout);

pub const EMPTY_INDEX: usize = std::usize::MAX;

pub trait RangeExtensions {
    fn width(&self) -> usize;
    fn shift(&mut self, d: isize);
    fn contains(&self, other: &Self) -> bool;
    fn intersects(&self, other: &Self) -> bool;
    #[must_use]
    fn intersection(&self, other: &Self) -> Self;
    fn empty(&self) -> bool;
}

impl RangeExtensions for Range<usize> {
    fn width(&self) -> usize {
        self.end - self.start
    }

    fn shift(&mut self, d: isize) {
        if d >= 0 {
            let u = d as usize;
            self.start += u;
            self.end += u;
        } else {
            let u = -d as usize;
            self.start -= u;
            self.end -= u;
        }
    }

    fn contains(&self, other: &Self) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn intersects(&self, other: &Self) -> bool {
        max(self.start, other.start) <= min(self.end, other.end)
    }

    fn intersection(&self, other: &Self) -> Self {
        Self {
            start: max(self.start, other.start),
            end: min(self.end, other.end),
        }
    }

    fn empty(&self) -> bool {
        self.start == EMPTY_INDEX && self.end == EMPTY_INDEX
    }
}

#[allow(clippy::reversed_empty_ranges)]
pub const EMPTY_RANGE: Range<usize> = Range {
    start: EMPTY_INDEX,
    end: EMPTY_INDEX,
};

pub use sb::skia_textlayout_TextBaseline as TextBaseline;
variant_name!(TextBaseline::Alphabetic);

pub use sb::skia_textlayout_TextHeightBehavior as TextHeightBehavior;
variant_name!(TextHeightBehavior::DisableFirstAscent);

// m84: LineMetricStyle is declared but not used in the public API yet.
