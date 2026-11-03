//! 📐 `remove-imperfection` diff: removes the addressed execution imperfection row.

use crate::remove_imperfection::RemoveImperfection;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991ExecutionImperfectionDelta};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &RemoveImperfection, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    let Some(index) = base.execution_imperfections.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Execution imperfection {} does not exist.", payload.id), [payload.id.clone()]);
    };
    protocol::MutationOutcome::new(En1991Diff { execution_imperfections: En1991ExecutionImperfectionDelta::removal(&base.execution_imperfections, index), ..Default::default() })
}
