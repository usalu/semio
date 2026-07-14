//! 🧩 Protocol plugin — standalone strict-list, Blockly-like builder app bundled as a hot-swappable
//! WASM component. Independently launchable/testable without going through `forms`.

use protocol::{
    empty_protocol_projection, ProtocolBlock, ProtocolOp, ProtocolSpec, PROTOCOL_BUILTIN_KINDS,
    PROTOCOL_DOCUMENT_SCHEMA,
};
use semio_framework_plugin::{
    add_block_op, add_step_op, create_default_layout, move_block_op, move_step_op, remove_block_op, remove_step_op,
    render_protocol_builder, ui_text, update_protocol_title_op, ActionArgDef, ActionArgOption, ActionEmit, App,
    DocumentApp, DocumentView, PluginBundle, ProtocolBuilderConfig, ProtocolPaletteEntry, SurfaceKind, UiNode,
    ViewState, PROTOCOL_BUILDER_LABELS_EN,
};
use serde_json::Value;

//#region 🔖Constants
const PROTOCOL_PLAY_PLUGIN_ID: &str = "protocol-play";
const PROTOCOL_PLAY_APP_ID: &str = "protocol-play";
const PROTOCOL_PLAY_CONTROLLER_ID: &str = "protocol-play";
const PROTOCOL_PLAY_SURFACE_BUILDER: &str = "protocol.play.builder";
const PROTOCOL_PLAY_BODY_BUILDER: &str = "protocol.play.builder";
const PROTOCOL_PLAY_WINDOW_BUILDER: &str = "protocol-builder";
//#endregion 🔖Constants

//#region 🔖Builder
fn protocol_builder_config() -> ProtocolBuilderConfig {
    ProtocolBuilderConfig {
        action_namespace: "protocol-builder",
        controller_id: PROTOCOL_PLAY_CONTROLLER_ID,
        labels: PROTOCOL_BUILDER_LABELS_EN,
    }
}

fn builtin_palette() -> Vec<ProtocolPaletteEntry> {
    PROTOCOL_BUILTIN_KINDS
        .iter()
        .map(|kind| ProtocolPaletteEntry {
            block_kind: (*kind).into(),
            label: (*kind).into(),
            icon_id: "circle".into(),
        })
        .collect()
}

/// 🧱 A blank block of the requested kind — every optional field defaulted, ready to be edited.
fn default_block(id: String, kind: &str) -> ProtocolBlock {
    ProtocolBlock {
        id,
        label: kind.into(),
        kind: kind.into(),
        description: None,
        required: None,
        placeholder: None,
        default: None,
        min: None,
        max: None,
        step: None,
        unit: None,
        text: None,
        options: None,
        fields: None,
        schema: None,
        src: None,
        accept: None,
        fixture_slug: None,
        params: None,
        condition: None,
    }
}

fn render_builder(spec: &ProtocolSpec, selected_id: Option<&str>) -> UiNode {
    render_protocol_builder(
        PROTOCOL_PLAY_SURFACE_BUILDER,
        spec,
        &builtin_palette(),
        selected_id,
        &protocol_builder_config(),
    )
}
//#endregion 🔖Builder

//#region 🔖App
/// 🎛️ Ephemeral view state (the current block/step selection) — lives in the app struct, never in the
/// document, so selecting an element never pollutes undo history.
#[derive(Default)]
struct ProtocolPlayApp {
    selected_ids: Vec<String>,
}

impl DocumentApp for ProtocolPlayApp {
    type Projection = ProtocolSpec;
    type Op = ProtocolOp;

    fn app_id(&self) -> &str {
        PROTOCOL_PLAY_APP_ID
    }

    fn document_schema(&self) -> &str {
        PROTOCOL_DOCUMENT_SCHEMA
    }

    fn initial_projection(&self) -> ProtocolSpec {
        empty_protocol_projection()
    }

