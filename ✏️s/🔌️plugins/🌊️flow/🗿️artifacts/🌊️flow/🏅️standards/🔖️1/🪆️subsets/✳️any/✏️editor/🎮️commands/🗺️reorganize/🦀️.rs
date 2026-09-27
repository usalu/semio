//! 🔄️ 🔄️ Flow play app commands command — `reorganize`.

use semio_framework_plugin::NoConfig;
use semio_framework_plugin::NoConfigMutation;
use crate::editor::flow::host_scene_edit;
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
/// 🗺️ Lays the composed scene out left-to-right and publishes the new layout on the content child — nothing when the
/// layout is already the host's own arrangement.
pub fn reorganize_edit(composed: &FlowSnapshot, config: &FlowMainWindowConfig, session: &FlowEvalSession) -> Result<Emit<FlowMutation, NoConfigMutation>, Fault> {
    host_scene_edit(composed, config, session, |host| Ok(host.reorganize(REORGANIZE_OPTIONS_JSON).is_ok()))
}
//#endregion 🔖️Reorganize

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
pub struct Reorganize {}

pub fn handle(_payload: &Reorganize, doc: &ArtifactView<'_, FlowSnapshot>, cfg: &ConfigView<'_, NoConfig>, session: &mut FlowEvalSession) -> Result<Emit<FlowMutation, NoConfigMutation>, Fault> {
    reorganize_edit(&crate::flow_composed_snapshot(doc.snapshot, &doc.children)?, &crate::editor::flow::modes::edit::windows::main::config::current(cfg), session)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
