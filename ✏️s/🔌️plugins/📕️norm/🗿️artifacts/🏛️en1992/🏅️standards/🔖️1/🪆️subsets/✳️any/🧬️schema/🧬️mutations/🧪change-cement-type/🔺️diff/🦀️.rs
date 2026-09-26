use crate::diff::En1992Diff;
use crate::mutations::change_cement_type::ChangeCementType;
use crate::En1992Snapshot;

pub fn diff(payload: &ChangeCementType, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    if base.cement_type == payload.new_cement_type {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1992Diff { cement_type: Some(payload.new_cement_type.clone()), ..Default::default() })
}
