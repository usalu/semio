use crate::remove_cutout::RemoveCutout;
use semio_s_artifact_norm_en1995_diff::En1995Diff;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
use semio_s_artifact_norm_en1995_diff::En1995CutoutDelta;
pub fn diff(payload: &RemoveCutout, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
    if payload.index >= base.cutouts.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Cutout index {} out of range.", payload.index), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1995Diff { cutouts: En1995CutoutDelta::removal(&base.cutouts, payload.index), ..Default::default() })
}
