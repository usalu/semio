//! 🧩️ Playbook-play app — `DocumentApp` impl, render, manifest (constitutional: ui).

use playbook::{empty_playbook_projection, PlaybookSpec, PLAYBOOK_BUILTIN_KINDS, PLAYBOOK_DOCUMENT_SCHEMA};
use playbook_engine::default_block;
use playbook_kernel::{render_playbook_builder, PlaybookBuilderConfig, PLAYBOOK_BUILDER_LABELS_EN};
use playbook_op::{
    add_block_operation, add_step_operation, move_block_operation, move_step_operation, remove_block_operation, remove_step_operation,
    update_playbook_title_operation, PlaybookOperation,
};
use semio_framework_plugin::{
    app_labels, create_default_layout, is_de_locale, localized_label_map, resolve_labels, ui_text, ActionArgDef, ActionArgOption, ActionEmit,
    App, AppLabelsOverlay, AppLabelsOverlayExt, BlockPaletteEntry, DocumentApp, DocumentView, SurfaceKind, UiNode, ViewState,
};
use serde_json::Value;

//#region 🔖️Constants
const PLAYBOOK_PLAY_APP_ID: &str = "playbook-play";
const PLAYBOOK_PLAY_CONTROLLER_ID: &str = "playbook-play";
const PLAYBOOK_PLAY_SURFACE_BUILDER: &str = "playbook.play.builder";
const PLAYBOOK_PLAY_BODY_BUILDER: &str = "playbook.play.builder";
const PLAYBOOK_PLAY_WINDOW_BUILDER: &str = "playbook-builder";
//#endregion 🔖️Constants

//#region 🔖️Terminology
/// 🗣️ Complete UI label set for the playbook-play app; one field per label makes every locale combination compile-checked.
app_labels! {
    struct PlaybookPlayLabels {
        window_builder: &'static str = en: "Builder", de: "Builder";
        mode_builder: &'static str = en: "Builder", de: "Builder";
        kind_arg: &'static str = en: "Kind", de: "Art";
    }
}
//#endregion 🔖️Terminology

//#region 🔖️CommandLabels
/// 🗣️ (action id) -> localized label for every operation/view-action declared in `create_playbook_play_app`'s
/// static manifest — the manifest itself has no `view_state`/locale parameter, so this overlay is how the command
/// palette and Actions rail get a translated label without threading locale through the whole builder chain.
fn playbook_play_action_labels(is_de: bool) -> std::collections::HashMap<String, String> {
    localized_label_map(is_de, &[
        ("addStep", "Add Step", "Schritt hinzufügen"),
        ("removeStep", "Remove Step", "Schritt entfernen"),
        ("moveStep", "Move Step", "Schritt verschieben"),
        ("addBlock", "Add Block", "Baustein hinzufügen"),
        ("removeBlock", "Remove Block", "Baustein entfernen"),
        ("moveBlock", "Move Block", "Baustein verschieben"),
        ("updatePlaybook", "Update Playbook", "Playbook aktualisieren"),
        ("setSelection", "Set Selection", "Auswahl festlegen"),
    ])
}
//#endregion 🔖️CommandLabels

//#region 🔖️Render
fn playbook_builder_config() -> PlaybookBuilderConfig {
    PlaybookBuilderConfig {
        action_namespace: "playbook-builder",
        controller_id: PLAYBOOK_PLAY_CONTROLLER_ID,
        labels: PLAYBOOK_BUILDER_LABELS_EN,
    }
}

fn builtin_palette() -> Vec<BlockPaletteEntry> {
    PLAYBOOK_BUILTIN_KINDS
        .iter()
        .map(|kind| BlockPaletteEntry {
            block_kind: (*kind).into(),
            label: (*kind).into(),
            icon_id: "circle".into(),
        })
        .collect()
}

fn render_builder(spec: &PlaybookSpec, selected_id: Option<&str>) -> UiNode {
    render_playbook_builder(
        PLAYBOOK_PLAY_SURFACE_BUILDER,
        spec,
        &builtin_palette(),
        selected_id,
        &playbook_builder_config(),
    )
}
//#endregion 🔖️Render

//#region 🔖️PlaybookPlayApp
use std::cell::RefCell;

/// 🎛️ Ephemeral view state (the current block/step selection) — lives in the app struct, never in the
/// document, so selecting an element never pollutes undo history.
pub struct PlaybookPlayApp {
    selected_ids: RefCell<Vec<String>>,
}

impl Default for PlaybookPlayApp {
    fn default() -> Self {
        Self { selected_ids: RefCell::new(Vec::new()) }
    }
}

