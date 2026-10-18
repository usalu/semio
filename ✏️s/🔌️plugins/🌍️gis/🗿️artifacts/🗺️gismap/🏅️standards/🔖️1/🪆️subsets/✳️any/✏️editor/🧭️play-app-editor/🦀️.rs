//! 🧭️ Editor impl of the GIS 2D play app: the context menu, the dispatch-only artifact editor surface and the window actions. See [`super`].

use super::*;
use semio_framework_2d_compute::EngineHandles;

//#region 🔖️Gis2dPlayApp
/// 🕹️ `interactionSelect` args for a single-feature pick against the `"features"` domain's
/// `"feature"` granularity — the generic replacement for the deleted bespoke `setFeatureSelection`
/// action (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM).
fn select_feature_action_args(feature_id: &str) -> semio_framework_value::DslValue {
    let targets = semio_framework_pack_json::json!([{ "granularity": GIS2D_FEATURE_GRANULARITY, "id": feature_id }]).to_string();
    semio_framework_value::DslValue::object([
        ("domainId".to_string(), semio_framework_value::DslValue::String(GIS2D_INTERACTION_DOMAIN.to_string())),
        ("targets".to_string(), semio_framework_value::DslValue::String(targets)),
        ("merge".to_string(), semio_framework_value::DslValue::String("replace".to_string())),
        ("method".to_string(), semio_framework_value::DslValue::String("pick".to_string())),
    ])
}

/// 🖱️ On-demand GIS tiled-map context menu from feature hit-test and selection — grouped
/// disclosure via `Menu::of(registry, view_state)`; `organize_context_menu` (run automatically at the
/// `VcsArtifactApp::context_menu` funnel) sorts the declared `.group(...)` rows into
/// `RIBBON_PARENT_CATEGORIES` taxonomy order and inserts the pre-destructive separator itself.
///
/// 🕹️ `selected_ids` is the live `"features"` domain selection when this is reached through
/// `context_menu_with_request_context`, and empty through the interaction-less `context_menu` twin —
/// it is what decides whether `clearSelection` renders enabled or disabled with its reason.
async fn gis2d_context_menu_items(registry: &semio_framework_plugin::AppActionRegistry, view_state: &semio_framework_plugin::ViewModel, surface: Option<&semio_framework_plugin::ContextMenuSurfaceTarget>, selected_ids: &[String]) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {
    let hits = surface.map_or(&[][..], |s| s.hits.as_slice());
    let feature = hits.iter().find(|h| h.domain == "feature" || h.domain == "position" || h.domain == "route");
    if let Some(feature) = feature {
        let kind = if feature.domain == "route" { "route" } else { "position" };
        let mut menu = Menu::of(registry, view_state)
            .action_args(INTERACTION_SELECT_ACTION_ID, select_feature_action_args(&feature.id))
            .action_args("focusFeature", semio_framework_value::DslValue::object([("featureId".to_string(), semio_framework_value::DslValue::String(feature.id.clone())), ("featureKind".to_string(), semio_framework_value::DslValue::String(kind.to_string()))]));
        if kind == "position" {
            menu = menu.action_args("openSource", semio_framework_value::DslValue::object([("featureId".to_string(), semio_framework_value::DslValue::String(feature.id.clone()))]));
        }
        return menu.build();
    }
    Menu::of(registry, view_state).action("selectAll").action("fitWorld").destructive("clearSelection").when(selected_ids.is_empty(), |menu| menu.disabled_because(&semio_framework_plugin::nothing_selected())).build()
}

