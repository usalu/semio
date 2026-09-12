//! 🧮️ Generation3d play app commands command — `flow-eval-resolve`: the editor's binding of the
//! surface-neutral chain in `🧵️preview-eval`, which owns the payload shape and the fold itself.

use crate::editor::generation3d::config::{Generation3dConfig, Generation3dConfigMutation};
use crate::preview_eval;
use crate::standards::v1::subsets::any::schema::mutations::text::Generation3dMutation;
use crate::Generation3dSnapshot;
use semio_framework_os_flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

pub use crate::preview_eval::FlowEvalResolve;

pub fn handle(payload: &FlowEvalResolve, _doc: &ArtifactView<'_, Generation3dSnapshot>, _cfg: &ConfigView<'_, Generation3dConfig>, session: &mut FlowEvalSession) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    Ok(Emit { effects: preview_eval::resolve_eval(payload, session), ..Default::default() })
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🧪️Budget
/// ⏱️ The BUDGET law of the `evaluate` capability — its own module because the law is about the
/// ENVELOPE and its continuation, not about the one finished answer `🔬️unit` pins
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[cfg(test)]
#[path = "🧪️tests/🔬️budget/🦀️.rs"]
mod budget;
//#endregion 🧪️Budget
