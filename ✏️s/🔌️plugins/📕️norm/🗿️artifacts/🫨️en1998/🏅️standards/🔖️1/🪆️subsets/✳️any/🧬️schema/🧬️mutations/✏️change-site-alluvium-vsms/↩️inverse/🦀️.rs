//! ↩️ Inverse for `change-site-alluvium-vsms`.
use crate::change_site_alluvium_v_s_m_s::ChangeSiteAlluviumVSMS;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeSiteAlluviumVSMS, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_site_alluvium_v_s_m_s::ChangeSiteAlluviumVSMS> {
    Ok(vec![Op::from(ChangeSiteAlluviumVSMS { new_alluvium_v_s_m_s: base.site.alluvium_v_s_m_s })])
}
