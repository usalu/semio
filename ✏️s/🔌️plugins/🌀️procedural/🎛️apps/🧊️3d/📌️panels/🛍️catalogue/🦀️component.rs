//! 🛍️ Procedural3d play app panel — the flow-node catalogue.

use crate::apps::procedural3d::procedural3d_action;
use crate::apps::procedural3d::terminology::Procedural3dLabels;
use semio_framework_plugin::{tree_item_with_action, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, UiNode, UiTreeItemNode, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL};
use serde_json::json;

//#region 🔖️Constants
pub const PROCEDURAL_3D_PLAY_BODY_CATALOGUE: &str = "procedural.play.catalogue";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_CATALOGUE_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, "Katalog"),
        group: PanelGroup::Workbench,
        body_key: Some(PROCEDURAL_3D_PLAY_BODY_CATALOGUE.into()),
        children: Vec::new()}
}
//#endregion 🔖️Definition

//#region 🔖️Render
fn tree_item_with_icon(id: impl Into<String>, label: impl Into<semio_framework_plugin::Label>, icon_id: Option<&str>, action: semio_framework_plugin::ActionDescriptor) -> UiTreeItemNode {
    UiTreeItemNode { icon_id: icon_id.map(Into::into), menu: None, ..tree_item_with_action(id, label, None, action) }
}

pub fn render(labels: &Procedural3dLabels) -> UiNode {
    let sections = flow::flow_palette_catalogue_sections();
    let items: Vec<UiTreeItemNode> = sections
        .iter()
        .flat_map(|section| {
            section.items.iter().map(|item| {
                let action_kind = if item.kind == "neuron" { format!("neuron|{}", item.neuron_kind.as_deref().unwrap_or("math.add")) } else { item.kind.clone() };
                let icon = if item.icon.starts_with("emoji:") { "box" } else { item.icon.as_str() };
                tree_item_with_icon(format!("procedural-play-catalogue.{}", item.neuron_kind.as_deref().unwrap_or(&item.kind)), semio_framework_plugin::Label::data(item.name.clone()), Some(icon), procedural3d_action("addWidget", Some(json!({ "kind": action_kind }))))
            })
        })
        .collect();
    PanelTreeBuilder::new("procedural-play-catalogue").section("procedural-play-catalogue.widgets", Some(labels.widgets.into()), true, items).build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::procedural3d::testkit::{app, render as render_body};

    #[test]
    fn procedural3d_labels_resolve_native_english_by_default() {
        let _serial = crate::apps::procedural3d::test_support::lock();
        let mut app = app();
        let json = render_body(&mut app, PROCEDURAL_3D_PLAY_BODY_CATALOGUE);
        assert!(json.contains("\"Widgets\""));
        assert!(!json.contains("Elemente"));
    }
}
//#endregion 🧪️Tests
