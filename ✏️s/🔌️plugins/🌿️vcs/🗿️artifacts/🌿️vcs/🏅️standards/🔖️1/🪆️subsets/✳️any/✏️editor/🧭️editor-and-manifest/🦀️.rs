//! 🧭️ Editor impl and manifest of the VCS play app: the dispatch-only artifact editor surface and the app definition that stitches every command, panel and window. See [`super`].

use super::*;
use semio_framework_2d_compute::EngineHandles;

//#region 🔖️VcsPlayApp
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

    const DIALECT: Dialect = crate::VCS_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = VCS_DOCUMENT_SCHEMA;

    fn fault_notices() -> &'static [(&'static str, LocalizedLabel)] {
        static NOTICES: std::sync::LazyLock<[(&str, LocalizedLabel); 1]> = std::sync::LazyLock::new(|| [("vcs.command.payload-too-large", LocalizedLabel::native("This edit is too large to apply in one step.", "Diese Änderung ist zu groß, um sie in einem Schritt anzuwenden."))]);
        NOTICES.as_slice()
    }

    /// 🧺️ Without these owners the document store holds no `initial_snapshot_retirement_factory`, so
    /// the FIRST verb that returns a snapshot read fails validation with `returned snapshot read
    /// requires its exact owned-snapshot retirement factory` — and because that poisons the
    /// instance, every later dispatch fails with `plugin.internal.prior-outcome`. Measured against
    /// the live React playground: all nine of this app's Actions-pane verbs were refused, the first
    /// on the missing factory and the other eight on the prior outcome.
    fn build_document_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
        Some(semio_framework_plugin::bounded_document_store_disposer::<Self::Snapshot, Self::Mutation>())
    }

    fn build_config_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::Config, Self::ConfigMutation>>>> {
        Some(semio_framework_plugin::bounded_config_store_disposer::<Self::Config, Self::ConfigMutation>())
    }

    /// 📝️ Vcs keeps no draft lane, but the framework's close ladder still demands the lane's exact
    /// bounded disposer — without it every close faults `app owner did not provide the required
    /// bounded disposer for draft-store` and the app can never retire.
    fn build_draft_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::DraftStore<Self::Draft, Self::DraftMutation>>>> {
        Some(semio_framework_plugin::no_draft_store_disposer())
    }

    fn build_transient_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::TransientStore<Self::Transient, Self::TransientMutation>>>> {
        Some(semio_framework_plugin::no_transient_store_disposer())
    }

    fn build_transient_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Transient>>> {
        Some(semio_framework_plugin::no_transient_local_root_retirement_factory())
    }

    /// 👥️ Without these the `PresenceStore` holds no local retirement owner, so the FIRST command
    /// whose ephemeral leg reads local presence is refused with `presence local read requires a live
    /// exact local retirement owner` — the `ArtifactEditor` default is `None` and `EditorApp` passes
    /// it through unchanged (`no_presence_*` only types over `NoPresence`, never over a real presence
    /// payload). `VcsDemoPresence` is the empty terminal, so its exact bounded root retirement is the
    /// generic owned-value one, exactly like shooting.
    fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::bounded_transient_root_retirement_factory::<Self::Presence>())
    }

    fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::bounded_transient_root_retirement_factory::<Self::Presence>())
    }

    fn build_presence_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::PresenceStore<Self::Presence, Self::PresenceMutation>>>> {
        Some(Box::new(semio_framework_plugin::PresenceStoreOwnedDisposer::new(std::sync::Arc::new(Self::Presence::default()), |_| true).expect("VcsDemoPresence is the exact empty terminal")))
    }

    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(std::sync::Arc::new(VcsOneItemPreparationFactory::new(store::os_store::store_schema::HistoryLane::Document)))
    }

    fn build_config_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Config, Self::ConfigMutation>>> {
        None
    }

    fn bounded_first_step_tool_proofs() -> Vec<semio_framework_plugin::ArtifactBoundedFirstStepProof> {
        VcsBoundedProofs::bounded_first_step_tool_proofs().into_iter().chain(VcsResumableProofs::bounded_first_step_tool_proofs()).collect()
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(VcsBoundedCommandJobFactory::new(&controller))?;
        registry.register(VcsResumableCommandJobFactory::new(&controller))
    }

    fn build_tool_job(request: &mut Option<ArtifactOwnedToolJobRequest<EditorApp<Self>>>, output: &mut Option<semio_framework_plugin::ErasedToolJob>, cx: &mut semio_framework_plugin::__job::StepContext<'_>) -> Result<bool, semio_framework_plugin::__job::OriginalTurnFailure> {
        let original = request.as_ref().ok_or_else(|| semio_framework_plugin::__value::ValueError::literal(semio_framework_plugin::__value::ValueRefusalKind::InvariantViolated, "original retained producer request is absent"))?;
        let bounded = VCS_BOUNDED_TOOL_IDS.contains(&original.tool_id.as_str());
        let resumable = VCS_RESUMABLE_TOOL_IDS.contains(&original.tool_id.as_str());
        if !bounded && !resumable {
            return Ok(false);
        }
        if original.command.command_id() != original.tool_id {
            return Err(semio_framework_plugin::__value::ValueError::literal(semio_framework_plugin::__value::ValueRefusalKind::InvariantViolated, "VCS command does not match its exact registered tool").into());
        }
        let extent = if bounded { vcs_bounded_extent(&original.command, &original.snapshot, &original.interaction_state) } else { vcs_edit_extent(&original.command, &original.snapshot, &original.interaction_state) };
        if extent.is_none() {
            return Err(semio_framework_plugin::__value::ValueError::literal(semio_framework_plugin::__value::ValueRefusalKind::InvariantViolated, "the vcs command payload exceeds its bounded capacity").into());
        }
        let tool_id = original.command.command_id();
        semio_framework_plugin::retained_command::producer::admit_retained_tool_producer(request, output, cx, VcsCommand::command_id, VCS_BOUNDED_RAW_BYTES, if bounded { VCS_BOUNDED_WORK_ITEMS } else { VCS_EDIT_MAXIMUM_WORK_ITEMS }, false, |request| -> Box<dyn ArtifactCommandWork<EditorApp<Self>>> { if bounded { Box::new(BoundedArtifactCommandWork::new(tool_id, vcs_bounded_reduce, vcs_bounded_extent)) } else { Box::new(VcsEditCommandWork::new(tool_id)) } })
    }

    fn app_schema() -> Option<::semio_framework_schema_registry::AppSchemaDescriptor> {
        Some(crate::editor::vcs::config::schema::app_schema_descriptor())
    }

    fn initial_snapshot() -> VcsSnapshot {
        crate::standards::v1::subsets::any::schema::empty_vcs_snapshot()
    }

    /// 🏷️ The manifest action id each command was declared under — supplied wholesale by
    fn command_id(command: &VcsCommand) -> &'static str {
        command.command_id()
    }

    fn command_from_action(action: &str, args: &mut Option<semio_framework_value::DslValue>, original_fault: &mut Option<Fault>) -> Result<Self::Command, Fault> {
        if let Some(fault) = original_fault.take() { return Err(fault); }
        let args = args.as_ref();
        let args = args.cloned().unwrap_or(semio_framework_value::DslValue::Null);
        let text_arg = |key: &str| args.get(key).and_then(semio_framework_value::DslValue::as_str).unwrap_or_default().to_string();
        let bounded_text = |key: &str| {
            let text = text_arg(key);
            if text.len() > VCS_BOUNDED_RAW_BYTES {
                return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("vcs.command.payload-too-large"), "the vcs command payload exceeds its bounded capacity"));
            }
            Ok(text)
        };
        let pointer_samples = || {
            args.get("samples")
                .and_then(semio_framework_value::DslValue::as_array)
                .ok_or_else(|| Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("app.command.invalid-args"), "canvasPointerMove requires its `samples` batch"))?
                .iter()
                .map(|item| match item.as_array() {
                    Some([x, y]) => x.as_f64().zip(y.as_f64()).map(|(x, y)| [x, y]),
                    _ => None,
                }
                .ok_or_else(|| Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("app.command.invalid-args"), "every canvasPointerMove sample is an [x, y] number pair")))
                .collect::<Result<Vec<[f64; 2]>, Fault>>()
        };
        let pointer_cancelled = || args.get("cancelled").and_then(semio_framework_value::DslValue::as_bool).ok_or_else(|| Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("app.command.invalid-args"), "canvasPointerUp requires its `cancelled` flag"));
        match action {
            "incrementCounter" => Ok(VcsCommand::IncrementCounter(increment_counter::IncrementCounter {})),
            "setActiveExample" => Ok(VcsCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: text_arg("exampleId") })),
            "renameVcs" => Ok(VcsCommand::RenameVcs(rename_vcs::RenameVcs { title: bounded_text("value")? })),
            "changeStatus" => Ok(VcsCommand::ChangeStatus(change_status::ChangeStatus { status: bounded_text("value")? })),
            "changeNotes" => Ok(VcsCommand::ChangeNotes(change_notes::ChangeNotes { notes: bounded_text("value")? })),
            "changeCounter" => bounded_text("value")?
                .trim()
                .parse::<i64>()
                .map(|value| VcsCommand::ChangeCounter(change_counter::ChangeCounter { value }))
                .map_err(|_| Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("app.command.invalid-args"), "changeCounter requires an integer `value`")),
            "textEdit" => {
                let text = text_arg("text");
                if text.len() > VCS_BOUNDED_RAW_BYTES {
                    return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("vcs.command.payload-too-large"), "the vcs command payload exceeds its bounded capacity"));
                }
                Ok(VcsCommand::TextEdit(text_edit::TextEdit { text }))
            }
            "edit" => {
                let text = text_arg("text");
                if text.len() > VCS_BOUNDED_RAW_BYTES {
                    return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("vcs.command.payload-too-large"), "the vcs command payload exceeds its bounded capacity"));
                }
                Ok(VcsCommand::Edit(edit_command::Edit { text }))
            }
            "noMutation" => Ok(VcsCommand::NoMutation(no_operation::NoMutation {})),
            "canvasPointerDown" => Ok(VcsCommand::CanvasPointerDown(canvas_pointer_down::CanvasPointerDown {})),
            "canvasPointerMove" => Ok(VcsCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove { samples: pointer_samples()? })),
            "canvasPointerUp" => Ok(VcsCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp { cancelled: pointer_cancelled()? })),
            "canvasWheel" => Ok(VcsCommand::CanvasWheel(canvas_wheel::CanvasWheel {})),
            other => Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("app.command.unsupported"), format!("unknown VCS app action '{other}'"))),
        }
    }

    fn handle(
        command: &VcsCommand,
        doc: &ArtifactView<'_, VcsSnapshot>,
        cfg: &ConfigView<'_, VcsDemoConfig>,
        _interaction: &InteractionView<'_>,
        _view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<VcsDemoMutation, VcsDemoConfigMutation, Self::DraftMutation>, Fault> {
        command.dispatch(doc, cfg)
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, VcsSnapshot>, _cfg: &ConfigView<'_, VcsDemoConfig>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let labels = vcs_play_labels(view_state);
        match body_key {
            VCS_PLAY_BODY_EDITOR => editor::render(doc.snapshot, labels).map(semio_framework_plugin::built_to_component_tree),
            VCS_PLAY_BODY_HISTORY => history::render(doc.history).map(semio_framework_plugin::built_to_component_tree),
            VCS_PLAY_BODY_ARTIFACT => document_panel::render(doc.history, labels, &semio_framework_plugin::TreeWindows::for_body(view_state, VCS_PLAY_BODY_ARTIFACT)).map(semio_framework_plugin::built_to_component_tree),
            VCS_PLAY_BODY_INSPECTION => inspection_panel::render(doc.snapshot, labels).map(semio_framework_plugin::built_to_component_tree),
            _ => semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️VcsPlayApp

//#region 🔖️Manifest
/// 🧱️ The manifest stitch: one call per taxonomy node, each sourced from that node's own `definition()`.
/// Only the leaf action/keybinding declarations (which have no dedicated `_def` passthrough) are written
/// out inline.
pub fn create_vcs_app() -> semio_framework_plugin::AppDefinition {
    Editor::builder(crate::VCS_DIALECT)
            .document(["semio", "vcs"])
            .artifact_kind(crate::artifact_kind())
            .icon_id("git-branch")
            .mode_def(edit::definition())
            .default_mode_id(edit::VCS_PLAY_MODE_EDIT)
            .window_kind_def(editor::definition())
            .window_kind_def(history::definition())
            .panel_tab_def(document_panel::definition())
            .panel_tab_def(inspection_panel::definition())
            .mutation("incrementCounter", LocalizedLabel::native("Increment Counter", "Zähler erhöhen"))
            .mutation("renameVcs", LocalizedLabel::native("Rename", "Umbenennen"))
            .mutation("changeCounter", LocalizedLabel::native("Change Counter", "Zähler ändern"))
            .mutation("changeStatus", LocalizedLabel::native("Change Status", "Status ändern"))
            .mutation("changeNotes", LocalizedLabel::native("Change Notes", "Notizen ändern"))
            .action_with(semio_framework_plugin::ActionDefinition::new("textEdit", LocalizedLabel::native("Edit Text", "Text bearbeiten"), semio_framework_plugin::ActionKind::Mutation, "typography"))
            .mutation("edit", LocalizedLabel::native("Edit", "Bearbeiten"))
            .view_action("noMutation", LocalizedLabel::native("No-operation", "Keine Aktion"))
            .action_with(semio_framework_plugin::ActionDefinition::new("canvasPointerDown", LocalizedLabel::native("Canvas Pointer Down", "Leinwand-Zeiger gedrückt"), semio_framework_plugin::ActionKind::View, "mouse-pointer"))
            .action_audience("canvasPointerDown", semio_framework_plugin::CapabilityAudience::Input)
            .action_with(semio_framework_plugin::ActionDefinition::new("canvasPointerMove", LocalizedLabel::native("Canvas Pointer Move", "Leinwand-Zeiger bewegt"), semio_framework_plugin::ActionKind::View, "mouse-pointer"))
            .action_audience("canvasPointerMove", semio_framework_plugin::CapabilityAudience::Input)
            .action_with(semio_framework_plugin::ActionDefinition::new("canvasPointerUp", LocalizedLabel::native("Canvas Pointer Up", "Leinwand-Zeiger losgelassen"), semio_framework_plugin::ActionKind::View, "mouse-pointer"))
            .action_audience("canvasPointerUp", semio_framework_plugin::CapabilityAudience::Input)
            .view_action("canvasWheel", LocalizedLabel::native("Canvas Wheel", "Leinwand-Mausrad"))
            .action_audience("canvasWheel", semio_framework_plugin::CapabilityAudience::Input)
            .action_with(semio_framework_plugin::ActionDefinition::new("setActiveExample", LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen"), semio_framework_plugin::ActionKind::Mutation, "panel-left"))
            .action_destructive("setActiveExample")
            .action_interactive_job("incrementCounter", InteractiveJobClassification::Migrated)
            .action_interactive_job("renameVcs", InteractiveJobClassification::Migrated)
            .action_interactive_job("changeCounter", InteractiveJobClassification::Migrated)
            .action_interactive_job("changeStatus", InteractiveJobClassification::Migrated)
            .action_interactive_job("changeNotes", InteractiveJobClassification::Migrated)
            .action_interactive_job("noMutation", InteractiveJobClassification::Migrated)
            .action_interactive_job("canvasPointerDown", InteractiveJobClassification::Migrated)
            .action_interactive_job("canvasPointerMove", InteractiveJobClassification::Migrated)
            .action_interactive_job("canvasPointerUp", InteractiveJobClassification::Migrated)
            .action_interactive_job("canvasWheel", InteractiveJobClassification::Migrated)
            .action_interactive_job("textEdit", InteractiveJobClassification::Migrated)
            .action_interactive_job("edit", InteractiveJobClassification::Migrated)
            .action_interactive_job("setActiveExample", InteractiveJobClassification::Migrated)
            .action_args("setActiveExample", vec![semio_framework_plugin::ActionArgDef::select("exampleId", LocalizedLabel::native("Example", "Beispiel"), vec![semio_framework_plugin::ActionArgOption::new(crate::examples::demo::ID, crate::examples::demo::label())])])
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo")
            .default_layout(edit::layout())
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
            .config(VcsPlayApp::config_spec())
            .action_describe("incrementCounter", LocalizedLabel::native("Adds one to the demo document's counter.", "Erhöht den Zähler des Demodokuments um eins."))
            .action_describe("renameVcs", LocalizedLabel::native("Sets the demo document's title.", "Setzt den Titel des Demodokuments."))
            .action_describe("changeCounter", LocalizedLabel::native("Sets the demo document's counter to an integer.", "Setzt den Zähler des Demodokuments auf eine ganze Zahl."))
            .action_describe("changeStatus", LocalizedLabel::native("Sets the demo document's status.", "Setzt den Status des Demodokuments."))
            .action_describe("changeNotes", LocalizedLabel::native("Sets the demo document's notes.", "Setzt die Notizen des Demodokuments."))
            .action_describe("textEdit", LocalizedLabel::native("Reads the given text as the demo document's projection and writes the title, counter, status and notes that differ; a live typing run commits as one edit.", "Liest den angegebenen Text als Projektion des Demodokuments und schreibt abweichenden Titel, Zähler, Status und Notizen; ein fortlaufender Tipplauf wird als eine Änderung übernommen."))
            .action_describe("edit", LocalizedLabel::native("Reads the given text as the demo document's projection and writes every field that differs as one edit.", "Liest den angegebenen Text als Projektion des Demodokuments und schreibt jedes abweichende Feld als eine Änderung."))
            .action_describe("setActiveExample", LocalizedLabel::native("Replaces the whole demo document with one of the plugin's bundled examples, by example id.", "Ersetzt das gesamte Demodokument durch eines der mitgelieferten Beispiele, anhand der Beispiel-Id."))
            .action_audience("noMutation", semio_framework_plugin::CapabilityAudience::Chrome)
            .build_definition()
}
//#endregion 🔖️Manifest
