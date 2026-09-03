//! 🕸️ 🕸️ Generation3d play app commands command — `reorganize`.

use crate::artifacts::generation3d::op::Generation3dMutation;
use crate::artifacts::generation3d::schema::{commit_fixture, host_from_fixture};
use crate::artifacts::generation3d::Generation3dSnapshot;
use crate::editor::generation3d::config::{Generation3dConfig, Generation3dConfigMutation};
use flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "reorganize")]
pub struct Reorganize {}

pub fn handle(_payload: &Reorganize, doc: &ArtifactView<'_, Generation3dSnapshot>, _cfg: &ConfigView<'_, Generation3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
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
    use crate::editor::generation3d::testkit::{app, dispatch};
    use crate::editor::generation3d::Generation3dCommand;

    #[semio_framework_async_macros::async_test]
    async fn set_lod_mode_is_a_view_action_with_no_artifact_mutations_via_reorganize_baseline() {
        let _serial = crate::editor::generation3d::test_support::lock();
        let mut app = app().await;
        let before = app.snapshot().expect("snapshot").fixture.widgets.len();
        dispatch(&mut app, Generation3dCommand::Reorganize(Reorganize {})).await;
        assert_eq!(app.snapshot().expect("snapshot").fixture.widgets.len(), before);
    }
}
//#endregion 🧪️Tests
