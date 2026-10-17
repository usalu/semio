//! 🕹️ Lowpoly play app: the `LowpolyPlayApp` editor with its media export, its render and the `ArtifactEditor` implementation.

use super::*;
use semio_s_artifact_lowpoly_lowpoly_modes::windows::model::LOWPOLY_PLAY_BODY_MAIN;
use semio_s_artifact_lowpoly_lowpoly_modes::editor::lowpoly::modes::paint::windows::uv::LOWPOLY_PLAY_BODY_UV;
use semio_s_artifact_lowpoly_lowpoly_panels::document::LOWPOLY_PLAY_BODY_ARTIFACT;
use semio_s_artifact_lowpoly_lowpoly_panels::inspection::LOWPOLY_PLAY_BODY_INSPECTION;
use semio_s_artifact_lowpoly_lowpoly_kit::reset_document_effect;
use semio_s_artifact_lowpoly_lowpoly_modes::lowpoly_window_engagement;
use semio_s_artifact_lowpoly_lowpoly_modes::lowpoly_window_engagement_with_selection;
use semio_s_artifact_lowpoly_lowpoly_modes::lowpoly_window_measures;

fn lowpoly_export_media(port: &str, doc: &ArtifactView<'_, LowpolySnapshot>, scratch: &LowpolyScratch) -> Result<Media, MediaError> {
    match port {
        "mesh:out" => {
            let mesh = semio_s_artifact_lowpoly_lowpoly_engine::lowpoly_mesh_from_document(doc.snapshot, &scratch.mesh_workspace_map()).map_err(|error| MediaError::Payload(port.into(), error))?;
            let mesh_document = semio_s_artifact_lowpoly_lowpoly_model::mesh_document_value(&mesh);
            let json = semio_framework_pack_json::to_json_string(&mesh_document);
            Ok(Media { media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Mesh }, payload: MediaPayload::Structured { schema: "mesh.document".into(), json } })
        }
        "artifact:out" => {
            let media_type = LowpolyPlayApp::io().map_or(MediaType { class: MediaClass::ThreeD, form: MediaForm::Mesh }, |io| io.artifact_media_type);
            let bytes = doc.snapshot.encode_pack();
            Ok(Media { media_type, payload: MediaPayload::Structured { schema: LOWPOLY_DOCUMENT_SCHEMA.to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) } })
        }
        _ => Err(MediaError::NotImplemented),
    }
}

fn lowpoly_render(
    body_key: &str,
    doc: &ArtifactView<'_, LowpolySnapshot>,
    cfg: &ConfigView<'_, LowpolyConfig>,
    view_state: &semio_framework_plugin::ViewModel,
    scratch: &mut LowpolyScratch,
    interaction: Option<&InteractionView<'_>>,
    preview: Option<&LowpolySnapshot>,
) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
    let projection = preview.unwrap_or(doc.snapshot);
    let config = cfg.snapshot;
    let empty_domain_selection = protocol::DomainSelection::default();
    let domain_selection = interaction.map_or(&empty_domain_selection, |interaction| interaction.selection(MESH_INTERACTION_DOMAIN));
    let world_selection = semio_s_artifact_lowpoly_lowpoly_view::world_selection_with_hover(projection, config, domain_selection, interaction.and_then(|interaction| interaction.active_granularity(MESH_INTERACTION_DOMAIN)), interaction.map(|interaction| interaction.hover(MESH_INTERACTION_DOMAIN, "pointer")));
    scratch.set_selection_object_id(semio_s_artifact_lowpoly_lowpoly_view::selection_object_id(projection, domain_selection));
    let labels = semio_s_artifact_lowpoly_lowpoly_view::editor::lowpoly::terminology::lowpoly_play_labels(view_state);
    let active_utility = view_state.active_utility_id.as_deref().filter(|utility| !utility.is_empty()).unwrap_or("move");
    if matches!(body_key, LOWPOLY_PLAY_BODY_MAIN | LOWPOLY_PLAY_BODY_UV) {
        scratch.refresh_texture_cache(projection);
    }
    let texture_cache = scratch.textures().clone();
    let view = LowpolyView { snapshot: projection, config };
    let loaded = matches!(body_key, LOWPOLY_PLAY_BODY_MAIN | LOWPOLY_PLAY_BODY_UV | LOWPOLY_PLAY_BODY_ARTIFACT).then(|| semio_s_artifact_lowpoly_lowpoly_view::build_doc(projection, config, scratch)).flatten();
    let node = match body_key {
        LOWPOLY_PLAY_BODY_MAIN => edit::windows::model::render(view, loaded.as_ref(), active_utility, &texture_cache, &world_selection),
        LOWPOLY_PLAY_BODY_UV => paint_mode::windows::uv::render(view, loaded.as_ref(), &texture_cache),
        LOWPOLY_PLAY_BODY_ARTIFACT => match &loaded {
            Some(loaded) => document_panel::render(view, loaded, labels, &semio_framework_plugin::TreeWindows::for_body(view_state, LOWPOLY_PLAY_BODY_ARTIFACT)),
            None => semio_framework_plugin::built_text_node(semio_framework_ui_locale::Label::data("Failed to load lowpoly document"))
                .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "lowpoly document failed-load text admission failed")),
        },
        LOWPOLY_PLAY_BODY_CATALOGUE => catalogue_panel::render(labels, &semio_framework_plugin::TreeWindows::for_body(view_state, LOWPOLY_PLAY_BODY_CATALOGUE)),
        LOWPOLY_PLAY_BODY_INSPECTION => inspection_panel::render(view, active_utility, labels, &semio_framework_plugin::TreeWindows::for_body(view_state, LOWPOLY_PLAY_BODY_INSPECTION)),
        LOWPOLY_PLAY_BODY_LAYERS => layers_panel::render(view, labels, &semio_framework_plugin::TreeWindows::for_body(view_state, LOWPOLY_PLAY_BODY_LAYERS)),
        _ => semio_framework_plugin::built_text_node(semio_framework_ui_locale::Label::data(format!("Unknown body: {body_key}")))
            .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "lowpoly unknown-body text admission failed")),
    }?;
    Ok(semio_framework_plugin::built_to_component_tree(node))
}

