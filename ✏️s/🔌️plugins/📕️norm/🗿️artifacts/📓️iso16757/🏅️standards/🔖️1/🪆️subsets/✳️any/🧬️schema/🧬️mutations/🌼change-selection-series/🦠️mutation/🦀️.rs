//! 🧵️ `change-selection-series` — sets the optional product-series scalar of the active selection.

use crate::artifacts::iso16757::{Iso16757Mutation, Iso16757Snapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeSelectionSeries {
    pub new_series_id: Option<String>,
}

impl protocol::MutationKind<Iso16757Snapshot, Iso16757Mutation> for ChangeSelectionSeries {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "selection-series", kind: "change-selection-series", record: "ChangedSelectionSeries" };

    fn diff(&self, base: &Iso16757Snapshot) -> protocol::MutationOutcome<<Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Iso16757Snapshot) -> Vec<Iso16757Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        match &self.new_series_id {
            Some(id) => format!("Change selection series to \"{id}\""),
            None => "Clear selection series".to_string(),
        }
    }
}
//#endregion 🔖️Payload
