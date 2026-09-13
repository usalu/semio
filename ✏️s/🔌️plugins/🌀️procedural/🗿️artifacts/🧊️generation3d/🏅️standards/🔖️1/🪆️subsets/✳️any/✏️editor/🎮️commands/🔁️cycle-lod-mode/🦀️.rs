//! 🔁️ 🎚️ Generation3d play app commands command — `cycle-lod-mode`.

use crate::editor::generation3d::config::{next_lod_mode, Generation3dConfig, Generation3dConfigMutation};
use crate::standards::v1::subsets::any::schema::mutations::text::Generation3dMutation;
use crate::Generation3dSnapshot;
use semio_framework_os_flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

/// 🔁️ The level-of-detail twin of `cycleShowMode` — same argument-free contract, same ladder
/// discipline (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "cycle-lod-mode")]
pub struct CycleLodMode {}

pub fn handle(_payload: &CycleLodMode, _doc: &ArtifactView<'_, Generation3dSnapshot>, cfg: &ConfigView<'_, Generation3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    Ok(Emit::config(vec![Generation3dConfigMutation::SetLodMode(crate::editor::generation3d::config::SetLodMode { value: next_lod_mode(&cfg.snapshot.lod_mode) })]))
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
