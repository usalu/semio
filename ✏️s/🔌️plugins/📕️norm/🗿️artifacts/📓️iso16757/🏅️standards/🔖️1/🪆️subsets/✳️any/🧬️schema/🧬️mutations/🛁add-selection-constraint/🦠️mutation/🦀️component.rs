//! ➕️ `add-selection-constraint` — appends one property constraint to the active selection.

use crate::artifacts::iso16757::{part_1::SelectionConstraint, Iso16757Mutation, Iso16757Snapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddSelectionConstraint {
    pub constraint: SelectionConstraint,
}

impl protocol::MutationKind<Iso16757Snapshot, Iso16757Mutation> for AddSelectionConstraint {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "add", entity: "selection-constraint", kind: "add-selection-constraint", record: "AddedSelectionConstraint" };

    fn diff(&self, base: &Iso16757Snapshot) -> protocol::MutationOutcome<<Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Iso16757Snapshot) -> Vec<Iso16757Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Add selection constraint on \"{}\"", self.constraint.property_id)
    }
}
//#endregion 🔖️Payload
