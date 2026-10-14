//! 👁️ 👁️ Generation2d play app commands command — `canvas-pointer-move`.

use semio_s_artifact_procedural_2d_editor_config::Generation2dConfig;
use semio_s_artifact_procedural_2d_editor_config::Generation2dConfigMutation;
use semio_s_artifact_procedural_2d_mutations::Generation2dMutation;
use semio_s_artifact_procedural_2d_model::snapshot::Generation2dSnapshot;
use semio_framework_os_flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
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

pub fn handle(_payload: &CanvasPointerMove, _doc: &ArtifactView<'_, Generation2dSnapshot>, _cfg: &ConfigView<'_, Generation2dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Generation2dMutation, Generation2dConfigMutation>, Fault> {
    Ok(Emit::default())
}
