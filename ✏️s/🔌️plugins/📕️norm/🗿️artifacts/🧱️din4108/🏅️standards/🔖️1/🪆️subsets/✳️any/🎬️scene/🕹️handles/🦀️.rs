//! 🕹️ Handles: the drag grips of a selection. Each handle is a one-parameter `s` (the absolute coordinate along its axis) mapped to document leaves; releasing it commits all its writes as one undoable `setFields`.
//!
//! 📏️ A thickness grip on every layer face of an envelope element, and the length and the ψ-value of a thermal bridge.

use super::kit::{along, entity_ref, loc, v3};
use super::layout::Layout;
use super::model::Din4108Snapshot;
use super::scene::*;
use super::scene::projection_kit::handle_kit::{grid, handle, HandleSpec as Spec, field_binding as binding};

pub const PSI_PER_METRE: f64 = 0.1;
pub const MAX_THICKNESS: f64 = 1.0;
pub const MIN_THICKNESS: f64 = 0.001;
pub const MAX_PSI: f64 = 0.4;

fn element_handles(d: &Din4108Snapshot, layout: &Layout, id: &str) -> Vec<Handle> {
    let (Some(element), Some(pose)) = (d.elements.iter().find(|e| e.id == id), layout.pose(id)) else { return Vec::new() };
    let entity = entity_ref(EntityKind::Solid, id, "elements");
    let f = &pose.frame;
    let mut face = 0.0;
    element
        .layers
        .iter()
        .map(|layer| {
            let before = face;
            face += super::elements::layer_thickness(layer);
            handle(Spec {
                entity: &entity,
                tag: &format!("thickness:{}", layer.id),
                kind: HandleKind::ScalarAlong { axis: f.y.clone() },
                anchor: along(&along(&f.origin, &f.x, pose.length / 2.0), &f.y, face),
                grip: GripStyle::Arrow,
                writes: vec![binding(format!("elements[id={id}].layers[id={}].thicknessM", layer.id), Affine { base: -before - f.origin.y, k: 1.0 }, "m", Some(Quantise::Round { step: 0.001 }))],
                snap: grid(0.005),
                limits: (before + f.origin.y + MIN_THICKNESS, before + f.origin.y + MAX_THICKNESS),
                name: loc(&format!("Thickness of layer {} of {id}", layer.id), &format!("Dicke der Schicht {} von {id}", layer.id)),
            })
        })
        .collect()
}

fn bridge_handles(d: &Din4108Snapshot, layout: &Layout, id: &str) -> Vec<Handle> {
    let (Some(bridge), Some(pose)) = (d.thermal_bridges.iter().find(|b| b.id == id), layout.pose(id)) else { return Vec::new() };
    let entity = entity_ref(EntityKind::Member, id, "thermalBridges");
    let f = &pose.frame;
    vec![
        handle(Spec {
            entity: &entity,
            tag: "length",
            kind: HandleKind::ScalarAlong { axis: f.x.clone() },
            anchor: along(&f.origin, &f.x, pose.length),
            grip: GripStyle::Arrow,
            writes: vec![binding(format!("thermalBridges[id={id}].lengthM"), Affine { base: -f.origin.x, k: 1.0 }, "m", Some(Quantise::Round { step: 0.01 }))],
            snap: grid(0.05),
            limits: (f.origin.x + super::layout::MIN_SIDE, f.origin.x + super::layout::MAX_SIDE),
            name: loc(&format!("Length of {id}"), &format!("Länge von {id}")),
        }),
        handle(Spec {
            entity: &entity,
            tag: "psi",
            kind: HandleKind::ScalarAlong { axis: Vec3::Z },
            anchor: v3(f.origin.x + pose.length / 2.0, f.origin.y, bridge.psi / PSI_PER_METRE),
            grip: GripStyle::Diamond,
            writes: vec![binding(format!("thermalBridges[id={id}].psi"), Affine { base: 0.0, k: PSI_PER_METRE }, "W/(m·K)", Some(Quantise::Round { step: 0.001 }))],
            snap: grid(0.01),
            limits: (0.0, MAX_PSI / PSI_PER_METRE),
            name: loc(&format!("Linear transmittance ψ of {id}"), &format!("Längenbezogener Wärmedurchgangskoeffizient ψ von {id}")),
        }),
    ]
}

fn build(d: &Din4108Snapshot, layout: &Layout, id: &str) -> Vec<Handle> {
    let mut out = element_handles(d, layout, id);
    out.extend(bridge_handles(d, layout, id));
    out
}

pub(crate) fn ids(document: &Din4108Snapshot, layout: &Layout, id: &str) -> Vec<String> {
    build(document, layout, id).into_iter().map(|h| h.id).collect()
}

/// 🕹️ The handles of every selected entity, in selection order, without duplicates.
pub fn handles(document: &Din4108Snapshot, selection: &[EntityRef]) -> Vec<Handle> {
    let layout = Layout::of(document);
    let mut out: Vec<Handle> = Vec::new();
    for entity in selection {
        for handle in build(document, &layout, &entity.id) {
            if !out.iter().any(|h| h.id == handle.id) {
                out.push(handle);
            }
        }
    }
    out
}
