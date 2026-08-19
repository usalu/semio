//! 🔺 Diff constructor for `change-link-path`.

use super::mutation::ChangeLinkPath;
use crate::artifacts::layout::schema::diff::{LayoutLinkPatchEntry, LayoutLinksDelta};
use crate::artifacts::layout::{ImageLinkPatch, LayoutDiff, LayoutSnapshot};

//#region 🔗ChangeLinkPath
pub async fn diff_change_link_path(payload: &ChangeLinkPath, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    let Some(link) = base.links.iter().find(|link| link.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Link \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if link.path == payload.new_path {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Link \"{}\" already has path \"{}\".", payload.id, payload.new_path));
    }
    protocol::MutationOutcome::new(LayoutDiff {
        links: Some(LayoutLinksDelta {
            patched: vec![LayoutLinkPatchEntry { id: payload.id.clone(), patch: ImageLinkPatch { path: Some(payload.new_path.clone()) } }],
            ..Default::default()
        }),
        ..Default::default()
    })
}
//#endregion 🔗ChangeLinkPath
