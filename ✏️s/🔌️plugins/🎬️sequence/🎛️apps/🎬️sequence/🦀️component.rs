//! 🖥️ Sequence play app — the `DocumentApp` impl (dispatch-only), the aggregated command enum and the
//! manifest stitch.
//!
//! Everything substantive lives in a taxonomy node: command bodies in `🎮️commands/*`, window renders in
//! `🎭️modes/✏️edit/🪟️windows/*`, panel trees in `📌️panels/*`, labels in `🦀️terminology.rs`, view state in
//! `🦀️config.rs`, shared compute in the artifact's `⚙️engine`. This file is a routing table: `handle` →
//! `SequenceCommand::dispatch`, `render` → body-key → node, and a `🔖️Manifest` region that calls one
//! `definition()` per node.

use crate::apps::sequence::commands::connection::{connect_steps, disconnect_steps};
use crate::apps::sequence::commands::layout::{reorganize, set_orientation};
use crate::apps::sequence::commands::locale::set_locale;
use crate::apps::sequence::commands::node_graph::{node_graph_edit, set_viewport};
use crate::apps::sequence::commands::playback::{run_command, stop_command};
use crate::apps::sequence::commands::selection::set_selection;
use crate::apps::sequence::commands::step::{add_step, add_step_dropped, add_step_to_slot, delete_selection, move_step, remove_step, set_step_collapsed, set_step_params};
use crate::apps::sequence::config::{SequenceConfig, SequenceConfigOperation};
use crate::apps::sequence::modes::edit;
use crate::apps::sequence::modes::edit::windows::{compiled, main, script};
use crate::apps::sequence::panels::{catalogue as catalogue_panel, document as document_panel, inspection as inspection_panel};
use crate::apps::sequence::terminology::sequence_play_labels;
use crate::artifacts::sequence::op::SequenceOperation;
use crate::artifacts::sequence::{SequenceFixture, StepParams, SEQUENCE_FIXTURE_SCHEMA};
use semio_framework_plugin::{NoDraft, NoDraftOperation, DraftView, 
    ActionArgDef, ActionArgOption, ActionDefinition, ActionDescriptor, ActionKind, App, AppActionRegistry, AppIo, ConfigFieldShape, ConfigFieldSpec, ConfigSpec, ConfigView, ContextMenuItemSpec, ContextMenuRequest, DocumentApp, DocumentView, DslValue, Emit,
    Fault, Label, LocalizedLabel, Media, MediaError, MediaPayload, UiNode,
};
use store::EngineHandles;
use serde_json::{json, Value};

//#region 🔖️Constants
pub const SEQUENCE_PLAY_APP_ID: &str = "sequence-play";
pub use catalogue_panel::SEQUENCE_PLAY_BODY_CATALOGUE;
pub use compiled::SEQUENCE_PLAY_BODY_COMPILED;
pub use document_panel::SEQUENCE_PLAY_BODY_DOCUMENT;
pub use inspection_panel::SEQUENCE_PLAY_BODY_INSPECTOR;
pub use main::SEQUENCE_PLAY_BODY_MAIN;
pub use script::SEQUENCE_PLAY_BODY_SCRIPT;

/// 🎯️ An `ActionDescriptor` addressed at this app — the single factory every taxonomy node's chrome
/// (`📌️panels/*`) builds its `on_change`/item actions with.
pub fn sequence_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    semio_framework_plugin::ActionFactory::new(SEQUENCE_PLAY_APP_ID).action(action, args)
}
//#endregion 🔖️Constants

