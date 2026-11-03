use crate::change_floor_damping::ChangeFloorDamping;
use semio_s_artifact_norm_en1995_diff::En1995Diff;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
use semio_s_artifact_norm_en1995_diff::{En1995FloorDelta, En1995FloorPatch};
pub fn diff(payload: &ChangeFloorDamping, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
    let Some(row) = base.floors.iter().find(|item| item.id == payload.floor_id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Unknown floor id.", vec![payload.floor_id.clone()]);
    };
    if !payload.new_value.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1995Diff { floors: En1995FloorDelta::modification(&row.id, En1995FloorPatch { damping_ratio: Some(payload.new_value), ..Default::default() }), ..Default::default() })
}
