//! ✏️ ✏️ Layout play app commands command — `patch-page`.

use semio_framework_plugin::{NoConfig, NoConfigMutation};
use crate::editor::layout::modes::edit::windows::blueprint::config::current;
use crate::mutations::change_page_height::ChangePageHeight;
use crate::mutations::change_page_width::ChangePageWidth;
use crate::mutations::delete_page::DeletePage;
use crate::mutations::rename_page::RenamePage;
use crate::mutations::reorder_pages::ReorderPages;
use crate::mutations::create_layer::CreateLayer;
use crate::mutations::set_page_guides::SetPageGuides;
use crate::mutations::set_page_parent::SetPageParent;
use crate::LayoutRect;
use crate::mutations::update_layer::UpdateLayer;
use crate::mutations::update_page_columns::UpdatePageColumns;
use crate::mutations::update_page_margins::UpdatePageMargins;
use crate::mutations::LayoutMutation;
use crate::{LayoutSnapshot, Page};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Shared
/// 🎯️ Builds the exact semantic mutation for a `patchPage` field write from the command's text
/// `value`; unknown fields/mistyped (non-numeric where a number is expected) values yield `None`.
/// `marginTop`/`marginRight`/`marginBottom`/`marginLeft`/`columnsCount`/`columnsGutter` read the
/// page's OTHER current value(s) so `update-page-margins`/`update-page-columns` stay atomic.
fn page_field_mutation(document: &LayoutSnapshot, page: &Page, field: &str, value: &str) -> Option<LayoutMutation> {
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
        "addGuide" => {
            let mut guides = page.guides.clone();
            if guides.len() >= 64 {
                return None;
            }
            guides.push(LayoutRect { x: 0.0, y: 0.0, width: page.width, height: 0.0 });
            Some(LayoutMutation::SetPageGuides(SetPageGuides { id, guides }))
        }
        "addLayer" => {
            if page.layers.len() >= 16 {
                return None;
            }
            let mut index = page.layers.len() + 1;
            let layer_id = loop {
                let candidate = format!("layer-{index}");
                if page.layers.iter().all(|layer| layer.id != candidate) {
                    break candidate;
                }
                index += 1;
            };
            let trimmed = value.trim();
            let name = if trimmed.is_empty() { "Layer".to_string() } else { trimmed.to_string() };
            Some(LayoutMutation::CreateLayer(CreateLayer { page_id: id, id: layer_id, name, remove: false, index: None }))
        }
        "parentPageId" => {
            let trimmed = value.trim();
            let parent_page_id = (!trimmed.is_empty()).then(|| trimmed.to_string());
            Some(LayoutMutation::SetPageParent(SetPageParent { id, parent_page_id }))
        }
        "delete" => (document.pages.len() > 1).then(|| LayoutMutation::DeletePage(DeletePage { id })),
        "moveEarlier" => {
            let index = document.pages.iter().position(|item| item.id == page.id)?;
            (index > 0).then(|| LayoutMutation::ReorderPages(ReorderPages { id, to_index: index - 1 }))
        }
        "moveLater" => {
            let index = document.pages.iter().position(|item| item.id == page.id)?;
            (index + 1 < document.pages.len()).then(|| LayoutMutation::ReorderPages(ReorderPages { id, to_index: index + 1 }))
        }
        _ => guide_mutation(page, field, value).or_else(|| layer_mutation(page, field, value)),
    }
}


fn guide_mutation(page: &Page, field: &str, value: &str) -> Option<LayoutMutation> {
    let rest = field.strip_prefix("guide.")?;
    let (index_text, key) = rest.split_once('.')?;
    let index = index_text.parse::<usize>().ok()?;
    let mut guides = page.guides.clone();
    if key == "delete" {
        if index >= guides.len() {
            return None;
        }
        guides.remove(index);
    } else {
        let guide = guides.get_mut(index)?;
        let number = value.parse::<f64>().ok()?;
        match key {
            "x" => guide.x = number,
            "y" => guide.y = number,
            "width" => guide.width = number,
            "height" => guide.height = number,
            _ => return None,
        }
    }
    Some(LayoutMutation::SetPageGuides(SetPageGuides { id: page.id.clone(), guides }))
}

fn layer_mutation(page: &Page, field: &str, value: &str) -> Option<LayoutMutation> {
    let (layer_id, key) = field.rsplit_once('.')?;
    if !matches!(key, "name" | "visible" | "locked") {
        return None;
    }
    let layer = page.layers.iter().find(|layer| layer.id == layer_id)?;
    let mut name = layer.name.clone();
    let mut visible = layer.visible;
    let mut locked = layer.locked;
    match key {
        "name" => name = value.to_string(),
        "visible" => visible = matches!(value.trim(), "true" | "1"),
        "locked" => locked = matches!(value.trim(), "true" | "1"),
        _ => return None,
    }
    Some(LayoutMutation::UpdateLayer(UpdateLayer { page_id: page.id.clone(), layer_id: layer_id.to_string(), name, visible, locked }))
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

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "patch-page")]
pub struct PatchPage {
    pub page_id: Option<String>,
    pub field: String,
    pub value: String,
}

pub fn handle(payload: &PatchPage, doc: &ArtifactView<'_, LayoutSnapshot>, cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<LayoutMutation, NoConfigMutation>, Fault> {
    let page_id = payload.page_id.clone().unwrap_or_else(|| current(cfg).active_page_id);
    match doc.snapshot.pages.iter().find(|page| page.id == page_id).and_then(|page| page_field_mutation(doc.snapshot, page, &payload.field, &payload.value)) {
        Some(mutation) => Ok(Emit::mutations(vec![mutation])),
        None => Ok(Emit::default()),
    }
}