//#region 🔖️Commands
semio_framework_plugin::app_commands! {
    /// 🎯️ `SequencePlayApp::Command` — the SOLE dispatch surface for sequence's own behavior,
    /// assembled from the `🎮️commands/*` payload modules. Each row states BOTH the manifest action id
    /// (`command_id()`, the camelCase id declared in `🔖️Manifest` below) and the `dsl` wire keyword
    /// (the kebab-case `#[dsl(key = ..)]` the codec uses) — every row's wire keyword happens to be the
    /// plain kebab-case of its id (no `flow`-style divergence here), but the two are still copied
    /// independently from the pre-migration `sequence_protocol` enum's `command_id()` match arm and
    /// `#[dsl(key = ..)]` attribute respectively, never derived one from the other. **Row order is the
    /// binary variant ordinal: appending is safe, reordering is a wire-format break.**
    pub enum SequenceCommand for SequenceFixture, SequenceOperation, SequenceConfig, SequenceConfigOperation {
        "addStep" as "add-step" => add_step::AddStep,
        "addStepToSlot" as "add-step-to-slot" => add_step_to_slot::AddStepToSlot,
        "addStepDropped" as "add-step-dropped" => add_step_dropped::AddStepDropped,
        "removeStep" as "remove-step" => remove_step::RemoveStep,
        "deleteSelection" as "delete-selection" => delete_selection::DeleteSelection,
        "moveStep" as "move-step" => move_step::MoveStep,
        "connectSteps" as "connect-steps" => connect_steps::ConnectSteps,
        "disconnectSteps" as "disconnect-steps" => disconnect_steps::DisconnectSteps,
        "setStepParams" as "set-step-params" => set_step_params::SetStepParams,
        "setStepCollapsed" as "set-step-collapsed" => set_step_collapsed::SetStepCollapsed,
        "reorganize" as "reorganize" => reorganize::Reorganize,
        "nodeGraphEdit" as "node-graph-edit" => node_graph_edit::NodeGraphEdit,
        "setSelection" as "set-selection" => set_selection::SetSelection,
        "setOrientation" as "set-orientation" => set_orientation::SetOrientation,
        "run" as "run" => run_command::Run,
        "stop" as "stop" => stop_command::Stop,
        "setViewport" as "set-viewport" => set_viewport::SetViewport,
        "setLocale" as "set-locale" => set_locale::SetLocale,
    }
}
//#endregion 🔖️Commands

//#region 🔖️SequencePlayApp
/// 🧪️ B1: unit struct — every former `SequencePlayRuntime` field now lives in
/// `crate::apps::sequence::config::SequenceConfig` (see `DocumentApp::Config`), written through
/// `SequenceConfigOperation`s.
#[derive(Default)]
pub struct SequencePlayApp;

impl DocumentApp for SequencePlayApp {
    type Projection = SequenceFixture;
    type Operation = SequenceOperation;
    type Config = SequenceConfig;
    type ConfigOperation = SequenceConfigOperation;
    type Draft = NoDraft;
    type DraftOperation = NoDraftOperation;

    type Command = SequenceCommand;

    const APP_ID: &'static str = SEQUENCE_PLAY_APP_ID;
    const DOCUMENT_SCHEMA: &'static str = SEQUENCE_FIXTURE_SCHEMA;

    fn initial_projection() -> SequenceFixture {
        crate::artifacts::sequence::default_fixture()
    }

    fn io() -> Option<AppIo> {
        Some(crate::artifacts::sequence::engine::sequence_io())
    }

    /// 🎞️ `steps:in` (Wave-2 port recipe): inserts incoming computation results as a new step at the
    /// far right of the flow — an object payload becomes that step's params verbatim, a bare
    /// scalar/array is wrapped under a single `"value"` key. Never mutates anything directly (matches
    /// every other `import_media` override): the caller (a headless runner or the UI) applies the
    /// returned `StepsAdd` through the ordinary, undoable document store.
    fn import_media(port: &str, media: &Media, doc: &DocumentView<'_, SequenceFixture>) -> Result<Emit<SequenceOperation, SequenceConfigOperation, Self::DraftOperation>, MediaError> {
        if port != "steps:in" {
            return Err(MediaError::NotImplemented);
        }
        let MediaPayload::Structured { json, .. } = &media.payload else {
            return Err(MediaError::Payload(port.to_string(), "steps:in importer only accepts a Structured (JSON) payload".into()));
        };
        let value: Value = serde_json::from_str(json).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
        let params_value = if value.is_object() { value } else { json!({ "value": value }) };
        let params: StepParams = serde_json::from_value(params_value).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
        let fixture = doc.projection;
        let id = crate::artifacts::sequence::engine::next_available_step_id(fixture);
        let x = fixture.steps.iter().map(|step| step.x).fold(0.0_f64, f64::max) + if fixture.steps.is_empty() { 0.0 } else { 280.0 };
        let step = crate::artifacts::sequence::SequenceStep { id, kind: "computation.import".into(), params, x, y: 0.0, slot: None, collapsed: false };
        Ok(Emit::operations(vec![SequenceOperation::StepsAdd { index: fixture.steps.len(), item: step }]))
    }

