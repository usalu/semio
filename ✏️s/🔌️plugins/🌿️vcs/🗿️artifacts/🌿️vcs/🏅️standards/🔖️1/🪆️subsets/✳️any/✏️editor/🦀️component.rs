//! 🖥️ VCS editor surface — the `ArtifactEditor` impl (dispatch-only), the aggregated command enum and
//! the manifest stitch.
//!
//! Everything substantive lives in a taxonomy node: command bodies in `🎮️commands/*`, window renders in
//! `🎭️modes/✏️edit/🪟️windows/*`, panel trees in `📌️panels/*`, labels in `🦀️terminology.rs`, view state in
//! `🦀️config.rs`, headless compute in the artifact's `🧬️schema` (dissolved from `⚙️engine` per ticket
//! 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES). This file is a routing table: `handle` →
//! `VcsCommand::dispatch`, `render` → body-key → node, and a `🔖️Manifest` region that calls one
//! `definition()` per node.

use crate::artifacts::vcs::{op::VcsDemoMutation, VcsSnapshot, VCS_DOCUMENT_SCHEMA};
use crate::editor::vcs::commands::edit as edit_command;
use crate::editor::vcs::commands::{canvas_pointer_down, canvas_pointer_move, canvas_pointer_up, canvas_wheel, increment_counter, no_operation, patch_snapshot, set_locale, text_edit};
use crate::editor::vcs::config::{VcsDemoConfig, VcsDemoConfigMutation};
use crate::editor::vcs::modes::edit;
use crate::editor::vcs::modes::edit::windows::{editor, history};
use crate::editor::vcs::panels::{document as document_panel, inspection as inspection_panel};
use crate::editor::vcs::presence::{VcsDemoPresence, VcsDemoPresenceMutation};
use crate::editor::vcs::terminology::vcs_play_labels;
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::{
    ui_text, ActionDescriptor, ArtifactEditor, ArtifactView, ConfigView, Dialect, DraftView, Editor, Emit, Fault, GranularityDefinition, HierarchyProvider, HoverSpec, InteractionDefinition, InteractionRef, Label, LocalizedLabel, MergeMode, NoDraft,
    NoDraftMutation, SelectionMethod, SelectionMode, SelectionSpec, UiNode,
};
use serde_json::Value;
use store::EngineHandles;

//#region 🔖️Constants
pub const VCS_PLAY_APP_ID: &str = "vcs-play";
pub use document_panel::VCS_PLAY_BODY_DOCUMENT;
pub use editor::VCS_PLAY_BODY_EDITOR;
pub use history::VCS_PLAY_BODY_HISTORY;
pub use inspection_panel::VCS_PLAY_BODY_INSPECTION;

/// 🎯️ An `ActionDescriptor` addressed at this app — the single factory every taxonomy node's chrome
/// (`📌️panels/*`, `🎭️modes/*/🪟️windows/*`) builds its `on_change`/item actions with.
pub async fn vcs_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    semio_framework_plugin::ActionFactory::new(VCS_PLAY_APP_ID).action(action, args)
}
//#endregion 🔖️Constants

//#region 🔖️Interaction
/// 🕹️ "history" — the single FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM (26/08/14) interaction domain
/// this app declares: multi-select highlighting over the seeded checkpoint history, `Flat` (checkpoints
/// have no selectable-entity nesting — the DAG's `parent_id` links only matter to the swimlane graph
/// layout, not to selection range/closure), one granularity `"commit"`. Distinct from the per-row
/// `checkoutCheckpoint`/`switchAlternative` click actions the document tree already declares (those are
/// navigation — they change the working checkpoint/alternative — not entity selection), which stay as
/// ordinary actions.
pub const VCS_INTERACTION_HISTORY: &str = "history";
//#endregion 🔖️Interaction

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
        "setLocale" as "locale" => set_locale::SetLocale,
        "noMutation" as "no-operation" => no_operation::NoMutation,
        "canvasPointerDown" as "canvas-pointer-down" => canvas_pointer_down::CanvasPointerDown,
        "canvasPointerMove" as "canvas-pointer-move" => canvas_pointer_move::CanvasPointerMove,
        "canvasPointerUp" as "canvas-pointer-up" => canvas_pointer_up::CanvasPointerUp,
        "canvasWheel" as "canvas-wheel" => canvas_wheel::CanvasWheel,
    }
}
//#endregion 🔖️Commands

