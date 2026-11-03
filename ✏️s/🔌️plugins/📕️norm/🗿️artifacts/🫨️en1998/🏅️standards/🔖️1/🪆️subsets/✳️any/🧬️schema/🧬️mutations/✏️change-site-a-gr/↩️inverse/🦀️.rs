//! ↩️ Inverse for `change-site-a-gr`.
use crate::change_site_a_gr::ChangeSiteAGr;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeSiteAGr, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_site_a_gr::ChangeSiteAGr> {
    Ok(vec![Op::from(ChangeSiteAGr { new_a_gr: base.site.a_gr })])
}
