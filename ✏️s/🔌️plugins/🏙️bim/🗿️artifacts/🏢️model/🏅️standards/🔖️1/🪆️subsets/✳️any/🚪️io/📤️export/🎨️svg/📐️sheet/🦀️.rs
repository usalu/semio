//! 📐️ The sheet: the drawn views stacked top-down in one column (per building the plans from the highest level down, then the ceiling plans, the sections and the elevations), each in a slot with a title band and a padded drawing area at the
//! scale of its view, and the fitted size of the whole sheet in paper millimetres. A camera view draws nothing and gets no slot.
//! 📎 https://www.w3.org/TR/SVG11/coords.html#ViewBoxAttribute

use super::path::{mm_per_metre, Frame};
use crate::standards::v1::subsets::any::schema::inferences::view_linework::ViewLinework;
use crate::{ModelSnapshot, ViewKind};
use std::collections::BTreeMap;

/// 📏️ Space around the whole sheet.
pub const MARGIN: f64 = 10.0;
/// 📏️ Space around the drawing inside a slot (room for tags that overhang the bounds).
pub const PADDING: f64 = 8.0;
/// 📏️ Height of the title band above a drawing.
pub const TITLE_BAND: f64 = 8.0;
/// 📏️ Space between two slots.
pub const GAP: f64 = 6.0;

/// 🧩️ The place of one view on the sheet.
#[derive(Clone, Debug, PartialEq)]
pub struct Slot {
    pub view: String,
    pub building: String,
    pub name: String,
    pub kind: ViewKind,
    pub storey: Option<String>,
    pub scale: u32,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub frame: Frame,
}

/// 📐️ The slots and the fitted size of the sheet.
#[derive(Clone, Debug, PartialEq)]
pub struct Layout {
    pub slots: Vec<Slot>,
    pub width: f64,
    pub height: f64,
}

/// 🖼️ Whether a kind of view is drawn on paper: the plans, sections and elevations are, the cameras are rendered by the 3D window.
pub fn is_drawn(kind: ViewKind) -> bool {
    matches!(kind, ViewKind::Plan | ViewKind::CeilingPlan | ViewKind::Section | ViewKind::Elevation)
}

fn rank(kind: ViewKind) -> u8 {
    match kind {
        ViewKind::Plan => 0,
        ViewKind::CeilingPlan => 1,
        ViewKind::Section => 2,
        _ => 3,
    }
}

/// 📐️ The layout of the views of `snapshot` that have a drawing: plans highest level first, then ceiling plans, sections and elevations, each by name and id.
pub fn layout(snapshot: &ModelSnapshot, drawings: &BTreeMap<String, ViewLinework>) -> Layout {
    let level = |id: &str| snapshot.views[id].storey.as_deref().and_then(|storey| snapshot.storeys.get(storey)).map_or(0, |storey| storey.level);
    let mut order: Vec<(&String, &crate::View)> = snapshot.views.iter().filter(|(id, view)| is_drawn(view.kind) && drawings.contains_key(*id)).collect();
    order.sort_by(|a, b| (&a.1.building, rank(a.1.kind), std::cmp::Reverse(level(a.0)), &a.1.name, a.0).cmp(&(&b.1.building, rank(b.1.kind), std::cmp::Reverse(level(b.0)), &b.1.name, b.0)));
    let (mut slots, mut y, mut widest) = (Vec::new(), MARGIN, 0.0_f64);
    for (id, view) in order {
        let bounds = drawings[id].lines.bounds;
        let mm = mm_per_metre(view.scale);
        let (width, height) = ((bounds.max_x - bounds.min_x) * mm + 2.0 * PADDING, (bounds.max_y - bounds.min_y) * mm + 2.0 * PADDING + TITLE_BAND);
        let frame = Frame { min_x: bounds.min_x, max_y: bounds.max_y, left: PADDING, top: TITLE_BAND + PADDING, mm };
        slots.push(Slot { view: id.clone(), building: view.building.clone(), name: view.name.clone(), kind: view.kind, storey: view.storey.clone(), scale: view.scale, x: MARGIN, y, width, height, frame });
        y += height + GAP;
        widest = widest.max(width);
    }
    let content = if slots.is_empty() { 0.0 } else { y - GAP - MARGIN };
    Layout { slots, width: 2.0 * MARGIN + widest, height: 2.0 * MARGIN + content }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
