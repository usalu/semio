//! 🛍️ CAD play app panel — the typology catalogue: one clickable row per creatable object typology.

use crate::editor::cad::terminology::{typology_label, CadLabels};
use crate::editor::cad::{cad_action, cad_tree_item, ui_label, ui_node_list, ui_value_map, ui_value_text, TYPOLOGY_CATALOG};
use semio_framework_plugin::{LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL};

//#region 🔖️Constants
pub const CAD_PLAY_BODY_CATALOGUE: &str = "cad.play.catalogue";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_CATALOGUE_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, "Katalog"),
        group: PanelGroup::Workbench,
        body_key: Some(CAD_PLAY_BODY_CATALOGUE.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn build_catalogue_tree(labels: &CadLabels) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let items = ui_node_list(TYPOLOGY_CATALOG.iter().map(|entry| {
        let args = ui_value_map([
            ("typology", ui_value_text(entry.typology)?),
            ("modelDefinitionId", ui_value_text(entry.model_definition_id)?),
        ])?;
        cad_tree_item(
                format!("cad-play-catalogue.{}", entry.typology),
                typology_label(entry.typology, labels).to_string(),
                Some(entry.icon),
                cad_action("addObject", Some(args))?,
            )
    }))?;
    PanelTreeBuilder::new("cad-play-catalogue")?.section("cad-play-catalogue.typologies", Some(ui_label(labels.typologies.as_str())?), true, items)?.build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::cad::standards::v1::subsets::any::schema::inferences::default_document;
    use crate::editor::cad::config::CadConfig;
    use crate::editor::cad::testkit::*;
    use crate::editor::cad::CadPlayApp;
    use semio_framework_plugin::ArtifactView;

    #[semio_framework_async_macros::async_test]
    async fn cad_labels_translate_catalogue_typologies_in_german() {
        let app = CadPlayApp::default();
        let scene = default_document();
        let history = empty_history();
        let doc = ArtifactView::new(&scene, &history);
        let config = CadConfig { locale: "de".into(), ..CadConfig::default() };
        let node = render_direct(&app, CAD_PLAY_BODY_CATALOGUE, &doc, &config);
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("Typologien"));
        assert!(json.contains("Quader"));
        assert!(json.contains("Platte"));
        assert!(json.contains("Stütze"));
        assert!(json.contains("Träger"));
        assert!(json.contains("Wand"));
        assert!(json.contains("Außenwand"));
        assert!(!json.contains("\"Slab\""));
        assert!(!json.contains("\"Balken\""));
    }
}
//#endregion 🧪️Tests
