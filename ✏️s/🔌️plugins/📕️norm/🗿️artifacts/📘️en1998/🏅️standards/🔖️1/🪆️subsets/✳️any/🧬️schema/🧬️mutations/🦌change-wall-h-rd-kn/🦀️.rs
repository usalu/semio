//! 🦌 `change-wall-h-rd-kn` payload — changes the En1998 document's `wall_h_rd_kn` (wall horizontal resistance H_Rd [kN]).


use crate::artifacts::en1998::En1998Snapshot;
use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::En1998Mutation;
use crate::artifacts::en1998::mutations::change_wall_h_rd_kn::ChangeWallHRdKn;

//#region 🔖️ChangeWallHRdKn
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeWallHRdKn {
    pub new_wall_h_rd_kn: f64,
}

impl protocol::MutationKind<En1998Snapshot, En1998Mutation> for ChangeWallHRdKn {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "wall-h-rd-kn", kind: "change-wall-h-rd-kn", record: "ChangedWallHRdKn" };

    fn diff(&self, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1998Snapshot) -> Vec<En1998Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change wall horizontal resistance H_Rd [kN] to {}", self.new_wall_h_rd_kn)
    }
}
//#endregion 🔖️ChangeWallHRdKn
