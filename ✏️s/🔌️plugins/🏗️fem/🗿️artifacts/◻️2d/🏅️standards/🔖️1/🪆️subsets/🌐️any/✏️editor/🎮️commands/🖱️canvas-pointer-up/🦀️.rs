//! 🖱️ Fem2d play app command — `canvas-pointer-up`: viewport pointer release.

use crate::standards::v1::subsets::any::schema::mutations::text::Fem2dMutation;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use semio_framework_value_derive::{FromValue, ToValue};

type Fem2dSnapshot = crate::Fem2dSnapshot;

//#region 🔖️CanvasPointerUp
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "canvas-pointer-up")]
pub struct CanvasPointerUp {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub shift: bool,
    pub ctrl: bool,
    pub meta: bool,
    pub alt: bool,
}

pub fn handle(_payload: &CanvasPointerUp, _doc: &ArtifactView<'_, Fem2dSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Fem2dMutation, NoConfigMutation>, Fault> {
    Err(Fault::from("fem2d.canvas-pointer-up.window-context-required"))
}

/// 🖱️ The pick is already committed on press and this editor has no drag gesture, so a release is a
/// deliberate no-op — declared rather than dropped so the host's pointer triple stays symmetric.
pub fn handle_window(_payload: &CanvasPointerUp, _doc: &ArtifactView<'_, Fem2dSnapshot>, _cfg: &ConfigView<'_, NoConfig>, _view: &semio_framework_plugin::ViewModel) -> Result<Emit<Fem2dMutation, NoConfigMutation>, Fault> {
    Ok(Emit::default())
}
//#endregion 🔖️CanvasPointerUp
