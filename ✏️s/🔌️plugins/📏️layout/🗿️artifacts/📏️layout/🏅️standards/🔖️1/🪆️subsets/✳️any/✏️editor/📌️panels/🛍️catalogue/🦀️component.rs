//! 🛍️ Layout play app panel — the catalogue: draggable page/frame-kind creation items.

use crate::editor::layout::layout_action;
use crate::editor::layout::terminology::{catalogue_kind_label, LayoutLabels};
use semio_framework_plugin::{tree_item_with_action_draggable, Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, UiNode, UiTreeItemNode, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL};
use serde_json::{json, Value};

//#region 🔖️Constants
pub const LAYOUT_PLAY_BODY_CATALOGUE: &str = "layout.play.catalogue";

const LAYOUT_CATALOGUE_KINDS: &[(&str, &str)] = &[("rect", "square"), ("text", "type"), ("image", "image")];
const LAYOUT_CATALOGUE_DRAG_MIME: &str = "application/x-semio-catalogue-item";
const LAYOUT_CATALOGUE_KIND_MIME_PREFIX: &str = "application/x-semio-catalogue-kind.";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
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
fn catalogue_tree_item(kind: &str, label: impl Into<Label>, icon: &str) -> UiTreeItemNode {
    let action = if kind == "page" { layout_action("addPage", None) } else { layout_action("addFrame", Some(json!({ "kind": kind }))) };
    let mut drag_data_entries = serde_json::Map::new();
    drag_data_entries.insert(LAYOUT_CATALOGUE_DRAG_MIME.to_string(), json!(json!({ "kind": kind }).to_string()));
    drag_data_entries.insert(format!("{LAYOUT_CATALOGUE_KIND_MIME_PREFIX}{kind}"), json!(""));
    let drag_data = Value::Object(drag_data_entries);
    let mut item = tree_item_with_action_draggable(format!("layout-catalogue.{kind}"), label, Some(kind.into()), action, &drag_data);
    item.icon_id = Some(icon.into());
    item
}

pub fn render(labels: &LayoutLabels) -> UiNode {
    let mut items = vec![catalogue_tree_item("page", labels.catalogue_page, "file")];
    items.extend(LAYOUT_CATALOGUE_KINDS.iter().map(|(kind, icon)| catalogue_tree_item(kind, catalogue_kind_label(kind, labels), icon)));
    PanelTreeBuilder::new("layout-catalogue").section("layout-catalogue.kinds", Some(Label::data(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL)), true, items).build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::layout::testkit::{layout_app, render as render_body};

    #[test]
    fn catalogue_lists_frame_kinds() {
        let mut app = layout_app();
        let json = render_body(&mut app, LAYOUT_PLAY_BODY_CATALOGUE);
        assert!(json.contains("layout-catalogue.rect"));
        assert!(json.contains("Text Frame"));
    }

    #[test]
    fn catalogue_items_are_draggable() {
        let mut app = layout_app();
        let json = render_body(&mut app, LAYOUT_PLAY_BODY_CATALOGUE);
        assert!(json.contains(LAYOUT_CATALOGUE_DRAG_MIME));
        assert!(json.contains("\"draggable\":true"));
        assert!(json.contains("layout-catalogue.page"));
    }

    #[test]
    fn definition_binds_the_framework_catalogue_tab_to_this_body_key() {
        let definition = definition();
        assert_eq!(definition.id(), FRAMEWORK_PANEL_TAB_CATALOGUE_ID);
        assert_eq!(definition.body_key.as_deref(), Some(LAYOUT_PLAY_BODY_CATALOGUE));
    }
}
//#endregion 🧪️Tests
