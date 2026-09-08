//! ✏️ ✏️ Layout play app commands command — `patch-page`.

use crate::mutations::change_page_height::ChangePageHeight;
use crate::mutations::change_page_width::ChangePageWidth;
use crate::mutations::rename_page::RenamePage;
use crate::mutations::update_page_columns::UpdatePageColumns;
use crate::mutations::update_page_margins::UpdatePageMargins;
use crate::mutations::LayoutMutation;
use crate::{LayoutSnapshot, Page};
use crate::editor::layout::config::{LayoutConfig, LayoutConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Shared
/// 🎯️ Builds the exact semantic mutation for a `patchPage` field write from the command's text
/// `value`; unknown fields/mistyped (non-numeric where a number is expected) values yield `None`.
/// `marginTop`/`marginRight`/`marginBottom`/`marginLeft`/`columnsCount`/`columnsGutter` read the
/// page's OTHER current value(s) so `update-page-margins`/`update-page-columns` stay atomic.
fn page_field_mutation(page: &Page, field: &str, value: &str) -> Option<LayoutMutation> {
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

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "patch-page")]
pub struct PatchPage {
    pub page_id: Option<String>,
    pub field: String,
    pub value: String,
}

pub fn handle(payload: &PatchPage, doc: &ArtifactView<'_, LayoutSnapshot>, cfg: &ConfigView<'_, LayoutConfig>) -> Result<Emit<LayoutMutation, LayoutConfigMutation>, Fault> {
    let page_id = payload.page_id.clone().unwrap_or_else(|| cfg.snapshot.active_page_id.clone());
    match doc.snapshot.pages.iter().find(|page| page.id == page_id).and_then(|page| page_field_mutation(page, &payload.field, &payload.value)) {
        Some(mutation) => Ok(Emit::mutations(vec![mutation])),
        None => Ok(Emit::default()),
    }
}
