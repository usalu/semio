//! 🔍️ Animate present app panel — the inspector: field editors for the selected tile(s).

use crate::apps::present::terminology::AnimatePresentLabels;
use crate::artifacts::present::{PresentSnapshot, PRESENT_DOCUMENT_SCHEMA};
use semio_framework_plugin::{ui_inspector_groups_to_tree, ui_inspector_readonly_field, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, UiInspectorFieldGroup, UiNode, UiPresence, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL};

//#region 🔖️Constants
pub const PRESENT_PLAY_BODY_DETAILS: &str = "animate.present.play.details";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition { kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_INSPECTION_ID.into()), label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, "Inspektion"), group: PanelGroup::Details, body_key: Some(PRESENT_PLAY_BODY_DETAILS.into()), children: Vec::new() }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// ⚠️ Ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the per-selected-tile field group
/// (crop x/y/width/height, name, delete) this panel used to build from `config.selected_ids` is
/// deleted along with that field — selection is framework-owned state now and
/// `ArtifactApp::render(body_key, doc, cfg)` is never given an `InteractionView` (only
/// `handle`/`copy_fragment`/`cut_operations` are). Documented reduced-fidelity gap, same shape as
/// `🖍️draw`'s `properties` panel (`🎛️apps/🖍️draw/📌️panels/🔍️properties/🦀️component.rs`): falls through
/// to a schema/tile-count summary until a resolved-selection render path exists.
pub fn render(deck: &PresentSnapshot, labels: &AnimatePresentLabels) -> UiNode {
    let (_, tiles) = crate::artifacts::present::present_working_scene(deck);
    ui_inspector_groups_to_tree(&[UiInspectorFieldGroup {
        id: "animate-present-play-inspector.empty".into(),
        label: semio_framework_plugin::Label::data(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL),
        default_open: Some(true),
        presence: UiPresence::default(),
        fields: vec![
            ui_inspector_readonly_field("animate-present-play-inspector.schema", labels.details_schema_field, PRESENT_DOCUMENT_SCHEMA),
            ui_inspector_readonly_field("animate-present-play-inspector.tiles", labels.details_tiles_field, tiles.len().to_string()),
        ],
    }])
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::present::testkit::{present_app, render as render_body};

    #[test]
    fn definition_binds_the_framework_inspection_tab_to_this_body_key() {
        let definition = definition();
        assert_eq!(definition.id(), FRAMEWORK_PANEL_TAB_INSPECTION_ID);
        assert_eq!(definition.body_key.as_deref(), Some(PRESENT_PLAY_BODY_DETAILS));
    }

    /// 🕹️ `render` has no `InteractionView` (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-
    /// MECHANISM), so the panel is a schema/tile-count summary regardless of selection now.
    #[test]
    fn details_panel_reports_schema_and_tile_count() {
        let mut app = present_app();
        let json_str = render_body(&mut app, PRESENT_PLAY_BODY_DETAILS);
        assert!(json_str.contains(PRESENT_DOCUMENT_SCHEMA));
    }
}
//#endregion 🧪️Tests
