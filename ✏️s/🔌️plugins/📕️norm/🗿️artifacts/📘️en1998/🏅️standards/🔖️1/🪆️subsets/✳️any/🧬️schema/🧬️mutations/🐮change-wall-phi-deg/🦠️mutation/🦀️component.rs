//! 🐮 `change-wall-phi-deg` payload — changes the En1998 document's `wall_phi_deg` (wall backfill friction angle [deg]).

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::En1998Mutation;
use crate::artifacts::en1998::En1998Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeWallPhiDeg
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeWallPhiDeg {
    pub new_wall_phi_deg: f64,
}

impl protocol::MutationKind<En1998Snapshot, En1998Mutation> for ChangeWallPhiDeg {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "wall-phi-deg", kind: "change-wall-phi-deg", record: "ChangedWallPhiDeg" };

    async fn diff(&self, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
        crate::artifacts::en1998::mutations::change_wall_phi_deg::diff::diff(self, base)
    }

    async fn inverse(&self, base: &En1998Snapshot) -> Vec<En1998Mutation> {
        crate::artifacts::en1998::mutations::change_wall_phi_deg::inverse::inverse(self, base)
    }

    async fn label(&self) -> String {
        format!("Change wall backfill friction angle [deg] to {}", self.new_wall_phi_deg)
    }
}
//#endregion 🔖️ChangeWallPhiDeg
