//! 🕹️ Forms play app: the `FormsPlayApp` editor and its `ArtifactEditor` implementation.

use super::*;
use semio_framework_2d_compute::EngineHandles;

//#region 🔖️FormsPlayApp
/// 🧪️ B1: unit struct — every former `FormsPlayRuntime` field now lives in `FormsConfig`, written through
/// `FormsConfigMutation`s.
#[derive(Default)]
pub struct FormsPlayApp;

impl ArtifactEditor for FormsPlayApp {
    type Snapshot = FormsSnapshot;
    type Members = semio_s_artifact_stdio_semio::SemioMembers;
    type Mutation = FormMutation;
    type Config = FormsConfig;
    type ConfigMutation = FormsConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = semio_framework_plugin::NoPresence;
    type PresenceMutation = semio_framework_plugin::NoPresenceMutation;
    type Transient = semio_framework_plugin::NoTransient;
    type TransientMutation = semio_framework_plugin::NoTransientMutation;

    type Command = FormsCommand;

    const DIALECT: Dialect = crate::FORMS_DIALECT;
    /// 🧬️ The crate's one loaded-parent child projection (`crate::forms_child_restore_projection`).
    fn genesis_child_pack(snapshot: &Self::Snapshot, slot: &str, child_id: &str) -> Result<Option<Vec<u8>>,semio_framework_value::ValueError> {
 Ok((||{
        crate::forms_genesis_child_pack(snapshot, slot, child_id)
    
})())
}

