//! 🖱️ 🖱️ Wires play app commands command — `canvas-pointer-move`.
//!
//! 🎯️ Batched (design L4 / §2 D): the host folds every DOM `pointermove` of one turn into ONE
//! command whose `samples` carry every canvas-pixel position oldest-first; `x`/`y` stay the LAST
//! sample. The Wires node drag keeps only `drag_last_{x,y}` (the node lands at start→last delta),
//! so the retained drag work reads the batch's last sample; intermediate samples carry no state.

use crate::WiresMutation;
use crate::WiresSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_plugin::{NoConfig, NoConfigMutation};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "pointer-move")]
pub struct CanvasPointerMove {
    pub x: f64,
    pub y: f64,
    /// 🧵️ Every pointer sample of this batch as canvas pixels, oldest first — required on the wire (both hosts always send
    /// it, the wgpu canvas and `Canvas2dHost`).
    pub samples: Vec<[f64; 2]>,
}

impl CanvasPointerMove {
    /// 🧵️ The newest sample of the batch; it wins over the trailing `x`/`y` when they disagree.
    pub fn last_sample(&self) -> [f64; 2] {
        self.samples.last().copied().unwrap_or([self.x, self.y])
    }

    /// 🧮️ Every coordinate of the batch (and the trailing `x`/`y`) is finite.
    pub fn is_finite(&self) -> bool {
        self.x.is_finite() && self.y.is_finite() && self.samples.iter().all(|[x, y]| x.is_finite() && y.is_finite())
    }
}

pub fn handle(_payload: &CanvasPointerMove, _doc: &ArtifactView<'_, WiresSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<WiresMutation, NoConfigMutation>, Fault> {
    Err(Fault::from("wires-pointer-move-requires-retained-window-owner"))
}
