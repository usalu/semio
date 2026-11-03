//! ↩️ `remove-fatigue-spectrum` inverse: inserts the removed row back at its position.

use crate::remove_fatigue_spectrum::RemoveFatigueSpectrum;
use crate::insert_fatigue_spectrum::InsertFatigueSpectrum;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(payload: &RemoveFatigueSpectrum, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_fatigue_spectrum::InsertFatigueSpectrum> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &RemoveFatigueSpectrum, base: &En1990Snapshot) -> Option<InsertFatigueSpectrum> {
    let index = payload.index;
    let row = base.fatigue_spectra.get(index)?;
    Some(InsertFatigueSpectrum { index: Some(index), item: row.clone() })
}
