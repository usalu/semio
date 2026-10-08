//! 🔺️ Sparse diff construction for `delete-tiles`.
use super::DeleteTiles;
use crate::diff::{PresentationDiff, PresentationSourcePatch, PresentationTilePatch, PresentationTilesDelta};
use crate::PresentationSnapshot;

//#region 🔹Diff
/// 🔺️ Reads the working-scene `(source, tiles)` off `base.presentation`, removes every addressed
/// tile, and mints a new content-addressed `presentation` handle for the result — real handcrafted
/// construction from `(payload, base)`, never apply-then-capture.
pub fn diff(payload: &DeleteTiles, base: &PresentationSnapshot) -> protocol::MutationOutcome<PresentationDiff> {
    let existing_ids: std::collections::HashSet<&str> = base.tiles.iter().map(|tile| tile.id.as_str()).collect();
    let missing: Vec<String> = payload.ids.iter().filter(|id| !existing_ids.contains(id.as_str())).cloned().collect();
    if missing.len() == payload.ids.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("No addressed tile(s) exist: {}.", missing.join(", ")), {
            let mut target = vec!["tiles".to_string()];
            target.extend(missing);
            target
        });
    }
    let indices: Vec<usize> = base.tiles.iter().enumerate().filter(|(_, tile)| payload.ids.contains(&tile.id)).map(|(index, _)| index).collect();
    let outcome = protocol::MutationOutcome::new(PresentationDiff { tiles: Some(PresentationTilesDelta::removals(&base.tiles, &indices)), ..Default::default() });
    if missing.is_empty() {
        outcome
    } else {
        outcome.absorb_messages([protocol::MutationMessage::warning("mutation.partial", format!("{} of {} addressed tile(s) did not exist: {}.", missing.len(), payload.ids.len(), missing.join(", "))).at({
            let mut target = vec!["tiles".to_string()];
            target.extend(missing);
            target
        })])
    }
}
//#endregion 🔹Diff
