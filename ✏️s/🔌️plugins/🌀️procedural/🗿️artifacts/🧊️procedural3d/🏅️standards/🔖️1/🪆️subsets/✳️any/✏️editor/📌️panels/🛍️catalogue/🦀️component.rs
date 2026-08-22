//! 🛍️ Procedural3d play app panel — the flow-node catalogue.

use crate::editor::procedural3d::terminology::Procedural3dLabels;
use crate::editor::procedural3d::PROCEDURAL_3D_PLAY_APP_ID;
use semio_framework_plugin::plugin_app_close_prelude::Component;
use semio_framework_plugin::{tree_item_with_action, ActionFactory, BuiltNode, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL};
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
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn render(labels: &Procedural3dLabels) -> BuiltNode {
    let sections = flow::flow_palette_catalogue_sections();
    let items: Vec<BuiltNode> = sections
        .iter()
        .flat_map(|section| {
            section.items.iter().map(|item| {
                let action_kind = if item.kind == "neuron" { format!("neuron|{}", item.neuron_kind.as_deref().unwrap_or("math.add")) } else { item.kind.clone() };
                let icon = if item.icon.starts_with("emoji:") { "box" } else { item.icon.as_str() };
                let mut node = tree_item_with_action(
                    format!("procedural-play-catalogue.{}", item.neuron_kind.as_deref().unwrap_or(&item.kind)),
                    item.name.clone(),
                    None,
                    ActionFactory::new(PROCEDURAL_3D_PLAY_APP_ID).action("addWidget", Some(json!({ "kind": action_kind }))),
                );
                if let Component::TreeItem(props) = &mut node.component {
                    props.icon = Some(icon.into());
                }
                node
            })
        })
        .collect();
    PanelTreeBuilder::new("procedural-play-catalogue").section("procedural-play-catalogue.widgets", Some(labels.widgets.as_str().into()), true, items).build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::procedural3d::testkit::{app, render as render_body};

    #[test]
    fn procedural3d_labels_resolve_native_english_by_default() {
        let _serial = crate::editor::procedural3d::test_support::lock();
        let mut app = app();
        let json = render_body(&mut app, PROCEDURAL_3D_PLAY_BODY_CATALOGUE);
        assert!(json.contains("\"Widgets\""));
        assert!(!json.contains("Elemente"));
    }
}
//#endregion 🧪️Tests
