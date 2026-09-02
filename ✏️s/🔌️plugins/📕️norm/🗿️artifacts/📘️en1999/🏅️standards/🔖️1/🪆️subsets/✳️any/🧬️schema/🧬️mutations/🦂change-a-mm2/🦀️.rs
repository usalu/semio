//! 🦂 `change-a-mm2` payload — changes the En1999 document's `a_mm2` (cross-section area [mm2]).


use crate::artifacts::en1999::En1999Snapshot;
use crate::artifacts::en1999::diff::En1999Diff;
use crate::artifacts::en1999::mutations::En1999Mutation;
use crate::artifacts::en1999::mutations::change_a_mm2::ChangeAMm2;

//#region 🔖️ChangeAMm2
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeAMm2 {
    pub new_a_mm2: f64,
}

impl protocol::MutationKind<En1999Snapshot, En1999Mutation> for ChangeAMm2 {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "a-mm2", kind: "change-a-mm2", record: "ChangedAMm2" };

    fn diff(&self, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1999Snapshot) -> Vec<En1999Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change cross-section area [mm2] to {}", self.new_a_mm2)
    }
}
//#endregion 🔖️ChangeAMm2
