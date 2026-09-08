
use super::*;
use crate::schema::mutations::*;
use crate::{Pose, Process3dSnapshot, ProcessMeasure, ProcessStep, WorkingSolid, brep_child_handle, brep_snapshot_for_working_solid, empty_process3d_snapshot};
use protocol::Mutation;

fn cut_step(id: &str) -> ProcessStep {
    ProcessStep { id: id.into(), label: "Cut".into(), enabled: true, origin: None, measure: ProcessMeasure::Cut { tool: WorkingSolid::Box { width: 0.1, depth: 0.1, height: 0.1 }, pose: Pose::default() } }
}

fn circular_saw_machine() -> WorkshopMachine {
    use crate::{CapabilityParameter, CapabilityRule, MeasureRecipe, StockQuantity};
    WorkshopMachine {
        id: "circularSaw".into(),
        label: "Circular Saw".into(),
        icon_id: "scissors".into(),
        catalog_id: Some("wood".into()),
        capabilities: vec![Capability {
            id: "crosscut".into(),
            label: "Crosscut".into(),
            icon_id: "scissors".into(),
            recipe: MeasureRecipe::DiscCut { diameter: "bladeDiameter".into(), kerf: "kerf".into() },
            parameters: vec![CapabilityParameter { id: "bladeDiameter".into(), label: "Blade Diameter".into(), value: 0.184 }, CapabilityParameter { id: "kerf".into(), label: "Kerf".into(), value: 0.002 }],
            rules: vec![CapabilityRule::Min { quantity: StockQuantity::Width, parameter: "bladeDiameter".into(), margin: 0.0 }],
        }],
    }
}

#[semio_framework_async_macros::async_test]
async fn process3d_op_text_round_trips_create_step() {
    store::os_store::test_support::assert_op_line_round_trip(&Process3dMutation::CreateStep(create_step::CreateStep { index: 0, step: cut_step("cut-1") }));
}

#[semio_framework_async_macros::async_test]
async fn process3d_op_text_round_trips_delete_step() {
    store::os_store::test_support::assert_op_line_round_trip(&Process3dMutation::DeleteStep(delete_step::DeleteStep { id: "cut-1".into() }));
}

#[semio_framework_async_macros::async_test]
async fn process3d_op_text_round_trips_rename_step() {
    store::os_store::test_support::assert_op_line_round_trip(&Process3dMutation::RenameStep(rename_step::RenameStep { id: "cut-1".into(), new_label: "Renamed".into() }));
}

#[semio_framework_async_macros::async_test]
async fn process3d_op_text_round_trips_change_step_enabled() {
    store::os_store::test_support::assert_op_line_round_trip(&Process3dMutation::ChangeStepEnabled(change_step_enabled::ChangeStepEnabled { id: "cut-1".into(), new_enabled: false }));
}

#[semio_framework_async_macros::async_test]
async fn process3d_op_text_round_trips_change_step_origin_set() {
    let new_origin = Some(StepOrigin { machine_id: "tableSaw".into(), capability_id: "crosscut".into() });
    store::os_store::test_support::assert_op_line_round_trip(&Process3dMutation::ChangeStepOrigin(change_step_origin::ChangeStepOrigin { id: "cut-1".into(), new_origin }));
}

#[semio_framework_async_macros::async_test]
async fn process3d_op_text_round_trips_change_step_origin_clear() {
    store::os_store::test_support::assert_op_line_round_trip(&Process3dMutation::ChangeStepOrigin(change_step_origin::ChangeStepOrigin { id: "cut-1".into(), new_origin: None }));
}

#[semio_framework_async_macros::async_test]
async fn process3d_op_text_round_trips_replace_step_measure() {
    let new_measure = ProcessMeasure::Drill { radius: 0.03, depth: 0.4, pose: Pose { position: [1.0, 2.0, 3.0], axis: [0.0, 1.0, 0.0], angle: 0.7 } };
    store::os_store::test_support::assert_op_line_round_trip(&Process3dMutation::ReplaceStepMeasure(replace_step_measure::ReplaceStepMeasure { id: "cut-1".into(), new_measure }));
}

#[semio_framework_async_macros::async_test]
async fn process3d_op_text_round_trips_reorder_steps() {
    store::os_store::test_support::assert_op_line_round_trip(&Process3dMutation::ReorderSteps(reorder_steps::ReorderSteps { id: "cut-1".into(), to_index: 2 }));
}

#[semio_framework_async_macros::async_test]
async fn process3d_op_text_round_trips_create_machine() {
    store::os_store::test_support::assert_op_line_round_trip(&Process3dMutation::CreateMachine(create_machine::CreateMachine { index: 0, machine: circular_saw_machine() }));
}

#[semio_framework_async_macros::async_test]
async fn process3d_op_text_round_trips_delete_machine() {
    store::os_store::test_support::assert_op_line_round_trip(&Process3dMutation::DeleteMachine(delete_machine::DeleteMachine { id: "circularSaw".into() }));
}

#[semio_framework_async_macros::async_test]
async fn process3d_op_text_round_trips_rename_machine() {
    store::os_store::test_support::assert_op_line_round_trip(&Process3dMutation::RenameMachine(rename_machine::RenameMachine { id: "circularSaw".into(), new_label: "Big Saw".into() }));
}

