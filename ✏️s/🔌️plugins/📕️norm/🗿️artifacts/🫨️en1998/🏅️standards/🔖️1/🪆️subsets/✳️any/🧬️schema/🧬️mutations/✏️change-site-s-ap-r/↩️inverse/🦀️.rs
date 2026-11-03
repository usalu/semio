//! ↩️ Inverse for `change-site-s-ap-r`.
use crate::change_site_s_ap_r::ChangeSiteSApR;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeSiteSApR, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_site_s_ap_r::ChangeSiteSApR> {
    Ok(vec![Op::from(ChangeSiteSApR { new_s_ap_r: base.site.s_ap_r })])
}
