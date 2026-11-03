use crate::change_taper_rods::ChangeTaperRods;
use semio_s_artifact_norm_en1995_diff::En1995Diff;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
use semio_s_artifact_norm_en1995_diff::{En1995TaperDelta, En1995TaperPatch};
pub fn diff(payload: &ChangeTaperRods, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
    let Some(row) = base.tapers.iter().find(|item| item.id == payload.taper_id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Unknown taper id.", vec![payload.taper_id.clone()]);
    };
    protocol::MutationOutcome::new(En1995Diff { tapers: En1995TaperDelta::modification(&row.id, En1995TaperPatch { rods: Some(payload.new_value), ..Default::default() }), ..Default::default() })
}
