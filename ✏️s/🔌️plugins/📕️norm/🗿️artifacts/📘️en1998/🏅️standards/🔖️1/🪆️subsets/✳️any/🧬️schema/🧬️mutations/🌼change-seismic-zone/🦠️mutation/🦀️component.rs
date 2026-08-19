//! 🌼 `change-seismic-zone` payload — changes the En1998 document's `seismic_zone` (seismic zone).

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::En1998Mutation;
use crate::artifacts::en1998::En1998Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeSeismicZone
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeSeismicZone {
    pub new_seismic_zone: u8,
}

impl protocol::MutationKind<En1998Snapshot, En1998Mutation> for ChangeSeismicZone {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "seismic-zone", kind: "change-seismic-zone", record: "ChangedSeismicZone" };

    async fn diff(&self, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
        crate::artifacts::en1998::mutations::change_seismic_zone::diff::diff(self, base)
    }

    async fn inverse(&self, base: &En1998Snapshot) -> Vec<En1998Mutation> {
        crate::artifacts::en1998::mutations::change_seismic_zone::inverse::inverse(self, base)
    }

    async fn label(&self) -> String {
        format!("Change seismic zone to {}", self.new_seismic_zone)
    }
}
//#endregion 🔖️ChangeSeismicZone
