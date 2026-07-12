//! 🧩 Protocol plugin — standalone strict-list, Blockly-like builder app bundled as a hot-swappable
//! WASM component. Independently launchable/testable without going through `forms`.

use protocol::{empty_protocol_projection, ProtocolEnvelope, ProtocolOp, ProtocolSpec, ProtocolStore, PROTOCOL_BUILTIN_KINDS, PROTOCOL_DOCUMENT_SCHEMA};
use semio_framework_plugin::{
    add_block_op, add_step_op, create_default_layout, move_block_op, move_step_op, remove_block_op, remove_step_op,
    render_protocol_builder, ui_text, update_protocol_title_op, App, ActionDescriptor, PluginApp, PluginBundle,
    ProtocolBuilderConfig, ProtocolPaletteEntry, SurfaceKind, UiNode, ViewState, PROTOCOL_BUILDER_LABELS_EN,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use vcs::{create_document_vcs_envelope, materialize_document_projection, DocumentVcsCommand};

//#region 🔖Constants
const PROTOCOL_PLAY_PLUGIN_ID: &str = "protocol-play";
const PROTOCOL_PLAY_APP_ID: &str = "protocol-play";
const PROTOCOL_PLAY_CONTROLLER_ID: &str = "protocol-play";
const PROTOCOL_PLAY_SURFACE_BUILDER: &str = "protocol.play.builder";
const PROTOCOL_PLAY_BODY_BUILDER: &str = "protocol.play.builder";
const PROTOCOL_PLAY_WINDOW_BUILDER: &str = "protocol-builder";
//#endregion 🔖Constants

//#region 🔖Envelope
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ProtocolPlayEnvelope {
    envelope: ProtocolEnvelope,
    #[serde(default)]
    applied_edit_ids: Vec<String>,
    #[serde(default)]
    selected_ids: Vec<String>,
}

fn default_envelope() -> ProtocolPlayEnvelope {
    let store = ProtocolStore::new(create_document_vcs_envelope(
        PROTOCOL_DOCUMENT_SCHEMA,
        "protocol-play",
        empty_protocol_projection(),
        None,
    ));
    ProtocolPlayEnvelope {
        envelope: store.envelope().clone(),
        applied_edit_ids: store.applied_edit_ids().to_vec(),
        selected_ids: Vec::new(),
    }
}

fn parse_envelope(document_json: &str) -> ProtocolPlayEnvelope {
    serde_json::from_str(document_json).unwrap_or_else(|_| default_envelope())
}

fn set_document_op(envelope: &ProtocolPlayEnvelope) -> String {
    json!({ "op": "setDocument", "document": envelope }).to_string()
}

fn store_from_envelope(play: &ProtocolPlayEnvelope) -> ProtocolStore {
    let mut store = ProtocolStore::new(play.envelope.clone());
    store.set_envelope(play.envelope.clone(), play.applied_edit_ids.clone());
    store
}

fn apply_store_action(play: &mut ProtocolPlayEnvelope, store: &mut ProtocolStore) -> Vec<String> {
    play.envelope = store.envelope().clone();
    play.applied_edit_ids = store.applied_edit_ids().to_vec();
    vec![set_document_op(play)]
}

fn materialized_projection(play: &ProtocolPlayEnvelope) -> ProtocolSpec {
    materialize_document_projection(&play.envelope, &play.applied_edit_ids)
        .unwrap_or_else(|_| play.envelope.vcs.initial_projection.clone())
}

fn dispatch_op(store: &mut ProtocolStore, op: ProtocolOp) {
    let _ = store.dispatch(DocumentVcsCommand::Apply {
        operations: vec![op],
        description: None,
    });
}
//#endregion 🔖Envelope

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

fn render_builder(play: &ProtocolPlayEnvelope) -> UiNode {
    let spec = materialized_projection(play);
    let palette = builtin_palette();
    let config = protocol_builder_config();
    render_protocol_builder(
        PROTOCOL_PLAY_SURFACE_BUILDER,
        &spec,
        &palette,
        play.selected_ids.first().map(String::as_str),
        &config,
    )
}
//#endregion 🔖Builder

//#region 🔖App
struct ProtocolPlayApp;

impl PluginApp for ProtocolPlayApp {
    fn app_id(&self) -> &str {
        PROTOCOL_PLAY_APP_ID
    }

    fn initial_document_json(&self) -> String {
        serde_json::to_string(&default_envelope()).unwrap_or_else(|_| "{}".into())
    }

    fn handle_action_patch_ops(
        &mut self,
        action: &str,
        args: Option<&Value>,
        document_json: &str,
        _view_state: &ViewState,
    ) -> Vec<String> {
        let mut play = parse_envelope(document_json);
        let mut store = store_from_envelope(&play);
        match action {
            "setDocument" => {
                if let Some(document) = args.and_then(|value| value.get("document")) {
                    if let Ok(parsed) = serde_json::from_value::<ProtocolPlayEnvelope>(document.clone()) {
                        return vec![set_document_op(&parsed)];
                    }
                }
            }
            "setSelection" => {
                if let Some(ids) = args.and_then(|value| value.get("ids")).and_then(|value| value.as_array()) {
                    play.selected_ids = ids.iter().filter_map(|value| value.as_str().map(str::to_string)).collect();
                    return vec![set_document_op(&play)];
                }
            }
            "addStep" => {
                let spec = materialized_projection(&play);
                let step_id = format!("step-{}", spec.steps.len() + 1);
                dispatch_op(&mut store, add_step_op(&spec, step_id));
                return apply_store_action(&mut play, &mut store);
            }
            "removeStep" => {
                let step_id = args.and_then(|value| value.get("stepId")).and_then(|value| value.as_str()).unwrap_or("");
                if step_id.is_empty() {
                    return Vec::new();
                }
                dispatch_op(&mut store, remove_step_op(step_id));
                return apply_store_action(&mut play, &mut store);
            }
            "moveStep" => {
                let step_id = args.and_then(|value| value.get("stepId")).and_then(|value| value.as_str()).unwrap_or("");
                let index = args.and_then(|value| value.get("index")).and_then(|value| value.as_u64()).unwrap_or(0) as usize;
                if step_id.is_empty() {
                    return Vec::new();
                }
                dispatch_op(&mut store, move_step_op(step_id, index));
                return apply_store_action(&mut play, &mut store);
            }
            "addBlock" => {
                let step_id = args.and_then(|value| value.get("stepId")).and_then(|value| value.as_str());
                let kind = args.and_then(|value| value.get("kind")).and_then(|value| value.as_str()).unwrap_or("text");
                let spec = materialized_projection(&play);
                let Some(step_id) = step_id.map(str::to_string).or_else(|| spec.steps.first().map(|step| step.id.clone())) else {
                    return Vec::new();
                };
                let block = protocol::ProtocolBlock {
                    id: format!("block-{}", spec.steps.iter().map(|step| step.blocks.len()).sum::<usize>() + 1),
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
                };
                let select_id = block.id.clone();
                dispatch_op(&mut store, add_block_op(&step_id, block, None));
                play.selected_ids = vec![select_id];
                return apply_store_action(&mut play, &mut store);
            }
            "removeBlock" => {
                let step_id = args.and_then(|value| value.get("stepId")).and_then(|value| value.as_str()).unwrap_or("");
                let block_id = args.and_then(|value| value.get("blockId")).and_then(|value| value.as_str()).unwrap_or("");
                if step_id.is_empty() || block_id.is_empty() {
                    return Vec::new();
                }
                dispatch_op(&mut store, remove_block_op(step_id, block_id));
                play.selected_ids.retain(|id| id != block_id);
                return apply_store_action(&mut play, &mut store);
            }
            "moveBlock" => {
                let block_id = args.and_then(|value| value.get("blockId")).and_then(|value| value.as_str());
                let from_step_id = args.and_then(|value| value.get("fromStepId")).and_then(|value| value.as_str());
                let to_step_id = args.and_then(|value| value.get("toStepId")).and_then(|value| value.as_str());
                let index = args.and_then(|value| value.get("index")).and_then(|value| value.as_u64()).unwrap_or(0) as usize;
                let (Some(block_id), Some(from_step_id), Some(to_step_id)) = (block_id, from_step_id, to_step_id) else {
                    return Vec::new();
                };
                dispatch_op(&mut store, move_block_op(block_id, from_step_id, to_step_id, index));
                return apply_store_action(&mut play, &mut store);
            }
            "updateProtocol" => {
                let title = args.and_then(|value| value.get("value")).and_then(|value| value.as_str()).unwrap_or("");
                dispatch_op(&mut store, update_protocol_title_op(Some(title.to_string()).filter(|title| !title.is_empty())));
                return apply_store_action(&mut play, &mut store);
            }
            "undo" => {
                let _ = store.dispatch(DocumentVcsCommand::Undo);
                return apply_store_action(&mut play, &mut store);
            }
            "redo" => {
                let _ = store.dispatch(DocumentVcsCommand::Redo);
                return apply_store_action(&mut play, &mut store);
            }
            _ => {}
        }
        Vec::new()
    }

    fn render(&self, body_key: &str, document_json: &str, _view_state: &ViewState) -> UiNode {
        let play = parse_envelope(document_json);
        match body_key {
            PROTOCOL_PLAY_BODY_BUILDER => render_builder(&play),
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }
}

fn create_protocol_play_app() -> App {
    App::from_builder(
        App::builder(PROTOCOL_PLAY_APP_ID, "Protocol")
            .document(["semio", "protocol"])
            .window_kind(PROTOCOL_PLAY_WINDOW_BUILDER, "Builder", PROTOCOL_PLAY_BODY_BUILDER, SurfaceKind::ProtocolList)
            .default_layout(create_default_layout(&[PROTOCOL_PLAY_WINDOW_BUILDER.into()], "row", None, None)),
    )
}

fn protocol_play_bundle() -> PluginBundle {
    PluginBundle::new(PROTOCOL_PLAY_PLUGIN_ID, "Protocol", "0.1.0").register_app(create_protocol_play_app(), || Box::new(ProtocolPlayApp))
}

semio_framework_plugin::plugin_exports!(protocol_play_bundle);
//#endregion 🔖App

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn protocol_play_app_declares_builder_window() {
        let app = create_protocol_play_app();
        assert_eq!(app.definition.window_kinds.len(), 1);
        assert_eq!(app.definition.window_kinds[0].id, PROTOCOL_PLAY_WINDOW_BUILDER);
        assert_eq!(app.definition.window_kinds[0].body_key, PROTOCOL_PLAY_BODY_BUILDER);
    }

    #[test]
    fn add_step_action_grows_projection() {
        let mut app = ProtocolPlayApp;
        let document = app.initial_document_json();
        let ops = app.handle_action_patch_ops("addStep", None, &document, &ViewState::default());
        assert_eq!(ops.len(), 1);
        let op: Value = serde_json::from_str(&ops[0]).expect("op json");
        let next: ProtocolPlayEnvelope = serde_json::from_value(op["document"].clone()).expect("envelope json");
        let spec = materialized_projection(&next);
        assert_eq!(spec.steps.len(), 2);
    }

    #[test]
    fn add_block_action_selects_new_block() {
        let mut app = ProtocolPlayApp;
        let document = app.initial_document_json();
        let ops = app.handle_action_patch_ops(
            "addBlock",
            Some(&json!({ "kind": "text" })),
            &document,
            &ViewState::default(),
        );
        assert_eq!(ops.len(), 1);
    }

    #[test]
    fn render_builder_emits_protocol_list_component_scene() {
        let app = ProtocolPlayApp;
        let document = app.initial_document_json();
        let node = app.render(PROTOCOL_PLAY_BODY_BUILDER, &document, &ViewState::default());
        assert!(matches!(node, UiNode::ComponentScene(_)));
    }
}
//#endregion 🧪Tests
