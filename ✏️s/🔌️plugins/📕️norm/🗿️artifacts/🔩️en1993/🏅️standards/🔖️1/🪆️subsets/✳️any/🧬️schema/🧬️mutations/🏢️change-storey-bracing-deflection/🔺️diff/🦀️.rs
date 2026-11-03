//! 🔺️ `change-storey-bracing-deflection` diff: patches `bracing_deflection` of the addressed storey row.

use crate::change_storey_bracing_deflection::ChangeStoreyBracingDeflection;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993StoreyDelta, En1993StoreyPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeStoreyBracingDeflection, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if !payload.new_bracing_deflection.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(row) = base.storeys.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "storey", Vec::<String>::new());
    };
    if row.bracing_deflection == payload.new_bracing_deflection {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The storey already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { storeys: En1993StoreyDelta::modification(payload.id.clone(), En1993StoreyPatch { bracing_deflection: Some(payload.new_bracing_deflection), ..Default::default() }), ..Default::default() })
}
