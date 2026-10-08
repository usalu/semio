//! 📐️ The sheet: the storey plans stacked top-down in one column (per building, the highest level first), each in a slot with a title band and a padded drawing area, and the
//! fitted size of the whole sheet in paper millimetres.
//! 📎 https://www.w3.org/TR/SVG11/coords.html#ViewBoxAttribute

use super::path::{Frame, MM_PER_METRE};
use crate::standards::v1::subsets::any::schema::inferences::plan_linework::PlanLinework;
use crate::ModelSnapshot;
use std::collections::BTreeMap;

/// 📏️ Space around the whole sheet.
pub const MARGIN: f64 = 10.0;
/// 📏️ Space around the drawing inside a slot (room for tags that overhang the bounds).
pub const PADDING: f64 = 8.0;
/// 📏️ Height of the title band above a drawing.
pub const TITLE_BAND: f64 = 8.0;
/// 📏️ Space between two slots.
pub const GAP: f64 = 6.0;

/// 🧩️ The place of one storey plan on the sheet.
#[derive(Clone, Debug, PartialEq)]
pub struct Slot {
    pub storey: String,
    pub building: String,
    pub name: String,
    pub level: i32,
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

/// 📐️ The layout of the plans of `snapshot`: only storeys that have a plan get a slot.
pub fn layout(snapshot: &ModelSnapshot, plans: &BTreeMap<String, PlanLinework>) -> Layout {
    let mut order: Vec<(&String, &crate::Storey)> = snapshot.storeys.iter().filter(|(id, _)| plans.contains_key(*id)).collect();
    order.sort_by(|a, b| (&a.1.building, std::cmp::Reverse(a.1.level), a.0).cmp(&(&b.1.building, std::cmp::Reverse(b.1.level), b.0)));
    let (mut slots, mut y, mut widest) = (Vec::new(), MARGIN, 0.0_f64);
    for (id, storey) in order {
        let plan = &plans[id];
        let (width, height) = ((plan.bounds.max_x - plan.bounds.min_x) * MM_PER_METRE + 2.0 * PADDING, (plan.bounds.max_y - plan.bounds.min_y) * MM_PER_METRE + 2.0 * PADDING + TITLE_BAND);
        let frame = Frame { min_x: plan.bounds.min_x, max_y: plan.bounds.max_y, left: PADDING, top: TITLE_BAND + PADDING };
        slots.push(Slot { storey: id.clone(), building: storey.building.clone(), name: storey.name.clone(), level: storey.level, x: MARGIN, y, width, height, frame });
        y += height + GAP;
        widest = widest.max(width);
    }
    let content = if slots.is_empty() { 0.0 } else { y - GAP - MARGIN };
    Layout { slots, width: 2.0 * MARGIN + widest, height: 2.0 * MARGIN + content }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
