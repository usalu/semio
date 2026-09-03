//! 🌞️ 🌞️ Generation3d play app commands command — `toggle-sun`.

use crate::artifacts::generation3d::op::Generation3dMutation;
use crate::artifacts::generation3d::Generation3dSnapshot;
use crate::editor::generation3d::config::{Generation3dConfig, Generation3dConfigMutation};
use flow::FlowEvalSession;
use semio_framework_plugin::{apply_world3d_sun_action, ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "toggle-sun")]
pub struct ToggleSun {}

pub fn handle(_payload: &ToggleSun, _doc: &ArtifactView<'_, Generation3dSnapshot>, cfg: &ConfigView<'_, Generation3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    let mut sun = cfg.snapshot.sun();
    apply_world3d_sun_action(&mut sun, "toggleSun", None);
    Ok(Emit::config(vec![Generation3dConfigMutation::SetSun { json: dsl::json::to_json_string(&sun) }]))
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::generation3d::testkit::{app, dispatch};
    use crate::editor::generation3d::Generation3dCommand;

    #[test]
    fn toggle_sun_never_mutates_the_document() {
        let _serial = crate::editor::generation3d::test_support::lock();
        let mut app = app();
        let before = app.snapshot().expect("snapshot");
        dispatch(&mut app, Generation3dCommand::ToggleSun(ToggleSun {}));
        assert_eq!(app.snapshot().expect("snapshot"), before, "toggleSun must not mutate the document");
    }
}
//#endregion 🧪️Tests
