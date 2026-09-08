//! 🛍️ Layout play app panel — the catalogue: draggable page/frame-kind creation items.

use crate::editor::layout::terminology::{catalogue_kind_label, LayoutLabels};
use crate::editor::layout::{layout_action, ui_value_map, ui_value_text};
use semio_framework_plugin::{
    tree_item_with_action, Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, PluginAssemblyError, UiFixedList, UiFixedMap, UiText, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
};

//#region 🔖️Constants
pub(crate) const LAYOUT_PLAY_BODY_CATALOGUE: &str = "layout.play.catalogue";

const LAYOUT_CATALOGUE_KINDS: &[(&str, &str)] = &[("rect", "square"), ("text", "type"), ("image", "image")];
const LAYOUT_CATALOGUE_DRAG_MIME: &str = "application/x-semio-catalogue-item";
const LAYOUT_CATALOGUE_KIND_MIME_PREFIX: &str = "application/x-semio-catalogue-kind.";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub(crate) fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_CATALOGUE_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, "Katalog"),
        group: PanelGroup::Workbench,
        body_key: Some(LAYOUT_PLAY_BODY_CATALOGUE.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
fn catalogue_tree_item(kind: &str, label: impl Into<Label>, icon: &str) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let action = if kind == "page" { layout_action("addPage", None)? } else { layout_action("addFrame", Some(ui_value_map([("kind", ui_value_text(kind)?)])?))? };
    let mut item = tree_item_with_action(format!("layout-catalogue.{kind}"), label.into().as_str(), Some(kind.into()), action)?;
    let mut drag_data = UiFixedMap::default();
    let payload = UiText::try_from_string(serde_json::json!({ "kind": kind }).to_string()).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "layout catalogue drag payload admission failed"))?;
    drag_data
        .try_push(UiText::try_from_str(LAYOUT_CATALOGUE_DRAG_MIME).ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "layout catalogue drag mime admission failed"))?, payload)
        .map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "layout catalogue drag entry admission failed"))?;
    drag_data
        .try_push(
            UiText::try_from_string(format!("{LAYOUT_CATALOGUE_KIND_MIME_PREFIX}{kind}")).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "layout catalogue kind mime admission failed"))?,
            UiText::try_from_str("").ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "layout catalogue empty drag value admission failed"))?,
        )
        .map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "layout catalogue kind drag entry admission failed"))?;
    if let semio_framework_plugin::Component::TreeItem(props) = &mut item.component {
        props.icon = Some(UiText::try_from_str(icon).ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "layout catalogue icon admission failed"))?);
        props.draggable = Some(true);
        props.drag_data = Some(drag_data);
    }
    Ok(item)
}

pub(crate) fn render(labels: &LayoutLabels) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let mut items = UiFixedList::default();
    let page = catalogue_tree_item("page", labels.catalogue_page, "file")?;
    items.try_push(page).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "layout catalogue page admission failed"))?;
    for (kind, icon) in LAYOUT_CATALOGUE_KINDS {
        let item = catalogue_tree_item(kind, catalogue_kind_label(kind, labels), icon)?;
        items.try_push(item).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "layout catalogue frame admission failed"))?;
    }
    PanelTreeBuilder::new("layout-catalogue")?.section("layout-catalogue.kinds", Some(crate::editor::layout::ui_label(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL)?), true, items)?.build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
