//! 🛍️ Procedural2d play app panel — the widget/component catalogue and show-mode toggles.

use crate::apps::procedural2d::procedural2d_action;
use crate::apps::procedural2d::terminology::Procedural2dLabels;
use semio_framework_plugin::{tree_item_with_action, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, UiNode, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL};
use serde_json::json;

//#region 🔖️Constants
pub const PROCEDURAL2D_PLAY_BODY_CATALOGUE: &str = "procedural2d.play.catalogue";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_CATALOGUE_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, "Katalog"),
        group: PanelGroup::Workbench,
        body_key: Some(PROCEDURAL2D_PLAY_BODY_CATALOGUE.into()),
        children: Vec::new()}
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn render(labels: &Procedural2dLabels) -> UiNode {
    let sources = [("inputSlider", labels.source_slider), ("inputNote", labels.source_note)];
    let components = [("math.add", labels.component_add), ("logic.and", labels.component_and), ("text.concat", labels.component_concat)];
    let sinks = [("outputPreview", labels.sink_preview), ("outputExport", labels.sink_export)];
    PanelTreeBuilder::new("procedural2d-play-catalogue")
        .section(
            "procedural2d-play-catalogue.sources",
            Some(labels.sources.into()),
            true,
            sources.iter().map(|(kind, label)| tree_item_with_action(format!("procedural2d-play-catalogue.source.{kind}"), *label, None, procedural2d_action("addWidget", Some(json!({ "kind": kind }))))).collect(),
        )
        .section(
            "procedural2d-play-catalogue.components",
            Some(labels.components.into()),
            true,
            components.iter().map(|(kind, label)| tree_item_with_action(format!("procedural2d-play-catalogue.component.{kind}"), *label, None, procedural2d_action("addWidget", Some(json!({ "kind": "neuron", "neuronKind": kind }))))).collect(),
        )
        .section(
            "procedural2d-play-catalogue.sinks",
            Some(labels.sinks.into()),
            true,
            sinks.iter().map(|(kind, label)| tree_item_with_action(format!("procedural2d-play-catalogue.sink.{kind}"), *label, None, procedural2d_action("addWidget", Some(json!({ "kind": kind }))))).collect(),
        )
        .section(
            "procedural2d-play-catalogue.modes",
            Some(labels.show_mode_section.into()),
            false,
            ["preview", "generate", "wire"]
                .iter()
                .map(|mode| tree_item_with_action(format!("procedural2d-play-catalogue.mode.{mode}"), semio_framework_plugin::Label::data(format!("{} {mode}", labels.show_prefix.as_str())), None, procedural2d_action("setShowMode", Some(json!({ "value": mode })))))
                .collect(),
        )
        .build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::procedural2d::testkit::{app, render as render_body};

    #[test]
    fn catalogue_lists_show_modes() {
        let mut app = app();
        assert!(render_body(&mut app, PROCEDURAL2D_PLAY_BODY_CATALOGUE).contains("procedural2d-play-catalogue.mode.preview"));
    }

    #[test]
    fn procedural2d_labels_resolve_native_english_by_default() {
        let mut app = app();
        let json = render_body(&mut app, PROCEDURAL2D_PLAY_BODY_CATALOGUE);
        assert!(json.contains("\"Sources\""));
        assert!(json.contains("\"Components\""));
        assert!(json.contains("\"Sinks\""));
        assert!(json.contains("\"Show mode\""));
        assert!(!json.contains("Quellen"));
    }
}
//#endregion 🧪️Tests
