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
use semio_framework::{ToolExecutionContract, ToolFactoryKey, ToolJobFactory, ToolJobFactoryError};
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::retained_command::{ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload, BoundedArtifactCommandWork};
use semio_framework_plugin::{
    AppOperationContext, ArtifactEditor, ArtifactOwnedToolJobRequest, ArtifactToolFactoryRegistry, ArtifactView, ComponentTree, ConfigView, Dialect, DraftView, Editor, EditorApp, Emit, Fault, InteractiveJobClassification, Label, LocalizedLabel,
    NoConfig, NoConfigMutation, NoDraft, NoDraftMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, UiAssemblyResult,
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

//#region 🧵️RetainedCommands
const PLAYGROUND_RETAINED_TOOL_IDS: &[&str] = &["changeSchema"];
const PLAYGROUND_RETAINED_PAYLOAD_SCHEMA: &str = "playground.playground.tool-command.v1";
const PLAYGROUND_RETAINED_RAW_BYTES: usize = 8_192;
const PLAYGROUND_RETAINED_WORK_ITEMS: usize = 1;

fn playground_retained_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(PLAYGROUND_RETAINED_RAW_BYTES, 32, 32, 16_384, 7_500)
}

fn playground_retained_extent(command: &PlaygroundCommand, _snapshot: &PlaygroundSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    match command {
        PlaygroundCommand::ChangeSchema(payload) if payload.new_schema.len() <= PLAYGROUND_RETAINED_RAW_BYTES => Some(1),
        PlaygroundCommand::ChangeSchema(_) => None,
    }
}

fn playground_retained_reduce(
    command: &PlaygroundCommand,
    snapshot: &PlaygroundSnapshot,
    config: &NoConfig,
    history: &semio_framework_plugin::HistoryView,
    _interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    operation: &AppOperationContext,
) -> Result<Emit<PlaygroundMutation, NoConfigMutation, NoDraftMutation>, Fault> {
    command.dispatch(&ArtifactView::with_operation(snapshot, history, operation.clone()), &ConfigView { snapshot: config })
}

struct PlaygroundCommandJobFactory {
    keys: Vec<ToolFactoryKey>,
}