    fn child_restore_projection(snapshot: &Self::Snapshot) -> Result<semio_framework_composition_admission::ChildRestoreProjection<'_>, Fault> {
        crate::forms_child_restore_projection(snapshot)
    }
    const DOCUMENT_SCHEMA: &'static str = FORMS_DOCUMENT_SCHEMA;

    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(std::sync::Arc::new(FormsStorePreparationFactory::<FormsSnapshot, FormMutation>::new("forms-artifact-retained")))
    }

    fn build_config_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Config, Self::ConfigMutation>>> {
        Some(std::sync::Arc::new(FormsStorePreparationFactory::<FormsConfig, FormsConfigMutation>::new("forms-config-retained")))
    }

    fn build_document_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
        Some(semio_framework_plugin::bounded_document_store_disposer::<Self::Snapshot, Self::Mutation>())
    }

    fn build_config_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::Config, Self::ConfigMutation>>>> {
        Some(semio_framework_plugin::bounded_config_store_disposer::<Self::Config, Self::ConfigMutation>())
    }

    fn build_draft_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::DraftStore<Self::Draft, Self::DraftMutation>>>> {
        Some(semio_framework_plugin::no_draft_store_disposer())
    }

    fn build_presence_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::PresenceStore<Self::Presence, Self::PresenceMutation>>>> {
        Some(semio_framework_plugin::no_presence_store_disposer())
    }

    fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::no_presence_local_root_retirement_factory())
    }

    fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::no_presence_peer_retirement_factory())
    }

    fn build_transient_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::TransientStore<Self::Transient, Self::TransientMutation>>>> {
        Some(semio_framework_plugin::no_transient_store_disposer())
    }

    fn build_transient_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Transient>>> {
        Some(semio_framework_plugin::no_transient_local_root_retirement_factory())
    }

    fn register_window_config_owners(registry: &mut semio_framework_plugin::WindowConfigOwnerRegistry) -> Result<(), Fault> {
        try_window::config::register(registry)
    }

    fn register_window_transient_owners(registry: &mut semio_framework_plugin::WindowTransientOwnerRegistry) -> Result<(), Fault> {
        try_window::transient::register(registry)
    }

    fn retained_window_transient_target(command: &Self::Command) -> Option<(&str, &'static str)> {
        let target = match command {
            FormsCommand::SetTryValue(payload) => (&payload.window_id, &payload.window_kind_id),
            FormsCommand::SetTryValues(payload) => (&payload.window_id, &payload.window_kind_id),
            FormsCommand::SetTryValueStep(payload) => (&payload.window_id, &payload.window_kind_id),
            FormsCommand::ResetTry(payload) => (&payload.window_id, &payload.window_kind_id),
            FormsCommand::PreviousStep(payload) => (&payload.window_id, &payload.window_kind_id),
            FormsCommand::NextStep(payload) => (&payload.window_id, &payload.window_kind_id),
            FormsCommand::Submit(payload) => (&payload.window_id, &payload.window_kind_id),
            _ => return None,
        };
        (!target.0.is_empty() && target.1 == support::FORMS_PLAY_WINDOW_TRY).then_some((target.0.as_str(), support::FORMS_PLAY_WINDOW_TRY))
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(FormsBoundedCommandJobFactory::new(&controller))
    }

    fn build_tool_job(request: &mut Option<ArtifactOwnedToolJobRequest<EditorApp<Self>>>, output: &mut Option<semio_framework_plugin::ErasedToolJob>, cx: &mut semio_framework_plugin::__job::StepContext<'_>) -> Result<bool, semio_framework_plugin::__job::OriginalTurnFailure> {
        let original = request.as_ref().ok_or_else(|| semio_framework_plugin::__value::ValueError::literal(semio_framework_plugin::__value::ValueRefusalKind::InvariantViolated, "original retained producer request is absent"))?;
        if !FORMS_RETAINED_TOOL_IDS.contains(&original.tool_id.as_str()) {
            return Ok(false);
        }
        if original.command.command_id() != original.tool_id {
            return Err(semio_framework_plugin::__value::ValueError::literal(semio_framework_plugin::__value::ValueRefusalKind::InvariantViolated, "the forms command does not match its exact registered tool").into());
        }
        let tool_id = original.command.command_id();
        let maximum_work_items = if tool_id == "exportResponses" { crate::standards::v1::subsets::any::io::text::snapshot::response::export::ResponseExport::work_items(&original.snapshot.responses) } else { 1 };
        semio_framework_plugin::retained_command::producer::admit_retained_tool_producer(request, output, cx, FormsCommand::command_id, FORMS_RETAINED_RAW_BYTES, maximum_work_items, false, |request| FormsWindowCommandWork::new(tool_id))
    }

    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: EditorApp<FormsPlayApp>,
        owner_file: "✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.forms.forms@1/*#editor",
        artifact_schema: "forms.form",
        factory: "FormsBoundedCommandJobFactory",
        factory_type: FormsBoundedCommandJobFactory,
        tools: {
            "setTryValue" => forms_bounded_contract(),
            "setTryValues" => forms_bounded_contract(),
            "resetTry" => forms_bounded_contract(),
            "previousStep" => forms_bounded_contract(),
            "nextStep" => forms_bounded_contract(),
            "submit" => forms_bounded_contract(),
            "setContributions" => forms_bounded_contract(),
            "addStep" => forms_bounded_contract(),
            "patchStep" => forms_bounded_contract(),
            "removeStep" => forms_bounded_contract(),
            "moveStep" => forms_bounded_contract(),
            "updateForm" => forms_bounded_contract(),
            "addBlock" => forms_bounded_contract(),
            "removeBlock" => forms_bounded_contract(),
            "patchQuestions" => forms_bounded_contract(),
            "patchQuestionOptions" => forms_bounded_contract(),
            "addQuestionOption" => forms_bounded_contract(),
            "removeQuestionOption" => forms_bounded_contract(),
            "patchVectorField" => forms_bounded_contract(),
            "addVectorField" => forms_bounded_contract(),
            "removeVectorField" => forms_bounded_contract(),
            "moveBlock" => forms_bounded_contract(),
            "dropQuestionKind" => forms_bounded_contract(),
            "setSpecJson" => forms_bounded_contract(),
            "setActiveExample" => forms_bounded_contract(),
            "exportSnapshot" => forms_bounded_contract(),
            "exportResponses" => forms_bounded_contract(),
            "discardResponse" => forms_bounded_contract(),
            "setTryValueStep" => forms_bounded_contract(),
        }
    }

    fn app_schema() -> Option<::semio_framework_schema_registry::AppSchemaDescriptor> {
        Some(crate::editor::forms::config::schema::app_schema_descriptor())
    }

    fn initial_snapshot() -> FormsSnapshot {
        crate::schema::empty_forms_snapshot()
    }

    fn io() -> Option<semio_framework_plugin::AppIo> {
        Some(forms_io())
    }

    /// 🏷️ The manifest action id each command was declared under — supplied wholesale by
    /// declaration (host-pushed, not user-facing actions).
    fn command_id(command: &FormsCommand) -> &'static str {
        command.command_id()
    }

    /// 🎯️ Host-action bridge into the closed `FormsCommand` enum (ticket
    /// 26/09/16/FORMS-PLUGIN-END-TO-END). The React/wgpu shells still speak `{action, args}` with
    /// camelCase argument keys; every `🎮️commands/*` payload derives `FromValue` over its own
    /// snake_case field names, so this boundary folds the keys and decodes — the default trait impl
    /// refuses every app action outright, which left `setActiveExample`/`addStep`/… dead in the shell.
    fn command_from_action(action: &str, args: &mut Option<semio_framework_value::DslValue>, original_fault: &mut Option<Fault>) -> Result<Self::Command, Fault> {
        if let Some(fault) = original_fault.take() { return Err(fault); }
        let args = args.as_ref();
        args_bridge::command_from_action(action, args)
    }

    fn handle(
        command: &FormsCommand,
        doc: &ArtifactView<'_, FormsSnapshot>,
        cfg: &ConfigView<'_, FormsConfig>,
        _interaction: &InteractionView<'_>,
        view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<FormMutation, FormsConfigMutation, Self::DraftMutation>, Fault> {
        match (command, view_state) {
            (FormsCommand::PreviousStep(payload), Some(view)) => {
                forms_try_view(view, &payload.window_id, &payload.window_kind_id)?;
                let mut emit = Emit::default();
                emit.window_config_mutations.extend(try_window::config::addressed(view, &try_window::config::current(cfg), previous_step::handle_window(payload, &try_window::config::current(cfg))?)?);
                Ok(emit)
            }
            (FormsCommand::NextStep(_), _)
            | (FormsCommand::SetTryValue(_), _)
            | (FormsCommand::SetTryValues(_), _)
            | (FormsCommand::SetTryValueStep(_), _)
            | (FormsCommand::ResetTry(_), _) => Err(Fault::from("forms-try-window-retained-context-required")),
            _ => command.dispatch(doc, cfg),
        }
    }

    /// 🕹️ `fields` domain: `HierarchyProvider::Topology` from the document's own step/question nesting —
    /// see `forms_fields_topology`'s doc comment.
    fn interaction_topology(doc: &ArtifactView<'_, FormsSnapshot>, _cfg: &ConfigView<'_, FormsConfig>) -> Result<InteractionTopology, semio_framework_value::ValueError> {
 Ok((||{
        let mut domains = std::collections::BTreeMap::new();
        domains.insert(FORMS_INTERACTION_FIELDS.to_string(), forms_fields_topology(doc.snapshot));
        InteractionTopology { domains }
    
})())
}

    //#region 🔖️Media
    /// 🎞️ WORKFLOWS-END-TO-END-TYPED-PORTS port recipe: `document:out` replicates the trait default
    /// exactly (overriding `export_media` for `dictionary:out` forfeits the default's dispatch);
    /// `dictionary:out` re-exports the form's currently-configured default field values as a
    /// typed `form.dictionary` intrinsic object keyed by question id — no `cfg` parameter reaches this method, so
    /// this is the form's authored defaults, not a live in-progress Try-wizard session (which lives
    /// in the exact Try window config and transient owners).
    fn export_media(port: &str, doc: &ArtifactView<'_, FormsSnapshot>) -> Result<semio_framework_plugin::Media, MediaError> {
        match port {
            "artifact:out" => {
                let bytes = pack::snapshot::ArtifactPack::encode_pack(doc.snapshot);
                Ok(semio_framework_plugin::Media {
                    media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
                    payload: MediaPayload::Structured { schema: FORMS_DOCUMENT_SCHEMA.into(), json: store::pack_rt::pack_value_to_base64(&bytes) },
                })
            }
            "dictionary:out" => {
                let dictionary=crate::schema::configured_dictionary(doc.snapshot).map_err(|error|MediaError::Payload(port.to_string(),error.into_message()))?;
                Ok(semio_framework_plugin::Media { media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value }, payload: MediaPayload::Intrinsic { schema: "form.dictionary".into(), value:dictionary.into_intrinsic() } })
            }
            _ => Err(MediaError::NotImplemented),
        }
    }
    //#endregion 🔖️Media

    fn render(body_key: &str, doc: &ArtifactView<'_, FormsSnapshot>, cfg: &ConfigView<'_, FormsConfig>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let spec = doc.snapshot;
        let config = cfg.snapshot;
        let labels = forms_play_labels(view_state);
        let node = match body_key {
            FORMS_PLAY_BODY_BLUEPRINT => builder::render(spec, config, view_state, None),
            responses::results::BODY => responses::results::render(spec, labels, &semio_framework_plugin::TreeWindows::for_body(view_state, responses::results::BODY)),
            FORMS_PLAY_BODY_TRY => try_window::render(spec, config, &try_window::config::current(cfg), &try_window::transient::FormsTryWindowTransient::default(), labels, view_state),
            FORMS_PLAY_BODY_ARTIFACT => document_panel::render(spec, labels, &semio_framework_plugin::TreeWindows::for_body(view_state, FORMS_PLAY_BODY_ARTIFACT)),
            FORMS_PLAY_BODY_CATALOGUE => catalogue_panel::render(config, view_state, &semio_framework_plugin::TreeWindows::for_body(view_state, FORMS_PLAY_BODY_CATALOGUE)),
            FORMS_PLAY_BODY_INSPECTION => inspection_panel::render(spec, config, &[], view_state, &semio_framework_plugin::TreeWindows::for_body(view_state, FORMS_PLAY_BODY_INSPECTION)),
            _ => return semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        }?;
        Ok(semio_framework_plugin::built_to_component_tree(node))
    }

    fn render_with_request_context(
        _owner: &semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
        body_key: &str,
        doc: &ArtifactView<'_, FormsSnapshot>,
        cfg: &ConfigView<'_, FormsConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        transient: &semio_framework_plugin::TransientView<'_, semio_framework_plugin::NoTransient>,
        interaction: &InteractionView<'_>,
        cx:&mut semio_framework_plugin::StepContext<'_>,
    ) -> semio_framework_plugin::OriginalRenderResult<semio_framework_plugin::ComponentTree> {
        let rendered:semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree>=(||{
        let spec = doc.snapshot;
        let config = cfg.snapshot;
        let labels = forms_play_labels(view_state);
        let node = match body_key {
            FORMS_PLAY_BODY_BLUEPRINT => builder::render(spec, config, view_state, interaction.selection(FORMS_INTERACTION_FIELDS).ids.first().map(String::as_str)),
            responses::results::BODY => responses::results::render(spec, labels, &semio_framework_plugin::TreeWindows::for_body(view_state, responses::results::BODY)),
            FORMS_PLAY_BODY_TRY => try_window::render(spec, config, &try_window::config::current(cfg), &try_window::transient::current(transient), labels, view_state),
            FORMS_PLAY_BODY_ARTIFACT => document_panel::render(spec, labels, &semio_framework_plugin::TreeWindows::for_body(view_state, FORMS_PLAY_BODY_ARTIFACT)),
            FORMS_PLAY_BODY_CATALOGUE => catalogue_panel::render(config, view_state, &semio_framework_plugin::TreeWindows::for_body(view_state, FORMS_PLAY_BODY_CATALOGUE)),
            FORMS_PLAY_BODY_INSPECTION => inspection_panel::render(spec, config, &interaction.selection(FORMS_INTERACTION_FIELDS).ids, view_state, &semio_framework_plugin::TreeWindows::for_body(view_state, FORMS_PLAY_BODY_INSPECTION)),
            _ => return semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        }?;
        Ok(semio_framework_plugin::built_to_component_tree(node))

        })();
        rendered.map(Some).map_err(Into::into)
        }
}
//#endregion 🔖️FormsPlayApp
