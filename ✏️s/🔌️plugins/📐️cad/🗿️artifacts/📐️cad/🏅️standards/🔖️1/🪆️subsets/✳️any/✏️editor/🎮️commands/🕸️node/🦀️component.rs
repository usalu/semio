//! 🕸️ CAD play app commands — the scene's node tree: create and rename.

use crate::artifacts::cad::mutations::create_node::mutation::CreateNode as CreateNodeMutation;
use crate::artifacts::cad::mutations::rename_node::mutation::RenameNode as RenameNodeMutation;
use crate::artifacts::cad::op::CadMutation;
use crate::artifacts::cad::standards::v1::subsets::any::schema::inferences::next_cad_id;
use crate::artifacts::cad::CadNode;
use crate::artifacts::cad::CadSnapshot;
use crate::editor::cad::config::{CadConfig, CadConfigMutation};
use crate::editor::cad::CadDispatchCtx;
use crate::editor::cad::{runtime_of, snapshot_of};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️AddNode
pub mod add_node {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "add-node")]
    pub struct AddNode {
        pub kind: String,
    }

    pub async fn handle(payload: &AddNode, doc: &ArtifactView<'_, CadSnapshot>, cfg: &ConfigView<'_, CadConfig>, _ctx: &mut CadDispatchCtx) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
        let document = doc.snapshot;
        let mut runtime = runtime_of(cfg);
        let id = next_cad_id("node");
        let label = format!("Node {}", document.nodes.len() + 1);
        let node = CadNode { id: id.clone(), label, kind: payload.kind.clone() };
        runtime.selected_node_ids = vec![id];
        let mut emit = Emit::mutations(vec![CadMutation::CreateNode(CreateNodeMutation { node })]);
        emit.config_mutations = vec![snapshot_of(&runtime, cfg.snapshot)?];
        Ok(emit)
    }
}
//#endregion 🔖️AddNode

//#region 🔖️RenameNode
pub mod rename_node {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "rename-node")]
    pub struct RenameNode {
        pub node_id: String,
        pub value: String,
    }

    pub async fn handle(payload: &RenameNode, _doc: &ArtifactView<'_, CadSnapshot>, _cfg: &ConfigView<'_, CadConfig>, _ctx: &mut CadDispatchCtx) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
        if payload.node_id.is_empty() || payload.value.is_empty() {
            return Ok(Emit::default());
        }
        Ok(Emit::mutations(vec![CadMutation::RenameNode(RenameNodeMutation { node_id: payload.node_id.clone(), new_label: payload.value.clone() })]))
    }
}
//#endregion 🔖️RenameNode

//#region 🔖️SetNodeSelection
/// 🕹️ FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM (26/08/14): document-tree node selection is
/// app-owned (not a mesh-geometry granularity of the framework `"cad"` interaction domain) —
/// relocated here (unchanged wire shape) from the deleted `🗂️selection` command directory.
pub mod set_node_selection {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "set-node-selection")]
    pub struct SetNodeSelection {
        pub node_ids: Vec<String>,
    }

    pub async fn handle(payload: &SetNodeSelection, _doc: &ArtifactView<'_, CadSnapshot>, cfg: &ConfigView<'_, CadConfig>, _ctx: &mut CadDispatchCtx) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
        let mut runtime = runtime_of(cfg);
        runtime.selected_node_ids = payload.node_ids.clone();
        Ok(Emit::config(vec![snapshot_of(&runtime, cfg.snapshot)?]))
    }
}
//#endregion 🔖️SetNodeSelection
