//! 🕹️ Handles: the drag grips of a selection. Each handle is a one-parameter `s` (the absolute coordinate along its axis) mapped to document leaves; releasing it commits all its writes as one undoable `setFields`.
//!
//! 📏️ A room carries a thermometer grip per season (operative temperature, 10 °C per metre of height), the floor-area grip on its east face, the outdoor-air grip (400 m³/h per metre of depth), the CO₂ grip (400 ppm per metre of height), the occupant counter and the category stepper; a ventilation system carries the design-airflow grip on its east face (500 m³/h per metre), the heat-recovery grip (efficiency 1 at the box top) and the specific-fan-power grip (1000 W/(m³/s) per metre of depth).

use super::kit::{along, entity_ref, loc, v3};
use super::layout::{Layout, Pose, AIRFLOW_PER_METRE, MAX_VENT_WIDTH, MAX_WIDTH, MIN_VENT_WIDTH, MIN_WIDTH, ROOM_DEPTH, ROOM_HEIGHT, VENT_DEPTH, VENT_HEIGHT};
use super::model::Din16798Snapshot;
use super::scene::*;
use super::scene::projection_kit::handle_kit::{choice_handle, field_binding as binding, grid, handle, round, HandleSpec as Spec};

pub const TEMPERATURE_PER_METRE: f64 = 10.0;
pub const CO2_PER_METRE: f64 = 400.0;
pub const OUTDOOR_AIR_PER_METRE: f64 = 400.0;
pub const SFP_PER_METRE: f64 = 1000.0;
pub const MIN_TEMPERATURE_C: f64 = 15.0;
pub const MAX_TEMPERATURE_C: f64 = 32.0;
pub const MIN_CO2_PPM: f64 = 400.0;
pub const MAX_CO2_PPM: f64 = 2000.0;
pub const MAX_OUTDOOR_AIR_M3_H: f64 = 4000.0;
pub const MAX_SFP_W_M3_S: f64 = 3000.0;
pub const MAX_OCCUPANTS: f64 = 200.0;

const CATEGORIES: [(&str, &str, &str); 4] = [("I", "Category I", "Kategorie I"), ("II", "Category II", "Kategorie II"), ("III", "Category III", "Kategorie III"), ("IV", "Category IV", "Kategorie IV")];

fn zone_handles(d: &Din16798Snapshot, layout: &Layout, id: &str) -> Vec<Handle> {
    let (Some(zone), Some(Pose { frame: f, length })) = (d.zones.iter().find(|z| z.id == id), layout.pose(id).cloned()) else { return Vec::new() };
    let entity = entity_ref(EntityKind::Solid, id, "zones");
    let path = |leaf: &str| format!("zones[id={id}].{leaf}");
    let temperature = |tag: &str, leaf: &str, value: f64, at: f64, en: &str, de: &str| {
        handle(Spec {
            entity: &entity,
            tag,
            kind: HandleKind::ScalarAlong { axis: Vec3::Z },
            anchor: v3(f.origin.x + at * length, f.origin.y + 0.25 * ROOM_DEPTH, value / TEMPERATURE_PER_METRE),
            grip: GripStyle::Diamond,
            writes: vec![binding(path(leaf), Affine { base: 0.0, k: TEMPERATURE_PER_METRE }, "°C", round(0.1))],
            snap: grid(0.05),
            limits: (MIN_TEMPERATURE_C / TEMPERATURE_PER_METRE, MAX_TEMPERATURE_C / TEMPERATURE_PER_METRE),
            name: loc(&format!("{en} of {id}"), &format!("{de} von {id}")),
        })
    };
    vec![
        temperature("t-op-summer", "tOpSummerC", zone.t_op_summer_c, 0.75, "Summer operative temperature", "Operative Sommertemperatur"),
        temperature("t-op-winter", "tOpWinterC", zone.t_op_winter_c, 0.25, "Winter operative temperature", "Operative Wintertemperatur"),
        handle(Spec {
            entity: &entity,
            tag: "floor-area",
            kind: HandleKind::ScalarAlong { axis: Vec3::X },
            anchor: v3(f.origin.x + length, f.origin.y + 0.5 * ROOM_DEPTH, 0.5 * ROOM_HEIGHT),
            grip: GripStyle::Arrow,
            writes: vec![binding(path("floorAreaM2"), Affine { base: -ROOM_DEPTH * f.origin.x, k: ROOM_DEPTH }, "m²", round(0.5))],
            snap: grid(0.1),
            limits: (f.origin.x + MIN_WIDTH, f.origin.x + MAX_WIDTH),
            name: loc(&format!("Floor area of {id}"), &format!("Grundfläche von {id}")),
        }),
        handle(Spec {
            entity: &entity,
            tag: "outdoor-air",
            kind: HandleKind::ScalarAlong { axis: Vec3::Y },
            anchor: v3(f.origin.x + 0.5 * length, f.origin.y + zone.outdoor_air_supplied_m3_h / OUTDOOR_AIR_PER_METRE, 0.5 * ROOM_HEIGHT),
            grip: GripStyle::Arrow,
            writes: vec![binding(path("outdoorAirSuppliedM3H"), Affine { base: -OUTDOOR_AIR_PER_METRE * f.origin.y, k: OUTDOOR_AIR_PER_METRE }, "m³/h", round(1.0))],
            snap: grid(0.05),
            limits: (f.origin.y, f.origin.y + MAX_OUTDOOR_AIR_M3_H / OUTDOOR_AIR_PER_METRE),
            name: loc(&format!("Supplied outdoor air of {id}"), &format!("Zugeführte Außenluft von {id}")),
        }),
        handle(Spec {
            entity: &entity,
            tag: "co2",
            kind: HandleKind::ScalarAlong { axis: Vec3::Z },
            anchor: v3(f.origin.x + 0.5 * length, f.origin.y + 0.75 * ROOM_DEPTH, zone.co2_ppm / CO2_PER_METRE),
            grip: GripStyle::Diamond,
            writes: vec![binding(path("co2Ppm"), Affine { base: 0.0, k: CO2_PER_METRE }, "ppm", round(10.0))],
            snap: grid(0.025),
            limits: (MIN_CO2_PPM / CO2_PER_METRE, MAX_CO2_PPM / CO2_PER_METRE),
            name: loc(&format!("CO₂ concentration of {id}"), &format!("CO₂-Konzentration von {id}")),
        }),
        handle(Spec {
            entity: &entity,
            tag: "occupants",
            kind: HandleKind::CountStepper,
            anchor: v3(f.origin.x + 0.1 * length, f.origin.y + 0.1 * ROOM_DEPTH, ROOM_HEIGHT),
            grip: GripStyle::Ring,
            writes: vec![binding(path("occupants"), Affine::IDENTITY, "1", Some(Quantise::Int))],
            snap: SnapRule::NONE,
            limits: (1.0, MAX_OCCUPANTS),
            name: loc(&format!("Number of occupants of {id}"), &format!("Personenanzahl von {id}")),
        }),
        choice_handle(&entity, "category", v3(f.origin.x + 0.9 * length, f.origin.y + 0.1 * ROOM_DEPTH, ROOM_HEIGHT), path("comfortCategory"), &CATEGORIES.map(|(token, en, de)| (token.to_owned(), en, de)), loc(&format!("Comfort category of {id}"), &format!("Komfortkategorie von {id}"))),
    ]
}

