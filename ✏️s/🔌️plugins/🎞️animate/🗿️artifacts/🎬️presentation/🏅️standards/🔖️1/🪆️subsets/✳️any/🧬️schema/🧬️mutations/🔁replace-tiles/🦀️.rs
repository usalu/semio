//! 🔁️ `replace-tiles` mutation payload — whole-value swaps the `tiles` collection (the play app's
//! grid-reseed and source-change-clears-tiles gestures always regenerate the whole set at once, so
//! this is `replace` on the collection field, per the taxonomy's rule 6 "targeted verb, not a
//! document-level snapshot swap" guidance).

use crate::artifacts::presentation::{FigureTileDraft, PresentationSnapshot};
use crate::artifacts::presentation::diff::PresentationDiff;
use crate::artifacts::presentation::mutations::PresentationMutation;
use protocol::{MutationKind, SemanticDescriptor};

//#region 🔹Payload
/// 🔁️ Replaces `tiles` with `new_tiles` wholesale (an empty `new_tiles` is the "clear tiles"
/// gesture — no separate `clear-tiles` verb is needed since `replace`'s own inverse already
/// restores whatever was cleared). Diff/inverse delegate to the sibling `🔺️diff`/`↩️inverse` leaves.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "replace-tiles")]
pub struct ReplaceTiles {
    #[dsl(table)]
    pub new_tiles: Vec<FigureTileDraft>,
}

impl MutationKind<PresentationSnapshot, PresentationMutation> for ReplaceTiles {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "tiles", kind: "replace-tiles", record: "ReplacedTiles" };

    fn diff(&self, base: &PresentationSnapshot) -> protocol::MutationOutcome<PresentationDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &PresentationSnapshot) -> Vec<PresentationMutation> {
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
