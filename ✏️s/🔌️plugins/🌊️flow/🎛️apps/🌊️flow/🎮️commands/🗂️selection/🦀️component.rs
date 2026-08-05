//! 🗂️ Flow play app commands — everything that only moves the selection.
//!
//! All of these are CONFIG-only (they were ephemeral `FlowPlayRuntime` fields before the typed-command
//! conversion): they emit `config_operations` and never document operations. The single exception is
//! `DeleteSelection`, which is a real document mutation and clears all three selection domains.

use crate::apps::flow::config::{FlowConfig, FlowConfigOperation};
use crate::artifacts::flow::engine::{focus_selection_camera, host_operations, sync_host_selection_domains, widget_id};
use crate::artifacts::flow::{op::FlowOperation, FlowFixture};
use flow_core::FlowEvalSession;
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️DeleteSelection
pub mod delete_selection {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "delete-selection")]
    pub struct DeleteSelection {}

    pub fn handle(_payload: &DeleteSelection, doc: &DocumentView<'_, FlowFixture>, cfg: &ConfigView<'_, FlowConfig>, session: &mut FlowEvalSession) -> Result<Emit<FlowOperation, FlowConfigOperation>, Fault> {
        let config = cfg.projection;
        let nodes = config.selected_node_ids.clone();
        let edges = config.selected_edge_ids.clone();
        let handles = config.selected_handle_ids.clone();
        let operations = host_operations(doc.projection, config, session, |host| {
            sync_host_selection_domains(host, &nodes, &edges, &handles);
            if !host.has_selection() {
                return false;
            }
            host.delete_selection().is_ok()
        });
        if operations.is_empty() {
            Ok(Emit::default())
        } else {
            Ok(Emit { document_operations: operations, config_operations: vec![FlowConfigOperation::SetSelection { node_ids: Vec::new(), edge_ids: Vec::new(), handle_ids: Vec::new() }], ..Default::default() })
        }
    }
}
//#endregion 🔖️DeleteSelection

//#region 🔖️SelectAll
pub mod select_all {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "select-all")]
    pub struct SelectAll {}

    pub fn handle(_payload: &SelectAll, doc: &DocumentView<'_, FlowFixture>, cfg: &ConfigView<'_, FlowConfig>, _session: &mut FlowEvalSession) -> Result<Emit<FlowOperation, FlowConfigOperation>, Fault> {
        let config = cfg.projection;
        let ids: Vec<String> = doc.projection.widgets.iter().map(widget_id).map(str::to_string).collect();
        Ok(Emit::config(vec![FlowConfigOperation::SetSelection { node_ids: ids, edge_ids: config.selected_edge_ids.clone(), handle_ids: config.selected_handle_ids.clone() }]))
    }
}
//#endregion 🔖️SelectAll

//#region 🔖️FocusSelection
pub mod focus_selection {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "focus-selection")]
    pub struct FocusSelection {}

    pub fn handle(_payload: &FocusSelection, doc: &DocumentView<'_, FlowFixture>, cfg: &ConfigView<'_, FlowConfig>, session: &mut FlowEvalSession) -> Result<Emit<FlowOperation, FlowConfigOperation>, Fault> {
        match focus_selection_camera(doc.projection, cfg.projection, session) {
            Some(camera) => Ok(Emit::config(vec![FlowConfigOperation::SetCamera { camera }])),
            None => Ok(Emit::default()),
        }
    }
}
//#endregion 🔖️FocusSelection

//#region 🔖️SetSelection
pub mod set_selection {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "set-selection")]
    pub struct SetSelection {
        pub ids: Vec<String>,
        pub edge_ids: Vec<String>,
        pub handle_ids: Vec<String>,
    }

    pub fn handle(payload: &SetSelection, _doc: &DocumentView<'_, FlowFixture>, _cfg: &ConfigView<'_, FlowConfig>, _session: &mut FlowEvalSession) -> Result<Emit<FlowOperation, FlowConfigOperation>, Fault> {
        Ok(Emit::config(vec![FlowConfigOperation::SetSelection { node_ids: payload.ids.clone(), edge_ids: payload.edge_ids.clone(), handle_ids: payload.handle_ids.clone() }]))
    }
}
//#endregion 🔖️SetSelection

//#region 🔖️SelectNode
pub mod select_node {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "select-node")]
    pub struct SelectNode {
        pub node_id: String,
    }

    pub fn handle(payload: &SelectNode, _doc: &DocumentView<'_, FlowFixture>, _cfg: &ConfigView<'_, FlowConfig>, _session: &mut FlowEvalSession) -> Result<Emit<FlowOperation, FlowConfigOperation>, Fault> {
        Ok(Emit::config(vec![FlowConfigOperation::SetSelection { node_ids: vec![payload.node_id.clone()], edge_ids: Vec::new(), handle_ids: Vec::new() }]))
    }
}
//#endregion 🔖️SelectNode

