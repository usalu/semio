//! 🧊 `change-fire-resistance-min` payload — changes the En1996 document's `fire_resistance_min` (fire resistance requirement [min]).

use crate::artifacts::en1996::diff::En1996Diff;
use crate::artifacts::en1996::mutations::En1996Mutation;
use crate::artifacts::en1996::En1996Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeFireResistanceMin
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeFireResistanceMin {
    pub new_fire_resistance_min: u32,
}

impl protocol::MutationKind<En1996Snapshot, En1996Mutation> for ChangeFireResistanceMin {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "fire-resistance-min", kind: "change-fire-resistance-min", record: "ChangedFireResistanceMin" };

    fn diff(&self, base: &En1996Snapshot) -> En1996Diff {
        crate::artifacts::en1996::mutations::change_fire_resistance_min::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1996Snapshot) -> Vec<En1996Mutation> {
        crate::artifacts::en1996::mutations::change_fire_resistance_min::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change fire resistance requirement [min] to {}", self.new_fire_resistance_min)
    }
}
//#endregion 🔖️ChangeFireResistanceMin
