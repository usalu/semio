//! 🖱️ Fem2d play app command — `canvas-pointer-up`: viewport pointer release.

use crate::editor::fem2d::interaction::canvas_gesture;
use crate::editor::fem2d::interaction::{fem2d_addressed_camera, selection_merge_mode};
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
    /// 🚫️ `true` when the host closed the gesture without a release (pointer left the canvas,
    /// capture lost): the in-flight marquee/lasso is dropped and NOTHING is selected or picked.
    #[value(default)]
    pub cancelled: bool,
}

pub fn handle(_payload: &CanvasPointerUp, _doc: &ArtifactView<'_, Fem2dSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Fem2dMutation, NoConfigMutation>, Fault> {
    Err(Fault::from("fem2d.canvas-pointer-up.window-context-required"))
}

/// 🖱️ Completes a marquee/lasso drag or falls back to a direct pick when the drag never crossed the threshold;
/// a `cancelled` release only clears the gesture and refreshes the window.
pub fn handle_window(payload: &CanvasPointerUp, doc: &ArtifactView<'_, Fem2dSnapshot>, cfg: &ConfigView<'_, NoConfig>, view: &semio_framework_plugin::ViewModel) -> Result<Emit<Fem2dMutation, NoConfigMutation>, Fault> {
    let camera = fem2d_addressed_camera(cfg, view, "fem2d.canvas-pointer-up")?;
    let merge = selection_merge_mode(payload.shift, payload.ctrl, payload.meta);
    canvas_gesture::pointer_up(doc.snapshot, &camera, view, payload.x, payload.y, payload.width, payload.height, merge, payload.cancelled)
}
//#endregion 🔖️CanvasPointerUp
