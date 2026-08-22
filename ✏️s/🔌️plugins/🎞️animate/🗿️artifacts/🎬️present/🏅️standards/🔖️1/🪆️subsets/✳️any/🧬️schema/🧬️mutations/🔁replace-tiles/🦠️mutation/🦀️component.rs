//! 🔁️ `replace-tiles` mutation payload — whole-value swaps the `tiles` collection (the play app's
//! grid-reseed and source-change-clears-tiles gestures always regenerate the whole set at once, so
//! this is `replace` on the collection field, per the taxonomy's rule 6 "targeted verb, not a
//! document-level snapshot swap" guidance).
use crate::artifacts::present::diff::PresentDiff;
use crate::artifacts::present::mutations::PresentMutation;
use crate::artifacts::present::{FigureTileDraft, PresentSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔹Payload
/// 🔁️ Replaces `tiles` with `new_tiles` wholesale (an empty `new_tiles` is the "clear tiles"
/// gesture — no separate `clear-tiles` verb is needed since `replace`'s own inverse already
/// restores whatever was cleared). Diff/inverse delegate to the sibling `🔺️diff`/`↩️inverse` leaves.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "replace-tiles")]
pub struct ReplaceTiles {
    #[dsl(table)]
    pub new_tiles: Vec<FigureTileDraft>,
}

impl MutationKind<PresentSnapshot, PresentMutation> for ReplaceTiles {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "tiles", kind: "replace-tiles", record: "ReplacedTiles" };

    fn diff(&self, base: &PresentSnapshot) -> protocol::MutationOutcome<PresentDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &PresentSnapshot) -> Vec<PresentMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Replace tiles with {} tiles", self.new_tiles.len())
    }

    fn target(&self) -> Vec<String> {
        vec!["tiles".into()]
    }
}
//#endregion 🔹Payload
