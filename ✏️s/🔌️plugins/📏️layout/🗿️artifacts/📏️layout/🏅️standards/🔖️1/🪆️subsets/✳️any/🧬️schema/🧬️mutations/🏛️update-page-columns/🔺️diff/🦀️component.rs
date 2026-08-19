//! 🔺 Diff constructor for `update-page-columns`.

use super::mutation::UpdatePageColumns;
use crate::artifacts::layout::schema::diff::{LayoutPagePatchEntry, LayoutPagesDelta};
use crate::artifacts::layout::{LayoutDiff, LayoutSnapshot, PagePatch};

//#region 🏛️UpdatePageColumns
pub async fn diff_update_page_columns(payload: &UpdatePageColumns, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    let Some(page) = base.pages.iter().find(|page| page.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Page \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if page.columns.count == payload.count && page.columns.gutter == payload.gutter {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Page \"{}\" already has those columns.", payload.id));
    }
    protocol::MutationOutcome::new(LayoutDiff {
        pages: Some(LayoutPagesDelta {
            patched: vec![LayoutPagePatchEntry {
                id: payload.id.clone(),
                patch: PagePatch { columns_count: Some(payload.count), columns_gutter: Some(payload.gutter), ..Default::default() },
            }],
            ..Default::default()
        }),
        ..Default::default()
    })
}
//#endregion 🏛️UpdatePageColumns
