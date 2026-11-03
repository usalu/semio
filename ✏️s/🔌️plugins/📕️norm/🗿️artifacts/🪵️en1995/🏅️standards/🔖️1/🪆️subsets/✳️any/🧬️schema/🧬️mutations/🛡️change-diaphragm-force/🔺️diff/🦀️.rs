use crate::change_diaphragm_force::ChangeDiaphragmForce;
use semio_s_artifact_norm_en1995_diff::En1995Diff;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
use semio_s_artifact_norm_en1995_diff::{En1995DiaphragmDelta, En1995DiaphragmPatch};
pub fn diff(payload: &ChangeDiaphragmForce, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
    let Some(row) = base.diaphragms.iter().find(|item| item.id == payload.diaphragm_id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Unknown diaphragm id.", vec![payload.diaphragm_id.clone()]);
    };
    if !payload.new_value.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1995Diff { diaphragms: En1995DiaphragmDelta::modification(&row.id, En1995DiaphragmPatch { diaphragm_force_n: Some(payload.new_value), ..Default::default() }), ..Default::default() })
}
