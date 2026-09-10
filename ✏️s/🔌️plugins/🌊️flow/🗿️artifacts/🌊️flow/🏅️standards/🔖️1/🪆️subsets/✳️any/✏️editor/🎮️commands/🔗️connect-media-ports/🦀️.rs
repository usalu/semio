//! 🔗️ 🔗️ Flow play app commands command — `connect-media-ports`.

use semio_framework_plugin::NoConfig;
use semio_framework_plugin::NoConfigMutation;
use crate::editor::flow::host_operations;
use crate::editor::flow::modes::edit::windows::main::config::FlowMainWindowConfig;
use crate::{op::FlowMutation, FlowSnapshot};
use flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
pub struct ConnectMediaPorts {
    pub source_node_id: String,
    pub source_port_id: String,
    pub target_node_id: String,
    pub target_port_id: String,
}

/// 🔗️ The connect document operations against an already-resolved window config — the one body the
/// batch `handle` below and the retained `FlowGraphOperationWork` route both run, so neither can
/// drift from the other (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
pub fn connect_operations(payload: &ConnectMediaPorts, snapshot: &FlowSnapshot, config: &FlowMainWindowConfig, session: &FlowEvalSession) -> Vec<FlowMutation> {
    host_operations(snapshot, config, session, |host| host.connect_ports(&payload.source_node_id, &payload.source_port_id, &payload.target_node_id, &payload.target_port_id).is_ok())
}

pub fn handle(payload: &ConnectMediaPorts, doc: &ArtifactView<'_, FlowSnapshot>, cfg: &ConfigView<'_, NoConfig>, session: &mut FlowEvalSession) -> Result<Emit<FlowMutation, NoConfigMutation>, Fault> {
    Ok(Emit::mutations(connect_operations(payload, doc.snapshot, &crate::editor::flow::modes::edit::windows::main::config::current(cfg), session)))
}
