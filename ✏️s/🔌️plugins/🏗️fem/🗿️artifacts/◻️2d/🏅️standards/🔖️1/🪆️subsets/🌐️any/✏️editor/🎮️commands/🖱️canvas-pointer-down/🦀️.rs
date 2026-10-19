//! 🖱️ Fem2d play app command — `canvas-pointer-down`: viewport pick — hit-tests the addressed model/results window and requests the framework `interactionSelect`.

use semio_s_artifact_fem_2d_editor_interaction::editor::fem2d::interaction::canvas_gesture;
use semio_s_artifact_fem_2d_editor_interaction::editor::fem2d::interaction::selection_merge_mode;
use crate::editor::fem2d::modes::edit::fem2d_addressed_body_key;
use crate::editor::fem2d::modes::edit::fem2d_addressed_camera;
use semio_s_artifact_fem_2d_mutations::standards::v1::subsets::any::schema::mutations::Fem2dMutation;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::ConfigView;
use semio_framework_plugin::Emit;
use semio_framework_plugin::Fault;
use semio_framework_plugin::NoConfig;
use semio_framework_plugin::NoConfigMutation;
use semio_framework_value_derive::FromValue;
use semio_framework_value_derive::ToValue;

type Fem2dSnapshot = semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshot;

//#region 🔖️CanvasPointerDown
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "canvas-pointer-down")]
pub struct CanvasPointerDown {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub button: u32,
    pub shift: bool,
    pub ctrl: bool,
    pub meta: bool,
    pub alt: bool,
}

pub fn handle(_payload: &CanvasPointerDown, _doc: &ArtifactView<'_, Fem2dSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Fem2dMutation, NoConfigMutation>, Fault> {
    Err(Fault::from("fem2d.canvas-pointer-down.window-context-required"))
}

/// 🖱️ A primary-button press either starts a marquee/lasso drag or performs a direct pick, depending on the active selection utility.
pub fn handle_window(payload: &CanvasPointerDown, doc: &ArtifactView<'_, Fem2dSnapshot>, cfg: &ConfigView<'_, NoConfig>, view: &semio_framework_plugin::ViewModel) -> Result<Emit<Fem2dMutation, NoConfigMutation>, Fault> {
    let camera = fem2d_addressed_camera(cfg, view, "fem2d.canvas-pointer-down")?;
    let merge = selection_merge_mode(payload.shift, payload.ctrl, payload.meta);
    canvas_gesture::pointer_down(doc.snapshot, &camera, view, fem2d_addressed_body_key(view, "fem2d.canvas-pointer-down")?, payload.x, payload.y, payload.width, payload.height, payload.button, merge)
}
//#endregion 🔖️CanvasPointerDown

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
