//! 🗑️ `delete-tiles` mutation payload — removes multiple figure tile crops from `tiles` by id (the
//! `delete-selection` multi-select editor gesture; a real plural mutation per the taxonomy's
//! bulk/plural rule, never a bare `Vec` arg bolted onto the singular `delete-tile`).
use crate::artifacts::present::diff::PresentDiff;
use crate::artifacts::present::mutations::PresentMutation;
use crate::artifacts::present::PresentSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔹Payload
/// 🗑️ Removes every `tiles` entry addressed by `ids` (BASE-state). Diff/inverse delegate to the
/// sibling `🔺️diff`/`↩️inverse` leaves.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "delete-tiles")]
pub struct DeleteTiles {
    pub ids: Vec<String>,
}

impl MutationKind<PresentSnapshot, PresentMutation> for DeleteTiles {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "tiles", kind: "delete-tiles", record: "DeletedTiles" };

    async fn diff(&self, base: &PresentSnapshot) -> protocol::MutationOutcome<PresentDiff> {
        super::diff::diff(self, base)
    }

    async fn inverse(&self, base: &PresentSnapshot) -> Vec<PresentMutation> {
        super::inverse::inverse(self, base)
    }

    async fn label(&self) -> String {
        format!("Delete {} tiles", self.ids.len())
    }

    async fn target(&self) -> Vec<String> {
        let mut target = vec!["tiles".to_string()];
        target.extend(self.ids.iter().cloned());
        target
    }
}
//#endregion 🔹Payload
