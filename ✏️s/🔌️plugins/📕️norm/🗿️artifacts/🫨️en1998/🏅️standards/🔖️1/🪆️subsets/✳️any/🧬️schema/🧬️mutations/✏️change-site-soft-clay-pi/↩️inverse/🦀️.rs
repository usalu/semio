//! ↩️ Inverse for `change-site-soft-clay-pi`.
use crate::change_site_soft_clay_pi::ChangeSiteSoftClayPi;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeSiteSoftClayPi, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_site_soft_clay_pi::ChangeSiteSoftClayPi> {
    Ok(vec![Op::from(ChangeSiteSoftClayPi { new_soft_clay_pi: base.site.soft_clay_pi })])
}
