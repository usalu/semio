//! 🔺️ Sparse diff builder for `DeleteTile` — removes the tile AND cascades to every rule naming it
//! and every cell pinned to it (a real BASE lookup, never a whole-snapshot capture).

use crate::diff::{cell_id, Grid2dDiff, Grid2dPinnedDelta, Grid2dRulesDelta, Grid2dTilesDelta};
use crate::schema::snapshot::Grid2dSnapshot;

pub fn diff(payload: &super::DeleteTile, base: &Grid2dSnapshot) -> protocol::MutationOutcome<Grid2dDiff> {
    if !base.tiles.iter().any(|tile| tile.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Tile \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    let rules_removed: Vec<String> = base.rules.iter().filter(|rule| rule.tile_a_id == payload.id || rule.tile_b_id == payload.id).map(|rule| rule.id.clone()).collect();
    let pinned_removed: Vec<String> = base.pinned.iter().filter(|cell| cell.tile_id == payload.id).map(|cell| cell_id(cell.x, cell.y)).collect();
    let cascaded = rules_removed.len() + pinned_removed.len();
    let outcome = protocol::MutationOutcome::new(Grid2dDiff { tiles: Grid2dTilesDelta::removal(&base.tiles, base.tiles.iter().position(|row| protocol::list_delta::Keyed::key(row) == payload.id.clone()).unwrap_or(usize::MAX)), rules: Grid2dRulesDelta::removals(&base.rules, &base.rules.iter().enumerate().filter(|(_, row)| rules_removed.contains(&protocol::list_delta::Keyed::key(*row))).map(|(index, _)| index).collect::<Vec<_>>()), pinned: Grid2dPinnedDelta::removals(&base.pinned, &base.pinned.iter().enumerate().filter(|(_, row)| pinned_removed.contains(&protocol::list_delta::Keyed::key(*row))).map(|(index, _)| index).collect::<Vec<_>>()), ..Default::default() });
    if cascaded == 0 {
        outcome
    } else {
        outcome.info("mutation.cascade", format!("Deleting tile \"{}\" also removed {cascaded} dependent row(s).", payload.id))
    }
}
