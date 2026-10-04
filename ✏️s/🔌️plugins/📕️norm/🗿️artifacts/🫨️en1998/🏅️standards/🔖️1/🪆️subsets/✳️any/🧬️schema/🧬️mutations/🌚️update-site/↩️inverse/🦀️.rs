//! Inverse for `update-site`.
use super::UpdateSite;
use crate::{En1998Mutation, En1998Snapshot};

pub fn inverse(_payload: &UpdateSite, base: &En1998Snapshot) -> Result<Vec<En1998Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![En1998Mutation::UpdateSite(UpdateSite { site: base.site.clone() })]

    })())
}
