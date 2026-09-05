//! 🌲 `change-pile-n-profiles` payload — changes the En1997 document's `pile_n_profiles` (number of investigated pile profiles).


use crate::artifacts::en1997::En1997Snapshot;
use crate::artifacts::en1997::diff::En1997Diff;
use crate::artifacts::en1997::mutations::En1997Mutation;
//#region 🔖️ChangePileNProfiles
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangePileNProfiles {
    pub new_pile_n_profiles: u32,
}

impl protocol::MutationKind<En1997Snapshot, En1997Mutation> for ChangePileNProfiles {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "pile-n-profiles", kind: "change-pile-n-profiles", record: "ChangedPileNProfiles" };

    fn diff(&self, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1997Snapshot) -> Vec<En1997Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change number of investigated pile profiles to {}", self.new_pile_n_profiles)
    }
}
//#endregion 🔖️ChangePileNProfiles
