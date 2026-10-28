//! 🟰️ Persisted Layout records compare native floating words directly without value-tree materialization.
use crate::{LayoutRect, LayoutBounds, PageMargins, PageColumns, ParagraphStyle, CharacterStyle, ParentPage, Page, GridSettings, Frame};
use semio_framework_value::{equal_f64, equal_f32};

/// 🪆️ Nullable floating overrides retain presence and exact authored words.
pub fn equal_optional_f64(left: Option<f64>, right: Option<f64>) -> bool { left.map(f64::to_bits) == right.map(f64::to_bits) }

/// 🎨️ Nullable RGBA values retain their native binary32 identities.
pub fn equal_color(left: &Option<[f32; 4]>, right: &Option<[f32; 4]>) -> bool {
    match (left, right) { (Some(left), Some(right)) => left.iter().zip(right).all(|(left, right)| equal_f32(*left, *right)), (None, None) => true, _ => false }
}

impl PartialEq for LayoutRect {
    fn eq(&self, other: &Self) -> bool {
        equal_f64(self.x, other.x)
            && equal_f64(self.y, other.y)
            && equal_f64(self.width, other.width)
            && equal_f64(self.height, other.height)
    }
}

impl PartialEq for LayoutBounds {
    fn eq(&self, other: &Self) -> bool {
        equal_f64(self.x, other.x)
            && equal_f64(self.y, other.y)
            && equal_f64(self.width, other.width)
            && equal_f64(self.height, other.height)
            && equal_f64(self.rotation, other.rotation)
    }
}

impl PartialEq for PageMargins {
    fn eq(&self, other: &Self) -> bool {
        equal_f64(self.top, other.top)
            && equal_f64(self.right, other.right)
            && equal_f64(self.bottom, other.bottom)
            && equal_f64(self.left, other.left)
    }
}

impl PartialEq for PageColumns {
    fn eq(&self, other: &Self) -> bool {
        self.count == other.count
            && equal_f64(self.gutter, other.gutter)
    }
}

impl PartialEq for ParagraphStyle {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.name == other.name
            && self.font_family == other.font_family
            && equal_f64(self.font_size, other.font_size)
            && self.font_weight == other.font_weight
            && equal_f64(self.leading, other.leading)
            && equal_f64(self.tracking, other.tracking)
            && self.alignment == other.alignment
    }
}

impl PartialEq for CharacterStyle {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.name == other.name
            && self.font_family == other.font_family
            && equal_optional_f64(self.font_size, other.font_size)
            && self.font_weight == other.font_weight
            && self.italic == other.italic
            && equal_color(&self.color, &other.color)
            && equal_optional_f64(self.tracking, other.tracking)
    }
}

impl PartialEq for ParentPage {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.name == other.name
            && equal_f64(self.width, other.width)
            && equal_f64(self.height, other.height)
            && self.layer_ids == other.layer_ids
            && self.layers == other.layers
            && self.frames == other.frames
    }
}

impl PartialEq for Page {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.name == other.name
            && self.spread_id == other.spread_id
            && self.parent_page_id == other.parent_page_id
            && equal_f64(self.width, other.width)
            && equal_f64(self.height, other.height)
            && self.margins == other.margins
            && self.columns == other.columns
            && self.guides == other.guides
            && self.layer_ids == other.layer_ids
            && self.layers == other.layers
            && self.frames == other.frames
            && self.overrides == other.overrides
            && self.background_drawing == other.background_drawing
    }
}

impl PartialEq for GridSettings {
    fn eq(&self, other: &Self) -> bool {
        equal_f64(self.baseline_grid, other.baseline_grid)
            && equal_f64(self.baseline_offset, other.baseline_offset)
            && self.snap_to_baseline == other.snap_to_baseline
    }
}

impl PartialEq for Frame {
    fn eq(&self, other: &Self) -> bool {
        if self.id() != other.id() || self.layer_id() != other.layer_id() || self.bounds() != other.bounds() { return false; }
        match (self, other) {
            (Self::Rect { locked: al, visible: av, fill: af, stroke: ast, .. }, Self::Rect { locked: bl, visible: bv, fill: bf, stroke: bst, .. }) => al == bl && av == bv && equal_color(af, bf) && equal_color(ast, bst),
            (Self::Text { locked: al, visible: av, story_id: asid, thread_next: an, columns: ac, inset: ai, wrap_mode: aw, .. }, Self::Text { locked: bl, visible: bv, story_id: bsid, thread_next: bn, columns: bc, inset: bi, wrap_mode: bw, .. }) => al == bl && av == bv && asid == bsid && an == bn && ac == bc && ai == bi && aw == bw,
            (Self::Image { locked: al, visible: av, link_id: aid, .. }, Self::Image { locked: bl, visible: bv, link_id: bid, .. }) => al == bl && av == bv && aid == bid,
            _ => false,
        }
    }
}
