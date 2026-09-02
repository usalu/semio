//! ✂️ `remove-selection-constraint` — removes one property constraint by BASE-state index.

use crate::artifacts::iso16757::{Iso16757Mutation, Iso16757Snapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub struct RemoveSelectionConstraint {
    pub index: usize,
}

impl protocol::MutationKind<Iso16757Snapshot, Iso16757Mutation> for RemoveSelectionConstraint {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "selection-constraint", kind: "remove-selection-constraint", record: "RemovedSelectionConstraint" };

    fn diff(&self, base: &Iso16757Snapshot) -> protocol::MutationOutcome<<Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Iso16757Snapshot) -> Vec<Iso16757Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Remove selection constraint #{}", self.index)
    }
}
//#endregion 🔖️Payload
