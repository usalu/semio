//! 📅️ `change-correction-as-of` — sets the document root's correction cut-off edition.


use crate::artifacts::vdi3805::{EditionId, Vdi3805Diff, Vdi3805Mutation, Vdi3805Snapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeCorrectionAsOf {
    pub new_correction_as_of: EditionId,
}

impl protocol::MutationKind<Vdi3805Snapshot, Vdi3805Mutation> for ChangeCorrectionAsOf {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "correction-as-of", kind: "change-correction-as-of", record: "ChangedCorrectionAsOf" };

    fn diff(&self, base: &Vdi3805Snapshot) -> protocol::MutationOutcome<<Vdi3805Mutation as protocol::Mutation<Vdi3805Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Vdi3805Snapshot) -> Vec<Vdi3805Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change correction-as-of to {}-{:02}", self.new_correction_as_of.year, self.new_correction_as_of.month)
    }
}
//#endregion 🔖️Payload