impl Gis2dPlayApp {
    /// 🖼️ The one render implementation both trait entry points share.
    fn render_body(body_key: &str, doc: &ArtifactView<'_, GisMapSnapshot>, cfg: &ConfigView<'_, NoConfig>, view_state: &semio_framework_plugin::ViewModel, interaction: &Gis2dInteractionSnapshot) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let config = &map_config::current(cfg);
        let labels = gis2d_labels(view_state);
        match body_key {
            map::GIS2D_PLAY_BODY_COMPOSITE => map::render(doc.snapshot, config, interaction).map(semio_framework_plugin::built_to_component_tree),
            document_panel::GIS2D_PLAY_BODY_ARTIFACT => document_panel::render(config, labels, &semio_framework_plugin::TreeWindows::for_body(view_state, document_panel::GIS2D_PLAY_BODY_ARTIFACT)).map(semio_framework_plugin::built_to_component_tree),
            catalogue_panel::GIS2D_PLAY_BODY_CATALOGUE => catalogue_panel::render(labels, &semio_framework_plugin::TreeWindows::for_body(view_state, catalogue_panel::GIS2D_PLAY_BODY_CATALOGUE)).map(semio_framework_plugin::built_to_component_tree),
            inspection_panel::GIS2D_PLAY_BODY_INSPECTION => inspection_panel::render(doc.snapshot, config, interaction, labels).map(semio_framework_plugin::built_to_component_tree),
            _ => semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}

impl ArtifactEditor for Gis2dPlayApp {
    /// 📚️ Artifact catalogue stamped by `PluginBuilder::editor` onto the navbar dropdown.
    fn examples() -> Vec<semio_framework_plugin::ExampleSource> {
        vec![crate::examples::demo::source()]
    }
    /// 🧩️ Composes `s.stdio.semio@v1/*` children, so every bundle of this surface opens them through the same roster.
    type Members = semio_s_artifact_stdio_semio::SemioMembers;
    type Snapshot = GisMapSnapshot;
    type Mutation = GisMapMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = semio_framework_plugin::NoPresence;
    type PresenceMutation = semio_framework_plugin::NoPresenceMutation;
    type Transient = semio_framework_plugin::NoTransient;
    type TransientMutation = semio_framework_plugin::NoTransientMutation;

    type Command = Gis2dCommand;

