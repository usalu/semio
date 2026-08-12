//! 🔽 `change-n-ed-kn` payload — changes the En1996 document's `n_ed_kn` (design axial force N_Ed [kN]).

use crate::artifacts::en1996::diff::En1996Diff;
use crate::artifacts::en1996::mutations::En1996Mutation;
use crate::artifacts::en1996::En1996Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeNEdKn
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeNEdKn {
    pub new_n_ed_kn: f64,
}

impl protocol::MutationKind<En1996Snapshot, En1996Mutation> for ChangeNEdKn {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "n-ed-kn", kind: "change-n-ed-kn", record: "ChangedNEdKn" };

    fn diff(&self, base: &En1996Snapshot) -> En1996Diff {
        crate::artifacts::en1996::mutations::change_n_ed_kn::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1996Snapshot) -> Vec<En1996Mutation> {
        crate::artifacts::en1996::mutations::change_n_ed_kn::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change design axial force N_Ed [kN] to {}", self.new_n_ed_kn)
    }
}
//#endregion 🔖️ChangeNEdKn
