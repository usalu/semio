//! 🔺 Diff constructor for `change-page-height`.

use super::mutation::ChangePageHeight;
use crate::artifacts::layout::schema::diff::{LayoutPagePatchEntry, LayoutPagesDelta};
use crate::artifacts::layout::{LayoutDiff, LayoutSnapshot, PagePatch};

//#region ↕️ChangePageHeight
pub async fn diff_change_page_height(payload: &ChangePageHeight, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    let Some(page) = base.pages.iter().find(|page| page.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Page \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if page.height == payload.new_height {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Page \"{}\" already has height {}.", payload.id, payload.new_height));
    }
    protocol::MutationOutcome::new(LayoutDiff {
        pages: Some(LayoutPagesDelta {
            patched: vec![LayoutPagePatchEntry { id: payload.id.clone(), patch: PagePatch { height: Some(payload.new_height), ..Default::default() } }],
            ..Default::default()
        }),
        ..Default::default()
    })
}
//#endregion ↕️ChangePageHeight
