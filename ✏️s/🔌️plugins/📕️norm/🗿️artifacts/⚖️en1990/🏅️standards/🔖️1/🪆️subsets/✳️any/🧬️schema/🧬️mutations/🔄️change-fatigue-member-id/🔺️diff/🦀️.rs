//! 🔄 `change-fatigue-member-id` diff: patches `member_id` of the addressed fatigue spectrum row.

use crate::change_fatigue_member_id::ChangeFatigueMemberId;
use semio_s_artifact_norm_en1990_diff::{En1990Diff, En1990FatigueSpectrumDelta, En1990FatigueSpectrumPatch};
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn diff(payload: &ChangeFatigueMemberId, base: &En1990Snapshot) -> protocol::MutationOutcome<En1990Diff> {
    let Some(r0) = base.fatigue_spectra.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Fatigue spectrum {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.member_id == payload.new_member_id {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The fatigue spectrum already has this value.");
    }
    protocol::MutationOutcome::new(En1990Diff { fatigue_spectra: En1990FatigueSpectrumDelta::modification(&payload.id, En1990FatigueSpectrumPatch { member_id: Some(payload.new_member_id.clone()), ..Default::default() }), ..Default::default() })
}
