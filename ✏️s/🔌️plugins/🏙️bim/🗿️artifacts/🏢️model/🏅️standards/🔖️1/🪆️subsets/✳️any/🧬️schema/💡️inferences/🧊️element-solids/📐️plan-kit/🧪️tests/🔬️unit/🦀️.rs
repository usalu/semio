use crate::standards::v1::subsets::any::io::text::inferences::element_solids::planar_projection_json;
use super::testing::{case, close};
use super::*;
use crate::standards::v1::subsets::any::schema::inferences::element_solids::compute_element_solids;
use crate::standards::v1::subsets::any::schema::inferences::model_graph::ModelInferenceSession;
use crate::{Entry, MaterialPatch, ModelDiff, ModelSnapshot, StoreyPatch};

const COLUMNS: &str = include_str!("../../../../../../🧫️fixtures/💡️inferences/🧊️element-solids/🏛️columns-profiles/🔣️.json");

#[test]
fn a_stack_lists_the_depths_of_every_layer_below_the_top() {
    assert_eq!(stack(&[0.05, 0.2, 0.1]), vec![(0.0, 0.05), (0.05, 0.25), (0.25, 0.35)]);
    assert!(stack(&[]).is_empty());
}

#[test]
fn a_loop_is_rotated_about_the_origin_and_then_moved() {
    let moved = placed(&rectangle(2.0, 1.0), Point::new(10.0, 0.0), std::f64::consts::FRAC_PI_2);
    let corners: Vec<(f64, f64)> = moved.iter().map(|v| (v.point.x, v.point.y)).collect();
    for ((x, y), (want_x, want_y)) in corners.iter().zip([(10.5, -1.0), (10.5, 1.0), (9.5, 1.0), (9.5, -1.0)]) {
        assert!(close(want_x, *x, 1e-12) && close(want_y, *y, 1e-12), "{x} {y} against {want_x} {want_y}");
    }
}

#[test]
fn layer_thicknesses_clamp_negative_values_to_zero() {
    let layer = |thickness: f64| Layer { material: String::new(), thickness, function: crate::LayerFunction::Structure };
    assert_eq!(layer_thicknesses(&[layer(0.1), layer(-0.3)]), vec![0.1, 0.0]);
}

const CASES: [&str; 6] = [
    include_str!("../../../../../../🧫️fixtures/💡️inferences/🧊️element-solids/🏛️columns-profiles/🔣️.json"),
    include_str!("../../../../../../🧫️fixtures/💡️inferences/🧊️element-solids/➖️beams-profiles/🔣️.json"),
    include_str!("../../../../../../🧫️fixtures/💡️inferences/🧊️element-solids/⬜️slabs-holes-slope/🔣️.json"),
    include_str!("../../../../../../🧫️fixtures/💡️inferences/🧊️element-solids/🏔️roofs-shapes/🔣️.json"),
    include_str!("../../../../../../🧫️fixtures/💡️inferences/🧊️element-solids/🪜️stairs-flights/🔣️.json"),
    include_str!("../../../../../../🧫️fixtures/💡️inferences/🧊️element-solids/🛤️railings-posts/🔣️.json"),
];

#[semio_framework_async_macros::async_test]
async fn the_planar_projection_is_the_table_the_third_party_case_compares() {
    for text in CASES {
        let case = case(text);
        let solids = compute_element_solids(&case.snapshot);
        let projection: serde_json::Value = serde_json::from_str(&planar_projection_json(&case.snapshot, &solids)).expect("the projection is JSON");
        let planar: Vec<(&String, &serde_json::Value)> = case.expected.as_object().expect("table").iter().filter(|(_, row)| !row.is_null() && row["curved"] == serde_json::Value::Bool(false)).collect();
        assert_eq!(projection.as_object().expect("table").len(), planar.len(), "exactly the planar elements are listed");
        for (id, row) in planar {
            assert!(!is_curved(&case.snapshot, id), "{id}");
            assert!(close(row["volume"].as_f64().expect("number"), projection[id]["volume"].as_f64().expect("number"), 1e-9), "{id}.volume");
            for (key, axis) in [("min", 0), ("max", 0), ("min", 1), ("max", 1), ("min", 2), ("max", 2)] {
                assert!(close(row[key][axis].as_f64().expect("number"), projection[id][key][axis].as_f64().expect("number"), 1e-9), "{id}.{key}[{axis}]");
            }
        }
        for (id, row) in case.expected.as_object().expect("table") {
            assert_eq!(row.is_object() && row["curved"] == serde_json::Value::Bool(true), is_curved(&case.snapshot, id) && !row.is_null(), "{id}: curved-ness agrees with the oracle");
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn a_diff_outside_the_reads_serves_the_stored_solids_and_a_storey_edit_re_infers_them() {
    let snapshot = case(COLUMNS).snapshot;
    let mut session = ModelInferenceSession::new();
    let first = session.update(&snapshot, &ModelDiff::default()).element_solids.clone();
    let colour = ModelDiff::materials("m-steel", Entry::Patched(MaterialPatch { density: Some(7000.0), ..Default::default() }));
    let untouched = session.update(&snapshot, &colour).element_solids.clone();
    assert_eq!(first, untouched, "a material edit leaves every solid alone");
    let height = ModelDiff::storeys("st-ground", Entry::Patched(StoreyPatch { height: Some(3.4), ..Default::default() }));
    let edited = protocol::apply_diff(&height, &snapshot).expect("applies");
    let recomputed = session.update(&edited, &height).element_solids.clone();
    assert!(close(recomputed["c-rect"].bounds.max.z - first["c-rect"].bounds.max.z, 0.4, 1e-9), "a storey height edit re-infers the columns that follow it");
}