//#region 🔖️LowpolyPlayApp
/// 🖌️ B1: sheds `RefCell<LowpolyPlayRuntime>` entirely — every former runtime field now lives in
/// `LowpolyConfig`, written through `LowpolyConfigMutation`s emitted from `handle`. The one remaining
/// field is genuine mid-gesture scratch state (`LowpolyScratch`) — the "scratch + commit" pattern the
/// `ArtifactEditor` trait itself sanctions for `&self`-only `handle`/`render`.
#[derive(Default, Clone, Copy)]
pub struct LowpolyPlayApp;

impl ArtifactEditor for LowpolyPlayApp {
    /// 📚️ Artifact catalogue stamped by `PluginBuilder::editor` onto the navbar dropdown.
    fn examples() -> Vec<semio_framework_plugin::ExampleSource> {
        vec![semio_s_artifact_lowpoly_lowpoly_examples::source()]
    }
    type Snapshot = LowpolySnapshot;
    type Mutation = LowpolyMutation;
    type Config = LowpolyConfig;
    type ConfigMutation = LowpolyConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = semio_s_artifact_lowpoly_lowpoly_presence::LowpolyPresence;
    type PresenceMutation = semio_s_artifact_lowpoly_lowpoly_presence::LowpolyPresenceMutation;
    type Transient = LowpolyTransient;
    type TransientMutation = LowpolyTransientMutation;

    type Command = LowpolyCommand;

    const DIALECT: semio_framework_plugin::app::Dialect = semio_s_artifact_lowpoly_lowpoly_model::snapshot::LOWPOLY_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = LOWPOLY_DOCUMENT_SCHEMA;

