//! 🧮️ ISO 16757 play app command — recompute the compliance report in place.
//!
//! 📌️ The report is not document state, it is derived on every read (`NormHost::from_document`
//! re-evaluates unconditionally) — so this command genuinely mutates nothing. The pre-migration
//! version recommitted the whole document as a no-op whole-document-replace mutation purely to leave
//! a command-log entry; now that whole-document replace has no vocabulary equivalent, the honest
//! fix is to emit zero mutations (`Emit::default()`) rather than inventing a fake semantic edit.

use crate::artifacts::iso16757::op::Iso16757Mutation;
use crate::artifacts::iso16757::Iso16757Snapshot;
use crate::config::{NormConfig, NormConfigMutation};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
/// 🧮️ Fieldless — this replaces a bare unit variant, whose wire form (`evaluate` / `01 <ord> 00 00`) a
/// fieldless `DslRecord` struct reproduces exactly.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "evaluate")]
pub struct Evaluate {}
//#endregion 🔖️Payload

//#region 🔖️Handler
pub fn handle(_payload: &Evaluate, _doc: &ArtifactView<'_, Iso16757Snapshot>, _cfg: &ConfigView<'_, NormConfig>) -> Result<Emit<Iso16757Mutation, NormConfigMutation>, Fault> {
    Ok(Emit::default())
}
//#endregion 🔖️Handler

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_plugin::HistoryView;

    #[test]
    fn handle_emits_no_mutation_since_the_report_is_always_recomputed() {
        let projection = Iso16757Snapshot::default();
        let config = NormConfig::default();
        let emit = handle(&Evaluate {}, &ArtifactView { snapshot: &projection, history: &HistoryView::empty() }, &ConfigView { snapshot: &config }).expect("handle");
        assert!(emit.artifact_mutations.is_empty());
        assert!(emit.config_mutations.is_empty());
    }
}
//#endregion 🧪️Tests
