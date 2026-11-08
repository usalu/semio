//! 🧱️ Zones and envelope: every thermal zone as a translucent box and every envelope element as a panel on its face, coloured by the utilisation of the energy checks that name it.

use super::kit::{binding, loc, Out, Spec, LAYER_GLAZING, LAYER_OPAQUE, LAYER_ZONES};
use super::layout::{Layout, Panel, ZoneBox};
use super::model::{ElementKind, EnvelopeElement, ThermalZone};
use super::scene::{Frame, Geometry, MaterialRole, SolidGeometry, SolidShape, Vec3};
use super::SceneInput;

const CHECKS: &[&str] = &["din18599."];

fn solid(frame: Frame, size: Vec3) -> Geometry {
    Geometry::Solid(SolidGeometry { shape: SolidShape::Box { frame, size } })
}

fn corners(frame: &Frame, size: &Vec3) -> Vec<Vec3> {
    let at = |s: f64, y: f64, z: f64| super::kit::projection_kit::at(frame, s, y, z);
    vec![at(-size.x / 2.0, -size.y / 2.0, 0.0), at(size.x / 2.0, size.y / 2.0, size.z)]
}

fn zone_spec(input: &SceneInput, zone: &ThermalZone, placed: &ZoneBox) -> (Spec, Vec<Vec3>) {
    let frame = Frame { origin: placed.centre.clone(), ..Frame::WORLD };
    let size = super::kit::v3(placed.side, placed.side, placed.height);
    let spec = Spec::new(zone.id.clone(), "zones", solid(frame.clone(), size.clone()), MaterialRole::Neutral, loc(&zone.label_en, &zone.label_de), LAYER_ZONES)
        .bound(binding(input.report, &zone.id, CHECKS))
        .detailed(loc(&format!("Thermal zone {}: {:.0} m², {:.0} m³, heating setpoint {:.0} °C.", zone.label_en, zone.area_m2, zone.volume_m3, zone.theta_i_heat_c), &format!("Thermische Zone {}: {:.0} m², {:.0} m³, Heizsollwert {:.0} °C.", zone.label_de, zone.area_m2, zone.volume_m3, zone.theta_i_heat_c)))
        .with_handles(vec![format!("{}:height", zone.id)])
        .translucent(0.22);
    (spec, corners(&frame, &size))
}

fn role_of(kind: ElementKind) -> MaterialRole {
    match kind {
        ElementKind::Wall => MaterialRole::Masonry,
        ElementKind::Roof | ElementKind::Floor => MaterialRole::Concrete,
        ElementKind::Window => MaterialRole::Glass,
        ElementKind::Door => MaterialRole::Timber,
    }
}

fn kind_words(kind: ElementKind) -> (&'static str, &'static str) {
    match kind {
        ElementKind::Wall => ("wall", "Wand"),
        ElementKind::Roof => ("roof", "Dach"),
        ElementKind::Floor => ("floor", "Boden"),
        ElementKind::Window => ("window", "Fenster"),
        ElementKind::Door => ("door", "Tür"),
    }
}

fn element_spec(input: &SceneInput, element: &EnvelopeElement, panel: &Panel) -> (Spec, Vec<Vec3>) {
    let (en, de) = kind_words(element.kind);
    let glazing = matches!(element.kind, ElementKind::Window | ElementKind::Door);
    let spec = Spec::new(element.id.clone(), "elements", solid(panel.frame.clone(), panel.size.clone()), role_of(element.kind), loc(&element.label_en, &element.label_de), if glazing { LAYER_GLAZING } else { LAYER_OPAQUE })
        .bound(binding(input.report, &element.id, CHECKS))
        .detailed(loc(&format!("Envelope {en} {}: {:.1} m², U = {:.2} W/(m²·K).", element.label_en, element.area_m2, element.u_value_w_m2k), &format!("Hüllbauteil {de} {}: {:.1} m², U = {:.2} W/(m²·K).", element.label_de, element.area_m2, element.u_value_w_m2k)))
        .with_handles(vec![format!("{}:u-value", element.id)]);
    (spec, corners(&panel.frame, &panel.size))
}

/// 🧊️ Pushes the zone boxes, then the envelope panels.
pub fn project(input: &SceneInput, layout: &Layout, out: &mut Out) {
    for zone in &input.document.zones {
        if let Some(placed) = layout.zone(&zone.id) {
            let (spec, points) = zone_spec(input, zone, placed);
            out.push(spec, &points);
        }
    }
    for element in &input.document.elements {
        if let Some(panel) = layout.panel(&element.id) {
            let (spec, points) = element_spec(input, element, panel);
            out.push(spec, &points);
        }
    }
}
