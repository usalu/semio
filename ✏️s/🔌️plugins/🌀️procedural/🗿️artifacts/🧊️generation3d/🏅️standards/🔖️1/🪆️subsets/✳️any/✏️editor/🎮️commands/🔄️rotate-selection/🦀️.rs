//! 🔄️ Generation3d command — `rotate-selection`: one gumball rotate gesture tick (design §13, `World3dHost` live protocol).
//! The gumball tool (`🧭️transforms`) yields the relative `rotate-transforms` leaf of the world-axis rotation; the phases are
//! those of `translate-selection`.

use crate::editor::generation3d::config::{Generation3dConfig, Generation3dConfigMutation};
use crate::editor::generation3d::transform_commands::{gumball_once, GumballMotion};
use crate::standards::v1::subsets::any::schema::mutations::Generation3dMutation;
use crate::Generation3dSnapshot;
use semio_framework_os_flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use crate::standards::v1::subsets::any::schema::transforms::AxisAngle;
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "rotate-selection")]
pub struct RotateSelection {
    pub node_ids: Vec<String>,
    pub ax: f64,
    pub ay: f64,
    pub az: f64,
    pub angle: f64,
    pub phase: Option<String>,
    pub reason: Option<String>,
    pub window_id: Option<String>,
}

impl RotateSelection {
    /// 🎬️ The tick's motion.
    pub fn motion(&self) -> GumballMotion {
        GumballMotion::Rotate(AxisAngle { axis: [self.ax, self.ay, self.az], angle: self.angle })
    }
}

/// 🕹️ The marks-free entry `app_commands!` generates: the explicit ids as ONE one-shot transaction.
pub fn handle(payload: &RotateSelection, doc: &ArtifactView<'_, Generation3dSnapshot>, _cfg: &ConfigView<'_, Generation3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    gumball_once("rotateSelection", payload.node_ids.clone(), payload.motion(), doc)
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
