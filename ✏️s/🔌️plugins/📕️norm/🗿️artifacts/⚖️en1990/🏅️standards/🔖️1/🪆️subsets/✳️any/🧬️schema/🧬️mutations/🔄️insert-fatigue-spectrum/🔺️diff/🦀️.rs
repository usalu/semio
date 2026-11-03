//! 🔄 `insert-fatigue-spectrum` diff: inserts the fatigue spectrum row; an id the list holds is a `mutation.duplicate-id`.

use crate::insert_fatigue_spectrum::InsertFatigueSpectrum;
use semio_s_artifact_norm_en1990_diff::{En1990Diff, En1990FatigueSpectrumDelta};
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn diff(payload: &InsertFatigueSpectrum, base: &En1990Snapshot) -> protocol::MutationOutcome<En1990Diff> {
    if base.fatigue_spectra.iter().any(|existing| existing.id == payload.item.id) {
        let key = payload.item.id.clone();
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Fatigue spectrum {key} already exists."), [key]);
    }
    let index = payload.index.unwrap_or(usize::MAX).min(base.fatigue_spectra.len());
    let outcome = protocol::MutationOutcome::new(En1990Diff { fatigue_spectra: En1990FatigueSpectrumDelta::insertion(index, payload.item.clone()), ..Default::default() });
    if payload.index.is_none_or(|requested| requested == index) {
        return outcome;
    }
    outcome.warning("mutation.clamped", format!("Position {} is past the end of the fatigue spectrum list; inserted at {index}.", payload.index.unwrap_or(index)))
}
