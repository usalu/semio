//! 🔺️ Diff for `change-structure-kind`.
use crate::change_structure_kind::ChangeStructureKind;
use semio_s_artifact_norm_en1991_diff::En1991Diff;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn diff(payload: &ChangeStructureKind, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.structure_kind == payload.new_structure_kind {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { structure_kind: Some(payload.new_structure_kind), ..Default::default() })
}
