//! 🦎 `change-n-ed-kn` payload — changes the En1999 document's `n_ed_kn` (design axial force N_Ed [kN]).


use crate::artifacts::en1999::En1999Snapshot;
use crate::artifacts::en1999::diff::En1999Diff;
use crate::artifacts::en1999::mutations::En1999Mutation;
use crate::artifacts::en1999::mutations::change_n_ed_kn::ChangeNEdKn;

//#region 🔖️ChangeNEdKn
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeNEdKn {
    pub new_n_ed_kn: f64,
}

impl protocol::MutationKind<En1999Snapshot, En1999Mutation> for ChangeNEdKn {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "n-ed-kn", kind: "change-n-ed-kn", record: "ChangedNEdKn" };

    fn diff(&self, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1999Snapshot) -> Vec<En1999Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change design axial force N_Ed [kN] to {}", self.new_n_ed_kn)
    }
}
//#endregion 🔖️ChangeNEdKn
