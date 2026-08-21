//! 🌞️ 🌞️ Procedural3d play app commands command — `toggle-sun`.

use crate::artifacts::procedural3d::op::Procedural3dMutation;
use crate::artifacts::procedural3d::Procedural3dSnapshot;
use crate::editor::procedural3d::config::{Procedural3dConfig, Procedural3dConfigMutation};
use flow::FlowEvalSession;
use semio_framework_plugin::{apply_world3d_sun_action, ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "toggle-sun")]
pub struct ToggleSun {}

pub async fn handle(_payload: &ToggleSun, _doc: &ArtifactView<'_, Procedural3dSnapshot>, cfg: &ConfigView<'_, Procedural3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Procedural3dMutation, Procedural3dConfigMutation>, Fault> {
    let mut sun = cfg.snapshot.sun();
    apply_world3d_sun_action(&mut sun, "toggleSun", None);
    Ok(Emit::config(vec![Procedural3dConfigMutation::SetSun { json: serde_json::to_string(&sun).unwrap_or_default() }]))
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::procedural3d::testkit::{app, dispatch};
    use crate::editor::procedural3d::Procedural3dCommand;

    #[semio_framework_async_macros::async_test]
    async fn toggle_sun_never_mutates_the_document() {
        let _serial = crate::editor::procedural3d::test_support::lock();
        let mut app = app();
        let before = app.snapshot().expect("snapshot");
        dispatch(&mut app, Procedural3dCommand::ToggleSun(ToggleSun {}));
        assert_eq!(app.snapshot().expect("snapshot"), before, "toggleSun must not mutate the document");
    }
}
//#endregion 🧪️Tests
