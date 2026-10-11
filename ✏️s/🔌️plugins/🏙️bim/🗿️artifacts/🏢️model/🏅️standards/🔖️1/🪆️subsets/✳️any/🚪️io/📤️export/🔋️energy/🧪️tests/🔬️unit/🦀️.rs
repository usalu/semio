use super::*;
use crate::standards::v1::subsets::any::io::export::svg::testkit::house;
use crate::standards::v1::subsets::any::schema::inferences::model_graph::instance as inference;
use semio_framework_pack_json::{from_json_str, JsonMemberPolicy};
use std::collections::BTreeSet;
use target::{OutsideBoundary, SurfaceClass};

const ROOM: &str = include_str!("../../../../../🧫️fixtures/🚪️energy/🏠️room/📸️snapshot/🔣️.json");
const PAIR: &str = include_str!("../../../../../🧫️fixtures/🚪️energy/🏘️pair/📸️snapshot/🔣️.json");
const STACK: &str = include_str!("../../../../../🧫️fixtures/🚪️energy/🧱️stack/📸️snapshot/🔣️.json");

/// 📁️ The directory of the committed energy exports (the model as JSON and as `.energy` text) that the jsonschema, shapely and numpy oracle reads.
pub const DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🚪️energy");

fn decoded(text: &str) -> ModelSnapshot {
    from_json_str(text, JsonMemberPolicy::Reject).expect("the committed case decodes")
}

fn written(snapshot: &ModelSnapshot) -> (EnergyModel, Vec<String>) {
    model_to_energy(snapshot).expect("the case is written")
}

fn cases() -> Vec<(&'static str, ModelSnapshot)> {
    vec![("🏠️room", decoded(ROOM)), ("🏘️pair", decoded(PAIR)), ("🧱️stack", decoded(STACK))]
}

fn audit(model: &EnergyModel) {
    let unique = |ids: Vec<u32>| ids.iter().collect::<BTreeSet<_>>().len() == ids.len();
    assert!(unique(model.zones.iter().map(|row| row.id).collect()) && unique(model.surfaces.iter().map(|row| row.id).collect()) && unique(model.fenestrations.iter().map(|row| row.id).collect()), "ids are unique");
    assert!(unique(model.materials.iter().map(|row| row.id).collect()) && unique(model.constructions.iter().map(|row| row.id).collect()), "ids are unique");
    let names: BTreeSet<&str> = model.zones.iter().map(|zone| zone.name.as_str()).collect();
    assert_eq!(names.len(), model.zones.len(), "zone names are unique");
    let zones: BTreeSet<u32> = model.zones.iter().map(|zone| zone.id).collect();
    let constructions: BTreeSet<u32> = model.constructions.iter().map(|row| row.id).collect();
    let materials: BTreeSet<u32> = model.materials.iter().map(|row| row.id).collect();
    for zone in &model.zones {
        assert!(zone.volume_m3 > 0.0, "{}: a zone has a volume", zone.name);
    }
    for surface in &model.surfaces {
        assert!(zones.contains(&surface.zone_id) && constructions.contains(&surface.construction_id) && surface.vertices_m.len() >= 3, "{}: its references resolve", surface.name);
        if let OutsideBoundary::Interzone(partner) = surface.outside_boundary_condition {
            let other = model.surfaces.iter().find(|other| other.id == partner).expect("the partner exists");
            assert_eq!(other.outside_boundary_condition, OutsideBoundary::Interzone(surface.id), "{}: the partner points back", surface.name);
            assert_ne!(other.zone_id, surface.zone_id, "{}: the partner is in another zone", surface.name);
        }
    }
    for construction in &model.constructions {
        assert!(!construction.layer_material_ids.is_empty() && construction.layer_material_ids.iter().all(|id| materials.contains(id)), "{}: its layers resolve", construction.name);
    }
    for material in &model.materials {
        assert!(material.thickness_m > 0.0 && material.conductivity_w_m_k > 0.0, "{}: a layer has thickness and conductivity", material.name);
    }
    for window in &model.fenestrations {
        let host = model.surfaces.iter().find(|surface| surface.id == window.surface_id).expect("the host exists");
        assert_eq!(host.outside_boundary_condition, OutsideBoundary::OutdoorAir, "{}: a fenestration stands in an exterior surface", window.name);
        assert!(window.u_value_w_m2k > 0.0 && (0.0..=1.0).contains(&window.shgc) && window.area_m2 > 0.0 && window.height_m > 0.0, "{}: its data is legal", window.name);
    }
    let schedule_ids: BTreeSet<u32> = model.schedules.constants.iter().map(|row| row.id).chain(model.schedules.daily.iter().map(|row| row.id)).chain(model.schedules.weekly.iter().map(|row| row.id)).collect();
    for id in model.people.iter().map(|row| row.schedule_id).chain(model.people.iter().map(|row| row.activity_schedule_id)).chain(model.lighting.iter().map(|row| row.schedule_id)).chain(model.equipment.iter().map(|row| row.schedule_id)).chain(model.thermostats.iter().flat_map(|row| [row.heating_setpoint_schedule_id, row.cooling_setpoint_schedule_id])) {
        assert!(schedule_ids.contains(&id), "schedule {id} is defined");
    }
}

