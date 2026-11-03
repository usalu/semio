//! ✏️ `change-site-alluvium-vsms` diff — patches the one field of the addressed row; a missing target is a `mutation.target-missing`.

use crate::change_site_alluvium_v_s_m_s::ChangeSiteAlluviumVSMS;
use semio_s_artifact_norm_en1998_diff::En1998Diff;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &ChangeSiteAlluviumVSMS, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    let mut site = base.site.clone();
    site.alluvium_v_s_m_s = payload.new_alluvium_v_s_m_s;
    if site == base.site {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The site already has this value.");
    }
    protocol::MutationOutcome::new(En1998Diff { site: Some(site), ..Default::default() })
}
