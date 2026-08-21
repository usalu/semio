//! 🔺️ Sparse diff construction for `delete-tiles`.
use super::mutation::DeleteTiles;
use crate::artifacts::present::diff::PresentDiff;
use crate::artifacts::present::PresentSnapshot;

//#region 🔹Diff
/// 🔺️ Reads the working-scene `(source, tiles)` off `base.presentation`, removes every addressed
/// tile, and mints a new content-addressed `presentation` handle for the result — real handcrafted
/// construction from `(payload, base)`, never apply-then-capture.
pub async fn diff(payload: &DeleteTiles, base: &PresentSnapshot) -> protocol::MutationOutcome<PresentDiff> {
    let (source, mut tiles) = crate::artifacts::present::present_working_scene(base);
    let targets: std::collections::HashSet<&str> = payload.ids.iter().map(String::as_str).collect();
    let existing_ids: std::collections::HashSet<&str> = tiles.iter().map(|tile| tile.id.as_str()).collect();
    let missing: Vec<String> = payload.ids.iter().filter(|id| !existing_ids.contains(id.as_str())).cloned().collect();
    if missing.len() == payload.ids.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("No addressed tile(s) exist: {}.", missing.join(", ")), {
            let mut target = vec!["tiles".to_string()];
            target.extend(missing);
            target
        });
    }
    tiles.retain(|tile| !targets.contains(tile.id.as_str()));
    let outcome = protocol::MutationOutcome::new(crate::artifacts::present::diff::diff_set_presentation(&source, &tiles));
    if missing.is_empty() {
        outcome
    } else {
        outcome.absorb_messages([protocol::MutationMessage::warn("mutation.partial", format!("{} of {} addressed tile(s) did not exist: {}.", missing.len(), payload.ids.len(), missing.join(", "))).at({
            let mut target = vec!["tiles".to_string()];
            target.extend(missing);
            target
        })])
    }
}
//#endregion 🔹Diff
