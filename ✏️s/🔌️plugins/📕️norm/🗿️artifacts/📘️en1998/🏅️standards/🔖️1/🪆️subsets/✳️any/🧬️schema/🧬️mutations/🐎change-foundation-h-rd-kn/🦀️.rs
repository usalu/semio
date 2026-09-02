//! 🐎 `change-foundation-h-rd-kn` payload — changes the En1998 document's `foundation_h_rd_kn` (foundation horizontal resistance H_Rd [kN]).


use crate::artifacts::en1998::En1998Snapshot;
use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::En1998Mutation;
use crate::artifacts::en1998::mutations::change_foundation_h_rd_kn::ChangeFoundationHRdKn;

//#region 🔖️ChangeFoundationHRdKn
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeFoundationHRdKn {
    pub new_foundation_h_rd_kn: f64,
}

impl protocol::MutationKind<En1998Snapshot, En1998Mutation> for ChangeFoundationHRdKn {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "foundation-h-rd-kn", kind: "change-foundation-h-rd-kn", record: "ChangedFoundationHRdKn" };

    fn diff(&self, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1998Snapshot) -> Vec<En1998Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change foundation horizontal resistance H_Rd [kN] to {}", self.new_foundation_h_rd_kn)
    }
}
//#endregion 🔖️ChangeFoundationHRdKn
