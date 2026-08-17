//! 🦎 `change-n-ed-kn` payload — changes the En1999 document's `n_ed_kn` (design axial force N_Ed [kN]).

use crate::artifacts::en1999::diff::En1999Diff;
use crate::artifacts::en1999::mutations::En1999Mutation;
use crate::artifacts::en1999::En1999Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeNEdKn
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeNEdKn {
    pub new_n_ed_kn: f64,
}

impl protocol::MutationKind<En1999Snapshot, En1999Mutation> for ChangeNEdKn {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "n-ed-kn", kind: "change-n-ed-kn", record: "ChangedNEdKn" };

    fn diff(&self, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
        crate::artifacts::en1999::mutations::change_n_ed_kn::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1999Snapshot) -> Vec<En1999Mutation> {
        crate::artifacts::en1999::mutations::change_n_ed_kn::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change design axial force N_Ed [kN] to {}", self.new_n_ed_kn)
    }
}
//#endregion 🔖️ChangeNEdKn