#[test]
fn every_case_is_written_as_a_model_the_engine_accepts() {
    for (case, snapshot) in cases() {
        let (model, notes) = written(&snapshot);
        audit(&model);
        assert!(!model.surfaces.is_empty() && !model.zones.is_empty(), "{case}: zones and surfaces");
        assert_eq!(model.spaces.len(), model.zones.len(), "{case}: one space per zone");
        assert!(notes.iter().all(|note| note.contains("merged into its wall")), "{case}: only partition openings are merged: {notes:?}");
    }
}

#[test]
fn the_site_carries_the_location_the_north_and_the_time_zone() {
    let (model, _) = written(&decoded(ROOM));
    assert_eq!((model.site.latitude_deg, model.site.longitude_deg, model.site.elevation_m, model.site.time_zone_hours), (47.0, 8.0, 0.0, 1.0));
    assert!((model.site.north_axis_deg - (0.3f64 - 0.2).to_degrees()).abs() < 1e-12, "the bearing of the model's north is the true north minus the building rotation");
    assert_eq!(model.name, "Model");
    assert_eq!(model.version, build::VERSION);
}

#[test]
fn a_partition_between_two_zones_is_two_interzone_partners_and_its_door_is_merged_into_the_wall() {
    let (model, notes) = written(&decoded(PAIR));
    let partners: Vec<_> = model.surfaces.iter().filter(|surface| matches!(surface.outside_boundary_condition, OutsideBoundary::Interzone(_))).collect();
    assert_eq!(partners.len(), 2, "one surface in each zone: {partners:?}");
    assert!(partners.iter().all(|surface| surface.class == SurfaceClass::Interzone));
    assert_eq!(model.adjacency_pairs.len(), 1);
    assert!(model.fenestrations.iter().all(|window| window.name.starts_with("window ")), "the partition door is no fenestration");
    assert_eq!(notes.iter().filter(|note| note.contains("door") && note.contains("merged into its wall")).count(), 2, "{notes:?}");
    assert_eq!(model.thermal_enclosures.len(), 2, "the two zones of the BIM model");
}

#[test]
fn rooms_with_equal_set_points_are_adiabatic_and_still_paired() {
    let (model, _) = written(&decoded(STACK));
    let adiabatic: Vec<_> = model.surfaces.iter().filter(|surface| surface.outside_boundary_condition == OutsideBoundary::Adiabatic).collect();
    assert!(adiabatic.len() >= 2, "floor and ceiling between the two heated storeys");
    assert!(model.adjacency_pairs.iter().any(|pair| adiabatic.iter().any(|surface| surface.id == pair.surface_a_id)), "an adiabatic pair is listed");
    assert!(model.surfaces.iter().any(|surface| surface.outside_boundary_condition == OutsideBoundary::Ground), "the cellar walls stand against the ground");
    assert!(model.surfaces.iter().any(|surface| surface.class == SurfaceClass::Roof), "the roof");
}

