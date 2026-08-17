//! 🔺 Diff constructor for `update-page-margins`.

use super::mutation::UpdatePageMargins;
use crate::artifacts::layout::schema::diff::{LayoutPagePatchEntry, LayoutPagesDelta};
use crate::artifacts::layout::{LayoutDiff, LayoutSnapshot, PagePatch};

//#region 📐UpdatePageMargins
pub fn diff_update_page_margins(payload: &UpdatePageMargins, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    let Some(page) = base.pages.iter().find(|page| page.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Page \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if page.margins.top == payload.top && page.margins.right == payload.right && page.margins.bottom == payload.bottom && page.margins.left == payload.left {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Page \"{}\" already has those margins.", payload.id));
    }
    protocol::MutationOutcome::new(LayoutDiff {
        pages: Some(LayoutPagesDelta {
            patched: vec![LayoutPagePatchEntry {
                id: payload.id.clone(),
                patch: PagePatch {
                    margin_top: Some(payload.top),
                    margin_right: Some(payload.right),
                    margin_bottom: Some(payload.bottom),
                    margin_left: Some(payload.left),
                    ..Default::default()
                },
            }],
            ..Default::default()
        }),
        ..Default::default()
    })
}
//#endregion 📐UpdatePageMargins
