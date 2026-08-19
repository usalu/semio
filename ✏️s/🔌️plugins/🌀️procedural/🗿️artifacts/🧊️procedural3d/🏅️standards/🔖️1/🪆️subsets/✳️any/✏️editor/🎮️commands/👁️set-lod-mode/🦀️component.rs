//! 👁️ 👁️ Procedural3d play app commands command — `set-lod-mode`.

use crate::editor::procedural3d::config::{Procedural3dConfig, Procedural3dConfigMutation};
use crate::artifacts::procedural3d::op::Procedural3dMutation;
use crate::artifacts::procedural3d::Procedural3dSnapshot;
use flow::FlowEvalSession;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "lod-mode")]
pub struct SetLodMode {
    pub value: String}

pub async fn handle(payload: &SetLodMode, _doc: &ArtifactView<'_, Procedural3dSnapshot>, _cfg: &ConfigView<'_, Procedural3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Procedural3dMutation, Procedural3dConfigMutation>, Fault> {
    Ok(Emit::config(vec![Procedural3dConfigMutation::SetLodMode { value: payload.value.clone() }]))
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::procedural3d::testkit::{app, app_with_registry, dispatch};
    use crate::editor::procedural3d::Procedural3dCommand;
    use crate::editor::procedural3d::commands::set_active_utility;

    #[test]
    async fn set_lod_mode_is_a_view_action_with_no_artifact_mutations() {
        let _serial = crate::editor::procedural3d::test_support::lock();
        let mut app = app();
        let before = app.snapshot().expect("snapshot");
        dispatch(&mut app, Procedural3dCommand::SetLodMode(SetLodMode { value: "wireframe".into() }));
        assert_eq!(app.snapshot().expect("snapshot"), before, "setLodMode must not mutate the document");
    }

    #[test]
    async fn set_active_utility_switch_clears_scratch_and_emits_no_operations() {
        let _serial = crate::editor::procedural3d::test_support::lock();
        let mut app = app_with_registry();
        let before = app.snapshot().expect("snapshot");
        let result = app.dispatch_typed(Procedural3dCommand::SetActiveUtility(set_active_utility::SetActiveUtility { utility_id: "rotate".into() }), &semio_framework_plugin::testkit::meta("local")).expect("switch utility");
        assert!(result.mutations.is_empty(), "utility switching never emits document operations");
        assert_eq!(app.snapshot().expect("snapshot"), before, "utility switching records no history entry");
    }
}
//#endregion 🧪️Tests
