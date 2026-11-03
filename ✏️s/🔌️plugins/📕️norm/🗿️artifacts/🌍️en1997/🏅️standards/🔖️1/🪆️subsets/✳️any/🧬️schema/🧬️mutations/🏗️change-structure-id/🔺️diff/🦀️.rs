use crate::change_structure_id::ChangeStructureId;
use semio_s_artifact_norm_en1997_diff::En1997Diff;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;
pub fn diff(payload: &ChangeStructureId, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if base.structure_id == payload.new_structure_id {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "structure id unchanged");
    }
    protocol::MutationOutcome::new(En1997Diff { structure_id: Some(payload.new_structure_id.clone()), ..Default::default() })
}