    /// 🏷️ The manifest action id each command was declared under — supplied wholesale by
    /// `app_commands!`'s generated `command_id()`.
    fn command_id(command: &SequenceCommand) -> &'static str {
        command.command_id()
    }

    fn handle(command: &SequenceCommand, doc: &DocumentView<'_, SequenceFixture>, cfg: &ConfigView<'_, SequenceConfig>, _draft: &DraftView<'_, Self::Draft>, _engines: &EngineHandles) -> Result<Emit<SequenceOperation, SequenceConfigOperation, Self::DraftOperation>, Fault> {
        command.dispatch(doc, cfg)
    }

    /// 🧮️ This app's typed configuration spec — the layout orientation `reorganize` reads.
    fn config_spec() -> ConfigSpec {
        ConfigSpec {
            fields: vec![ConfigFieldSpec { key: "orientation".into(), label: "Layout Orientation".into(), shape: ConfigFieldShape::Select { options: vec!["leftRight".into(), "topBottom".into()] }, default: Some(DslValue::String("leftRight".into())) }],
        }
    }

    fn render(body_key: &str, doc: &DocumentView<'_, SequenceFixture>, cfg: &ConfigView<'_, SequenceConfig>) -> UiNode {
        let fixture = doc.projection;
        let config = cfg.projection;
        let labels = sequence_play_labels(config);
        match body_key {
            SEQUENCE_PLAY_BODY_MAIN => main::render(fixture, config),
            SEQUENCE_PLAY_BODY_SCRIPT => script::render(fixture, config),
            SEQUENCE_PLAY_BODY_COMPILED => compiled::render(fixture),
            SEQUENCE_PLAY_BODY_DOCUMENT => document_panel::render(fixture, &config.selected_step_ids, labels),
            SEQUENCE_PLAY_BODY_CATALOGUE => catalogue_panel::render(fixture, labels),
            SEQUENCE_PLAY_BODY_INSPECTOR => inspection_panel::render(fixture, &config.selected_step_ids, labels),
            _ => semio_framework_plugin::ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
    }

    /// 🗂️ Grouped disclosure: `run`/`stop`/`addStep` stay top-level (the most frequent verbs);
    /// `reorganize` folds into the `transform` group and a single-node hit's `setStepCollapsed` folds
    /// into the `selection` group; `deleteSelection` stays a direct destructive item last —
    /// `organize_context_menu` (applied automatically at the `VcsDocumentApp::context_menu` funnel)
    /// sorts the groups into `RIBBON_PARENT_CATEGORIES` order and inserts the pre-destructive
    /// separator itself.
    fn context_menu(request: &ContextMenuRequest, _doc: &DocumentView<'_, SequenceFixture>, cfg: &ConfigView<'_, SequenceConfig>, registry: &AppActionRegistry) -> Vec<ContextMenuItemSpec> {
        use semio_framework_plugin::{node_graph_delete_selection_spec, selection_domains_from_surface, Menu, NodeGraphDeleteDispatch};

        let is_de = cfg.projection.locale.starts_with("de");
        let selected = cfg.projection.selected_step_ids.clone();
        let (nodes, edges) = selection_domains_from_surface(request.surface.as_ref(), &selected, &[]);

        let mut menu = Menu::of(registry).action("run").action("stop").action("addStep").group("transform", |m| m.action("reorganize"));

        if nodes.len() == 1 {
            let id = nodes[0].clone();
            menu = menu.group("selection", |m| {
                m.item(ContextMenuItemSpec {
                    id: "setStepCollapsed".into(),
                    label: Some(if is_de { "Schritt einklappen".into() } else { "Toggle Collapsed".into() }),
                    icon: Some("chevrons-up-down".into()),
                    action: Some("setStepCollapsed".into()),
                    args: semio_framework_plugin::optional_json_to_dsl(Some(json!({ "id": id }))),
                    ..Default::default()
                })
            });
        }

        if let Some(spec) = node_graph_delete_selection_spec("Delete selection", is_de, nodes.len(), edges.len(), NodeGraphDeleteDispatch::Direct) {
            menu = menu.item(spec);
        }
        menu.build()
    }
}
//#endregion 🔖️SequencePlayApp

