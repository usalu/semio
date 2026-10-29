//! 🕹️ Layout play app: the editor app record, its default and constructor and the `ArtifactEditor` implementation with the dispatch, render, media and history hooks. See [`super`].

use super::*;

/// 🧭️ Each render keeps its original preparation and child source owners until retirement.
#[derive(semio_framework_value::RetireOwned)]
pub enum LayoutPanelRenderWork {
    Canvas(semio_s_artifact_layout_layout_editor_commands::canvas::paragraph_work::LayoutRenderWork),
    Inspection(inspection_panel::drawing_work::LayoutInspectorWork),
}
impl LayoutPanelRenderWork {
    fn demand(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        match self { Self::Canvas(work) => work.demand(body), Self::Inspection(work) => work.demand(body) }
    }
}


//#region 🔖️LayoutPlayApp
/// 🧪️ Stateless app shell; exact window owners hold persisted view preferences and ephemeral input.
pub struct LayoutPlayApp;

impl Default for LayoutPlayApp {
    /// 📥️ Opening an app instance is where the host's package assets are readable: the built-in text fonts the host preloaded
    /// on `Event::InstanceOpen` are installed into the layout engine here, before any story is shaped. A host that delivered
    /// none leaves the engine on its own native door or on the layout font alone.
    fn default() -> Self {
        use semio_s_artifact_layout_layout_engine::scene::text_fonts::{TextFontsInstallError, install_package_text_fonts};
        match install_package_text_fonts(semio_framework_plugin::reactor::package_asset) {
            Ok(()) | Err(TextFontsInstallError::AlreadyInstalled | TextFontsInstallError::PackageAssetMissing { .. }) => {}
            Err(error) => eprintln!("[ERROR] layout: the host's built-in text fonts were refused: {error}"),
        }
        Self
    }
}

impl LayoutPlayApp {
    fn render_body(
        body_key: &str,
        doc: &ArtifactView<'_, LayoutSnapshot>,
        cfg: &ConfigView<'_, NoConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        transient: &LayoutWindowTransient,
        interaction: &LayoutInteractionSnapshot,
    ) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let document = doc.snapshot;
        let mut config = blueprint::config::current(cfg);
        if body_key == LAYOUT_PLAY_BODY_BLUEPRINT {
            if let Some(utility) = view_state.active_utility_id.as_deref().filter(|utility| !utility.is_empty()) {
                config.active_utility = utility.to_string();
            }
        }
        let labels = layout_labels(view_state);
        match body_key {
            LAYOUT_PLAY_BODY_BLUEPRINT => match transient.transform_tool.as_deref() {
                Some(tool) => blueprint::render(&blueprint::transform::layout_transform_tool_preview(document, tool), &config, transient, interaction),
                None => blueprint::render(document, &config, transient, interaction),
            },
            LAYOUT_PLAY_BODY_PREVIEW => preview::render(document, &config, transient, interaction),
            LAYOUT_PLAY_BODY_ARTIFACT => document_panel::render(document, &config, labels, &semio_framework_plugin::TreeWindows::for_body(view_state, LAYOUT_PLAY_BODY_ARTIFACT)),
            LAYOUT_PLAY_BODY_CATALOGUE => catalogue_panel::render(labels, &semio_framework_plugin::TreeWindows::for_body(view_state, LAYOUT_PLAY_BODY_CATALOGUE)),
            LAYOUT_PLAY_BODY_INSPECTION => inspection_panel::render(document, &config, interaction, labels, &semio_framework_plugin::TreeWindows::for_body(view_state, LAYOUT_PLAY_BODY_INSPECTION)),
            LAYOUT_PLAY_BODY_PREFLIGHT => preflight_panel::render(document, labels, &semio_framework_plugin::TreeWindows::for_body(view_state, LAYOUT_PLAY_BODY_PREFLIGHT)),
            _ => semio_framework_plugin::built_text_node(Label::data(format!("Unknown body: {body_key}"))).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "layout error text admission failed")),
        }
        .map(semio_framework_plugin::built_to_component_tree)
    }
}

