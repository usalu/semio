//! ✏️ Playground editor — the `ArtifactEditor` impl (dispatch-only), the one aggregated command and
//! the manifest stitch (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET). Playground's whole
//! persistent snapshot is one opaque `schema` metadata string (a demonstrator stub with no other
//! structured content today — see `🧬️schema/🧬️mutations/🦀️.rs`'s own doc comment), so this
//! surface authors exactly one command over one `TextWindowKit` window rather than the larger
//! command/panel/config taxonomy a migrated app tree carries. `Config`/`Presence`/`Transient` are the
//! framework's `NoConfig`/`NoPresence`/`NoTransient` — a single-field metadata document needs no
//! persisted per-session view state.

use crate::editor::playground::commands::{change_schema, set_active_example};
use crate::editor::playground::modes::edit;
use crate::editor::playground::modes::edit::windows::main;
use crate::standards::v1::subsets::any::schema::empty_playground_snapshot;
use crate::standards::v1::subsets::any::schema::mutations::PlaygroundMutation;
use crate::standards::v1::subsets::any::schema::snapshot::PlaygroundSnapshot;
use crate::{PLAYGROUND_DIALECT, PLAYGROUND_DOCUMENT_SCHEMA};
use semio_framework::{ToolExecutionContract, ToolFactoryKey, ToolJobFactory, ToolJobFactoryError};
use semio_framework_2d_compute::EngineHandles;
#[cfg(test)]
use semio_framework_pack_json::{parse, Value};
use semio_plugin::app::InteractionView;
use semio_plugin::retained_command::{ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload, BoundedArtifactCommandWork};
use semio_plugin::AppOperationContext;
use semio_plugin::ArtifactEditor;
use semio_plugin::ArtifactOwnedToolJobRequest;
use semio_plugin::ArtifactToolFactoryRegistry;
use semio_plugin::ArtifactToolPublicationContract;
use semio_plugin::ArtifactToolPublicationLane;
use semio_plugin::ArtifactView;
use semio_plugin::ComponentTree;
use semio_plugin::ConfigView;
use {semio_framework_artifact_reference::Dialect};
use semio_plugin::DraftView;
use semio_plugin::Editor;
use semio_plugin::EditorApp;
use semio_plugin::Emit;
use semio_plugin::Fault;
use semio_plugin::InteractiveJobClassification;
use semio_plugin::NoConfig;
use semio_plugin::NoConfigMutation;
use semio_plugin::NoDraft;
use semio_plugin::NoDraftMutation;
use semio_plugin::NoPresence;
use semio_plugin::NoPresenceMutation;
use semio_plugin::NoTransient;
use semio_plugin::NoTransientMutation;
use semio_plugin::UiAssemblyResult;
use semio_ui_locale::Label;
use semio_ui_locale::LocalizedLabel;

//#region 🔖️Commands
semio_plugin::app_commands! {
    /// 🎯️ `PlaygroundEditor::Command` — one row, the document's one mutation kind.
    pub enum PlaygroundCommand for PlaygroundSnapshot, PlaygroundMutation, NoConfig, NoConfigMutation {
        "changeSchema" as "change-schema" => change_schema::ChangeSchema,
        "setActiveExample" as "active-example" => set_active_example::SetActiveExample,
    }
}
//#endregion 🔖️Commands

//#region 🔖️PlaygroundEditor
#[derive(Default)]
pub struct PlaygroundEditor;

//#region 🧵️RetainedCommands
const PLAYGROUND_RETAINED_TOOL_IDS: &[&str] = &["changeSchema", "setActiveExample"];
pub const PLAYGROUND_RETAINED_PAYLOAD_SCHEMA: &str = "playground.playground.tool-command.v1";
pub const PLAYGROUND_RETAINED_RAW_BYTES: usize = 8_192;
pub const PLAYGROUND_RETAINED_WORK_ITEMS: usize = 1;

fn playground_retained_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(PLAYGROUND_RETAINED_RAW_BYTES, 32, 32, 16_384, 7_500)
}

pub fn playground_retained_extent(command: &PlaygroundCommand, _snapshot: &PlaygroundSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    match command {
        PlaygroundCommand::ChangeSchema(payload) if payload.new_schema.len() <= PLAYGROUND_RETAINED_RAW_BYTES => Some(1),
        PlaygroundCommand::ChangeSchema(_) => None,
        PlaygroundCommand::SetActiveExample(_) => Some(1),
    }
}