impl PlaygroundCommandJobFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: PLAYGROUND_RETAINED_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl semio_framework::ToolJobFactory for PlaygroundCommandJobFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<PlaygroundEditor>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<PlaygroundEditor>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        PLAYGROUND_RETAINED_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> ToolExecutionContract {
        playground_retained_contract()
    }

    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> {
        Ok(ArtifactRetainedCommandJob::new(payload))
    }

    fn create_job_from_wire_pages_with_payload(
        &mut self,
        _operation: semio_framework_job::Operation,
        payload: Self::Payload,
        input: semio_framework::action_bus::RetainedToolWireInput,
        checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>,
    ) -> Result<Self::Job, (ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
        if input.declared_bytes() > PLAYGROUND_RETAINED_RAW_BYTES || checkpoint.is_some() {
            return Err((ToolJobFactoryError::new("Playground bounded command rejects oversized wire or checkpoint owner"), input, checkpoint));
        }
        Ok(ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}

impl semio_framework_plugin::ArtifactOwnedToolJobFactory for PlaygroundCommandJobFactory {
    type Owner = semio_framework_plugin::EditorApp<PlaygroundEditor>;
    const TOOL_IDS: &'static [&'static str] = PLAYGROUND_RETAINED_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = PLAYGROUND_DOCUMENT_SCHEMA;
}
//#endregion 🧵️RetainedCommands

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

    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: semio_framework_plugin::EditorApp<PlaygroundEditor>,
        owner_file: "✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs",
        controller: "s.demonstrator.playground@1/*#editor",
        document_schema: "playground.playground",
        factory: "PlaygroundCommandJobFactory",
        tools: {
            "changeSchema" => semio_framework::ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500),
        }
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(PlaygroundCommandJobFactory::new(&controller))
    }

    fn build_tool_job(request: ArtifactOwnedToolJobRequest<EditorApp<Self>>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
        if !PLAYGROUND_RETAINED_TOOL_IDS.contains(&request.tool_id.as_str()) {
            return Ok(None);
        }
        if request.command.command_id() != request.tool_id {
            return Err(Fault::from("playground-command-tool-mismatch"));
        }
        if playground_retained_extent(&request.command, &request.snapshot, &request.interaction_state).is_none() {
            return Err(Fault::from("playground-command-payload-too-large"));
        }
        let tool_id = request.command.command_id();
        let work = Box::new(BoundedArtifactCommandWork::new(tool_id, playground_retained_reduce, playground_retained_extent));
        let operation_context = AppOperationContext {
            app_instance_id: request.app_instance_id,
            parent_document_id: request.parent_document_id.clone(),
            operation_id: request.operation.operation.0,
            generation: request.operation.generation.0,
            canonical_base_revision: request.canonical_base_revision,
        };
        let payload = ArtifactRetainedCommandPayload::try_new_with_context(
            *request.command,
            request.snapshot,
            request.config,
            request.history,
            request.interaction_state,
            request.interaction_hover,
            request.context,
            operation_context,
            request.completion,
            PlaygroundCommand::command_id,
            PLAYGROUND_RETAINED_RAW_BYTES,
            PLAYGROUND_RETAINED_WORK_ITEMS,
            work,
        )?;
        Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }

    fn initial_snapshot() -> PlaygroundSnapshot {
        empty_playground_snapshot()
    }

    fn command_id(command: &PlaygroundCommand) -> &'static str {
        command.command_id()
    }

    /// 🗺️ Maps the manifest `changeSchema` action (declared via `.mutation(...)` below) to the one
    /// typed command row — the same shape `gis2d`'s `command_from_action` uses for its own rows.
    fn command_from_action(action: &str, args: Option<&Value>) -> Result<PlaygroundCommand, Fault> {
        let args = args.cloned().unwrap_or(Value::Null);
        match action {
            "changeSchema" => {
                let new_schema = args.get("newSchema").or_else(|| args.get("new_schema")).and_then(Value::as_str).unwrap_or_default();
                if new_schema.len() > PLAYGROUND_RETAINED_RAW_BYTES {
                    return Err(Fault::from("playground-command-payload-too-large"));
                }
                Ok(PlaygroundCommand::ChangeSchema(change_schema::ChangeSchema { new_schema: new_schema.to_string() }))
            }
            other => Err(Fault::from(format!(
                "action '{other}' is not a framework-reserved action (history/clipboard/revert/filter/noteShellCommand) — \
                 app actions are dispatched exclusively through the typed command channel now (see `dispatch_typed_command`)"
            ))),
        }
    }

    fn handle(
        command: &PlaygroundCommand,
        doc: &ArtifactView<'_, PlaygroundSnapshot>,
        cfg: &ConfigView<'_, NoConfig>,
        _interaction: &InteractionView<'_>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<PlaygroundMutation, NoConfigMutation, Self::DraftMutation>, Fault> {
        command.dispatch(doc, cfg)
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, PlaygroundSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> UiAssemblyResult<ComponentTree> {
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
        .action_interactive_job("changeSchema", InteractiveJobClassification::Migrated)
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

    const RETAINED_LIMITS: &str = include_str!("🧪️fixtures/🎯️retained-command-limits.json");

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
    fn change_schema_factory_declares_the_exact_bounded_contract() {
        let factory = PlaygroundCommandJobFactory::new("s.demonstrator.playground@1/*#editor");
        assert_eq!(factory.keys(), &[ToolFactoryKey::new("s.demonstrator.playground@1/*#editor", "changeSchema")]);
        assert_eq!(factory.payload_schema_id(), PLAYGROUND_RETAINED_PAYLOAD_SCHEMA);
        assert_eq!(factory.execution_contract(), ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500));
    }

    #[test]
    fn change_schema_admission_matches_the_language_neutral_limit_oracle() {
        let fixture: Value = serde_json::from_str(RETAINED_LIMITS).expect("retained command limits decode through serde_json");
        let maximum = fixture.get("maximumSchemaBytes").and_then(Value::as_u64).expect("maximumSchemaBytes") as usize;
        let additional = fixture.get("rejectedAdditionalBytes").and_then(Value::as_u64).expect("rejectedAdditionalBytes") as usize;
        let expected_items = fixture.get("expectedWorkItems").and_then(Value::as_u64).expect("expectedWorkItems") as usize;
        assert_eq!(maximum, PLAYGROUND_RETAINED_RAW_BYTES);
        assert_eq!(expected_items, PLAYGROUND_RETAINED_WORK_ITEMS);
        let accepted = PlaygroundCommand::ChangeSchema(change_schema::ChangeSchema { new_schema: "s".repeat(maximum) });
        let rejected = PlaygroundCommand::ChangeSchema(change_schema::ChangeSchema { new_schema: "s".repeat(maximum + additional) });
        let snapshot = empty_playground_snapshot();
        let interaction = protocol::InteractionState::default();
        assert_eq!(playground_retained_extent(&accepted, &snapshot, &interaction), Some(expected_items));
        assert_eq!(playground_retained_extent(&rejected, &snapshot, &interaction), None);
        assert!(semio_framework_plugin::resolve_ready(PlaygroundEditor::command_from_action("changeSchema", Some(&serde_json::json!({ "newSchema": "s".repeat(maximum) })))).is_ok());
        assert!(semio_framework_plugin::resolve_ready(PlaygroundEditor::command_from_action("changeSchema", Some(&serde_json::json!({ "newSchema": "s".repeat(maximum + additional) })))).is_err());
    }

    #[semio_framework_async_macros::async_test]
    async fn change_schema_command_mutates_the_schema_field() {
        let document = empty_playground_snapshot();
        let history = HistoryView::empty().await;
        let doc = ArtifactView::new(&document, &history).await;
        let config = NoConfig::default();
        let cfg = ConfigView { snapshot: &config };
        let command = PlaygroundCommand::ChangeSchema(change_schema::ChangeSchema { new_schema: "playground.custom".into() });
        let emit = command.dispatch(&doc, &cfg).expect("dispatch");
        assert_eq!(emit.artifact_mutations, vec![PlaygroundMutation::ChangeSchema(crate::artifacts::playground::standards::v1::subsets::any::schema::mutations::change_schema::mutation::ChangeSchema { new_schema: "playground.custom".into() })]);
    }

    #[semio_framework_async_macros::async_test]
    async fn registry_backed_editor_installs_its_exact_bounded_command_proof() {
        let _app = semio_framework_plugin::testkit::new_app_with_registry::<EditorApp<PlaygroundEditor>>(testkit::playground_editor_manifest_for_testkit).await;
    }

    #[test]
    fn command_from_action_covers_the_declared_action_and_rejects_unknown_ones() {
        semio_framework_plugin::testkit::assert_declared_actions_bridge_to_commands::<EditorApp<PlaygroundEditor>>(testkit::playground_editor_manifest_for_testkit);
        assert!(semio_framework_plugin::resolve_ready(PlaygroundEditor::command_from_action("noSuchAction", None)).is_err());
    }
}
//#endregion 🧪️Tests
