use crate::change_truss_web_depth::ChangeTrussWebDepth;
use semio_s_artifact_norm_en1995_diff::En1995Diff;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
use semio_s_artifact_norm_en1995_diff::{En1995TrussDelta, En1995TrussPatch};
pub fn diff(payload: &ChangeTrussWebDepth, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
    let Some(row) = base.trusses.iter().find(|item| item.id == payload.truss_id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Unknown truss id.", vec![payload.truss_id.clone()]);
    };
    if !payload.new_value.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1995Diff { trusses: En1995TrussDelta::modification(&row.id, En1995TrussPatch { web_depth_m: Some(payload.new_value), ..Default::default() }), ..Default::default() })
}
