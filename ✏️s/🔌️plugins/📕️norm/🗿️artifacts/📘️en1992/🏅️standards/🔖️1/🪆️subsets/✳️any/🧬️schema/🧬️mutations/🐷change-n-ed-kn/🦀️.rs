//! 🔧 `change-n-ed-kn` payload — changes the En1992 document's `n_ed_kn` (EN 1992 input).


use crate::artifacts::en1992::En1992Snapshot;
use crate::artifacts::en1992::diff::En1992Diff;
use crate::artifacts::en1992::mutations::En1992Mutation;
use crate::artifacts::en1992::mutations::change_n_ed_kn::ChangeNEdKn;

//#region 🔖️ChangeNEdKn
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeNEdKn {
    pub new_n_ed_kn: f64,
}

impl protocol::MutationKind<En1992Snapshot, En1992Mutation> for ChangeNEdKn {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "n-ed-kn", kind: "change-n-ed-kn", record: "ChangedNEdKn" };

    fn diff(&self, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1992Snapshot) -> Vec<En1992Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change n ed kn to {:?}", self.new_n_ed_kn)
    }
}
//#endregion 🔖️ChangeNEdKn
