//! 🆕️ `create-tile` mutation payload — adds a new figure tile crop to `tiles`.
use crate::artifacts::present::diff::PresentDiff;
use crate::artifacts::present::mutations::PresentMutation;
use crate::artifacts::present::{FigureTileDraft, PresentSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔹Payload
/// 🆕️ Inserts `tile` into `tiles` at `index` (FINAL-state, per the taxonomy's index-addressing
/// law — `tiles` is id-keyed, so `index` only determines append order, never lookup). Diff/inverse
/// delegate to the sibling `🔺️diff`/`↩️inverse` leaves.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "create-tile")]
pub struct CreateTile {
    pub index: usize,
    #[dsl(block)]
    pub tile: FigureTileDraft,
}

impl MutationKind<PresentSnapshot, PresentMutation> for CreateTile {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "tile", kind: "create-tile", record: "CreatedTile" };

    async fn diff(&self, base: &PresentSnapshot) -> protocol::MutationOutcome<PresentDiff> {
        super::diff::diff(self, base)
    }

    async fn inverse(&self, base: &PresentSnapshot) -> Vec<PresentMutation> {
        super::inverse::inverse(self, base)
    }

    async fn label(&self) -> String {
        format!("Create tile \"{}\"", self.tile.name)
    }

    async fn target(&self) -> Vec<String> {
        vec!["tiles".into(), self.tile.id.clone()]
    }
}
//#endregion 🔹Payload
