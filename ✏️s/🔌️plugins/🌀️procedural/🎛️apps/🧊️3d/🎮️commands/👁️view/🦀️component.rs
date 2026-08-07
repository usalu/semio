//! 👁️ Procedural3d play app commands — LOD/show-mode display toggles, the preview camera, and the
//! transform-gumball active-utility switch (all config-only; never document operations).

use crate::apps::procedural3d::config::{Procedural3dConfig, Procedural3dConfigOperation, Procedural3dPreviewCamera};
use crate::artifacts::procedural3d::op::Procedural3dOperation;
use crate::artifacts::procedural3d::Procedural3dDocument;
use flow::FlowEvalSession;
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️SetLodMode
pub mod set_lod_mode {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "lod-mode")]
    pub struct SetLodMode {
        pub value: String,
    }

    pub fn handle(payload: &SetLodMode, _doc: &DocumentView<'_, Procedural3dDocument>, _cfg: &ConfigView<'_, Procedural3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Procedural3dOperation, Procedural3dConfigOperation>, Fault> {
        Ok(Emit::config(vec![Procedural3dConfigOperation::SetLodMode { value: payload.value.clone() }]))
    }
}
//#endregion 🔖️SetLodMode

//#region 🔖️SetShowMode
pub mod set_show_mode {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "show-mode")]
    pub struct SetShowMode {
        pub value: String,
    }

    pub fn handle(payload: &SetShowMode, _doc: &DocumentView<'_, Procedural3dDocument>, _cfg: &ConfigView<'_, Procedural3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Procedural3dOperation, Procedural3dConfigOperation>, Fault> {
        Ok(Emit::config(vec![Procedural3dConfigOperation::SetShowMode { value: payload.value.clone() }]))
    }
}
//#endregion 🔖️SetShowMode

//#region 🔖️SetCamera
pub mod set_camera {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "camera")]
    pub struct SetCamera {
        #[dsl(block)]
        pub camera: Procedural3dPreviewCamera,
    }

    pub fn handle(payload: &SetCamera, _doc: &DocumentView<'_, Procedural3dDocument>, _cfg: &ConfigView<'_, Procedural3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Procedural3dOperation, Procedural3dConfigOperation>, Fault> {
        Ok(Emit::config(vec![Procedural3dConfigOperation::SetPreviewCamera { camera: payload.camera.clone() }]))
    }
}
//#endregion 🔖️SetCamera

//#region 🔖️SetActiveUtility
pub mod set_active_utility {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "active-utility")]
    pub struct SetActiveUtility {
        pub utility_id: String,
    }

    /// 🧰️ Host-owned active-utility switch — clears in-progress hover scratch, never emits document
    /// operations.
    pub fn handle(payload: &SetActiveUtility, _doc: &DocumentView<'_, Procedural3dDocument>, _cfg: &ConfigView<'_, Procedural3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Procedural3dOperation, Procedural3dConfigOperation>, Fault> {
        Ok(Emit::config(vec![Procedural3dConfigOperation::SetActiveUtility { utility_id: payload.utility_id.clone() }, Procedural3dConfigOperation::SetHover { node_id: None }]))
    }
}
//#endregion 🔖️SetActiveUtility

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::procedural3d::testkit::{app, app_with_registry, dispatch};
    use crate::apps::procedural3d::Procedural3dCommand;

    #[test]
    fn set_lod_mode_is_a_view_action_with_no_document_operations() {
        let _serial = crate::artifacts::procedural3d::engine::test_support::lock();
        let mut app = app();
        let before = app.projection().expect("projection");
        dispatch(&mut app, Procedural3dCommand::SetLodMode(set_lod_mode::SetLodMode { value: "wireframe".into() }));
        assert_eq!(app.projection().expect("projection"), before, "setLodMode must not mutate the document");
    }

    #[test]
    fn set_active_utility_switch_clears_scratch_and_emits_no_operations() {
        let _serial = crate::artifacts::procedural3d::engine::test_support::lock();
        let mut app = app_with_registry();
        let before = app.projection().expect("projection");
        let result = app.dispatch_typed(Procedural3dCommand::SetActiveUtility(set_active_utility::SetActiveUtility { utility_id: "rotate".into() }), &semio_framework_plugin::testkit::meta("local")).expect("switch utility");
        assert!(result.operations.is_empty(), "utility switching never emits document operations");
        assert_eq!(app.projection().expect("projection"), before, "utility switching records no history entry");
    }
}
//#endregion 🧪️Tests
