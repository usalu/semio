use crate::change_cutout_member::ChangeCutoutMember;
use semio_s_artifact_norm_en1995_diff::En1995Diff;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
use semio_s_artifact_norm_en1995_diff::{En1995CutoutDelta, En1995CutoutPatch};
pub fn diff(payload: &ChangeCutoutMember, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
    let Some(row) = base.cutouts.iter().find(|item| item.id == payload.cutout_id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Unknown cutout id.", vec![payload.cutout_id.clone()]);
    };
    protocol::MutationOutcome::new(En1995Diff { cutouts: En1995CutoutDelta::modification(&row.id, En1995CutoutPatch { member_id: Some(payload.new_value.clone()), ..Default::default() }), ..Default::default() })
}
