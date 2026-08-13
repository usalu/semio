//! 🕸️ 🕸️ Procedural2d play app commands command — `connect-media-ports`.

use crate::apps::procedural2d::config::{Procedural2dConfig, Procedural2dConfigMutation};
use crate::artifacts::procedural2d::schema::host_operations;
use crate::artifacts::procedural2d::op::Procedural2dMutation;
use crate::artifacts::procedural2d::Procedural2dSnapshot;
use flow::{CameraJson, FlowEvalSession, FlowFixture};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "connect-media-ports")]
pub struct ConnectMediaPorts {
    pub source_node_id: String,
    pub source_port_id: String,
    pub target_node_id: String,
    pub target_port_id: String}

pub fn handle(payload: &ConnectMediaPorts, doc: &ArtifactView<'_, Procedural2dSnapshot>, _cfg: &ConfigView<'_, Procedural2dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Procedural2dMutation, Procedural2dConfigMutation>, Fault> {
    let fixture = &doc.snapshot.fixture;
    Ok(Emit::mutations(host_operations(fixture, |host| {
        let _ = host.connect_ports(&payload.source_node_id, &payload.source_port_id, &payload.target_node_id, &payload.target_port_id);
    })))
}
