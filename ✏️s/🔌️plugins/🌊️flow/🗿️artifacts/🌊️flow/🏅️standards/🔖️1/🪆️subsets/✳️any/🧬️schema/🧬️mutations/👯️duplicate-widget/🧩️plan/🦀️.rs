//! 🧩️ Plan body for `duplicate-widget`: calls `create-widget` for the copy, then `connect-widgets`
//! to wire it to its source — the exact leaf kinds it composes, over the SAME shared `Planner` so
//! `fold_plan_diff`/`fold_plan_inverse` see one continuous local-step sequence.
use crate::artifacts::flow::schema::mutations::connect_widgets::mutation::ConnectWidgets;
use crate::artifacts::flow::schema::mutations::create_widget::mutation::CreateWidget;
use crate::artifacts::flow::schema::mutations::FlowMutation;
use crate::artifacts::flow::schema::widget_with_id;
use crate::artifacts::flow::{flow_working_scene, FlowSnapshot};
use protocol::{Identified, PlanError, Planner};

use super::mutation::DuplicateWidget;

//#region 🧩️Plan
pub fn plan(payload: &DuplicateWidget, base: &FlowSnapshot, planner: &mut Planner<FlowSnapshot, FlowMutation>) -> Result<(), PlanError> {
    precondition(payload, base).map_err(PlanError::Invalid)?;
    let scene = flow_working_scene(base);
    let source = scene.widgets.iter().find(|widget| widget.id() == &payload.source_id).expect("precondition confirmed source_id is present");
    let copy = widget_with_id(source, payload.new_id.clone());
    planner.call(FlowMutation::CreateWidget(CreateWidget { index: scene.widgets.len(), widget: copy }))?;

    let wired = flow_working_scene(planner.base());
    planner.call(FlowMutation::ConnectWidgets(ConnectWidgets {
        index: wired.synapses.len(),
        id: payload.synapse_id.clone(),
        from: payload.source_id.clone(),
        from_port: payload.from_port.clone(),
        to: payload.new_id.clone(),
        to_port: payload.to_port.clone(),
    }))?;
    Ok(())
}

/// ✅️ Shared by `plan` (mapped to a typed `PlanError`, so a direct `Planner::call`/`plan_of` caller
/// never panics on bad input) and `CompositeMutationKind::validate` (the `ArtifactStore::dispatch`
/// pre-check every mutation gets before it is even encoded).
pub fn precondition(payload: &DuplicateWidget, base: &FlowSnapshot) -> Result<(), String> {
    if payload.source_id == payload.new_id {
        return Err("duplicate-widget: new_id must differ from source_id".into());
    }
    let scene = flow_working_scene(base);
    if !scene.widgets.iter().any(|widget| widget.id() == &payload.source_id) {
        return Err(format!("duplicate-widget: source widget \"{}\" not found", payload.source_id));
    }
    if scene.widgets.iter().any(|widget| widget.id() == &payload.new_id) {
        return Err(format!("duplicate-widget: id \"{}\" already taken", payload.new_id));
    }
    Ok(())
}
//#endregion 🧩️Plan

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use flow::Widget;
    use protocol::{fold_plan_diff, fold_plan_inverse, Mutation, MutationDiff};

    async fn base_with_source_widget() -> FlowSnapshot {
        let base = FlowSnapshot::default();
        let create = FlowMutation::CreateWidget(CreateWidget { index: 0, widget: Widget::InputNote { id: "note-1".into(), text: "hello".into() } });
        create.diff(&base).diff().apply(&base).expect("valid mutation diff")
    }

    async fn sample_payload() -> DuplicateWidget {
        DuplicateWidget { source_id: "note-1".into(), new_id: "note-2".into(), synapse_id: "note-1-to-note-2".into(), from_port: "out".into(), to_port: "in".into() }
    }

    #[semio_framework_async_macros::async_test]
    async fn plan_folds_to_the_same_snapshot_as_applying_create_then_connect_by_hand() {
        let base = base_with_source_widget();
        let payload = sample_payload();

        let via_composite = fold_plan_diff(&payload, &base).diff().apply(&base).expect("valid mutation diff");

        let create = FlowMutation::CreateWidget(CreateWidget { index: 1, widget: Widget::InputNote { id: "note-2".into(), text: "hello".into() } });
        let after_create = create.diff(&base).diff().apply(&base).expect("valid mutation diff");
        let connect = FlowMutation::ConnectWidgets(ConnectWidgets { index: 0, id: "note-1-to-note-2".into(), from: "note-1".into(), from_port: "out".into(), to: "note-2".into(), to_port: "in".into() });
        let by_hand = connect.diff(&after_create).diff().apply(&after_create).expect("valid mutation diff");

        assert_eq!(via_composite, by_hand);
    }

    #[semio_framework_async_macros::async_test]
    async fn fold_plan_inverse_restores_base_exactly() {
        let base = base_with_source_widget();
        let payload = sample_payload();

        let forward = fold_plan_diff(&payload, &base).diff().apply(&base).expect("valid mutation diff");
        assert_ne!(forward, base, "the composite must actually change the snapshot");

        let inverses = fold_plan_inverse(&payload, &base);
        let restored = inverses.iter().fold(forward, |snapshot, inverse| inverse.diff(&snapshot).diff().apply(&snapshot).expect("valid mutation diff"));
        assert_eq!(restored, base);
    }

    #[semio_framework_async_macros::async_test]
    async fn precondition_rejects_a_missing_source_widget() {
        let base = FlowSnapshot::default();
        let error = precondition(&sample_payload(), &base).await.expect_err("note-1 does not exist yet");
        assert!(error.contains("note-1"));
    }

    #[semio_framework_async_macros::async_test]
    async fn precondition_rejects_a_new_id_already_taken() {
        let base = base_with_source_widget();
        let payload = DuplicateWidget { new_id: "note-1".into(), ..sample_payload() };
        let error = precondition(&payload, &base).await.expect_err("new_id collides with source_id");
        assert!(error.contains("differ"));
    }
}
//#endregion 🧪️Tests