#[test]
fn a_wall_is_listed_outside_first_and_named_mirrored_when_the_room_lies_on_the_right_of_its_axis() {
    let (model, _) = written(&decoded(ROOM));
    let insulation = model.materials.iter().find(|material| material.name.starts_with("Insulation")).expect("the insulation layer").id;
    for surface in model.surfaces.iter().filter(|surface| surface.class == SurfaceClass::ExteriorWall) {
        let construction = model.constructions.iter().find(|row| row.id == surface.construction_id).expect("the construction");
        assert_eq!(construction.layer_material_ids.first() == Some(&insulation), !construction.name.ends_with("(mirrored)"), "{}: {}", surface.name, construction.name);
    }
}

#[test]
fn the_loads_and_set_points_of_the_conditions_become_gains_a_thermostat_and_an_ideal_loads_system() {
    let (model, _) = written(&decoded(ROOM));
    let (people, lighting, equipment) = (&model.people[0], &model.lighting[0], &model.equipment[0]);
    assert_eq!((people.people_per_area, lighting.watts_per_area, equipment.watts_per_area), (0.1, 8.0, 12.0));
    let thermostat = &model.thermostats[0];
    let value = |id: u32| model.schedules.constants.iter().find(|row| row.id == id).map(|row| row.value);
    assert_eq!((value(thermostat.heating_setpoint_schedule_id), value(thermostat.cooling_setpoint_schedule_id)), (Some(20.0), Some(26.0)));
    assert!((model.ideal_loads[0].outdoor_air_per_area_m3_s_m2 - 0.0015).abs() < 1e-15, "1.5 litres per second per square metre");
    let weekly = model.schedules.weekly.iter().find(|row| row.id == people.schedule_id).expect("Office 08-18 is a weekly schedule");
    let on = model.schedules.daily.iter().find(|row| row.id == weekly.daily_schedule_ids[1]).expect("the weekday");
    let off = model.schedules.daily.iter().find(|row| row.id == weekly.daily_schedule_ids[0]).expect("the weekend");
    assert_eq!(on.hourly_values.iter().filter(|hour| **hour == 1.0).count(), 10);
    assert_eq!((on.hourly_values[7], on.hourly_values[8], on.hourly_values[17], on.hourly_values[18]), (0.0, 1.0, 1.0, 0.0));
    assert!(off.hourly_values.iter().all(|hour| *hour == 0.0));
    assert_eq!(weekly.daily_schedule_ids[0], weekly.daily_schedule_ids[6], "Sunday and Saturday");
}

#[test]
fn an_unheated_space_gets_gains_and_no_thermostat() {
    let (model, _) = written(&decoded(STACK));
    let cellar = model.zones.iter().find(|zone| zone.name.starts_with("sp-b")).expect("the cellar");
    assert!(!cellar.conditioned && !cellar.part_of_total_floor_area);
    assert!(model.thermostats.iter().all(|row| row.zone_id != cellar.id) && model.ideal_loads.iter().all(|row| row.zone_id != cellar.id));
}

#[test]
fn a_model_without_a_thermal_envelope_is_refused() {
    let snapshot = crate::standards::v1::subsets::any::io::text::snapshot::default_snapshot();
    let refusal = model_to_energy(&snapshot).expect_err("no space with conditions");
    assert!(refusal.contains("no space with a thermal envelope"), "{refusal}");
}

#[test]
fn the_staged_export_runs_one_stage_per_step_and_equals_the_one_shot() {
    let snapshot = decoded(STACK);
    let direct = written(&snapshot);
    let staged = inference::try_with_inference(None, &snapshot, |inferred| {
        let mut job = StagedEnergy::new();
        let mut seen = Vec::new();
        loop {
            let (fraction, stage) = (job.fraction(), job.stage());
            seen.push((fraction, stage));
            if let Some(done) = job.step(&snapshot, inferred).expect("the step") {
                return (done, seen);
            }
        }
    })
    .expect("inferred");
    let (done, seen) = staged;
    assert_eq!(done, direct);
    assert_eq!(seen.iter().map(|(_, stage)| stage.expect("a stage")).collect::<Vec<_>>(), STAGES.iter().map(|(name, _)| *name).collect::<Vec<_>>());
    assert!(seen.windows(2).all(|pair| pair[0].0 < pair[1].0) && seen[0].0 == 0.0, "progress only grows");
}

