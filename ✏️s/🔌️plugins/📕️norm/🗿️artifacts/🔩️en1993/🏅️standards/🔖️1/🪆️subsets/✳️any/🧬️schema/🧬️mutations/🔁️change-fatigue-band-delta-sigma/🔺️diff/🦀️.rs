//! 🔺️ `change-fatigue-band-delta-sigma` diff: patches `spectrum[].delta_sigma` of the addressed row's list.

use crate::change_fatigue_band_delta_sigma::ChangeFatigueBandDeltaSigma;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993FatigueDetailDelta, En1993FatigueDetailPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeFatigueBandDeltaSigma, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if !payload.new_delta_sigma.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(row) = base.fatigue_details.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "fatigue-band", Vec::<String>::new());
    };
    let Some(at) = row.spectrum.iter().position(|element| element.id == payload.element_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "fatigue-band", Vec::<String>::new());
    };
    if row.spectrum[at].delta_sigma == payload.new_delta_sigma {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The element already has this value.");
    }
    let mut value = row.spectrum.clone();
    value[at].delta_sigma = payload.new_delta_sigma;
    protocol::MutationOutcome::new(En1993Diff { fatigue_details: En1993FatigueDetailDelta::modification(payload.id.clone(), En1993FatigueDetailPatch { spectrum: Some(value), ..Default::default() }), ..Default::default() })
}
