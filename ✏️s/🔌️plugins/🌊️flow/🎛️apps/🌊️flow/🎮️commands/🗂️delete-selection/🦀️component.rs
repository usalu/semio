//! 🗂️ 🗂️ Flow play app commands command — `delete-selection`.

use crate::apps::flow::config::{FlowConfig, FlowConfigMutation};
use crate::apps::flow::{focus_selection_camera, host_operations, sync_host_selection_domains};
use crate::artifacts::flow::schema::widget_id;
use crate::artifacts::flow::{op::FlowMutation, FlowSnapshot};
use flow::FlowEvalSession;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
pub struct DeleteSelection {}

pub fn handle(_payload: &DeleteSelection, doc: &ArtifactView<'_, FlowSnapshot>, cfg: &ConfigView<'_, FlowConfig>, session: &mut FlowEvalSession) -> Result<Emit<FlowMutation, FlowConfigMutation>, Fault> {
    let config = cfg.snapshot;
    let nodes = config.selected_node_ids.clone();
    let edges = config.selected_edge_ids.clone();
    let handles = config.selected_handle_ids.clone();
    let operations = host_operations(doc.snapshot, config, session, |host| {
        sync_host_selection_domains(host, &nodes, &edges, &handles);
        if !host.has_selection() {
            return false;
        }
        host.delete_selection().is_ok()
    });
    if operations.is_empty() {
        Ok(Emit::default())
    } else {
        Ok(Emit { artifact_mutations: operations, config_mutations: vec![FlowConfigMutation::SetSelection { node_ids: Vec::new(), edge_ids: Vec::new(), handle_ids: Vec::new() }], ..Default::default() })
    }
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::flow::testkit::{dispatch, dispatch_with_registry, flow_app, flow_app_with_registry, render};
    use crate::apps::flow::{FlowCommand, FLOW_PLAY_BODY_MAIN};

    #[test]
    fn selection_is_config_state_and_emits_no_artifact_mutations() {
        let mut app = flow_app();
        let result = dispatch(&mut app, FlowCommand::SetSelection(set_selection::SetSelection { ids: vec!["slider".into()], edge_ids: Vec::new(), handle_ids: Vec::new() }));
        assert!(result.mutations.is_empty(), "selection must not produce document operations");
    }

    #[test]
    fn select_all_and_focus_selection_update_scene() {
        let mut app = flow_app();
        dispatch(&mut app, FlowCommand::SelectAll(select_all::SelectAll {}));
        let selected = render(&mut app, FLOW_PLAY_BODY_MAIN);
        assert!(selected.contains("slider"));
        let before = selected;
        dispatch(&mut app, FlowCommand::FocusSelection(focus_selection::FocusSelection {}));
        let after = render(&mut app, FLOW_PLAY_BODY_MAIN);
        assert_ne!(before, after);
    }

    #[test]
    fn delete_selection_action_removes_selected_synapses() {
        let mut app = flow_app_with_registry();
        let before = app.snapshot().expect("snapshot").to_fixture().synapses.len();
        dispatch_with_registry(&mut app, FlowCommand::SetSelection(set_selection::SetSelection { ids: Vec::new(), edge_ids: vec!["s1".into()], handle_ids: Vec::new() }));
        let result = dispatch_with_registry(&mut app, FlowCommand::DeleteSelection(DeleteSelection {}));
        let after = app.snapshot().expect("snapshot").to_fixture();
        assert!(!result.mutations.is_empty(), "deleteSelection must emit operations for an edge");
        assert!(!after.synapses.iter().any(|synapse| synapse.id == "s1"), "synapse s1 must be removed");
        assert_eq!(after.synapses.len(), before - 1);
    }

    #[test]
    fn context_menu_at_with_a_blank_id_is_a_no_operation() {
        let mut app = flow_app();
        let result = dispatch(&mut app, FlowCommand::ContextMenuAt(context_menu_at::ContextMenuAt { id: String::new() }));
        assert!(result.mutations.is_empty());
        assert!(!render(&mut app, FLOW_PLAY_BODY_MAIN).contains(r#""selection":["#), "a blank id must leave the selection untouched (and empty)");
    }
}
//#endregion 🧪️Tests