    fn handle_action(
        &mut self,
        action: &str,
        args: Option<&Value>,
        doc: &DocumentView<'_, ProtocolSpec>,
        _view_state: &ViewState,
    ) -> ActionEmit<ProtocolOp> {
        let spec = doc.projection;
        match action {
            "setSelection" => {
                if let Some(ids) = args.and_then(|value| value.get("ids")).and_then(|value| value.as_array()) {
                    self.selected_ids = ids.iter().filter_map(|value| value.as_str().map(str::to_string)).collect();
                }
                ActionEmit::default()
            }
            "addStep" => {
                let step_id = format!("step-{}", spec.steps.len() + 1);
                ActionEmit::ops(vec![add_step_op(spec, step_id)])
            }
            "removeStep" => {
                let step_id = args.and_then(|value| value.get("stepId")).and_then(|value| value.as_str()).unwrap_or("");
                if step_id.is_empty() {
                    return ActionEmit::default();
                }
                ActionEmit::ops(vec![remove_step_op(step_id)])
            }
            "moveStep" => {
                let step_id = args.and_then(|value| value.get("stepId")).and_then(|value| value.as_str()).unwrap_or("");
                let index = args.and_then(|value| value.get("index")).and_then(|value| value.as_u64()).unwrap_or(0) as usize;
                if step_id.is_empty() {
                    return ActionEmit::default();
                }
                ActionEmit::ops(vec![move_step_op(step_id, index)])
            }
            "addBlock" => {
                let kind = args.and_then(|value| value.get("kind")).and_then(|value| value.as_str()).unwrap_or("text");
                let Some(step_id) = args
                    .and_then(|value| value.get("stepId"))
                    .and_then(|value| value.as_str())
                    .map(str::to_string)
                    .or_else(|| spec.steps.first().map(|step| step.id.clone()))
                else {
                    return ActionEmit::default();
                };
                let block_id = format!("block-{}", spec.steps.iter().map(|step| step.blocks.len()).sum::<usize>() + 1);
                self.selected_ids = vec![block_id.clone()];
                ActionEmit::ops(vec![add_block_op(&step_id, default_block(block_id, kind), None)])
            }
            "removeBlock" => {
                let step_id = args.and_then(|value| value.get("stepId")).and_then(|value| value.as_str()).unwrap_or("");
                let block_id = args.and_then(|value| value.get("blockId")).and_then(|value| value.as_str()).unwrap_or("");
                if step_id.is_empty() || block_id.is_empty() {
                    return ActionEmit::default();
                }
                self.selected_ids.retain(|id| id != block_id);
                ActionEmit::ops(vec![remove_block_op(step_id, block_id)])
            }
            "moveBlock" => {
                let block_id = args.and_then(|value| value.get("blockId")).and_then(|value| value.as_str());
                let from_step_id = args.and_then(|value| value.get("fromStepId")).and_then(|value| value.as_str());
                let to_step_id = args.and_then(|value| value.get("toStepId")).and_then(|value| value.as_str());
                let index = args.and_then(|value| value.get("index")).and_then(|value| value.as_u64()).unwrap_or(0) as usize;
                let (Some(block_id), Some(from_step_id), Some(to_step_id)) = (block_id, from_step_id, to_step_id) else {
                    return ActionEmit::default();
                };
                ActionEmit::ops(vec![move_block_op(block_id, from_step_id, to_step_id, index)])
            }
            "updateProtocol" => {
                let title = args.and_then(|value| value.get("value")).and_then(|value| value.as_str()).unwrap_or("");
                ActionEmit::amend(
                    vec![update_protocol_title_op(Some(title.to_string()).filter(|title| !title.is_empty()))],
                    "protocol.title",
                )
            }
            _ => ActionEmit::default(),
        }
    }