#[expect(clippy::too_many_arguments, reason = "Implements the framework ArtifactCommandReducer callback signature.")]
fn playground_retained_reduce(
    command: &PlaygroundCommand,
    snapshot: &PlaygroundSnapshot,
    config: &NoConfig,
    history: &semio_plugin::HistoryView,
    _interaction: &protocol::InteractionState,
    _hover: &semio_plugin::app::InteractionHoverState,
    _context: Option<&semio_plugin::app::ArtifactOwnedToolJobContext<EditorApp<PlaygroundEditor>>>,
    operation: &AppOperationContext,
) -> Result<Emit<PlaygroundMutation, NoConfigMutation, NoDraftMutation>, Fault> {
    command.dispatch(&ArtifactView::with_operation(snapshot, history, operation.clone()), &ConfigView { snapshot: config, window: None })
}

pub struct PlaygroundCommandJobFactory {
    keys: Vec<ToolFactoryKey>,
}

impl PlaygroundCommandJobFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: PLAYGROUND_RETAINED_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl ToolJobFactory for PlaygroundCommandJobFactory {
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

    semio_plugin::retained_command::retained_command_factory_admission!(maximum_raw_bytes: PLAYGROUND_RETAINED_RAW_BYTES, checkpoint_resume: false);

}

impl semio_plugin::ArtifactOwnedToolJobFactory for PlaygroundCommandJobFactory {
    type Owner = EditorApp<PlaygroundEditor>;
    const TOOL_IDS: &'static [&'static str] = PLAYGROUND_RETAINED_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = PLAYGROUND_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] =
        &[ArtifactToolPublicationContract { tool_id: "changeSchema", lanes: &[ArtifactToolPublicationLane::Artifact] }, ArtifactToolPublicationContract { tool_id: "setActiveExample", lanes: &[ArtifactToolPublicationLane::Artifact] }];
}
//#endregion 🧵️RetainedCommands

//#region 📬️StorePreparation
const PLAYGROUND_STORE_MAXIMUM_BYTES: usize = 8_192;

/// 🏭️ Every Playground mutation is prepared by the shared cold apply over its own cold semantics, within the fixed 8 KiB schema envelope of mutation and base.
fn playground_store_preparation_factory() -> std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<PlaygroundSnapshot, PlaygroundMutation>> {
    store::bounded_cold_mutation_apply_preparation_factory::<PlaygroundSnapshot, PlaygroundMutation>(store::ColdPreparationBounds {
        maximum_bytes: PLAYGROUND_STORE_MAXIMUM_BYTES,
        mutation_bytes: |mutation| match mutation { PlaygroundMutation::ChangeSchema(payload) => payload.new_schema.len() },
        snapshot_bytes: |snapshot| snapshot.schema.len(),
    })
}
//#endregion 📬️StorePreparation