//#region 🔖️Manifest
/// 🧱️ The manifest stitch: one call per taxonomy node, each sourced from that node's own
/// `definition()`. Only the leaf action/keybinding declarations (which have no dedicated `_def`
/// passthrough) are written out inline.
pub fn create_sequence_app() -> App {
    App::from_builder(
        App::builder(SEQUENCE_PLAY_APP_ID, LocalizedLabel::native("Sequence", "Sequenz"))
            .document(["semio", "sequence"])
            .artifact_kind(crate::artifacts::sequence::artifact_kind())
            .icon_id("sequence")
            .mode_def(edit::definition())
            .default_mode_id(edit::SEQUENCE_PLAY_MODE_EDIT)
            .window_kind_def(main::definition())
            .window_kind_def(script::definition())
            .window_kind_def(compiled::definition())
            .default_layout(edit::layout())
            .panel_tab_def(document_panel::definition())
            .panel_tab_def(catalogue_panel::definition())
            .panel_tab_def(inspection_panel::definition())
            // ✏️ Document-mutating actions — dispatched as VCS operations with true inverses.
            .action_with(ActionDefinition::new_catalog("addStep", LocalizedLabel::native("Add Step", "Schritt hinzufügen"), ActionKind::Operation).with_category("create"))
            .operation("addStepToSlot", LocalizedLabel::native("Add Step To Slot", "Schritt zu Slot hinzufügen"))
            .operation("addStepDropped", LocalizedLabel::native("Add Step Dropped", "Schritt per Ablegen hinzufügen"))
            .operation("removeStep", LocalizedLabel::native("Remove Step", "Schritt entfernen"))
            .action_with(ActionDefinition::new_catalog("deleteSelection", LocalizedLabel::native("Delete Selection", "Auswahl löschen"), ActionKind::Operation).with_category("selection"))
            .operation("moveStep", LocalizedLabel::native("Move Step", "Schritt verschieben"))
            .operation("connectSteps", LocalizedLabel::native("Connect Steps", "Schritte verbinden"))
            .operation("disconnectSteps", LocalizedLabel::native("Disconnect Steps", "Schritte trennen"))
            .operation("setStepParams", LocalizedLabel::native("Set Step Params", "Schrittparameter festlegen"))
            .action_with(ActionDefinition::new_catalog("setStepCollapsed", LocalizedLabel::native("Set Step Collapsed", "Schritt einklappen"), ActionKind::Operation).with_category("selection"))
            .action_with(ActionDefinition::new_catalog("reorganize", LocalizedLabel::native("Reorganize", "Neu anordnen"), ActionKind::Operation).with_category("transform"))
            .operation("nodeGraphEdit", LocalizedLabel::native("Node Graph Edit", "Knotengraph bearbeiten"))
            .view_action("setViewport", LocalizedLabel::native("Node Graph Viewport", "Knotengraph-Ansicht"))
            // 👁️ Ephemeral view state — selection, run output, layout orientation, locale.
            .view_action("setSelection", LocalizedLabel::native("Set Selection", "Auswahl festlegen"))
            .view_action("setOrientation", LocalizedLabel::native("Set Orientation", "Ausrichtung festlegen"))
            .action_with(ActionDefinition::new_catalog("run", LocalizedLabel::native("Run", "Ausführen"), ActionKind::View).with_category("actions"))
            .action_with(ActionDefinition::new_catalog("stop", LocalizedLabel::native("Stop", "Stopp"), ActionKind::View).with_category("actions"))
            .view_action("setLocale", LocalizedLabel::native("Set Locale", "Sprache festlegen"))
            // 📝️ Staged argument forms for the panel-visible create + layout actions.
            .action_args("addStep", vec![
                ActionArgDef::select("kind", LocalizedLabel::native("Kind", "Art"), vec![
                    ActionArgOption::new("state.set", LocalizedLabel::native("Set State", "Zustand setzen")),
                    ActionArgOption::new("log.print", LocalizedLabel::native("Print", "Ausgeben")),
                    ActionArgOption::new("control.if", LocalizedLabel::native("If", "Wenn")),
                    ActionArgOption::new("control.while", LocalizedLabel::native("While", "Solange")),
                    ActionArgOption::new("math.add", LocalizedLabel::native("Add", "Addieren")),
                ]).default_value("log.print"),
            ])
            .action_args("setOrientation", vec![
                ActionArgDef::select("orientation", LocalizedLabel::native("Orientation", "Ausrichtung"), vec![
                    ActionArgOption::new("leftRight", LocalizedLabel::native("Left to Right", "Links nach rechts")),
                    ActionArgOption::new("topBottom", LocalizedLabel::native("Top to Bottom", "Oben nach unten")),
                ]).required(),
            ])
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo")
            .config(SequencePlayApp::config_spec())
            .io(crate::artifacts::sequence::engine::sequence_io()),
    )
    .example("demo", LocalizedLabel::native("Demo", "Demo"), crate::artifacts::sequence::engine::sequence_example_json(), "cylinder")
    .workflow("sequence", "Sequence", "graph")
}
//#endregion 🔖️Manifest

