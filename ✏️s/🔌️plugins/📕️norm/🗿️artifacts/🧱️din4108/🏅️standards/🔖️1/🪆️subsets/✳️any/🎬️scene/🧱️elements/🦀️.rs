//! 🧱️ Elements: every envelope element as an upright layer section (a wall assembly whose layers carry their real thicknesses, interior first), every thermal zone as a floor plate and every thermal bridge as a line, with their labels, dimensions and clauses.

use super::kit::*;
use super::layout::{Layout, Pose};
use super::scene::*;
use super::{checks, handles, model, SceneInput};

const PANEL_HEIGHT_RATIO: f64 = 1.0;
const MIN_LAYER: f64 = 1e-3;
const PLATE_THICKNESS: f64 = 0.15;
const BRIDGE_SECTION: f64 = 0.12;
const LABEL_LIFT: f64 = 0.4;
const NO_LAYER_THICKNESS: f64 = 0.02;

/// 🎨️ The material role of a layer: its catalogue id and conductivity decide the colour family.
pub fn role_of(layer: &model::LayerDocument) -> MaterialRole {
    let id = layer.material_id.as_str();
    if id.contains("wood") || id.contains("timber") || id.contains("osb") {
        MaterialRole::Timber
    } else if id.contains("concrete") || id.contains("screed") {
        MaterialRole::Concrete
    } else if id.contains("brick") || id.contains("plaster") || id.contains("render") || id.contains("masonry") || id.contains("gypsum") {
        MaterialRole::Masonry
    } else if id.contains("glass") {
        MaterialRole::Glass
    } else if id.contains("steel") || id.contains("metal") {
        MaterialRole::Steel
    } else {
        MaterialRole::Neutral
    }
}

pub(crate) fn layer_thickness(layer: &model::LayerDocument) -> f64 {
    layer.thickness_m.max(MIN_LAYER)
}

/// 🌡️ The U-value of an element in W/(m²·K), `None` for an element without layers.
pub fn u_value_of(element: &model::EnvelopeElement) -> Option<f64> {
    (!element.layers.is_empty()).then(|| {
        let (r_si, r_se) = checks::surface_resistances_inclined(&element.kind, &element.adjacent, element.inclination_deg);
        checks::u_value(checks::total_resistance(&element.layers, r_si, r_se))
    })
}

fn wall_layers(element: &model::EnvelopeElement) -> Vec<WallLayer> {
    if element.layers.is_empty() {
        return vec![WallLayer { thickness: NO_LAYER_THICKNESS, role: MaterialRole::Neutral, label: loc("No layers", "Keine Schichten") }];
    }
    element.layers.iter().map(|layer| WallLayer { thickness: layer_thickness(layer), role: role_of(layer), label: loc(&layer.material_id, &layer.material_id) }).collect()
}

fn element(input: &SceneInput, layout: &Layout, out: &mut Out, element: &model::EnvelopeElement) {
    let Some(Pose { frame, length }) = layout.pose(&element.id).cloned() else { return };
    let height = length * PANEL_HEIGHT_RATIO;
    let layers = wall_layers(element);
    let total: f64 = layers.iter().map(|l| l.thickness).sum();
    let end = along(&frame.origin, &frame.x, length);
    let binding = binding(input.document, input.report, &[&element.id]);
    let u = u_value_of(element).map_or_else(|| ("n/a".to_owned(), "k. A.".to_owned()), |u| (format!("{u:.3}"), format!("{u:.3}")));
    let name = loc(&format!("{} ({})", element.id, element.kind), &format!("{} ({})", element.id, element.kind));
    out.push(
        Spec {
            kind: EntityKind::Solid,
            id: element.id.clone(),
            collection: "elements",
            geometry: Geometry::Solid(SolidGeometry { shape: SolidShape::WallAssembly { path: Polyline3 { points: vec![frame.origin.clone(), end.clone()] }, height, layers, openings: Vec::new() } }),
            role: MaterialRole::Masonry,
            binding: binding.clone(),
            name: name.clone(),
            detail: loc(&format!("{} layers, {:.0} mm thick, U = {} W/(m²·K), area {:.1} m²", element.layers.len(), total * 1e3, u.0, element.area_m2), &format!("{} Schichten, {:.0} mm dick, U = {} W/(m²·K), Fläche {:.1} m²", element.layers.len(), total * 1e3, u.1, element.area_m2)),
            layer: LAYER_ELEMENTS,
            handles: handles::ids(input.document, layout, &element.id),
        },
        &[frame.origin.clone(), end, along(&along(&frame.origin, &frame.x, length), &frame.z, height)],
    );
    let entity = entity_ref(EntityKind::Solid, &element.id, "elements");
    let label_at = along(&along(&frame.origin, &frame.x, length / 2.0), &frame.z, height + LABEL_LIFT);
    out.annotate(Annotation::Label { id: format!("{}:label", element.id), anchor: label_at.clone(), text: name, tone: tone_of(&binding) }, &label_at);
    let from = along(&frame.origin, &frame.z, -LABEL_LIFT);
    out.annotate(Annotation::Dimension { id: format!("{}:thickness", element.id), from: from.clone(), to: along(&from, &frame.y, total), offset: Vec3::ZERO, value: Quantity::new(QuantityKind::Length, total), binding: None }, &from);
    if let Some(clause) = binding.as_ref().and_then(|b| clause_of(input.report, &b.check_id)) {
        out.annotate(Annotation::Clause { id: format!("{}:clause", element.id), anchor: entity, clause }, &label_at);
    }
}

