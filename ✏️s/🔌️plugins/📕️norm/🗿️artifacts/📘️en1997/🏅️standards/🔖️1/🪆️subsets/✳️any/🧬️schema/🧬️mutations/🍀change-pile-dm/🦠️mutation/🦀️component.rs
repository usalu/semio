//! 🍀 `change-pile-dm` payload — changes the En1997 document's `pile_d_m` (pile diameter [m]).

use crate::artifacts::en1997::diff::En1997Diff;
use crate::artifacts::en1997::mutations::En1997Mutation;
use crate::artifacts::en1997::En1997Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangePileDM
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangePileDM {
    pub new_pile_d_m: f64,
}

impl protocol::MutationKind<En1997Snapshot, En1997Mutation> for ChangePileDM {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "pile-dm", kind: "change-pile-dm", record: "ChangedPileDM" };

    fn diff(&self, base: &En1997Snapshot) -> En1997Diff {
        crate::artifacts::en1997::mutations::change_pile_d_m::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1997Snapshot) -> Vec<En1997Mutation> {
        crate::artifacts::en1997::mutations::change_pile_d_m::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change pile diameter [m] to {}", self.new_pile_d_m)
    }
}
//#endregion 🔖️ChangePileDM
