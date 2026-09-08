
use super::*;

//#region 🔖️ConfigTests


/// 🔁️ B1 dsl/pack round-trip law for `WiresConfig` — a non-default fixture exercising every field.
#[semio_framework_async_macros::async_test]
async fn wires_config_dsl_pack_round_trip() {
    let config = WiresConfig { drag_node_id: Some("node-1".into()), drag_last_x: 12.5, drag_last_y: -7.25, };
    store::os_store::test_support::assert_dsl_pack_equivalence(&config);
}
//#endregion 🔖️ConfigTests

//#region 🔖️ConfigOperationTests
#[semio_framework_async_macros::async_test]
async fn config_drag_op_text_round_trip() {
    store::os_store::test_support::assert_op_line_round_trip(&WiresConfigMutation::SetDrag(SetDrag { node_id: Some("node-1".into()), last_x: 12.5, last_y: -7.25 }));
    store::os_store::test_support::assert_op_line_round_trip(&WiresConfigMutation::SetDrag(SetDrag { node_id: None, last_x: 0.0, last_y: 0.0 }));
}


/// ⏪️ `backwards()` returns the SAME variant re-addressed at the pre-op field value — a targeted,
/// in-kind inverse, not a whole-config replace.
#[semio_framework_async_macros::async_test]
async fn config_backwards_restores_the_same_field_from_base() {
    let base = WiresConfig { drag_node_id: Some("node-1".into()), drag_last_x: 1.0, drag_last_y: 2.0, ..Default::default() };
    let forward = WiresConfigMutation::SetDrag(SetDrag { node_id: Some("node-2".into()), last_x: 5.0, last_y: 6.0 });
    let inverse = forward.inverse(&base);
    assert_eq!(inverse, vec![WiresConfigMutation::SetDrag(SetDrag { node_id: base.drag_node_id.clone(), last_x: base.drag_last_x, last_y: base.drag_last_y })]);
    assert_eq!(forward.diff(&base).diff().clone(), WiresConfig { drag_node_id: Some("node-2".into()), drag_last_x: 5.0, drag_last_y: 6.0, ..base });
}
//#endregion 🔖️ConfigOperationTests
