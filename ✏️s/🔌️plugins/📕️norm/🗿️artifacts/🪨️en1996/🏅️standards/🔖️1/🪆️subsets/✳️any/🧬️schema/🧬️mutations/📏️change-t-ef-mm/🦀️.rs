//! 🔆 `change-t-ef-mm` payload — changes the En1996 document's `t_ef_mm` (effective thickness t_ef [mm]).


use crate::artifacts::en1996::En1996Snapshot;
use crate::artifacts::en1996::diff::En1996Diff;
use crate::artifacts::en1996::mutations::En1996Mutation;
//#region 🔖️ChangeTEfMm
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeTEfMm {
    pub new_t_ef_mm: f64,
}

impl protocol::MutationKind<En1996Snapshot, En1996Mutation> for ChangeTEfMm {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "t-ef-mm", kind: "change-t-ef-mm", record: "ChangedTEfMm" };

    fn diff(&self, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1996Snapshot) -> Vec<En1996Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change effective thickness t_ef [mm] to {}", self.new_t_ef_mm)
    }
}
//#endregion 🔖️ChangeTEfMm
