//! 🗂️ 🗂️ Flow play app commands command — `delete-selection`.

use crate::artifacts::flow::{op::FlowMutation, FlowSnapshot};
use crate::editor::flow::config::{FlowConfig, FlowConfigMutation};
use crate::editor::flow::{flow_graph_selection_domains, host_operations, sync_host_selection_domains, FLOW_INTERACTION_GRAPH};
use flow::FlowEvalSession;
use semio_framework_plugin::{app::InteractionView, ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
pub struct DeleteSelection {}

/// 🕹️ `app_commands!`'s generated `dispatch(doc, cfg, session)` is framework-fixed at this exact 4-arg
/// shape (no `interaction` slot — ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM), so it
/// still requires a `handle` of this signature to exist even though it is reachable only through that
/// macro-generated path (`FlowPlayApp::handle` always routes this command through `apply` below
/// instead) — degrades to treating the selection as empty, mirroring `space::delete_selection::handle`.
pub async fn handle(_payload: &DeleteSelection, _doc: &ArtifactView<'_, FlowSnapshot>, _cfg: &ConfigView<'_, FlowConfig>, _session: &mut FlowEvalSession) -> Result<Emit<FlowMutation, FlowConfigMutation>, Fault> {
    Ok(Emit::default())
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: reads the "graph" domain's live
/// selection instead of the deleted `FlowConfig.selected_*` fields — no `SetSelection` config mutation
/// afterwards, the framework auto-prunes the deleted ids out of `graph`'s selection via
/// `interaction_topology`. `app_commands!`'s generated `dispatch(doc, cfg, session)` is framework-fixed
/// at that 3-arg shape (no `interaction` slot), so `FlowPlayApp::handle` routes this command through
/// `apply` directly instead (mirrors `space`'s `delete_selection::apply`).
pub async fn apply(_payload: &DeleteSelection, doc: &ArtifactView<'_, FlowSnapshot>, cfg: &ConfigView<'_, FlowConfig>, session: &mut FlowEvalSession, interaction: &InteractionView<'_>) -> Result<Emit<FlowMutation, FlowConfigMutation>, Fault> {
    let (nodes, edges) = flow_graph_selection_domains(&interaction.selection(FLOW_INTERACTION_GRAPH).ids);
    let operations = host_operations(doc.snapshot, cfg.snapshot, session, |host| {
        sync_host_selection_domains(host, &nodes, &edges, &[]);
        if !host.has_selection() {
            return false;
        }
        host.delete_selection().is_ok()
    });
    Ok(Emit::mutations(operations))
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::flow::testkit::{dispatch, dispatch_with_registry, flow_app_with_registry, select_graph};
    use crate::editor::flow::{FlowCommand, FLOW_PLAY_BODY_MAIN};

    #[semio_framework_async_macros::async_test]
    async fn delete_selection_deletes_the_widgets_picked_via_interaction_select() {
        let mut app = flow_app_with_registry();
        select_graph(&mut app, &["slider"], &[]);
        let result = dispatch(&mut app, FlowCommand::DeleteSelection(DeleteSelection {}));
        assert!(!result.mutations.is_empty(), "deleteSelection must emit operations for a picked widget");
        assert!(!app.snapshot().expect("snapshot").to_fixture().widgets.iter().any(|widget| crate::artifacts::flow::schema::widget_id(widget) == "slider"), "slider must be deleted");
    }

    #[semio_framework_async_macros::async_test]
    async fn delete_selection_action_removes_selected_synapses() {
        let mut app = flow_app_with_registry();
        let before = app.snapshot().expect("snapshot").to_fixture().synapses.len();
        select_graph(&mut app, &[], &["s1"]);
        let result = dispatch_with_registry(&mut app, FlowCommand::DeleteSelection(DeleteSelection {}));
        let after = app.snapshot().expect("snapshot").to_fixture();
        assert!(!result.mutations.is_empty(), "deleteSelection must emit operations for an edge");
        assert!(!after.synapses.iter().any(|synapse| synapse.id == "s1"), "synapse s1 must be removed");
        assert_eq!(after.synapses.len(), before - 1);
    }

    /// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `contextMenuAt` no longer sets
    /// selection (framework-owned now) — kept only because the shared `NodeGraph` canvas renderer
    /// (framework layer, unmigrated this wave) still dispatches it on right-click; a blank id (or any
    /// id) is a genuine no-operation.
    #[semio_framework_async_macros::async_test]
    async fn context_menu_at_is_a_no_operation() {
        use crate::editor::flow::commands::context_menu_at;
        let mut app = flow_app_with_registry();
        let result = dispatch(&mut app, FlowCommand::ContextMenuAt(context_menu_at::ContextMenuAt { id: String::new() }));
        assert!(result.mutations.is_empty());
        assert!(!crate::editor::flow::testkit::render(&mut app, FLOW_PLAY_BODY_MAIN).contains(r#""selection":["#));
    }
}
//#endregion 🧪️Tests
