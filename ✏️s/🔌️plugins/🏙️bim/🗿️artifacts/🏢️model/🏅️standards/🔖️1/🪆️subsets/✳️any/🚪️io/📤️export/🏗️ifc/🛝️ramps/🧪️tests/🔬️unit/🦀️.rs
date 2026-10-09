use crate::standards::v1::subsets::any::io::export::ifc::testkit::{count, document, house, quantity, rows, string, tags, target};
use crate::{ModelSnapshot, Point2, Ramp, TopConstraint, Vertex};

fn vertex(x: f64, y: f64, bulge: f64) -> Vertex {
    Vertex { point: Point2 { x, y }, bulge }
}

/// 🛝️ The house with a 10 m ramp on its first storey: 1.5 m landings at both ends, 0.5 m of rise.
pub fn with_ramp(left: bool) -> ModelSnapshot {
    let mut model = house();
    let storey = model.storeys.iter().min_by_key(|(_, storey)| storey.level).map(|(id, _)| id.clone()).expect("a storey");
    let material = model.materials.keys().next().cloned().expect("a material");
    model.ramps.insert(
        "rp-1".into(),
        Ramp {
            storey,
            path: vec![vertex(0.0, -3.0, 0.0), vertex(10.0, -3.0, 0.0)],
            width: 1.2,
            landing_start: 1.5,
            landing_end: 1.5,
            landing_turn: 1.5,
            max_slope: 1.0 / 12.0,
            thickness: 0.2,
            material,
            base_offset: 0.0,
            top: TopConstraint::Unconnected { height: 0.5 },
            railing_left: left,
            railing_right: false,
            name: "Entrance Ramp".into(),
        },
    );
    model
}

#[test]
fn a_ramp_aggregates_a_flight_per_slope_and_a_landing_slab_per_landing() {
    let document = document(&with_ramp(false));
    assert_eq!(tags(&document, "IFCRAMP"), ["rp-1"]);
    let (ramp, args) = rows(&document, "IFCRAMP").into_iter().next().expect("a ramp");
    assert_eq!(args[8].as_enum(), Some("STRAIGHT_RUN_RAMP"));
    assert!(target(&document, args, 6).is_none(), "a decomposed ramp carries no body of its own");
    let (_, aggregate) = rows(&document, "IFCRELAGGREGATES").into_iter().find(|(_, relation)| relation[4].as_ref_id() == Some(ramp.id)).expect("the parts");
    let parts = aggregate[5].as_list().expect("parts");
    assert_eq!(parts.len(), 3);
    let flights: Vec<_> = parts.iter().filter_map(|part| document.resolve(part)).filter_map(|instance| instance.entity("IFCRAMPFLIGHT")).collect();
    assert_eq!(flights.len(), 1);
    assert!(target(&document, flights[0], 6).is_some(), "the flight carries its share of the brep");
    let landings: Vec<_> = parts.iter().filter_map(|part| document.resolve(part)).filter_map(|instance| instance.entity("IFCSLAB")).collect();
    assert_eq!(landings.len(), 2);
    assert!(landings.iter().all(|landing| landing[8].as_enum() == Some("LANDING") && target(&document, landing, 6).is_some()));
}

#[test]
fn the_ramp_carries_its_base_quantities_and_the_authored_record() {
    let document = document(&with_ramp(false));
    assert!((quantity(&document, "rp-1", "Qto_RampBaseQuantities", "Length").expect("a length") - 10.0).abs() < 1e-6);
    assert!((quantity(&document, "rp-1", "Qto_RampBaseQuantities", "Width").expect("a width") - 1.2).abs() < 1e-6);
    assert!((quantity(&document, "rp-1", "Qto_RampBaseQuantities", "Height").expect("a height") - 0.5).abs() < 1e-6);
    assert!((quantity(&document, "rp-1", "Qto_RampBaseQuantities", "GrossArea").expect("an area") - 12.0).abs() < 1e-6);
    assert!((quantity(&document, "rp-1", "Qto_RampBaseQuantities", "GrossVolume").expect("a volume") - 2.4).abs() < 1e-6);
    let record = rows(&document, "IFCPROPERTYSINGLEVALUE").into_iter().find(|(_, args)| string(args, 0).as_deref() == Some("Ramp")).expect("the authored record row");
    let text = record.1[2].as_typed().and_then(|(_, items)| items.first()).and_then(|item| item.as_str()).expect("a string");
    assert!(text.contains("\"landing_start\"") && text.contains("Entrance Ramp"));
}