impl ArtifactEditor for PlaygroundEditor {
    /// 📚️ Artifact catalogue stamped by `PluginBuilder::editor` onto the navbar dropdown.
    fn examples() -> Vec<semio_plugin::ExampleSource> {
        vec![crate::examples::demo::source()]
    }
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

    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(playground_store_preparation_factory())
    }

    /// ♻️ The exact store owners and disposers every lane of this editor retires through. The app
    /// declared none, so `changeSchema` — its ONE mutation — reached the typed operation and died
    /// there with `dispatch-failed … returned snapshot read requires its exact owned-snapshot
    /// retirement factory` (measured headless on :6107, `🗑️generated/b3a-demonstrator/report.json`),
    /// which then poisoned the shell into `data-semio-os-error="demonstrator"`. Same defect and same
    /// cure as 📖️playbook's.
    fn build_document_store_disposer() -> Option<Box<dyn semio_plugin::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
        Some(semio_plugin::bounded_document_store_disposer::<Self::Snapshot, Self::Mutation>())
    }

    fn build_config_store_disposer() -> Option<Box<dyn semio_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::Config, Self::ConfigMutation>>>> {
        Some(semio_plugin::no_config_store_disposer())
    }

    fn build_draft_store_disposer() -> Option<Box<dyn semio_plugin::ArtifactOwnedDisposer<store::DraftStore<Self::Draft, Self::DraftMutation>>>> {
        Some(semio_plugin::no_draft_store_disposer())
    }

    fn build_presence_store_disposer() -> Option<Box<dyn semio_plugin::ArtifactOwnedDisposer<store::PresenceStore<Self::Presence, Self::PresenceMutation>>>> {
        Some(semio_plugin::no_presence_store_disposer())
    }

    /// 👥️ The disposer alone does not close the presence store: its close loop demands the INSTALLED
    /// local-root retirement factory (`presence close requires its installed local-root retirement
    /// factory`), and the `ArtifactEditor` default is `None` — `EditorApp` passes it through unchanged.
    fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_plugin::no_presence_local_root_retirement_factory())
    }

    fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_plugin::no_presence_peer_retirement_factory())
    }

    fn build_transient_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Transient>>> {
        Some(semio_plugin::no_transient_local_root_retirement_factory())
    }

    fn build_transient_store_disposer() -> Option<Box<dyn semio_plugin::ArtifactOwnedDisposer<store::TransientStore<Self::Transient, Self::TransientMutation>>>> {
        Some(semio_plugin::no_transient_store_disposer())
    }

    semio_plugin::bounded_first_step_tool_proofs! {
        owner: EditorApp<PlaygroundEditor>,
        owner_file: "✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.demonstrator.playground@1/*#editor",
        artifact_schema: "playground.playground",
        factory: "PlaygroundCommandJobFactory",
        factory_type: PlaygroundCommandJobFactory,
        tools: {
            "changeSchema" => ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500),
            "setActiveExample" => ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500),
        }
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(PlaygroundCommandJobFactory::new(&controller))
    }

    fn build_tool_job(request: &mut Option<ArtifactOwnedToolJobRequest<EditorApp<Self>>>, output: &mut Option<semio_plugin::ErasedToolJob>, cx: &mut semio_plugin::__job::StepContext<'_>) -> Result<bool, semio_plugin::__job::OriginalTurnFailure> {
        let original = request.as_ref().ok_or_else(|| semio_plugin::__value::ValueError::literal(semio_plugin::__value::ValueRefusalKind::InvariantViolated, "original retained producer request is absent"))?;
        if !PLAYGROUND_RETAINED_TOOL_IDS.contains(&original.tool_id.as_str()) {
            return Ok(false);
        }
        if original.command.command_id() != original.tool_id {
            return Err(semio_plugin::__value::ValueError::literal(semio_plugin::__value::ValueRefusalKind::InvariantViolated, "playground-command-tool-mismatch").into());
        }
        if playground_retained_extent(&original.command, &original.snapshot, &original.interaction_state).is_none() {
            return Err(Fault::from("playground-command-payload-too-large").into());
        }
        let tool_id = original.command.command_id();
        semio_plugin::retained_command::producer::admit_retained_tool_producer(request, output, cx, PlaygroundCommand::command_id, PLAYGROUND_RETAINED_RAW_BYTES, PLAYGROUND_RETAINED_WORK_ITEMS, false, |request| BoundedArtifactCommandWork::new(tool_id, playground_retained_reduce, playground_retained_extent))
    }

    fn reference_birth()->Option<fn()->Result<PlaygroundSnapshot,semio_plugin::Fault>>{Some(||{
        Ok(empty_playground_snapshot())
    })}

    fn command_id(command: &PlaygroundCommand) -> &'static str {
        command.command_id()
    }

    /// 🗺️ Maps the manifest `changeSchema` action (declared via `.mutation(...)` below) to the one
    /// typed command row — the same shape `gis2d`'s `command_from_action` uses for its own rows.
    fn command_from_action(action: &str, args: &mut Option<semio_framework_value::DslValue>, original_fault: &mut Option<Fault>) -> Result<PlaygroundCommand, Fault> {
        if let Some(fault) = original_fault.take() { return Err(fault); }
        let args = args.as_ref();
        let args = args.cloned().unwrap_or(semio_framework_value::DslValue::Null);
        match action {
            "changeSchema" => {
                let new_schema = args.get("newSchema").or_else(|| args.get("new_schema")).and_then(semio_framework_value::DslValue::as_str).unwrap_or_default();
                if new_schema.len() > PLAYGROUND_RETAINED_RAW_BYTES {
                    return Err(Fault::from("playground-command-payload-too-large"));
                }
                Ok(PlaygroundCommand::ChangeSchema(change_schema::ChangeSchema { new_schema: new_schema.to_string() }))
            }
            "setActiveExample" => {
                let example_id = args.get("exampleId").or_else(|| args.get("example_id")).and_then(semio_framework_value::DslValue::as_str).unwrap_or_default();
                Ok(PlaygroundCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: example_id.to_string() }))
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
        _view_state: Option<&semio_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<PlaygroundMutation, NoConfigMutation, Self::DraftMutation>, Fault> {
        command.dispatch(doc, cfg)
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, PlaygroundSnapshot>, _cfg: &ConfigView<'_, NoConfig>, view_state: &semio_plugin::ViewModel) -> UiAssemblyResult<ComponentTree> {
        match body_key {
            main::BODY_KEY => {
                let publication_revision = semio_plugin::app::artifact_render_publication_revision(doc)?;
                main::render(doc.snapshot, view_state.locale, publication_revision).map(semio_plugin::built_to_component_tree)
            }
            _ => semio_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️PlaygroundEditor

//#region 🔖️Manifest
/// 🚧️ SDK GAP (pilot `📓️w2-cad-report.md`, confirmed still open by `📓️w2-p8-report.md`):
/// `.example(...)`/`.workflow(...)` do not exist on `EditorBuilder` — playground never registered
/// either, so nothing is dropped here (unlike migrated packets which had real examples to lose).
pub fn create_playground_editor() -> semio_plugin::AppDefinition {
    Editor::builder(PLAYGROUND_DIALECT)
        .document(["semio", "playground"])
        .artifact_kind(crate::artifact_kind())
        .icon_id("playground")
        .mode_def(edit::definition())
        .default_mode_id(edit::PLAYGROUND_EDIT_MODE_EDIT)
        .window_kind_def(main::definition())
        .default_layout(edit::layout())
        .mutation("changeSchema", LocalizedLabel::native("Change Schema", "Schema ändern"))
        .action_interactive_job("changeSchema", InteractiveJobClassification::Migrated)
        .mutation("setActiveExample", LocalizedLabel::native("Set Active Example", "Beispiel setzen"))
        .action_interactive_job("setActiveExample", InteractiveJobClassification::Migrated)
        .action_destructive("setActiveExample")
        .action_describe(
            "changeSchema",
            LocalizedLabel::native(
                "Replaces the playground's schema metadata string, the whole persistent content of a playground document, with the given text.",
                "Ersetzt die Schema-Metadaten-Zeichenkette des Playgrounds, den gesamten dauerhaften Inhalt eines Playground-Dokuments, durch den angegebenen Text.",
            ),
        )
        .action_describe(
            "setActiveExample",
            LocalizedLabel::native("Loads the bundled demo playground's schema, or clears the schema for an empty example id.", "Lädt das Schema des mitgelieferten Demo-Playgrounds oder leert das Schema bei leerer Beispiel-Id."),
        )
        .action_destructive("changeSchema")
        .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️UnitTests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
pub(crate) mod unit_tests;
//#endregion 🧪️UnitTests

//#region 🧩️ArtifactItems2
pub mod artifact_items_2 {
use semio_framework_artifact_reference::{Dialect, StandardId, SubsetId};

//#region 🔖️Dialect
/// ⏲️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.1 — the one `Dialect`
/// coordinate `PlaygroundEditor`/`PlaygroundViewer` (`✏️editor`/`👁️viewer`) both read `DIALECT` off,
/// lives at the ARTIFACT root (not under either surface) so a viewer file can read it without ever
/// importing through the sibling editor module. The root, definition and native dialect share
/// one plugin-owned identity; the document payload schema remains independently named.
pub const PLAYGROUND_DIALECT: Dialect = Dialect { artifact_kind: "s.demonstrator.playground", standard: StandardId("1"), subset: SubsetId::ANY };
//#endregion 🔖️Dialect
}
//#endregion 🧩️ArtifactItems2

//#region 🧩️ArtifactItems3
pub mod artifact_items_3 {
use crate::{PLAYGROUND_DOCUMENT_SCHEMA};
use semio_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec`.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "playground.document".into(),
        label: semio_ui_locale::LocalizedLabel::native("Playground Document", "Playground-Dokument"),
        source_format: PLAYGROUND_DOCUMENT_SCHEMA.into(),
        component_kind: "playground".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        schema: PLAYGROUND_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec!["stdio.json".into(), "stdio.zip".into()],
        import_stdio_kinds: vec!["stdio.json".into(), "stdio.zip".into()],
    }
}
//#endregion 🔖️ArtifactKind
}
//#endregion 🧩️ArtifactItems3
