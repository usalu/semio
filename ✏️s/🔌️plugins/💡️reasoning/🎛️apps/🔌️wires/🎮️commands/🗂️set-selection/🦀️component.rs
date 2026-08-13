//! 🗂️ 🗂️ Wires play app commands command — `set-selection`.

use crate::apps::wires::config::{WiresConfig, WiresConfigMutation};
use crate::artifacts::wires::op::WiresMutation;
use crate::artifacts::wires::WiresSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "set-selection")]
pub struct SetSelection {
    pub ids: Vec<String>,
}

pub fn handle(payload: &SetSelection, _doc: &ArtifactView<'_, WiresSnapshot>, _cfg: &ConfigView<'_, WiresConfig>) -> Result<Emit<WiresMutation, WiresConfigMutation>, Fault> {
    Ok(Emit::config(vec![WiresConfigMutation::SetSelection { ids: payload.ids.clone() }]))
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::wires::testkit::{dispatch, new_app};
    use crate::apps::wires::WiresCommand;

    #[test]
    fn set_selection_is_config_state_and_emits_no_artifact_mutations() {
        let mut app = new_app();
        let result = dispatch(&mut app, WiresCommand::SetSelection(SetSelection { ids: vec!["node-1".into()] }));
        assert!(result.mutations.is_empty(), "selection must not produce document operations");
    }

    #[test]
    fn document_select_is_config_state_and_emits_no_artifact_mutations() {
        let mut app = new_app();
        let result = dispatch(&mut app, WiresCommand::DocumentSelect(document_select::DocumentSelect { ids: vec!["node-1".into()] }));
        assert!(result.mutations.is_empty(), "document select must not produce document operations");
    }
}
//#endregion 🧪️Tests
