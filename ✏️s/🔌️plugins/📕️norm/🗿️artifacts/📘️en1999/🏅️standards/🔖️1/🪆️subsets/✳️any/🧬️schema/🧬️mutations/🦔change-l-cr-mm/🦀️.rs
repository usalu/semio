//! 🦔 `change-l-cr-mm` payload — changes the En1999 document's `l_cr_mm` (buckling length L_cr [mm]).


use crate::artifacts::en1999::En1999Snapshot;
use crate::artifacts::en1999::diff::En1999Diff;
use crate::artifacts::en1999::mutations::En1999Mutation;
use crate::artifacts::en1999::mutations::change_l_cr_mm::ChangeLCrMm;

//#region 🔖️ChangeLCrMm
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeLCrMm {
    pub new_l_cr_mm: f64,
}

impl protocol::MutationKind<En1999Snapshot, En1999Mutation> for ChangeLCrMm {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "l-cr-mm", kind: "change-l-cr-mm", record: "ChangedLCrMm" };

    fn diff(&self, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1999Snapshot) -> Vec<En1999Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change buckling length L_cr [mm] to {}", self.new_l_cr_mm)
    }
}
//#endregion 🔖️ChangeLCrMm
