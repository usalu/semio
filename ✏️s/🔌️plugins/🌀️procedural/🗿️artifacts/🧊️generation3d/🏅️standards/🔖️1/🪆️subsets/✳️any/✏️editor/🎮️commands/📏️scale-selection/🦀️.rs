//! 📏️ Generation3d command — `scale-selection`: one gumball scale gesture tick (design §13, `World3dHost` live protocol).
//! The gumball tool (`🧭️transforms`) yields the relative `scale-transforms` leaf of the per-axis factors; the phases are
//! those of `translate-selection`.

use crate::editor::generation3d::config::{Generation3dConfig, Generation3dConfigMutation};
use crate::editor::generation3d::transform_commands::{gumball_once, GumballMotion};
use crate::standards::v1::subsets::any::schema::mutations::Generation3dMutation;
use crate::Generation3dSnapshot;
use semio_framework_os_flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "scale-selection")]
pub struct ScaleSelection {
    pub node_ids: Vec<String>,
    pub sx: f64,
    pub sy: f64,
    pub sz: f64,
    pub phase: Option<String>,
    pub reason: Option<String>,
    pub window_id: Option<String>,
}

impl ScaleSelection {
    /// 🎬️ The tick's motion.
    pub fn motion(&self) -> GumballMotion {
        GumballMotion::Scale([self.sx, self.sy, self.sz])
    }
}

/// 🕹️ The marks-free entry `app_commands!` generates: the explicit ids as ONE one-shot transaction.
pub fn handle(payload: &ScaleSelection, doc: &ArtifactView<'_, Generation3dSnapshot>, _cfg: &ConfigView<'_, Generation3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    gumball_once("scaleSelection", payload.node_ids.clone(), payload.motion(), doc)
}
