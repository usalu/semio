//! 🧱️ Forms play app — the blueprint window: the drag/drop playbook builder authoring the form.

use crate::FormsSnapshot;
use crate::editor::forms::config::FormsConfig;
use crate::editor::forms::terminology::FormsLabels;
use semio_framework_plugin::{BlockPaletteEntry, LocalizedLabel, SurfaceKind, WindowKindDefinition, WindowOptions};

//#region 🔖️Constants
pub const FORMS_PLAY_WINDOW_BLUEPRINT: &str = "forms-blueprint";
pub const FORMS_PLAY_BODY_BLUEPRINT: &str = "forms.play.blueprint";
const FORMS_PLAY_SURFACE_BLUEPRINT: &str = "forms.play.blueprint";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: FORMS_PLAY_WINDOW_BLUEPRINT.into(),
        label: LocalizedLabel::native("Blueprint", "Entwurf"),
        body_key: FORMS_PLAY_BODY_BLUEPRINT.into(),
        surface_kind: SurfaceKind::BlockList,
        icon_id: "clipboard-list".into(),
        options: WindowOptions::default(),
        actions: Vec::new(),
        utilities: Vec::new(),
        params_schema: None,
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
        // 🕹️ Populated post-hoc by `create_forms_app`'s `.window_kind_interactions(..)` call.
        interactions: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `ArtifactEditor::render` carries no
/// `InteractionView` (a known SDK gap — matches `gis2d`'s and `note`'s inspection panel precedent), so
/// this block-list surface's own selected-card highlight (`render_playbook_builder`'s `selected_id`)
/// can no longer be driven from live framework selection — it always renders with none highlighted now.
pub fn render(spec: &FormsSnapshot, config: &FormsConfig, labels: &FormsLabels) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let contributions = crate::editor::forms::parse_contributions(config);
    let palette: Vec<BlockPaletteEntry> = crate::editor::forms::catalogue_kinds(&contributions, labels).into_iter().map(|(kind, label, icon_id)| BlockPaletteEntry { block_kind: kind, label, icon_id }).collect();
    semio_framework_plugin::scene_surface(FORMS_PLAY_SURFACE_BLUEPRINT, semio_framework_ui_contract::SurfaceKind::BlockList, &crate::playbook::build_playbook_list_scene(&crate::mutations::as_playbook_spec(spec), &palette, None))
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn renders_blueprint_builder_cards() {
        let spec = FormsSnapshot::default();
        let config = FormsConfig::default();
        let labels = crate::editor::forms::terminology::forms_play_labels(&config);
        let node = render(&spec, &config, labels).expect("blueprint surface");
        let semio_framework_ui_contract::Component::Surface(props) = node.component else { panic!("blueprint must render a semantic surface") };
        let scene: semio_framework_ui_scene::BlockListScene = semio_framework_ui_scene::decode(&props).expect("block-list payload");
        let expected = crate::mutations::as_playbook_spec(&spec);
        assert_eq!(serde_json::from_str::<serde_json::Value>(&scene.steps_json).unwrap(), serde_json::to_value(&expected.steps).unwrap());
        assert!(scene.selected_id.is_none());
    }

    #[semio_framework_async_macros::async_test]
    async fn definition_declares_the_block_list_surface_and_body_key() {
        let definition = definition();
        assert_eq!(definition.body_key, FORMS_PLAY_BODY_BLUEPRINT);
        assert!(matches!(definition.surface_kind, SurfaceKind::BlockList));
    }
}
//#endregion 🧪️Tests
