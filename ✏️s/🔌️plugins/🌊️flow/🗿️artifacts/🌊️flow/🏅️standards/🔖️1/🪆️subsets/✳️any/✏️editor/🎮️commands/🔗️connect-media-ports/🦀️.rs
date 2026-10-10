//! 🔗️ 🔗️ Flow play app commands command — `connect-media-ports`.

use semio_framework_plugin::NoConfig;
use semio_framework_plugin::NoConfigMutation;
use crate::editor::flow::edit_rules::ContentEdit;
use crate::editor::flow::{flow_composed_content, flow_content_leaves_emit, host_from_snapshot};
use crate::editor::flow::modes::edit::windows::main::config::FlowMainWindowConfig;
use crate::{FlowMutation, FlowSnapshot};
use flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[derive(semio_framework_value::RetireOwned)]
pub struct ConnectMediaPorts {
    pub source_node_id: String,
    pub source_port_id: String,
    pub target_node_id: String,
    pub target_port_id: String,
}

/// 🔗️ The connect document operations against an already-resolved window config — the one body the
/// batch `handle` below and the retained `FlowGraphOperationWork` route both run, so neither can
/// drift from the other (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
/// 🔗️ Connects two ports of the composed scene through the host's own compatibility rules and publishes the new
/// synapse on the content child — refused by name when the host adds none.
pub fn connect_edit(payload: &ConnectMediaPorts, composed: &FlowSnapshot, config: &FlowMainWindowConfig, session: &FlowEvalSession) -> Result<Emit<FlowMutation, NoConfigMutation>, Fault> {
    let mut host = host_from_snapshot(composed, config, session);
    let connected = host.connect_ports(&payload.source_node_id, &payload.source_port_id, &payload.target_node_id, &payload.target_port_id);
    host.retire_cold();
    let Ok(id) = connected else {
        return Err(Fault::new(
            semio_framework_plugin::FaultOrigin::App,
            semio_framework_plugin::FaultCode::new("flow.connect-incompatible"),
            format!("connectMediaPorts cannot connect {}@{} to {}@{}", payload.source_node_id, payload.source_port_id, payload.target_node_id, payload.target_port_id),
        ));
    };
    let mut edit = ContentEdit::new(flow_composed_content(composed)?);
    edit.connect(semio_s_artifact_stdio_semio::standards::v1::subsets::flow::schema::snapshot::FlowEdge {
        id,
        from: semio_s_artifact_stdio_semio::standards::v1::subsets::flow::schema::snapshot::PortRef { node: payload.source_node_id.clone(), port: payload.source_port_id.clone() },
        to: semio_s_artifact_stdio_semio::standards::v1::subsets::flow::schema::snapshot::PortRef { node: payload.target_node_id.clone(), port: payload.target_port_id.clone() },
        kind: "data".into(),
    });
    Ok(flow_content_leaves_emit(&composed.content.child_id, edit.leaves))
}

pub fn handle(payload: &ConnectMediaPorts, doc: &ArtifactView<'_, FlowSnapshot>, cfg: &ConfigView<'_, NoConfig>, session: &mut FlowEvalSession) -> Result<Emit<FlowMutation, NoConfigMutation>, Fault> {
    connect_edit(payload, &crate::flow_composed_snapshot(doc.snapshot, &doc.children)?, &crate::editor::flow::modes::edit::windows::main::config::current(cfg), session)
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