    /// 🖱️ "Use selection" over the mesh domain: a row `lowpoly-document.<object>` names that object for an `object`
    /// reference, and a row `lowpoly-document.<object>.<granularity>.<n>` names the component number `n` for a reference of
    /// that granularity (`vertex`, `edge`, `face`); any other row names nothing.
    fn selection_reference_id(kinds: &[String], row: &str) -> Option<String> {
        let (object_id, component) = semio_s_artifact_lowpoly_lowpoly_view::parse_mesh_target_id(row)?;
        match component {
            None => kinds.iter().any(|kind| kind == "object").then_some(object_id),
            Some((granularity, number)) => kinds.iter().any(|kind| *kind == granularity).then(|| number.to_string()),
        }
    }

    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(std::sync::Arc::new(LowpolyArtifactStorePreparationFactory))
    }

    fn build_config_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Config, Self::ConfigMutation>>> {
        Some(std::sync::Arc::new(LowpolyConfigStorePreparationFactory))
    }

    // 🧹️ Registered-app store ownership: without owners + disposers for every lane the mounted instance
    // faults `interactive-job.close-owned-disposer-missing` and threaded commands cannot republish scratch.
    /// 🎯️ What a mesh-domain interaction verb repaints. Hover: the Model scene only (the guest echoes
    /// `hoveredComponent` there; the rail and panels never show a hover). A moved selection: the scene,
    /// the Artifact tree and Inspection panels, the engagement rail (`N faces selected`) and the
    /// measures. A granularity or mode switch: the scene (the pick targets change), the engagement
    /// rail and the measures — the framework default refreshes measures but never engagements, which
    /// left the rail's pressed granularity stale until the next pick (2026-09-18).
    fn interaction_scope(verb: semio_framework_plugin::InteractionVerb, domains: &[&str]) -> Option<semio_framework::kernel::UiDirtyScope> {
        use semio_framework::kernel::UiDirtyScope;
        use semio_framework_plugin::InteractionVerb;
        if domains.is_empty() || domains.iter().any(|domain| *domain != MESH_INTERACTION_DOMAIN) {
            return None;
        }
        let windows = vec![LOWPOLY_PLAY_BODY_MAIN.to_string(), LOWPOLY_PLAY_BODY_UV.to_string()];
        Some(match verb {
            InteractionVerb::Hover => UiDirtyScope::Partial { window_bodies: vec![LOWPOLY_PLAY_BODY_MAIN.to_string()], panel_bodies: Vec::new(), utilities: false, tools: false, engagements: false, measures: false, labels: false },
            InteractionVerb::Select | InteractionVerb::ClearSelection | InteractionVerb::SelectAll => UiDirtyScope::Partial {
                window_bodies: windows,
                panel_bodies: vec![LOWPOLY_PLAY_BODY_ARTIFACT.to_string(), LOWPOLY_PLAY_BODY_INSPECTION.to_string()],
                utilities: false,
                tools: false,
                engagements: true,
                measures: true,
                labels: false,
            },
            InteractionVerb::SetSelectionMode | InteractionVerb::SetGranularity => UiDirtyScope::Partial { window_bodies: windows, panel_bodies: Vec::new(), utilities: false, tools: false, engagements: true, measures: true, labels: false },
        })
    }

    fn build_transient_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactEphemeralOneItemPreparationFactory<Self::Transient, Self::TransientMutation>>> {
        Some(semio_framework_plugin::bounded_transient_preparation_factory::<Self::Transient, Self::TransientMutation>())
    }

    fn build_transient_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Transient>>> {
        Some(semio_framework_plugin::bounded_transient_root_retirement_factory::<Self::Transient>())
    }

    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: EditorApp<LowpolyPlayApp>,
        owner_file: "✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.lowpoly.lowpoly@1/*#editor",
        artifact_schema: "lowpoly.document",
        factory: "LowpolyCommandJobFactory",
        factory_type: LowpolyCommandJobFactory,
        tools: {
            "patchObject" => ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "addPaintLayer" => ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "setActiveObject" => ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "setActivePaintLayer" => ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "setUtilityParam" => ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "engagementInput" => ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "toggleShowEdges" => ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "toggleSun" => ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "setSunAzimuth" => ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "setSunElevation" => ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "setSunIntensity" => ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "setCamera" => ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "importSnapshotJson" => ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "replaceSnapshotJson" => ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "exportMesh" => ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "loadMeshRequest" => ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            // 📥️ Every row must equal the ONE contract `LowpolyCommandJobFactory` registers
            // (`registration.contract == row.contract`, else the boot-time proof join refuses the tool);
            // the mesh file rides inside that shared wire budget.
            "importMeshFile" => ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "deleteSelection" => ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "duplicateObject" => ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "paintSample" => ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "extrude" => ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "inset" => ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "bevel" => ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "loopCut" => ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "subdivide" => ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "triangulate" => ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "mirror" => ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "decimate" => ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "flipFaces" => ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "merge" => ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "dissolve" => ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "snap" => ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "toggleSmooth" => ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "unwrapActive" => ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "markUvSeam" => ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "clearSeam" => ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "engagementSubmit" => ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "translateSelection" => ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "rotateSelection" => ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "scaleSelection" => ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "paintStroke" => ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "paintAt" => ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "canvasPointerDown" => ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "canvasPointerMove" => ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "canvasPointerUp" => ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "paintFill" => ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "fillBucket" => ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "addPrimitive" => ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
        }
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(LowpolyCommandJobFactory::new(&controller))
    }

    fn build_tool_job(request: ArtifactOwnedToolJobRequest<EditorApp<Self>>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
        let Some(disposition) = lowpoly_command_disposition(&request.tool_id) else {
            return Ok(None);
        };
        if request.command.command_id() != request.tool_id {
            return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("app.command.tool-mismatch"), "lowpoly retained command does not match its exact registered tool"));
        }
        if !lowpoly_command_admitted(&request.command, &request.snapshot, &request.config) {
            return Err(Fault::from("lowpoly-retained-command-capacity"));
        }
        let tool_id = request.command.command_id();
        let work: Box<dyn ArtifactCommandWork<EditorApp<Self>>> =
            Box::new(LowpolyRetainedCommandWork::new(tool_id, disposition, request.operation.operation.0, request.operation.generation.0, request.canonical_base_revision, request.context.identity_digest()));
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
            LowpolyCommand::command_id,
            LOWPOLY_RETAINED_RAW_BYTES,
            LOWPOLY_RETAINED_WORK_ITEMS,
            work,
        );
        Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }

    fn app_schema() -> Option<::semio_framework_schema_registry::AppSchemaDescriptor> {
        Some(semio_s_artifact_lowpoly_lowpoly_config::schema::app_schema_descriptor())
    }

    fn initial_snapshot() -> LowpolySnapshot {
        semio_s_artifact_lowpoly_lowpoly_model::standards::v1::subsets::any::io::text::snapshot::default_snapshot()
    }

    fn io() -> Option<semio_framework_plugin::AppIo> {
        Some(lowpoly_io())
    }

    /// 🧬️ `import_media`'s `"mesh:in"`/`"artifact:in"` arms build `reset_document_effect` (a `Effect::LoadDocument`, outside
    /// undo history); whole-document replace has no mutation.
    ///
    /// 🎞️ `mesh:out` plus the inherited `document:out` default (the pack of `doc.snapshot`, replicated
    /// inline — overriding `export_media` shadows the trait's provided body for every port on this app,
    /// not just the new one).
    fn export_media(port: &str, doc: &ArtifactView<'_, LowpolySnapshot>) -> Result<Media, MediaError> {
        lowpoly_export_media(port, doc, &LowpolyScratch::default())
    }

    fn export_media_with_request_context(_owner: &semio_framework_plugin::ArtifactInstanceOperationOwnerHandle, port: &str, doc: &ArtifactView<'_, LowpolySnapshot>, transient: &semio_framework_plugin::TransientView<'_, LowpolyTransient>) -> Result<Media, MediaError> {
        let scratch = LowpolyScratch::from_transient(transient.snapshot, semio_s_artifact_lowpoly_lowpoly_model::snapshot::LowpolySelection::default());
        lowpoly_export_media(port, doc, &scratch)
    }

    /// 🎞️ `mesh:in` round-trips a `mesh.document` payload into a `reset_document_effect`; `document:in`
    /// replicates the trait's default whole-pack import inline (overriding `import_media` shadows the
    /// default for every port on this app, not just the new one).
    fn import_media(port: &str, media: &Media, _doc: &ArtifactView<'_, LowpolySnapshot>) -> Result<Emit<LowpolyMutation, LowpolyConfigMutation, Self::DraftMutation>, MediaError> {
        match port {
            "mesh:in" => {
                let MediaPayload::Structured { json, .. } = &media.payload else {
                    return Err(MediaError::Payload(port.into(), "mesh:in importer only accepts a Structured payload".into()));
                };
                let mesh_document = semio_framework_pack_json::from_json_str::<semio_framework_value::DslValue>(json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| MediaError::Payload(port.into(), error.to_string()))?;
                let mesh = semio_s_artifact_lowpoly_lowpoly_model::mesh_from_document_value(&mesh_document).map_err(|error| MediaError::Payload(port.into(), error))?;
                let snapshot = semio_s_artifact_lowpoly_lowpoly_model::lowpoly_snapshot_from_mesh(&mesh).map_err(|error| MediaError::Payload(port.into(), error))?;
                Ok(Emit { effects: vec![reset_document_effect(&snapshot)], ..Default::default() })
            }
            "artifact:in" => {
                let MediaPayload::Structured { json, .. } = &media.payload else {
                    return Err(MediaError::Payload(port.into(), "default document:in importer only accepts a Structured (base64 pack) payload".into()));
                };
                let bytes = store::pack_rt::pack_value_from_base64(json).map_err(|error| MediaError::Payload(port.into(), error.to_string()))?;
                let projection = <LowpolySnapshot as ArtifactPack>::decode_pack(&bytes).map_err(|error| MediaError::Payload(port.into(), error.to_string()))?;
                Ok(Emit { effects: vec![reset_document_effect(&projection)], ..Default::default() })
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    fn command_id(command: &LowpolyCommand) -> &'static str {
        command.command_id()
    }

    fn command_from_action(action: &str, args: &mut Option<semio_framework_value::DslValue>, original_fault: &mut Option<Fault>) -> Result<LowpolyCommand, Fault> {
        if let Some(fault) = original_fault.take() { return Err(fault); }
        let args = args.as_ref();
        args_bridge::command_from_action(action, args)
    }

    /// 🕹️ `interaction` is the mesh domain's current selection/hover/mode/granularity, resolved once per
    /// dispatch into `LowpolyScratch::current_selection` — the `app_commands!`-generated `dispatch` calls
    /// every leaf `🎮️commands/*::handle(payload, doc, cfg, ctx)` uniformly (no `interaction` parameter
    /// of its own), so this is the one seam by which those handlers (via `view::build_doc`/
    /// `session::mesh_edit`) see the framework-owned selection. See `🧭️view/🦀️.rs`'s
    /// `🔖️MeshDomain` region for the id scheme.
    fn handle(
        command: &LowpolyCommand,
        doc: &ArtifactView<'_, LowpolySnapshot>,
        cfg: &ConfigView<'_, LowpolyConfig>,
        interaction: &InteractionView<'_>,
        _view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<LowpolyMutation, LowpolyConfigMutation, Self::DraftMutation>, Fault> {
        let domain_selection = interaction.selection(MESH_INTERACTION_DOMAIN);
        let active = semio_s_artifact_lowpoly_lowpoly_view::active_object_for_selection(doc.snapshot, cfg.snapshot, domain_selection);
        let selection = selection_from_interaction(&active, interaction);
        let mut scratch = LowpolyScratch::from_transient(&LowpolyTransient::default(), selection);
        scratch.set_selection_object_id(semio_s_artifact_lowpoly_lowpoly_view::selection_object_id(doc.snapshot, domain_selection));
        command.dispatch(doc, cfg, &mut scratch)
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, LowpolySnapshot>, cfg: &ConfigView<'_, LowpolyConfig>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        lowpoly_render(body_key, doc, cfg, view_state, &mut LowpolyScratch::default(), None, None)
    }

    fn render_with_request_context(
        _owner: &semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
        body_key: &str,
        doc: &ArtifactView<'_, LowpolySnapshot>,
        cfg: &ConfigView<'_, LowpolyConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        transient: &semio_framework_plugin::TransientView<'_, LowpolyTransient>,
        interaction: &InteractionView<'_>,
        cx:&mut semio_framework_plugin::StepContext<'_>,
    ) -> semio_framework_plugin::OriginalRenderResult<semio_framework_plugin::ComponentTree> {
        let rendered:semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree>=(||{
        let mut scratch = LowpolyScratch::from_transient(transient.snapshot, semio_s_artifact_lowpoly_lowpoly_model::snapshot::LowpolySelection::default());
        let preview = transient.snapshot.paint_preview(doc.snapshot);
        lowpoly_render(body_key, doc, cfg, view_state, &mut scratch, Some(interaction), preview.as_ref())

        })();
        rendered.map(Some).map_err(Into::into)
        }

    fn window_engagements(doc: &ArtifactView<'_, LowpolySnapshot>, cfg: &ConfigView<'_, LowpolyConfig>, view_state: &semio_framework_plugin::ViewModel) -> HashMap<String, WindowEngagement> {
        let config = cfg.snapshot;
        let active_utility = view_state.active_utility_id.as_deref().filter(|utility| !utility.is_empty()).unwrap_or("move");
        let labels = semio_s_artifact_lowpoly_lowpoly_view::editor::lowpoly::terminology::lowpoly_play_labels(view_state);
        let engagement = lowpoly_window_engagement(LowpolyView { snapshot: doc.snapshot, config }, active_utility, labels);
        HashMap::from([(edit::windows::model::LOWPOLY_PLAY_WINDOW_MAIN.into(), engagement.clone()), (paint_mode::windows::uv::LOWPOLY_PLAY_WINDOW_UV.into(), engagement)])
    }

    /// 🎯️ The live engagement: granularity switches pressed to the mesh domain's state, the selected
    /// component count in the status line.
    fn window_engagements_with_request_context(doc: &ArtifactView<'_, LowpolySnapshot>, cfg: &ConfigView<'_, LowpolyConfig>, view_state: &semio_framework_plugin::ViewModel, _transient: &semio_framework_plugin::TransientView<'_, LowpolyTransient>, interaction: &InteractionView<'_>) -> HashMap<String, WindowEngagement> {
        let config = cfg.snapshot;
        let active_utility = view_state.active_utility_id.as_deref().filter(|utility| !utility.is_empty()).unwrap_or("move");
        let labels = semio_s_artifact_lowpoly_lowpoly_view::editor::lowpoly::terminology::lowpoly_play_labels(view_state);
        let select = semio_s_artifact_lowpoly_lowpoly_options::select::SelectState::from_interaction(interaction);
        let world = semio_s_artifact_lowpoly_lowpoly_view::world_selection_from_state(doc.snapshot, config, interaction.selection(MESH_INTERACTION_DOMAIN), interaction.active_granularity(MESH_INTERACTION_DOMAIN));
        let selected = if world.granularity == MESH_GRANULARITY_OBJECT { world.object_ids.len() } else { world.component_ids.len() };
        let engagement = lowpoly_window_engagement_with_selection(LowpolyView { snapshot: doc.snapshot, config }, active_utility, labels, &select, selected);
        HashMap::from([(edit::windows::model::LOWPOLY_PLAY_WINDOW_MAIN.into(), engagement.clone()), (paint_mode::windows::uv::LOWPOLY_PLAY_WINDOW_UV.into(), engagement)])
    }

    fn window_measures(_doc: &ArtifactView<'_, LowpolySnapshot>, cfg: &ConfigView<'_, LowpolyConfig>, view_state: &semio_framework_plugin::ViewModel) -> HashMap<String, Vec<WindowMeasure>> {
        let config = cfg.snapshot;
        let labels = semio_s_artifact_lowpoly_lowpoly_view::editor::lowpoly::terminology::lowpoly_play_labels(view_state);
        let measures = lowpoly_window_measures(config, labels, &semio_s_artifact_lowpoly_lowpoly_options::select::SelectState::default());
        HashMap::from([(edit::windows::model::LOWPOLY_PLAY_WINDOW_MAIN.into(), measures.clone()), (paint_mode::windows::uv::LOWPOLY_PLAY_WINDOW_UV.into(), measures)])
    }

    /// 🎛️ The live measures: the granularity/selection-mode toggles read the mesh domain's current
    /// state, so the pressed toggle IS the one the next pick uses (they used to be always-off).
    fn window_measures_with_request_context(_doc: &ArtifactView<'_, LowpolySnapshot>, cfg: &ConfigView<'_, LowpolyConfig>, view_state: &semio_framework_plugin::ViewModel, interaction: &InteractionView<'_>) -> HashMap<String, Vec<WindowMeasure>> {
        let config = cfg.snapshot;
        let labels = semio_s_artifact_lowpoly_lowpoly_view::editor::lowpoly::terminology::lowpoly_play_labels(view_state);
        let select = semio_s_artifact_lowpoly_lowpoly_options::select::SelectState::from_interaction(interaction);
        let measures = lowpoly_window_measures(config, labels, &select);
        HashMap::from([(edit::windows::model::LOWPOLY_PLAY_WINDOW_MAIN.into(), measures.clone()), (paint_mode::windows::uv::LOWPOLY_PLAY_WINDOW_UV.into(), measures)])
    }
}
//#endregion 🔖️LowpolyPlayApp