//#region 🔖️DocumentHelpers
// 🌱️ `seed_vcs_demo_history` (test-only demo history seeding) now lives in the `🔖️Testkit` region
// below — it must dispatch through `VcsArtifactApp`'s public surface (`dispatch_typed`/
// `handle_action`), not a raw `store::ArtifactStore`, since `ArtifactApp::seed(&mut ArtifactStore)`
// (this app's old direct-store-touch hook) no longer exists on the trait as of ticket
// 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M4 (`ArtifactApp::genesis() -> Vec<Self::Mutation>`
// replaced it, but `genesis` can only emit flat document mutations — it has no way to express
// `CommitCheckpoint`/`CreateAlternative`/`SwitchAlternative`, so it cannot reconstruct branching
// checkpoint history at construction time). Consequence: this demo's rich seeded history is reachable
// from tests (`testkit::app`/`app_with_registry` seed it explicitly) but no longer auto-populates a
// freshly constructed production instance the way `ArtifactApp::seed` used to — restoring that would
// need a framework-level hook `genesis` doesn't provide, which is out of this plugin's boundary
// (`🔌️plugin/🦀️component.rs` is W1-owned).
//#endregion 🔖️DocumentHelpers

//#region 🔖️VcsPlayApp
/// 🧪️ B1: unit struct — the former `VcsPlayApp::selected_checkpoint_ids` `RefCell` field passed through
/// `crate::editor::vcs::config::VcsDemoConfig` before becoming the framework-owned "history" interaction
/// domain (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM, see `VCS_INTERACTION_HISTORY`'s
/// doc comment); `locale` is the only field left in `Config`, written through `VcsDemoConfigMutation`s.
#[derive(Default)]
pub struct VcsPlayApp;

impl ArtifactEditor for VcsPlayApp {
    type Snapshot = VcsSnapshot;
    type Mutation = VcsDemoMutation;
    type Config = VcsDemoConfig;
    type ConfigMutation = VcsDemoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = VcsDemoPresence;
    type PresenceMutation = VcsDemoPresenceMutation;
    type Transient = semio_framework_plugin::NoTransient;
    type TransientMutation = semio_framework_plugin::NoTransientMutation;

    type Command = VcsCommand;

    const DIALECT: Dialect = crate::artifacts::vcs::VCS_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = VCS_DOCUMENT_SCHEMA;

    async fn app_schema() -> Option<::schema::AppSchemaDescriptor> {
        Some(crate::editor::vcs::config::schema::app_schema_descriptor())
    }

    async fn initial_snapshot() -> VcsSnapshot {
        crate::artifacts::vcs::standards::v1::subsets::any::schema::empty_vcs_snapshot()
    }

