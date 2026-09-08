//! 🔍️ Animate presentation app panel — the inspector: field editors for the selected tile(s).

use crate::{PresentationSnapshot, PRESENTATION_DOCUMENT_SCHEMA};
use crate::editor::animate::terminology::AnimatePresentationLabels;
use semio_framework_plugin::{LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL};
use semio_framework_ui_contract::{column, field, section, text, BuiltNode};
use crate::editor::animate::{ui_children, ui_label, ui_node};

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
pub fn render(deck: &PresentationSnapshot, labels: &AnimatePresentationLabels) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let (_, tiles) = crate::presentation_working_scene(deck);
    let schema = ui_node(text(ui_label(PRESENTATION_DOCUMENT_SCHEMA)?), "animate-presentation-play-inspector.schema.value")?;
    let tile_count = ui_node(text(ui_label(tiles.len().to_string())?), "animate-presentation-play-inspector.tiles.value")?;
    let schema = ui_node(ui_children(field(ui_label(labels.details_schema_field.as_str())?), [schema])?, "animate-presentation-play-inspector.schema")?;
    let tile_count = ui_node(ui_children(field(ui_label(labels.details_tiles_field.as_str())?), [tile_count])?, "animate-presentation-play-inspector.tiles")?;
    let summary = ui_node(ui_children(section(ui_label(labels.details_title.as_str())?).default_open(true), [schema, tile_count])?, "animate-presentation-play-inspector.empty")?;
    ui_node(ui_children(column(), [summary])?, "animate-presentation-play-inspector")
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

#[cfg(test)]
mod semantic_contract {
    use super::*;

    fn project(node: BuiltNode) -> serde_json::Value {
        let text = semio_framework_plugin::testkit::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("retire semantic tree");
        serde_json::from_str(&text).expect("independent UI oracle")
    }

    #[test]
    fn presentation_semantic_panels_match_the_json_oracle() {
        let vectors: serde_json::Value = serde_json::from_str(include_str!("🧪️fixtures/🔣️panels.json")).expect("neutral UI vectors");
        let document = crate::default_presentation_snapshot();
        let (_, tiles) = crate::presentation_working_scene(&document);
        for row in vectors["cases"].as_array().expect("locales") {
            let labels = semio_framework_plugin::resolve_labels_for_locale::<AnimatePresentationLabels>(row["locale"].as_str().expect("locale"));
            let tree = project(crate::editor::animate::panels::artifact::render(&document, labels).expect("document"));
            assert_eq!(tree["children"][0]["component"]["label"], row["tiles"]);
            let catalogue = crate::editor::animate::panels::catalogue::render(&document, labels).expect("catalogue");
            let grid_args: Vec<serde_json::Value> = catalogue.children[0].children.iter().skip(1).take(2).map(|node| serde_json::to_value(&node.bindings[0].args).expect("independent binding oracle")).collect();
            let figure_args = serde_json::to_value(&catalogue.children[1].children[0].bindings[0].args).expect("independent figure binding oracle");
            let source_disabled = catalogue.children[1].children[1].children[0].disabled;
            let tree = project(catalogue);
            assert_eq!(tree["children"][0]["component"]["label"], row["templates"]);
            assert_eq!(tree["children"][1]["component"]["label"], row["figure"]);
            for (index, expected) in vectors["gridActions"].as_array().expect("grid actions").iter().enumerate() {
                assert_eq!(grid_args[index].as_object().expect("grid arguments").len(), expected.as_object().expect("grid vector").len());
                for field in ["columns", "rows"] {
                    assert_eq!(grid_args[index][field].as_f64(), expected[field].as_f64());
                }
            }
            assert_eq!(figure_args, vectors["figureSource"]);
            assert!(source_disabled);
            let tree = project(render(&document, labels).expect("inspection"));
            assert_eq!(tree["children"][0]["component"]["label"], row["inspection"]);
            let fields = tree["children"][0]["children"].as_array().expect("summary fields");
            assert_eq!(serde_json::Value::Array(fields.iter().map(|node| node["component"]["label"].clone()).collect()), row["summary"]);
            assert_eq!(fields[0]["children"][0]["component"]["value"], "animate.presentation");
            assert_eq!(fields[1]["children"][0]["component"]["value"], tiles.len().to_string());
        }
        for node in [
            crate::editor::animate::modes::main::windows::tile_editor::render(&document).expect("editor canvas"),
            crate::viewer::animate::modes::view::windows::tile_editor::render(&document).expect("viewer canvas"),
        ] {
            let semio_framework_plugin::Component::Surface(props) = &node.component else { panic!("canvas surface") };
            let scene: semio_framework_plugin::Canvas2dScene = semio_framework_ui_scene::decode(props).expect("packed canvas");
            assert_eq!(scene.camera_x, vectors["canvas"]["cameraX"].as_f64().expect("camera x"));
            assert_eq!(scene.camera_y, vectors["canvas"]["cameraY"].as_f64().expect("camera y"));
            assert_eq!(scene.zoom, vectors["canvas"]["zoom"].as_f64().expect("zoom"));
            let layers: serde_json::Value = serde_json::from_str(&scene.layers_json).expect("independent layers oracle");
            assert_eq!(layers.as_array().expect("layers").len(), tiles.len() + 1);
            assert_eq!(layers[0]["id"], "source-frame");
            project(node);
        }
    }
}
