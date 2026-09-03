//! 🔢 `change-frame-columns` — sets a `Frame::Text`'s `columns` count. A no-op on non-text frames.


use crate::artifacts::layout::{Frame, FramePatch, LayoutDiff, LayoutSnapshot, PageFramePatched, PagePatch};
use crate::artifacts::layout::mutations::LayoutMutation;
use crate::artifacts::layout::schema::diff::{LayoutPagePatchEntry, LayoutPagesDelta};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔢ChangeFrameColumns
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, ToValue, FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeFrameColumns {
    pub page_id: String,
    pub frame_id: String,
    pub new_columns: u32,
}

impl MutationKind<LayoutSnapshot, LayoutMutation> for ChangeFrameColumns {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "frame-columns", kind: "change-frame-columns", record: "ChangedFrameColumns" };
    async fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
        diff_change_frame_columns(self, base)
    }
    async fn inverse(&self, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
        inverse_change_frame_columns(self, base)
    }
    async fn label(&self) -> String {
        format!("Change frame \"{}\" columns", self.frame_id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.page_id.clone(), self.frame_id.clone()]
    }
}
//#endregion 🔢ChangeFrameColumns


//#region 🔢ChangeFrameColumns
pub async fn diff_change_frame_columns(payload: &ChangeFrameColumns, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    let Some(page) = base.pages.iter().find(|page| page.id == payload.page_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Page \"{}\" does not exist.", payload.page_id), [payload.page_id.clone()]);
    };
    let Some(frame) = page.frames.iter().find(|frame| frame.id() == payload.frame_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Frame \"{}\" does not exist on page \"{}\".", payload.frame_id, payload.page_id), [payload.frame_id.clone()]);
    };
    if let Frame::Text { columns, .. } = frame {
        if *columns == payload.new_columns {
            return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Frame \"{}\" already has {} column(s).", payload.frame_id, payload.new_columns));
        }
    }
    protocol::MutationOutcome::new(LayoutDiff {
        pages: Some(LayoutPagesDelta {
            patched: vec![LayoutPagePatchEntry {
                id: payload.page_id.clone(),
                patch: PagePatch { frame_patched: Some(PageFramePatched { frame_id: payload.frame_id.clone(), patch: FramePatch { columns: Some(payload.new_columns), ..Default::default() } }), ..Default::default() },
            }],
            ..Default::default()
        }),
        ..Default::default()
    })
}
//#endregion 🔢ChangeFrameColumns


//#region 🔢ChangeFrameColumns
pub async fn inverse_change_frame_columns(payload: &ChangeFrameColumns, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
    let Some(page) = base.pages.iter().find(|page| page.id == payload.page_id) else {
        return Vec::new();
    };
    let Some(frame) = page.frames.iter().find(|frame| frame.id() == payload.frame_id) else {
        return Vec::new();
    };
    let Frame::Text { columns, .. } = frame else {
        return Vec::new();
    };
    vec![LayoutMutation::ChangeFrameColumns(ChangeFrameColumns { page_id: payload.page_id.clone(), frame_id: payload.frame_id.clone(), new_columns: *columns })]
}
//#endregion 🔢ChangeFrameColumns
