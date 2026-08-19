//! 🔺 Diff constructor for `change-page-width`.

use super::mutation::ChangePageWidth;
use crate::artifacts::layout::schema::diff::{LayoutPagePatchEntry, LayoutPagesDelta};
use crate::artifacts::layout::{LayoutDiff, LayoutSnapshot, PagePatch};

//#region ↔️ChangePageWidth
pub async fn diff_change_page_width(payload: &ChangePageWidth, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    let Some(page) = base.pages.iter().find(|page| page.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Page \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if page.width == payload.new_width {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Page \"{}\" already has width {}.", payload.id, payload.new_width));
    }
    protocol::MutationOutcome::new(LayoutDiff {
        pages: Some(LayoutPagesDelta {
            patched: vec![LayoutPagePatchEntry { id: payload.id.clone(), patch: PagePatch { width: Some(payload.new_width), ..Default::default() } }],
            ..Default::default()
        }),
        ..Default::default()
    })
}
//#endregion ↔️ChangePageWidth
