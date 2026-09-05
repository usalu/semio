//! 🔧 `change-hd-over-h` payload — changes the En1992 document's `hd_over_h` (EN 1992 input).


use crate::artifacts::en1992::En1992Snapshot;
use crate::artifacts::en1992::diff::En1992Diff;
use crate::artifacts::en1992::mutations::En1992Mutation;
//#region 🔖️ChangeHdOverH
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeHdOverH {
    pub new_hd_over_h: f64,
}

impl protocol::MutationKind<En1992Snapshot, En1992Mutation> for ChangeHdOverH {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "hd-over-h", kind: "change-hd-over-h", record: "ChangedHdOverH" };

    fn diff(&self, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1992Snapshot) -> Vec<En1992Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change hd over h to {:?}", self.new_hd_over_h)
    }
}
//#endregion 🔖️ChangeHdOverH
