//! 🪟️ 🧩️ Flow play app commands command — `move-media-node`.

use semio_framework_plugin::NoConfig;
use semio_framework_plugin::NoConfigMutation;
use crate::editor::flow::commands::node_graph_edit::{self, FlowNodeGraphEditOp};
use crate::{op::FlowMutation, FlowSnapshot};
use flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
pub struct MoveMediaNode {
    pub node_id: String,
    pub x: f64,
    pub y: f64,
}

pub fn handle(payload: &MoveMediaNode, doc: &ArtifactView<'_, FlowSnapshot>, cfg: &ConfigView<'_, NoConfig>, session: &mut FlowEvalSession) -> Result<Emit<FlowMutation, NoConfigMutation>, Fault> {
    let operations = [FlowNodeGraphEditOp::Move { node_id: payload.node_id.clone(), x: payload.x, y: payload.y }];
    let mut emit = node_graph_edit::node_graph_edit_result(doc, &crate::editor::flow::modes::edit::windows::main::config::current(cfg), session, &operations, &[])?;
    emit.coalesce_key = (!emit.child_emits.is_empty()).then(|| format!("move-{}", payload.node_id));
    Ok(emit)
}
