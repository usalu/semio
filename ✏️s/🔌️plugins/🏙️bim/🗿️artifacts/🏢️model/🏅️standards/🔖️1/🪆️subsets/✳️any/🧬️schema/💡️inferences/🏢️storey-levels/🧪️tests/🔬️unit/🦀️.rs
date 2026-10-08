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
