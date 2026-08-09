//! 🧬️ present artifact — document mutation dispatch.

use crate::artifacts::present::diff::{
    diff_set_snapshot, tiles_delta_from_collection_mutation, tiles_delta_from_set_tiles, PresentDiff,
};
use crate::artifacts::present::{FigureTileDraft, FigureTileDraftPatch, FigureTileSource, PresentSnapshot};
use protocol::{inverse_collection_mutation, CollectionMutation, Mutation};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutations
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum PresentMutation {
    Tiles(CollectionMutation<String, FigureTileDraft, FigureTileDraftPatch>),
    SetSource { source: FigureTileSource },
    SetTiles { tiles: Vec<FigureTileDraft> },
    SetSnapshot { snapshot: PresentSnapshot },
}

impl Mutation<PresentSnapshot> for PresentMutation {
    type Diff = PresentDiff;

    fn diff(&self, snapshot: &PresentSnapshot) -> PresentDiff {
        match self {
            PresentMutation::Tiles(operation) => PresentDiff {
                tiles: Some(tiles_delta_from_collection_mutation(&snapshot.tiles, operation)),
                ..Default::default()
            },
            PresentMutation::SetSource { source } => PresentDiff { source: Some(source.clone()), ..Default::default() },
            PresentMutation::SetTiles { tiles } => PresentDiff {
                tiles: Some(tiles_delta_from_set_tiles(&snapshot.tiles, tiles)),
                ..Default::default()
            },
            PresentMutation::SetSnapshot { snapshot } => diff_set_snapshot(snapshot),
        }
    }

    fn inverse(&self, snapshot: &PresentSnapshot) -> Vec<Self> {
        match self {
            PresentMutation::Tiles(operation) => vec![PresentMutation::Tiles(inverse_collection_mutation(&snapshot.tiles, operation))],
            PresentMutation::SetSource { .. } => vec![PresentMutation::SetSource { source: snapshot.source.clone() }],
            PresentMutation::SetTiles { .. } => vec![PresentMutation::SetTiles { tiles: snapshot.tiles.clone() }],
            PresentMutation::SetSnapshot { .. } => vec![PresentMutation::SetSnapshot { snapshot: snapshot.clone() }],
        }
    }
}

pub fn apply_present_mutation(snapshot: &PresentSnapshot, mutation: &PresentMutation) -> PresentSnapshot {
    protocol::MutationDiff::apply(&mutation.diff(snapshot), snapshot)
}

pub fn inverse_present_mutation(snapshot: &PresentSnapshot, mutation: &PresentMutation) -> Vec<PresentMutation> {
    mutation.inverse(snapshot)
}
//#endregion 🔖️Mutations
