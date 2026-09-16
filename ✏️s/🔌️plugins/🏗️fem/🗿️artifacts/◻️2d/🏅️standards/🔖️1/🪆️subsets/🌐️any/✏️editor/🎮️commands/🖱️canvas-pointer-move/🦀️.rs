//! 🖱️ Fem2d play app command — `canvas-pointer-move`: viewport hover — requests the framework `interactionHover` on the `pointer` channel.
//!
//! 🎯️ Batched (design L4 / §2 D): the host folds every DOM `pointermove` of one turn into ONE
//! command whose `samples` carry every canvas-pixel position oldest-first; `x`/`y` stay the LAST
//! sample so a reader that ignores `samples` keeps today's semantics with fewer calls.

use crate::editor::fem2d::interaction::canvas_gesture;
use crate::editor::fem2d::interaction::fem2d_addressed_camera;
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
    /// 🧵️ Every pointer sample of this batch as canvas pixels, oldest first. Empty on a legacy
    /// (unbatched) wire — [`CanvasPointerMove::samples_or_last`] then yields the single `(x, y)`.
    #[value(default)]
    pub samples: Vec<[f64; 2]>,
}

impl CanvasPointerMove {
    /// 🧵️ The batch as gesture samples, oldest first — never empty: an absent/empty `samples`
    /// (an app or host that still sends one command per event) degrades to the single `(x, y)`.
    pub fn samples_or_last(&self) -> Vec<(f64, f64)> {
        if self.samples.is_empty() {
            vec![(self.x, self.y)]
        } else {
            self.samples.iter().map(|[x, y]| (*x, *y)).collect()
        }
    }
}

pub fn handle(_payload: &CanvasPointerMove, _doc: &ArtifactView<'_, Fem2dSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Fem2dMutation, NoConfigMutation>, Fault> {
    Err(Fault::from("fem2d.canvas-pointer-move.window-context-required"))
}

/// 🖱️ While a marquee/lasso drag is active this refreshes the overlay; otherwise it reports pointer hover.
pub fn handle_window(payload: &CanvasPointerMove, doc: &ArtifactView<'_, Fem2dSnapshot>, cfg: &ConfigView<'_, NoConfig>, view: &semio_framework_plugin::ViewModel) -> Result<Emit<Fem2dMutation, NoConfigMutation>, Fault> {
    let camera = fem2d_addressed_camera(cfg, view, "fem2d.canvas-pointer-move")?;
    canvas_gesture::pointer_move(doc.snapshot, &camera, view, &payload.samples_or_last(), payload.width, payload.height)
}
//#endregion 🔖️CanvasPointerMove