impl ArtifactEditor for LayoutPlayApp {
    /// 📚️ Artifact catalogue stamped by `PluginBuilder::editor` onto the navbar dropdown.
    fn examples() -> Vec<semio_framework_plugin::ExampleSource> {
        vec![semio_s_artifact_layout_layout_examples::demo::source()]
    }
    type Snapshot = LayoutSnapshot;
    type Mutation = LayoutMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = semio_framework_plugin::NoPresence;
    type PresenceMutation = semio_framework_plugin::NoPresenceMutation;
    type Transient = semio_framework_plugin::NoTransient;
    type TransientMutation = semio_framework_plugin::NoTransientMutation;

    type Command = LayoutCommand;

    const DIALECT: Dialect = semio_s_artifact_layout_layout_model::LAYOUT_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = semio_s_artifact_layout_layout_model::LAYOUT_DOCUMENT_SCHEMA;

    fn reference_birth()->Option<fn()->Result<LayoutSnapshot,semio_framework_plugin::Fault>>{Some(||{
        Ok(semio_s_artifact_layout_layout_model::io::text::snapshot::snapshot_wire_codec::default_document())
    })}

    fn io() -> Option<semio_framework_plugin::AppIo> {
        Some(semio_s_artifact_layout_layout_engine::layout_io())
    }

