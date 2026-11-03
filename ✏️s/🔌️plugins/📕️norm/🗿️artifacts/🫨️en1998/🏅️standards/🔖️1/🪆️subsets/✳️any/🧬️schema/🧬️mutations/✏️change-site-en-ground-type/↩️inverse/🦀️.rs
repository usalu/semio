//! ↩️ Inverse for `change-site-en-ground-type`.
use crate::change_site_en_ground_type::ChangeSiteEnGroundType;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeSiteEnGroundType, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_site_en_ground_type::ChangeSiteEnGroundType> {
    Ok(vec![Op::from(ChangeSiteEnGroundType { new_en_ground_type: base.site.en_ground_type })])
}
