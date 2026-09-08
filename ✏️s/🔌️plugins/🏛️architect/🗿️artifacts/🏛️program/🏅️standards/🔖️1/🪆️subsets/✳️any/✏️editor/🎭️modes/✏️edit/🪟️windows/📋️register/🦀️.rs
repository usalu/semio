//! 📋️ Architect register window — the active register's rows as a block-list surface.

use crate::artifacts::program::ProgramSnapshot;
use crate::editor::architect::catalog::register_entities;
use crate::editor::architect::chrome::{entity_id_from_json, entity_name_from_json};
use crate::editor::architect::config::{active_register, ArchitectConfig};
use semio_framework_plugin::{BlockListScene, Label, LocalizedLabel, SurfaceKind, WindowKindDefinition, WindowOptions};

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
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
struct RegisterBlockStep {
    id: String,
    title: String,
    blocks: Vec<RegisterBlockItem>,
}

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
struct RegisterBlockItem {
    id: String,
    label: String,
    kind: String,
}

pub fn render(program: &ProgramSnapshot, cfg: &ArchitectConfig) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let register = active_register(cfg);
    let entities = register_entities(program, register);
    if entities.is_empty() {
        return semio_framework_plugin::built_text_node(Label::data(format!("No entities in register '{register}'."))).map_err(|_| crate::editor::architect::ui_capacity_error());
    }

    let steps: Vec<RegisterBlockStep> = entities
        .iter()
        .filter_map(|entity| {
            let id = entity_id_from_json(entity)?;
            let name = entity_name_from_json(entity);
            Some(RegisterBlockStep { id: id.clone(), title: name.clone(), blocks: vec![RegisterBlockItem { id: format!("{id}-block"), label: name, kind: register.into() }] })
        })
        .collect();
    let steps_json = dsl::json::to_json_string(&steps);
    let palette_json = dsl::json::to_json_string(&vec![dsl::DslValue::object([
        ("blockKind".to_string(), dsl::DslValue::String(register.to_string())),
        ("label".to_string(), dsl::DslValue::String(register.to_string())),
        ("iconId".to_string(), dsl::DslValue::String("square".to_string())),
    ])]);
    // 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `ArtifactEditor::render` carries no
    // `InteractionView` and `BlockListScene` has no `interaction_domain` field for the wrapper to
    // stamp post-render either (unlike `UiNode::Tree`) — `selected_id` is left at `None`, matching
    // `dag`'s/`space`'s identical `NodeGraphScene` gap.
    let scene = BlockListScene { steps_json, palette_json, selected_id: None, dragging_id: None, domain_id: None };
    semio_framework_plugin::scene_surface(ARCHITECT_BODY_REGISTER, semio_framework_ui_contract::SurfaceKind::BlockList, &scene)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::program::sample_plugin;

    #[semio_framework_async_macros::async_test]
    async fn definition_declares_the_block_list_surface_and_body_key() {
        let definition = definition();
        assert_eq!(definition.body_key, ARCHITECT_BODY_REGISTER);
        assert!(matches!(definition.surface_kind, SurfaceKind::BlockList));
    }

    #[semio_framework_async_macros::async_test]
    async fn the_active_registers_rows_become_block_list_steps() {
        let node = render(&sample_plugin(), &ArchitectConfig::default()).expect("register");
        let semio_framework_plugin::Component::Surface(props) = &node.component else { panic!("register surface") };
        let scene: BlockListScene = semio_framework_ui_scene::decode(props).expect("packed register");
        let steps: serde_json::Value = serde_json::from_str(&scene.steps_json).expect("independent step oracle");
        assert!(steps.to_string().contains("Reception"));
        crate::editor::architect::testkit::project_render(Ok(node));
    }

    #[semio_framework_async_macros::async_test]
    async fn an_empty_register_renders_the_placeholder() {
        let cfg = ArchitectConfig { active_register: "benchmarks".into(), ..ArchitectConfig::default() };
        let json = crate::editor::architect::testkit::project_render(render(&sample_plugin(), &cfg));
        assert!(json.contains("No entities in register 'benchmarks'"));
    }
}
//#endregion 🧪️Tests
