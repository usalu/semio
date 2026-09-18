//! 🔺️ Sparse diff builder for `DeleteTile` — removes the tile AND cascades to every rule naming it
//! and every cell pinned to it (a real BASE lookup, never a whole-snapshot capture).

use crate::diff::{cell_id, Grid2dDiff};
use crate::schema::snapshot::Grid2dSnapshot;

pub fn diff(payload: &super::DeleteTile, base: &Grid2dSnapshot) -> protocol::MutationOutcome<Grid2dDiff> {
    if !base.tiles.iter().any(|tile| tile.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Tile \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    let rules_removed: Vec<String> = base.rules.iter().filter(|rule| rule.tile_a_id == payload.id || rule.tile_b_id == payload.id).map(|rule| rule.id.clone()).collect();
    let pinned_removed: Vec<String> = base.pinned.iter().filter(|cell| cell.tile_id == payload.id).map(|cell| cell_id(cell.x, cell.y)).collect();
    let cascaded = rules_removed.len() + pinned_removed.len();
    let outcome = protocol::MutationOutcome::new(Grid2dDiff { tiles_removed: vec![payload.id.clone()], rules_removed, pinned_removed, ..Default::default() });
    if cascaded == 0 {
        outcome
    } else {
        outcome.info("mutation.cascade", format!("Deleting tile \"{}\" also removed {cascaded} dependent row(s).", payload.id))
    }
}
