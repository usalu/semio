//! 🕸️ 🕸️ Generation2d play app commands command — `connect-media-ports`.

use crate::artifacts::generation2d::op::Generation2dMutation;
use crate::artifacts::generation2d::schema::host_operations;
use crate::artifacts::generation2d::Generation2dSnapshot;
use crate::editor::generation2d::config::{Generation2dConfig, Generation2dConfigMutation};
use flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "connect-media-ports")]
pub struct ConnectMediaPorts {
    pub source_node_id: String,
    pub source_port_id: String,
    pub target_node_id: String,
    pub target_port_id: String,
}

pub fn handle(payload: &ConnectMediaPorts, doc: &ArtifactView<'_, Generation2dSnapshot>, _cfg: &ConfigView<'_, Generation2dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Generation2dMutation, Generation2dConfigMutation>, Fault> {
    let fixture = &doc.snapshot.fixture;
    Ok(Emit::mutations(host_operations(fixture, |host| {
        let _ = host.connect_ports(&payload.source_node_id, &payload.source_port_id, &payload.target_node_id, &payload.target_port_id);
    })))
}
