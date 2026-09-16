//! 🖱️ Fem2d play app command — `canvas-pointer-move`: viewport hover — requests the framework `interactionHover` on the `pointer` channel.

use crate::editor::fem2d::interaction::{fem2d_addressed_camera, fem2d_hit_test, interaction_hover_effect, Fem2dPick};
use crate::standards::v1::subsets::any::schema::mutations::text::Fem2dMutation;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use semio_framework_value_derive::{FromValue, ToValue};

type Fem2dSnapshot = crate::Fem2dSnapshot;

//#region 🔖️CanvasPointerMove
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "canvas-pointer-move")]
pub struct CanvasPointerMove {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

pub fn handle(_payload: &CanvasPointerMove, _doc: &ArtifactView<'_, Fem2dSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Fem2dMutation, NoConfigMutation>, Fault> {
    Err(Fault::from("fem2d.canvas-pointer-move.window-context-required"))
}

/// 🖱️ Pointer travel is one stateless hover batch per sample — the app cannot know whether the hovered
/// id changed (hover is framework-owned state it never reads back at dispatch time), so it always
/// reports what is under the cursor and the framework's hover machine drops an unchanged batch.
pub fn handle_window(payload: &CanvasPointerMove, doc: &ArtifactView<'_, Fem2dSnapshot>, cfg: &ConfigView<'_, NoConfig>, view: &semio_framework_plugin::ViewModel) -> Result<Emit<Fem2dMutation, NoConfigMutation>, Fault> {
    let camera = fem2d_addressed_camera(cfg, view, "fem2d.canvas-pointer-move")?;
    let targets: Vec<Fem2dPick> = fem2d_hit_test(doc.snapshot, &camera, payload.x, payload.y, payload.width, payload.height).into_iter().collect();
    Ok(Emit { effects: vec![interaction_hover_effect(&targets)], ..Default::default() })
}
//#endregion 🔖️CanvasPointerMove
