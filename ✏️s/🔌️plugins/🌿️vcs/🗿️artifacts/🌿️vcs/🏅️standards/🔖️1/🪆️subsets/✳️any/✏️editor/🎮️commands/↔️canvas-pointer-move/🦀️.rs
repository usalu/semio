//! 🖱️ 🖱️ VCS play app commands command — `canvas-pointer-move`.

use crate::editor::vcs::config::{VcsDemoConfig, VcsDemoConfigMutation};
use crate::{op::VcsDemoMutation, VcsSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "canvas-pointer-move")]
pub struct CanvasPointerMove {
    /// 🧵️ Every pointer sample of this batch as canvas pixels, oldest first (design L4 / §2 D);
    /// the action bridge folds a legacy `{x, y}` wire into the single `[[x, y]]`. This app keeps
    /// no drag gesture on its canvas, so the batch is carried, not consumed.
    #[value(default)]
    pub samples: Vec<[f64; 2]>,
}

impl CanvasPointerMove {
    /// 🧵️ The newest sample of the batch, if any.
    pub fn last_sample(&self) -> Option<[f64; 2]> {
        self.samples.last().copied()
    }
}

pub fn handle(_payload: &CanvasPointerMove, _doc: &ArtifactView<'_, VcsSnapshot>, _cfg: &ConfigView<'_, VcsDemoConfig>) -> Result<Emit<VcsDemoMutation, VcsDemoConfigMutation>, Fault> {
    Ok(Emit::default())
}
