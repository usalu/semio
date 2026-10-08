//! 🔺️ Sparse diff construction for `rename-tile`.
use super::RenameTile;
use crate::diff::{diff_set_presentation, PresentationDiff, PresentationSourcePatch, PresentationTilePatch, PresentationTilesDelta};
use crate::PresentationSnapshot;

//#region 🔹Diff
/// 🔺️ Reads the working-scene `(source, tiles)` off `base.presentation`, applies the name-only
/// patch to the addressed tile, and mints a new content-addressed `presentation` handle for the
/// result — real handcrafted construction from `(payload, base)`, never apply-then-capture.
pub fn diff(payload: &RenameTile, base: &PresentationSnapshot) -> protocol::MutationOutcome<PresentationDiff> {
    let Some(existing) = base.tiles.iter().find(|tile| tile.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Tile \"{}\" does not exist.", payload.id), ["tiles".to_string(), payload.id.clone()]);
    };
    if existing.name == payload.new_name {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Tile \"{}\" is already named \"{}\".", payload.id, payload.new_name));
    }
    protocol::MutationOutcome::new(diff_set_presentation(base, None, Some(PresentationTilesDelta { patched: vec![PresentationTilePatch { id: payload.id.clone(), name: Some(payload.new_name.clone()), ..Default::default() }], ..Default::default() })))
}
//#endregion 🔹Diff
