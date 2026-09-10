//! 🔄️ 🔄️ Flow play app commands command — `reorganize`.

use semio_framework_plugin::NoConfig;
use semio_framework_plugin::NoConfigMutation;
use crate::editor::flow::host_operations;
use crate::editor::flow::modes::edit::windows::main::config::FlowMainWindowConfig;
use crate::{op::FlowMutation, FlowSnapshot};
use flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Reorganize
/// 🔄️ The single left-to-right auto-layout pass. Shared verbatim with the `auto-layout` extension's
/// `"reorganize"` effect (see `🎮️commands/🧩️toggle-extension`), which is why the host argument JSON lives here.
pub const REORGANIZE_OPTIONS_JSON: &str = r#"{"orientation":"leftRight"}"#;

/// 🔄️ The reorganize document operations, extracted so the extension action can reuse them without
/// round-tripping through the command enum.
pub fn reorganize_operations(snapshot: &FlowSnapshot, config: &FlowMainWindowConfig, session: &FlowEvalSession) -> Vec<FlowMutation> {
    host_operations(snapshot, config, session, |host| host.reorganize(REORGANIZE_OPTIONS_JSON).is_ok())
}
//#endregion 🔖️Reorganize

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
pub struct Reorganize {}

pub fn handle(_payload: &Reorganize, doc: &ArtifactView<'_, FlowSnapshot>, cfg: &ConfigView<'_, NoConfig>, session: &mut FlowEvalSession) -> Result<Emit<FlowMutation, NoConfigMutation>, Fault> {
    Ok(Emit::mutations(reorganize_operations(doc.snapshot, &crate::editor::flow::modes::edit::windows::main::config::current(cfg), session)))
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
