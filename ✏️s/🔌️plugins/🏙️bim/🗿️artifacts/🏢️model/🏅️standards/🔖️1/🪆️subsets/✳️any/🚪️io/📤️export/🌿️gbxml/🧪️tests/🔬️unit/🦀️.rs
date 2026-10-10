use super::*;
use crate::standards::v1::subsets::any::schema::inferences::energy_envelope::EnvelopeSurface;
use semio_framework_pack_json::{from_json_str, JsonMemberPolicy};
use semio_s_artifact_stdio_xml::standards::v1_0::subsets::base::io::text::snapshot::xml_document_from_text;

const BOX: &str = include_str!("../../../../../🧫️fixtures/💡️inferences/🌡️energy-envelope/🏠️box/📸️snapshot/🔣️.json");
const ZONING: &str = include_str!("../../../../../🧫️fixtures/💡️inferences/🌡️energy-envelope/🏘️zoning/📸️snapshot/🔣️.json");
const STACK: &str = include_str!("../../../../../🧫️fixtures/💡️inferences/🌡️energy-envelope/🧱️stack/📸️snapshot/🔣️.json");

fn load(text: &str) -> ModelSnapshot {
    from_json_str(text, JsonMemberPolicy::Reject).expect("the committed fixture decodes")
}

fn plan_for(model: &ModelSnapshot) -> Plan {
    crate::standards::v1::subsets::any::schema::inferences::model_graph::registry::try_with_inference(None, model, |inferred| plan_of(model, inferred)).expect("infers").expect("a plan")
}

fn count(text: &str, needle: &str) -> usize {
    text.matches(needle).count()
}

fn near(left: f64, right: f64) -> bool {
    (left - right).abs() <= 1e-6 * right.abs().max(1.0)
}

#[test]
fn ids_are_xs_ids_unique_and_made_of_the_prefix_and_the_model_id() {
    let mut names = Names::default();
    assert_eq!(names.id("space", "sp-b1"), "space-sp-b1");
    assert_eq!(names.id("space", "sp/b1"), "space-sp_b1");
    assert_eq!(names.id("space", "sp_b1"), "space-sp_b1-2", "a clash is numbered");
    assert!(names.id("surface", "a b\u{e4}").chars().all(|letter| letter.is_ascii_alphanumeric() || matches!(letter, '-' | '_' | '.')));
}

#[test]
fn surface_types_follow_the_mapping_of_the_contract() {
    let of = |kind, boundary| surface_type(&EnvelopeSurface { kind, boundary, ..EnvelopeSurface::default() });
    assert_eq!(of(SurfaceKind::Wall, Boundary::Exterior), "ExteriorWall");
    assert_eq!(of(SurfaceKind::Wall, Boundary::Ground), "UndergroundWall");
    assert_eq!(of(SurfaceKind::Wall, Boundary::Adjacent), "InteriorWall");
    assert_eq!(of(SurfaceKind::Wall, Boundary::Adiabatic), "InteriorWall");
    assert_eq!(of(SurfaceKind::CurtainWall, Boundary::Exterior), "ExteriorWall");
    assert_eq!(of(SurfaceKind::Floor, Boundary::Ground), "SlabOnGrade");
    assert_eq!(of(SurfaceKind::Floor, Boundary::Exterior), "ExposedFloor");
    assert_eq!(of(SurfaceKind::Floor, Boundary::Adjacent), "InteriorFloor");
    assert_eq!(of(SurfaceKind::Ceiling, Boundary::Exterior), "Roof");
    assert_eq!(of(SurfaceKind::Ceiling, Boundary::Adjacent), "Ceiling");
    assert!(exposed_to_sun("ExteriorWall") && exposed_to_sun("Roof") && exposed_to_sun("ExposedFloor"));
    assert!(!exposed_to_sun("InteriorWall") && !exposed_to_sun("SlabOnGrade") && !exposed_to_sun("UndergroundWall"));
}

