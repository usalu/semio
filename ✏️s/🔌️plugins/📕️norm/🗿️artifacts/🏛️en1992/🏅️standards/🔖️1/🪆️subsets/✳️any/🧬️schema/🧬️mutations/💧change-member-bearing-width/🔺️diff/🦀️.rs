use semio_s_artifact_norm_en1992_diff::{En1992Diff, En1992MembersRows, En1992MembersPatch};
use crate::change_member_bearing_width::ChangeMemberBearingWidth;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn diff(payload: &ChangeMemberBearingWidth, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    if !payload.new_value.is_finite() || !(0.0..=1.0).contains(&payload.new_value) {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The bearing width must be finite and between zero and one metre.", Vec::<String>::new());
    }
    let Some(m) = base.members.iter().find(|m| m.id == payload.member_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Member {} not found.", payload.member_id), Vec::<String>::new());
    };
    if (m.bearing_width - payload.new_value).abs() < f64::EPSILON {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1992Diff {
        members: Some(En1992MembersRows::modification(&payload.member_id, En1992MembersPatch { bearing_width: Some(payload.new_value), ..Default::default() })),
        ..Default::default()
    })
}

#[cfg(test)]
#[path = "🧪️.rs"]
mod tests;
