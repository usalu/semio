use super::*;
use protocol::Inference;
use semio_framework_pack_json::{from_json_str, JsonMemberPolicy};

const HOUSE: &str = include_str!("../../../../🧫️fixtures/💡️inferences/🏠️house/📸️snapshot/🔣️.json");

fn house() -> ModelSnapshot {
    from_json_str(HOUSE, JsonMemberPolicy::Reject).expect("house decodes")
}

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = house();
    assert_eq!(ModelInference::infer(&snapshot).expect("infers"), ModelInference::infer(&snapshot).expect("infers"));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(ModelInference::infer(&ModelSnapshot::default()).expect("infers"), ModelInference::default());
}

#[semio_framework_async_macros::async_test]
async fn infers_a_level_per_storey_and_a_layout_per_wall() {
    let snapshot = house();
    let inferred = ModelInference::infer(&snapshot).expect("infers");
    assert_eq!(inferred.storey_levels.keys().collect::<Vec<_>>(), snapshot.storeys.keys().collect::<Vec<_>>());
    assert_eq!(inferred.wall_layout.keys().collect::<Vec<_>>(), snapshot.walls.keys().collect::<Vec<_>>());
}

const LEVELS: &str = include_str!("../../../../🧫️fixtures/💡️inferences/🏠️house/💡️inference/🪜️storey-levels/🔣️.json");
const LAYOUTS: &str = include_str!("../../../../🧫️fixtures/💡️inferences/🏠️house/💡️inference/🧱️wall-layout/🔣️.json");

const JOINS_SNAPSHOT: &str = include_str!("../../../../🧫️fixtures/💡️inferences/🔗️wall-joins/📸️snapshot/🔣️.json");
const JOINS_LAYOUTS: &str = include_str!("../../../../🧫️fixtures/💡️inferences/🔗️wall-joins/💡️inference/🧱️wall-layout/🔣️.json");

fn disagreements(expected: &serde_json::Value, actual: &serde_json::Value, path: &str) -> Vec<String> {
    use serde_json::Value;
    match (expected, actual) {
        (Value::Object(want), Value::Object(got)) => {
            let mut problems: Vec<String> = if want.len() == got.len() { Vec::new() } else { vec![format!("{path}: {} members expected, {} found", want.len(), got.len())] };
            problems.extend(want.iter().flat_map(|(key, value)| got.get(key).map_or_else(|| vec![format!("{path}.{key}: missing")], |found| disagreements(value, found, &format!("{path}.{key}")))));
            problems
        }
        (Value::Array(want), Value::Array(got)) if want.len() == got.len() => want.iter().zip(got).enumerate().flat_map(|(index, (value, found))| disagreements(value, found, &format!("{path}[{index}]"))).collect(),
        (Value::Array(want), Value::Array(got)) => vec![format!("{path}: {} items expected, {} found", want.len(), got.len())],
        (Value::Number(want), Value::Number(got)) => {
            let (want, got) = (want.as_f64().expect("number"), got.as_f64().expect("number"));
            if (want - got).abs() <= 1e-9 * want.abs().max(1.0) { Vec::new() } else { vec![format!("{path}: oracle {want}, subject {got}")] }
        }
        _ if expected == actual => Vec::new(),
        _ => vec![format!("{path}: oracle {expected}, subject {actual}")],
    }
}

fn agrees(table: &serde_json::Value, produced: &serde_json::Value) {
    let problems = disagreements(table, produced, "");
    assert!(problems.is_empty(), "{problems:#?}");
}

#[semio_framework_async_macros::async_test]
async fn the_subject_reproduces_the_third_party_oracle_tables() {
    let table = |text: &str| serde_json::from_str::<serde_json::Value>(text).expect("JSON table");
    let produced = |text: String| serde_json::from_str::<serde_json::Value>(&text).expect("JSON table");
    let model = house();
    let inferred = ModelInference::infer(&model).expect("infers");
    agrees(&table(LEVELS), &produced(semio_framework_pack_json::to_json_string(&inferred.storey_levels)));
    agrees(&table(LAYOUTS), &produced(semio_framework_pack_json::to_json_string(&inferred.wall_layout)));
    let joins: ModelSnapshot = from_json_str(JOINS_SNAPSHOT, JsonMemberPolicy::Reject).expect("the joins snapshot decodes");
    agrees(&table(JOINS_LAYOUTS), &produced(semio_framework_pack_json::to_json_string(&ModelInference::infer(&joins).expect("infers").wall_layout)));
}

#[semio_framework_async_macros::async_test]
async fn the_wall_solids_projection_lists_the_straight_walls_with_their_layout_extent() {
    use crate::standards::v1::subsets::any::io::text::snapshot::encode_inference_projection_json;
    let model = house();
    let inferred = ModelInference::infer(&model).expect("infers");
    let solids: serde_json::Value = serde_json::from_str(&encode_inference_projection_json(&model, "wall-solids").expect("known table")).expect("JSON table");
    let straight: Vec<&String> = model.walls.iter().filter(|(_, wall)| matches!(wall.axis, crate::Axis::Line { .. })).map(|(id, _)| id).collect();
    assert_eq!(solids.as_object().expect("table").len(), straight.len());
    for id in straight {
        let layout = &inferred.wall_layout[id];
        assert!((solids[id]["base_z"].as_f64().expect("number") - layout.base_z).abs() < 1e-12, "{id}.base_z");
        assert!((solids[id]["top_z"].as_f64().expect("number") - layout.top_z).abs() < 1e-12, "{id}.top_z");
        assert!((solids[id]["volume"].as_f64().expect("number") - layout.volume).abs() < 1e-12, "{id}.volume");
    }
    assert!(encode_inference_projection_json(&model, "no-such-table").is_none());
}

#[semio_framework_async_macros::async_test]
async fn a_diff_outside_the_reads_serves_the_stored_result_without_walking_the_plan() {
    use crate::{Entry, ModelDiff, StoreyPatch};
    use crate::standards::v1::subsets::any::schema::inferences::model_graph::ModelInferenceSession;
    let snapshot = house();
    let mut session = ModelInferenceSession::new();
    let first = session.update(&snapshot, &ModelDiff::default()).clone();
    let project = ModelDiff { project: Some(Default::default()), ..ModelDiff::default() };
    let untouched = session.update(&snapshot, &project).clone();
    assert_eq!(first, untouched, "a project edit leaves the inference alone");
    assert!(session.report().gated && session.report().computed == 0, "a diff outside the reads walks no plan");
    let height = ModelDiff::storeys("st-ground", Entry::Patched(StoreyPatch { height: Some(3.4), ..Default::default() }));
    let edited = protocol::apply_diff(&height, &snapshot).expect("applies");
    let recomputed = session.update(&edited, &height).storey_levels.clone();
    assert!((recomputed["st-first"].elevation - first.storey_levels["st-first"].elevation - 0.4).abs() < 1e-9, "a storey height edit re-infers the levels above");
}