    fn render(&self, body_key: &str, doc: &DocumentView<'_, ProtocolSpec>, _view_state: &ViewState) -> UiNode {
        match body_key {
            PROTOCOL_PLAY_BODY_BUILDER => render_builder(doc.projection, self.selected_ids.first().map(String::as_str)),
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }
}

fn create_protocol_play_app() -> App {
    App::from_builder(
        App::builder(PROTOCOL_PLAY_APP_ID, "Protocol")
            .document(["semio", "protocol"])
            .mode("builder", "Builder")
            .default_mode_id("builder")
            .window_kind(PROTOCOL_PLAY_WINDOW_BUILDER, "Builder", PROTOCOL_PLAY_BODY_BUILDER, SurfaceKind::ProtocolList)
            .default_layout(create_default_layout(&[PROTOCOL_PLAY_WINDOW_BUILDER.into()], "row", None, None))
            .operation("addStep", "Add Step")
            .operation("removeStep", "Remove Step")
            .operation("moveStep", "Move Step")
            .operation("addBlock", "Add Block")
            .operation("removeBlock", "Remove Block")
            .operation("moveBlock", "Move Block")
            .operation("updateProtocol", "Update Protocol")
            .view_action("setSelection", "Set Selection")
            // 📝 Staged argument form for the panel-visible create action (block kind is a choice).
            .action_args("addBlock", vec![
                ActionArgDef::select(
                    "kind",
                    "Kind",
                    PROTOCOL_BUILTIN_KINDS.iter().map(|kind| ActionArgOption::new(*kind, *kind)).collect(),
                )
                .default_value("text"),
            ]),
    )
}

fn protocol_play_bundle() -> PluginBundle {
    PluginBundle::new(PROTOCOL_PLAY_PLUGIN_ID, "Protocol", "0.1.0")
        .register_document_app(create_protocol_play_app(), ProtocolPlayApp::default)
}

semio_framework_plugin::plugin_exports!(protocol_play_bundle);
//#endregion 🔖App

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_plugin::{ActionMeta, PluginApp, VcsDocumentApp};
    use serde_json::json;
    use vcs::{Backbone, MemoryBackbone};

    fn meta(actor: &str) -> ActionMeta {
        ActionMeta { actor: actor.into(), instance_id: 1 }
    }

    fn new_app() -> VcsDocumentApp<ProtocolPlayApp> {
        VcsDocumentApp::new(ProtocolPlayApp::default())
    }

    /// 🧬 A wrapper carrying the real action registry so `addBlock`'s declared `kind` default materializes.
    fn new_app_with_registry() -> VcsDocumentApp<ProtocolPlayApp> {
        use semio_framework_plugin::app::AppActionRegistry;
        let definition = create_protocol_play_app().definition;
        VcsDocumentApp::with_registry(ProtocolPlayApp::default(), AppActionRegistry::from_definition(&definition))
    }

    #[test]
    fn add_block_materializes_declared_kind_default() {
        let mut app = new_app_with_registry();
        app.handle_action("addStep", None, &ViewState::default(), &meta("local")).expect("add step");
        // addBlock fired with no args: the declared `kind` default ("text") must be materialized host-side.
        app.handle_action("addBlock", None, &ViewState::default(), &meta("local")).expect("add block");
        let projection = app.projection().expect("materialize projection");
        assert_eq!(projection.steps[0].blocks.last().unwrap().kind, "text", "kind default materialized from the registry");
    }

    #[test]
    fn protocol_play_app_declares_builder_window() {
        let app = create_protocol_play_app();
        assert_eq!(app.definition.window_kinds.len(), 1);
        assert_eq!(app.definition.window_kinds[0].id, PROTOCOL_PLAY_WINDOW_BUILDER);
        assert_eq!(app.definition.window_kinds[0].body_key, PROTOCOL_PLAY_BODY_BUILDER);
    }

    #[test]
    fn add_step_action_grows_projection() {
        let mut app = new_app();
        app.handle_action("addStep", None, &ViewState::default(), &meta("local")).expect("add step");
        assert_eq!(app.projection().expect("materialize projection").steps.len(), 2);
    }

    #[test]
    fn add_block_action_appends_and_selects_block() {
        let mut app = new_app();
        let result = app
            .handle_action("addBlock", Some(&json!({ "kind": "text" })), &ViewState::default(), &meta("local"))
            .expect("add block");
        assert_eq!(result.operations.len(), 1);
        let projection = app.projection().expect("materialize projection");
        assert_eq!(projection.steps[0].blocks.len(), 1);
        assert_eq!(projection.steps[0].blocks[0].kind, "text");
        let node = app.render(PROTOCOL_PLAY_BODY_BUILDER, None, &ViewState::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains(&projection.steps[0].blocks[0].id));
    }