#[test]
fn the_box_is_one_space_with_six_surfaces_two_openings_and_its_constructions() {
    let model = load(BOX);
    let plan = plan_for(&model);
    assert_eq!((plan.spaces.len(), plan.zones.len(), plan.surfaces.len()), (1, 1, 6));
    assert_eq!(plan.surfaces.iter().map(|row| row.openings.len()).sum::<usize>(), 2);
    let kinds: Vec<&str> = plan.surfaces.iter().map(|row| row.kind).collect();
    assert_eq!(kinds.iter().filter(|kind| **kind == "ExteriorWall").count(), 4);
    assert_eq!((kinds.contains(&"SlabOnGrade"), kinds.contains(&"Roof")), (true, true));
    assert_eq!(plan.window_types.len(), 1);
    assert_eq!(plan.door_constructions.len(), 1);
    assert!(plan.constructions.len() >= 2, "a wall and a slab construction");
    let space = &plan.spaces[0];
    assert_eq!((space.condition, space.people.map(|people| near(people, 0.999))), ("HeatedAndCooled", Some(true)));
    assert_eq!((plan.zones[0].heating, plan.zones[0].cooling), (Some(20.0), Some(26.0)));
}

#[test]
fn the_roof_without_a_construction_has_no_construction_reference() {
    let plan = plan_for(&load(BOX));
    let roof = plan.surfaces.iter().find(|row| row.kind == "Roof").expect("a roof");
    assert_eq!(roof.construction, None);
    assert!(plan.surfaces.iter().filter(|row| row.kind != "Roof").all(|row| row.construction.is_some()));
}

#[test]
fn the_document_is_gbxml_7_03_with_metric_units_and_reads_back() {
    let model = load(BOX);
    let (text, notes) = export_gbxml(&model).expect("an export");
    assert!(notes.is_empty(), "{notes:?}");
    assert!(text.starts_with("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<gbXML xmlns=\"http://www.gbxml.org/schema\" version=\"7.03\" useSIUnitsForResults=\"true\""), "{}", &text[..200]);
    assert!(text.contains("temperatureUnit=\"C\"") && text.contains("lengthUnit=\"Meters\"") && text.contains("areaUnit=\"SquareMeters\"") && text.contains("volumeUnit=\"CubicMeters\""));
    assert_eq!(count(&text, "<Surface "), 6);
    assert_eq!(count(&text, "<Opening "), 2);
    assert_eq!(count(&text, "surfaceType=\"ExteriorWall\""), 4);
    assert!(text.contains("openingType=\"FixedWindow\"") && text.contains("openingType=\"NonSlidingDoor\""));
    assert_eq!(count(&text, "<AdjacentSpaceId "), 6);
    assert!(xml_document_from_text(&text).is_ok());
}

#[test]
fn the_polygons_of_a_space_are_closed_and_enclose_its_volume() {
    let model = load(BOX);
    let plan = plan_for(&model);
    let (mut sum, mut volume) = ([0.0; 3], 0.0);
    for row in &plan.surfaces {
        let vector = geometry::area_vector(&row.polygon);
        let centre = [0, 1, 2].map(|axis| row.polygon.iter().map(|point| point[axis]).sum::<f64>() / row.polygon.len() as f64);
        for axis in 0..3 {
            sum[axis] += vector[axis] / 2.0;
        }
        volume += (centre[0] * vector[0] + centre[1] * vector[1] + centre[2] * vector[2]) / 6.0;
    }
    assert!(sum.iter().all(|part| part.abs() < 1e-6), "{sum:?}");
    assert!(near(volume, plan.spaces[0].volume), "{volume} vs {}", plan.spaces[0].volume);
}

#[test]
fn the_window_and_the_door_have_the_areas_of_their_types() {
    let plan = plan_for(&load(BOX));
    let openings: Vec<&OpeningRow> = plan.surfaces.iter().flat_map(|row| row.openings.iter()).collect();
    let window = openings.iter().find(|row| row.kind == "FixedWindow").expect("window");
    let door = openings.iter().find(|row| row.kind == "NonSlidingDoor").expect("door");
    assert!(near(window.area, 1.44) && near(door.area, 2.1), "{} {}", window.area, door.area);
    assert_eq!(window.u_value, Some(1.1));
    assert!(window.window_type.is_some() && window.construction.is_none() && door.construction.is_some() && door.window_type.is_none());
}

