//! 🔺️ Sparse diff builder for `ChangeTileMedia` — an in-place upsert at the tile's EXISTING index;
//! the pattern universe, the rules and the pins all keep addressing the same id.

use crate::diff::Grid2dDiff;
use crate::schema::snapshot::Grid2dSnapshot;

pub fn diff(payload: &super::ChangeTileMedia, base: &Grid2dSnapshot) -> protocol::MutationOutcome<Grid2dDiff> {
    let Some(index) = base.tiles.iter().position(|tile| tile.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Tile \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if base.tiles[index].media == payload.media {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Tile \"{}\" already carries this media.", payload.id));
    }
    let mut tile = base.tiles[index].clone();
    tile.media = payload.media.clone();
    protocol::MutationOutcome::new(Grid2dDiff { tiles_upserted: vec![(index, tile)], ..Default::default() })
}
