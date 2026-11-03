use crate::remove_truss::RemoveTruss;
use semio_s_artifact_norm_en1995_diff::En1995Diff;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
use semio_s_artifact_norm_en1995_diff::En1995TrussDelta;
pub fn diff(payload: &RemoveTruss, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
    if payload.index >= base.trusses.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Truss index {} out of range.", payload.index), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1995Diff { trusses: En1995TrussDelta::removal(&base.trusses, payload.index), ..Default::default() })
}