#[test]
fn two_rooms_share_their_partition_once() {
    let plan = plan_for(&load(ZONING));
    assert_eq!((plan.spaces.len(), plan.zones.len()), (2, 2));
    let shared: Vec<&SurfaceRow> = plan.surfaces.iter().filter(|row| row.spaces.len() == 2).collect();
    assert!(!shared.is_empty(), "the partition between the rooms is shared");
    assert!(shared.iter().all(|row| row.kind == "InteriorWall" && row.cad.len() == 2 && !row.exposed));
    assert_eq!(plan.surfaces.iter().filter(|row| row.kind == "InteriorWall" && row.spaces.len() == 1).count(), 0, "no partition without its partner");
    assert!(shared.iter().any(|row| !row.openings.is_empty()), "the door of the partition is written once");
}

#[test]
fn a_stack_has_underground_walls_a_shared_interior_floor_and_a_roof() {
    let plan = plan_for(&load(STACK));
    let kinds: Vec<&str> = plan.surfaces.iter().map(|row| row.kind).collect();
    for wanted in ["UndergroundWall", "SlabOnGrade", "InteriorFloor", "Roof", "ExteriorWall"] {
        assert!(kinds.contains(&wanted), "{wanted} in {kinds:?}");
    }
    assert!(plan.surfaces.iter().filter(|row| row.kind == "InteriorFloor").all(|row| row.spaces.len() == 2));
}

#[test]
fn a_model_without_conditions_has_no_envelope_and_is_refused() {
    let message = export_gbxml(&ModelSnapshot::default()).expect_err("refused");
    assert!(message.contains("no space states thermal conditions"), "{message}");
}

#[test]
fn the_export_is_deterministic() {
    let model = load(ZONING);
    assert_eq!(export_gbxml(&model).expect("first"), export_gbxml(&model).expect("second"));
}

#[test]
fn the_staged_export_ends_with_the_same_document_and_reports_its_progress() {
    let model = load(ZONING);
    let one_shot = export_gbxml(&model).expect("an export").0;
    let staged = crate::standards::v1::subsets::any::schema::inferences::model_graph::registry::try_with_inference(None, &model, |inferred| {
        let mut job = StagedGbxml::new();
        let mut seen = vec![job.stage()];
        let mut last = job.fraction();
        loop {
            let done = job.step(&model, inferred);
            assert!(job.fraction() >= last);
            last = job.fraction();
            if let Some(result) = done {
                return (seen, result);
            }
            seen.push(job.stage());
        }
    })
    .expect("infers");
    assert_eq!(staged.0.first(), Some(&Some("zones")));
    assert_eq!(staged.0.last(), Some(&Some("document")));
    assert_eq!(staged.1.expect("a document").0, one_shot);
}

#[test]
fn the_table_starts_with_the_counts_of_the_plan() {
    let table = export_table(&load(BOX)).expect("a table");
    assert!(table.starts_with("{\"counts\":{\"campuses\":1,\"buildings\":1,\"storeys\":1,\"spaces\":1,\"zones\":1,\"surfaces\":6,\"openings\":2"), "{}", &table[..120.min(table.len())]);
}

const FIXTURES: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🌿️gbxml");
const HOUSE: &str = include_str!("../../../../../🧫️fixtures/🌿️gbxml/🏠️house/📸️snapshot/🔣️.json");

#[test]
fn the_committed_files_are_the_current_export() {
    for (case, model) in [("🏠️box", load(BOX)), ("🏘️zoning", load(ZONING)), ("🧱️stack", load(STACK)), ("🏠️house", load(HOUSE))] {
        let (text, notes) = export_gbxml(&model).expect("the export");
        assert!(notes.is_empty(), "{case}: {notes:?}");
        let path = format!("{FIXTURES}/{case}/gbxml.xml");
        if std::env::var("BIM_BLESS").is_ok() {
            std::fs::create_dir_all(format!("{FIXTURES}/{case}")).expect("the fixture directory");
            std::fs::write(&path, text.as_bytes()).expect("the file is written");
        }
        let committed = std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("{case}: {error}. Run the test with BIM_BLESS=1 to write the file, then `python 🐍️.py write` of the export case."));
        assert_eq!(committed, text, "{case}: the committed export drifted: rewrite it with BIM_BLESS=1");
    }
}
