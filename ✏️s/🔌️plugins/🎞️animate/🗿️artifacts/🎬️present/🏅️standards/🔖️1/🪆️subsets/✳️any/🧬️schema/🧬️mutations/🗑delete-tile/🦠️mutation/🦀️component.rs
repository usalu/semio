//! 🗑️ `delete-tile` mutation payload — removes a figure tile crop from `tiles` by id.
use crate::artifacts::present::diff::PresentDiff;
use crate::artifacts::present::mutations::PresentMutation;
use crate::artifacts::present::PresentSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔹Payload
/// 🗑️ Removes the `tiles` entry addressed by `id` (BASE-state, per the taxonomy's index/id
/// addressing law). Diff/inverse delegate to the sibling `🔺️diff`/`↩️inverse` leaves.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "delete-tile")]
pub struct DeleteTile {
    pub id: String,
}

impl MutationKind<PresentSnapshot, PresentMutation> for DeleteTile {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "tile", kind: "delete-tile", record: "DeletedTile" };

    async fn diff(&self, base: &PresentSnapshot) -> protocol::MutationOutcome<PresentDiff> {
        super::diff::diff(self, base)
    }

    async fn inverse(&self, base: &PresentSnapshot) -> Vec<PresentMutation> {
        super::inverse::inverse(self, base)
    }

    async fn label(&self) -> String {
        format!("Delete tile \"{}\"", self.id)
    }

    async fn target(&self) -> Vec<String> {
        vec!["tiles".into(), self.id.clone()]
    }
}
//#endregion 🔹Payload
