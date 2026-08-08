//! 🔄️ Flow play app commands — auto-layout of the whole canvas.

use crate::apps::flow::config::{FlowConfig, FlowConfigMutation};
use crate::artifacts::flow::engine::host_operations;
use crate::artifacts::flow::{op::FlowMutation, FlowFixture};
use flow::FlowEvalSession;
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️Reorganize
/// 🔄️ The single left-to-right auto-layout pass. Shared verbatim with the `auto-layout` extension's
/// `"reorganize"` effect (see `🎮️commands/🧩️extension`), which is why the host argument JSON lives here.
pub const REORGANIZE_OPTIONS_JSON: &str = r#"{"orientation":"leftRight"}"#;

pub mod reorganize {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "reorganize")]
    pub struct Reorganize {}

    pub fn handle(_payload: &Reorganize, doc: &DocumentView<'_, FlowFixture>, cfg: &ConfigView<'_, FlowConfig>, session: &mut FlowEvalSession) -> Result<Emit<FlowMutation, FlowConfigMutation>, Fault> {
        Ok(Emit::mutations(reorganize_operations(doc, cfg, session)))
    }
}

/// 🔄️ The reorganize document operations, extracted so the extension action can reuse them without
/// round-tripping through the command enum.
pub fn reorganize_operations(doc: &DocumentView<'_, FlowFixture>, cfg: &ConfigView<'_, FlowConfig>, session: &mut FlowEvalSession) -> Vec<FlowMutation> {
    host_operations(doc.projection, cfg.projection, session, |host| host.reorganize(REORGANIZE_OPTIONS_JSON).is_ok())
}
//#endregion 🔖️Reorganize

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::flow::testkit::{dispatch, flow_app};
    use crate::apps::flow::FlowCommand;

    #[test]
    fn reorganize_keeps_every_widget() {
        let mut app = flow_app();
        let before = app.projection().expect("projection").widgets.len();
        dispatch(&mut app, FlowCommand::Reorganize(reorganize::Reorganize {}));
        assert_eq!(app.projection().expect("projection").widgets.len(), before);
    }
}
//#endregion 🧪️Tests
