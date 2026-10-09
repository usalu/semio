use super::*;
use super::testing::{close, gable_roof, model, rect, wall};
use crate::standards::v1::subsets::any::schema::inferences::wall_layout::compute_wall_layout;
use serde_json::json;

#[semio_framework_async_macros::async_test]
async fn a_wall_on_a_gable_end_follows_the_underside_of_the_roof() {
    let snapshot = model(json!({ "w-gable": wall([8.0, 0.0, 8.0, 6.0], json!({ "Roof": { "roof": "r", "offset": 0.0 } }), json!({})) }), json!({ "roofs": gable_roof() }));
    let layout = &compute_wall_layout(&snapshot)["w-gable"];
    let rise = 0.5f64.tan() * 3.0;
    assert!(close(layout.base_z, 0.0) && close(layout.top_z, 3.0 + rise) && close(layout.height, 3.0 + rise), "{layout:?}");
    assert_eq!(layout.top_profile.len(), 3, "{:?}", layout.top_profile);
    assert!(close(layout.top_at(0.0), 3.0) && close(layout.top_at(3.0), 3.0 + rise) && close(layout.top_at(6.0), 3.0));
    assert!(close(layout.side_area, 3.0 * 6.0 + 9.0 * 0.5f64.tan()), "{}", layout.side_area);
    assert!(close(layout.volume, 0.3 * layout.side_area), "a free wall of constant thickness: volume {} side area {}", layout.volume, layout.side_area);
    let state = layout.top_attach.as_ref().expect("the top is attached");
    assert!(state.found && close(state.covered, 1.0) && !state.clamped && !state.cycle && state.target == "r");
}

#[semio_framework_async_macros::async_test]
async fn a_wall_along_the_eave_keeps_a_flat_top() {
    let snapshot = model(json!({ "w-eave": wall([0.0, 0.0, 8.0, 0.0], json!({ "Roof": { "roof": "r", "offset": -0.1 } }), json!({})) }), json!({ "roofs": gable_roof() }));
    let layout = &compute_wall_layout(&snapshot)["w-eave"];
    assert!(close(layout.top_z, 2.9) && close(layout.base_z, 0.0) && close(layout.side_area, 8.0 * 2.9), "{layout:?}");
    assert!(layout.top_profile.len() <= 2, "a constant top needs two breakpoints at most: {:?}", layout.top_profile);
}

#[semio_framework_async_macros::async_test]
async fn a_missing_target_leaves_the_wall_at_the_storey_top_and_says_so() {
    let snapshot = model(json!({ "w": wall([0.0, 0.0, 8.0, 0.0], json!({ "Roof": { "roof": "r-gone", "offset": 0.0 } }), json!({})) }), json!({}));
    let layout = &compute_wall_layout(&snapshot)["w"];
    assert!(close(layout.top_z, 3.0) && layout.top_profile.is_empty());
    let state = layout.top_attach.as_ref().expect("the attach is reported");
    assert!(!state.found && state.covered == 0.0 && state.target == "r-gone");
}

#[semio_framework_async_macros::async_test]
async fn the_base_follows_a_sloped_slab() {
    let slab = json!({ "sl": { "storey": "st", "slab_type": "slt", "boundary": rect(0.0, 0.0, 10.0, 10.0), "holes": [], "offset": 0.0, "slope": { "direction": 0.0, "angle": 0.1 }, "phase": "New", "name": "" } });
    let snapshot = model(json!({ "w": wall([1.0, 1.0, 9.0, 1.0], json!({ "StoreyTop": { "offset": 0.0 } }), json!({ "base_slab": "sl" })) }), json!({ "slabs": slab }));
    let layout = &compute_wall_layout(&snapshot)["w"];
    let fall = 0.1f64.tan();
    assert!(close(layout.base_at(0.0), -fall) && close(layout.base_at(8.0), -9.0 * fall), "{:?}", layout.base_profile);
    assert!(close(layout.base_z, -9.0 * fall) && close(layout.top_z, 3.0));
    assert!(close(layout.side_area, 3.0 * 8.0 + fall * (1.0 + 9.0) / 2.0 * 8.0), "{}", layout.side_area);
    assert!(layout.top_profile.is_empty() && layout.base_attach.as_ref().is_some_and(|state| state.found && close(state.covered, 1.0)));
}

#[semio_framework_async_macros::async_test]
async fn where_the_target_does_not_reach_the_nearest_covered_height_is_held() {
    let slab = json!({ "sl": { "storey": "st", "slab_type": "slt", "boundary": rect(0.0, 0.0, 4.0, 4.0), "holes": [], "offset": 0.0, "phase": "New", "name": "" } });
    let snapshot = model(json!({ "w": wall([0.0, 1.0, 8.0, 1.0], json!({ "Slab": { "slab": "sl", "offset": 0.5 } }), json!({})) }), json!({ "slabs": slab }));
    let layout = &compute_wall_layout(&snapshot)["w"];
    let state = layout.top_attach.as_ref().expect("the top is attached");
    assert!(state.found && close(state.covered, 0.5), "{state:?}");
    assert!(close(layout.top_z, -0.2 + 0.5) && close(layout.top_at(7.0), 0.3), "the underside of a 0.2 m slab at 0.0 plus 0.5: {:?}", layout.top_profile);
}