fn zone(input: &SceneInput, layout: &Layout, out: &mut Out, zone: &model::ThermalZone) {
    let Some(Pose { frame, length }) = layout.pose(&zone.id).cloned() else { return };
    let binding = binding(input.document, input.report, &[&zone.id]);
    let name = loc(&format!("Zone {}", zone.id), &format!("Zone {}", zone.id));
    let windows: f64 = zone.windows.iter().map(|w| w.area_m2).sum();
    out.push(
        Spec {
            kind: EntityKind::Solid,
            id: zone.id.clone(),
            collection: "zones",
            geometry: Geometry::Solid(SolidGeometry { shape: projection_kit::vectors::solid::box_from_corner(&frame, v3(length, length, PLATE_THICKNESS)) }),
            role: MaterialRole::Concrete,
            binding: binding.clone(),
            name: name.clone(),
            detail: loc(&format!("floor area {:.1} m², {} windows of {windows:.1} m², {}", zone.floor_area_m2, zone.windows.len(), zone.heaviness), &format!("Grundfläche {:.1} m², {} Fenster mit {windows:.1} m², {}", zone.floor_area_m2, zone.windows.len(), zone.heaviness)),
            layer: LAYER_ZONES,
            handles: handles::ids(input.document, layout, &zone.id),
        },
        &[frame.origin.clone(), along(&along(&frame.origin, &frame.x, length), &frame.y, length)],
    );
    let at = along(&along(&frame.origin, &frame.x, length / 2.0), &frame.z, PLATE_THICKNESS + LABEL_LIFT);
    out.annotate(Annotation::Label { id: format!("{}:label", zone.id), anchor: at.clone(), text: name, tone: tone_of(&binding) }, &at);
}

fn bridge(input: &SceneInput, layout: &Layout, out: &mut Out, bridge: &model::ThermalBridge) {
    let Some(Pose { frame, length }) = layout.pose(&bridge.id).cloned() else { return };
    let binding = binding(input.document, input.report, &[&bridge.id]);
    let end = along(&frame.origin, &frame.x, length);
    let name = loc(&format!("Bridge {} ({})", bridge.id, bridge.bb2_type), &format!("Wärmebrücke {} ({})", bridge.id, bridge.bb2_type));
    out.push(
        Spec {
            kind: EntityKind::Member,
            id: bridge.id.clone(),
            collection: "thermalBridges",
            geometry: Geometry::Member(MemberGeometry {
                axis: polyline(&[frame.origin.clone(), end.clone()]),
                frame: frame.clone(),
                section: SectionShape::parametric(ProfileKind::Rect, &[("h", BRIDGE_SECTION), ("b", BRIDGE_SECTION)]),
                section_taper: None,
                start_offset: Vec2::ZERO,
                end_offset: Vec2::ZERO,
                start_release: EndRelease::Rigid,
                end_release: EndRelease::Rigid,
                stations: Vec::new(),
            }),
            role: MaterialRole::Steel,
            binding: binding.clone(),
            name: name.clone(),
            detail: loc(&format!("ψ = {:.3} W/(m·K), length {:.1} m", bridge.psi, bridge.length_m), &format!("ψ = {:.3} W/(m·K), Länge {:.1} m", bridge.psi, bridge.length_m)),
            layer: LAYER_BRIDGES,
            handles: handles::ids(input.document, layout, &bridge.id),
        },
        &[frame.origin.clone(), end],
    );
    let at = along(&along(&frame.origin, &frame.x, length / 2.0), &frame.z, LABEL_LIFT);
    out.annotate(Annotation::Label { id: format!("{}:label", bridge.id), anchor: at.clone(), text: name, tone: tone_of(&binding) }, &at);
}

pub(crate) fn project(input: &SceneInput, layout: &Layout, out: &mut Out) {
    let d = input.document;
    for row in &d.elements {
        if out.wants(&row.id) {
            element(input, layout, out, row);
        }
    }
    for row in &d.zones {
        if out.wants(&row.id) {
            zone(input, layout, out, row);
        }
    }
    for row in &d.thermal_bridges {
        if out.wants(&row.id) {
            bridge(input, layout, out, row);
        }
    }
}
