//! ↩️ Inverse for `update-site`.
use crate::update_site::UpdateSite;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(_payload: &UpdateSite, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::update_site::UpdateSite> {
    Ok((|| {
    vec![Op::from(UpdateSite { site: base.site.clone() })]

    })())
}
