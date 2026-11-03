//! 🚆 `remove-derailment` diff: removes the addressed derailment case row.

use crate::remove_derailment::RemoveDerailment;
use semio_s_artifact_norm_en1991_diff::{En1991DerailmentCaseDelta, En1991Diff};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &RemoveDerailment, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    let Some(index) = base.derailment_cases.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Derailment case {} does not exist.", payload.id), [payload.id.clone()]);
    };
    protocol::MutationOutcome::new(En1991Diff { derailment_cases: En1991DerailmentCaseDelta::removal(&base.derailment_cases, index), ..Default::default() })
}
