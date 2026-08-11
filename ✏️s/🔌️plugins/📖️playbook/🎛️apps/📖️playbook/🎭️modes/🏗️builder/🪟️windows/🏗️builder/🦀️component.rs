//! 🏗️ Playbook play app — the builder window: the drag/drop Blockly-like form authoring surface.

use crate::apps::playbook::config::PlaybookConfig;
use crate::artifacts::playbook::{PlaybookSnapshot, PLAYBOOK_BUILTIN_KINDS};
use semio_framework::parse_contributions;
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
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
fn builtin_palette_tuples() -> Vec<(&'static str, &'static str, &'static str)> {
    PLAYBOOK_BUILTIN_KINDS.iter().map(|kind| (*kind, *kind, "circle")).collect()
}

/// 🗂️ `playbook.blockKind` topic payload shape (see `TopicContribution` in `semio-framework-manifest`).
#[derive(Clone, Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct PlaybookBlockKindTopicPayload {
    block_kind: String,
    label: String,
    icon_id: semio_framework::IconName,
}

const PLAYBOOK_BLOCK_KIND_TOPIC: &str = "playbook.blockKind";

/// 🗂️ Reads the open `TopicContribution` (`"playbook.blockKind"` topic) shape per entry.
fn extension_palette_entries(config: &PlaybookConfig) -> Vec<(String, String, String)> {
    parse_contributions(&config.contributions_json)
        .into_iter()
        .filter_map(|entry| {
            let topic_contribution = entry.topic_contribution.as_ref().filter(|topic_contribution| topic_contribution.topic == PLAYBOOK_BLOCK_KIND_TOPIC)?;
            let payload = topic_contribution.decode::<PlaybookBlockKindTopicPayload>().ok()?;
            Some((payload.block_kind, payload.label, payload.icon_id.to_string()))
        })
        .collect()
}

fn build_palette(config: &PlaybookConfig) -> Vec<BlockPaletteEntry> {
    let builtins = builtin_palette_tuples();
    crate::playbook::build_palette(&builtins, &extension_palette_entries(config))
}

fn playbook_builder_config() -> crate::playbook::PlaybookBuilderConfig {
    crate::playbook::PlaybookBuilderConfig { action_namespace: "playbook-builder", controller_id: PLAYBOOK_PLAY_CONTROLLER_ID, labels: crate::playbook::PLAYBOOK_BUILDER_LABELS_EN }
}

pub fn render(spec: &PlaybookSnapshot, config: &PlaybookConfig) -> UiNode {
    let kernel = spec.as_kernel();
    crate::playbook::render_playbook_builder(PLAYBOOK_PLAY_SURFACE_BUILDER, &kernel, &build_palette(config), config.selected_ids.first().map(String::as_str), &playbook_builder_config())
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

    /// 🗂️ The open `playbook.blockKind` topic shape must surface the palette entry.
    #[test]
    fn render_builder_palette_includes_topic_contributed_block_kinds() {
        use crate::apps::playbook::config::PlaybookConfig;
        use semio_framework::{ProgramContributionEntry, TopicContribution};
        let mut config = PlaybookConfig::default();
        let entry = ProgramContributionEntry {
            plugin_id: "playbook-module-procedural".into(),
            topic_contribution: Some(TopicContribution::new(
                "playbook.blockKind",
                serde_json::json!({ "blockKind": "buildingComponent", "label": "Building Component", "iconId": "building" }),
            )),
        };
        config.contributions_json = serde_json::to_string(&vec![entry]).unwrap();
        let palette = build_palette(&config);
        assert!(palette.iter().any(|entry| entry.block_kind == "buildingComponent"));
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
