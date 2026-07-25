//! 🧩 Protocol plugin — standalone strict-list, Blockly-like builder app bundled as a hot-swappable
//! WASM component. Independently launchable/testable without going through `forms`.

use protocol::{
    add_block_operation, add_step_operation, empty_protocol_projection, move_block_operation, move_step_operation, remove_block_operation,
    remove_step_operation, render_protocol_builder, update_protocol_title_operation, ProtocolBlock, ProtocolBuilderConfig,
    ProtocolOperation, ProtocolSpec, PROTOCOL_BUILDER_LABELS_EN, PROTOCOL_BUILTIN_KINDS, PROTOCOL_DOCUMENT_SCHEMA,
};
use semio_framework_plugin::{
    app_labels, create_default_layout, is_de_locale, localized_label_map, resolve_labels, ui_text, ActionArgDef,
    ActionArgOption, ActionEmit, App, AppLabelsOverlay, AppLabelsOverlayExt, BlockPaletteEntry, DocumentApp,
    DocumentView, PluginBundle, SurfaceKind, UiNode, ViewState,
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

//#region 🔖DocumentHelpers
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
//#endregion 🔖DocumentHelpers

//#region 🔖Terminology
/// 🗣️ Complete UI label set for the protocol-play app; one field per label makes every locale combination compile-checked.
app_labels! {
    struct ProtocolPlayLabels {
        window_builder: &'static str = en: "Builder", de: "Builder";
        mode_builder: &'static str = en: "Builder", de: "Builder";
        kind_arg: &'static str = en: "Kind", de: "Art";
    }
}
//#endregion 🔖Terminology

//#region 🔖CommandLabels
/// 🗣️ (action id) -> localized label for every operation/view-action declared in `create_protocol_play_app`'s
/// static manifest — the manifest itself has no `view_state`/locale parameter, so this overlay is how the command
/// palette and Actions rail get a translated label without threading locale through the whole builder chain.
fn protocol_play_action_labels(is_de: bool) -> std::collections::HashMap<String, String> {
    localized_label_map(is_de, &[
        ("addStep", "Add Step", "Schritt hinzufügen"),
        ("removeStep", "Remove Step", "Schritt entfernen"),
        ("moveStep", "Move Step", "Schritt verschieben"),
        ("addBlock", "Add Block", "Baustein hinzufügen"),
        ("removeBlock", "Remove Block", "Baustein entfernen"),
        ("moveBlock", "Move Block", "Baustein verschieben"),
        ("updateProtocol", "Update Protocol", "Protokoll aktualisieren"),
        ("setSelection", "Set Selection", "Auswahl festlegen"),
    ])
}
//#endregion 🔖CommandLabels

//#region 🔖Render
fn protocol_builder_config() -> ProtocolBuilderConfig {
    ProtocolBuilderConfig {
        action_namespace: "protocol-builder",
        controller_id: PROTOCOL_PLAY_CONTROLLER_ID,
        labels: PROTOCOL_BUILDER_LABELS_EN,
    }
}

fn builtin_palette() -> Vec<BlockPaletteEntry> {
    PROTOCOL_BUILTIN_KINDS
        .iter()
        .map(|kind| BlockPaletteEntry {
            block_kind: (*kind).into(),
            label: (*kind).into(),
            icon_id: "circle".into(),
        })
        .collect()
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
//#endregion 🔖Render

//#region 🔖ProtocolPlayApp
/// 🎛️ Ephemeral view state (the current block/step selection) — lives in the app struct, never in the
/// document, so selecting an element never pollutes undo history.
#[derive(Default)]
struct ProtocolPlayApp {
    selected_ids: Vec<String>,
}

impl DocumentApp for ProtocolPlayApp {
    type Projection = ProtocolSpec;
    type Operation = ProtocolOperation;

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
    ) -> ActionEmit<ProtocolOperation> {
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
                ActionEmit::operations(vec![add_step_operation(spec, step_id)])
            }
            "removeStep" => {
                let step_id = args.and_then(|value| value.get("stepId")).and_then(|value| value.as_str()).unwrap_or("");
                if step_id.is_empty() {
                    return ActionEmit::default();
                }
                ActionEmit::operations(vec![remove_step_operation(step_id)])
            }
            "moveStep" => {
                let step_id = args.and_then(|value| value.get("stepId")).and_then(|value| value.as_str()).unwrap_or("");
                let index = args.and_then(|value| value.get("index")).and_then(|value| value.as_u64()).unwrap_or(0) as usize;
                if step_id.is_empty() {
                    return ActionEmit::default();
                }
                ActionEmit::operations(vec![move_step_operation(step_id, index)])
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
                ActionEmit::operations(vec![add_block_operation(&step_id, default_block(block_id, kind), None)])
            }
            "removeBlock" => {
                let step_id = args.and_then(|value| value.get("stepId")).and_then(|value| value.as_str()).unwrap_or("");
                let block_id = args.and_then(|value| value.get("blockId")).and_then(|value| value.as_str()).unwrap_or("");
                if step_id.is_empty() || block_id.is_empty() {
                    return ActionEmit::default();
                }
                self.selected_ids.retain(|id| id != block_id);
                ActionEmit::operations(vec![remove_block_operation(step_id, block_id)])
            }
            "moveBlock" => {
                let block_id = args.and_then(|value| value.get("blockId")).and_then(|value| value.as_str());
                let from_step_id = args.and_then(|value| value.get("fromStepId")).and_then(|value| value.as_str());
                let to_step_id = args.and_then(|value| value.get("toStepId")).and_then(|value| value.as_str());
                let index = args.and_then(|value| value.get("index")).and_then(|value| value.as_u64()).unwrap_or(0) as usize;
                let (Some(block_id), Some(from_step_id), Some(to_step_id)) = (block_id, from_step_id, to_step_id) else {
                    return ActionEmit::default();
                };
                ActionEmit::operations(vec![move_block_operation(block_id, from_step_id, to_step_id, index)])
            }
            "updateProtocol" => {
                let title = args.and_then(|value| value.get("value")).and_then(|value| value.as_str()).unwrap_or("");
                ActionEmit::amend(
                    vec![update_protocol_title_operation(Some(title.to_string()).filter(|title| !title.is_empty()))],
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

    fn app_labels(&self, view_state: &ViewState) -> AppLabelsOverlay {
        let labels = resolve_labels::<ProtocolPlayLabels>(view_state);
        let is_de = is_de_locale(view_state);
        AppLabelsOverlay::default()
            .window_kind_label(PROTOCOL_PLAY_WINDOW_BUILDER, labels.window_builder)
            .mode_label("builder", labels.mode_builder)
            .action_labels(protocol_play_action_labels(is_de))
            .action_arg_label("addBlock.kind", labels.kind_arg)
    }
}
//#endregion 🔖ProtocolPlayApp

//#region 🔖Manifest
fn create_protocol_play_app() -> App {
    App::from_builder(
        App::builder(PROTOCOL_PLAY_APP_ID, "Protocol")
            .document(["semio", "protocol"])
            .mode("builder", "Builder")
            .default_mode_id("builder")
            .window_kind(PROTOCOL_PLAY_WINDOW_BUILDER, "Builder", PROTOCOL_PLAY_BODY_BUILDER, SurfaceKind::BlockList)
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
//#endregion 🔖Manifest

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_plugin::{testkit, PluginApp};
    use serde_json::json;

    #[test]
    fn add_block_materializes_declared_kind_default() {
        let mut app = testkit::new_app_with_registry::<ProtocolPlayApp>(create_protocol_play_app);
        app.handle_action("addStep", None, &ViewState::default(), &testkit::meta("local")).expect("add step");
        // addBlock fired with no args: the declared `kind` default ("text") must be materialized host-side.
        app.handle_action("addBlock", None, &ViewState::default(), &testkit::meta("local")).expect("add block");
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
        let mut app = testkit::new_app::<ProtocolPlayApp>();
        app.handle_action("addStep", None, &ViewState::default(), &testkit::meta("local")).expect("add step");
        assert_eq!(app.projection().expect("materialize projection").steps.len(), 2);
    }

    #[test]
    fn add_block_action_appends_and_selects_block() {
        let mut app = testkit::new_app::<ProtocolPlayApp>();
        let result = app
            .handle_action("addBlock", Some(&json!({ "kind": "text" })), &ViewState::default(), &testkit::meta("local"))
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
    fn set_selection_is_a_view_action_without_operations() {
        let mut app = testkit::new_app::<ProtocolPlayApp>();
        let result = app
            .handle_action("setSelection", Some(&json!({ "ids": ["block-1"] })), &ViewState::default(), &testkit::meta("local"))
            .expect("set selection");
        assert!(result.operations.is_empty(), "selection is ephemeral view state, not a document operation");
    }

    #[test]
    fn render_builder_emits_protocol_list_component_scene() {
        let mut app = testkit::new_app::<ProtocolPlayApp>();
        let node = app.render(PROTOCOL_PLAY_BODY_BUILDER, None, &ViewState::default()).expect("render");
        assert!(matches!(node, UiNode::ComponentScene(_)));
    }

    #[test]
    fn undo_redo_round_trip_through_the_wrapper() {
        let mut app = testkit::new_app::<ProtocolPlayApp>();
        testkit::assert_undo_redo_round_trip(
            &mut app,
            "addStep",
            None,
            |app| app.projection().expect("materialize projection").steps.len(),
            1,
            2,
        );
    }

    #[test]
    fn update_protocol_title_coalesces_into_one_undo_step() {
        let mut app = testkit::new_app::<ProtocolPlayApp>();
        for title in ["R", "Re", "Recipe"] {
            app.handle_action("updateProtocol", Some(&json!({ "value": title })), &ViewState::default(), &testkit::meta("local")).expect("type title");
        }
        assert_eq!(app.projection().expect("materialize projection").title.as_deref(), Some("Recipe"));
        app.handle_action("undo", None, &ViewState::default(), &testkit::meta("local")).expect("undo");
        assert_eq!(app.projection().expect("materialize projection").title, None, "coalesced typing is one undo step");
    }

    /// 🧪 The definitional proof: two independent instances start from the same document, apply
    /// DISJOINT edits (A adds a step, B adds a block to the pre-existing step), and exchanging operations over
    /// a backbone converges both sides onto the same projection — impossible under whole-document
    /// `setDocument` snapshots, where one side's write would clobber the other's.
    #[test]
    fn two_instances_converge_disjoint_edits_via_backbone() {
        testkit::assert_two_instances_converge::<ProtocolPlayApp, (usize, usize)>(
            "mem://protocol-convergence",
            ("addStep", None),
            ("addBlock", Some(&json!({ "kind": "number" }))),
            |app| {
                let projection = app.projection().expect("materialize projection");
                (projection.steps.len(), projection.steps[0].blocks.len())
            },
        );
    }
}
//#endregion 🧪Tests
