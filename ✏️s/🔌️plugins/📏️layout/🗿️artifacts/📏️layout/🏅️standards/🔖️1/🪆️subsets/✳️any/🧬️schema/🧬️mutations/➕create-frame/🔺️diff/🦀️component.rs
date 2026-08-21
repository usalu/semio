//! 🔺 Diff constructor for `create-frame` — a `PagePatch.frame_added` fragment nested under the
//! target page's `LayoutPagesDelta` patch entry (never apply-then-capture).

use super::mutation::CreateFrame;
use crate::artifacts::layout::schema::diff::{LayoutPagePatchEntry, LayoutPagesDelta};
use crate::artifacts::layout::{LayoutDiff, LayoutSnapshot, PageFrameAdded, PagePatch};

//#region ➕️CreateFrame
pub async fn diff_create_frame(payload: &CreateFrame, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    let Some(page) = base.pages.iter().find(|page| page.id == payload.page_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Page \"{}\" does not exist.", payload.page_id), [payload.page_id.clone()]);
    };
    if page.frames.iter().any(|frame| frame.id() == payload.frame.id()) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A frame with id \"{}\" already exists on page \"{}\".", payload.frame.id(), payload.page_id), [payload.frame.id().to_string()]);
    }
    protocol::MutationOutcome::new(LayoutDiff {
        pages: Some(LayoutPagesDelta {
            patched: vec![LayoutPagePatchEntry { id: payload.page_id.clone(), patch: PagePatch { frame_added: Some(PageFrameAdded { frame: payload.frame.clone(), index: payload.index, layer_id: payload.layer_id.clone() }), ..Default::default() } }],
            ..Default::default()
        }),
        ..Default::default()
    })
}
//#endregion ➕️CreateFrame
