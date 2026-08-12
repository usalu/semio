//! 🖥️ VCS play app — the `ArtifactApp` impl (dispatch-only), the aggregated command enum and the
//! manifest stitch.
//!
//! Everything substantive lives in a taxonomy node: command bodies in `🎮️commands/*`, window renders in
//! `🎭️modes/✏️edit/🪟️windows/*`, panel trees in `📌️panels/*`, labels in `🦀️terminology.rs`, view state in
//! `🦀️config.rs`, headless compute in the artifact's `🧬️schema` (dissolved from `⚙️engine` per ticket
//! 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES). This file is a routing table: `handle` →
//! `VcsCommand::dispatch`, `render` → body-key → node, and a `🔖️Manifest` region that calls one
//! `definition()` per node.

use crate::apps::vcs::commands::{canvas, counter, locale, patch, selection};
use crate::apps::vcs::config::{VcsDemoConfig, VcsDemoConfigMutation};
use crate::apps::vcs::presence::{VcsDemoPresence, VcsDemoPresenceMutation};
use crate::apps::vcs::modes::edit;
use crate::apps::vcs::modes::edit::windows::{editor, history};
use crate::apps::vcs::panels::{document as document_panel, inspection as inspection_panel};
use crate::apps::vcs::terminology::vcs_play_labels;
use crate::artifacts::vcs::{op::VcsDemoMutation, VcsSnapshot, VCS_DOCUMENT_SCHEMA};
use semio_framework_plugin::{NoDraft, NoDraftMutation, DraftView, ui_text, ActionDescriptor, App, ConfigView, ArtifactApp, ArtifactView, Emit, Fault, Label, LocalizedLabel, UiNode};
use store::EngineHandles;
use serde_json::Value;
use store::{ArtifactCommand, ArtifactStore};

//#region 🔖️Constants
pub const VCS_PLAY_APP_ID: &str = "vcs-play";
pub use editor::VCS_PLAY_BODY_EDITOR;
pub use history::VCS_PLAY_BODY_HISTORY;
pub use document_panel::VCS_PLAY_BODY_DOCUMENT;
pub use inspection_panel::VCS_PLAY_BODY_INSPECTION;

/// 🎯️ An `ActionDescriptor` addressed at this app — the single factory every taxonomy node's chrome
/// (`📌️panels/*`, `🎭️modes/*/🪟️windows/*`) builds its `on_change`/item actions with.
pub fn vcs_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    semio_framework_plugin::ActionFactory::new(VCS_PLAY_APP_ID).action(action, args)
}
//#endregion 🔖️Constants

//#region 🔌️Registration
/// 🗂️ Registers `VcsSnapshot`'s pack<->dsl codec under its real `document_schema()` string so
/// `framework/sync`'s `FolderEndpoint::Pack` (and any other schema-keyed caller) can print/parse vcs-play
/// documents without depending on this crate's concrete snapshot/mutation types. Called by
/// `semio_plugin!`'s `setup:` hook — was the old bundle crate's `register_vcs_exports()`, then
/// `⚙️engine::register()` (dissolved per ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES:
/// `register()`/`register_pilot_languages()` are app-constitutional, not headless-engine compute — this
/// body calls `crate::apps::vcs::config::schema::register_app_schema()` and
/// `register_document_codec_for_app::<VcsPlayApp>(…)`, both app types).
pub fn register() {
    crate::artifacts::vcs::io_registry::register();

    register_artifact_schema();
    register_artifact_inference();
    register_pilot_languages();
    crate::apps::vcs::config::schema::register_app_schema();
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<VcsPlayApp>(VCS_DOCUMENT_SCHEMA);
}