#[semio_framework_async_macros::async_test]
async fn a_top_below_the_base_is_lifted_and_reported() {
    let snapshot = model(json!({ "w": wall([0.0, 0.0, 8.0, 0.0], json!({ "Roof": { "roof": "r", "offset": -9.0 } }), json!({})) }), json!({ "roofs": gable_roof() }));
    let layout = &compute_wall_layout(&snapshot)["w"];
    assert!(layout.top_attach.as_ref().is_some_and(|state| state.clamped), "{layout:?}");
    assert!(close(layout.height, MIN_HEIGHT) && layout.top_at(4.0) > layout.base_at(4.0));
}

#[test]
fn the_reference_graph_reports_a_loop_and_none_for_a_chain() {
    let edges = |node: &str| match node {
        "a" => vec!["b".to_string()],
        "b" => vec!["c".to_string()],
        "c" => vec!["a".to_string()],
        "x" => vec!["y".to_string()],
        _ => Vec::new(),
    };
    assert_eq!(find_cycle("a", &edges), Some(vec!["a".to_string(), "b".to_string(), "c".to_string()]));
    assert_eq!(find_cycle("b", &edges), Some(vec!["b".to_string(), "c".to_string(), "a".to_string()]));
    assert_eq!(find_cycle("x", &edges), None);
    let snapshot = model(json!({ "w": wall([0.0, 0.0, 8.0, 0.0], json!({ "Roof": { "roof": "r", "offset": 0.0 } }), json!({ "base_slab": "sl" })) }), json!({ "roofs": gable_roof() }));
    assert_eq!(find_cycle("w", &edges_of(&snapshot)), None, "walls point at leaf elements, so the authored references cannot loop");
}

fn edges_of(snapshot: &ModelSnapshot) -> impl Fn(&str) -> Vec<String> + '_ {
    edges(snapshot)
}

#[test]
fn elevation_edges_interpolate_extend_and_restrict() {
    let edge = [ElevationPoint { s: 1.0, z: 2.0 }, ElevationPoint { s: 3.0, z: 4.0 }, ElevationPoint { s: 5.0, z: 4.0 }];
    assert!(close(elevation_at(&edge, 2.0), 3.0) && close(elevation_at(&edge, 4.0), 4.0));
    assert!(close(elevation_at(&edge, 0.0), 1.0), "the first segment continues backwards");
    assert!(close(elevation_at(&edge, 6.0), 4.0), "the last segment continues forwards");
    let cut = restricted(&edge, 2.0, 4.0);
    assert_eq!(cut.len(), 3);
    assert!(close(cut[0].z, 3.0) && close(cut[1].z, 4.0) && close(cut[2].z, 4.0));
    assert!(close(elevation_at(&[ElevationPoint { s: 0.0, z: 7.0 }], 3.0), 7.0) && elevation_at(&[], 1.0) == 0.0);
    let flat = simplified(vec![ElevationPoint { s: 0.0, z: 1.0 }, ElevationPoint { s: 1.0, z: 1.0 }, ElevationPoint { s: 2.0, z: 1.0 }, ElevationPoint { s: 2.0, z: 3.0 }]);
    assert_eq!(flat.len(), 3, "a collinear middle point goes, a step stays: {flat:?}");
}

#[semio_framework_async_macros::async_test]
async fn the_surface_node_is_a_parent_of_the_attached_wall_only() {
    use crate::standards::v1::subsets::any::schema::inferences::model_graph::{kinds, plan, ModelNode};
    let snapshot = model(json!({ "w-gable": wall([8.0, 0.0, 8.0, 6.0], json!({ "Roof": { "roof": "r", "offset": 0.0 } }), json!({})), "w-plain": wall([0.0, 0.0, 8.0, 0.0], json!({ "StoreyTop": { "offset": 0.0 } }), json!({})) }), json!({ "roofs": gable_roof() }));
    let steps = plan::build(&snapshot, kinds::closure(kinds::LAYOUTS));
    let parents = |id: &str| steps.iter().find(|step| step.key == ModelNode::WallLayout(id.to_string())).map(|step| step.parents.clone()).expect("a layout step");
    assert!(parents("w-gable").contains(&ModelNode::Surface("r".to_string())));
    assert!(!parents("w-plain").iter().any(|node| matches!(node, ModelNode::Surface(_))));
    let position = |key: &ModelNode| steps.iter().position(|step| &step.key == key).expect("planned");
    assert!(position(&ModelNode::Surface("r".to_string())) < position(&ModelNode::WallLayout("w-gable".to_string())), "parents come before children");
}
