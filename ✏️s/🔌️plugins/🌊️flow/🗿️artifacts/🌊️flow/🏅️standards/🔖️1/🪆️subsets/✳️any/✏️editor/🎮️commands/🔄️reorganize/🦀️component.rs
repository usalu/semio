//! 🔄️ 🔄️ Flow play app commands command — `reorganize`.

use crate::artifacts::flow::{op::FlowMutation, FlowSnapshot};
use crate::editor::flow::config::{FlowConfig, FlowConfigMutation};
use crate::editor::flow::host_operations;
use flow::FlowEvalSession;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️Reorganize
/// 🔄️ The single left-to-right auto-layout pass. Shared verbatim with the `auto-layout` extension's
/// `"reorganize"` effect (see `🎮️commands/🧩️toggle-extension`), which is why the host argument JSON lives here.
pub const REORGANIZE_OPTIONS_JSON: &str = r#"{"orientation":"leftRight"}"#;

/// 🔄️ The reorganize document operations, extracted so the extension action can reuse them without
/// round-tripping through the command enum.
pub fn reorganize_operations(doc: &ArtifactView<'_, FlowSnapshot>, cfg: &ConfigView<'_, FlowConfig>, session: &mut FlowEvalSession) -> Vec<FlowMutation> {
    host_operations(doc.snapshot, cfg.snapshot, session, |host| host.reorganize(REORGANIZE_OPTIONS_JSON).is_ok())
}
//#endregion 🔖️Reorganize

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
pub struct Reorganize {}

pub fn handle(_payload: &Reorganize, doc: &ArtifactView<'_, FlowSnapshot>, cfg: &ConfigView<'_, FlowConfig>, session: &mut FlowEvalSession) -> Result<Emit<FlowMutation, FlowConfigMutation>, Fault> {
    Ok(Emit::mutations(reorganize_operations(doc, cfg, session)))
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::flow::testkit::{dispatch, flow_app};
    use crate::editor::flow::FlowCommand;

    #[test]
    fn reorganize_keeps_every_widget() {
        let mut app = flow_app();
        let before = app.snapshot().expect("snapshot").to_fixture().widgets.len();
        dispatch(&mut app, FlowCommand::Reorganize(Reorganize {}));
        assert_eq!(app.snapshot().expect("snapshot").to_fixture().widgets.len(), before);
    }
}
//#endregion 🧪️Tests
