//! 🔺 Diff constructor for `rename-page`.

use super::mutation::RenamePage;
use crate::artifacts::layout::schema::diff::{LayoutPagePatchEntry, LayoutPagesDelta};
use crate::artifacts::layout::{LayoutDiff, LayoutSnapshot, PagePatch};

//#region 🏷️RenamePage
pub async fn diff_rename_page(payload: &RenamePage, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    let Some(page) = base.pages.iter().find(|page| page.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Page \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if page.name == payload.new_name {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Page \"{}\" already has that name.", payload.id));
    }
    protocol::MutationOutcome::new(LayoutDiff {
        pages: Some(LayoutPagesDelta {
            patched: vec![LayoutPagePatchEntry { id: payload.id.clone(), patch: PagePatch { name: Some(payload.new_name.clone()), ..Default::default() } }],
            ..Default::default()
        }),
        ..Default::default()
    })
}
//#endregion 🏷️RenamePage
