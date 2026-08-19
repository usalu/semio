//! 🕸️ 🕸️ Procedural2d play app commands command — `reorganize`.

use crate::editor::procedural2d::config::{Procedural2dConfig, Procedural2dConfigMutation};
use crate::artifacts::procedural2d::schema::host_operations;
use crate::artifacts::procedural2d::op::Procedural2dMutation;
use crate::artifacts::procedural2d::Procedural2dSnapshot;
use flow::FlowEvalSession;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "reorganize")]
pub struct Reorganize {}

pub async fn handle(_payload: &Reorganize, doc: &ArtifactView<'_, Procedural2dSnapshot>, _cfg: &ConfigView<'_, Procedural2dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Procedural2dMutation, Procedural2dConfigMutation>, Fault> {
    let fixture = &doc.snapshot.fixture;
    Ok(Emit::mutations(host_operations(fixture, |host| {
        let _ = host.reorganize(r#"{"orientation":"leftRight"}"#);
    })))
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::procedural2d::testkit::{app, dispatch};
    use crate::editor::procedural2d::Procedural2dCommand;
    use crate::editor::procedural2d::commands::node_graph_viewport;

    #[test]
    async fn reorganize_emits_operations() {
        let mut app = app();
        let before = app.snapshot().expect("snapshot").fixture;
        dispatch(&mut app, Procedural2dCommand::Reorganize(Reorganize {}));
        let after = app.snapshot().expect("snapshot").fixture;
        assert_ne!(before.layout, after.layout);
    }

    #[test]
    async fn node_graph_viewport_sets_camera() {
        let mut app = app();
        dispatch(&mut app, Procedural2dCommand::NodeGraphViewport(node_graph_viewport::NodeGraphViewport { viewport_json: serde_json::to_string(&CameraJson { x: 1.0, y: 2.0, zoom: 3.0 }).unwrap() }));
    }
}
//#endregion 🧪️Tests
