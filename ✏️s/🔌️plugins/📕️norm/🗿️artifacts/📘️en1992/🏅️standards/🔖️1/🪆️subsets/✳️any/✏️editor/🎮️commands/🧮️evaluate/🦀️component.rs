//! 🧮️ EN 1992 play app command — recompute the compliance report in place.
//!
//! 📌️ The report is not document state, it is derived on every read (`NormHost::from_document`
//! re-evaluates unconditionally) — so this command genuinely mutates nothing. The pre-migration
//! version recommitted the whole document as a no-op whole-document-replace mutation purely to leave
//! a command-log entry; now that whole-document replace has no vocabulary equivalent, the honest
//! fix is to emit zero mutations (`Emit::default()`) rather than inventing a fake semantic edit.

use crate::artifacts::en1992::op::En1992Mutation;
use crate::artifacts::en1992::En1992Snapshot;
use crate::config::{NormConfig, NormConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
/// 🧮️ Fieldless — this replaces a bare unit variant, whose wire form (`evaluate` / `01 <ord> 00 00`) a
/// fieldless `DslRecord` struct reproduces exactly.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "evaluate")]
pub struct Evaluate {}
//#endregion 🔖️Payload

//#region 🔖️Handler
pub async fn handle(_payload: &Evaluate, _doc: &ArtifactView<'_, En1992Snapshot>, _cfg: &ConfigView<'_, NormConfig>) -> Result<Emit<En1992Mutation, NormConfigMutation>, Fault> {
    Ok(Emit::default())
}
//#endregion 🔖️Handler

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_plugin::HistoryView;

    #[semio_framework_async_macros::async_test]
    async fn handle_emits_no_mutation_since_the_report_is_always_recomputed() {
        let projection = En1992Snapshot::default();
        let config = NormConfig::default();
        let emit = handle(&Evaluate {}, &ArtifactView::new(&projection, &HistoryView::empty()), &ConfigView { snapshot: &config }).expect("handle");
        assert!(emit.artifact_mutations.is_empty());
        assert!(emit.config_mutations.is_empty());
    }
}
//#endregion 🧪️Tests
