//! 🖱️ Fem2d play app command — `canvas-pointer-down`: viewport pick — hit-tests the addressed model/results window and requests the framework `interactionSelect`.

use crate::editor::fem2d::interaction::{fem2d_addressed_camera, fem2d_hit_test, interaction_select_effect, selection_merge_mode, Fem2dPick};
use crate::standards::v1::subsets::any::schema::mutations::text::Fem2dMutation;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use semio_framework_value_derive::{FromValue, ToValue};

type Fem2dSnapshot = crate::Fem2dSnapshot;

//#region 🔖️CanvasPointerDown
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
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

/// 🖱️ A primary-button press is a viewport pick: hit-test the addressed window's projection and ask
/// the framework to apply the resulting selection. Nothing hit is an explicit empty `"replace"` batch,
/// so a background click clears. Every other button belongs to the host (middle drag pans).
pub fn handle_window(payload: &CanvasPointerDown, doc: &ArtifactView<'_, Fem2dSnapshot>, cfg: &ConfigView<'_, NoConfig>, view: &semio_framework_plugin::ViewModel) -> Result<Emit<Fem2dMutation, NoConfigMutation>, Fault> {
    if payload.button != 0 {
        return Ok(Emit::default());
    }
    let camera = fem2d_addressed_camera(cfg, view, "fem2d.canvas-pointer-down")?;
    let hit = fem2d_hit_test(doc.snapshot, &camera, payload.x, payload.y, payload.width, payload.height);
    let merge = if hit.is_some() { selection_merge_mode(payload.shift, payload.ctrl, payload.meta) } else { "replace" };
    let targets: Vec<Fem2dPick> = hit.into_iter().collect();
    Ok(Emit { effects: vec![interaction_select_effect(&targets, merge)], ..Default::default() })
}
//#endregion 🔖️CanvasPointerDown

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
