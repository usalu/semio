use crate::change_fastener_spacing_perp::ChangeFastenerSpacingPerp;
use semio_s_artifact_norm_en1995_diff::En1995Diff;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
use semio_s_artifact_norm_en1995_diff::{En1995FastenerDelta, En1995FastenerPatch};
pub fn diff(payload: &ChangeFastenerSpacingPerp, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
    let Some(row) = base.fasteners.iter().find(|item| item.id == payload.fastener_id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Unknown fastener id.", vec![payload.fastener_id.clone()]);
    };
    if !payload.new_value.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1995Diff { fasteners: En1995FastenerDelta::modification(&row.id, En1995FastenerPatch { spacing_perpendicular_m: Some(payload.new_value), ..Default::default() }), ..Default::default() })
}
