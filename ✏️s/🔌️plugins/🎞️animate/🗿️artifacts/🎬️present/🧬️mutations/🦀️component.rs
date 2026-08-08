//! 🧬️ present artifact — document mutation dispatch.

use crate::artifacts::present::diff::PresentDiff;
use crate::artifacts::present::{FigureTileDraft, FigureTileDraftPatch, FigureTileSource, PresentDeck};
use protocol::{collection_diff_from_mutation, inverse_collection_mutation, CollectionMutation, Mutation};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutations
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum PresentMutation {
    Tiles(CollectionMutation<String, FigureTileDraft, FigureTileDraftPatch>),
    SetSource { source: FigureTileSource },
    SetTiles { tiles: Vec<FigureTileDraft> },
    SetDeck { deck: PresentDeck },
}

impl Mutation<PresentDeck> for PresentMutation {
    type Diff = PresentDiff;

    fn diff(&self, projection: &PresentDeck) -> PresentDiff {
        match self {
            PresentMutation::Tiles(operation) => PresentDiff { tiles: Some(collection_diff_from_mutation(&projection.tiles, operation)), ..Default::default() },
            PresentMutation::SetSource { source } => PresentDiff { source: Some(source.clone()), ..Default::default() },
            PresentMutation::SetTiles { tiles } => PresentDiff { set_tiles: Some(tiles.clone()), ..Default::default() },
            PresentMutation::SetDeck { deck } => PresentDiff { deck: Some(deck.clone()), ..Default::default() },
        }
    }

    fn inverse(&self, projection: &PresentDeck) -> Vec<Self> {
        match self {
            PresentMutation::Tiles(operation) => vec![PresentMutation::Tiles(inverse_collection_mutation(&projection.tiles, operation))],
            PresentMutation::SetSource { .. } => vec![PresentMutation::SetSource { source: projection.source.clone() }],
            PresentMutation::SetTiles { .. } => vec![PresentMutation::SetTiles { tiles: projection.tiles.clone() }],
            PresentMutation::SetDeck { .. } => vec![PresentMutation::SetDeck { deck: projection.clone() }],
        }
    }
}

/// ▶️ Applies `mutation` onto a deck clone via its diff.
pub fn apply_present_mutation(projection: &PresentDeck, mutation: &PresentMutation) -> PresentDeck {
    protocol::MutationDiff::apply(&mutation.diff(projection), projection)
}

pub fn inverse_present_mutation(projection: &PresentDeck, mutation: &PresentMutation) -> Vec<PresentMutation> {
    mutation.inverse(projection)
}
//#endregion 🔖️Mutations
