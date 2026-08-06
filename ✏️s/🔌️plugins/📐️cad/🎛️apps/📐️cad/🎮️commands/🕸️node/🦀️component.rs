//! 🕸️ CAD play app commands — the scene's node tree: create and rename.

use crate::apps::cad::config::{CadConfig, CadConfigOperation};
use crate::apps::cad::CadDispatchCtx;
use crate::artifacts::cad::op::CadOperation;
use crate::artifacts::cad::CadProjection;
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};
use crate::apps::cad::{runtime_of, snapshot_of};
use crate::artifacts::cad::engine::next_cad_id;
use crate::artifacts::cad::CadNode;


//#region 🔖️AddNode
pub mod add_node {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "add-node")]
    pub struct AddNode {
        pub kind: String,
    }

    pub fn handle(payload: &AddNode, doc: &DocumentView<'_, CadProjection>, cfg: &ConfigView<'_, CadConfig>, _ctx: &mut CadDispatchCtx<'_>) -> Result<Emit<CadOperation, CadConfigOperation>, Fault> {
        let document = doc.projection;
        let mut runtime = runtime_of(cfg);
        let id = next_cad_id("node");
        let label = format!("Node {}", document.nodes.len() + 1);
        let node = CadNode { id: id.clone(), label, kind: payload.kind.clone() };
        runtime.selected_node_ids = vec![id];
        let mut emit = Emit::operations(vec![CadOperation::AddNode { node }]);
        emit.config_operations = vec![snapshot_of(&runtime, cfg.projection)];
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

    pub fn handle(payload: &RenameNode, _doc: &DocumentView<'_, CadProjection>, _cfg: &ConfigView<'_, CadConfig>, _ctx: &mut CadDispatchCtx<'_>) -> Result<Emit<CadOperation, CadConfigOperation>, Fault> {
        if payload.node_id.is_empty() || payload.value.is_empty() {
            return Ok(Emit::default());
        }
        Ok(Emit::operations(vec![CadOperation::RenameNode { node_id: payload.node_id.clone(), label: payload.value.clone() }]))
    }
}
//#endregion 🔖️RenameNode
