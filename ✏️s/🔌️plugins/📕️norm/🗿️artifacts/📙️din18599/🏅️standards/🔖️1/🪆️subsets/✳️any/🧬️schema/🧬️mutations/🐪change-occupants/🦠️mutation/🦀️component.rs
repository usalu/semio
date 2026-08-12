//! 🐪 `change-occupants` payload — changes the Din18599 document's `occupants` (number of occupants).

use crate::artifacts::din18599::diff::Din18599Diff;
use crate::artifacts::din18599::mutations::Din18599Mutation;
use crate::artifacts::din18599::Din18599Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeOccupants
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeOccupants {
    pub new_occupants: u32,
}

impl protocol::MutationKind<Din18599Snapshot, Din18599Mutation> for ChangeOccupants {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "occupants", kind: "change-occupants", record: "ChangedOccupants" };

    fn diff(&self, base: &Din18599Snapshot) -> Din18599Diff {
        crate::artifacts::din18599::mutations::change_occupants::diff::diff(self, base)
    }

    fn inverse(&self, base: &Din18599Snapshot) -> Vec<Din18599Mutation> {
        crate::artifacts::din18599::mutations::change_occupants::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change number of occupants to {}", self.new_occupants)
    }
}
//#endregion 🔖️ChangeOccupants
