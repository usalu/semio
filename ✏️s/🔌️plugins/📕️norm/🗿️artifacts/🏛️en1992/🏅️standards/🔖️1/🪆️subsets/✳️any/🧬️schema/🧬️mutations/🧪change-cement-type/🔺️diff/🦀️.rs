use semio_s_artifact_norm_en1992_diff::En1992Diff;
use crate::change_cement_type::ChangeCementType;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn diff(payload: &ChangeCementType, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    if base.cement_type == payload.new_cement_type {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1992Diff { cement_type: Some(payload.new_cement_type.clone()), ..Default::default() })
}
