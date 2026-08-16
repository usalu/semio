//! 🔺️ Sparse diff construction for `change-exaggeration`.
use super::mutation::ChangeExaggeration;
use crate::artifacts::gisterrain::diff::GisTerrainDiff;
use crate::artifacts::gisterrain::GisTerrainSnapshot;

//#region 🔹Diff
/// 🔺️ Builds the sparse `exaggeration` field delta directly from the payload — real handcrafted
/// construction, never apply-then-capture, never a snapshot clone. Warning `no-op` when
/// `new_exaggeration` already equals `base.exaggeration`.
pub fn diff(payload: &ChangeExaggeration, base: &GisTerrainSnapshot) -> protocol::MutationOutcome<GisTerrainDiff> {
    if base.exaggeration == payload.new_exaggeration {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Exaggeration is already {}.", payload.new_exaggeration));
    }
    protocol::MutationOutcome::new(crate::artifacts::gisterrain::diff::diff_exaggeration(payload.new_exaggeration))
}
//#endregion 🔹Diff
