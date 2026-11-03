//! ↩️ Inverse for `change-site-cu-kpa`.
use crate::change_site_c_u_kpa::ChangeSiteCUKpa;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeSiteCUKpa, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_site_c_u_kpa::ChangeSiteCUKpa> {
    Ok(vec![Op::from(ChangeSiteCUKpa { new_c_u_kpa: base.site.c_u_kpa })])
}
