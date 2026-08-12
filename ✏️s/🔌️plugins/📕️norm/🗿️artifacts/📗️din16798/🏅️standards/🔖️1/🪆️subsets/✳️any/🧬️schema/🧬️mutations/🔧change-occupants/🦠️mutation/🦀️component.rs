//! 🔧 `change-occupants` payload — changes the Din16798 document's `occupants` (number of occupants).

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::Din16798Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeOccupants
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeOccupants {
    pub new_occupants: u32,
}

impl protocol::MutationKind<Din16798Snapshot, Din16798Mutation> for ChangeOccupants {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "occupants", kind: "change-occupants", record: "ChangedOccupants" };

    fn diff(&self, base: &Din16798Snapshot) -> Din16798Diff {
        crate::artifacts::din16798::mutations::change_occupants::diff::diff(self, base)
    }

    fn inverse(&self, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
        crate::artifacts::din16798::mutations::change_occupants::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change number of occupants to {}", self.new_occupants)
    }
}
//#endregion 🔖️ChangeOccupants
