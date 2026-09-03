//! 👁️ 👁️ Generation3d play app commands command — `set-lod-mode`.

use crate::artifacts::generation3d::op::Generation3dMutation;
use crate::artifacts::generation3d::Generation3dSnapshot;
use crate::editor::generation3d::config::{Generation3dConfig, Generation3dConfigMutation};
use flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "lod-mode")]
pub struct SetLodMode {
    pub value: String,
}

pub fn handle(payload: &SetLodMode, _doc: &ArtifactView<'_, Generation3dSnapshot>, _cfg: &ConfigView<'_, Generation3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    Ok(Emit::config(vec![Generation3dConfigMutation::SetLodMode { value: payload.value.clone() }]))
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::generation3d::commands::set_active_utility;
    use crate::editor::generation3d::testkit::{app, app_with_registry, dispatch};
    use crate::editor::generation3d::Generation3dCommand;

    #[test]
    fn set_lod_mode_is_a_view_action_with_no_artifact_mutations() {
        let _serial = crate::editor::generation3d::test_support::lock();
        let mut app = app();
        let before = app.snapshot().expect("snapshot");
        dispatch(&mut app, Generation3dCommand::SetLodMode(SetLodMode { value: "wireframe".into() }));
        assert_eq!(app.snapshot().expect("snapshot"), before, "setLodMode must not mutate the document");
    }

    #[test]
    fn set_active_utility_switch_clears_scratch_and_emits_no_operations() {
        let _serial = crate::editor::generation3d::test_support::lock();
        let mut app = app_with_registry();
        let before = app.snapshot().expect("snapshot");
        let result = app.dispatch_typed(Generation3dCommand::SetActiveUtility(set_active_utility::SetActiveUtility { utility_id: "rotate".into() }), &semio_framework_plugin::testkit::meta("local")).expect("switch utility");
        assert!(result.mutations.is_empty(), "utility switching never emits document operations");
        assert_eq!(app.snapshot().expect("snapshot"), before, "utility switching records no history entry");
    }
}
//#endregion 🧪️Tests
