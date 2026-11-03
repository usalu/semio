//! ✏️ `change-foundation-base-pore-pressure-pa` diff — patches the one field of the addressed row; a missing target is a `mutation.target-missing`.

use crate::change_foundation_base_pore_pressure_pa::ChangeFoundationBasePorePressurePa;
use semio_s_artifact_norm_en1998_diff::{En1998Diff, En1998FoundationDelta, En1998FoundationPatch};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &ChangeFoundationBasePorePressurePa, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    let Some(row) = base.foundations.get(payload.index) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "foundation", Vec::<String>::new());
    };
    let patch = En1998FoundationPatch { base_pore_pressure_pa: Some(payload.new_base_pore_pressure_pa), ..Default::default() };
    protocol::MutationOutcome::new(En1998Diff { foundations: En1998FoundationDelta::modification(&row.id, patch), ..Default::default() })
}
