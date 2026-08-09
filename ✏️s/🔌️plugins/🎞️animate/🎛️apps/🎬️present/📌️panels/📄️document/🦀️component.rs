//! 📄️ Animate present app panel — the document tree: tiles of the current deck.

use crate::apps::present::animate_present_action;
use crate::apps::present::terminology::AnimatePresentLabels;
use crate::artifacts::present::PresentSnapshot;
use semio_framework_plugin::{Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, UiNode, UiPresence, UiTreeItemNode, UiTreeNode, UiTreeSectionNode, FRAMEWORK_PANEL_TAB_DOCUMENT_ID, FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL};
use serde_json::json;

//#region 🔖️Constants
pub const PRESENT_PLAY_BODY_DOCUMENT: &str = "animate.present.play.document";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition { kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_DOCUMENT_ID.into()), label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, "Dokument"), group: PanelGroup::Workbench, body_key: Some(PRESENT_PLAY_BODY_DOCUMENT.into()), children: Vec::new() }
}
//#endregion 🔖️Definition

//#region 🔖️Render
fn tree_item(id: impl Into<String>, label: impl Into<Label>) -> UiTreeItemNode {
    UiTreeItemNode {
        id: id.into(),
        label: label.into(),
        description: None,
        icon_id: None,
        presence: UiPresence::default(),
        default_open: None,
        action: None,
        hover_action: None,
        unhover_action: None,
        actions: None,
        draggable: None,
        drag_data: None,
        items: None,
        control: None,
        dimmed: None,
        menu: None,
    }
}

pub fn render(deck: &PresentSnapshot, selected: &[String], labels: &AnimatePresentLabels) -> UiNode {
    let items: Vec<UiTreeItemNode> = deck
        .tiles
        .iter()
        .map(|tile| UiTreeItemNode {
            id: tile.id.clone(),
            label: Label::data(tile.name.clone()),
            description: Some(format!("x={:.3} y={:.3} w={:.3} h={:.3}", tile.crop.x, tile.crop.y, tile.crop.width, tile.crop.height)),
            icon_id: None,
            presence: UiPresence::selected(selected.contains(&tile.id)),
            default_open: None,
            action: Some(animate_present_action("setSelectedIds", Some(json!({ "ids": [tile.id] })))),
            hover_action: None,
            unhover_action: None,
            actions: None,
            draggable: None,
            drag_data: None,
            items: None,
            control: None,
            dimmed: None,
            menu: None,
        })
        .collect();
    UiNode::Tree(UiTreeNode {
        sections: vec![UiTreeSectionNode {
            id: "animate-present-play.tiles".into(),
            presence: UiPresence::default(),
            label: Some(labels.tiles_section.into()),
            default_open: Some(true),
            items: if items.is_empty() { vec![tree_item("empty", labels.no_tiles)] } else { items },
        }],
        presence: UiPresence::default(),
        selected_ids: None,
        highlighted_ids: None,
        selection_change: Some(animate_present_action("setSelectedIds", Some(json!({ "ids": [] })))),
        drop_action: None,
        menu: None,
    })
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::present::testkit::{present_app, render as render_body};
    use crate::apps::present::PresentCommand;

    #[test]
    fn document_lists_seeded_tiles() {
        use semio_framework_plugin::testkit::meta;
        let mut app = present_app();
        app.dispatch_typed(PresentCommand::SeedGrid(crate::apps::present::commands::grid::seed_grid::SeedGrid { rows: 1, columns: 2 }), &meta("local")).expect("seed grid");
        let document = render_body(&mut app, PRESENT_PLAY_BODY_DOCUMENT);
        assert!(document.contains("tile-r0-c0"));
    }

    #[test]
    fn definition_binds_the_framework_document_tab_to_this_body_key() {
        let definition = definition();
        assert_eq!(definition.id(), FRAMEWORK_PANEL_TAB_DOCUMENT_ID);
        assert_eq!(definition.body_key.as_deref(), Some(PRESENT_PLAY_BODY_DOCUMENT));
    }
}
//#endregion 🧪️Tests
