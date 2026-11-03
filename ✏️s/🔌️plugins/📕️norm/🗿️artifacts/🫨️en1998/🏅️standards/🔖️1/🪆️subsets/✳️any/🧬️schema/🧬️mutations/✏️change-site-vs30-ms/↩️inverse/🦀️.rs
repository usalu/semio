//! ↩️ Inverse for `change-site-vs30-ms`.
use crate::change_site_v_s30_m_s::ChangeSiteVS30MS;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeSiteVS30MS, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_site_v_s30_m_s::ChangeSiteVS30MS> {
    Ok(vec![Op::from(ChangeSiteVS30MS { new_v_s30_m_s: base.site.v_s30_m_s })])
}
