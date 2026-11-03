//! ↩️ Inverse for `change-site-alluvium-thickness-m`.
use crate::change_site_alluvium_thickness_m::ChangeSiteAlluviumThicknessM;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeSiteAlluviumThicknessM, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_site_alluvium_thickness_m::ChangeSiteAlluviumThicknessM> {
    Ok(vec![Op::from(ChangeSiteAlluviumThicknessM { new_alluvium_thickness_m: base.site.alluvium_thickness_m })])
}
