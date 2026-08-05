//! 🗂️ Wires play app commands — everything that only moves the selection. Both are CONFIG-only (they
//! were ephemeral `WiresPlayRuntime` fields before the typed-command conversion): they emit
//! `config_operations` and never document operations.

use crate::apps::wires::config::{WiresConfig, WiresConfigOperation};
use crate::artifacts::wires::op::MindmapWiresOperation;
use crate::artifacts::wires::MindmapWiresDocument;
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️SetSelection
pub mod set_selection {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "set-selection")]
    pub struct SetSelection {
        pub ids: Vec<String>,
    }

    pub fn handle(payload: &SetSelection, _doc: &DocumentView<'_, MindmapWiresDocument>, _cfg: &ConfigView<'_, WiresConfig>) -> Result<Emit<MindmapWiresOperation, WiresConfigOperation>, Fault> {
        Ok(Emit::config(vec![WiresConfigOperation::SetSelection { ids: payload.ids.clone() }]))
    }
}
//#endregion 🔖️SetSelection

//#region 🔖️DocumentSelect
pub mod document_select {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "document-select")]
    pub struct DocumentSelect {
        pub ids: Vec<String>,
    }

    pub fn handle(payload: &DocumentSelect, _doc: &DocumentView<'_, MindmapWiresDocument>, _cfg: &ConfigView<'_, WiresConfig>) -> Result<Emit<MindmapWiresOperation, WiresConfigOperation>, Fault> {
        Ok(Emit::config(vec![WiresConfigOperation::SetSelection { ids: payload.ids.clone() }]))
    }
}
//#endregion 🔖️DocumentSelect

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::wires::testkit::{dispatch, new_app};
    use crate::apps::wires::WiresCommand;

    #[test]
    fn set_selection_is_config_state_and_emits_no_document_operations() {
        let mut app = new_app();
        let result = dispatch(&mut app, WiresCommand::SetSelection(set_selection::SetSelection { ids: vec!["node-1".into()] }));
        assert!(result.operations.is_empty(), "selection must not produce document operations");
    }

    #[test]
    fn document_select_is_config_state_and_emits_no_document_operations() {
        let mut app = new_app();
        let result = dispatch(&mut app, WiresCommand::DocumentSelect(document_select::DocumentSelect { ids: vec!["node-1".into()] }));
        assert!(result.operations.is_empty(), "document select must not produce document operations");
    }
}
//#endregion 🧪️Tests
