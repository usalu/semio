//! ↩️ Inverse for `change-site-n-spt`.
use crate::change_site_n_spt::ChangeSiteNSpt;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeSiteNSpt, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_site_n_spt::ChangeSiteNSpt> {
    Ok(vec![Op::from(ChangeSiteNSpt { new_n_spt: base.site.n_spt })])
}
