//! 🔺️ Sparse diff builder for `ChangeTileMedia` — an in-place upsert at the tile's EXISTING index;
//! the pattern universe, the rules and the pins all keep addressing the same id.

use crate::diff::{Grid2dDiff, Grid2dRowPatch, Grid2dRows, Grid2dTilePatch};
use crate::schema::snapshot::Grid2dSnapshot;

pub fn diff(payload: &super::ChangeTileMedia, base: &Grid2dSnapshot) -> protocol::MutationOutcome<Grid2dDiff> {
    let Some(index) = base.tiles.iter().position(|tile| tile.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Tile \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if base.tiles[index].media == payload.media {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Tile \"{}\" already carries this media.", payload.id));
    }
    protocol::MutationOutcome::new(Grid2dDiff { tiles: Grid2dRows { patched: vec![Grid2dRowPatch { id: payload.id.clone(), patch: Grid2dTilePatch { media: Some(payload.media.clone()), ..Default::default() } }], ..Default::default() }, ..Default::default() })
}
