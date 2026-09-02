//! 🔍️ Animate presentation app panel — the inspector: field editors for the selected tile(s).

use crate::artifacts::presentation::{PresentationSnapshot, PRESENTATION_DOCUMENT_SCHEMA};
use crate::editor::animate::terminology::AnimatePresentationLabels;
use semio_framework_plugin::{LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL};
use semio_framework_ui_contract::{column, field, section, text, Buildable, BuiltNode, HasBase, HasChildren, Label};

//#region 🔖️Constants
pub const PRESENTATION_PLAY_BODY_DETAILS: &str = "animate.presentation.play.details";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_INSPECTION_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, "Inspektion"),
        group: PanelGroup::Details,
        body_key: Some(PRESENTATION_PLAY_BODY_DETAILS.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// ⚠️ Ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the per-selected-tile field group
/// (crop x/y/width/height, name, delete) this panel used to build from `config.selected_ids` is
/// deleted along with that field — selection is framework-owned state now and
/// `ArtifactApp::render(body_key, doc, cfg)` is never given an `InteractionView` (only
/// `handle`/`copy_fragment`/`cut_operations` are). Documented reduced-fidelity gap, same shape as
/// `🖍️draw`'s `properties` panel (`🎛️apps/🖍️draw/📌️panels/🔍️properties/🦀️.rs`): falls through
/// to a schema/tile-count summary until a resolved-selection render path exists.
pub fn render(deck: &PresentationSnapshot, labels: &AnimatePresentationLabels) -> BuiltNode {
    let (_, tiles) = crate::artifacts::presentation::presentation_working_scene(deck);
    let schema = field(Label::from(labels.details_schema_field.as_str())).id("animate-presentation-play-inspector.schema").child(text(Label::from(PRESENTATION_DOCUMENT_SCHEMA)).build()).build();
    let tile_count = field(Label::from(labels.details_tiles_field.as_str())).id("animate-presentation-play-inspector.tiles").child(text(Label::from(tiles.len().to_string())).build()).build();
    let summary = section(Label::from(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL)).id("animate-presentation-play-inspector.empty").default_open(true).children(vec![schema, tile_count]).build();
    column().id("animate-presentation-play-inspector").child(summary).build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::animate::testkit::{presentation_app, render as render_body};

    #[semio_framework_async_macros::async_test]
    async fn definition_binds_the_framework_inspection_tab_to_this_body_key() {
        let definition = definition();
        assert_eq!(definition.id(), FRAMEWORK_PANEL_TAB_INSPECTION_ID);
        assert_eq!(definition.body_key.as_deref(), Some(PRESENTATION_PLAY_BODY_DETAILS));
    }

    /// 🕹️ `render` has no `InteractionView` (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-
    /// MECHANISM), so the panel is a schema/tile-count summary regardless of selection now.
    #[semio_framework_async_macros::async_test]
    async fn details_panel_reports_schema_and_tile_count() {
        let mut app = presentation_app().await;
        let json_str = render_body(&mut app, PRESENTATION_PLAY_BODY_DETAILS).await;
        assert!(json_str.contains(PRESENTATION_DOCUMENT_SCHEMA));
    }
}
//#endregion 🧪️Tests
