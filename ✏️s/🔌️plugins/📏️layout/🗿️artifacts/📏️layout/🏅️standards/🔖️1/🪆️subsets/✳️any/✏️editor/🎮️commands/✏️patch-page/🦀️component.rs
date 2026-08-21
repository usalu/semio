//! ✏️ ✏️ Layout play app commands command — `patch-page`.

use crate::artifacts::layout::mutations::change_frame_columns::mutation::ChangeFrameColumns;
use crate::artifacts::layout::mutations::change_frame_fill::mutation::ChangeFrameFill;
use crate::artifacts::layout::mutations::change_frame_stroke::mutation::ChangeFrameStroke;
use crate::artifacts::layout::mutations::change_frame_wrap_mode::mutation::ChangeFrameWrapMode;
use crate::artifacts::layout::mutations::change_link_path::mutation::ChangeLinkPath;
use crate::artifacts::layout::mutations::change_page_height::mutation::ChangePageHeight;
use crate::artifacts::layout::mutations::change_page_width::mutation::ChangePageWidth;
use crate::artifacts::layout::mutations::create_frame::mutation::CreateFrame;
use crate::artifacts::layout::mutations::create_page::mutation::CreatePage;
use crate::artifacts::layout::mutations::edit_story::mutation::EditStory;
use crate::artifacts::layout::mutations::move_frame::mutation::MoveFrame;
use crate::artifacts::layout::mutations::rename_page::mutation::RenamePage;
use crate::artifacts::layout::mutations::resize_frame::mutation::ResizeFrame;
use crate::artifacts::layout::mutations::update_page_columns::mutation::UpdatePageColumns;
use crate::artifacts::layout::mutations::update_page_margins::mutation::UpdatePageMargins;
use crate::artifacts::layout::mutations::LayoutMutation;
use crate::artifacts::layout::schema::text_to_rgba;
use crate::artifacts::layout::{Frame, LayoutSnapshot, Page, PageColumns, PageMargins};
use crate::editor::layout::config::{LayoutConfig, LayoutConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️Shared
/// 🎯️ Builds the exact semantic mutation for a `patchPage` field write from the command's text
/// `value`; unknown fields/mistyped (non-numeric where a number is expected) values yield `None`.
/// `marginTop`/`marginRight`/`marginBottom`/`marginLeft`/`columnsCount`/`columnsGutter` read the
/// page's OTHER current value(s) so `update-page-margins`/`update-page-columns` stay atomic.
async fn page_field_mutation(page: &Page, field: &str, value: &str) -> Option<LayoutMutation> {
    let id = page.id.clone();
    match field {
        "name" => Some(LayoutMutation::RenamePage(RenamePage { id, new_name: value.into() })),
        "width" => value.parse::<f64>().ok().map(|new_width| LayoutMutation::ChangePageWidth(ChangePageWidth { id, new_width })),
        "height" => value.parse::<f64>().ok().map(|new_height| LayoutMutation::ChangePageHeight(ChangePageHeight { id, new_height })),
        "marginTop" => value.parse::<f64>().ok().map(|top| LayoutMutation::UpdatePageMargins(UpdatePageMargins { id, top, right: page.margins.right, bottom: page.margins.bottom, left: page.margins.left })),
        "marginRight" => value.parse::<f64>().ok().map(|right| LayoutMutation::UpdatePageMargins(UpdatePageMargins { id, top: page.margins.top, right, bottom: page.margins.bottom, left: page.margins.left })),
        "marginBottom" => value.parse::<f64>().ok().map(|bottom| LayoutMutation::UpdatePageMargins(UpdatePageMargins { id, top: page.margins.top, right: page.margins.right, bottom, left: page.margins.left })),
        "marginLeft" => value.parse::<f64>().ok().map(|left| LayoutMutation::UpdatePageMargins(UpdatePageMargins { id, top: page.margins.top, right: page.margins.right, bottom: page.margins.bottom, left })),
        "columnsCount" => value.parse::<f64>().ok().map(|v| LayoutMutation::UpdatePageColumns(UpdatePageColumns { id, count: v.max(0.0) as u32, gutter: page.columns.gutter })),
        "columnsGutter" => value.parse::<f64>().ok().map(|gutter| LayoutMutation::UpdatePageColumns(UpdatePageColumns { id, count: page.columns.count, gutter })),
        _ => None,
    }
}
//#endregion 🔖️Shared

//#region 🔖️AddFrame
//#endregion 🔖️AddFrame

//#region 🔖️AddPage
//#endregion 🔖️AddPage

//#region 🔖️PatchPage
//#endregion 🔖️PatchPage

//#region 🔖️PatchFrame
//#endregion 🔖️PatchFrame

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "patch-page")]
pub struct PatchPage {
    pub page_id: Option<String>,
    pub field: String,
    pub value: String,
}

pub async fn handle(payload: &PatchPage, doc: &ArtifactView<'_, LayoutSnapshot>, cfg: &ConfigView<'_, LayoutConfig>) -> Result<Emit<LayoutMutation, LayoutConfigMutation>, Fault> {
    let page_id = payload.page_id.clone().unwrap_or_else(|| cfg.snapshot.active_page_id.clone());
    match doc.snapshot.pages.iter().find(|page| page.id == page_id).and_then(|page| page_field_mutation(page, &payload.field, &payload.value)) {
        Some(mutation) => Ok(Emit::mutations(vec![mutation])),
        None => Ok(Emit::default()),
    }
}