    /// 🏷️ The manifest action id each command was declared under — supplied wholesale by
    /// `app_commands!`'s generated `command_id()`. `setLocale` isn't declared in the manifest (mirrors
    /// `ShootingCommand::SetLocale` — see `shooting_ui`'s identical doc), so it skips enforcement.
    async fn command_id(command: &VcsCommand) -> &'static str {
        command.command_id()
    }

    async fn handle(
        command: &VcsCommand,
        doc: &ArtifactView<'_, VcsSnapshot>,
        cfg: &ConfigView<'_, VcsDemoConfig>,
        _interaction: &InteractionView<'_>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<VcsDemoMutation, VcsDemoConfigMutation, Self::DraftMutation>, Fault> {
        command.dispatch(doc, cfg)
    }

    async fn render(body_key: &str, doc: &ArtifactView<'_, VcsSnapshot>, cfg: &ConfigView<'_, VcsDemoConfig>) -> UiNode {
        let labels = vcs_play_labels(cfg.snapshot);
        match body_key {
            VCS_PLAY_BODY_EDITOR => editor::render(doc.snapshot, labels),
            VCS_PLAY_BODY_HISTORY => history::render(doc.history),
            VCS_PLAY_BODY_DOCUMENT => document_panel::render(doc.history, labels),
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
pub fn create_vcs_app() -> semio_framework_plugin::AppDefinition {
    Editor::builder(crate::artifacts::vcs::VCS_DIALECT)
            .document(["semio", "vcs"])
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
            .view_action("noMutation", LocalizedLabel::native("No-operation", "Keine Aktion"))
            .view_action("canvasPointerDown", LocalizedLabel::native("Canvas Pointer Down", "Leinwand-Zeiger gedrückt"))
            .view_action("canvasPointerMove", LocalizedLabel::native("Canvas Pointer Move", "Leinwand-Zeiger bewegt"))
            .view_action("canvasPointerUp", LocalizedLabel::native("Canvas Pointer Up", "Leinwand-Zeiger losgelassen"))
            .view_action("canvasWheel", LocalizedLabel::native("Canvas Wheel", "Leinwand-Mausrad"))
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo")
            .default_layout(edit::layout())
            // 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the "history" interaction
            // domain — one granularity ("commit"), `HierarchyProvider::Flat` (see `VCS_INTERACTION_HISTORY`'s
            // doc comment for why this is entity selection, not navigation, and why it is Flat). Multi-select
            // via Pick (tree rows) only — no canvas/marquee surface exists for checkpoints — all five merges
            // since the document tree is a plain ordered list (shift-range over the seeded history reads
            // naturally). Replaces the deleted bespoke `setSelection` action/config field/command.
            .interaction(InteractionDefinition {
                id: VCS_INTERACTION_HISTORY.into(),
                label: LocalizedLabel::native("History", "Verlauf"),
                granularities: vec![GranularityDefinition { id: "commit".into(), label: LocalizedLabel::native("Commit", "Commit"), icon_id: "git-commit".into() }],
                hierarchy: HierarchyProvider::Flat,
                hover: HoverSpec::default(),
                selection: SelectionSpec {
                    modes: vec![SelectionMode::Multiple, SelectionMode::Single],
                    methods: vec![SelectionMethod::Pick],
                    merges: vec![MergeMode::Replace, MergeMode::Additive, MergeMode::Subtractive, MergeMode::Invertive, MergeMode::Range],
                    transitive: false,
                    broadcast: true,
                },
            })
            .window_kind_interactions(history::VCS_PLAY_WINDOW_HISTORY, vec![InteractionRef::new(VCS_INTERACTION_HISTORY)])
            // 🎯️ Typed channel surface (HEADLESS-APP-ENGINE-BINARY-COMMAND-PROTOCOL-FOUNDATIONS Wave 1) —
            // this app has no user-visible sticky defaults, so `config_spec()` stays the trait default
            // `ConfigSpec::empty()`; declared anyway for parity with every other converted app.
            .config(VcsPlayApp::config_spec())
            // 🚧️ SDK GAP (contract §2.4): `Editor::builder`/`.editor::<E>(def: AppDefinition)` take a
            // bare `AppDefinition`, not the old `App { definition, examples }` — there is no
            // `.example(...)`/`.workflow(...)` on this builder, so this app never had either call to
            // port (the old `create_vcs_app` had none), noted here anyway for parity with the other W2
            // packets' identical gap note. The subset's own `📚️examples/🎬️demo-session` facet (real
            // content, moved intact) is the modern, role-agnostic replacement surface for this.
            .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Testkit
/// 🧪️ Shared test scaffolding for every taxonomy node's own `🧪️Tests` region — a component file must be
/// able to drive the whole app without re-deriving the harness.
#[cfg(test)]
pub(crate) mod testkit {
    use super::*;
    use semio_framework_plugin::testkit::{meta, new_app, new_app_with_registry};
    use semio_framework_plugin::{EditorApp, InvocationResult, PluginApp, VcsArtifactApp, ViewModel};
    use store::ArtifactEnvelope;

    /// ✏️ `VcsPlayApp` implements the AUTHORING trait `ArtifactEditor`, not the runtime `ArtifactApp`
    /// — `EditorApp<VcsPlayApp>` (SDK adapter, contract §2.1) is the real `ArtifactApp` implementor
    /// `VcsArtifactApp` wraps, exactly the way `PluginBuilder::editor::<VcsPlayApp>` builds it.
    pub type VcsApp = VcsArtifactApp<EditorApp<VcsPlayApp>>;

    /// ✏️ Adapts `create_vcs_app`'s `AppDefinition` (contract §2.4) into the `App { definition,
    /// examples }` shape `testkit::new_app_with_registry` still expects — framework testkit gap, not
    /// modifiable here (`🧰️framework/**` is outside this packet's lease).
    async fn vcs_app_manifest_for_testkit() -> semio_framework_plugin::App {
        semio_framework_plugin::App { definition: create_vcs_app(), examples: Vec::new() }
    }

    /// 🧪️ A bare, pre-seeded app instance — no `AppActionRegistry`, so undeclared internal commands
    /// dispatch freely. Seeded via `seed_vcs_demo_history` (see its own doc comment for why this
    /// replaced `ArtifactApp::seed`).
    pub async fn app() -> VcsApp {
        let mut instance = new_app::<EditorApp<VcsPlayApp>>();
        seed_vcs_demo_history(&mut instance);
        instance
    }

    /// 🧪️ A pre-seeded app wired to the real manifest registry — enforces View/Shell kind discipline.
    pub async fn app_with_registry() -> VcsApp {
        let mut instance = new_app_with_registry::<EditorApp<VcsPlayApp>>(vcs_app_manifest_for_testkit);
        seed_vcs_demo_history(&mut instance);
        instance
    }

    pub async fn dispatch(instance: &mut VcsApp, command: VcsCommand) -> InvocationResult {
        instance.dispatch_typed(command, &meta("local")).expect("dispatch")
    }

    pub async fn render(instance: &mut VcsApp, body_key: &str) -> String {
        serde_json::to_string(&instance.render(body_key, None, &ViewModel::default()).expect("render")).expect("render json")
    }

    /// 📦️ Parses `document_pack()` (the full envelope) for tests that need to inspect raw
    /// checkpoints/alternatives directly — safe here because none of these tests undo/redo, so every
    /// edit in the log is still applied.
    pub async fn seeded_envelope(instance: &VcsApp) -> ArtifactEnvelope<VcsSnapshot, VcsDemoMutation> {
        let files = instance.document_pack().expect("document pack");
        store::parse_document_pack::<VcsSnapshot, VcsDemoMutation>(&files.pack, &files.spr).expect("parse document pack").envelope
    }

    /// 🌱️ Seeds a rich, forked checkpoint/alternative history through `VcsApp`'s own public dispatch
    /// surface (`dispatch_typed`/`handle_action`) — this app's whole point is exercising the history UI
    /// (swimlane graph, checkpoints, alternatives, undo/redo), so every test instance starts as a
    /// populated history, not a bare projection. Replaces the old direct-`ArtifactStore`-touch
    /// `seed_vcs_demo_history(&mut ArtifactStore)` dispatched via the now-removed `ArtifactApp::seed`
    /// hook (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M4). Field edits go through
    /// `VcsCommand::TextEdit` (whole-projection diff, matching `patch::text_edit_operations`) so one
    /// call can bundle several field changes into one undo-log entry, mirroring the original narrative's
    /// grouping. Per-checkpoint authorship is lost here: `handle_action`'s `"commitCheckpoint"` arm
    /// hardcodes `authors: Vec::new()` with no wire path for real authors (framework-owned, out of this
    /// plugin's boundary) — no test asserts on authorship, so this is a silent, documented fidelity
    /// loss, not a functional gap.
    pub async fn seed_vcs_demo_history(app: &mut VcsApp) {
        let local = meta("local");
        let edit = |app: &mut VcsApp, f: fn(&mut VcsSnapshot)| {
            let mut next = app.snapshot().expect("materialize snapshot");
            f(&mut next);
            let text = serde_json::to_string(&next).expect("serialize snapshot");
            let _ = app.dispatch_typed(VcsCommand::TextEdit(text_edit::TextEdit { text }), &local);
        };
        let commit = |app: &mut VcsApp, message: &str| {
            let _ = app.handle_action("commitCheckpoint", Some(&serde_json::json!({ "message": message })), &local);
        };
        let checkout = |app: &mut VcsApp, checkpoint_id: &str| {
            let _ = app.handle_action("checkoutCheckpoint", Some(&serde_json::json!({ "checkpointId": checkpoint_id })), &local);
        };
        let create_alternative = |app: &mut VcsApp, name: &str| -> String {
            let _ = app.handle_action("createAlternative", Some(&serde_json::json!({ "name": name })), &local);
            seeded_envelope(app).active_alternative_id.clone().expect("alternative id")
        };
        let switch_alternative = |app: &mut VcsApp, alternative_id: &str| {
            let _ = app.handle_action("switchAlternative", Some(&serde_json::json!({ "alternativeId": alternative_id })), &local);
        };
        let last_checkpoint_id = |app: &VcsApp| -> String { seeded_envelope(app).vcs.checkpoints.last().expect("checkpoint just committed").id.clone() };

        edit(app, |s| {
            s.counter = 1;
            s.title = "VCS Demo".into();
        });
        commit(app, "Bootstrap");
        let c1 = last_checkpoint_id(app);

        edit(app, |s| {
            s.notes = "main line".into();
            s.status = "draft".into();
        });
        commit(app, "Annotate main draft");
        let c2 = last_checkpoint_id(app);

        edit(app, |s| {
            s.counter = 2;
        });
        commit(app, "Main milestone");
        let c3 = last_checkpoint_id(app);

        checkout(app, &c3);
        let feature_a_id = create_alternative(app, "feature-a");
        edit(app, |s| {
            s.title = "Feature A".into();
            s.tags.push("feature-a".into());
        });
        commit(app, "Start feature A");
        let c4 = last_checkpoint_id(app);

        edit(app, |s| {
            s.counter = 10;
        });
        commit(app, "Feature A progress");

        checkout(app, &c3);
        let feature_b_id = create_alternative(app, "feature-b");
        edit(app, |s| {
            s.title = "Feature B".into();
            s.notes = "branch b".into();
        });
        commit(app, "Start feature B");

        edit(app, |s| {
            s.counter = 20;
        });
        commit(app, "Feature B try");

        checkout(app, &c3);
        edit(app, |s| {
            s.status = "active".into();
        });
        commit(app, "Resume main");
        let c8 = last_checkpoint_id(app);

        switch_alternative(app, &feature_a_id);
        edit(app, |s| {
            s.counter = 11;
            s.tags.push("wip".into());
        });
        commit(app, "Feature A sprint");

        checkout(app, &c4);
        let _ = create_alternative(app, "feature-a-hotfix");
        edit(app, |s| {
            s.status = "hotfix".into();
        });
        commit(app, "Hotfix off feature A");

        switch_alternative(app, &feature_b_id);
        edit(app, |s| {
            s.tags.push("review".into());
        });
        commit(app, "Feature B review");

        checkout(app, &c8);
        edit(app, |s| {
            s.counter = 3;
            s.notes = "main polish".into();
            s.tags.push("release".into());
        });
        commit(app, "Main batch polish");

        edit(app, |s| {
            s.status = "done".into();
        });
        commit(app, "Main release");

        checkout(app, &c2);
        let _ = create_alternative(app, "docs");
        edit(app, |s| {
            s.notes = "documentation pass".into();
        });
        commit(app, "Docs branch");

        checkout(app, &c1);
        let _ = create_alternative(app, "spike");
        edit(app, |s| {
            s.title = "Spike prototype".into();
        });
        commit(app, "Spike experiment");
    }
}
//#endregion 🧪️Testkit

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::vcs::testkit::{app, dispatch, seeded_envelope};
    use semio_framework_plugin::testkit::meta;
    use semio_framework_plugin::PluginApp;
    use store::HistoryColumn;

    //#region 🔖️CommandSurface
    /// 🏷️ Every declared manifest action id must be reachable as exactly one command row, and every row's
    /// wire keyword must be distinct — the cross-cutting invariant `app_commands!` is there to hold.
    #[semio_framework_async_macros::async_test]
    async fn command_ids_are_unique_and_match_the_declared_manifest_actions() {
        let commands = every_command();
        let ids: Vec<&str> = commands.iter().map(|command| command.command_id()).collect();
        let mut sorted = ids.clone();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), ids.len(), "duplicate command ids in {ids:?}");
        assert_eq!(ids.len(), 10, "every VcsCommand row must be covered by every_command()");
    }

    /// ⚖️ LAW: text and binary are two projections of the same command, for every single row.
    #[semio_framework_async_macros::async_test]
    async fn every_command_round_trips_through_text_and_binary() {
        for command in every_command() {
            store::os_store::test_support::assert_op_text_binary_equivalence(&command);
        }
    }

    /// ⚖️ LAW: the leading token of every printed op line is the row's `dsl` wire keyword — the
    /// kebab-cased command id, except for the one documented divergence (`setLocale` → `locale`, an
    /// undeclared host-pushed command). This is what a missing `#[dsl(keyword = ..)]` on a payload struct
    /// silently breaks (the record prints with no keyword at all and no longer parses).
    #[semio_framework_async_macros::async_test]
    async fn every_printed_op_line_starts_with_the_rows_wire_keyword() {
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
    pub(super) async fn every_command() -> Vec<VcsCommand> {
        vec![
            VcsCommand::IncrementCounter(increment_counter::IncrementCounter {}),
            VcsCommand::PatchSnapshot(patch_snapshot::PatchSnapshot { field: "title".into(), value: "Renamed".into() }),
            VcsCommand::TextEdit(text_edit::TextEdit { text: "{}".into() }),
            VcsCommand::Edit(edit_command::Edit { text: "{}".into() }),
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
    #[semio_framework_async_macros::async_test]
    async fn the_manifest_stitches_every_taxonomy_node() {
        let json = serde_json::to_string(&create_vcs_app()).expect("app definition json");
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
    #[semio_framework_async_macros::async_test]
    async fn registry_enforced_app_dispatches_a_declared_action() {
        use crate::editor::vcs::testkit::app_with_registry;
        let mut instance = app_with_registry();
        let before = instance.snapshot().expect("materialize snapshot").counter;
        dispatch(&mut instance, VcsCommand::IncrementCounter(increment_counter::IncrementCounter {}));
        assert_eq!(instance.snapshot().expect("materialize snapshot").counter, before + 1);
    }
    //#endregion 🔖️ManifestSanity

    //#region 🔖️Interaction
    /// 🕹️ The "history" domain is declared `HierarchyProvider::Flat`, Pick-only, and scoped to the
    /// history window kind — see `VCS_INTERACTION_HISTORY`'s doc comment for why this is entity
    /// selection over checkpoints, not the per-row `checkoutCheckpoint`/`switchAlternative` navigation.
    #[semio_framework_async_macros::async_test]
    async fn history_interaction_domain_is_declared_flat_and_scoped_to_the_history_window() {
        let definition = create_vcs_app();
        let history_domain = definition.interactions.iter().find(|interaction| interaction.id == VCS_INTERACTION_HISTORY).expect("history interaction domain declared");
        assert!(matches!(history_domain.hierarchy, HierarchyProvider::Flat));
        assert!(!history_domain.selection.transitive, "checkpoints have no selectable-entity nesting");
        assert_eq!(history_domain.granularities.len(), 1);
        assert_eq!(history_domain.granularities[0].id, "commit");
        let history_window = definition.window_kinds.iter().find(|window| window.id == history::VCS_PLAY_WINDOW_HISTORY).expect("history window kind declared");
        assert!(history_window.interactions.iter().any(|interaction_ref| interaction_ref.as_str() == VCS_INTERACTION_HISTORY), "history window must reference the history interaction domain");
        let editor_window = definition.window_kinds.iter().find(|window| window.id == editor::VCS_PLAY_WINDOW_EDITOR).expect("editor window kind declared");
        assert!(editor_window.interactions.is_empty(), "the editor window has no checkpoint tree, so no interaction domain is scoped to it");
    }
    //#endregion 🔖️Interaction

    //#region 🔖️CrossCutting
    #[semio_framework_async_macros::async_test]
    async fn seeded_history_has_checkpoints() {
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

    #[semio_framework_async_macros::async_test]
    async fn checkout_then_commit_forks_across_actions() {
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

    #[semio_framework_async_macros::async_test]
    async fn undo_redo_round_trips_through_the_wrapper() {
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

    #[semio_framework_async_macros::async_test]
    async fn create_and_switch_alternative_round_trip_through_the_wrapper() {
        let mut instance = app();
        let create = instance.handle_action("createAlternative", Some(&serde_json::json!({ "name": "trying-something" })), &meta("local")).expect("create alternative");
        assert!(create.mutations.is_empty());
        let envelope = seeded_envelope(&instance);
        assert!(envelope.active_alternative_id.is_some(), "createAlternative must set an active alternative");
    }
    //#endregion 🔖️CrossCutting
}
//#endregion 🧪️Tests
