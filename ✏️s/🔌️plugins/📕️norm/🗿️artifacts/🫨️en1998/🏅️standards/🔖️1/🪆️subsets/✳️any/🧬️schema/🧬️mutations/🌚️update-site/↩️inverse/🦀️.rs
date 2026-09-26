//! Inverse for `update-site`.
use super::UpdateSite;
use crate::{En1998Mutation, En1998Snapshot};

pub fn inverse(_payload: &UpdateSite, base: &En1998Snapshot) -> Vec<En1998Mutation> {
    vec![En1998Mutation::UpdateSite(UpdateSite { site: base.site.clone() })]
}
