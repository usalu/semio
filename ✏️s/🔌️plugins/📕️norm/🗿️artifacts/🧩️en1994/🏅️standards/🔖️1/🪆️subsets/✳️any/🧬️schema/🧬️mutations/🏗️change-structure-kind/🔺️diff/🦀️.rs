//! 🔺️ Diff for `change-structure-kind`.
use crate::change_structure_kind::ChangeStructureKind;
use semio_s_artifact_norm_en1994_diff::En1994Diff;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
pub fn diff(payload: &ChangeStructureKind, base: &En1994Snapshot) -> protocol::MutationOutcome<En1994Diff> {
    if base.structure_kind == payload.new_structure_kind {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "unchanged");
    }
    protocol::MutationOutcome::new(En1994Diff { structure_kind: Some(payload.new_structure_kind), ..Default::default() })
}
