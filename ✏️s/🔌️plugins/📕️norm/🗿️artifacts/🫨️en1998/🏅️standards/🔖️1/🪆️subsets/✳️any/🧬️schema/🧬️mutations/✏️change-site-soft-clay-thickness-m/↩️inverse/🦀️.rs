//! ↩️ Inverse for `change-site-soft-clay-thickness-m`.
use crate::change_site_soft_clay_thickness_m::ChangeSiteSoftClayThicknessM;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeSiteSoftClayThicknessM, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_site_soft_clay_thickness_m::ChangeSiteSoftClayThicknessM> {
    Ok(vec![Op::from(ChangeSiteSoftClayThicknessM { new_soft_clay_thickness_m: base.site.soft_clay_thickness_m })])
}