    /// 🏷️ Supplied wholesale by `app_commands!`'s generated `command_id()`.
    fn command_id(command: &LayoutCommand) -> &'static str {
        command.command_id()
    }

    /// 🎯️ Host-action bridge into the closed `LayoutCommand` enum (ticket
    /// 26/09/16/LAYOUT-PLUGIN-END-TO-END). The React/wgpu shells still speak `{action, args}` with
    /// camelCase argument keys; every `🎮️commands/*` payload derives `FromValue` over its own
    /// snake_case field names, so this boundary folds the keys and decodes — the default trait impl
    /// refuses every app action outright, which left every panel/canvas verb dead in the shell.
    fn command_from_action(action: &str, args: &mut Option<DslValue>, original_fault: &mut Option<Fault>) -> Result<Self::Command, Fault> {
        if let Some(fault) = original_fault.take() { return Err(fault); }
        let args = args.as_ref();
        args_bridge::command_from_action(action, args)
    }

    /// 📨️ Every host event ends the window's open transform-tool gesture with zero trace under the reason the tool
    /// records: a blur `blur`, a lost pointer capture `captureLost`, a utility switch or a closing window `retired`, an
    /// opened history edit `frozen` and a remote edit `baseMoved` — the window's typed `translateSelection{phase: "abort"}`.
    fn host_event(event: &semio_framework_plugin::HostEvent) -> Option<Self::Command> {
        use semio_framework_plugin::HostEvent;
        use semio_framework_tool_machine::ToolAbortReason;
        let reason = match event {
            HostEvent::WindowBlurred { .. } => ToolAbortReason::Blur,
            HostEvent::PointerCaptureLost { .. } => ToolAbortReason::CaptureLost,
            HostEvent::UtilityChanged { .. } | HostEvent::Retiring { .. } => ToolAbortReason::Retired,
            HostEvent::TimeTravelFrozen { .. } => ToolAbortReason::Frozen,
            HostEvent::BaseMoved { .. } => ToolAbortReason::BaseMoved,
        };
        Some(LayoutCommand::TranslateSelection(gumball::TranslateSelection { ids: Vec::new(), dx: 0.0, dy: 0.0, phase: Some("abort".into()), reason: Some(reason.as_str().into()) }))
    }

    /// 🪧️ A history-edit reference chip names a frame by its kind, content and page ([`layout_entity_label`]).
    fn entity_label(snapshot: &LayoutSnapshot, kinds: &[String], id: &str) -> Option<LocalizedLabel> {
        layout_entity_label(snapshot, kinds, id)
    }

    fn build_transient_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Transient>>> {
        Some(semio_framework_plugin::no_transient_local_root_retirement_factory())
    }

    /// 🧾️ Store publication authority for the `Artifact` lane — without it the host refuses every
    /// document verb at dispatch (`declares the unsupported artifact publication lane`). The paged document-lane
    /// preparation clones the snapshot page by page, so `CreatePage` (a whole page with its layers) and
    /// `ChangeDataFields` (a `fields:in` dictionary) are bounded only by their own encoded mutation footprint
    /// (`ArtifactStoreOneItemFootprint::for_leaf`).
    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(store::cold_mutation_apply_preparation_factory::<Self::Snapshot, Self::Mutation>())
    }

    fn register_window_config_owners(registry: &mut semio_framework_plugin::WindowConfigOwnerRegistry) -> Result<(), Fault> {
        blueprint::config::register(registry)
    }

    fn register_window_transient_owners(registry: &mut semio_framework_plugin::WindowTransientOwnerRegistry) -> Result<(), Fault> {
        blueprint::transient::register(registry)
    }

    fn bounded_first_step_tool_proofs() -> Vec<semio_framework_plugin::ArtifactBoundedFirstStepProof> {
        LayoutRetainedProofs::bounded_first_step_tool_proofs().into_iter().chain(LayoutExportProofs::bounded_first_step_tool_proofs()).collect()
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(LayoutRetainedCommandJobFactory::new(&controller))?;
        registry.register(LayoutExportJobFactory::new(&controller))?;
        registry.register(LayoutMediaExportJobFactory::new(&controller))
    }

    fn build_tool_job(request: &mut Option<ArtifactOwnedToolJobRequest<EditorApp<Self>>>, output: &mut Option<semio_framework_plugin::ErasedToolJob>, cx: &mut semio_framework_plugin::__job::StepContext<'_>) -> Result<bool, semio_framework_plugin::__job::OriginalTurnFailure> {
        let original = request.as_ref().ok_or_else(|| semio_framework_plugin::__value::ValueError::literal(semio_framework_plugin::__value::ValueRefusalKind::InvariantViolated, "original retained producer request is absent"))?;
        if !LAYOUT_RETAINED_TOOL_IDS.contains(&original.tool_id.as_str()) {
            return layout_build_export_tool_job(request, output, cx);
        }
        if original.command.command_id() != original.tool_id {
            return Err(semio_framework_plugin::__value::ValueError::literal(semio_framework_plugin::__value::ValueRefusalKind::InvariantViolated, "layout retained command does not match its exact registered tool").into());
        }
        let tool_id = original.command.command_id();
        semio_framework_plugin::retained_command::producer::admit_retained_tool_producer(request, output, cx, LayoutCommand::command_id, LAYOUT_RETAINED_RAW_BYTES, LAYOUT_RETAINED_WORK_ITEMS, false, |request| LayoutWindowWork::new(tool_id))
    }

    /// 🎞️ `import-media` is the only reserved route layout owns: every inbound `fields:in` delivery is
    /// routed through this builder (`dispatch_import_media` → `build_artifact_reserved_media_job`);
    /// `copy`/`cut`/`paste` stay on the framework's own reserved factories.
    fn build_reserved_tool_job(request: ArtifactReservedToolJobRequest<EditorApp<Self>>) -> Result<Option<ArtifactReservedToolJob>, Fault> {
        if request.tool_id.as_str() != LAYOUT_IMPORT_TOOL_ID {
            return Ok(None);
        }
        if !request.raw_wire.is_empty() {
            return Err(Fault::from("layout import-media admits a decoded media value, never a wire payload"));
        }
        let ArtifactReservedToolInput::Media { port, media } = &request.input else {
            return Err(Fault::from("layout import-media requires media input"));
        };
        let (port, media) = (port.clone(), media.clone());
        Ok(Some(ArtifactReservedToolJob::new(LayoutImportJob::new(request, port, media))))
    }

    fn build_media_export_job(request:&mut Option<ArtifactMediaExportJobRequest<EditorApp<Self>>>,body:&mut Option<Box<dyn semio_framework_plugin::ArtifactReservedJob+Send>>,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        use semio_s_artifact_layout_layout_engine_export::LAYOUT_MEDIA_EXPORT_TOOL_ID;
        if request.as_ref().is_some_and(|input|input.port!="layout:out"||input.tool_id!=LAYOUT_MEDIA_EXPORT_TOOL_ID){return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"layout producer requires its exact registered output port").into())}
        LayoutMediaExportJob::admit_original(request,body,cx)
    }

    fn handle(
        command: &LayoutCommand,
        doc: &ArtifactView<'_, LayoutSnapshot>,
        cfg: &ConfigView<'_, NoConfig>,
        interaction: &InteractionView<'_>,
        _view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<LayoutMutation, NoConfigMutation, Self::DraftMutation>, Fault> {
        let gumball_verb = match command {
            LayoutCommand::TranslateSelection(payload) => Some(payload.verb()),
            LayoutCommand::RotateSelection(payload) => Some(payload.verb()),
            LayoutCommand::ScaleSelection(payload) => Some(payload.verb()),
            _ => None,
        };
        let mut emit = match (command, gumball_verb) {
            (_, Some(verb)) => gumball::transform(verb, doc, cfg, &interaction.selection(LAYOUT_INTERACTION_ELEMENTS).ids, None, _view_state.map(|view| layout_labels(view).frames_locked.as_str())).emit,
            (LayoutCommand::DeleteSelection(payload), None) => delete_selection::apply(payload, doc, cfg, interaction)?,
            _ => command.dispatch(doc, cfg)?,
        };
        if let (LayoutCommand::AddPage(_), Some(view)) = (command, _view_state) {
            let mut config = blueprint::config::current(cfg);
            config.active_page_id = format!("page-{}", doc.snapshot.pages.len() + 1);
            emit.window_config_mutations.push(blueprint::config::addressed(view, config)?);
        }
        Ok(emit)
    }

    //#region 🔖️Media
    /// 🎞️ `document:out` is the bounded document-pack batch recipe. Interactive `layout:out`
    /// is intercepted by the exact registered media-export factory before this reducer is reached;
    /// direct reducer reachability therefore fails closed instead of completing an export inline.
    fn export_media(port: &str, doc: &ArtifactView<'_, LayoutSnapshot>) -> Result<Media, MediaError> {
        match port {
            "artifact:out" => {
                let bytes = pack::snapshot::ArtifactPack::encode_pack(doc.snapshot);
                Ok(Media { media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Vector }, payload: MediaPayload::Structured { schema: semio_s_artifact_layout_layout_model::LAYOUT_DOCUMENT_SCHEMA.into(), json: store::pack_rt::pack_value_to_base64(&bytes) } })
            }
            "layout:out" => Err(MediaError::NotImplemented),
            _ => Err(MediaError::NotImplemented),
        }
    }

    /// 🎞️ WORKFLOWS-END-TO-END-TYPED-PORTS port recipe: `fields:in` binds the incoming `form.dictionary`
    /// values into `LayoutSnapshot::data_fields` — layout has no existing text-interpolation/
    /// field-binding concept for frames/stories yet, so this stores the dictionary verbatim as a new
    /// named data source (see `crate::LayoutSnapshot::data_fields`'s doc) rather
    /// than wiring it into rendering today.
    fn import_media(port: &str, media: &Media, _doc: &ArtifactView<'_, LayoutSnapshot>) -> Result<Emit<LayoutMutation, NoConfigMutation, Self::DraftMutation>, MediaError> {
        match port {
            "artifact:in" => {
                let document = match &media.payload {
                    MediaPayload::Structured { schema, json } if schema == semio_s_artifact_layout_layout_model::LAYOUT_DOCUMENT_SCHEMA => {
                        let bytes = store::pack_rt::pack_value_from_base64(json).map_err(|error| MediaError::Payload(port.into(), error.to_string()))?;
                        <LayoutSnapshot as pack::snapshot::ArtifactPack>::decode_pack(&bytes).map_err(|error| MediaError::Payload(port.into(), error.to_string()))?
                    },
                    MediaPayload::Intrinsic { schema, value } if schema == semio_s_artifact_layout_layout_model::LAYOUT_DOCUMENT_SCHEMA => {
                        <LayoutSnapshot as semio_framework_value::FromValue>::from_value(value.clone()).map_err(|error| MediaError::Payload(port.into(), error.to_string()))?
                    },
                    _ => return Err(MediaError::Payload(port.into(), "artifact:in requires a composed native layout snapshot".into())),
                };
                Ok(Emit::mutations(vec![LayoutMutation::ReplaceDocument(semio_s_artifact_layout_layout_mutations_content::replace_document::ReplaceDocument { document })]))
            },
            "fields:in" => {
                let MediaPayload::Intrinsic {schema,value}= &media.payload else{return Err(MediaError::Payload(port.to_string(),"fields:in requires an intrinsic form dictionary".into()))};
                if schema!="form.dictionary"{return Err(MediaError::Payload(port.to_string(),"fields:in requires the declared form.dictionary schema".into()))}
                let mut callback=|_|true;let mut control=semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new(usize::MAX,&mut callback);
                let dictionary=semio_framework_value_dictionary::QuestionDictionary::from_intrinsic_controlled(value,&mut control).map_err(|error|MediaError::Payload(port.to_string(),error.into_message()))?;
                Ok(Emit::mutations(vec![LayoutMutation::ChangeDataFields(ChangeDataFields {new_fields:Some(dictionary)})]))
            }
            _ => Err(MediaError::NotImplemented),
        }
    }
    //#endregion 🔖️Media

    fn render(body_key: &str, doc: &ArtifactView<'_, LayoutSnapshot>, cfg: &ConfigView<'_, NoConfig>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        Self::render_body(body_key, doc, cfg, view_state, &LayoutWindowTransient::default(), &LayoutInteractionSnapshot::default())
    }

    fn window_engagements(_doc: &ArtifactView<'_, LayoutSnapshot>, cfg: &ConfigView<'_, NoConfig>, view_state: &semio_framework_plugin::ViewModel) -> HashMap<String, WindowEngagement> {
        let config = blueprint::config::current(cfg);
        let labels = layout_labels(view_state);
        let Some(window_id) = view_state.window_id.clone() else { return HashMap::new() };
        HashMap::from([(window_id, layout_window_engagement(&config, &LayoutWindowTransient::default(), "window", labels))])
    }

    type RenderWork=LayoutPanelRenderWork;
    fn uses_render_work(body_key:&str)->bool{matches!(body_key,LAYOUT_PLAY_BODY_BLUEPRINT|LAYOUT_PLAY_BODY_PREVIEW|LAYOUT_PLAY_BODY_INSPECTION)}
    fn begin_render_work(body_key:&str,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<Self::RenderWork>,semio_framework_job::OriginalTurnFailure>{let demand=semio_framework_value::RetirementDemand{copy_bytes:std::mem::size_of::<Self::RenderWork>(),depth:1,..Default::default()};if cx.should_yield()||cx.is_cancelled()||!cx.retained_work_can_enter(demand){return Ok(None);}let work=if body_key==LAYOUT_PLAY_BODY_INSPECTION{LayoutPanelRenderWork::Inspection(inspection_panel::drawing_work::LayoutInspectorWork::new())}else{LayoutPanelRenderWork::Canvas(semio_s_artifact_layout_layout_editor_commands::canvas::paragraph_work::LayoutRenderWork::new())};cx.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})?;cx.consume_fuel(1);Ok(Some(work))}
    fn render_work_demand(work:&Self::RenderWork,maximum_body_bytes:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{work.demand(maximum_body_bytes)}
    fn render_with_work_context(work:&mut Self::RenderWork,_owner:&semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,body_key:&str,_source:&std::sync::Arc<LayoutSnapshot>,doc:&ArtifactView<'_,LayoutSnapshot>,cfg:&ConfigView<'_,NoConfig>,view_state:&semio_framework_plugin::ViewModel,transient:&semio_framework_plugin::TransientView<'_,semio_framework_plugin::NoTransient>,interaction:&InteractionView<'_>,cx:&mut semio_framework_job::StepContext<'_>)->semio_framework_plugin::OriginalRenderResult<semio_framework_plugin::ComponentTree>{
        match work {
            LayoutPanelRenderWork::Inspection(work) => {
                let config=blueprint::config::current(cfg);
                if !work.advance(doc,&config,cx)? { return Ok(None); }
                work.render(doc.snapshot,&config,&LayoutInteractionSnapshot::from_interaction(interaction),layout_labels(view_state),&semio_framework_plugin::TreeWindows::for_body(view_state,LAYOUT_PLAY_BODY_INSPECTION)).map(semio_framework_plugin::built_to_component_tree).map(Some).map_err(Into::into)
            }
            LayoutPanelRenderWork::Canvas(work) => {if cx.should_yield()||cx.is_cancelled(){return Ok(None);}if work.needs_inputs(){let mut config=blueprint::config::current(cfg);if body_key==LAYOUT_PLAY_BODY_BLUEPRINT{if let Some(utility)=view_state.active_utility_id.as_deref().filter(|utility|!utility.is_empty()){config.active_utility=utility.to_string();}}work.prepare(doc.snapshot,&config,&blueprint::transient::current(transient),&LayoutInteractionSnapshot::from_interaction(interaction),body_key==LAYOUT_PLAY_BODY_BLUEPRINT).map_err(|_|semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue,"layout original render preparation failed"))?;cx.consume_fuel(1);return Ok(None);}if !work.advance(doc.snapshot,cx)?{return Ok(None);}work.take_tree().map(Some).map_err(Into::into)}
        }
    }


    fn render_with_request_context(
        _owner: &semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
        body_key: &str,
        doc: &ArtifactView<'_, LayoutSnapshot>,
        cfg: &ConfigView<'_, NoConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        transient: &semio_framework_plugin::TransientView<'_, semio_framework_plugin::NoTransient>,
        interaction: &InteractionView<'_>,
        cx:&mut semio_framework_plugin::StepContext<'_>,
    ) -> semio_framework_plugin::OriginalRenderResult<semio_framework_plugin::ComponentTree> {
        let rendered:semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree>=(||{
        Self::render_body(body_key, doc, cfg, view_state, &blueprint::transient::current(transient), &LayoutInteractionSnapshot::from_interaction(interaction))

        })();
        rendered.map(Some).map_err(Into::into)
        }

    fn window_engagements_with_request_context(
        doc: &ArtifactView<'_, LayoutSnapshot>,
        cfg: &ConfigView<'_, NoConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        transient: &semio_framework_plugin::TransientView<'_, semio_framework_plugin::NoTransient>,
        _interaction: &InteractionView<'_>,
    ) -> HashMap<String, WindowEngagement> {
        let mut engagements = Self::window_engagements(doc, cfg, view_state);
        let Some(window_id) = view_state.window_id.as_ref() else { return HashMap::new() };
        if let Some(engagement) = engagements.get_mut(window_id) {
            *engagement = layout_window_engagement(&blueprint::config::current(cfg), &blueprint::transient::current(transient), "window", layout_labels(view_state));
        }
        engagements
    }

    fn context_menu(
        request: &ContextMenuRequest,
        doc: &ArtifactView<'_, LayoutSnapshot>,
        _cfg: &ConfigView<'_, NoConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        registry: &AppActionRegistry,
    ) -> Vec<ContextMenuItemSpec> {
        layout_context_menu_items(registry, doc.snapshot, layout_labels(view_state), view_state, request.surface.as_ref(), &[])
    }

    fn context_menu_with_request_context(
        request: &ContextMenuRequest,
        doc: &ArtifactView<'_, LayoutSnapshot>,
        _cfg: &ConfigView<'_, NoConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        interaction: &InteractionView<'_>,
        registry: &AppActionRegistry,
    ) -> Vec<ContextMenuItemSpec> {
        let fallback = LayoutInteractionSnapshot::from_interaction(interaction).ids;
        layout_context_menu_items(registry, doc.snapshot, layout_labels(view_state), view_state, request.surface.as_ref(), &fallback)
    }
}
//#endregion 🔖️LayoutPlayApp
