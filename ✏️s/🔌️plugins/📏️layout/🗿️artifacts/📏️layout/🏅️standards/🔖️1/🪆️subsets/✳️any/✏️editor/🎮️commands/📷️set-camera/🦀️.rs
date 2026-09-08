//! 🖱️ 🖱️ Layout play app commands command — `set-camera`.

use crate::mutations::LayoutMutation;
use crate::{LayoutCamera, LayoutSnapshot};
use crate::editor::layout::config::LayoutConfig;
use crate::editor::layout::config::LayoutConfigMutation;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Shared
/// 🖱️ A surface id names its blueprint/preview surface directly (`"layout.play.blueprint"` /
/// `"layout.play.preview"`); an absent id defaults to blueprint (the interactive authoring surface).
fn surface_is_blueprint(surface_id: Option<&str>) -> bool {
    surface_id.is_none_or(|surface| surface.contains("blueprint"))
}

//#endregion 🔖️Shared

//#region 🔖️CanvasPointerDown
//#endregion 🔖️CanvasPointerDown

//#region 🔖️CanvasPointerMove
//#endregion 🔖️CanvasPointerMove

//#region 🔖️CanvasPointerUp
//#endregion 🔖️CanvasPointerUp

//#region 🔖️CanvasDragOver
//#endregion 🔖️CanvasDragOver

//#region 🔖️CanvasDragLeave
//#endregion 🔖️CanvasDragLeave

//#region 🔖️SetCamera
//#endregion 🔖️SetCamera

//#region 🔖️CanvasDrop
//#endregion 🔖️CanvasDrop

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
pub struct SetCamera {
    pub surface_id: Option<String>,
    #[dsl(block)]
    pub camera: LayoutCamera,
}

pub fn handle(payload: &SetCamera, _doc: &ArtifactView<'_, LayoutSnapshot>, cfg: &ConfigView<'_, LayoutConfig>) -> Result<Emit<LayoutMutation, LayoutConfigMutation>, Fault> {
    let blueprint = surface_is_blueprint(payload.surface_id.as_deref());
    let _ = cfg;
    if blueprint {
        Ok(Emit::config(vec![LayoutConfigMutation::SetCamera(crate::editor::layout::config::SetCamera { camera: payload.camera.clone() })]))
    } else {
        Ok(Emit::config(vec![LayoutConfigMutation::SetPreviewCamera(crate::editor::layout::config::SetPreviewCamera { camera: payload.camera.clone() })]))
    }
}