//#region 🔖️NodeGraphSelect
pub mod node_graph_select {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "node-graph-select")]
    pub struct NodeGraphSelect {
        pub node_ids: Vec<String>,
    }

    pub fn handle(payload: &NodeGraphSelect, _doc: &DocumentView<'_, FlowFixture>, _cfg: &ConfigView<'_, FlowConfig>, _session: &mut FlowEvalSession) -> Result<Emit<FlowOperation, FlowConfigOperation>, Fault> {
        Ok(Emit::config(vec![FlowConfigOperation::SetSelection { node_ids: payload.node_ids.clone(), edge_ids: Vec::new(), handle_ids: Vec::new() }]))
    }
}
//#endregion 🔖️NodeGraphSelect

//#region 🔖️ClearSelection
pub mod clear_selection {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "clear-selection")]
    pub struct ClearSelection {}

    pub fn handle(_payload: &ClearSelection, _doc: &DocumentView<'_, FlowFixture>, _cfg: &ConfigView<'_, FlowConfig>, _session: &mut FlowEvalSession) -> Result<Emit<FlowOperation, FlowConfigOperation>, Fault> {
        Ok(Emit::config(vec![FlowConfigOperation::SetSelection { node_ids: Vec::new(), edge_ids: Vec::new(), handle_ids: Vec::new() }]))
    }
}
//#endregion 🔖️ClearSelection

//#region 🔖️GraphPointerDown
pub mod graph_pointer_down {
    use super::*;

    /// 🖱️ A bare pointer-down on empty canvas — drops the NODE selection only.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "graph-pointer-down")]
    pub struct GraphPointerDown {}

    pub fn handle(_payload: &GraphPointerDown, _doc: &DocumentView<'_, FlowFixture>, cfg: &ConfigView<'_, FlowConfig>, _session: &mut FlowEvalSession) -> Result<Emit<FlowOperation, FlowConfigOperation>, Fault> {
        let config = cfg.projection;
        Ok(Emit::config(vec![FlowConfigOperation::SetSelection { node_ids: Vec::new(), edge_ids: config.selected_edge_ids.clone(), handle_ids: config.selected_handle_ids.clone() }]))
    }
}
//#endregion 🔖️GraphPointerDown

//#region 🔖️ContextMenuAt
pub mod context_menu_at {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "context-menu-at")]
    pub struct ContextMenuAt {
        pub id: String,
    }

    pub fn handle(payload: &ContextMenuAt, _doc: &DocumentView<'_, FlowFixture>, cfg: &ConfigView<'_, FlowConfig>, _session: &mut FlowEvalSession) -> Result<Emit<FlowOperation, FlowConfigOperation>, Fault> {
        if payload.id.is_empty() {
            return Ok(Emit::default());
        }
        let config = cfg.projection;
        Ok(Emit::config(vec![FlowConfigOperation::SetSelection { node_ids: vec![payload.id.clone()], edge_ids: config.selected_edge_ids.clone(), handle_ids: config.selected_handle_ids.clone() }]))
    }
}
//#endregion 🔖️ContextMenuAt

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::flow::testkit::{dispatch, dispatch_with_registry, flow_app, flow_app_with_registry, render};
    use crate::apps::flow::{FlowCommand, FLOW_PLAY_BODY_MAIN};

    #[test]
    fn selection_is_config_state_and_emits_no_document_operations() {
        let mut app = flow_app();
        let result = dispatch(&mut app, FlowCommand::SetSelection(set_selection::SetSelection { ids: vec!["slider".into()], edge_ids: Vec::new(), handle_ids: Vec::new() }));
        assert!(result.operations.is_empty(), "selection must not produce document operations");
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
        let before = app.projection().expect("projection").synapses.len();
        dispatch_with_registry(&mut app, FlowCommand::SetSelection(set_selection::SetSelection { ids: Vec::new(), edge_ids: vec!["s1".into()], handle_ids: Vec::new() }));
        let result = dispatch_with_registry(&mut app, FlowCommand::DeleteSelection(delete_selection::DeleteSelection {}));
        let after = app.projection().expect("projection");
        assert!(!result.operations.is_empty(), "deleteSelection must emit operations for an edge");
        assert!(!after.synapses.iter().any(|synapse| synapse.id == "s1"), "synapse s1 must be removed");
        assert_eq!(after.synapses.len(), before - 1);
    }

    #[test]
    fn context_menu_at_with_a_blank_id_is_a_no_operation() {
        let mut app = flow_app();
        let result = dispatch(&mut app, FlowCommand::ContextMenuAt(context_menu_at::ContextMenuAt { id: String::new() }));
        assert!(result.operations.is_empty());
        assert!(!render(&mut app, FLOW_PLAY_BODY_MAIN).contains(r#""selection":["#), "a blank id must leave the selection untouched (and empty)");
    }
}
//#endregion 🧪️Tests
