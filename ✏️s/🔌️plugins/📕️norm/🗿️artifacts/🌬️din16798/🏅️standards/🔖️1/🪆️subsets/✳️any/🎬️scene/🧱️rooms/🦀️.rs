//! 🧱️ Rooms: every zone as a translucent room box coloured by its comfort category (I–IV) and every ventilation system as an air-handling box, with their labels, dimensions and clauses.

use super::kit::*;
use super::layout::{Layout, Pose, ROOM_DEPTH, ROOM_HEIGHT, VENT_DEPTH, VENT_HEIGHT};
use super::scene::*;
use super::{checks, handles, model, SceneInput};

const ROOM_OPACITY: f64 = 0.55;
const LABEL_LIFT: f64 = 0.5;

/// 🏷️ The comfort category of a zone as its number 1 to 4.
pub fn category_number(zone: &model::ZoneDocument) -> u8 {
    match checks::parse_category(&zone.comfort_category) {
        checks::ComfortCategory::I => 1,
        checks::ComfortCategory::II => 2,
        checks::ComfortCategory::III => 3,
        checks::ComfortCategory::IV => 4,
    }
}

/// 🔢️ The Roman numeral of a category number.
pub fn category_name(category: u8) -> &'static str {
    ["I", "II", "III", "IV"][usize::from(category.clamp(1, 4)) - 1]
}

fn zone(input: &SceneInput, layout: &Layout, out: &mut Out, zone: &model::ZoneDocument) {
    let Some(Pose { frame, length }) = layout.pose(&zone.id).cloned() else { return };
    let category = category_number(zone);
    let binding = binding(input.document, input.report, &[&zone.id]);
    let name = loc(&format!("Room «{}» for category {}", zone.name, category_name(category)), &format!("Raum «{}» für Kategorie {}", zone.name, category_name(category)));
    let corner_far = along(&along(&frame.origin, &frame.x, length), &frame.y, ROOM_DEPTH);
    out.push(
        Spec {
            kind: EntityKind::Solid,
            id: zone.id.clone(),
            collection: "zones",
            geometry: Geometry::Solid(SolidGeometry { shape: projection_kit::vectors::solid::box_from_corner(&frame, v3(length, ROOM_DEPTH, ROOM_HEIGHT)) }),
            role: category_role(category),
            binding: binding.clone(),
            name: name.clone(),
            detail: loc(
                &format!("{} m², {} occupants, θ_op {:.1} °C summer and {:.1} °C winter, {:.0} ppm CO₂, {:.0} % relative humidity", zone.floor_area_m2, zone.occupants, zone.t_op_summer_c, zone.t_op_winter_c, zone.co2_ppm, zone.rh_percent),
                &format!("{} m², {} Personen, θ_op {:.1} °C im Sommer und {:.1} °C im Winter, {:.0} ppm CO₂, {:.0} % relative Feuchte", zone.floor_area_m2, zone.occupants, zone.t_op_summer_c, zone.t_op_winter_c, zone.co2_ppm, zone.rh_percent),
            ),
            layer: LAYER_ROOMS,
            handles: handles::ids(input.document, layout, &zone.id),
            opacity: ROOM_OPACITY,
        },
        &[frame.origin.clone(), corner_far, along(&along(&frame.origin, &frame.x, length), &frame.z, ROOM_HEIGHT)],
    );
    let entity = entity_ref(EntityKind::Solid, &zone.id, "zones");
    let label_at = along(&along(&along(&frame.origin, &frame.x, length / 2.0), &frame.y, ROOM_DEPTH / 2.0), &frame.z, ROOM_HEIGHT + LABEL_LIFT);
    out.annotate(Annotation::Label { id: format!("{}:label", zone.id), anchor: label_at.clone(), text: name, tone: tone_of(&binding) }, &label_at);
    let from = along(&frame.origin, &frame.y, -LABEL_LIFT);
    out.annotate(Annotation::Dimension { id: format!("{}:width", zone.id), from: from.clone(), to: along(&from, &frame.x, length), offset: Vec3::ZERO, value: Quantity::new(QuantityKind::Length, length), binding: None }, &from);
    if let Some(clause) = binding.as_ref().and_then(|b| clause_of(input.report, &b.check_id)) {
        out.annotate(Annotation::Clause { id: format!("{}:clause", zone.id), anchor: entity, clause }, &label_at);
    }
}

fn vent(input: &SceneInput, layout: &Layout, out: &mut Out, vent: &model::VentSystemDocument) {
    let Some(Pose { frame, length }) = layout.pose(&vent.id).cloned() else { return };
    let binding = binding(input.document, input.report, &[&vent.id]);
    let served = input.document.zones.iter().filter(|z| z.vent_system_id == vent.id).count();
    let name = loc(&format!("Ventilation system «{}» ({})", vent.name, vent.system_type), &format!("Lüftungsanlage «{}» ({})", vent.name, vent.system_type));
    let far = along(&along(&frame.origin, &frame.x, length), &frame.y, VENT_DEPTH);
    out.push(
        Spec {
            kind: EntityKind::Solid,
            id: vent.id.clone(),
            collection: "ventSystems",
            geometry: Geometry::Solid(SolidGeometry { shape: projection_kit::vectors::solid::box_from_corner(&frame, v3(length, VENT_DEPTH, VENT_HEIGHT)) }),
            role: MaterialRole::Steel,
            binding: binding.clone(),
            name: name.clone(),
            detail: loc(
                &format!("design airflow {:.0} m³/h serving {served} rooms, SFP {:.0} W/(m³/s), heat recovery {:.0} %", vent.design_airflow_m3_h, vent.sfp_w_m3_s, vent.heat_recovery_eta * 100.0),
                &format!("Auslegungsvolumenstrom {:.0} m³/h für {served} Räume, SFP {:.0} W/(m³/s), Wärmerückgewinnung {:.0} %", vent.design_airflow_m3_h, vent.sfp_w_m3_s, vent.heat_recovery_eta * 100.0),
            ),
            layer: LAYER_VENTILATION,
            handles: handles::ids(input.document, layout, &vent.id),
            opacity: 1.0,
        },
        &[frame.origin.clone(), far, along(&along(&frame.origin, &frame.x, length), &frame.z, VENT_HEIGHT)],
    );
    let at = along(&along(&frame.origin, &frame.x, length / 2.0), &frame.z, VENT_HEIGHT + LABEL_LIFT);
    out.annotate(Annotation::Label { id: format!("{}:label", vent.id), anchor: at.clone(), text: name, tone: tone_of(&binding) }, &at);
}

pub(crate) fn project(input: &SceneInput, layout: &Layout, out: &mut Out) {
    let d = input.document;
    for row in &d.zones {
        if out.wants(&row.id) {
            zone(input, layout, out, row);
        }
    }
    for row in &d.vent_systems {
        if out.wants(&row.id) {
            vent(input, layout, out, row);
        }
    }
}