#[semio_framework_async_macros::async_test]
async fn process3d_op_text_round_trips_change_machine_icon() {
    store::os_store::test_support::assert_op_line_round_trip(&Process3dMutation::ChangeMachineIcon(change_machine_icon::ChangeMachineIcon { id: "circularSaw".into(), new_icon_id: "drill".into() }));
}

#[semio_framework_async_macros::async_test]
async fn process3d_op_text_round_trips_replace_machine_capabilities_full() {
    store::os_store::test_support::assert_op_line_round_trip(&Process3dMutation::ReplaceMachineCapabilities(replace_machine_capabilities::ReplaceMachineCapabilities {
        id: "circularSaw".into(),
        new_capabilities: circular_saw_machine().capabilities,
    }));
}

#[semio_framework_async_macros::async_test]
async fn process3d_op_text_round_trips_replace_machine_capabilities_empty() {
    store::os_store::test_support::assert_op_line_round_trip(&Process3dMutation::ReplaceMachineCapabilities(replace_machine_capabilities::ReplaceMachineCapabilities { id: "circularSaw".into(), new_capabilities: vec![] }));
}

#[semio_framework_async_macros::async_test]
async fn process3d_op_text_round_trips_move_stock() {
    store::os_store::test_support::assert_op_line_round_trip(&Process3dMutation::MoveStock(move_stock::MoveStock { new_pose: Pose { position: [1.0, 2.0, 3.0], axis: [0.0, 1.0, 0.0], angle: 0.7 } }));
}

#[semio_framework_async_macros::async_test]
async fn process3d_op_text_round_trips_change_stock_label() {
    store::os_store::test_support::assert_op_line_round_trip(&Process3dMutation::ChangeStockLabel(change_stock_label::ChangeStockLabel { new_label: "Timber Beam".into() }));
}

#[semio_framework_async_macros::async_test]
async fn process3d_op_text_round_trips_replace_stock_solid() {
    let new_solid = brep_child_handle("stock", &brep_snapshot_for_working_solid(&WorkingSolid::ImportedMesh { mesh_url: "data:model/gltf-binary;base64,AAAA".into() }));
    store::os_store::test_support::assert_op_line_round_trip(&Process3dMutation::ReplaceStockSolid(replace_stock_solid::ReplaceStockSolid { new_solid }));
}

#[semio_framework_async_macros::async_test]
async fn process3d_op_text_round_trips_change_cursor_some() {
    store::os_store::test_support::assert_op_line_round_trip(&Process3dMutation::ChangeCursor(change_cursor::ChangeCursor { new_resolved_up_to: Some(3) }));
}

#[semio_framework_async_macros::async_test]
async fn process3d_op_text_round_trips_change_cursor_none() {
    store::os_store::test_support::assert_op_line_round_trip(&Process3dMutation::ChangeCursor(change_cursor::ChangeCursor { new_resolved_up_to: None }));
}

/// ↩️ Ticket `26/09/01/PROCESS-END-TO-END`: `CreateStep` is a real mutation against the durable
/// `step_payloads` timeline now, so undo of a create is a `DeleteStep` by the created id —
/// mirrors `inverse_of_create_machine_is_delete_machine` below.
#[semio_framework_async_macros::async_test]
async fn inverse_of_create_step_is_delete_step() {
    let snapshot = empty_process3d_snapshot();
    let mutation = Process3dMutation::CreateStep(create_step::CreateStep { index: 0, step: cut_step("a") });
    let inverse = mutation.inverse(&snapshot);
    assert_eq!(inverse.len(), 1);
    match &inverse[0] {
        Process3dMutation::DeleteStep(payload) => assert_eq!(payload.id, "a"),
        _ => panic!("expected DeleteStep"),
    }
}

#[semio_framework_async_macros::async_test]
async fn inverse_of_create_machine_is_delete_machine() {
    let snapshot = empty_process3d_snapshot();
    let mutation = Process3dMutation::CreateMachine(create_machine::CreateMachine { index: 0, machine: circular_saw_machine() });
    let inverse = mutation.inverse(&snapshot);
    assert_eq!(inverse.len(), 1);
    match &inverse[0] {
        Process3dMutation::DeleteMachine(payload) => assert_eq!(payload.id, "circularSaw"),
        _ => panic!("expected DeleteMachine"),
    }
}

/// 📸️ Sanity: the stock fields (unrelated to the mutation vocabulary) still round-trip through
/// the artifact's DSL document codec.
#[semio_framework_async_macros::async_test]
async fn imported_mesh_stock_round_trips_document_dsl() {
    let stock_solid = brep_child_handle("stock", &brep_snapshot_for_working_solid(&WorkingSolid::ImportedMesh { mesh_url: "data:model/gltf-binary;base64,AAAA".into() }));
    let snapshot = Process3dSnapshot { stock_id: "stock".into(), stock_label: "Imported GLB".into(), stock_pose: Pose::default(), stock_solid, ..empty_process3d_snapshot() };
    store::os_store::test_support::assert_dsl_round_trip(&snapshot);
}
