//! ✏️ Playground editor — the `ArtifactEditor` impl (dispatch-only), the one aggregated command and
//! the manifest stitch (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET). Playground's whole
//! persistent snapshot is one opaque `schema` metadata string (a demonstrator stub with no other
//! structured content today — see `🧬️schema/🧬️mutations/🦀️component.rs`'s own doc comment), so this
//! surface authors exactly one command over one `TextWindowKit` window rather than the larger
//! command/panel/config taxonomy a migrated app tree carries. `Config`/`Presence`/`Transient` are the
//! framework's `NoConfig`/`NoPresence`/`NoTransient` — a single-field metadata document needs no
//! persisted per-session view state.

use crate::artifacts::playground::standards::v1::subsets::any::schema::empty_playground_snapshot;
use crate::artifacts::playground::standards::v1::subsets::any::schema::mutations::PlaygroundMutation;
use crate::artifacts::playground::standards::v1::subsets::any::schema::snapshot::PlaygroundSnapshot;
use crate::artifacts::playground::{PLAYGROUND_DIALECT, PLAYGROUND_DOCUMENT_SCHEMA};
use crate::editor::playground::commands::change_schema;
use crate::editor::playground::modes::edit;
use crate::editor::playground::modes::edit::windows::main;
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::{
    ArtifactEditor, ArtifactView, ComponentTree, ConfigView, Dialect, DraftView, Editor, Emit, Fault, Label, LocalizedLabel, NoConfig, NoConfigMutation, NoDraft, NoDraftMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, UiAssemblyResult,
};
use serde_json::Value;
use store::EngineHandles;

//#region 🔖️Commands
semio_framework_plugin::app_commands! {
    /// 🎯️ `PlaygroundEditor::Command` — one row, the document's one mutation kind.
    pub enum PlaygroundCommand for PlaygroundSnapshot, PlaygroundMutation, NoConfig, NoConfigMutation {
        "changeSchema" as "change-schema" => change_schema::ChangeSchema,
    }
}
//#endregion 🔖️Commands

//#region 🔖️PlaygroundEditor
#[derive(Default)]
pub struct PlaygroundEditor;

impl ArtifactEditor for PlaygroundEditor {
    type Snapshot = PlaygroundSnapshot;
    type Mutation = PlaygroundMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = PlaygroundCommand;

    const DIALECT: Dialect = PLAYGROUND_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = PLAYGROUND_DOCUMENT_SCHEMA;

    async fn initial_snapshot() -> PlaygroundSnapshot {
        empty_playground_snapshot()
    }

    async fn command_id(command: &PlaygroundCommand) -> &'static str {
        command.command_id()
    }

    /// 🗺️ Maps the manifest `changeSchema` action (declared via `.mutation(...)` below) to the one
    /// typed command row — the same shape `gis2d`'s `command_from_action` uses for its own rows.
    async fn command_from_action(action: &str, args: Option<&Value>) -> Result<PlaygroundCommand, Fault> {
        let args = args.cloned().unwrap_or(Value::Null);
        match action {
            "changeSchema" => Ok(PlaygroundCommand::ChangeSchema(change_schema::ChangeSchema { new_schema: args.get("newSchema").or_else(|| args.get("new_schema")).and_then(Value::as_str).unwrap_or_default().to_string() })),
            other => Err(Fault::from(format!(
                "action '{other}' is not a framework-reserved action (history/clipboard/revert/filter/noteShellCommand) — \
                 app actions are dispatched exclusively through the typed command channel now (see `dispatch_typed_command`)"
            ))),
        }
    }

    async fn handle(
        command: &PlaygroundCommand,
        doc: &ArtifactView<'_, PlaygroundSnapshot>,
        cfg: &ConfigView<'_, NoConfig>,
        _interaction: &InteractionView<'_>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<PlaygroundMutation, NoConfigMutation, Self::DraftMutation>, Fault> {
        command.dispatch(doc, cfg)
    }

    async fn render(body_key: &str, doc: &ArtifactView<'_, PlaygroundSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> UiAssemblyResult<ComponentTree> {
        match body_key {
            main::BODY_KEY => main::render(doc.snapshot).map(semio_framework_plugin::built_to_component_tree),
            _ => semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️PlaygroundEditor

//#region 🔖️Manifest
/// 🚧️ SDK GAP (pilot `📓️w2-cad-report.md`, confirmed still open by `📓️w2-p8-report.md`):
/// `.example(...)`/`.workflow(...)` do not exist on `EditorBuilder` — playground never registered
/// either, so nothing is dropped here (unlike migrated packets which had real examples to lose).
pub fn create_playground_editor() -> semio_framework_plugin::AppDefinition {
    Editor::builder(PLAYGROUND_DIALECT)
        .document(["semio", "playground"])
        .icon_id("playground")
        .mode_def(edit::definition())
        .default_mode_id(edit::PLAYGROUND_EDIT_MODE_EDIT)
        .window_kind_def(main::definition())
        .default_layout(edit::layout())
        .mutation("changeSchema", LocalizedLabel::native("Change Schema", "Schema ändern"))
        .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Testkit
#[cfg(test)]
pub mod testkit {
    //! 🧪️ `testkit::assert_declared_actions_bridge_to_commands`'s signature is still
    //! `fn(manifest: fn() -> App)` (framework testkit gap, `📓️w0-f-report.md` Gap 3) — `App { definition,
    //! examples }` shape kept alive here purely to satisfy that call.
    use super::create_playground_editor;
    use semio_framework_plugin::App;

    pub fn playground_editor_manifest_for_testkit() -> App {
        App { definition: create_playground_editor(), examples: Vec::new() }
    }
}
//#endregion 🧪️Testkit

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_plugin::{EditorApp, HistoryView};

    #[test]
    fn create_playground_editor_builds_a_definition_for_the_editor_role() {
        let def = create_playground_editor();
        assert_eq!(def.role, semio_framework_plugin::AppRole::Editor);
        assert_eq!(def.dialect, PLAYGROUND_DIALECT.into());
    }

    #[test]
    fn editor_dialect_matches_the_artifact_coordinate() {
        assert_eq!(<PlaygroundEditor as ArtifactEditor>::DIALECT, PLAYGROUND_DIALECT);
    }

    #[test]
    fn change_schema_command_mutates_the_schema_field() {
        let document = empty_playground_snapshot();
        let history = HistoryView::empty();
        let doc = ArtifactView::new(&document, &history);
        let config = NoConfig::default();
        let cfg = ConfigView { snapshot: &config };
        let command = PlaygroundCommand::ChangeSchema(change_schema::ChangeSchema { new_schema: "playground.custom".into() });
        let emit = command.dispatch(&doc, &cfg).expect("dispatch");
        assert_eq!(emit.artifact_mutations, vec![PlaygroundMutation::ChangeSchema(crate::artifacts::playground::standards::v1::subsets::any::schema::mutations::change_schema::mutation::ChangeSchema { new_schema: "playground.custom".into() })]);
    }

    #[test]
    fn command_from_action_covers_the_declared_action_and_rejects_unknown_ones() {
        semio_framework_plugin::testkit::assert_declared_actions_bridge_to_commands::<EditorApp<PlaygroundEditor>>(testkit::playground_editor_manifest_for_testkit);
        assert!(semio_framework_plugin::resolve_ready(PlaygroundEditor::command_from_action("noSuchAction", None)).is_err());
    }
}
//#endregion 🧪️Tests
