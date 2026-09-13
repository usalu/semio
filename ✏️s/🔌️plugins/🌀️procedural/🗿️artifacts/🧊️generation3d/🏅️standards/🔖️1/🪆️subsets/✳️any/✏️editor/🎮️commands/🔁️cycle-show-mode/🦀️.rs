//! 🔁️ 👁️ Generation3d play app commands command — `cycle-show-mode`.

use crate::editor::generation3d::config::{next_show_mode, Generation3dConfig, Generation3dConfigMutation};
use crate::standards::v1::subsets::any::schema::mutations::text::Generation3dMutation;
use crate::Generation3dSnapshot;
use semio_framework_os_flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

/// 🔁️ Argument-free by construction: `AppDefinition.keybinding` carries a chord and an action id and
/// nothing else, so a keyboard-reachable display toggle has to read its own next value out of the
/// config rather than take one (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "cycle-show-mode")]
pub struct CycleShowMode {}

pub fn handle(_payload: &CycleShowMode, _doc: &ArtifactView<'_, Generation3dSnapshot>, cfg: &ConfigView<'_, Generation3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    Ok(Emit::config(vec![Generation3dConfigMutation::SetShowMode(crate::editor::generation3d::config::SetShowMode { value: next_show_mode(&cfg.snapshot.show_mode) })]))
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
