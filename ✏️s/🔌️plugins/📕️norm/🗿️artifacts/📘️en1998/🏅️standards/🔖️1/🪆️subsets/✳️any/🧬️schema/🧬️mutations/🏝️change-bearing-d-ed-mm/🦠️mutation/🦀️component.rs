//! 🏝️ `change-bearing-d-ed-mm` payload — changes the En1998 document's `bearing_d_ed_mm` (bearing design displacement D_Ed [mm]).

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::En1998Mutation;
use crate::artifacts::en1998::En1998Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeBearingDEdMm
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeBearingDEdMm {
    pub new_bearing_d_ed_mm: f64,
}

impl protocol::MutationKind<En1998Snapshot, En1998Mutation> for ChangeBearingDEdMm {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "bearing-d-ed-mm", kind: "change-bearing-d-ed-mm", record: "ChangedBearingDEdMm" };

    fn diff(&self, base: &En1998Snapshot) -> En1998Diff {
        crate::artifacts::en1998::mutations::change_bearing_d_ed_mm::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1998Snapshot) -> Vec<En1998Mutation> {
        crate::artifacts::en1998::mutations::change_bearing_d_ed_mm::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change bearing design displacement D_Ed [mm] to {}", self.new_bearing_d_ed_mm)
    }
}
//#endregion 🔖️ChangeBearingDEdMm