fn vent_handles(d: &Din16798Snapshot, layout: &Layout, id: &str) -> Vec<Handle> {
    let (Some(vent), Some(Pose { frame: f, length })) = (d.vent_systems.iter().find(|v| v.id == id), layout.pose(id).cloned()) else { return Vec::new() };
    let entity = entity_ref(EntityKind::Solid, id, "ventSystems");
    let path = |leaf: &str| format!("ventSystems[id={id}].{leaf}");
    vec![
        handle(Spec {
            entity: &entity,
            tag: "design-airflow",
            kind: HandleKind::ScalarAlong { axis: Vec3::X },
            anchor: v3(f.origin.x + length, f.origin.y + 0.5 * VENT_DEPTH, 0.5 * VENT_HEIGHT),
            grip: GripStyle::Arrow,
            writes: vec![binding(path("designAirflowM3H"), Affine { base: -AIRFLOW_PER_METRE * f.origin.x, k: AIRFLOW_PER_METRE }, "m³/h", round(10.0))],
            snap: grid(0.02),
            limits: (f.origin.x + MIN_VENT_WIDTH, f.origin.x + MAX_VENT_WIDTH),
            name: loc(&format!("Design airflow of {id}"), &format!("Auslegungsvolumenstrom von {id}")),
        }),
        handle(Spec {
            entity: &entity,
            tag: "heat-recovery",
            kind: HandleKind::ScalarAlong { axis: Vec3::Z },
            anchor: v3(f.origin.x + 0.5 * length, f.origin.y + 0.5 * VENT_DEPTH, vent.heat_recovery_eta * VENT_HEIGHT),
            grip: GripStyle::Diamond,
            writes: vec![binding(path("heatRecoveryEta"), Affine { base: 0.0, k: 1.0 / VENT_HEIGHT }, "1", round(0.01))],
            snap: grid(0.015),
            limits: (0.0, VENT_HEIGHT),
            name: loc(&format!("Heat recovery efficiency of {id}"), &format!("Temperaturübertragungsgrad der Wärmerückgewinnung von {id}")),
        }),
        handle(Spec {
            entity: &entity,
            tag: "sfp",
            kind: HandleKind::ScalarAlong { axis: Vec3::Y },
            anchor: v3(f.origin.x + 0.25 * length, f.origin.y + vent.sfp_w_m3_s / SFP_PER_METRE, 0.5 * VENT_HEIGHT),
            grip: GripStyle::Arrow,
            writes: vec![binding(path("sfpWM3S"), Affine { base: -SFP_PER_METRE * f.origin.y, k: SFP_PER_METRE }, "W/(m³/s)", round(10.0))],
            snap: grid(0.01),
            limits: (f.origin.y, f.origin.y + MAX_SFP_W_M3_S / SFP_PER_METRE),
            name: loc(&format!("Specific fan power of {id}"), &format!("Spezifische Ventilatorleistung von {id}")),
        }),
    ]
}

fn build(d: &Din16798Snapshot, layout: &Layout, id: &str) -> Vec<Handle> {
    let mut out = zone_handles(d, layout, id);
    out.extend(vent_handles(d, layout, id));
    out
}

pub(crate) fn ids(document: &Din16798Snapshot, layout: &Layout, id: &str) -> Vec<String> {
    build(document, layout, id).into_iter().map(|h| h.id).collect()
}

/// 🕹️ The handles of every selected entity, in selection order, without duplicates.
pub fn handles(document: &Din16798Snapshot, selection: &[EntityRef]) -> Vec<Handle> {
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
