//! ↩️ `change-building-accidental-ecc-ratio` inverse: restores the previous `accidental_eccentricity_ratio`.

use crate::change_building_accidental_ecc_ratio::ChangeBuildingAccidentalEccRatio;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeBuildingAccidentalEccRatio, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_building_accidental_ecc_ratio::ChangeBuildingAccidentalEccRatio> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeBuildingAccidentalEccRatio, base: &En1998Snapshot) -> Option<ChangeBuildingAccidentalEccRatio> {
    let r0 = base.buildings.iter().find(|row| row.id == payload.id)?;
    Some(ChangeBuildingAccidentalEccRatio { id: payload.id.clone(), new_accidental_eccentricity_ratio: r0.accidental_eccentricity_ratio })
}
