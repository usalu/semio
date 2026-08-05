//! 🏗️ Playbook play app — the builder window: the drag/drop Blockly-like form authoring surface.

use crate::apps::playbook::config::PlaybookConfig;
use crate::artifacts::playbook::{PlaybookSpec, PLAYBOOK_BUILTIN_KINDS};
use semio_framework_plugin::{BlockPaletteEntry, LocalizedLabel, SurfaceKind, UiNode, WindowKindDefinition, WindowOptions};

//#region 🔖️Constants
pub const PLAYBOOK_PLAY_WINDOW_BUILDER: &str = "playbook-builder";
pub const PLAYBOOK_PLAY_BODY_BUILDER: &str = "playbook.play.builder";
const PLAYBOOK_PLAY_SURFACE_BUILDER: &str = "playbook.play.builder";
const PLAYBOOK_PLAY_CONTROLLER_ID: &str = "playbook-play";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: PLAYBOOK_PLAY_WINDOW_BUILDER.into(),
        label: LocalizedLabel::native("Builder", "Builder"),
        body_key: PLAYBOOK_PLAY_BODY_BUILDER.into(),
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
fn builtin_palette() -> Vec<BlockPaletteEntry> {
    PLAYBOOK_BUILTIN_KINDS.iter().map(|kind| BlockPaletteEntry { block_kind: (*kind).into(), label: (*kind).into(), icon_id: "circle".into() }).collect()
}

fn playbook_builder_config() -> playbook::PlaybookBuilderConfig {
    playbook::PlaybookBuilderConfig { action_namespace: "playbook-builder", controller_id: PLAYBOOK_PLAY_CONTROLLER_ID, labels: playbook::PLAYBOOK_BUILDER_LABELS_EN }
}

pub fn render(spec: &PlaybookSpec, config: &PlaybookConfig) -> UiNode {
    playbook::render_playbook_builder(PLAYBOOK_PLAY_SURFACE_BUILDER, spec, &builtin_palette(), config.selected_ids.first().map(String::as_str), &playbook_builder_config())
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::playbook::testkit::{playbook_app, render as render_body};
    use crate::apps::playbook::PLAYBOOK_PLAY_BODY_BUILDER as BODY_BUILDER;

    #[test]
    fn definition_declares_the_block_list_surface_and_body_key() {
        let definition = definition();
        assert_eq!(definition.body_key, PLAYBOOK_PLAY_BODY_BUILDER);
        assert!(matches!(definition.surface_kind, SurfaceKind::BlockList));
    }

    #[test]
    fn render_builder_emits_playbook_list_component_scene() {
        let mut app = playbook_app();
        let json = render_body(&mut app, BODY_BUILDER);
        assert!(json.contains(r#""componentKind":"block-list""#));
        assert!(json.contains(&format!(r#""surfaceId":"{PLAYBOOK_PLAY_SURFACE_BUILDER}""#)));
    }
}
//#endregion 🧪️Tests
