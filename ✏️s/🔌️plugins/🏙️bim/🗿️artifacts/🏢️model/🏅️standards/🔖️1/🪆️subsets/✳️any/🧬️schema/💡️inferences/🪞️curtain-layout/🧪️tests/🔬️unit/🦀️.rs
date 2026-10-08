use super::*;
use crate::standards::v1::subsets::any::schema::inferences::model_graph::{kinds, plan, ModelNode};
use crate::{Entry, ModelDiff, StoreyPatch};
use semio_framework_pack_json::{from_json_str, JsonMemberPolicy};

const HOUSE: &str = include_str!("../../../../../🧫️fixtures/💡️inferences/🏠️house/📸️snapshot/🔣️.json");

fn curtain(storey: &str, top: serde_json::Value, base_offset: f64, length: f64) -> serde_json::Value {
    serde_json::json!({
        "storey": storey,
        "axis": { "Line": { "start": { "x": 0.0, "y": 10.0 }, "end": { "x": length, "y": 10.0 } } },
        "base_offset": base_offset,
        "top": top,
        "u_spacing": 1.5,
        "v_spacing": 1.0,
        "mullion": { "Rectangle": { "width": 0.05, "depth": 0.1 } },
        "panel_material": "m-brick",
        "mullion_material": "m-insulation",
        "name": "Curtain"
    })
}

fn model() -> ModelSnapshot {
    let mut value: serde_json::Value = serde_json::from_str(HOUSE).expect("house is JSON");
    value["curtain_walls"] = serde_json::json!({
        "cw-ground": curtain("st-ground", serde_json::json!({ "StoreyTop": { "offset": 0.0 } }), 0.0, 7.5),
        "cw-first": curtain("st-first", serde_json::json!({ "Storey": { "storey": "st-roof", "offset": 0.0 } }), 0.2, 6.0),
        "cw-free": curtain("st-shed", serde_json::json!({ "Unconnected": { "height": 2.0 } }), 0.0, 3.0),
        "cw-orphan": curtain("st-missing", serde_json::json!({ "StoreyTop": { "offset": 0.0 } }), 0.0, 3.0)
    });
    from_json_str(&value.to_string(), JsonMemberPolicy::Reject).expect("the model decodes")
}

fn raised(snapshot: &ModelSnapshot, delta: f64) -> ModelSnapshot {
    let height = snapshot.storeys["st-ground"].height + delta;
    protocol::apply_diff(&ModelDiff::storeys("st-ground", Entry::Patched(StoreyPatch { height: Some(height), ..Default::default() })), snapshot).expect("the height edit applies")
}

fn close(left: f64, right: f64) -> bool {
    (left - right).abs() <= 1e-9 * right.abs().max(1.0)
}

#[semio_framework_async_macros::async_test]
async fn curtain_walls_resolve_extent_length_and_panel_grid() {
    let layouts = compute_curtain_layout(&model());
    assert_eq!(layouts.keys().map(String::as_str).collect::<Vec<_>>(), ["cw-first", "cw-free", "cw-ground"], "a curtain wall on a missing storey has no layout");
    let ground = layouts["cw-ground"];
    assert!(close(ground.base_z, 0.0) && close(ground.top_z, 3.0) && close(ground.height, 3.0) && close(ground.length, 7.5) && close(ground.area, 22.5));
    assert_eq!((ground.u_panels, ground.v_panels), (5, 3));
    assert!(close(ground.panel_width, 1.5) && close(ground.panel_height, 1.0));
    let first = layouts["cw-first"];
    assert!(close(first.base_z, 3.2) && close(first.top_z, 5.8), "resolved against the elevation of the roof storey: 3.0 + 2.8 = 5.8, base 3.0 + 0.2");
    assert_eq!((first.u_panels, first.v_panels), (4, 3));
    let free = layouts["cw-free"];
    assert!(close(free.base_z, 0.0) && close(free.height, 2.0) && close(free.length, 3.0));
    assert_eq!((free.u_panels, free.v_panels), (2, 2));
}

#[semio_framework_async_macros::async_test]
async fn changing_a_storey_height_re_infers_exactly_the_curtain_walls_resolved_by_it() {
    let (before, after) = (compute_curtain_layout(&model()), compute_curtain_layout(&raised(&model(), 0.4)));
    assert!(close(after["cw-ground"].height - before["cw-ground"].height, 0.4), "a StoreyTop curtain wall grows with its storey");
    assert_eq!(after["cw-ground"].v_panels, 4, "3.4 m at 1 m spacing needs four rows");
    assert!(close(after["cw-ground"].panel_height, 3.4 / 4.0));
    assert!(close(after["cw-first"].base_z - before["cw-first"].base_z, 0.4) && close(after["cw-first"].height, before["cw-first"].height), "curtain walls above are lifted, not stretched");
    assert_eq!(after["cw-free"], before["cw-free"], "another building is untouched");
}

#[semio_framework_async_macros::async_test]
async fn the_curtain_layout_is_deterministic_and_empty_by_default() {
    assert_eq!(compute_curtain_layout(&model()), compute_curtain_layout(&model()));
    assert!(compute_curtain_layout(&ModelSnapshot::default()).is_empty());
}

#[test]
fn the_panel_grid_rounds_up_and_never_drops_below_one() {
    assert_eq!((panels(7.5, 1.5), panels(7.6, 1.5), panels(0.2, 1.5), panels(3.0, 0.0), panels(0.0, 1.0)), (5, 6, 1, 1, 1));
}

#[semio_framework_async_macros::async_test]
async fn a_curtain_wall_depends_on_its_storey_and_the_storey_its_top_targets() {
    let snapshot = model();
    let steps = plan::build(&snapshot, kinds::closure(kinds::CURTAINS));
    let parents = |id: &str| steps.iter().find(|step| step.key == ModelNode::CurtainLayout(id.into())).map(|step| step.parents.clone()).expect("planned");
    assert_eq!(parents("cw-ground"), vec![ModelNode::Storey("st-ground".into())]);
    assert_eq!(parents("cw-first"), vec![ModelNode::Storey("st-first".into()), ModelNode::Storey("st-roof".into())]);
    assert!(steps.iter().all(|step| step.key != ModelNode::CurtainLayout("cw-orphan".into())));
}