#[test]
fn the_text_is_the_energy_artifacts_dsl_and_the_json_its_snapshot() {
    let (text, _) = export_energy(&decoded(ROOM)).expect("the text");
    assert!(text.starts_with("semio energy.model.dsl v1"), "{}", &text[..text.len().min(80)]);
    assert!(text.contains("schema=energy.model") && text.contains("structure=") && text.contains("surfaces=["));
    let (json, _) = export_energy_json(&decoded(ROOM)).expect("the json");
    let root = crate::standards::v1::subsets::any::io::export::json::codec::read_value(&json).expect("RFC 8259");
    let member = |name: &str| crate::standards::v1::subsets::any::io::export::json::codec::member(&root, name);
    assert!(member("schema").is_some() && member("model").is_some() && member("structure").is_some() && member("zones").is_some());
    assert!(member("referencedModel").is_some() && member("weatherLink").is_some(), "the link slots are null, not absent");
    assert!(json.ends_with("}\n") && json.contains("\"childId\": \"energy-value\""));
}

#[test]
fn the_serializer_writes_the_text_and_one_warning_per_note() {
    let snapshot = decoded(PAIR);
    let (text, notes) = export_energy(&snapshot).expect("the export");
    let payload = futures_lite_block(ModelIntoEnergy::serialize(&snapshot, &ArchiveChildren::default())).expect("the hop");
    assert_eq!(payload.value, IoPayload::Text(text));
    assert_eq!(payload.diagnostics.len(), notes.len());
    assert!(payload.diagnostics.iter().all(|diagnostic| diagnostic.severity == semio_framework_diagnostic::Severity::Warning));
    assert_eq!(<ModelIntoEnergy as Serializer<ModelSnapshot>>::INTO, ENERGY_DIALECT);
}

fn futures_lite_block<T>(future: impl std::future::Future<Output = T>) -> T {
    semio_framework_async::poll::resolve_ready(future)
}

#[test]
fn the_table_of_the_export_adds_up_to_the_inferred_totals_of_the_project() {
    for (case, snapshot) in cases() {
        inference::try_with_inference(None, &snapshot, |inferred| {
            let table = table::table_of(&snapshot, inferred).expect("the table");
            let (ours, theirs) = (&table.totals["project"], &inferred.energy_totals["project"]);
            assert_eq!(ours.spaces, theirs.spaces, "{case}");
            assert!((ours.envelope_area - theirs.envelope_area).abs() < 1e-9 && (ours.transmission - theirs.transmission).abs() < 1e-9 && (ours.solar_aperture - theirs.solar_aperture).abs() < 1e-9, "{case}: {ours:?} against {theirs:?}");
        })
        .expect("inferred");
    }
}

#[test]
fn the_committed_files_are_the_current_export() {
    for (case, snapshot) in cases() {
        let (model, _) = written(&snapshot);
        let files = [("🔋️model.json", document::json_text(&model).expect("json")), ("🗣️.dsl.semio", document::dsl_text(&model).expect("dsl"))];
        if std::env::var("BIM_BLESS").is_ok() {
            std::fs::create_dir_all(format!("{DIR}/{case}")).expect("the fixture directory");
            for (name, text) in &files {
                std::fs::write(format!("{DIR}/{case}/{name}"), text).expect("the file is written");
            }
        }
        for (name, text) in &files {
            let committed = std::fs::read_to_string(format!("{DIR}/{case}/{name}")).unwrap_or_else(|error| panic!("{case}/{name}: {error}. Run the test with BIM_BLESS=1 to write the file, then `python 🐍️.py write` of the export case."));
            assert!(committed == *text, "{case}/{name}: the committed export drifted: rewrite it with BIM_BLESS=1");
        }
    }
}

#[test]
fn the_house_is_written_with_its_roof_space_floating_and_everything_else_conditioned() {
    let snapshot = house();
    let (model, notes) = written(&snapshot);
    audit(&model);
    assert!(model.zones.iter().any(|zone| zone.conditioned) && model.zones.iter().any(|zone| !zone.conditioned));
    assert!(notes.iter().all(|note| !note.contains("not written")), "the house has all its thermal data: {notes:?}");
}
