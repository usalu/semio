//! Diff for `update-site`.
use super::UpdateSite;
use crate::{En1998Diff, En1998Snapshot};

pub fn diff(payload: &UpdateSite, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if base.site == payload.site {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The site already has these values.");
    }
    protocol::MutationOutcome::new(En1998Diff { site: Some(payload.site.clone()), ..Default::default() })
}
