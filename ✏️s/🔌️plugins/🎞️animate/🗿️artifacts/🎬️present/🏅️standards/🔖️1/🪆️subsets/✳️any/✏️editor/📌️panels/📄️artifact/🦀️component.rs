//! 📄️ Animate present app panel — the document tree: tiles of the current deck.

use crate::artifacts::present::PresentSnapshot;
use crate::editor::animate::terminology::AnimatePresentLabels;
use crate::editor::animate::PRESENT_INTERACTION_DOMAIN;
use semio_framework_plugin::{tree_item_desc, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, FRAMEWORK_PANEL_TAB_ARTIFACT_ID, FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL};
use semio_framework_ui_contract::{BuiltNode, Label};

//#region 🔖️Constants
pub const PRESENT_PLAY_BODY_DOCUMENT: &str = "animate.present.play.document";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_ARTIFACT_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL, "Dokument"),
        group: PanelGroup::Workbench,
        body_key: Some(PRESENT_PLAY_BODY_DOCUMENT.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 🕹️ No per-row selection `action`: the tree is bound to the `tiles` interaction domain via
/// `.interaction_domain(...)` below, so the framework auto-injects `interactionSelect` for row
/// clicks — never declare that yourself (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM).
pub fn render(deck: &PresentSnapshot, labels: &AnimatePresentLabels) -> BuiltNode {
    let (_, tiles) = crate::artifacts::present::present_working_scene(deck);
    let items: Vec<BuiltNode> = tiles.iter().map(|tile| tree_item_desc(tile.id.clone(), Label::from(tile.name.clone()), Some(format!("x={:.3} y={:.3} w={:.3} h={:.3}", tile.crop.x, tile.crop.y, tile.crop.width, tile.crop.height)))).collect();
    PanelTreeBuilder::new("animate-present-play")
        .section_or_placeholder("animate-present-play.tiles", Some(Label::from(labels.tiles_section.as_str())), true, items, Label::from(labels.no_tiles.as_str()))
        .interaction_domain(PRESENT_INTERACTION_DOMAIN)
        .build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::animate::testkit::{present_app, render as render_body};
    use crate::editor::animate::PresentCommand;

    #[semio_framework_async_macros::async_test]
    async fn document_lists_seeded_tiles() {
        use semio_framework_plugin::testkit::meta;
        let mut app = present_app().await;
        app.dispatch_typed(PresentCommand::SeedGrid(crate::editor::animate::commands::seed_grid::SeedGrid { rows: 1, columns: 2 }), &meta("local")).await.expect("seed grid");
        let document = render_body(&mut app, PRESENT_PLAY_BODY_DOCUMENT).await;
        assert!(document.contains("tile-r0-c0"));
    }

    #[semio_framework_async_macros::async_test]
    async fn definition_binds_the_framework_document_tab_to_this_body_key() {
        let definition = definition();
        assert_eq!(definition.id(), FRAMEWORK_PANEL_TAB_ARTIFACT_ID);
        assert_eq!(definition.body_key.as_deref(), Some(PRESENT_PLAY_BODY_DOCUMENT));
    }
}
//#endregion 🧪️Tests
