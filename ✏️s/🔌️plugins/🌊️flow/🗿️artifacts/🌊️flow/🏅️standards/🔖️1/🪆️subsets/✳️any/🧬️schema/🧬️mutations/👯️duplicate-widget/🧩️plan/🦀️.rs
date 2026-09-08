//! 🧩️ Plan body for `duplicate-widget`: calls `create-widget` for the copy, then `connect-widgets`
//! to wire it to its source — the exact leaf kinds it composes, over the SAME shared `Planner` so
//! `fold_plan_diff`/`fold_plan_inverse` see one continuous local-step sequence.
use crate::schema::mutations::connect_widgets::ConnectWidgets;
use crate::schema::mutations::create_widget::CreateWidget;
use crate::schema::mutations::FlowMutation;
use crate::schema::widget_with_id;
use crate::{flow_working_scene, FlowSnapshot};
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
