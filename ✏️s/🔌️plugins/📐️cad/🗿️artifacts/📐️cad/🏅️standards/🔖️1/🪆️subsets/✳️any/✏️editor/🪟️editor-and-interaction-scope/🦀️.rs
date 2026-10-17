//! 🪟️ Editor implementation of the CAD play app: the world body keys, the interaction scope and the artifact editor trait implementation. See [`super`].

use super::*;
use semio_s_artifact_cad_cad_kit::CAD_INTERACTION_DOMAIN;
use semio_s_artifact_cad_cad_kit::cad_io;
use semio_s_artifact_cad_cad_kit::reset_document_effect;
use semio_s_artifact_cad_cad_kit::collect_modelspace_solids;
use semio_s_artifact_cad_cad_modes::windows::config::runtime_of;

//#region 🔖️PlayApp
/// 🎯️ The four world bodies the `cad` interaction domain paints into.
pub(crate) const CAD_WORLD_BODY_KEYS: [&str; 4] = [shape::BODY_KEY, building::BODY_KEY, energy::BODY_KEY, structure_classic::BODY_KEY];

/// 🎯️ `CadPlayApp::interaction_scope`'s pure body — see the method's doc. `None` hands a verb that
/// touched a domain this app does not declare (or none) back to the framework's own answer.
pub(crate) fn cad_interaction_scope(verb: semio_framework_plugin::InteractionVerb, domains: &[&str]) -> Option<semio_framework::kernel::UiDirtyScope> {
    use semio_framework::kernel::UiDirtyScope;
    use semio_framework_plugin::InteractionVerb;
    if domains.is_empty() || domains.iter().any(|domain| *domain != CAD_INTERACTION_DOMAIN) {
        return None;
    }
    let windows: Vec<String> = CAD_WORLD_BODY_KEYS.iter().map(|key| (*key).to_string()).collect();
    Some(match verb {
        InteractionVerb::Hover => UiDirtyScope::Partial { window_bodies: windows, panel_bodies: Vec::new(), utilities: false, tools: false, engagements: false, measures: false, labels: false },
        // 🎯️ A selection change: the world bodies, the two panels that render the selection, and the
        // engagement HUD's `N selected` count.
        InteractionVerb::Select | InteractionVerb::ClearSelection | InteractionVerb::SelectAll => UiDirtyScope::Partial {
            window_bodies: windows,
            panel_bodies: vec![inspection::CAD_PLAY_BODY_PROPERTIES.to_string(), document::CAD_PLAY_BODY_ARTIFACT.to_string()],
            utilities: false,
            tools: false,
            engagements: true,
            measures: false,
            labels: false,
        },
        InteractionVerb::SetSelectionMode | InteractionVerb::SetGranularity => UiDirtyScope::None,
    })
}

impl ArtifactEditor for CadPlayApp {
    fn owned_mutation_batch_birth_bytes()->Option<usize>{Some(store::MemberStoreOwnedBatch::scaffold_byte_demand::<CadMutation>())}
    fn admit_owned_mutation_batch(values:&mut Option<Vec<CadMutation>>,grant:semio_framework_value::retained_clone::RetainedCloneGrant)->Result<Option<(store::MemberStoreOwnedBatch,semio_framework_value::retained_clone::RetainedCloneProgress)>,semio_framework_value::ValueError>{store::MemberStoreOwnedBatch::admit(values,grant)}
    /// 📚️ Artifact catalogue stamped by `PluginBuilder::editor` onto the navbar dropdown.
    fn examples() -> Vec<semio_framework_plugin::ExampleSource> {
        vec![semio_s_artifact_cad_cad_examples::source()]
    }
    type Snapshot = CadSnapshot;
    type Mutation = CadMutation;
    type Config = CadConfig;
    type ConfigMutation = CadConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = semio_s_artifact_cad_cad_presence::CadPresence;
    type PresenceMutation = semio_s_artifact_cad_cad_presence::CadPresenceMutation;
    type Transient = semio_framework_plugin::NoTransient;
    type TransientMutation = semio_framework_plugin::NoTransientMutation;
    type Command = CadCommand;
    /// 🧩️ Every pane's model (and the forward-declared drawing) child is an `s.stdio.semio` member.
    type Members = semio_s_artifact_stdio_semio::SemioMembers;

