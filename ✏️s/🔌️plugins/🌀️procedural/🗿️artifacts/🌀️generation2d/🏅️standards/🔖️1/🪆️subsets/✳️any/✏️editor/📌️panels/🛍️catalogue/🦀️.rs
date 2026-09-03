//! 🛍️ Generation2d play app panel — the widget/component catalogue and show-mode toggles.

use crate::editor::generation2d::terminology::Generation2dLabels;
use crate::editor::generation2d::GENERATION2D_PLAY_APP_ID;
use semio_framework_plugin::{tree_item_with_action, ActionFactory, BuiltNode, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL};

//#region 🔖️Constants
pub const GENERATION2D_PLAY_BODY_CATALOGUE: &str = "generation2d.play.catalogue";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_CATALOGUE_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, "Katalog"),
        group: PanelGroup::Workbench,
        body_key: Some(GENERATION2D_PLAY_BODY_CATALOGUE.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn render(labels: &Generation2dLabels) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let sources = [("inputSlider", labels.source_slider), ("inputNote", labels.source_note)];
    let components = [("math.add", labels.component_add), ("logic.and", labels.component_and), ("text.concat", labels.component_concat)];
    let sinks = [("outputPreview", labels.sink_preview), ("outputExport", labels.sink_export)];
    PanelTreeBuilder::new("procedural2d-play-catalogue")?
        .section(
            "procedural2d-play-catalogue.sources",
            Some(crate::ui_label(labels.sources.as_str())?),
            true,
            crate::ui_node_list(sources.iter().map(|(kind, label)| {
                let args = crate::ui_value_map([("kind", crate::ui_value_text(kind)?)])?;
                tree_item_with_action(format!("procedural2d-play-catalogue.source.{kind}"), label.as_str(), None, ActionFactory::new(GENERATION2D_PLAY_APP_ID).action("addWidget", Some(args))?)
            }))?,
        )?
        .section(
            "procedural2d-play-catalogue.components",
            Some(crate::ui_label(labels.components.as_str())?),
            true,
            crate::ui_node_list(components.iter().map(|(kind, label)| {
                let args = crate::ui_value_map([("kind", crate::ui_value_text("neuron")?), ("neuronKind", crate::ui_value_text(kind)?)])?;
                tree_item_with_action(format!("procedural2d-play-catalogue.component.{kind}"), label.as_str(), None, ActionFactory::new(GENERATION2D_PLAY_APP_ID).action("addWidget", Some(args))?)
            }))?,
        )?
        .section(
            "procedural2d-play-catalogue.sinks",
            Some(crate::ui_label(labels.sinks.as_str())?),
            true,
            crate::ui_node_list(sinks.iter().map(|(kind, label)| {
                let args = crate::ui_value_map([("kind", crate::ui_value_text(kind)?)])?;
                tree_item_with_action(format!("procedural2d-play-catalogue.sink.{kind}"), label.as_str(), None, ActionFactory::new(GENERATION2D_PLAY_APP_ID).action("addWidget", Some(args))?)
            }))?,
        )?
        .section(
            "procedural2d-play-catalogue.modes",
            Some(crate::ui_label(labels.show_mode_section.as_str())?),
            false,
            crate::ui_node_list(["preview", "generate", "wire"].iter().map(|mode| {
                let args = crate::ui_value_map([("value", crate::ui_value_text(mode)?)])?;
                tree_item_with_action(format!("procedural2d-play-catalogue.mode.{mode}"), format!("{} {mode}", labels.show_prefix.as_str()), None, ActionFactory::new(GENERATION2D_PLAY_APP_ID).action("setShowMode", Some(args))?)
            }))?,
        )?
        .build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::generation2d::testkit::{app, render as render_body};

    #[test]
    fn catalogue_lists_show_modes() {
        let mut app = app();
        assert!(render_body(&mut app, GENERATION2D_PLAY_BODY_CATALOGUE).contains("procedural2d-play-catalogue.mode.preview"));
    }

    #[test]
    fn generation2d_labels_resolve_native_english_by_default() {
        let mut app = app();
        let json = render_body(&mut app, GENERATION2D_PLAY_BODY_CATALOGUE);
        assert!(json.contains("\"Sources\""));
        assert!(json.contains("\"Components\""));
        assert!(json.contains("\"Sinks\""));
        assert!(json.contains("\"Show mode\""));
        assert!(!json.contains("Quellen"));
    }
}
//#endregion 🧪️Tests
