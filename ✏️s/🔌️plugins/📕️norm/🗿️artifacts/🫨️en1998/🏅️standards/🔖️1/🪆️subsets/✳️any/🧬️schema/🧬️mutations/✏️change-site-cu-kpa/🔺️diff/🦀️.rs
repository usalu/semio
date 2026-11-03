//! ✏️ `change-site-cu-kpa` diff — patches the one field of the addressed row; a missing target is a `mutation.target-missing`.

use crate::change_site_c_u_kpa::ChangeSiteCUKpa;
use semio_s_artifact_norm_en1998_diff::En1998Diff;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &ChangeSiteCUKpa, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    let mut site = base.site.clone();
    site.c_u_kpa = payload.new_c_u_kpa;
    if site == base.site {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The site already has this value.");
    }
    protocol::MutationOutcome::new(En1998Diff { site: Some(site), ..Default::default() })
}
