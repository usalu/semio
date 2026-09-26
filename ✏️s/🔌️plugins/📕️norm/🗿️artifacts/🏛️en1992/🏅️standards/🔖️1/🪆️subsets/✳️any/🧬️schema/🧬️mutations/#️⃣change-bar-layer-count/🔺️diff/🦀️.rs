use crate::diff::{En1992Diff, En1992MemberList};
use super::ChangeBarLayerCount;
use crate::En1992Snapshot;

pub fn diff(payload: &ChangeBarLayerCount, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    let mut members = base.members.clone();
    let Some(m) = members.iter_mut().find(|m| m.id == payload.member_id) else {
        return protocol::MutationOutcome::fatal("mutation.missing", format!("Member {} not found.", payload.member_id), Vec::<String>::new());
    };
    let Some(layer) = m.longitudinal.iter_mut().find(|l| l.id == payload.layer_id) else {
        return protocol::MutationOutcome::fatal("mutation.missing", format!("Layer {} not found.", payload.layer_id), Vec::<String>::new());
    };
    if layer.count == payload.new_count {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Value unchanged.");
    }
    layer.count = payload.new_count;
    protocol::MutationOutcome::new(En1992Diff { members: Some(En1992MemberList { values: members }), ..Default::default() })
}
