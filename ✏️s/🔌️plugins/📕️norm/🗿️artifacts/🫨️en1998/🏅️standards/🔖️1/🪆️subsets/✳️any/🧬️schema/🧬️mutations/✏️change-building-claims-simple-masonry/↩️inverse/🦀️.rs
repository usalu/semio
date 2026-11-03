//! ↩️ `change-building-claims-simple-masonry` inverse: restores the previous `claims_simple_masonry`.

use crate::change_building_claims_simple_masonry::ChangeBuildingClaimsSimpleMasonry;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeBuildingClaimsSimpleMasonry, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_building_claims_simple_masonry::ChangeBuildingClaimsSimpleMasonry> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeBuildingClaimsSimpleMasonry, base: &En1998Snapshot) -> Option<ChangeBuildingClaimsSimpleMasonry> {
    let r0 = base.buildings.iter().find(|row| row.id == payload.id)?;
    Some(ChangeBuildingClaimsSimpleMasonry { id: payload.id.clone(), new_claims_simple_masonry: r0.claims_simple_masonry })
}
