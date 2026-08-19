//! 🕸️ 🕸️ Procedural2d play app commands command — `move-media-node`.

use crate::editor::procedural2d::config::{Procedural2dConfig, Procedural2dConfigMutation};
use crate::artifacts::procedural2d::schema::host_operations;
use crate::artifacts::procedural2d::op::Procedural2dMutation;
use crate::artifacts::procedural2d::Procedural2dSnapshot;
use flow::FlowEvalSession;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "move-media-node")]
pub struct MoveMediaNode {
    pub node_id: String,
    pub x: f64,
    pub y: f64}

pub async fn handle(payload: &MoveMediaNode, doc: &ArtifactView<'_, Procedural2dSnapshot>, _cfg: &ConfigView<'_, Procedural2dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Procedural2dMutation, Procedural2dConfigMutation>, Fault> {
    let fixture = &doc.snapshot.fixture;
    Ok(Emit::mutations(host_operations(fixture, |host| {
        let _ = host.move_widget(&payload.node_id, payload.x, payload.y);
    })))
}
