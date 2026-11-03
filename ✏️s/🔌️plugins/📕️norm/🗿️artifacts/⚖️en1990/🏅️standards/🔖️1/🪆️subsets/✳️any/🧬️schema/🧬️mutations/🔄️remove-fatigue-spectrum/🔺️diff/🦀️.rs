//! 🔄 `remove-fatigue-spectrum` diff: removes the addressed fatigue spectrum row.

use crate::remove_fatigue_spectrum::RemoveFatigueSpectrum;
use semio_s_artifact_norm_en1990_diff::{En1990Diff, En1990FatigueSpectrumDelta};
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn diff(payload: &RemoveFatigueSpectrum, base: &En1990Snapshot) -> protocol::MutationOutcome<En1990Diff> {
    let index = payload.index;
    if index >= base.fatigue_spectra.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "fatigue_spectra index out of range", [index.to_string()]);
    }
    protocol::MutationOutcome::new(En1990Diff { fatigue_spectra: En1990FatigueSpectrumDelta::removal(&base.fatigue_spectra, index), ..Default::default() })
}
