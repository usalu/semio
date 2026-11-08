//! 🕹️ Handles: the drag grips of a selection. Each handle is a one-parameter `s` (the absolute coordinate along its axis) mapped to document leaves; releasing it commits all its writes as one undoable `setFields`.
//!
//! 📏️ Three face grips (width, depth, height) on every parametric geometry and on every product that references one: they write the maximum corner of the geometry bounding box, so product and geometry grow together. A radiator without geometry gets the same grips on its length, depth and height attributes; the mutation resolver rewrites the native 210 record with the attribute.

use super::kit::{entity_ref, loc, v3, GEOMETRY, PRODUCTS};
use super::layout::{Layout, MAX_EDGE, MIN_EDGE};
use super::model::{SheetAttributes, Vdi3805Snapshot};
use super::scene::*;
use super::scene::projection_kit::handle_kit::{grid, handle, HandleSpec as Spec, field_binding as binding};

struct Face {
    tag: &'static str,
    leaf: &'static str,
    attribute: &'static str,
    axis: Vec3,
    en: &'static str,
    de: &'static str,
}

const FACES: [Face; 3] = [
    Face { tag: "width", leaf: "maxX", attribute: "lengthM", axis: Vec3::X, en: "Width", de: "Breite" },
    Face { tag: "depth", leaf: "maxY", attribute: "depthM", axis: Vec3::Y, en: "Depth", de: "Tiefe" },
    Face { tag: "height", leaf: "maxZ", attribute: "heightM", axis: Vec3::Z, en: "Height", de: "Höhe" },
];

/// 🧲️ What a set of face grips writes: the bounding box of a parametric geometry, or the extent attributes of a radiator without geometry.
struct Target {
    entity: EntityRef,
    path: Box<dyn Fn(&Face) -> String>,
    low: [f64; 3],
    scale: f64,
}

fn target_of(d: &Vdi3805Snapshot, id: &str) -> Option<Target> {
    if let Some(row) = d.geometry.get(id) {
        let key = id.to_owned();
        return Some(Target { entity: entity_ref(EntityKind::Solid, id, GEOMETRY), path: Box::new(move |face| format!("geometry[id={key}].bbox.{}", face.leaf)), low: [row.bbox.min_x, row.bbox.min_y, row.bbox.min_z], scale: row.parameters.get("scale").copied().unwrap_or(1.0) });
    }
    let product = d.catalog.products.iter().find(|p| p.id == id)?;
    if let Some(row) = product.configuration.geometry_ref.as_deref().and_then(|key| d.geometry.get(key).map(|row| (key, row))) {
        let key = row.0.to_owned();
        return Some(Target { entity: entity_ref(EntityKind::Solid, id, PRODUCTS), path: Box::new(move |face| format!("geometry[id={key}].bbox.{}", face.leaf)), low: [row.1.bbox.min_x, row.1.bbox.min_y, row.1.bbox.min_z], scale: row.1.parameters.get("scale").copied().unwrap_or(1.0) });
    }
    matches!(product.configuration.attributes, SheetAttributes::Radiator(_)).then(|| {
        let id = id.to_owned();
        Target { entity: entity_ref(EntityKind::Solid, &id, PRODUCTS), path: Box::new(move |face| format!("catalog.products[id={id}].configuration.attributes.{}", face.attribute)), low: [0.0; 3], scale: 1.0 }
    })
}

fn build(d: &Vdi3805Snapshot, layout: &Layout, id: &str) -> Vec<Handle> {
    let Some(pose) = layout.pose(id) else { return Vec::new() };
    let mut grips = face_handles(d, pose, id);
    grips.extend(attribute_handles(d, pose, id));
    grips
}

fn face_handles(d: &Vdi3805Snapshot, pose: &super::layout::Pose, id: &str) -> Vec<Handle> {
    let Some(target) = target_of(d, id) else { return Vec::new() };
    if !(target.scale.is_finite() && target.scale > 0.0) {
        return Vec::new();
    }
    let (origin, size) = (&pose.min, &pose.size);
    let extent = [size.x, size.y, size.z];
    let start = [origin.x, origin.y, origin.z];
    FACES
        .iter()
        .enumerate()
        .map(|(i, face)| {
            let mut anchor = [origin.x + size.x / 2.0, origin.y + size.y / 2.0, size.z / 2.0];
            anchor[i] = start[i] + extent[i];
            handle(Spec {
                entity: &target.entity,
                tag: face.tag,
                kind: HandleKind::ScalarAlong { axis: face.axis.clone() },
                anchor: v3(anchor[0], anchor[1], anchor[2]),
                grip: GripStyle::Arrow,
                writes: vec![binding((target.path)(face), Affine { base: target.low[i] - start[i] / target.scale, k: 1.0 / target.scale }, "m", Some(Quantise::Round { step: 0.001 }))],
                snap: grid(0.005),
                limits: (start[i] + MIN_EDGE, start[i] + MAX_EDGE),
                name: loc(&format!("{} of {id}", face.en), &format!("{} von {id}", face.de)),
            })
        })
        .collect()
}