    fn child_restore_projection(snapshot: &Self::Snapshot) -> Result<semio_framework_composition_admission::ChildRestoreProjection<'_>, Fault> {
        semio_s_artifact_cad_cad_model::snapshot::cad_child_restore_projection(snapshot)
    }

    /// 🌱️ See [`crate::cad_genesis_child_pack`].
    fn genesis_child_pack(snapshot: &Self::Snapshot, slot: &str, child_id: &str) -> Result<Option<Vec<u8>>,semio_framework_value::ValueError> {
 Ok((||{
        semio_s_artifact_cad_cad_play::cad_genesis_child_pack(snapshot, slot, child_id)
    
})())
}

    fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(std::sync::Arc::new(semio_s_artifact_cad_cad_presence::retirement::CadPresenceRetirementFactory))
    }

    fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(std::sync::Arc::new(semio_s_artifact_cad_cad_presence::retirement::CadPresenceRetirementFactory))
    }

    fn build_presence_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::PresenceStore<Self::Presence, Self::PresenceMutation>>>> {
        Some(semio_s_artifact_cad_cad_presence::retirement::store_disposer())
    }

    /// 🎯️ The framework interaction verbs on the `cad` domain, answered out of this app's own body
    /// table instead of the framework's fallback (every window kind that declares the domain — the
    /// same four bodies — but `Full` for a verb the fallback cannot place). A hover repaints the four
    /// world bodies alone (the four panes share the domain, and only their selection lanes read the
    /// hover); a selection change adds the two panels that render the selection (Inspection and the
    /// Artifact tree); a mode/granularity switch touches nothing this app renders differently.
    fn interaction_scope(verb: semio_framework_plugin::InteractionVerb, domains: &[&str]) -> Option<semio_framework::kernel::UiDirtyScope> {
        cad_interaction_scope(verb, domains)
    }

    fn register_window_config_owners(registry: &mut semio_framework_plugin::WindowConfigOwnerRegistry) -> Result<(), Fault> {
        registry.register::<shape::config::CadShapeWindowConfigOwner>()?;
        registry.register::<building::config::CadBuildingWindowConfigOwner>()?;
        registry.register::<energy::config::CadEnergyWindowConfigOwner>()?;
        registry.register::<structure_classic::config::CadStructureClassicWindowConfigOwner>()
    }

    /// 🫧️ Every world window's engagement transient (design §17.4).
    fn register_window_transient_owners(registry: &mut semio_framework_plugin::WindowTransientOwnerRegistry) -> Result<(), Fault> {
        window_transient::register(registry)
    }

    const DIALECT: Dialect = semio_s_artifact_cad_cad_model::snapshot::CAD_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = CAD_DOCUMENT_SCHEMA;

    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(std::sync::Arc::new(CadArtifactStorePreparationFactory))
    }

    fn build_config_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Config, Self::ConfigMutation>>> {
        Some(std::sync::Arc::new(CadConfigStorePreparationFactory))
    }

    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: EditorApp<CadPlayApp>,
        owner_file: "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.cad.cad@1/*#editor",
        artifact_schema: "cad.scene",
        factory: "CadRetainedCommandJobFactory",
        factory_type: CadRetainedCommandJobFactory,
        contract: cad_retained_contract(),
        tools: [
            "addNode",
            "renameNode",
            "patchCadPlayReference",
            "setReferenceHidden",
            "setReferenceLocked",
            "addObject",
            "patchObject",
            "patchSelection",
            "deleteObject",
            "duplicateObject",
            "translateSelection",
            "rotateSelection",
            "scaleSelection",
            "engagementSubmit",
            "worldPointerDown",
            "setCamera",
            "setProjection",
            "setProjectionParam",
            "setDislocateOption",
            "setNodeSelection",
            "setReferenceSelection",
            "referenceHover",
            "engagementInput",
            "engagementPossibleSelect",
            "engagementRepeatLast",
            "engagementAbort",
            "worldPointerMove",
            "toggleSun",
            "setSunAzimuth",
            "setSunElevation",
            "setSunIntensity",
            "setContributions",
            "setActiveExample",
            "loadRawRequest",
            "applyTransformation",
            "importCadFile",
            "saveSelected",
            "saveInPlay",
            "saveCurrent"
        ]
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(CadRetainedCommandJobFactory::new(&controller))
    }

    fn build_tool_job(request: ArtifactOwnedToolJobRequest<EditorApp<Self>>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
        if !CAD_RETAINED_TOOL_IDS.contains(&request.tool_id.as_str()) {
            return Ok(None);
        }
        if request.command.command_id() != request.tool_id || cad_retained_extent(&request.command, &request.snapshot, &request.interaction_state) != Some(1) {
            return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("app.command.tool-mismatch"), "CAD command does not match its exact retained tool registration"));
        }
        let tool_id = request.command.command_id();
        let work: Box<dyn ArtifactCommandWork<EditorApp<Self>>> = Box::new(CadRetainedCommandWork { tool_id, consumed: false });
        let operation_context = AppOperationContext { retained: request.retained,
            app_instance_id: request.app_instance_id,
            parent_document_id: request.parent_document_id.clone(),
            operation_id: request.operation.operation.0,
            generation: request.operation.generation.0,
            canonical_base_revision: request.canonical_base_revision,
            authoring_seed: request.authoring_seed.clone(),
        };
        let payload = ArtifactRetainedCommandPayload::new(
            semio_framework_plugin::retained_command::ArtifactRetainedCommandInputs {
                command: *request.command,
                snapshot: request.snapshot,
                snapshot_read: request.snapshot_read,
                config: request.config,
                history: request.history,
                interaction_state: request.interaction_state,
                interaction_hover: request.interaction_hover,
                context: Some(request.context),
                operation: operation_context,
                completion: request.completion,
            },
            CadCommand::command_id,
            cad_retained_raw_bytes(tool_id),
            CAD_RETAINED_WORK_ITEMS,
            work,
        );
        Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }

    fn build_reserved_tool_job(request: semio_framework_plugin::ArtifactReservedToolJobRequest<EditorApp<Self>>) -> Result<Option<semio_framework_plugin::ArtifactReservedToolJob>, Fault> {
        if request.tool_id != "import-media" { return Ok(None); }
        if !request.raw_wire.is_empty() || request.authoring_seed.is_empty() { return Err(Fault::from("Cad media requires exact decoded admission authority")); }
        let semio_framework_plugin::ArtifactReservedToolInput::Media { port, media } = request.input else { return Err(Fault::from("Cad media requires its declared input owner")); };
        if !matches!(port.as_str(), "geometry:in" | "artifact:in") { return Err(Fault::from("Cad media input port is not declared")); }
        let operation = AppOperationContext { retained: request.retained, app_instance_id: request.app_instance_id, parent_document_id: request.parent_document_id, operation_id: 0, generation: 0, canonical_base_revision: request.canonical_base_revision, authoring_seed: request.authoring_seed };
        let work = Box::new(CadMediaWork::new(port, media, Some(request.children)));
        let payload = ArtifactRetainedCommandPayload::new(
            semio_framework_plugin::retained_command::ArtifactRetainedCommandInputs { command: CadCommand::ImportCadFile(import_cad_file::ImportCadFile { name: String::new(), payload: String::new() }), snapshot: request.snapshot, snapshot_read:request.snapshot_read, config: request.config, history: request.history, interaction_state: std::sync::Arc::new(protocol::InteractionState::default()), interaction_hover: std::sync::Arc::new(Default::default()), context: None, operation, completion: request.completion },
            CadCommand::command_id, CAD_RETAINED_RAW_BYTES, CAD_RETAINED_WORK_ITEMS, work,
        );
        Ok(Some(semio_framework_plugin::ArtifactReservedToolJob::new(CadMediaJob(ArtifactRetainedCommandJob::new(payload)))))
    }

    fn app_schema() -> Option<::semio_framework_schema_registry::AppSchemaDescriptor> {
        Some(semio_s_artifact_cad_cad_config::schema::app_schema_descriptor())
    }

    fn initial_snapshot() -> CadSnapshot {
        forest_play_scene()
    }

    fn io() -> Option<semio_framework_plugin::AppIo> {
        Some(cad_io())
    }

    /// 🎞️ `geometry:in` (WORKFLOWS-END-TO-END-TYPED-PORTS port recipe): accepts incoming mesh/brep
    /// geometry from any upstream 3D producer and inserts it as one `insert-element` on the Shape pane's composed model
    /// child, through the same brep kernel every other import path shares (design §20.15). Falls through to the default
    /// `document:in` importer for any other port.
    fn import_media(port: &str, media: &Media, doc: &ArtifactView<'_, CadSnapshot>) -> Result<Emit<CadMutation, CadConfigMutation, Self::DraftMutation>, MediaError> {
        if port != "geometry:in" {
            if port != "artifact:in" {
                return Err(MediaError::NotImplemented);
            }
            let MediaPayload::Structured { json, .. } = &media.payload else {
                return Err(MediaError::Payload(port.to_string(), "default document:in importer only accepts a Structured (base64 pack) payload".into()));
            };
            let bytes = store::pack_rt::pack_value_from_base64(json).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
            let projection = <CadSnapshot as pack::snapshot::ArtifactPack>::decode_pack(&bytes).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
            return Ok(Emit { effects: vec![reset_document_effect(&projection)], ..Default::default() });
        }
        let name = match &media.media_type.form {
            MediaForm::Brep => "import.step",
            _ => "import.obj",
        };
        let structured;
        let payload = match &media.payload {
            MediaPayload::Structured { json, .. } => { structured = semio_framework_value::DslValue::String(json.clone()); &structured },
            MediaPayload::Intrinsic { value, .. } => value,
            MediaPayload::Binary { .. } => return Err(MediaError::Payload(port.to_string(), "geometry:in requires an intrinsic or structured file payload".into())),
        };
        let imported = semio_s_artifact_cad_cad_io::import_cad_object_by_extension(name, payload).ok_or_else(|| MediaError::Payload(port.to_string(), "unrecognized geometry payload".into()))?;
        cad_import_object_emit(doc, CadPaneId::Shape, "import-media", imported).map_err(|fault| MediaError::Payload(port.to_string(), fault.message))
    }

    /// 🎞️ `brep:out` (WORKFLOWS-END-TO-END-TYPED-PORTS port recipe): exports the cad document's current
    /// brep geometry (every pane's solids fused into one modelspace, same as `saveInPlay`'s STEP export)
    /// wrapped as `Media`. Falls through to the default whole-document `document:out` for any other port.
    fn export_media(port: &str, doc: &ArtifactView<'_, CadSnapshot>) -> Result<Media, MediaError> {
        if port != "brep:out" {
            if port != "artifact:out" {
                return Err(MediaError::NotImplemented);
            }
            let media_type = Self::io().map_or(MediaType { class: MediaClass::ThreeD, form: MediaForm::Brep }, |io| io.artifact_media_type);
            let bytes = <CadSnapshot as pack::snapshot::ArtifactPack>::encode_pack(doc.snapshot);
            return Ok(Media { media_type, payload: MediaPayload::Structured { schema: Self::DOCUMENT_SCHEMA.to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) } });
        }
        let view = CadPlayView::of(doc.snapshot, &doc.children, CadPlayRuntime::default(), CadInteractionSnapshot::default());
        let mut kernel = cad_brep_kernel();
        let solids = collect_modelspace_solids(&mut kernel, &view);
        if solids.is_empty() {
            return Err(MediaError::Payload(port.to_string(), "no solids to export".into()));
        }
        let Some(export) = export_solids_as(&mut kernel, &solids, CAD_SOLID_EXPORT_DIALECT_STEP, "cad.modelspace") else {
            return Err(MediaError::Payload(port.to_string(), "brep export failed".into()));
        };
        let text = match export.data {
            semio_framework_value::DslValue::String(text) => text,
            other => semio_framework_pack_json::to_json_string(&other),
        };
        Ok(Media { media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Brep }, payload: MediaPayload::Structured { schema: "3d.cad".into(), json: base64_codec::base64_standard_encode(text.as_bytes()) } })
    }

    fn command_id(command: &CadCommand) -> &'static str {
        command.command_id()
    }

    fn command_from_action(action: &str, args: &mut Option<semio_framework_value::DslValue>, original_fault: &mut Option<Fault>) -> Result<CadCommand, Fault> {
        if let Some(fault) = original_fault.take() { return Err(fault); }
        let args = args.as_ref();
        cad_command_from_action(action, args)
    }

    fn host_configuration_mutation(action: &str, args: Option<&semio_framework_value::DslValue>) -> Result<Option<Self::ConfigMutation>, Fault> {
        Ok((action == "setContributions").then(|| CadConfigMutation::SetContributions(SetContributionsEdit { json: args.and_then(|value| value.get("json")).and_then(semio_framework_value::DslValue::as_str).unwrap_or("[]").to_string() })))
    }

    fn handle(
        command: &CadCommand,
        doc: &ArtifactView<'_, CadSnapshot>,
        cfg: &ConfigView<'_, CadConfig>,
        interaction: &semio_framework_plugin::app::InteractionView<'_>,
        view_state: Option<&ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<CadMutation, CadConfigMutation, Self::DraftMutation>, Fault> {
        let mut ctx = CadDispatchCtx::new(CadInteractionSnapshot::from_interaction(interaction), view_state.cloned(), CadWorldWindowTransient::default());
        command.dispatch(doc, cfg, &mut ctx)
    }

    /// 🕹️ Interaction-less twin of [`Self::render_with_request_context`] (exports, tests, the
    /// framework's own `render_with_request_context` default body): one render implementation
    /// ([`Self::render_body`]) against an empty `"cad"` domain.
    fn render(body_key: &str, doc: &ArtifactView<'_, CadSnapshot>, cfg: &ConfigView<'_, CadConfig>, view_state: &ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        Self::render_body(body_key, doc, cfg, view_state, &CadWorldWindowTransient::default(), CadInteractionSnapshot::default())
    }

    /// 🕹️ FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: resolves the live `"cad"` domain (selection +
    /// `"pointer"` hover) once per render and threads it through [`CadPlayView`] into every body —
    /// the world scenes' selected/hovered instance flags and gumball, the engagement HUD's selected
    /// count, the inspection panel's selected-object fields and the document tree's marks.
    fn render_with_request_context(
        _owner: &semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
        body_key: &str,
        doc: &ArtifactView<'_, CadSnapshot>,
        cfg: &ConfigView<'_, CadConfig>,
        view_state: &ViewModel,
        transient: &semio_framework_plugin::TransientView<'_, semio_framework_plugin::NoTransient>,
        interaction: &semio_framework_plugin::app::InteractionView<'_>,
        cx:&mut semio_framework_plugin::StepContext<'_>,
    ) -> semio_framework_plugin::OriginalRenderResult<semio_framework_plugin::ComponentTree> {
        let rendered:semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree>=(||{
        Self::render_body(body_key, doc, cfg, view_state, &window_transient::current(transient), CadInteractionSnapshot::from_interaction(interaction))

        })();
        rendered.map(Some).map_err(Into::into)
        }

    /// 🕹️ Interaction-less twin of [`Self::window_engagements_with_request_context`] — the HUD's
    /// selected count reads `0` here, which is correct for an empty `"cad"` domain.
    fn window_engagements(doc: &ArtifactView<'_, CadSnapshot>, cfg: &ConfigView<'_, CadConfig>, view_state: &ViewModel) -> HashMap<String, WindowEngagement> {
        Self::window_engagements_body(doc, cfg, view_state, &CadWorldWindowTransient::default(), CadInteractionSnapshot::default())
    }

    /// 🕹️ The engagement HUD's `"N selected"` status reads the live framework-owned `"cad"` selection —
    /// the same one [`Self::render_with_request_context`] paints the world scenes from, so the HUD and
    /// the viewport can never disagree about what is picked.
    fn window_engagements_with_request_context(
        doc: &ArtifactView<'_, CadSnapshot>,
        cfg: &ConfigView<'_, CadConfig>,
        view_state: &ViewModel,
        transient: &semio_framework_plugin::TransientView<'_, semio_framework_plugin::NoTransient>,
        interaction: &semio_framework_plugin::app::InteractionView<'_>,
    ) -> HashMap<String, WindowEngagement> {
        Self::window_engagements_body(doc, cfg, view_state, &window_transient::current(transient), CadInteractionSnapshot::from_interaction(interaction))
    }

    /// 🪟️ Resolves measures for the exact addressed world-window instance.
    fn window_measures(_doc: &ArtifactView<'_, CadSnapshot>, cfg: &ConfigView<'_, CadConfig>, view_state: &ViewModel) -> HashMap<String, Vec<WindowMeasure>> {
        let Some(window_id) = view_state.window_id.as_deref() else { return HashMap::new() };
        let Some(kind) = view_state.window_instances.iter().find(|window| window.id == window_id).map(|window| window.window_kind_id.as_str()) else { return HashMap::new() };
        let runtime = runtime_of(cfg, &CadWorldWindowTransient::default());
        let is_de = cad_is_de_locale(view_state);
        let measures = match kind {
            shape::WINDOW_KIND_ID => shape::window_measures(&runtime, is_de),
            building::WINDOW_KIND_ID => building::window_measures(&runtime, is_de),
            energy::WINDOW_KIND_ID => energy::window_measures(&runtime, is_de),
            structure_classic::WINDOW_KIND_ID => structure_classic::window_measures(&runtime, is_de),
            _ => return HashMap::new(),
        };
        HashMap::from([(window_id.to_string(), measures)])
    }

    /// 🖱️ Transform/duplicate/delete section for the World3d context menu. Deliberately NOT gated on
    /// "is anything selected": every row already no-ops on an empty `"cad"` selection at dispatch
    /// time, and the section is what makes the menu discoverable before the first pick. The
    /// `InteractionView` is available here now (via `context_menu_with_request_context`), so gating is
    /// a UX decision this app can revisit — it is no longer an SDK limitation.
    fn context_menu(_request: &ContextMenuRequest, _doc: &ArtifactView<'_, CadSnapshot>, _cfg: &ConfigView<'_, CadConfig>, view_state: &ViewModel, registry: &AppActionRegistry) -> Vec<ContextMenuItemSpec> {
        Menu::of(registry, view_state).action("translateSelection").action("rotateSelection").action("scaleSelection").action("duplicateObject").destructive("deleteObject").build()
    }
}
//#endregion 🔖️PlayApp
