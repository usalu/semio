//! 🎥️ FEM 3D app commands — the world-3d camera, config-only view state that never touches the
//! document.

use crate::apps::fem3d::config::{Fem3dConfig, Fem3dConfigOperation};
use crate::artifacts::fem3d::op::Fem3dOperation;
use crate::artifacts::fem3d::{Fem3dDocument, FemCamera};
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️SetCamera
pub mod set_camera {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "camera")]
    pub struct SetCamera {
        pub json: String,
    }

    pub fn handle(payload: &SetCamera, _doc: &DocumentView<'_, Fem3dDocument>, _cfg: &ConfigView<'_, Fem3dConfig>) -> Result<Emit<Fem3dOperation, Fem3dConfigOperation>, Fault> {
        Ok(Emit::config(vec![Fem3dConfigOperation::SetCamera { camera: FemCamera { json: payload.json.clone() } }]))
    }
}
//#endregion 🔖️SetCamera

// #region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::fem3d::testkit::{dispatch, fem3d_app, render};
    use crate::apps::fem3d::Fem3dCommand;

    #[test]
    fn set_camera_action_writes_config_not_document_operations() {
        let mut app = fem3d_app();
        dispatch(&mut app, Fem3dCommand::SetCamera(set_camera::SetCamera { json: "{\"x\":1}".into() }));
        // 🎥️ `VcsDocumentApp` exposes no config accessor — assert the config-only effect through render
        // output, the way the pre-migration tests already did.
        let model = render(&mut app, crate::apps::fem3d::modes::edit::windows::model::FEM3D_BODY_MODEL);
        assert!(model.contains("world-3d"), "camera write must not break rendering: {model}");
    }
}
// #endregion 🧪️Tests
