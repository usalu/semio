//! 🕹️ Handles: the drag grips of a selection. Each handle is a one-parameter `s` mapped to document leaves; releasing it commits all its writes as one undoable `setFields`.
//!
//! 🧱️ Envelope elements: the U-value, dragged along the outward normal of the panel (`U_SCALE_M` metres per W/(m²·K)). 🏠️ Zones: the storey height, dragged along `+Z`, which writes the zone volume `A · h`.

use super::kit::{entity_ref, loc};
use super::layout::Layout;
use super::model::Din18599Snapshot;
use super::scene::projection_kit::handle_kit::{grid, handle, round, HandleSpec as Spec};
use super::scene::*;

pub const U_SCALE_M: f64 = 0.4;
pub const U_MAX: f64 = 3.5;
pub const U_UNIT: &str = "W/(m²·K)";

fn dot(a: &Vec3, b: &Vec3) -> f64 {
    a.x * b.x + a.y * b.y + a.z * b.z
}

fn element_handles(document: &Din18599Snapshot, layout: &Layout, id: &str) -> Vec<Handle> {
    let (Some(element), Some(panel)) = (document.elements.iter().find(|e| e.id == id), layout.panel(id)) else { return Vec::new() };
    let entity = entity_ref(EntityKind::Solid, id, "elements");
    let n = &panel.normal;
    let half = if n.z.abs() > 0.5 { panel.size.z } else { panel.size.x } / 2.0;
    let rest = dot(&panel.centre, n) + half;
    let reach = half + element.u_value_w_m2k * U_SCALE_M;
    let anchor = Vec3::new(panel.centre.x + n.x * reach, panel.centre.y + n.y * reach, panel.centre.z + n.z * reach);
    vec![handle(Spec {
        entity: &entity,
        tag: "u-value",
        kind: HandleKind::ScalarAlong { axis: n.clone() },
        anchor,
        grip: GripStyle::Arrow,
        writes: vec![FieldBinding { path: format!("elements[id={id}].uValueWM2k"), map: Affine { base: -rest / U_SCALE_M, k: 1.0 / U_SCALE_M }, unit: U_UNIT.into(), quantise: round(0.01) }],
        snap: grid(0.01 * U_SCALE_M),
        limits: (rest, rest + U_MAX * U_SCALE_M),
        name: loc(&format!("U-value of {}", element.label_en), &format!("U-Wert von {}", element.label_de)),
    })]
}

fn zone_handles(document: &Din18599Snapshot, layout: &Layout, id: &str) -> Vec<Handle> {
    let (Some(zone), Some(placed)) = (document.zones.iter().find(|z| z.id == id), layout.zone(id)) else { return Vec::new() };
    let entity = entity_ref(EntityKind::Solid, id, "zones");
    vec![handle(Spec {
        entity: &entity,
        tag: "height",
        kind: HandleKind::ScalarAlong { axis: Vec3::Z },
        anchor: Vec3::new(placed.centre.x, placed.centre.y, placed.height),
        grip: GripStyle::Arrow,
        writes: vec![FieldBinding { path: format!("zones[id={id}].volumeM3"), map: Affine { base: 0.0, k: zone.area_m2 }, unit: "m³".into(), quantise: round(0.1) }],
        snap: grid(0.05),
        limits: (super::layout::MIN_HEIGHT, super::layout::MAX_HEIGHT),
        name: loc(&format!("Storey height of {}", zone.label_en), &format!("Geschosshöhe von {}", zone.label_de)),
    })]
}

/// 🕹️ The drag grips of the selected entities of `document`.
pub fn handles(document: &Din18599Snapshot, selection: &[EntityRef]) -> Vec<Handle> {
    let layout = Layout::of(document);
    selection.iter().flat_map(|e| zone_handles(document, &layout, &e.id).into_iter().chain(element_handles(document, &layout, &e.id))).collect()
}
