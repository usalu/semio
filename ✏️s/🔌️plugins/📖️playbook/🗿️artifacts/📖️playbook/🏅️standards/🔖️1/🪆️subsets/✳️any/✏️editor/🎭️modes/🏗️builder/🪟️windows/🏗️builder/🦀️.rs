//! 🏗️ Playbook play app — the builder window: the drag/drop Blockly-like form authoring surface.

use crate::{PlaybookSnapshot, PLAYBOOK_BUILTIN_KINDS};
use crate::editor::playbook::config::PlaybookConfig;
use semio_framework::parse_contributions;
use semio_framework_plugin::{BlockPaletteEntry, LocalizedLabel, SurfaceKind, WindowKindDefinition, WindowOptions};

//#region 🔖️Constants
pub const PLAYBOOK_PLAY_WINDOW_BUILDER: &str = "playbook-builder";
pub const PLAYBOOK_PLAY_BODY_BUILDER: &str = "playbook.play.builder";
const PLAYBOOK_PLAY_SURFACE_BUILDER: &str = "playbook.play.builder";
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
        // 🕹️ Populated post-hoc by `create_playbook_play_app`'s `.window_kind_interactions(..)` call.
        interactions: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
fn builtin_palette_tuples() -> Vec<(&'static str, &'static str, &'static str)> {
    PLAYBOOK_BUILTIN_KINDS.iter().map(|kind| (*kind, *kind, "circle")).collect()
}

/// 🗂️ `playbook.blockKind` topic payload shape (see `TopicContribution` in `semio-framework-manifest`).
#[derive(Clone, Debug, semio_framework_value_derive::FromValue)]
#[value(rename_all = "camelCase")]
struct PlaybookBlockKindTopicPayload {
    block_kind: String,
    label: String,
    icon_id: String,
}

const PLAYBOOK_BLOCK_KIND_TOPIC: &str = "playbook.blockKind";

/// 🗂️ Reads the open `TopicContribution` (`"playbook.blockKind"` topic) shape per entry.
fn extension_palette_entries(config: &PlaybookConfig) -> Vec<(String, String, String)> {
    parse_contributions(&config.contributions_json)
        .into_iter()
        .filter_map(|entry| {
            let topic_contribution = entry.topic_contribution.as_ref().filter(|topic_contribution| topic_contribution.topic == PLAYBOOK_BLOCK_KIND_TOPIC)?;
            let payload = topic_contribution.decode::<PlaybookBlockKindTopicPayload>().ok()?;
            Some((payload.block_kind, payload.label, payload.icon_id))
        })
        .collect()
}

fn build_palette(config: &PlaybookConfig) -> Vec<BlockPaletteEntry> {
    let builtins = builtin_palette_tuples();
    crate::playbook::build_palette(&builtins, &extension_palette_entries(config))
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `ArtifactEditor::render` carries no
/// `InteractionView` (a known SDK gap — matches `forms`'/`note`'s render-surface precedent), so this
/// block-list surface's own selected-card highlight (`render_playbook_builder`'s `selected_id`) can no
/// longer be driven from live framework selection — it always renders with none highlighted now.
pub fn render(spec: &PlaybookSnapshot, config: &PlaybookConfig) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let kernel = spec.as_kernel();
    semio_framework_plugin::scene_surface(PLAYBOOK_PLAY_SURFACE_BUILDER, semio_framework_ui_contract::SurfaceKind::BlockList, &crate::playbook::build_playbook_list_scene(&kernel, &build_palette(config), None))
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
