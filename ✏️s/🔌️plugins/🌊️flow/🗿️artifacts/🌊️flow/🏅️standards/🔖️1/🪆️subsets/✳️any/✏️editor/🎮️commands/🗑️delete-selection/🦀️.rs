//! 🗂️ 🗂️ Flow play app commands command — `delete-selection`.

use semio_framework_plugin::NoConfig;
use semio_framework_plugin::NoConfigMutation;
use crate::editor::flow::edit_rules::ContentEdit;
use crate::editor::flow::{flow_composed_content, flow_content_leaves_emit, flow_graph_selection_domains, FLOW_INTERACTION_GRAPH};
use crate::{FlowMutation, FlowSnapshot};
use flow::FlowEvalSession;
use semio_framework_plugin::{app::InteractionView, ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[derive(semio_framework_value::RetireOwned)]
pub struct DeleteSelection {}

/// 🕹️ `app_commands!`'s generated `dispatch(doc, cfg, session)` is framework-fixed at this exact 4-arg
/// shape (no `interaction` slot — ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM), so it
/// still requires a `handle` of this signature to exist even though it is reachable only through that
/// macro-generated path (`FlowPlayApp::handle` always routes this command through `apply` below
/// instead) — degrades to treating the selection as empty, mirroring `space::delete_selection::handle`.
pub fn handle(_payload: &DeleteSelection, _doc: &ArtifactView<'_, FlowSnapshot>, _cfg: &ConfigView<'_, NoConfig>, _session: &mut FlowEvalSession) -> Result<Emit<FlowMutation, NoConfigMutation>, Fault> {
    Ok(Emit::default())
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: reads the "graph" domain's live
/// selection instead of the deleted `FlowMainWindowConfig.selected_*` fields — no `SetSelection` config mutation
/// afterwards, the framework auto-prunes the deleted ids out of `graph`'s selection via
/// `interaction_topology`. `app_commands!`'s generated `dispatch(doc, cfg, session)` is framework-fixed
/// at that 3-arg shape (no `interaction` slot), so `FlowPlayApp::handle` routes this command through
/// `apply` directly instead (mirrors `space`'s `delete_selection::apply`).
pub fn apply(_payload: &DeleteSelection, doc: &ArtifactView<'_, FlowSnapshot>, _cfg: &ConfigView<'_, NoConfig>, _session: &mut FlowEvalSession, interaction: &InteractionView<'_>) -> Result<Emit<FlowMutation, NoConfigMutation>, Fault> {
    let (nodes, edges) = flow_graph_selection_domains(&interaction.selection(FLOW_INTERACTION_GRAPH).ids);
    let composed = crate::flow_composed_snapshot(doc.snapshot, &doc.children)?;
    let mut edit = ContentEdit::new(flow_composed_content(&composed)?);
    if !edit.remove(&nodes, &edges) {
        return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("flow.delete-selection-empty"), "deleteSelection needs at least one selected widget or synapse"));
    }
    Ok(flow_content_leaves_emit(&composed.content.child_id, edit.leaves))
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
