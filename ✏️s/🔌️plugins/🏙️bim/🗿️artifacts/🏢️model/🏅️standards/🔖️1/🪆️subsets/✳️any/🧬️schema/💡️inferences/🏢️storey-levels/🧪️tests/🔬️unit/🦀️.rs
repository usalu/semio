use super::*;
use crate::standards::v1::subsets::any::schema::inferences::model_graph::{kinds, ModelGraph, ModelNode};

fn storey_id(node: &ModelNode) -> String {
    match node {
        ModelNode::Storey(id) => id.clone(),
        other => panic!("not a storey: {other:?}"),
    }
}
use semio_framework_pack_json::{from_json_str, JsonMemberPolicy};

const HOUSE: &str = include_str!("../../../../../🧫️fixtures/💡️inferences/🏠️house/📸️snapshot/🔣️.json");

fn house() -> ModelSnapshot {
    from_json_str(HOUSE, JsonMemberPolicy::Reject).expect("house decodes")
}

fn close(left: f64, right: f64) -> bool {
    (left - right).abs() <= 1e-9 * right.abs().max(1.0)
}

#[semio_framework_async_macros::async_test]
async fn level_zero_is_the_datum_and_levels_stack_both_ways() {
    let levels = compute_storey_levels(&house());
    let at = |id: &str| levels[id];
    assert!(close(at("st-basement").elevation, -2.6) && close(at("st-basement").top_elevation, 0.0));
    assert!(close(at("st-ground").elevation, 0.0) && close(at("st-ground").top_elevation, 3.0));
    assert!(close(at("st-first").elevation, 3.0) && close(at("st-first").top_elevation, 5.8));
    assert!(close(at("st-roof").elevation, 5.8) && close(at("st-roof").top_elevation, 6.3));
}

#[semio_framework_async_macros::async_test]
async fn absolute_levels_add_site_and_building_elevation() {
    let levels = compute_storey_levels(&house());
    assert!(close(levels["st-ground"].absolute_elevation, 102.5));
    assert!(close(levels["st-shed"].absolute_elevation, 99.5), "the shed building sits at -0.5 on the same 100 m site");
    assert!(close(levels["st-basement"].absolute_top_elevation, 102.5));
}

#[semio_framework_async_macros::async_test]
async fn every_storey_has_at_most_one_parent_in_its_own_building() {
    let snapshot = house();
    let plan = <ModelGraph<{ kinds::LEVELS }> as protocol::InferredField<ModelSnapshot>>::plan(&snapshot);
    assert_eq!(plan.len(), snapshot.storeys.len());
    for step in &plan {
        assert!(step.parents.len() <= 1);
        for parent in &step.parents {
            assert_eq!(snapshot.storeys[&storey_id(parent)].building, snapshot.storeys[&storey_id(&step.key)].building);
        }
    }
    assert_eq!(parent_of(&snapshot, "st-first").as_deref(), Some("st-ground"));
    assert_eq!(parent_of(&snapshot, "st-basement"), None);
    assert_eq!(parent_of(&snapshot, "st-ground"), None);
}

#[semio_framework_async_macros::async_test]
async fn dependency_covers_exactly_what_compute_reads() {
    let before = house();
    let mut after = before.clone();
    after.storeys.get_mut("st-ground").expect("ground").height = 3.4;
    let key = ModelNode::Storey("st-ground".to_string());
    let read = |snapshot: &ModelSnapshot| <ModelGraph<{ kinds::LEVELS }> as protocol::InferredField<ModelSnapshot>>::dep_input(snapshot, &key, &[]);
    assert_ne!(read(&before), read(&after), "a height edit must change the dependency");
    after.storeys.get_mut("st-ground").expect("ground").name = "Renamed".into();
    after.storeys.get_mut("st-ground").expect("ground").height = 3.0;
    assert_eq!(read(&before), read(&after), "a rename must not invalidate the levels");
}

#[semio_framework_async_macros::async_test]
async fn a_height_edit_recomputes_the_storeys_above_and_nothing_else_and_a_material_edit_none() {
    use crate::standards::v1::subsets::any::schema::inferences::model_graph::ModelInferenceSession;
    use crate::{Entry, MaterialPatch, ModelDiff, StoreyPatch};
    let snapshot = house();
    let mut session = ModelInferenceSession::new();
    let first = session.update(&snapshot, &ModelDiff::default()).storey_levels.clone();
    let paint = ModelDiff::materials("m-brick", Entry::Patched(MaterialPatch { density: Some(1900.0), ..Default::default() }));
    let untouched = session.update(&snapshot, &paint).storey_levels.clone();
    assert_eq!(first, untouched);
    assert_eq!(session.report().computed_by_kind.get("storey"), None, "a material edit recomputes no storey");
    let height = ModelDiff::storeys("st-ground", Entry::Patched(StoreyPatch { height: Some(3.4), ..Default::default() }));
    let edited = protocol::apply_diff(&height, &snapshot).expect("applies");
    let recomputed = session.update(&edited, &height).storey_levels.clone();
    assert!(close(recomputed["st-first"].elevation - first["st-first"].elevation, 0.4) && close(recomputed["st-roof"].elevation - first["st-roof"].elevation, 0.4), "the storeys above are lifted");
    assert_eq!(recomputed["st-basement"], first["st-basement"], "the storey below is untouched");
    assert_eq!(recomputed["st-shed"], first["st-shed"], "another building is untouched");
    assert_eq!(session.report().computed_by_kind.get("storey"), Some(&3), "the edited storey and the two above it, no other");
}

#[semio_framework_async_macros::async_test]
async fn the_levels_are_deterministic_and_empty_by_default() {
    assert_eq!(compute_storey_levels(&house()), compute_storey_levels(&house()));
    assert!(compute_storey_levels(&ModelSnapshot::default()).is_empty());
}

#[semio_framework_async_macros::async_test]
async fn one_top_resolver_serves_every_constraint() {
    let own = StoreyLevel { elevation: 3.0, top_elevation: 6.0, absolute_elevation: 3.0, absolute_top_elevation: 6.0 };
    let above = StoreyLevel { elevation: 6.0, top_elevation: 9.0, absolute_elevation: 6.0, absolute_top_elevation: 9.0 };
    assert_eq!(vertical_of(0.1, &TopConstraint::Unconnected { height: 2.0 }, &own, None), (3.1, 5.1));
    assert_eq!(vertical_of(0.0, &TopConstraint::StoreyTop { offset: -0.2 }, &own, None), (3.0, 5.8));
    assert_eq!(vertical_of(0.0, &TopConstraint::Storey { storey: "x".into(), offset: 0.5 }, &own, Some(&above)), (3.0, 6.5));
    assert_eq!(vertical_of(0.0, &TopConstraint::Storey { storey: "x".into(), offset: 0.5 }, &own, None), (3.0, 3.5), "a missing target falls back to the own storey");
}
