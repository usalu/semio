//! 🕸️ 🕸️ Procedural3d play app commands command — `move-media-node`.

use crate::artifacts::procedural3d::op::Procedural3dMutation;
use crate::artifacts::procedural3d::schema::{commit_fixture, host_from_fixture};
use crate::artifacts::procedural3d::Procedural3dSnapshot;
use crate::editor::procedural3d::config::{Procedural3dConfig, Procedural3dConfigMutation};
use flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "move-node")]
pub struct MoveMediaNode {
    pub node_id: String,
    pub x: f64,
    pub y: f64,
}

pub fn handle(payload: &MoveMediaNode, doc: &ArtifactView<'_, Procedural3dSnapshot>, _cfg: &ConfigView<'_, Procedural3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Procedural3dMutation, Procedural3dConfigMutation>, Fault> {
    let fixture = &doc.snapshot.fixture;
    let mut host = host_from_fixture(fixture);
    if host.move_widget(&payload.node_id, payload.x, payload.y).is_ok() {
        Ok(Emit::mutations(commit_fixture(fixture, &host.fixture)))
    } else {
        Ok(Emit::default())
    }
}
