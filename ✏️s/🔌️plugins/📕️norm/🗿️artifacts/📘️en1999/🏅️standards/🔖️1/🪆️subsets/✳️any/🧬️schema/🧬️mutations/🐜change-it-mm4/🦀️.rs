//! 🐜 `change-it-mm4` payload — changes the En1999 document's `i_t_mm4` (torsion constant I_t [mm4]).


use crate::artifacts::en1999::En1999Snapshot;
use crate::artifacts::en1999::diff::En1999Diff;
use crate::artifacts::en1999::mutations::En1999Mutation;
use crate::artifacts::en1999::mutations::change_i_t_mm4::ChangeITMm4;

//#region 🔖️ChangeITMm4
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeITMm4 {
    pub new_i_t_mm4: f64,
}

impl protocol::MutationKind<En1999Snapshot, En1999Mutation> for ChangeITMm4 {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "it-mm4", kind: "change-it-mm4", record: "ChangedITMm4" };

    fn diff(&self, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1999Snapshot) -> Vec<En1999Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change torsion constant I_t [mm4] to {}", self.new_i_t_mm4)
    }
}
//#endregion 🔖️ChangeITMm4
