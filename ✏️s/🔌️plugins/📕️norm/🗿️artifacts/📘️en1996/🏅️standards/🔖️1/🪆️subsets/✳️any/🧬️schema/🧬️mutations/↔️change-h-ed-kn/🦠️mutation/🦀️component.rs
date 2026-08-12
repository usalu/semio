//! ↔️ `change-h-ed-kn` payload — changes the En1996 document's `h_ed_kn` (design horizontal force H_Ed [kN]).

use crate::artifacts::en1996::diff::En1996Diff;
use crate::artifacts::en1996::mutations::En1996Mutation;
use crate::artifacts::en1996::En1996Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeHEdKn
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeHEdKn {
    pub new_h_ed_kn: f64,
}

impl protocol::MutationKind<En1996Snapshot, En1996Mutation> for ChangeHEdKn {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "h-ed-kn", kind: "change-h-ed-kn", record: "ChangedHEdKn" };

    fn diff(&self, base: &En1996Snapshot) -> En1996Diff {
        crate::artifacts::en1996::mutations::change_h_ed_kn::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1996Snapshot) -> Vec<En1996Mutation> {
        crate::artifacts::en1996::mutations::change_h_ed_kn::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change design horizontal force H_Ed [kN] to {}", self.new_h_ed_kn)
    }
}
//#endregion 🔖️ChangeHEdKn
