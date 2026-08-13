//! 🗂️ 🗂️ Procedural2d play app commands command — `set-selection`.

use crate::apps::procedural2d::config::{Procedural2dConfig, Procedural2dConfigMutation};
use crate::artifacts::procedural2d::op::Procedural2dMutation;
use crate::artifacts::procedural2d::Procedural2dSnapshot;
use flow::FlowEvalSession;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "set-selection")]
pub struct SetSelection {
    pub ids: Vec<String>}

pub fn handle(payload: &SetSelection, _doc: &ArtifactView<'_, Procedural2dSnapshot>, _cfg: &ConfigView<'_, Procedural2dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Procedural2dMutation, Procedural2dConfigMutation>, Fault> {
    Ok(Emit::config(vec![Procedural2dConfigMutation::SetSelection { ids: payload.ids.clone() }]))
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::procedural2d::testkit::{app, dispatch};
    use crate::apps::procedural2d::Procedural2dCommand;

    #[test]
    fn set_selection_updates_config_only() {
        let mut app = app();
        let before = app.snapshot().expect("snapshot");
        dispatch(&mut app, Procedural2dCommand::SetSelection(SetSelection { ids: vec!["w1".into()] }));
        assert_eq!(app.snapshot().expect("snapshot"), before, "setSelection must not mutate the document");
    }
}
//#endregion 🧪️Tests
