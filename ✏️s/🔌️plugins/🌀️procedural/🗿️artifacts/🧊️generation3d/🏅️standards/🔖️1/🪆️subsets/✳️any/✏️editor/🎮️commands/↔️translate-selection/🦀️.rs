//! ↔️ Generation3d command — `translate-selection`: one gumball translate gesture tick (design §13, `World3dHost` live
//! protocol). The gumball tool (`🧭️transforms`) yields the relative `drag-transforms` leaf of the offset; without a `phase`
//! the dispatch is one transaction, `stream` ticks accumulate in the window's ONE open transaction, `commit` releases it and
//! `abort` (with a `reason`) drops it with zero trace.

use crate::editor::generation3d::config::{Generation3dConfig, Generation3dConfigMutation};
use crate::editor::generation3d::transform_commands::{gumball_once, GumballMotion};
use crate::standards::v1::subsets::any::schema::mutations::text::Generation3dMutation;
use crate::Generation3dSnapshot;
use semio_framework_os_flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "translate-selection")]
pub struct TranslateSelection {
    pub node_ids: Vec<String>,
    pub dx: f64,
    pub dy: f64,
    pub dz: f64,
    pub phase: Option<String>,
    pub reason: Option<String>,
    pub window_id: Option<String>,
}

impl TranslateSelection {
    /// 🎬️ The tick's motion.
    pub fn motion(&self) -> GumballMotion {
        GumballMotion::Translate([self.dx, self.dy, self.dz])
    }
}

/// 🕹️ The marks-free entry `app_commands!` generates: the explicit ids as ONE one-shot transaction.
pub fn handle(payload: &TranslateSelection, doc: &ArtifactView<'_, Generation3dSnapshot>, _cfg: &ConfigView<'_, Generation3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    gumball_once("translateSelection", payload.node_ids.clone(), payload.motion(), doc)
}
