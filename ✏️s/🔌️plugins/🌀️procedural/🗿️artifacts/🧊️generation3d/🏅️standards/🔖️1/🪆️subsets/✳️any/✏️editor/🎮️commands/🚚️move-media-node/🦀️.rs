//! 🕸️ 🕸️ Generation3d play app commands command — `move-media-node`.

use crate::editor::generation3d::config::{Generation3dConfig, Generation3dConfigMutation};
use crate::standards::v1::subsets::any::schema::mutations::text::Generation3dMutation;
use crate::standards::v1::subsets::any::schema::{commit_fixture, with_host};
use crate::Generation3dSnapshot;
use semio_framework_os_flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "move-node")]
pub struct MoveMediaNode {
    pub node_id: String,
    pub x: f64,
    pub y: f64,
}

pub fn handle(payload: &MoveMediaNode, doc: &ArtifactView<'_, Generation3dSnapshot>, _cfg: &ConfigView<'_, Generation3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    let fixture = &doc.snapshot.fixture;
    with_host(fixture, |host| {
        if host.move_widget(&payload.node_id, payload.x, payload.y).is_ok() {
            Ok(Emit::mutations(commit_fixture(fixture, &host.fixture)))
        } else {
            Ok(Emit::default())
        }
    })
}