/// 📏️ The drawn metres per millimetre of nominal diameter of a valve body (width) and of a pump body (width), the metres of grip travel per metre of pump head and per kilowatt of generator output (see `layout::product_size`).
pub const VALVE_M_PER_DN: f64 = 0.003;
pub const PUMP_M_PER_DN: f64 = 0.004;
pub const HEAD_M_PER_M: f64 = 0.02;
pub const MAX_HEAD_M: f64 = 100.0;
pub const OUTPUT_M_PER_KW: f64 = 0.002;
pub const MAX_OUTPUT_KW: f64 = 1000.0;

/// 🎛️ The attribute grips of a product drawn from its sheet attributes: the nominal diameter of a valve and of a pump on the width face, the head of a pump and the output of a heat generator as value grips above the body.
fn attribute_handles(d: &Vdi3805Snapshot, pose: &super::layout::Pose, id: &str) -> Vec<Handle> {
    let Some(product) = d.catalog.products.iter().find(|p| p.id == id) else { return Vec::new() };
    if product.configuration.geometry_ref.as_deref().is_some_and(|key| d.geometry.contains_key(key)) {
        return Vec::new();
    }
    let entity = entity_ref(EntityKind::Solid, id, PRODUCTS);
    let path = |leaf: &str| format!("catalog.products[id={id}].configuration.attributes.{leaf}");
    let (min, size) = (&pose.min, &pose.size);
    let mid = v3(min.x + size.x / 2.0, min.y + size.y / 2.0, size.z / 2.0);
    let top = v3(min.x + size.x / 2.0, min.y + size.y / 2.0, size.z);
    let diameter = |tag: &str, leaf: &str, per_dn: f64, en: &str, de: &str| {
        handle(Spec {
            entity: &entity,
            tag,
            kind: HandleKind::ScalarAlong { axis: Vec3::X },
            anchor: v3(min.x + size.x, mid.y, mid.z),
            grip: GripStyle::Arrow,
            writes: vec![binding(path(leaf), Affine { base: -min.x / per_dn, k: 1.0 / per_dn }, "mm", Some(Quantise::Int))],
            snap: grid(per_dn),
            limits: (min.x + 10.0 * per_dn, min.x + 300.0 * per_dn),
            name: loc(&format!("{en} of {id}"), &format!("{de} von {id}")),
        })
    };
    let value = |tag: &str, leaf: &str, anchor_z: f64, k: f64, travel: f64, unit: &str, step: f64, en: &str, de: &str| {
        handle(Spec {
            entity: &entity,
            tag,
            kind: HandleKind::ScalarAlong { axis: Vec3::Z },
            anchor: v3(top.x, top.y, anchor_z),
            grip: GripStyle::Diamond,
            writes: vec![binding(path(leaf), Affine { base: -size.z * k, k }, unit, Some(Quantise::Round { step }))],
            snap: grid(0.005),
            limits: (size.z, size.z + travel),
            name: loc(&format!("{en} of {id}"), &format!("{de} von {id}")),
        })
    };
    match &product.configuration.attributes {
        SheetAttributes::ValveHeating(_) => vec![diameter("dn", "dn", VALVE_M_PER_DN, "Nominal diameter", "Nennweite")],
        SheetAttributes::PumpHeating(a) => vec![
            diameter("dn", "dnDischarge", PUMP_M_PER_DN, "Nominal diameter", "Nennweite"),
            value("head", "nominalHeadM", size.z + a.nominal_head_m.clamp(0.0, MAX_HEAD_M) * HEAD_M_PER_M, 1.0 / HEAD_M_PER_M, MAX_HEAD_M * HEAD_M_PER_M, "m", 0.1, "Nominal head", "Nennförderhöhe"),
        ],
        SheetAttributes::HeatGenerator(a) => vec![value("output", "nominalHeatOutputW", size.z + (a.nominal_heat_output_w / 1000.0).clamp(0.0, MAX_OUTPUT_KW) * OUTPUT_M_PER_KW, 1000.0 / OUTPUT_M_PER_KW, MAX_OUTPUT_KW * OUTPUT_M_PER_KW, "W", 100.0, "Nominal heat output", "Nennwärmeleistung")],
        _ => Vec::new(),
    }
}

pub(crate) fn ids(document: &Vdi3805Snapshot, layout: &Layout, id: &str) -> Vec<String> {
    build(document, layout, id).into_iter().map(|h| h.id).collect()
}

/// 🕹️ The handles of every selected entity, in selection order, without duplicates.
pub fn handles(document: &Vdi3805Snapshot, selection: &[EntityRef]) -> Vec<Handle> {
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