    #[test]
    fn set_selection_is_a_view_action_without_ops() {
        let mut app = new_app();
        let result = app
            .handle_action("setSelection", Some(&json!({ "ids": ["block-1"] })), &ViewState::default(), &meta("local"))
            .expect("set selection");
        assert!(result.operations.is_empty(), "selection is ephemeral view state, not a document op");
    }

    #[test]
    fn render_builder_emits_protocol_list_component_scene() {
        let mut app = new_app();
        let node = app.render(PROTOCOL_PLAY_BODY_BUILDER, None, &ViewState::default()).expect("render");
        assert!(matches!(node, UiNode::ComponentScene(_)));
    }

    #[test]
    fn undo_redo_round_trip_through_the_wrapper() {
        let mut app = new_app();
        app.handle_action("addStep", None, &ViewState::default(), &meta("local")).expect("add step");
        assert_eq!(app.projection().expect("materialize projection").steps.len(), 2);
        app.handle_action("undo", None, &ViewState::default(), &meta("local")).expect("undo");
        assert_eq!(app.projection().expect("materialize projection").steps.len(), 1);
        app.handle_action("redo", None, &ViewState::default(), &meta("local")).expect("redo");
        assert_eq!(app.projection().expect("materialize projection").steps.len(), 2);
    }

    #[test]
    fn update_protocol_title_coalesces_into_one_undo_step() {
        let mut app = new_app();
        for title in ["R", "Re", "Recipe"] {
            app.handle_action("updateProtocol", Some(&json!({ "value": title })), &ViewState::default(), &meta("local")).expect("type title");
        }
        assert_eq!(app.projection().expect("materialize projection").title.as_deref(), Some("Recipe"));
        app.handle_action("undo", None, &ViewState::default(), &meta("local")).expect("undo");
        assert_eq!(app.projection().expect("materialize projection").title, None, "coalesced typing is one undo step");
    }

    /// 🧪 The definitional proof: two independent instances start from the same document, apply
    /// DISJOINT edits (A adds a step, B adds a block to the pre-existing step), and exchanging ops over
    /// a `MemoryBackbone` converges both sides to contain BOTH edits — impossible under whole-document
    /// `setDocument` snapshots, where one side's write would clobber the other's.
    #[test]
    fn two_instances_converge_disjoint_edits_via_backbone() {
        let mut instance_a = new_app();
        let mut instance_b = new_app();
        let (backbone_a, backbone_b) = MemoryBackbone::pair("mem://protocol-convergence", "mem://protocol-convergence");
        instance_a.attach_backbone(Box::new(backbone_a)).expect("attach a");
        instance_b.attach_backbone(Box::new(backbone_b)).expect("attach b");

        instance_a.handle_action("addStep", None, &ViewState::default(), &meta("actor-a")).expect("a adds a step");
        instance_b
            .handle_action("addBlock", Some(&json!({ "kind": "number" })), &ViewState::default(), &meta("actor-b"))
            .expect("b adds a block");

        // A neutral history action always pumps inbound ops without touching applied_edit_ids the way
        // undo would (ProtocolOp does not override Operation::author_id, so undo would misclassify the
        // just-received remote edit as local).
        instance_a.handle_action("commitCheckpoint", None, &ViewState::default(), &meta("actor-a")).expect("pump a");
        instance_b.handle_action("commitCheckpoint", None, &ViewState::default(), &meta("actor-b")).expect("pump b");

        let projection_a = instance_a.projection().expect("materialize projection");
        let projection_b = instance_b.projection().expect("materialize projection");

        assert_eq!(projection_a.steps.len(), 2, "instance A keeps its own added step");
        assert_eq!(projection_b.steps.len(), 2, "instance B converges on A's remote step");
        assert_eq!(projection_a.steps[0].blocks.len(), 1, "instance A converges on B's remote block");
        assert_eq!(projection_b.steps[0].blocks.len(), 1, "instance B keeps its own added block");
    }
}
//#endregion 🧪Tests