/// 📌️ Registers handcrafted facet grammars (text) and protocols (binary) for in-process execution.
pub fn register_pilot_languages() {
    dsl::register_language(dsl::LanguageSpec {
        id: "vcs.document",
        extension: Some("vcs"),
        role: dsl::LanguageRole::Document,
        grammar: Some(crate::artifacts::vcs::dsl::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::vcs::dsl::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::vcs::pack::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::vcs::pack::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("vcs.document"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "vcs.op",
        extension: None,
        role: dsl::LanguageRole::Ops,
        grammar: Some(crate::artifacts::vcs::op::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::vcs::op::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::vcs::spr::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::vcs::spr::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("vcs.op"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "vcs.diff",
        extension: None,
        role: dsl::LanguageRole::Diff,
        grammar: Some(crate::artifacts::vcs::schema::diff::text::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::vcs::schema::diff::text::COMPONENT_GRAMMAR_PATH),
        protocol: None,
        protocol_path: None,
        hooks: dsl::passthrough_hooks("vcs.diff"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "vcs.pack",
        extension: None,
        role: dsl::LanguageRole::Pack,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::vcs::pack::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::vcs::pack::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("vcs.pack"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "vcs.spr",
        extension: None,
        role: dsl::LanguageRole::Spr,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::vcs::spr::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::vcs::spr::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("vcs.spr"),
    });
}
//#endregion 🔌️Registration

//#region 🔖️SchemaRegistry
/// 📌️ Registers the twenty handcrafted schema leaves for `s.vcs.vcs`.
pub fn register_artifact_schema() {
    ::schema::register_artifact_schema_descriptor(crate::artifacts::vcs::schema::vcs_artifact_schema_descriptor());
}

/// 💡️ Registers `s.vcs.vcs.inference`'s facet leaves into the OS-wide inference catalog — sibling
/// to `register_artifact_schema()` (separate registry, ticket
/// 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING).
pub fn register_artifact_inference() {
    ::schema::register_artifact_inference_descriptor(crate::artifacts::vcs::standards::v1::subsets::any::schema::inferences::vcs_artifact_inference_descriptor());
}
//#endregion 🔖️SchemaRegistry

//#region 🔖️Commands
semio_framework_plugin::app_commands! {
    /// 🎯️ `VcsPlayApp::Command` — the SOLE dispatch surface for the vcs demo app's own behavior. The six
    /// history actions (undo/redo/commitCheckpoint/createAlternative/switchAlternative/checkoutCheckpoint)
    /// never reach here — `VcsArtifactApp` intercepts those itself as host mechanics, not app behavior
    /// (see `shooting_protocol::ShootingCommand`'s identical doc). Field shapes mirror each action's old
    /// JSON `args` object exactly. **Row order is the binary variant ordinal: appending is safe,
    /// reordering is a wire-format break.**
    pub enum VcsCommand for VcsSnapshot, VcsDemoMutation, VcsDemoConfig, VcsDemoConfigMutation {
        "incrementCounter" as "increment-counter" => increment_counter::IncrementCounter,
        "patchSnapshot" as "patch-snapshot" => patch_snapshot::PatchSnapshot,
        "textEdit" as "text-edit" => text_edit::TextEdit,
        "edit" as "edit" => edit_command::Edit,
        "setSelection" as "set-selection" => set_selection::SetSelection,
        "setLocale" as "locale" => set_locale::SetLocale,
        "noMutation" as "no-operation" => no_operation::NoMutation,
        "canvasPointerDown" as "canvas-pointer-down" => canvas_pointer_down::CanvasPointerDown,
        "canvasPointerMove" as "canvas-pointer-move" => canvas_pointer_move::CanvasPointerMove,
        "canvasPointerUp" as "canvas-pointer-up" => canvas_pointer_up::CanvasPointerUp,
        "canvasWheel" as "canvas-wheel" => canvas_wheel::CanvasWheel,
    }
}

// 🧷️ `app_commands!` addresses each payload module by a single identifier, so every `🎮️commands/*`
// payload module is imported here under its own flat name.
use canvas::{canvas_pointer_down, canvas_pointer_move, canvas_pointer_up, canvas_wheel, no_operation};
use counter::increment_counter;
use locale::set_locale;
use patch::{edit as edit_command, patch_snapshot, text_edit};
use selection::set_selection;
//#endregion 🔖️Commands

//#region 🔖️DocumentHelpers
fn demo_authors() -> Vec<vcs_kernel::Author> {
    vec![
        vcs_kernel::Author { id: "author-alice".into(), name: "Alice".into(), avatar: None },
        vcs_kernel::Author { id: "author-bob".into(), name: "Bob".into(), avatar: None },
        vcs_kernel::Author { id: "author-carol".into(), name: "Carol".into(), avatar: None },
    ]
}

/// 🌱️ Seeds a rich, forked checkpoint/alternative history directly against the store — this app's
/// whole point is exercising the history UI (swimlane graph, checkpoints, alternatives, undo/redo),
/// so its "initial document" is itself a populated history, not a bare projection. Dispatched via
/// `ArtifactApp::seed`, called once by `VcsArtifactApp::new` right after store construction.
fn seed_vcs_demo_history(store: &mut ArtifactStore<VcsSnapshot, VcsDemoMutation>) {
    let authors = demo_authors();
    let alice = authors[0].clone();
    let bob = authors[1].clone();
    let carol = authors[2].clone();
    let last_checkpoint_id = |store: &ArtifactStore<VcsSnapshot, VcsDemoMutation>| -> String { store.envelope().vcs.checkpoints.last().expect("checkpoint just committed").id.clone() };

    let _ = store.dispatch(ArtifactCommand::Apply { mutations: vec![VcsDemoMutation::SetCounter { counter: 1 }, VcsDemoMutation::SetTitle { title: "VCS Demo".into() }], description: Some("bootstrap".into()) });
    let _ = store.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("Bootstrap".into()), authors: vec![alice.clone()] });
    let c1 = last_checkpoint_id(store);

    let _ = store.dispatch(ArtifactCommand::Apply { mutations: vec![VcsDemoMutation::SetNotes { notes: "main line".into() }, VcsDemoMutation::SetStatus { status: "draft".into() }], description: None });
    let _ = store.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("Annotate main draft".into()), authors: vec![alice.clone(), bob.clone()] });
    let c2 = last_checkpoint_id(store);

    let _ = store.dispatch(ArtifactCommand::Apply { mutations: vec![VcsDemoMutation::SetCounter { counter: 2 }], description: None });
    let _ = store.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("Main milestone".into()), authors: vec![alice.clone(), bob.clone(), carol.clone()] });
    let c3 = last_checkpoint_id(store);

    let _ = store.dispatch(ArtifactCommand::CheckoutCheckpoint { checkpoint_id: c3.clone() });
    let _ = store.dispatch(ArtifactCommand::CreateAlternative { name: "feature-a".into() });
    let _ = store.dispatch(ArtifactCommand::Apply { mutations: vec![VcsDemoMutation::SetTitle { title: "Feature A".into() }, VcsDemoMutation::AddTag { tag: "feature-a".into() }], description: None });
    let _ = store.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("Start feature A".into()), authors: vec![alice.clone()] });
    let c4 = last_checkpoint_id(store);
    let feature_a_id = store.envelope().active_alternative_id.clone().expect("feature-a alternative id");

    let _ = store.dispatch(ArtifactCommand::Apply { mutations: vec![VcsDemoMutation::SetCounter { counter: 10 }], description: None });
    let _ = store.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("Feature A progress".into()), authors: vec![alice.clone(), bob.clone()] });

    let _ = store.dispatch(ArtifactCommand::CheckoutCheckpoint { checkpoint_id: c3.clone() });
    let _ = store.dispatch(ArtifactCommand::CreateAlternative { name: "feature-b".into() });
    let _ = store.dispatch(ArtifactCommand::Apply { mutations: vec![VcsDemoMutation::SetTitle { title: "Feature B".into() }, VcsDemoMutation::SetNotes { notes: "branch b".into() }], description: None });
    let _ = store.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("Start feature B".into()), authors: vec![bob.clone()] });
    let feature_b_id = store.envelope().active_alternative_id.clone().expect("feature-b alternative id");

    let _ = store.dispatch(ArtifactCommand::Apply { mutations: vec![VcsDemoMutation::SetCounter { counter: 20 }], description: None });
    let _ = store.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("Feature B try".into()), authors: vec![bob.clone(), carol.clone()] });

    let _ = store.dispatch(ArtifactCommand::CheckoutCheckpoint { checkpoint_id: c3 });
    let _ = store.dispatch(ArtifactCommand::Apply { mutations: vec![VcsDemoMutation::SetStatus { status: "active".into() }], description: None });
    let _ = store.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("Resume main".into()), authors: vec![carol.clone()] });
    let c8 = last_checkpoint_id(store);

    let _ = store.dispatch(ArtifactCommand::SwitchAlternative { alternative_id: feature_a_id });
    let _ = store.dispatch(ArtifactCommand::Apply { mutations: vec![VcsDemoMutation::SetCounter { counter: 11 }, VcsDemoMutation::AddTag { tag: "wip".into() }], description: None });
    let _ = store.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("Feature A sprint".into()), authors: vec![alice.clone(), carol.clone()] });

    let _ = store.dispatch(ArtifactCommand::CheckoutCheckpoint { checkpoint_id: c4 });
    let _ = store.dispatch(ArtifactCommand::CreateAlternative { name: "feature-a-hotfix".into() });
    let _ = store.dispatch(ArtifactCommand::Apply { mutations: vec![VcsDemoMutation::SetStatus { status: "hotfix".into() }], description: None });
    let _ = store.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("Hotfix off feature A".into()), authors: vec![bob.clone()] });

    let _ = store.dispatch(ArtifactCommand::SwitchAlternative { alternative_id: feature_b_id });
    let _ = store.dispatch(ArtifactCommand::Apply { mutations: vec![VcsDemoMutation::AddTag { tag: "review".into() }], description: None });
    let _ = store.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("Feature B review".into()), authors: vec![bob.clone()] });

    let _ = store.dispatch(ArtifactCommand::CheckoutCheckpoint { checkpoint_id: c8 });
    let _ = store.dispatch(ArtifactCommand::Apply { mutations: vec![VcsDemoMutation::SetCounter { counter: 3 }, VcsDemoMutation::SetNotes { notes: "main polish".into() }, VcsDemoMutation::AddTag { tag: "release".into() }], description: None });
    let _ = store.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("Main batch polish".into()), authors: vec![alice.clone(), bob.clone(), carol.clone()] });

    let _ = store.dispatch(ArtifactCommand::Apply { mutations: vec![VcsDemoMutation::SetStatus { status: "done".into() }], description: None });
    let _ = store.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("Main release".into()), authors: vec![alice] });

    let _ = store.dispatch(ArtifactCommand::CheckoutCheckpoint { checkpoint_id: c2 });
    let _ = store.dispatch(ArtifactCommand::CreateAlternative { name: "docs".into() });
    let _ = store.dispatch(ArtifactCommand::Apply { mutations: vec![VcsDemoMutation::SetNotes { notes: "documentation pass".into() }], description: None });
    let _ = store.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("Docs branch".into()), authors: vec![carol.clone()] });

    let _ = store.dispatch(ArtifactCommand::CheckoutCheckpoint { checkpoint_id: c1 });
    let _ = store.dispatch(ArtifactCommand::CreateAlternative { name: "spike".into() });
    let _ = store.dispatch(ArtifactCommand::Apply { mutations: vec![VcsDemoMutation::SetTitle { title: "Spike prototype".into() }], description: None });
    let _ = store.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("Spike experiment".into()), authors: vec![bob, carol] });
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️VcsPlayApp
/// 🧪️ B1: unit struct — the former `VcsPlayApp::selected_checkpoint_ids` `RefCell` field now lives in
/// `crate::apps::vcs::config::VcsDemoConfig` (see `ArtifactApp::Config`), written through
/// `VcsDemoConfigMutation`s.
#[derive(Default)]
pub struct VcsPlayApp;

