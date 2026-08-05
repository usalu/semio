//! 🧱️ Forms play app — the blueprint window: the drag/drop playbook builder authoring the form.

use crate::apps::forms::config::FormsConfig;
use crate::apps::forms::terminology::FormsLabels;
use crate::artifacts::forms::FormSpec;
use semio_framework_plugin::{BlockPaletteEntry, LocalizedLabel, SurfaceKind, UiNode, WindowKindDefinition, WindowOptions};

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
        document_projection_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
fn forms_playbook_builder_config() -> playbook::PlaybookBuilderConfig {
    playbook::PlaybookBuilderConfig { action_namespace: "forms-blueprint", controller_id: crate::apps::forms::FORMS_PLAY_APP_ID, labels: playbook::PLAYBOOK_BUILDER_LABELS_EN }
}

pub fn render(spec: &FormSpec, config: &FormsConfig, labels: &FormsLabels) -> UiNode {
    let contributions = crate::apps::forms::parse_contributions(config);
    let palette: Vec<BlockPaletteEntry> = crate::apps::forms::catalogue_kinds(&contributions, labels).into_iter().map(|(kind, label, icon_id)| BlockPaletteEntry { block_kind: kind, label, icon_id }).collect();
    let builder_config = forms_playbook_builder_config();
    playbook::render_playbook_builder(FORMS_PLAY_SURFACE_BLUEPRINT, spec, &palette, config.selected_ids.first().map(String::as_str), &builder_config)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::forms::testkit::{forms_app, render as render_body};
    use crate::apps::forms::FORMS_PLAY_BODY_BLUEPRINT as BODY_BLUEPRINT;

    #[test]
    fn renders_blueprint_builder_cards() {
        let mut app = forms_app();
        let first_question_id = app.projection().expect("projection").steps[0].blocks[0].id.clone();
        let json = render_body(&mut app, BODY_BLUEPRINT);
        assert!(json.contains(r#""componentKind":"block-list""#));
        assert!(json.contains(r#""surfaceId":"forms.play.blueprint""#));
        assert!(json.contains("\"blockList\""));
        assert!(json.contains(&first_question_id));
    }

    #[test]
    fn definition_declares_the_block_list_surface_and_body_key() {
        let definition = definition();
        assert_eq!(definition.body_key, FORMS_PLAY_BODY_BLUEPRINT);
        assert!(matches!(definition.surface_kind, SurfaceKind::BlockList));
    }
}
//#endregion 🧪️Tests
