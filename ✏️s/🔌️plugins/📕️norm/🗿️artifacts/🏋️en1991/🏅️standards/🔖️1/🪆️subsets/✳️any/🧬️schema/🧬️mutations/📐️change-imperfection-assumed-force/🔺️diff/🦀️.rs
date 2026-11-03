//! 📐 `change-imperfection-assumed-force` diff: patches `assumed_force` of the addressed execution imperfection row.

use crate::change_imperfection_assumed_force::ChangeImperfectionAssumedForce;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991ExecutionImperfectionDelta, En1991ExecutionImperfectionPatch};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &ChangeImperfectionAssumedForce, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if !payload.new_assumed_force.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.execution_imperfections.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Execution imperfection {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.assumed_force == payload.new_assumed_force {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The execution imperfection already has this value.");
    }
    protocol::MutationOutcome::new(En1991Diff { execution_imperfections: En1991ExecutionImperfectionDelta::modification(&payload.id, En1991ExecutionImperfectionPatch { assumed_force: Some(payload.new_assumed_force), ..Default::default() }), ..Default::default() })
}
