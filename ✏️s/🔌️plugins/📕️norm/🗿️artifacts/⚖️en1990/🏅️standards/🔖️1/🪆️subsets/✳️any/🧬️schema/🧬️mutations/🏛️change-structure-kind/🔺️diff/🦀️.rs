//! 🏛️ `change-structure-kind` diff: sets `structure_kind`.

use crate::change_structure_kind::ChangeStructureKind;
use semio_s_artifact_norm_en1990_diff::En1990Diff;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn diff(payload: &ChangeStructureKind, base: &En1990Snapshot) -> protocol::MutationOutcome<En1990Diff> {
    if base.structure_kind == payload.new_structure_kind {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The value is unchanged.");
    }
    protocol::MutationOutcome::new(En1990Diff { structure_kind: Some(payload.new_structure_kind), ..Default::default() })
}
