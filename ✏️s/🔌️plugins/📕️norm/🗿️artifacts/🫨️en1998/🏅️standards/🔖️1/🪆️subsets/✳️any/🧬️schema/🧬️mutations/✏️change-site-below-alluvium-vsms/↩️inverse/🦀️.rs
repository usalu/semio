//! ↩️ Inverse for `change-site-below-alluvium-vsms`.
use crate::change_site_below_alluvium_v_s_m_s::ChangeSiteBelowAlluviumVSMS;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeSiteBelowAlluviumVSMS, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_site_below_alluvium_v_s_m_s::ChangeSiteBelowAlluviumVSMS> {
    Ok(vec![Op::from(ChangeSiteBelowAlluviumVSMS { new_below_alluvium_v_s_m_s: base.site.below_alluvium_v_s_m_s })])
}
