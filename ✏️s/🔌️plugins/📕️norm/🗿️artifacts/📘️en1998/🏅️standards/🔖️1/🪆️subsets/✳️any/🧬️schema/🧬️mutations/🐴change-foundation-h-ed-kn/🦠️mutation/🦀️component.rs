//! 🐴 `change-foundation-h-ed-kn` payload — changes the En1998 document's `foundation_h_ed_kn` (foundation design horizontal force H_Ed [kN]).

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::En1998Mutation;
use crate::artifacts::en1998::En1998Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeFoundationHEdKn
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeFoundationHEdKn {
    pub new_foundation_h_ed_kn: f64,
}

impl protocol::MutationKind<En1998Snapshot, En1998Mutation> for ChangeFoundationHEdKn {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "foundation-h-ed-kn", kind: "change-foundation-h-ed-kn", record: "ChangedFoundationHEdKn" };

    fn diff(&self, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
        crate::artifacts::en1998::mutations::change_foundation_h_ed_kn::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1998Snapshot) -> Vec<En1998Mutation> {
        crate::artifacts::en1998::mutations::change_foundation_h_ed_kn::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change foundation design horizontal force H_Ed [kN] to {}", self.new_foundation_h_ed_kn)
    }
}
//#endregion 🔖️ChangeFoundationHEdKn