#[test]
fn a_side_railing_adds_an_ifc_railing_part() {
    let document = document(&with_ramp(true));
    let (_, relation) = rows(&document, "IFCRELAGGREGATES").into_iter().find(|(_, relation)| relation[4].as_ref_id().and_then(|id| document.instances.iter().find(|instance| instance.id == id)).is_some_and(|instance| instance.entity("IFCRAMP").is_some())).expect("the parts");
    let parts = relation[5].as_list().expect("parts");
    assert_eq!(parts.len(), 4);
    let railing = parts.iter().filter_map(|part| document.resolve(part)).find_map(|instance| instance.entity("IFCRAILING")).expect("a railing");
    assert_eq!(railing[8].as_enum(), Some("GUARDRAIL"));
    assert!(target(&document, railing, 6).is_some());
}

#[test]
fn a_bent_ramp_writes_a_flight_per_slope_and_a_landing_per_landing_of_its_run() {
    use crate::ModelInference;
    use protocol::Inference;
    let mut model = with_ramp(false);
    model.ramps.get_mut("rp-1").expect("the ramp").path = vec![vertex(0.0, -3.0, 0.0), vertex(6.0, -3.0, 0.0), vertex(6.0, 2.0, 0.0)];
    let run = ModelInference::infer(&model).expect("infers").ramp_runs["rp-1"].clone();
    let document = document(&model);
    assert_eq!(count(&document, "IFCRAMPFLIGHT"), run.flights.len());
    assert_eq!(rows(&document, "IFCSLAB").iter().filter(|(_, args)| args[8].as_enum() == Some("LANDING")).count(), run.landings.len());
    let (_, args) = rows(&document, "IFCRAMP").into_iter().next().expect("a ramp");
    assert_eq!(args[8].as_enum(), Some("TWO_STRAIGHT_RUN_RAMP"));
}

const SUBSET_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures");
const RAMPS_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🏗️ifc/🛝️ramps");

fn ramps() -> ModelSnapshot {
    let text = std::fs::read_to_string(format!("{SUBSET_DIR}/💡️inferences/🛝️ramp-runs/🏞️ramps/📸️snapshot/🔣️.json")).expect("the committed ramps model");
    semio_framework_pack_json::from_json_str(&text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("the committed ramps decode")
}

fn read_ramps(name: &str) -> Vec<u8> {
    std::fs::read(format!("{RAMPS_DIR}/{name}")).unwrap_or_else(|error| panic!("{name}: {error}. Run the test with BIM_BLESS=1 to write the file, then `python 🐍️.py write` of the export case."))
}

#[test]
fn the_report_counts_one_ramp_with_its_flights_landings_and_railings_per_resolved_ramp() {
    use crate::standards::v1::subsets::any::io::export::ifc::projection::projection;
    use crate::ModelInference;
    use protocol::Inference;
    let model = ramps();
    let runs = ModelInference::infer(&model).expect("infers").ramp_runs;
    let written: Vec<_> = runs.iter().filter(|(_, run)| run.length > 1e-9).collect();
    let report = projection(&model);
    assert_eq!(report.counts["IfcRamp"], written.len(), "a ramp on a missing storey and one without a path are skipped");
    assert_eq!(report.counts["IfcRampFlight"], written.iter().map(|(_, run)| run.flights.len()).sum::<usize>());
    assert_eq!(report.counts["IfcRailing"], model.ramps.iter().filter(|(id, ramp)| runs[id.as_str()].length > 1e-9 && (ramp.railing_left || ramp.railing_right)).count());
    assert_eq!(report.containment["st-ground"].iter().filter(|tag| tag.starts_with("r-")).count() + report.containment["st-first"].iter().filter(|tag| tag.starts_with("r-")).count(), written.len());
}

#[test]
fn the_committed_ramps_file_is_the_current_export() {
    let (bytes, notes) = crate::standards::v1::subsets::any::io::export::ifc::export_ifc2x3(&ramps()).expect("the ramps export");
    assert_eq!(notes.len(), 2, "exactly the ramp on a missing storey and the ramp without a path are skipped: {notes:?}");
    if std::env::var("BIM_BLESS").is_ok() {
        std::fs::write(format!("{RAMPS_DIR}/🛝️ramps.ifc"), &bytes).expect("the file is written");
    }
    assert_eq!(read_ramps("🛝️ramps.ifc"), bytes, "the committed export drifted: rewrite it with BIM_BLESS=1");
}

#[test]
fn the_ramps_subject_report_equals_the_table_the_ifcopenshell_oracle_measured_from_the_committed_file() {
    use crate::standards::v1::subsets::any::io::export::ifc::projection::projection;
    let oracle: serde_json::Value = serde_json::from_slice(&read_ramps("🔬️measure/🔣️.json")).expect("the oracle table");
    let ours: serde_json::Value = serde_json::from_str(&projection(&ramps()).to_json()).expect("the report parses");
    assert_eq!(oracle["schema"], ours["schema"]);
    assert_eq!(oracle["counts"], ours["counts"]);
    assert_eq!(oracle["containment"], ours["containment"]);
    assert_eq!(oracle["volumes"], ours["volumes"]);
}
