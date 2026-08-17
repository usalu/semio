//! 🕸️ 🕸️ Procedural3d play app commands command — `reorganize`.

use crate::editor::procedural3d::config::{Procedural3dConfig, Procedural3dConfigMutation};
use crate::artifacts::procedural3d::schema::{commit_fixture, host_from_fixture};
use crate::artifacts::procedural3d::op::Procedural3dMutation;
use crate::artifacts::procedural3d::Procedural3dSnapshot;
use flow::{CameraJson, FlowEvalSession, FlowFixture};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "reorganize")]
pub struct Reorganize {}

pub fn handle(_payload: &Reorganize, doc: &ArtifactView<'_, Procedural3dSnapshot>, _cfg: &ConfigView<'_, Procedural3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Procedural3dMutation, Procedural3dConfigMutation>, Fault> {
    let fixture = &doc.snapshot.fixture;
    let mut host = host_from_fixture(fixture);
    if host.reorganize(r#"{"orientation":"leftRight"}"#).is_ok() {
        Ok(Emit::mutations(commit_fixture(fixture, &host.fixture)))
    } else {
        Ok(Emit::default())
    }
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::procedural3d::testkit::{app, dispatch};
    use crate::editor::procedural3d::Procedural3dCommand;

    #[test]
    fn set_lod_mode_is_a_view_action_with_no_artifact_mutations_via_reorganize_baseline() {
        let _serial = crate::editor::procedural3d::test_support::lock();
        let mut app = app();
        let before = app.snapshot().expect("snapshot").fixture.widgets.len();
        dispatch(&mut app, Procedural3dCommand::Reorganize(Reorganize {}));
        assert_eq!(app.snapshot().expect("snapshot").fixture.widgets.len(), before);
    }
}
//#endregion 🧪️Tests
