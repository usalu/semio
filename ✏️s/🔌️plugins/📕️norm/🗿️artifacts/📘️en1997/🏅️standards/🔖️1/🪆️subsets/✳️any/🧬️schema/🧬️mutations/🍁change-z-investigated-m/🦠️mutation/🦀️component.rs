//! 🍁 `change-z-investigated-m` payload — changes the En1997 document's `z_investigated_m` (investigated depth [m]).

use crate::artifacts::en1997::diff::En1997Diff;
use crate::artifacts::en1997::mutations::En1997Mutation;
use crate::artifacts::en1997::En1997Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeZInvestigatedM
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeZInvestigatedM {
    pub new_z_investigated_m: f64,
}

impl protocol::MutationKind<En1997Snapshot, En1997Mutation> for ChangeZInvestigatedM {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "z-investigated-m", kind: "change-z-investigated-m", record: "ChangedZInvestigatedM" };

    async fn diff(&self, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
        crate::artifacts::en1997::mutations::change_z_investigated_m::diff::diff(self, base)
    }

    async fn inverse(&self, base: &En1997Snapshot) -> Vec<En1997Mutation> {
        crate::artifacts::en1997::mutations::change_z_investigated_m::inverse::inverse(self, base)
    }

    async fn label(&self) -> String {
        format!("Change investigated depth [m] to {}", self.new_z_investigated_m)
    }
}
//#endregion 🔖️ChangeZInvestigatedM
