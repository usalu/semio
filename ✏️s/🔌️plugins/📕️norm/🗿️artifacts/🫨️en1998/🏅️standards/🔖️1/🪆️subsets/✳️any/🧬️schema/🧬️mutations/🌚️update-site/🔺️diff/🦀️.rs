//! Diff for `update-site`.
use super::UpdateSite;
use crate::{En1998Diff, En1998Snapshot};

pub fn diff(payload: &UpdateSite, _base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    protocol::MutationOutcome::new(En1998Diff { site: Some(payload.site.clone()), ..Default::default() })
}
