//! 🕸️ CAD play app commands — the scene's node tree: create and rename.

use crate::apps::cad::config::{CadConfig, CadConfigMutation};
use crate::apps::cad::CadDispatchCtx;
use crate::artifacts::cad::mutations::create_node::mutation::CreateNode as CreateNodeMutation;
use crate::artifacts::cad::mutations::rename_node::mutation::RenameNode as RenameNodeMutation;
use crate::artifacts::cad::op::CadMutation;
use crate::artifacts::cad::CadSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};
use crate::apps::cad::{runtime_of, snapshot_of};
use crate::artifacts::cad::standards::v1::subsets::any::schema::inferences::next_cad_id;
use crate::artifacts::cad::CadNode;


//#region 🔖️AddNode
pub mod add_node {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "add-node")]
    pub struct AddNode {
        pub kind: String,
    }

    pub fn handle(payload: &AddNode, doc: &ArtifactView<'_, CadSnapshot>, cfg: &ConfigView<'_, CadConfig>, _ctx: &mut CadDispatchCtx<'_>) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
        let document = doc.snapshot;
        let mut runtime = runtime_of(cfg);
        let id = next_cad_id("node");
        let label = format!("Node {}", document.nodes.len() + 1);
        let node = CadNode { id: id.clone(), label, kind: payload.kind.clone() };
        runtime.selected_node_ids = vec![id];
        let mut emit = Emit::mutations(vec![CadMutation::CreateNode(CreateNodeMutation { node })]);
        emit.config_mutations = vec![snapshot_of(&runtime, cfg.snapshot)];
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

    pub fn handle(payload: &RenameNode, _doc: &ArtifactView<'_, CadSnapshot>, _cfg: &ConfigView<'_, CadConfig>, _ctx: &mut CadDispatchCtx<'_>) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
        if payload.node_id.is_empty() || payload.value.is_empty() {
            return Ok(Emit::default());
        }
        Ok(Emit::mutations(vec![CadMutation::RenameNode(RenameNodeMutation { node_id: payload.node_id.clone(), new_label: payload.value.clone() })]))
    }
}
//#endregion 🔖️RenameNode
