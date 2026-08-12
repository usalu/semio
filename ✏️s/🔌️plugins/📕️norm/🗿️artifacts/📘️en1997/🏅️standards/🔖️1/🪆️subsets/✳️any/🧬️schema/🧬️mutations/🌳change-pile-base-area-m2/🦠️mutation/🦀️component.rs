//! 🌳 `change-pile-base-area-m2` payload — changes the En1997 document's `pile_base_area_m2` (pile base area [m2]).

use crate::artifacts::en1997::diff::En1997Diff;
use crate::artifacts::en1997::mutations::En1997Mutation;
use crate::artifacts::en1997::En1997Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangePileBaseAreaM2
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangePileBaseAreaM2 {
    pub new_pile_base_area_m2: f64,
}

impl protocol::MutationKind<En1997Snapshot, En1997Mutation> for ChangePileBaseAreaM2 {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "pile-base-area-m2", kind: "change-pile-base-area-m2", record: "ChangedPileBaseAreaM2" };

    fn diff(&self, base: &En1997Snapshot) -> En1997Diff {
        crate::artifacts::en1997::mutations::change_pile_base_area_m2::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1997Snapshot) -> Vec<En1997Mutation> {
        crate::artifacts::en1997::mutations::change_pile_base_area_m2::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change pile base area [m2] to {}", self.new_pile_base_area_m2)
    }
}
//#endregion 🔖️ChangePileBaseAreaM2