//#region 🧪️Testkit
/// 🧪️ Shared test scaffolding for every taxonomy node's own `🧪️Tests` region — a component file must
/// be able to drive the whole app without re-deriving the harness.
#[cfg(test)]
pub(crate) mod testkit {
    use super::*;
    use semio_framework_plugin::testkit::{meta, new_app_with_registry};
    use semio_framework_plugin::{InvocationResult, PluginApp, VcsDocumentApp, ViewState};

    pub type SequenceApp = VcsDocumentApp<SequencePlayApp>;

    /// 🧪️ A bare app instance — no `AppActionRegistry`, so undeclared internal commands dispatch freely.
    pub fn new_app() -> SequenceApp {
        semio_framework_plugin::testkit::new_app::<SequencePlayApp>()
    }

    /// 🧪️ An app wired to the real manifest registry — enforces View/Shell kind discipline.
    pub fn new_app_with_registry_wired() -> SequenceApp {
        new_app_with_registry::<SequencePlayApp>(create_sequence_app)
    }

    pub fn dispatch(app: &mut SequenceApp, command: SequenceCommand) -> InvocationResult {
        app.dispatch_typed(command, &meta("local")).expect("dispatch")
    }

    pub fn render(app: &mut SequenceApp, body_key: &str) -> String {
        serde_json::to_string(&app.render(body_key, None, &ViewState::default()).expect("render")).expect("render json")
    }
}
//#endregion 🧪️Testkit

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::sequence::testkit::{dispatch, new_app, new_app_with_registry_wired};
    use semio_framework_plugin::{testkit::assert_undo_redo_round_trip, Locale, PluginApp, Terminology};

    #[test]
    fn default_fixture_has_steps() {
        assert_eq!(crate::artifacts::sequence::default_fixture().steps.len(), 2);
    }

    #[test]
    fn undo_redo_round_trip_through_the_wrapper() {
        let mut app = new_app();
        assert_undo_redo_round_trip(&mut app, SequenceCommand::AddStep(add_step::AddStep { kind: "log.print".into(), x: 0.0, y: 0.0 }), |app| app.projection().expect("projection").steps.len(), 2, 3);
    }

    /// 🧪️ The definitional regression proof: two independent instances start from the same fixture,
    /// apply DISJOINT edits (A moves step-1, B moves step-2), and exchanging operations over a
    /// `MemoryBackbone` converges both sides onto an identical projection.
    #[test]
    fn two_instances_converge_disjoint_edits_via_backbone() {
        semio_framework_plugin::testkit::assert_two_instances_converge::<SequencePlayApp, _>(
            "mem://sequence-convergence",
            SequenceCommand::MoveStep(move_step::MoveStep { node_id: "step-1".into(), x: 111.0, y: 0.0 }),
            SequenceCommand::MoveStep(move_step::MoveStep { node_id: "step-2".into(), x: 222.0, y: 0.0 }),
            |app| app.projection().expect("projection"),
        );
    }

    #[test]
    fn sequence_action_ids_resolve_to_labels_in_native_english_and_german() {
        let definition = create_sequence_app().definition;
        for (id, label) in [("run", "Run"), ("stop", "Stop"), ("reorganize", "Reorganize")] {
            let action = definition.actions.iter().find(|action| action.id == id).expect("action");
            assert_eq!(action.label.resolve(Terminology::Native, Locale::En), label, "{id} action label");
        }
        for (id, label) in [("run", "Ausführen"), ("stop", "Stopp"), ("reorganize", "Neu anordnen")] {
            let action = definition.actions.iter().find(|action| action.id == id).expect("action");
            assert_eq!(action.label.resolve(Terminology::Native, Locale::De), label, "{id} action label");
        }
    }

    #[test]
    fn an_unknown_body_key_renders_a_diagnostic_instead_of_panicking() {
        let mut app = new_app();
        assert!(testkit::render(&mut app, "sequence.play.nope").contains("Unknown body"));
    }

    //#region 🔖️ManifestSanity
    #[test]
    fn the_manifest_stitches_every_taxonomy_node() {
        let json = serde_json::to_string(&create_sequence_app().definition).expect("app definition json");
        for id in [main::SEQUENCE_PLAY_WINDOW_MAIN, script::SEQUENCE_PLAY_WINDOW_SCRIPT, compiled::SEQUENCE_PLAY_WINDOW_COMPILED] {
            assert!(json.contains(id), "window kind {id} missing from the manifest: {json}");
        }
        assert!(json.contains(edit::SEQUENCE_PLAY_MODE_EDIT), "edit mode missing from the manifest");
        for body in [SEQUENCE_PLAY_BODY_DOCUMENT, SEQUENCE_PLAY_BODY_CATALOGUE, SEQUENCE_PLAY_BODY_INSPECTOR] {
            assert!(json.contains(body), "panel body {body} missing from the manifest");
        }
        assert!(json.contains("computation.sequence"), "artifact kind missing from the manifest");
    }
    //#endregion 🔖️ManifestSanity

    //#region 🔖️ContextMenuTests
    #[test]
    fn context_menu_stays_within_nine_rows_and_ends_with_destructive_delete() {
        let mut app = new_app_with_registry_wired();
        dispatch(&mut app, SequenceCommand::SetSelection(set_selection::SetSelection { step_ids: vec!["step-1".into()] }));
        let request = ContextMenuRequest { menu: semio_framework_plugin::UiMenuRef { id: "nodeGraph".into(), args: None }, surface: None, window_instance_id: None, point: None };
        let items = app.context_menu(&request);
        assert!(items.len() <= 9, "expected <= 9 top-level rows, got {} ({items:?})", items.len());
        let last = items.last().expect("at least one row");
        assert_eq!(last.id, "delete-selection");
        assert_eq!(last.destructive, Some(true));
    }
    //#endregion 🔖️ContextMenuTests

    //#region 🔖️PortTests
    #[test]
    fn sequence_io_declares_steps_in_and_document_ports() {
        let ports = SequencePlayApp::io().expect("io").all_ports();
        assert!(ports.iter().any(|port| port.id == "document:in"));
        assert!(ports.iter().any(|port| port.id == "document:out"));
        assert!(ports.iter().any(|port| port.id == "steps:in"));
    }

    #[test]
    fn import_media_steps_in_inserts_a_new_step_from_an_object_payload() {
        let mut app = new_app_with_registry_wired();
        let before = app.projection().expect("projection").steps.len();
        let media = Media {
            media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::Computation, form: semio_framework_plugin::MediaForm::Any },
            payload: MediaPayload::Structured { schema: "computation.value".into(), json: json!({ "message": "from upstream" }).to_string() },
        };
        app.import_media("steps:in", &media, &semio_framework_plugin::testkit::meta("local")).expect("import steps:in");
        let after = app.projection().expect("projection");
        assert_eq!(after.steps.len(), before + 1);
        let imported = after.steps.last().expect("imported step");
        assert_eq!(imported.kind, "computation.import");
        assert_eq!(imported.params.get("message").and_then(|value| value.as_atom()).and_then(|atom| atom.as_str()), Some("from upstream"));
    }

    #[test]
    fn import_media_steps_in_wraps_a_bare_scalar_payload() {
        let mut app = new_app_with_registry_wired();
        let media = Media {
            media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::Computation, form: semio_framework_plugin::MediaForm::Any },
            payload: MediaPayload::Structured { schema: "computation.value".into(), json: "42".into() },
        };
        app.import_media("steps:in", &media, &semio_framework_plugin::testkit::meta("local")).expect("import steps:in");
        let after = app.projection().expect("projection");
        let imported = after.steps.last().expect("imported step");
        assert_eq!(imported.params.get("value").and_then(|value| value.as_atom()).and_then(|atom| atom.as_f64()), Some(42.0));
    }

    #[test]
    fn import_media_rejects_unknown_port() {
        let mut app = new_app_with_registry_wired();
        let media = Media {
            media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::Computation, form: semio_framework_plugin::MediaForm::Any },
            payload: MediaPayload::Structured { schema: "computation.value".into(), json: "{}".into() },
        };
        assert!(app.import_media("not-a-port", &media, &semio_framework_plugin::testkit::meta("local")).is_err());
    }
    //#endregion 🔖️PortTests

    //#region 🔖️CommandSurface
    /// 🏷️ Every declared manifest action id must be reachable as exactly one command row, and every
    /// row's wire keyword must be distinct — the cross-cutting invariant `app_commands!` is there to
    /// hold.
    #[test]
    fn command_ids_are_unique_and_match_the_declared_manifest_actions() {
        let commands = every_command();
        let ids: Vec<&str> = commands.iter().map(|command| command.command_id()).collect();
        let mut sorted = ids.clone();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), ids.len(), "duplicate command ids in {ids:?}");
        assert_eq!(ids.len(), 18, "every SequenceCommand row must be covered by every_command()");
    }

    /// ⚖️ LAW: text and binary are two projections of the same command, for every single row.
    #[test]
    fn every_command_round_trips_through_text_and_binary() {
        for command in every_command() {
            store::test_support::assert_op_text_binary_equivalence(&command);
        }
    }

    /// ⚖️ LAW: the leading token of every printed op line is the row's `dsl` wire keyword — the
    /// kebab-cased command id, for every row (sequence has no `flow`-style id/keyword divergence).
    #[test]
    fn every_printed_op_line_starts_with_the_rows_wire_keyword() {
        for command in every_command() {
            let id = command.command_id();
            let expected: String = id.chars().flat_map(|c| if c.is_ascii_uppercase() { vec!['-', c.to_ascii_lowercase()] } else { vec![c] }).collect();
            let printed = protocol::OpText::print_op(&command);
            assert_eq!(printed.split(' ').next().unwrap_or_default(), expected, "wire keyword drifted for command {id}: {printed:?}");
        }
    }

    /// 🧾️ One representative value per row, in declaration (= binary ordinal) order.
    pub(super) fn every_command() -> Vec<SequenceCommand> {
        vec![
            SequenceCommand::AddStep(add_step::AddStep { kind: "log.print".into(), x: 1.0, y: 2.0 }),
            SequenceCommand::AddStepToSlot(add_step_to_slot::AddStepToSlot { kind: "log.print".into(), x: 1.0, y: 2.0, owner: "step-1".into(), slot_name: "then".into() }),
            SequenceCommand::AddStepDropped(add_step_dropped::AddStepDropped { kind: "log.print".into(), x: 1.0, y: 2.0, picked_step_id: Some("step-1".into()) }),
            SequenceCommand::RemoveStep(remove_step::RemoveStep { id: "step-1".into() }),
            SequenceCommand::DeleteSelection(delete_selection::DeleteSelection {}),
            SequenceCommand::MoveStep(move_step::MoveStep { node_id: "step-1".into(), x: 5.0, y: 6.0 }),
            SequenceCommand::ConnectSteps(connect_steps::ConnectSteps { source_node_id: "step-1".into(), target_node_id: "step-2".into() }),
            SequenceCommand::DisconnectSteps(disconnect_steps::DisconnectSteps { from_id: "step-1".into(), to_id: "step-2".into() }),
            SequenceCommand::SetStepParams(set_step_params::SetStepParams { id: "step-1".into(), params_json: "{\"a\":1}".into() }),
            SequenceCommand::SetStepCollapsed(set_step_collapsed::SetStepCollapsed { id: "step-1".into() }),
            SequenceCommand::Reorganize(reorganize::Reorganize {}),
            SequenceCommand::NodeGraphEdit(node_graph_edit::NodeGraphEdit { operations_json: "[]".into() }),
            SequenceCommand::SetSelection(set_selection::SetSelection { step_ids: vec!["step-1".into(), "step-2".into()] }),
            SequenceCommand::SetOrientation(set_orientation::SetOrientation { value: "topBottom".into() }),
            SequenceCommand::Run(run_command::Run {}),
            SequenceCommand::Stop(stop_command::Stop {}),
            SequenceCommand::SetViewport(set_viewport::SetViewport { camera: crate::artifacts::sequence::SequenceCamera { x: 1.0, y: 2.0, zoom: 3.0 } }),
            SequenceCommand::SetLocale(set_locale::SetLocale { value: "de-DE".into() }),
        ]
    }

    /// ⚖️ Pinned to the exact hex captured from the pre-merge `sequence_protocol` crate — a
    /// regression here is a real wire-format break, not a test-fixture mismatch.
    #[test]
    fn optional_field_row_keeps_its_pre_migration_bytes() {
        let some = SequenceCommand::AddStepDropped(add_step_dropped::AddStepDropped { kind: "log.print".into(), x: 1.0, y: 2.0, picked_step_id: Some("step-1".into()) });
        assert_eq!(protocol::OpText::print_op(&some), "add-step-dropped kind=log.print x=1 y=2 picked-step-id=step-1");
        assert_eq!(protocol::OpBinary::encode_op(&some).expect("encode").iter().map(|b| format!("{b:02x}")).collect::<String>(), "010202096c6f672e7072696e7406737465702d31040006000105000000000000f03f02050000000000000040030601");
        let none = SequenceCommand::AddStepDropped(add_step_dropped::AddStepDropped { kind: "log.print".into(), x: 1.0, y: 2.0, picked_step_id: None });
        assert_eq!(protocol::OpText::print_op(&none), "add-step-dropped kind=log.print x=1 y=2");
        assert_eq!(protocol::OpBinary::encode_op(&none).expect("encode").iter().map(|b| format!("{b:02x}")).collect::<String>(), "010201096c6f672e7072696e74030006000105000000000000f03f02050000000000000040");
    }
    //#endregion 🔖️CommandSurface
}
//#endregion 🧪️Tests