impl DocumentApp for PlaybookPlayApp {
    type Projection = PlaybookSpec;
    type Operation = PlaybookOperation;
        type Config = semio_framework_plugin::NoConfig;
        type ConfigOperation = semio_framework_plugin::NoConfigOperation;

    fn app_id(&self) -> &str {
        PLAYBOOK_PLAY_APP_ID
    }

    fn document_schema(&self) -> &str {
        PLAYBOOK_DOCUMENT_SCHEMA
    }

    fn initial_projection(&self) -> PlaybookSpec {
        empty_playbook_projection()
    }

    fn handle_action(
        &self,
        action: &str,
        args: Option<&Value>,
        doc: &DocumentView<'_, PlaybookSpec>,
        _cfg: &semio_framework_plugin::ConfigView<'_, semio_framework_plugin::NoConfig>,
        _view_state: &ViewState,
    ) -> ActionEmit<PlaybookOperation> {
        let spec = doc.projection;
        match action {
            "setSelection" => {
                if let Some(ids) = args.and_then(|value| value.get("ids")).and_then(|value| value.as_array()) {
                    *self.selected_ids.borrow_mut() = ids.iter().filter_map(|value| value.as_str().map(str::to_string)).collect();
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
                *self.selected_ids.borrow_mut() = vec![block_id.clone()];
                ActionEmit::operations(vec![add_block_operation(&step_id, default_block(block_id, kind), None)])
            }
            "removeBlock" => {
                let step_id = args.and_then(|value| value.get("stepId")).and_then(|value| value.as_str()).unwrap_or("");
                let block_id = args.and_then(|value| value.get("blockId")).and_then(|value| value.as_str()).unwrap_or("");
                if step_id.is_empty() || block_id.is_empty() {
                    return ActionEmit::default();
                }
                self.selected_ids.borrow_mut().retain(|id| id != block_id);
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
            "updatePlaybook" => {
                let title = args.and_then(|value| value.get("value")).and_then(|value| value.as_str()).unwrap_or("");
                ActionEmit::amend(
                    vec![update_playbook_title_operation(Some(title.to_string()).filter(|title| !title.is_empty()))],
                    "playbook.title",
                )
            }
            _ => ActionEmit::default(),
        }
    }

    fn render(&self, body_key: &str, doc: &DocumentView<'_, PlaybookSpec>, _cfg: &semio_framework_plugin::ConfigView<'_, semio_framework_plugin::NoConfig>, _view_state: &ViewState) -> UiNode {
        match body_key {
            PLAYBOOK_PLAY_BODY_BUILDER => render_builder(doc.projection, self.selected_ids.borrow().first().map(String::as_str)),
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }

    fn app_labels(&self, view_state: &ViewState) -> AppLabelsOverlay {
        let labels = resolve_labels::<PlaybookPlayLabels>(view_state);
        let is_de = is_de_locale(view_state);
        AppLabelsOverlay::default()
            .window_kind_label(PLAYBOOK_PLAY_WINDOW_BUILDER, labels.window_builder)
            .mode_label("builder", labels.mode_builder)
            .action_labels(playbook_play_action_labels(is_de))
            .action_arg_label("addBlock.kind", labels.kind_arg)
    }
}
//#endregion 🔖️PlaybookPlayApp

//#region 🔖️Manifest
pub fn create_playbook_play_app() -> App {
    App::from_builder(
        App::builder(PLAYBOOK_PLAY_APP_ID, "Playbook")
            .document(["semio", "playbook"])
            .mode("builder", "Builder")
            .default_mode_id("builder")
            .window_kind(PLAYBOOK_PLAY_WINDOW_BUILDER, "Builder", PLAYBOOK_PLAY_BODY_BUILDER, SurfaceKind::BlockList, "clipboard-list")
            .default_layout(create_default_layout(&[PLAYBOOK_PLAY_WINDOW_BUILDER.into()], "row", None, None))
            .operation("addStep", "Add Step")
            .operation("removeStep", "Remove Step")
            .operation("moveStep", "Move Step")
            .operation("addBlock", "Add Block")
            .operation("removeBlock", "Remove Block")
            .operation("moveBlock", "Move Block")
            .operation("updatePlaybook", "Update Playbook")
            .view_action("setSelection", "Set Selection")
            // 📝️ Staged argument form for the panel-visible create action (block kind is a choice).
            .action_args("addBlock", vec![
                ActionArgDef::select(
                    "kind",
                    "Kind",
                    PLAYBOOK_BUILTIN_KINDS.iter().map(|kind| ActionArgOption::new(*kind, *kind)).collect(),
                )
                .default_value("text"),
            ]),
    )
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_plugin::{testkit, PluginApp};
    use serde_json::json;

    #[test]
    fn add_block_materializes_declared_kind_default() {
        let mut app = testkit::new_app_with_registry::<PlaybookPlayApp>(create_playbook_play_app);
        app.handle_action("addStep", None, &ViewState::default(), &testkit::meta("local")).expect("add step");
        // addBlock fired with no args: the declared `kind` default ("text") must be materialized host-side.
        app.handle_action("addBlock", None, &ViewState::default(), &testkit::meta("local")).expect("add block");
        let projection = app.projection().expect("materialize projection");
        assert_eq!(projection.steps[0].blocks.last().unwrap().kind, "text", "kind default materialized from the registry");
    }

    #[test]
    fn playbook_play_app_declares_builder_window() {
        let app = create_playbook_play_app();
        assert_eq!(app.definition.window_kinds.len(), 1);
        assert_eq!(app.definition.window_kinds[0].id, PLAYBOOK_PLAY_WINDOW_BUILDER);
        assert_eq!(app.definition.window_kinds[0].body_key, PLAYBOOK_PLAY_BODY_BUILDER);
    }

    #[test]
    fn add_step_action_grows_projection() {
        let mut app = testkit::new_app::<PlaybookPlayApp>();
        app.handle_action("addStep", None, &ViewState::default(), &testkit::meta("local")).expect("add step");
        assert_eq!(app.projection().expect("materialize projection").steps.len(), 2);
    }

    #[test]
    fn add_block_action_appends_and_selects_block() {
        let mut app = testkit::new_app::<PlaybookPlayApp>();
        let result = app
            .handle_action("addBlock", Some(&json!({ "kind": "text" })), &ViewState::default(), &testkit::meta("local"))
            .expect("add block");
        assert_eq!(result.operations.len(), 1);
        let projection = app.projection().expect("materialize projection");
        assert_eq!(projection.steps[0].blocks.len(), 1);
        assert_eq!(projection.steps[0].blocks[0].kind, "text");
        let node = app.render(PLAYBOOK_PLAY_BODY_BUILDER, None, &ViewState::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains(&projection.steps[0].blocks[0].id));
    }

    #[test]
    fn set_selection_is_a_view_action_without_operations() {
        let mut app = testkit::new_app::<PlaybookPlayApp>();
        let result = app
            .handle_action("setSelection", Some(&json!({ "ids": ["block-1"] })), &ViewState::default(), &testkit::meta("local"))
            .expect("set selection");
        assert!(result.operations.is_empty(), "selection is ephemeral view state, not a document operation");
    }

    #[test]
    fn render_builder_emits_playbook_list_component_scene() {
        let mut app = testkit::new_app::<PlaybookPlayApp>();
        let node = app.render(PLAYBOOK_PLAY_BODY_BUILDER, None, &ViewState::default()).expect("render");
        assert!(matches!(node, UiNode::ComponentScene(_)));
    }

    #[test]
    fn undo_redo_round_trip_through_the_wrapper() {
        let mut app = testkit::new_app::<PlaybookPlayApp>();
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
    fn update_playbook_title_coalesces_into_one_undo_step() {
        let mut app = testkit::new_app::<PlaybookPlayApp>();
        for title in ["R", "Re", "Recipe"] {
            app.handle_action("updatePlaybook", Some(&json!({ "value": title })), &ViewState::default(), &testkit::meta("local")).expect("type title");
        }
        assert_eq!(app.projection().expect("materialize projection").title.as_deref(), Some("Recipe"));
        app.handle_action("undo", None, &ViewState::default(), &testkit::meta("local")).expect("undo");
        assert_eq!(app.projection().expect("materialize projection").title, None, "coalesced typing is one undo step");
    }

    /// 🧪️ The definitional proof: two independent instances start from the same document, apply
    /// DISJOINT edits (A adds a step, B adds a block to the pre-existing step), and exchanging operations over
    /// a backbone converges both sides onto the same projection — impossible under whole-document
    /// `setDocument` snapshots, where one side's write would clobber the other's.
    #[test]
    fn two_instances_converge_disjoint_edits_via_backbone() {
        testkit::assert_two_instances_converge::<PlaybookPlayApp, (usize, usize)>(
            "mem://playbook-convergence",
            ("addStep", None),
            ("addBlock", Some(&json!({ "kind": "number" }))),
            |app| {
                let projection = app.projection().expect("materialize projection");
                (projection.steps.len(), projection.steps[0].blocks.len())
            },
        );
    }
}
//#endregion 🧪️Tests
