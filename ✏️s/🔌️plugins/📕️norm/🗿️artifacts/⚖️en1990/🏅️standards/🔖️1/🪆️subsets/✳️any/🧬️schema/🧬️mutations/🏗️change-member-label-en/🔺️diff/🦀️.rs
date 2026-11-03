//! 🏗️ `change-member-label-en` diff: patches `label_en` of the addressed member row.

use crate::change_member_label_en::ChangeMemberLabelEn;
use semio_s_artifact_norm_en1990_diff::{En1990Diff, En1990MemberDelta, En1990MemberPatch};
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn diff(payload: &ChangeMemberLabelEn, base: &En1990Snapshot) -> protocol::MutationOutcome<En1990Diff> {
    let Some(r0) = base.members.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Member {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.label_en == payload.new_label_en {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The member already has this value.");
    }
    protocol::MutationOutcome::new(En1990Diff { members: En1990MemberDelta::modification(&payload.id, En1990MemberPatch { label_en: Some(payload.new_label_en.clone()), ..Default::default() }), ..Default::default() })
}
