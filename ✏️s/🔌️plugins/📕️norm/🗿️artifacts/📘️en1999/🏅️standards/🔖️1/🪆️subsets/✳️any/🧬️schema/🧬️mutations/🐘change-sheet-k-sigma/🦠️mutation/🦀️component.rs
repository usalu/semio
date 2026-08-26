//! 🐘 `change-sheet-k-sigma` payload — changes the En1999 document's `sheet_k_sigma` (sheet plate buckling factor k_sigma).

use crate::artifacts::en1999::diff::En1999Diff;
use crate::artifacts::en1999::mutations::En1999Mutation;
use crate::artifacts::en1999::En1999Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeSheetKSigma
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeSheetKSigma {
    pub new_sheet_k_sigma: f64,
}

impl protocol::MutationKind<En1999Snapshot, En1999Mutation> for ChangeSheetKSigma {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "sheet-k-sigma", kind: "change-sheet-k-sigma", record: "ChangedSheetKSigma" };

    fn diff(&self, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
        crate::artifacts::en1999::mutations::change_sheet_k_sigma::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1999Snapshot) -> Vec<En1999Mutation> {
        crate::artifacts::en1999::mutations::change_sheet_k_sigma::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change sheet plate buckling factor k_sigma to {}", self.new_sheet_k_sigma)
    }
}
//#endregion 🔖️ChangeSheetKSigma
