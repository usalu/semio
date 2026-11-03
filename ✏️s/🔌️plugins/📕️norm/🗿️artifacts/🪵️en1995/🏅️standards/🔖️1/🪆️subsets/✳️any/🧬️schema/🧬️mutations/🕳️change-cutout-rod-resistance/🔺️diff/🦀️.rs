use crate::change_cutout_rod_resistance::ChangeCutoutRodResistance;
use semio_s_artifact_norm_en1995_diff::En1995Diff;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
use semio_s_artifact_norm_en1995_diff::{En1995CutoutDelta, En1995CutoutPatch};
pub fn diff(payload: &ChangeCutoutRodResistance, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
    let Some(row) = base.cutouts.iter().find(|item| item.id == payload.cutout_id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Unknown cutout id.", vec![payload.cutout_id.clone()]);
    };
    if !payload.new_value.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1995Diff { cutouts: En1995CutoutDelta::modification(&row.id, En1995CutoutPatch { rod_resistance_n: Some(payload.new_value), ..Default::default() }), ..Default::default() })
}
