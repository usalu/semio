//! 🦇 `change-foundation-area-m2` payload — changes the En1998 document's `foundation_area_m2` (foundation area [m2]).

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::En1998Mutation;
use crate::artifacts::en1998::En1998Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeFoundationAreaM2
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeFoundationAreaM2 {
    pub new_foundation_area_m2: f64,
}

impl protocol::MutationKind<En1998Snapshot, En1998Mutation> for ChangeFoundationAreaM2 {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "foundation-area-m2", kind: "change-foundation-area-m2", record: "ChangedFoundationAreaM2" };

    async fn diff(&self, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
        crate::artifacts::en1998::mutations::change_foundation_area_m2::diff::diff(self, base)
    }

    async fn inverse(&self, base: &En1998Snapshot) -> Vec<En1998Mutation> {
        crate::artifacts::en1998::mutations::change_foundation_area_m2::inverse::inverse(self, base)
    }

    async fn label(&self) -> String {
        format!("Change foundation area [m2] to {}", self.new_foundation_area_m2)
    }
}
//#endregion 🔖️ChangeFoundationAreaM2
