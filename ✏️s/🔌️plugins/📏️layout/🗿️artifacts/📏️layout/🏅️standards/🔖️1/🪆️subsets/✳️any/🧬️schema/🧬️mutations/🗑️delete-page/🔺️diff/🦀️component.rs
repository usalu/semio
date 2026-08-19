//! 🔺 Diff constructor for `delete-page`.

use super::mutation::DeletePage;
use crate::artifacts::layout::schema::diff::LayoutPagesDelta;
use crate::artifacts::layout::{LayoutDiff, LayoutSnapshot};

//#region 🗑️DeletePage
pub async fn diff_delete_page(payload: &DeletePage, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    if !base.pages.iter().any(|page| page.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Page \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    protocol::MutationOutcome::new(LayoutDiff { pages: Some(LayoutPagesDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🗑️DeletePage
