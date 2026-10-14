//! 🔺️ Diff for `remove-roofs`.
use crate::remove_roofs::RemoveRoofs;
use semio_s_artifact_norm_en1991_diff::En1991Diff;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
use semio_s_artifact_norm_en1991_diff::En1991RoofDelta;
pub fn diff(payload: &RemoveRoofs, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if payload.index >= base.roofs.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "Index out of range.", [payload.index.to_string()]);
    }
    protocol::MutationOutcome::new(En1991Diff { roofs: En1991RoofDelta::removal(&base.roofs, payload.index), ..Default::default() })
}
