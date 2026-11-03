//! ↩️ `change-fatigue-detail-category` inverse: restores the previous `detail_category`.

use crate::change_fatigue_detail_category::ChangeFatigueDetailCategory;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(payload: &ChangeFatigueDetailCategory, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_fatigue_detail_category::ChangeFatigueDetailCategory> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeFatigueDetailCategory, base: &En1990Snapshot) -> Option<ChangeFatigueDetailCategory> {
    let r0 = base.fatigue_spectra.iter().find(|row| row.id == payload.id)?;
    Some(ChangeFatigueDetailCategory { id: payload.id.clone(), new_detail_category: r0.detail_category })
}
