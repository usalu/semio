//! ↩️ Inverse for `change-site-de-ground-combo`.
use crate::change_site_de_ground_combo::ChangeSiteDeGroundCombo;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeSiteDeGroundCombo, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_site_de_ground_combo::ChangeSiteDeGroundCombo> {
    Ok(vec![Op::from(ChangeSiteDeGroundCombo { new_de_ground_combo: base.site.de_ground_combo.clone() })])
}
