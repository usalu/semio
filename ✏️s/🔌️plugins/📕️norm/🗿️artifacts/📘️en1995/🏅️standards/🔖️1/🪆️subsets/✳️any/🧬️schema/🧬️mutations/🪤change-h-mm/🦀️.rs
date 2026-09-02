//! 🔧 `change-h-mm` payload — changes the En1995 document's `h_mm` (EN 1995 input).


use crate::artifacts::en1995::En1995Snapshot;
use crate::artifacts::en1995::diff::En1995Diff;
use crate::artifacts::en1995::mutations::En1995Mutation;
use crate::artifacts::en1995::mutations::change_h_mm::ChangeHMm;

//#region 🔖️ChangeHMm
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeHMm {
    pub new_h_mm: f64,
}

impl protocol::MutationKind<En1995Snapshot, En1995Mutation> for ChangeHMm {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "h-mm", kind: "change-h-mm", record: "ChangedHMm" };

    fn diff(&self, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1995Snapshot) -> Vec<En1995Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change h mm to {:?}", self.new_h_mm)
    }
}
//#endregion 🔖️ChangeHMm
