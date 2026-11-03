use crate::change_floor_member::ChangeFloorMember;
use semio_s_artifact_norm_en1995_diff::En1995Diff;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
use semio_s_artifact_norm_en1995_diff::{En1995FloorDelta, En1995FloorPatch};
pub fn diff(payload: &ChangeFloorMember, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
    let Some(row) = base.floors.iter().find(|item| item.id == payload.floor_id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Unknown floor id.", vec![payload.floor_id.clone()]);
    };
    protocol::MutationOutcome::new(En1995Diff { floors: En1995FloorDelta::modification(&row.id, En1995FloorPatch { member_id: Some(payload.new_value.clone()), ..Default::default() }), ..Default::default() })
}
