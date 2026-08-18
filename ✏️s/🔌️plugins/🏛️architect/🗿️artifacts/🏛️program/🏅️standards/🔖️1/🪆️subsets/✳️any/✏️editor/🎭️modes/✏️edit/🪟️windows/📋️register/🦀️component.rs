//! 📋️ Architect register window — the active register's rows as a block-list surface.

use crate::editor::architect::catalog::register_entities;
use crate::editor::architect::chrome::{empty_component_scene, entity_id_from_json, entity_name_from_json};
use crate::editor::architect::config::{active_register, ArchitectConfig};
use crate::artifacts::program::ProgramSnapshot;
use semio_framework_plugin::{ui_text, BlockListScene, Label, LocalizedLabel, SurfaceKind, UiNode, WindowKindDefinition, WindowOptions};
use serde::{Deserialize, Serialize};
use serde_json::json;

//#region 🔖️Constants
pub const ARCHITECT_WINDOW_REGISTER: &str = "architect-register";
pub const ARCHITECT_BODY_REGISTER: &str = "architect.register";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🏛️ Stitched into the app manifest by `crate::editor::architect::create_architect_app`.
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: ARCHITECT_WINDOW_REGISTER.into(),
        label: LocalizedLabel::native("Register", "Register"),
        body_key: ARCHITECT_BODY_REGISTER.into(),
        surface_kind: SurfaceKind::BlockList,
        icon_id: "list".into(),
        options: WindowOptions::default(),
        actions: Vec::new(),
        utilities: Vec::new(),
        params_schema: None,
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
        // 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `BlockListScene` has no
        // `interaction_domain` field for the wrapper to stamp, so this window is not scoped to the
        // "program" domain (mirrors the graph window's declaration, not this one).
        interactions: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 🧱️ One block-list step per register row — the wire shape the block-list surface consumes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RegisterBlockStep {
    id: String,
    title: String,
    blocks: Vec<RegisterBlockItem>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RegisterBlockItem {
    id: String,
    label: String,
    kind: String,
}

pub fn render(program: &ProgramSnapshot, cfg: &ArchitectConfig) -> UiNode {
    let register = active_register(cfg);
    let entities = register_entities(program, register);
    if entities.is_empty() {
        return ui_text(Label::data(format!("No entities in register '{register}'.")));
    }

    let steps: Vec<RegisterBlockStep> = entities
        .iter()
        .filter_map(|entity| {
            let id = entity_id_from_json(entity)?;
            let name = entity_name_from_json(entity);
            Some(RegisterBlockStep { id: id.clone(), title: name.clone(), blocks: vec![RegisterBlockItem { id: format!("{id}-block"), label: name, kind: register.into() }] })
        })
        .collect();
    let steps_json = serde_json::to_string(&steps).unwrap_or_else(|_| "[]".into());
    let palette_json = serde_json::to_string(&[json!({
        "blockKind": register,
        "label": register,
        "iconId": "square",
    })])
    .unwrap_or_else(|_| "[]".into());
    // 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `ArtifactEditor::render` carries no
    // `InteractionView` and `BlockListScene` has no `interaction_domain` field for the wrapper to
    // stamp post-render either (unlike `UiNode::Tree`) — `selected_id` is left at `None`, matching
    // `dag`'s/`space`'s identical `NodeGraphScene` gap.
    let mut scene = empty_component_scene(ARCHITECT_BODY_REGISTER, SurfaceKind::BlockList);
    scene.block_list = Some(BlockListScene { steps_json, palette_json, selected_id: None, dragging_id: None, domain_id: None });
    UiNode::ComponentScene(scene)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::program::sample_plugin;

    #[test]
    fn definition_declares_the_block_list_surface_and_body_key() {
        let definition = definition();
        assert_eq!(definition.body_key, ARCHITECT_BODY_REGISTER);
        assert!(matches!(definition.surface_kind, SurfaceKind::BlockList));
    }

    #[test]
    fn the_active_registers_rows_become_block_list_steps() {
        let json = serde_json::to_string(&render(&sample_plugin(), &ArchitectConfig::default())).expect("json");
        assert!(json.contains("Reception"));
    }

    #[test]
    fn an_empty_register_renders_the_placeholder() {
        let cfg = ArchitectConfig { active_register: "benchmarks".into(), ..ArchitectConfig::default() };
        let json = serde_json::to_string(&render(&sample_plugin(), &cfg)).expect("json");
        assert!(json.contains("No entities in register 'benchmarks'"));
    }
}
//#endregion 🧪️Tests
