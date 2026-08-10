//! 🧮️ EN 1992 play app command — recompute the compliance report in place.
//!
//! 📌️ Recommitting the current projection is what forces `NormHost` to re-evaluate: the report is not
//! document state, it is derived on every read, so a no-op whole-document commit is the honest way to
//! record "the user asked for a fresh evaluation" in the command log.

use crate::artifacts::en1992::op::En1992Mutation;
use crate::artifacts::en1992::En1992Snapshot;
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
pub fn handle(_payload: &Evaluate, doc: &ArtifactView<'_, En1992Snapshot>, _cfg: &ConfigView<'_, NormConfig>) -> Result<Emit<En1992Mutation, NormConfigMutation>, Fault> {
    crate::app_surface::commit_snapshot(En1992Mutation::SetSnapshot { snapshot: doc.snapshot.clone() }, "evaluate")
}
//#endregion 🔖️Handler

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
        use semio_framework_plugin::HistoryView;

    #[test]
    fn handle_recommits_the_current_projection_under_its_action_id() {
        let projection = En1992Snapshot::default();
        let config = NormConfig::default();
        let emit = handle(&Evaluate {}, &ArtifactView { snapshot: &projection, history: &HistoryView::empty() }, &ConfigView { snapshot: &config }).expect("handle");
        assert_eq!(emit.artifact_mutations, vec![En1992Mutation::SetSnapshot { snapshot: En1992Snapshot::default() }]);
        assert_eq!(emit.description.as_deref(), Some("evaluate"));
    }
}
//#endregion 🧪️Tests
