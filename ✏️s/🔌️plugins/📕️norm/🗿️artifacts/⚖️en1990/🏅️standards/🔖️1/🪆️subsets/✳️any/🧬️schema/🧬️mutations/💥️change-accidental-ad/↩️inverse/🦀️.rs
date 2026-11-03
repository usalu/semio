//! ↩️ `change-accidental-ad` inverse: restores the previous `ad`.

use crate::change_accidental_ad::ChangeAccidentalAd;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(payload: &ChangeAccidentalAd, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_accidental_ad::ChangeAccidentalAd> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeAccidentalAd, base: &En1990Snapshot) -> Option<ChangeAccidentalAd> {
    let r0 = base.accidentals.iter().find(|row| row.id == payload.id)?;
    Some(ChangeAccidentalAd { id: payload.id.clone(), new_ad: r0.ad })
}