    const DIALECT: Dialect = crate::GISMAP_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = GIS_MAP_SCHEMA;

    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(std::sync::Arc::new(Gis2dOneItemPreparationFactory::<Self::Snapshot, Self::Mutation>::default()))
    }

    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: EditorApp<Gis2dPlayApp>,
        owner_file: "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.gis.gismap@1/*#editor",
        artifact_schema: "gis.map",
        factory: "Gis2dRetainedCommandJobFactory",
        factory_type: Gis2dRetainedCommandJobFactory,
        contract: ToolExecutionContract::bounded_first_step(8_192, 64, 64, 16_384, 7_500),
        tools: [
            "setActiveExample", "patchPositions", "patchRoutes", "patchRoute", "addFeature", "moveFeature", "renameFeature", "deleteFeature", "toggleLayerVisibility", "fitWorld", "setCamera", "setRenderMode", "setVectorStyle", "setLodMode", "focusFeature", "setLayerStrokeScale", "openSource", "proposeBoundsRegion",
        ]
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        let controller_id = registry.controller_id().to_string();
        registry.register(Gis2dRetainedCommandJobFactory::new(&controller_id))
    }

    fn build_tool_job(request: ArtifactOwnedToolJobRequest<EditorApp<Self>>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
        if !GIS2D_RETAINED_TOOL_IDS.contains(&request.tool_id.as_str()) {
            return Ok(None);
        }
        if request.command.command_id() != request.tool_id {
            return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("app.command.tool-mismatch"), "the gis map command does not match its exact registered tool or exceeds its declared extent"));
        }
        let tool_id = request.command.command_id();
        let work: Box<dyn ArtifactCommandWork<EditorApp<Self>>> = Box::new(GisMapWindowWork::new(tool_id));
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
            Gis2dCommand::command_id,
            GIS2D_RETAINED_RAW_BYTES,
            GIS2D_RETAINED_WORK_ITEMS,
            work,
        );
        Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }

    fn build_envelope_decode_owner_bundle() -> Option<store::os_store::owned_schema_decode::ArtifactEnvelopeDecodeOwnerBundle<Self::Snapshot, Self::Mutation>> {
        Some(crate::host::owned::gis_map_envelope_decode_owner_bundle())
    }

    fn build_document_store_initialization_job(
        envelope: store::os_store::store_schema::ArtifactEnvelope<Self::Snapshot, Self::Mutation>,
        operation: semio_framework_job::OperationId,
        generation: semio_framework_job::Generation,
        actor: protocol::ActorId,
    ) -> Result<semio_framework_plugin::ArtifactStoreInitializationJob<Self::Snapshot, Self::Mutation>, store::os_store::store_schema::ArtifactEnvelope<Self::Snapshot, Self::Mutation>> {
        Ok(crate::host::owned::gis_map_document_store_initialization_job(envelope, operation, generation, actor))
    }

    fn build_document_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
        Some(Box::new(semio_framework_plugin::ArtifactDocumentStoreDisposer::<Self::Snapshot, Self::Mutation>::new()))
    }

    fn build_transient_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Transient>>> {
        Some(semio_framework_plugin::no_transient_local_root_retirement_factory())
    }

    fn register_window_config_owners(registry: &mut semio_framework_plugin::WindowConfigOwnerRegistry) -> Result<(), Fault> {
        map_config::register(registry)
    }

    fn initial_snapshot() -> GisMapSnapshot {
        crate::standards::v1::subsets::any::io::text::snapshot::default_document()
    }

    fn child_restore_projection(snapshot: &Self::Snapshot) -> Result<semio_framework_composition_admission::ChildRestoreProjection<'_>, Fault> {
        semio_framework_composition_admission::ChildRestoreProjection::from_snapshot(snapshot).map_err(|error| Fault::from(format!("gis map child projection failed: {error}")))
    }

    fn genesis_child_pack(snapshot: &Self::Snapshot, slot: &str, child_id: &str) -> Result<Option<Vec<u8>>,semio_framework_value::ValueError> {
 Ok((||{
        crate::standards::v1::subsets::any::map_composition::genesis_gis_map_child_pack(snapshot, slot, child_id)
    
})())
}

    /// 🔌️ `features:in`/`map:out` (WORKFLOWS-END-TO-END-TYPED-PORTS Wave 2 port recipe) plus the
    /// implicit document ports.
    fn io() -> Option<AppIo> {
        Some(gis2d_io())
    }

    // 🧬️ Whole-document replace has no in-history mutation — `document:in`, `features:in` and
    // `setActiveExample` are document loads (`Effect::LoadDocument`, no history row, no before/after diffing).

    /// 🎞️ `map:out` (see `gis2d_map_media` in `🔖️Io` below) plus the inherited
    /// `document:out` default (the pack of `doc.snapshot`, replicated inline — overriding
    /// `export_media` shadows the trait's provided body for every port on this app, not just the new one).
    fn export_media(port: &str, doc: &ArtifactView<'_, GisMapSnapshot>) -> Result<Media, MediaError> {
        match port {
            "map:out" => Ok(gis2d_map_media(doc.snapshot)),
            "artifact:out" => {
                let media_type = Self::io().map_or(MediaType { class: MediaClass::Data, form: MediaForm::Value }, |io| io.artifact_media_type);
                let bytes = doc.snapshot.encode_pack();
                Ok(Media { media_type, payload: MediaPayload::Structured { schema: Self::DOCUMENT_SCHEMA.to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) } })
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    /// 🎞️ `features:in` loads an incoming `{positions,routes,regions}` descriptor as the document's
    /// feature collections (a generic vector-features sink — not pinned to `2d.map`, so a
    /// `draw`/another `gis2d`'s producer both work) plus the inherited
    /// `document:in` default (replicated inline for the same reason as `export_media`).
    fn import_media(port: &str, media: &Media, doc: &ArtifactView<'_, GisMapSnapshot>) -> Result<Emit<GisMapMutation, NoConfigMutation, Self::DraftMutation>, MediaError> {
        match port {
            "features:in" => {
                let MediaPayload::Structured { json, .. } = &media.payload else {
                    return Err(MediaError::Payload(port.to_string(), "features:in only accepts a Structured JSON payload".into()));
                };
                let incoming = gis_map_document_from_descriptor_json(json);
                let loaded = crate::standards::v1::subsets::any::map_composition::gis_map_snapshot_with_derived_children(GisMapSnapshot { positions: incoming.positions, routes: incoming.routes, regions: incoming.regions, ..doc.snapshot.clone() });
                Ok(semio_framework_plugin::app::document_load_emit(&loaded, Self::DOCUMENT_SCHEMA))
            }
            "artifact:in" => {
                let MediaPayload::Structured { json, .. } = &media.payload else {
                    return Err(MediaError::Payload(port.to_string(), "default document:in importer only accepts a Structured (base64 pack) payload".into()));
                };
                let bytes = store::pack_rt::pack_value_from_base64(json).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
                let snapshot = <GisMapSnapshot as ArtifactPack>::decode_pack(&bytes).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
                Ok(semio_framework_plugin::app::document_load_emit(&snapshot, Self::DOCUMENT_SCHEMA))
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    fn command_id(command: &Gis2dCommand) -> &'static str {
        command.command_id()
    }

    /// 🎯️ Maps host action id + JSON args onto `Gis2dCommand` — React/wgpu still speak the
    /// stringly `{action,args}` wire; this is the typed-command bridge until those call sites send
    /// `OpBinary` bytes directly.
    fn command_from_action(action: &str, args: &mut Option<semio_framework_value::DslValue>, original_fault: &mut Option<Fault>) -> Result<Self::Command, Fault> {
        if let Some(fault) = original_fault.take() { return Err(fault); }
        let args = args.as_ref();
        let args = args.map_or(Value::Null, Value::from);
        let str_arg = |keys: &[&str]| -> Option<String> { keys.iter().find_map(|key| args.get(key).and_then(|value| value.as_str()).map(str::to_string)) };
        let string_list = |key: &str| -> Vec<String> { args.get(key).and_then(|value| value.as_array()).map(|rows| rows.iter().filter_map(|row| row.as_str().map(str::to_string)).collect()).unwrap_or_default() };
        let f64_arg = |keys: &[&str]| -> Option<f64> { keys.iter().find_map(|key| args.get(key).and_then(|value| value.as_f64())) };
        match action {
            "setActiveExample" => Ok(Gis2dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: str_arg(&["exampleId", "example_id", "value"]).unwrap_or_default() })),
            "patchPositions" => {
                Ok(Gis2dCommand::PatchPositions(patch_positions::PatchPositions { positions_json: str_arg(&["positionsJson", "positions_json"]).or_else(|| args.get("positions").map(ToString::to_string)).unwrap_or_else(|| "[]".into()) }))
            }
            "patchRoutes" => Ok(Gis2dCommand::PatchRoutes(patch_routes::PatchRoutes {
                route_ids: {
                    let mut ids = string_list("routeIds");
                    if ids.is_empty() {
                        ids = string_list("route_ids");
                    }
                    ids
                },
                field: str_arg(&["field"]).unwrap_or_default(),
                value: str_arg(&["value"]).unwrap_or_default(),
            })),
            "patchRoute" => Ok(Gis2dCommand::PatchRoute(patch_route::PatchRoute { route_id: str_arg(&["routeId", "route_id"]).unwrap_or_default(), field: str_arg(&["field"]).unwrap_or_default(), value: str_arg(&["value"]).unwrap_or_default() })),
            "addFeature" => Ok(Gis2dCommand::AddFeature(add_feature::AddFeature {
                collection: str_arg(&["collection"]).unwrap_or_else(|| "positions".into()),
                label: str_arg(&["label"]).unwrap_or_default(),
                lon: f64_arg(&["lon"]).unwrap_or_default(),
                lat: f64_arg(&["lat"]).unwrap_or_default(),
                span: f64_arg(&["span"]).unwrap_or(GIS2D_DEFAULT_FEATURE_SPAN),
            })),
            "moveFeature" => Ok(Gis2dCommand::MoveFeature(move_feature::MoveFeature {
                collection: str_arg(&["collection"]).unwrap_or_else(|| "positions".into()),
                feature_id: str_arg(&["featureId", "feature_id"]).unwrap_or_default(),
                lon: f64_arg(&["lon"]).unwrap_or_default(),
                lat: f64_arg(&["lat"]).unwrap_or_default(),
            })),
            "renameFeature" => Ok(Gis2dCommand::RenameFeature(rename_feature::RenameFeature {
                collection: str_arg(&["collection"]).unwrap_or_else(|| "positions".into()),
                feature_id: str_arg(&["featureId", "feature_id"]).unwrap_or_default(),
                label: str_arg(&["label"]).unwrap_or_default(),
            })),
            "deleteFeature" => Ok(Gis2dCommand::DeleteFeature(delete_feature::DeleteFeature {
                collection: str_arg(&["collection"]).unwrap_or_else(|| "positions".into()),
                feature_id: str_arg(&["featureId", "feature_id"]).unwrap_or_default(),
            })),
            "toggleLayerVisibility" => Ok(Gis2dCommand::ToggleLayerVisibility(toggle_layer_visibility::ToggleLayerVisibility { layer_id: str_arg(&["layerId", "layer_id"]).unwrap_or_default() })),
            "fitWorld" => Ok(Gis2dCommand::FitWorld(fit_world::FitWorld {})),
            "setCamera" => {
                let camera_json = str_arg(&["cameraJson", "camera_json"]).or_else(|| args.get("camera").map(|value| if value.is_string() { value.as_str().unwrap_or("{}").to_string() } else { value.to_string() })).unwrap_or_else(|| "{}".into());
                Ok(Gis2dCommand::SetCamera(set_camera::SetCamera { camera_json }))
            }
            "setRenderMode" => Ok(Gis2dCommand::SetRenderMode(set_render_mode::SetRenderMode { value: str_arg(&["value", "renderMode", "render_mode"]).unwrap_or_default() })),
            "setVectorStyle" => Ok(Gis2dCommand::SetVectorStyle(set_vector_style::SetVectorStyle { value: str_arg(&["value", "vectorStyle", "vector_style"]).unwrap_or_default() })),
            "setLodMode" => Ok(Gis2dCommand::SetLodMode(set_lod_mode::SetLodMode { value: str_arg(&["value", "lodMode", "lod_mode"]).unwrap_or_default() })),
            "focusFeature" => {
                Ok(Gis2dCommand::FocusFeature(focus_feature::FocusFeature { feature_id: str_arg(&["featureId", "feature_id"]).unwrap_or_default(), feature_kind: str_arg(&["featureKind", "feature_kind"]).unwrap_or_else(|| "position".into()) }))
            }
            "setLayerStrokeScale" => Ok(Gis2dCommand::SetLayerStrokeScale(set_layer_stroke_scale::SetLayerStrokeScale { layer_id: str_arg(&["layerId", "layer_id"]).unwrap_or_default(), value: f64_arg(&["value"]).unwrap_or(1.0) })),
            "openSource" => Ok(Gis2dCommand::OpenSource(open_source::OpenSource { feature_id: str_arg(&["featureId", "feature_id"]).unwrap_or_default() })),
            "proposeBoundsRegion" => Ok(Gis2dCommand::ProposeBoundsRegion(propose_bounds_region::ProposeBoundsRegion {})),
            other => Err(Fault::from(format!(
                "action '{other}' is not a framework-reserved action (history/clipboard/revert/filter/noteShellCommand) — \
                 app actions are dispatched exclusively through the typed command channel now (see `dispatch_typed_command`)"
            ))),
        }
    }

    fn handle(
        command: &Gis2dCommand,
        doc: &ArtifactView<'_, GisMapSnapshot>,
        cfg: &ConfigView<'_, NoConfig>,
        _interaction: &InteractionView<'_>,
        _view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<GisMapMutation, NoConfigMutation, Self::DraftMutation>, Fault> {
        let mut emit = command.dispatch(doc, cfg)?;
        if let Some(view) = _view_state {
            let config = map_config::current(cfg);
            if let Some(mutation) = gis2d_map_window_mutation(command, doc.snapshot, &config) {
                emit.window_config_mutations.push(map_config::addressed(view, mutation)?);
            }
        }
        Ok(emit)
    }

    /// 🕹️ Interaction-less twin of [`Self::render_with_request_context`] (exports, tests, the
    /// framework's own default body): one render implementation against an empty domain.
    fn render(body_key: &str, doc: &ArtifactView<'_, GisMapSnapshot>, cfg: &ConfigView<'_, NoConfig>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        Self::render_body(body_key, doc, cfg, view_state, &Gis2dInteractionSnapshot::default())
    }

    /// 🕹️ Resolves the live [`GIS2D_INTERACTION_DOMAIN`] (selection + `"pointer"` hover) once per
    /// render and threads it into the map window, so a `TiledMapHost` feature pick round-trips back
    /// as a highlighted feature and a hover popup instead of dying in the framework's store.
    fn render_with_request_context(
        _owner: &semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
        body_key: &str,
        doc: &ArtifactView<'_, GisMapSnapshot>,
        cfg: &ConfigView<'_, NoConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        _transient: &semio_framework_plugin::TransientView<'_, Self::Transient>,
        interaction: &InteractionView<'_>,
        cx:&mut semio_framework_plugin::StepContext<'_>,
    ) -> semio_framework_plugin::OriginalRenderResult<semio_framework_plugin::ComponentTree> {
        let rendered:semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree>=(||{
        Self::render_body(body_key, doc, cfg, view_state, &Gis2dInteractionSnapshot::from_interaction(interaction))

        })();
        rendered.map(Some).map_err(Into::into)
        }

    fn window_measures(doc: &ArtifactView<'_, GisMapSnapshot>, cfg: &ConfigView<'_, NoConfig>, view_state: &semio_framework_plugin::ViewModel) -> HashMap<String, Vec<WindowMeasure>> {
        let config = &map_config::current(cfg);
        HashMap::from([(map::GIS2D_PLAY_WINDOW_MAIN.into(), map::window_measures(doc.snapshot, config, gis2d_labels(view_state)))])
    }

    /// 🖱️ Interaction-less twin of [`Self::context_menu_with_request_context`] — an empty `"features"`
    /// domain, so `clearSelection` renders disabled.
    fn context_menu(
        request: &semio_framework_plugin::ContextMenuRequest,
        _doc: &ArtifactView<'_, GisMapSnapshot>,
        _cfg: &ConfigView<'_, NoConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        registry: &semio_framework_plugin::AppActionRegistry,
    ) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {
        ::semio_framework_async::poll::resolve_ready(async { gis2d_context_menu_items(registry, view_state, request.surface.as_ref(), &[]).await })
    }

    /// 🕹️ `clearSelection` is gated on the AUTHORITATIVE framework-owned `"features"` selection, not on
    /// `ContextMenuRequest.surface.selection` — a layer picked in the document tree never reaches the
    /// map surface's own painted ids, so the surface field alone would leave the row permanently
    /// disabled (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM).
    fn context_menu_with_request_context(
        request: &semio_framework_plugin::ContextMenuRequest,
        _doc: &ArtifactView<'_, GisMapSnapshot>,
        _cfg: &ConfigView<'_, NoConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        interaction: &InteractionView<'_>,
        registry: &semio_framework_plugin::AppActionRegistry,
    ) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {
        let selected = Gis2dInteractionSnapshot::from_interaction(interaction).ids;
        ::semio_framework_async::poll::resolve_ready(async { gis2d_context_menu_items(registry, view_state, request.surface.as_ref(), &selected).await })
    }
}
//#endregion 🔖️Gis2dPlayApp