impl ArtifactApp for VcsPlayApp {
    type Snapshot = VcsSnapshot;
    type Mutation = VcsDemoMutation;
    type Config = VcsDemoConfig;
    type ConfigMutation = VcsDemoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = VcsDemoPresence;
    type PresenceMutation = VcsDemoPresenceMutation;

    type Command = VcsCommand;

    const APP_ID: &'static str = VCS_PLAY_APP_ID;
    const DOCUMENT_SCHEMA: &'static str = VCS_DOCUMENT_SCHEMA;

    fn initial_snapshot() -> VcsSnapshot {
        crate::artifacts::vcs::standards::v1::subsets::any::schema::empty_vcs_snapshot()
    }

    fn seed(store: &mut ArtifactStore<VcsSnapshot, VcsDemoMutation>) {
        seed_vcs_demo_history(store);
    }

    /// 🏷️ The manifest action id each command was declared under — supplied wholesale by
    /// `app_commands!`'s generated `command_id()`. `setLocale` isn't declared in the manifest (mirrors
    /// `ShootingCommand::SetLocale` — see `shooting_ui`'s identical doc), so it skips enforcement.
    fn command_id(command: &VcsCommand) -> &'static str {
        command.command_id()
    }

    fn handle(command: &VcsCommand, doc: &ArtifactView<'_, VcsSnapshot>, cfg: &ConfigView<'_, VcsDemoConfig>, _draft: &DraftView<'_, Self::Draft>, _engines: &EngineHandles) -> Result<Emit<VcsDemoMutation, VcsDemoConfigMutation, Self::DraftMutation>, Fault> {
        command.dispatch(doc, cfg)
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, VcsSnapshot>, cfg: &ConfigView<'_, VcsDemoConfig>) -> UiNode {
        let labels = vcs_play_labels(cfg.snapshot);
        match body_key {
            VCS_PLAY_BODY_EDITOR => editor::render(doc.snapshot, labels),
            VCS_PLAY_BODY_HISTORY => history::render(doc.history),
            VCS_PLAY_BODY_DOCUMENT => document_panel::render(doc.history, &cfg.snapshot.selected_checkpoint_ids, labels),
            VCS_PLAY_BODY_INSPECTION => inspection_panel::render(doc.snapshot, labels),
            _ => ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️VcsPlayApp

//#region 🔖️Manifest
/// 🧱️ The manifest stitch: one call per taxonomy node, each sourced from that node's own `definition()`.
/// Only the leaf action/keybinding declarations (which have no dedicated `_def` passthrough) are written
/// out inline.
pub fn create_vcs_app() -> App {
    App::from_builder(
        App::builder(VCS_PLAY_APP_ID, LocalizedLabel::native("VCS", "VCS")).document(["semio", "vcs"])
            .artifact_kind(crate::artifacts::vcs::artifact_kind())
            .icon_id("git-branch")
            .mode_def(edit::definition())
            .default_mode_id(edit::VCS_PLAY_MODE_EDIT)
            .window_kind_def(editor::definition())
            .window_kind_def(history::definition())
            .panel_tab_def(document_panel::definition())
            .panel_tab_def(inspection_panel::definition())
            .mutation("incrementCounter", LocalizedLabel::native("Increment Counter", "Zähler erhöhen"))
            .mutation("patchSnapshot", LocalizedLabel::native("Patch Projection", "Projektion aktualisieren"))
            .mutation("textEdit", LocalizedLabel::native("Edit Text", "Text bearbeiten"))
            .mutation("edit", LocalizedLabel::native("Edit", "Bearbeiten"))
            .view_action("setSelection", LocalizedLabel::native("Set Selection", "Auswahl festlegen"))
            .view_action("noMutation", LocalizedLabel::native("No-operation", "Keine Aktion"))
            .view_action("canvasPointerDown", LocalizedLabel::native("Canvas Pointer Down", "Leinwand-Zeiger gedrückt"))
            .view_action("canvasPointerMove", LocalizedLabel::native("Canvas Pointer Move", "Leinwand-Zeiger bewegt"))
            .view_action("canvasPointerUp", LocalizedLabel::native("Canvas Pointer Up", "Leinwand-Zeiger losgelassen"))
            .view_action("canvasWheel", LocalizedLabel::native("Canvas Wheel", "Leinwand-Mausrad"))
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo")
            .default_layout(edit::layout())
            // 🎯️ Typed channel surface (HEADLESS-APP-ENGINE-BINARY-COMMAND-PROTOCOL-FOUNDATIONS Wave 1) —
            // this app has no user-visible sticky defaults, so `config_spec()` stays the trait default
            // `ConfigSpec::empty()`; declared anyway for parity with every other converted app.
            .config(VcsPlayApp::config_spec()),
    )
}
//#endregion 🔖️Manifest

//#region 🧪️Testkit
/// 🧪️ Shared test scaffolding for every taxonomy node's own `🧪️Tests` region — a component file must be
/// able to drive the whole app without re-deriving the harness.
#[cfg(test)]
pub(crate) mod testkit {
    use super::*;
    use semio_framework_plugin::testkit::{meta, new_app, new_app_with_registry};
    use semio_framework_plugin::{InvocationResult, PluginApp, VcsArtifactApp, ViewModel};
    use store::ArtifactEnvelope;

    pub type VcsApp = VcsArtifactApp<VcsPlayApp>;

    /// 🧪️ A bare app instance — no `AppActionRegistry`, so undeclared internal commands dispatch freely.
    pub fn app() -> VcsApp {
        new_app::<VcsPlayApp>()
    }

    /// 🧪️ An app wired to the real manifest registry — enforces View/Shell kind discipline.
    pub fn app_with_registry() -> VcsApp {
        new_app_with_registry::<VcsPlayApp>(create_vcs_app)
    }

    pub fn dispatch(instance: &mut VcsApp, command: VcsCommand) -> InvocationResult {
        instance.dispatch_typed(command, &meta("local")).expect("dispatch")
    }

    pub fn render(instance: &mut VcsApp, body_key: &str) -> String {
        serde_json::to_string(&instance.render(body_key, None, &ViewModel::default()).expect("render")).expect("render json")
    }

    /// 📦️ Parses `document_pack()` (the full envelope) for tests that need to inspect raw
    /// checkpoints/alternatives directly — safe here because none of these tests undo/redo, so every
    /// edit in the log is still applied.
    pub fn seeded_envelope(instance: &VcsApp) -> ArtifactEnvelope<VcsSnapshot, VcsDemoMutation> {
        let files = instance.document_pack().expect("document pack");
        store::parse_document_pack::<VcsSnapshot, VcsDemoMutation>(&files.pack, &files.spr).expect("parse document pack").envelope
    }
}
//#endregion 🧪️Testkit

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::vcs::testkit::{app, dispatch, seeded_envelope};
    use semio_framework_plugin::testkit::meta;
    use semio_framework_plugin::PluginApp;
    use store::HistoryColumn;

    //#region 🔖️CommandSurface
    /// 🏷️ Every declared manifest action id must be reachable as exactly one command row, and every row's
    /// wire keyword must be distinct — the cross-cutting invariant `app_commands!` is there to hold.
    #[test]
    fn command_ids_are_unique_and_match_the_declared_manifest_actions() {
        let commands = every_command();
        let ids: Vec<&str> = commands.iter().map(|command| command.command_id()).collect();
        let mut sorted = ids.clone();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), ids.len(), "duplicate command ids in {ids:?}");
        assert_eq!(ids.len(), 11, "every VcsCommand row must be covered by every_command()");
    }

    /// ⚖️ LAW: text and binary are two projections of the same command, for every single row.
    #[test]
    fn every_command_round_trips_through_text_and_binary() {
        for command in every_command() {
            store::os_store::test_support::assert_op_text_binary_equivalence(&command);
        }
    }

    /// ⚖️ LAW: the leading token of every printed op line is the row's `dsl` wire keyword — the
    /// kebab-cased command id, except for the one documented divergence (`setLocale` → `locale`, an
    /// undeclared host-pushed command). This is what a missing `#[dsl(keyword = ..)]` on a payload struct
    /// silently breaks (the record prints with no keyword at all and no longer parses).
    #[test]
    fn every_printed_op_line_starts_with_the_rows_wire_keyword() {
        for command in every_command() {
            let id = command.command_id();
            let expected = if id == "setLocale" {
                "locale".to_string()
            } else if id == "noMutation" {
                "no-operation".to_string()
            } else {
                id.chars().flat_map(|c| if c.is_ascii_uppercase() { vec!['-', c.to_ascii_lowercase()] } else { vec![c] }).collect()
            };
            let printed = protocol::OpText::print_op(&command);
            assert_eq!(printed.split(' ').next().unwrap_or_default(), expected, "wire keyword drifted for command {id}: {printed:?}");
        }
    }

    // 🧷️ No `VcsCommand` payload has an `Option` field (unlike flow's `AddWidget`/`SetGridVisible`), so
    // there is no `None`/`Some`-distinguishing wire case here and no
    // `optional_field_rows_keep_their_pre_migration_bytes`-style pinning test is needed.

    /// 🧾️ One representative value per row, in declaration (= binary ordinal) order. Matches the pilot's
    /// wire baseline dump byte-for-byte (ticket `🧪️wire-baseline-before.txt`).
    pub(super) fn every_command() -> Vec<VcsCommand> {
        vec![
            VcsCommand::IncrementCounter(increment_counter::IncrementCounter {}),
            VcsCommand::PatchSnapshot(patch_snapshot::PatchSnapshot { field: "title".into(), value: "Renamed".into() }),
            VcsCommand::TextEdit(text_edit::TextEdit { text: "{}".into() }),
            VcsCommand::Edit(edit_command::Edit { text: "{}".into() }),
            VcsCommand::SetSelection(set_selection::SetSelection { ids: vec!["checkpoint-1".into(), "checkpoint-2".into()] }),
            VcsCommand::SetLocale(set_locale::SetLocale { value: "de-DE".into() }),
            VcsCommand::NoMutation(no_operation::NoMutation {}),
            VcsCommand::CanvasPointerDown(canvas_pointer_down::CanvasPointerDown {}),
            VcsCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove {}),
            VcsCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp {}),
            VcsCommand::CanvasWheel(canvas_wheel::CanvasWheel {}),
        ]
    }
    //#endregion 🔖️CommandSurface

    //#region 🔖️ManifestSanity
    #[test]
    fn the_manifest_stitches_every_taxonomy_node() {
        let json = serde_json::to_string(&create_vcs_app().definition).expect("app definition json");
        for id in [editor::VCS_PLAY_WINDOW_EDITOR, history::VCS_PLAY_WINDOW_HISTORY] {
            assert!(json.contains(id), "window kind {id} missing from the manifest: {json}");
        }
        assert!(json.contains(edit::VCS_PLAY_MODE_EDIT), "mode missing from the manifest");
        for body in [VCS_PLAY_BODY_DOCUMENT, VCS_PLAY_BODY_INSPECTION] {
            assert!(json.contains(body), "panel body {body} missing from the manifest");
        }
        assert!(json.contains("vcs.document"), "artifact kind missing from the manifest");
    }

    /// 🧪️ The registry-enforced app (View/Shell kind discipline) must still dispatch every declared
    /// manifest action — exercises `testkit::app_with_registry`, the counterpart to the bare `app()`
    /// every other node's tests use.
    #[test]
    fn registry_enforced_app_dispatches_a_declared_action() {
        use crate::apps::vcs::testkit::app_with_registry;
        let mut instance = app_with_registry();
        let before = instance.snapshot().expect("materialize snapshot").counter;
        dispatch(&mut instance, VcsCommand::IncrementCounter(increment_counter::IncrementCounter {}));
        assert_eq!(instance.snapshot().expect("materialize snapshot").counter, before + 1);
    }
    //#endregion 🔖️ManifestSanity

    //#region 🔖️CrossCutting
    #[test]
    fn seeded_history_has_checkpoints() {
        let instance = app();
        let envelope = seeded_envelope(&instance);
        assert!(envelope.vcs.alternatives.len() >= 5, "expected >=5 alternatives, got {}", envelope.vcs.alternatives.len());
        assert!(envelope.vcs.checkpoints.len() >= 14, "expected >=14 checkpoints, got {}", envelope.vcs.checkpoints.len());
        let mut children_by_parent: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
        for checkpoint in &envelope.vcs.checkpoints {
            if let Some(parent_id) = &checkpoint.parent_id {
                *children_by_parent.entry(parent_id.clone()).or_insert(0) += 1;
            }
        }
        assert!(children_by_parent.values().any(|count| *count >= 2), "seed must contain a real fork (a checkpoint with >=2 children)");
        let lanes: std::collections::HashSet<usize> = store::build_history_columns(&envelope).into_iter().map(|column: HistoryColumn| column.lane).collect();
        assert!(lanes.len() >= 3, "expected >=3 distinct swimlanes, got {lanes:?}");
    }

    #[test]
    fn checkout_then_commit_forks_across_actions() {
        let mut instance = app();
        let envelope_before = seeded_envelope(&instance);
        let root_checkpoint_id = envelope_before.vcs.checkpoints[0].id.clone();
        let children_of_root_before = envelope_before.vcs.checkpoints.iter().filter(|checkpoint| checkpoint.parent_id.as_deref() == Some(root_checkpoint_id.as_str())).count();

        let checkout = instance.handle_action("checkoutCheckpoint", Some(&serde_json::json!({ "checkpointId": root_checkpoint_id })), &meta("local")).expect("checkout");
        assert!(checkout.mutations.is_empty(), "history actions never emit KernelMutations");

        dispatch(&mut instance, VcsCommand::IncrementCounter(increment_counter::IncrementCounter {}));
        instance.handle_action("commitCheckpoint", Some(&serde_json::json!({ "message": "forked from root" })), &meta("local")).expect("commit");

        let envelope_after = seeded_envelope(&instance);
        let children_of_root_after = envelope_after.vcs.checkpoints.iter().filter(|checkpoint| checkpoint.parent_id.as_deref() == Some(root_checkpoint_id.as_str())).count();
        assert_eq!(children_of_root_after, children_of_root_before + 1, "checking out the root then committing through actions must add a new fork of the root, not extend the trunk");
    }

    #[test]
    fn undo_redo_round_trips_through_the_wrapper() {
        let mut instance = app();
        let before = instance.snapshot().expect("materialize snapshot").counter;
        dispatch(&mut instance, VcsCommand::IncrementCounter(increment_counter::IncrementCounter {}));
        assert_eq!(instance.snapshot().expect("materialize snapshot").counter, before + 1);
        let undo = instance.handle_action("undo", None, &meta("local")).expect("undo");
        assert!(undo.mutations.is_empty());
        assert!(undo.events.iter().any(|event| event.kind == "history-changed"));
        assert_eq!(instance.snapshot().expect("materialize snapshot").counter, before);
        instance.handle_action("redo", None, &meta("local")).expect("redo");
        assert_eq!(instance.snapshot().expect("materialize snapshot").counter, before + 1);
    }

    #[test]
    fn create_and_switch_alternative_round_trip_through_the_wrapper() {
        let mut instance = app();
        let create = instance.handle_action("createAlternative", Some(&serde_json::json!({ "name": "trying-something" })), &meta("local")).expect("create alternative");
        assert!(create.mutations.is_empty());
        let envelope = seeded_envelope(&instance);
        assert!(envelope.active_alternative_id.is_some(), "createAlternative must set an active alternative");
    }
    //#endregion 🔖️CrossCutting
}
//#endregion 🧪️Tests
