//! 🕹️ Play app of the shooting editor: the `ShootingPlayApp` and its `ArtifactEditor` implementation. See [`super`].

use super::*;
use semio_framework_2d_compute::EngineHandles;

//#region 🔖️ShootingPlayApp
/// 🧪️ B1: unit struct — every former runtime field now lives in `ShootingConfig`, written through
/// `ShootingConfigMutation`s.
#[derive(Default)]
pub struct ShootingPlayApp;

impl ArtifactEditor for ShootingPlayApp {
    /// 📚️ Artifact catalogue stamped by `PluginBuilder::editor` onto the navbar dropdown.
    fn examples() -> Vec<semio_framework_plugin::ExampleSource> {
        vec![
        crate::examples::demo::source(),
        crate::examples::hexagonal_cut_concrete_forest_left::source(),
    ]
    }
    type Snapshot = ShootingSnapshot;
    type Mutation = ShootingMutation;
    type Config = ShootingConfig;
    type ConfigMutation = ShootingConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = ShootingPresence;
    type PresenceMutation = ShootingPresenceMutation;
    type Transient = semio_framework_plugin::NoTransient;
    type TransientMutation = semio_framework_plugin::NoTransientMutation;

    type Command = ShootingCommand;

    const DIALECT: Dialect = crate::SHOOTING_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = SHOOTING_DOCUMENT_SCHEMA;

    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: EditorApp<ShootingPlayApp>,
        owner_file: "✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.shooting.shooting@1/*#editor",
        artifact_schema: "shooting.shooting",
        factory: "ShootingCommandJobFactory",
        factory_type: ShootingCommandJobFactory,
        tools: {
            "importSnapshotJson" => ToolExecutionContract::bounded_first_step(65_536, 64, 1, 262_144, 7_500),
            "setActiveExample" => ToolExecutionContract::bounded_first_step(65_536, 64, 1, 262_144, 7_500),
            "setActiveShot" => ToolExecutionContract::bounded_first_step(65_536, 64, 1, 262_144, 7_500),
            "setActiveAsset" => ToolExecutionContract::bounded_first_step(65_536, 64, 1, 262_144, 7_500),
            "setShotCamera" => ToolExecutionContract::bounded_first_step(65_536, 64, 1, 262_144, 7_500),
            "saveCamera" => ToolExecutionContract::bounded_first_step(65_536, 64, 1, 262_144, 7_500),
            "setSunAzimuth" => ToolExecutionContract::bounded_first_step(65_536, 64, 1, 262_144, 7_500),
            "setSunElevation" => ToolExecutionContract::bounded_first_step(65_536, 64, 1, 262_144, 7_500),
            "setSunIntensity" => ToolExecutionContract::bounded_first_step(65_536, 64, 1, 262_144, 7_500),
            "setAmbientIntensity" => ToolExecutionContract::bounded_first_step(65_536, 64, 1, 262_144, 7_500),
            "setMaterialRoughness" => ToolExecutionContract::bounded_first_step(65_536, 64, 1, 262_144, 7_500),
            "setShadowEnabled" => ToolExecutionContract::bounded_first_step(65_536, 64, 1, 262_144, 7_500),
            "toggleSun" => ToolExecutionContract::bounded_first_step(65_536, 64, 1, 262_144, 7_500),
            "setActiveShotLabel" => ToolExecutionContract::bounded_first_step(65_536, 64, 1, 262_144, 7_500),
            "setActiveShotFormat" => ToolExecutionContract::bounded_first_step(65_536, 64, 1, 262_144, 7_500),
            "setActiveShotShape" => ToolExecutionContract::bounded_first_step(65_536, 64, 1, 262_144, 7_500),
            "patchShots" => ToolExecutionContract::bounded_first_step(65_536, 64, 1, 262_144, 7_500),
            "patchAssets" => ToolExecutionContract::bounded_first_step(65_536, 64, 1, 262_144, 7_500),
            "addShot" => ToolExecutionContract::bounded_first_step(65_536, 64, 1, 262_144, 7_500),
            "addAsset" => ToolExecutionContract::bounded_first_step(65_536, 64, 1, 262_144, 7_500),
            "importAsset" => ToolExecutionContract::bounded_first_step(65_536, 64, 1, 262_144, 7_500),
            "resetSnapshot" => ToolExecutionContract::bounded_first_step(65_536, 64, 1, 262_144, 7_500),
            "translateSelection" => ToolExecutionContract::bounded_first_step(65_536, 64, 1, 262_144, 7_500),
            "rotateSelection" => ToolExecutionContract::bounded_first_step(65_536, 64, 1, 262_144, 7_500),
            "scaleSelection" => ToolExecutionContract::bounded_first_step(65_536, 64, 1, 262_144, 7_500),
            "setCamera" => ToolExecutionContract::bounded_first_step(65_536, 64, 1, 262_144, 7_500),
            "loadSavedCamera" => ToolExecutionContract::bounded_first_step(65_536, 64, 1, 262_144, 7_500),
            "setCenterModel" => ToolExecutionContract::bounded_first_step(65_536, 64, 1, 262_144, 7_500),
            "setShotSelection" => ToolExecutionContract::bounded_first_step(65_536, 64, 1, 262_144, 7_500),
            "worldPointerDown" => ToolExecutionContract::bounded_first_step(65_536, 64, 1, 262_144, 7_500),
            "worldPointerMove" => ToolExecutionContract::bounded_first_step(65_536, 64, 1, 262_144, 7_500),
            "saveDownload" => ToolExecutionContract::bounded_first_step(65_536, 64, 1, 262_144, 7_500),
            "importAssetRequest" => ToolExecutionContract::bounded_first_step(65_536, 64, 1, 262_144, 7_500),
            "exportActiveShot" => ToolExecutionContract::bounded_first_step(65_536, 64, 1, 262_144, 7_500),
            "exportAllShots" => ToolExecutionContract::bounded_first_step(65_536, 64, 1, 262_144, 7_500),
        }
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(ShootingCommandJobFactory::new(&controller))
    }

    fn build_tool_job(request: &mut Option<ArtifactOwnedToolJobRequest<EditorApp<Self>>>, output: &mut Option<semio_framework_plugin::ErasedToolJob>, cx: &mut semio_framework_plugin::__job::StepContext<'_>) -> Result<bool, semio_framework_plugin::__job::OriginalTurnFailure> {
        let original = request.as_ref().ok_or_else(|| semio_framework_plugin::__value::ValueError::literal(semio_framework_plugin::__value::ValueRefusalKind::InvariantViolated, "original retained producer request is absent"))?;
        if !SHOOTING_BOUNDED_TOOL_IDS.contains(&original.tool_id.as_str()) {
            return Ok(false);
        }
        if original.command.command_id() != original.tool_id {
            return Err(semio_framework_plugin::__value::ValueError::literal(semio_framework_plugin::__value::ValueRefusalKind::InvariantViolated, "Shooting command does not match its exact registered tool").into());
        }
        if shooting_bounded_extent(&original.command, &original.snapshot, &original.interaction_state).is_none() {
            return Err(semio_framework_plugin::__value::ValueError::literal(semio_framework_plugin::__value::ValueRefusalKind::InvariantViolated, "Shooting bounded route exceeded its declared work extent").into());
        }
        let tool_id = original.command.command_id();
        semio_framework_plugin::retained_command::producer::admit_retained_tool_producer(request, output, cx, ShootingCommand::command_id, SHOOTING_BOUNDED_RAW_BYTES, SHOOTING_BOUNDED_WORK_ITEMS, false, |request| BoundedArtifactCommandWork::new(tool_id, shooting_bounded_reduce, shooting_bounded_extent))
    }

    fn app_schema() -> Option<::semio_framework_schema_registry::AppSchemaDescriptor> {
        Some(crate::editor::shooting::config::schema::app_schema_descriptor())
    }

    fn reference_birth()->Option<fn()->Result<ShootingSnapshot,semio_framework_plugin::Fault>>{Some(||{
        Ok(crate::standards::v1::subsets::any::io::text::snapshot::default_snapshot())
    })}

    fn io() -> Option<AppIo> {
        Some(shooting_io())
    }

    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(store::cold_mutation_apply_preparation_factory::<Self::Snapshot, Self::Mutation>())
    }

    fn build_config_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Config, Self::ConfigMutation>>> {
        Some(store::snapshot_clone_preparation::config_apply_preparation_factory::<Self::Config, Self::ConfigMutation>())
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

    fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::bounded_transient_root_retirement_factory::<Self::Presence>())
    }

    fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::bounded_transient_root_retirement_factory::<Self::Presence>())
    }

    /// 👥️ Shooting presence is a shot-id list plus one camera, so the default root is its exact empty terminal.
    fn build_presence_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::PresenceStore<Self::Presence, Self::PresenceMutation>>>> {
        Some(Box::new(semio_framework_plugin::PresenceStoreOwnedDisposer::new(std::sync::Arc::new(Self::Presence::default()), |_| true).expect("default shooting presence is the exact empty terminal")))
    }

    fn build_transient_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::TransientStore<Self::Transient, Self::TransientMutation>>>> {
        Some(semio_framework_plugin::no_transient_store_disposer())
    }

    fn build_transient_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Transient>>> {
        Some(semio_framework_plugin::no_transient_local_root_retirement_factory())
    }

    /// 🎞️ `photos:out` (see `shooting_photo_media`) plus the
    /// inherited `document:out` default (the pack of `doc.snapshot`, replicated inline — overriding
    /// `export_media` shadows the trait's provided body for every port on this app, not just the new
    /// one).
    fn export_media(port: &str, doc: &ArtifactView<'_, ShootingSnapshot>) -> Result<Media, MediaError> {
        match port {
            "photos:out" => shooting_photo_media(doc.snapshot),
            "artifact:out" => {
                let media_type = Self::io().map_or(MediaType { class: MediaClass::Data, form: MediaForm::Value }, |io| io.artifact_media_type);
                let bytes = pack::snapshot::ArtifactPack::encode_pack(doc.snapshot);
                Ok(Media { media_type, payload: MediaPayload::Structured { schema: Self::DOCUMENT_SCHEMA.to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) } })
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    /// 🧬️ `"artifact:in"` is handled below as a `reset_document_effect` (a `Effect::LoadDocument`, outside undo history);
    /// whole-document replace has no mutation.
    fn import_media(port: &str, media: &Media, _doc: &ArtifactView<'_, ShootingSnapshot>) -> Result<Emit<ShootingMutation, ShootingConfigMutation, NoDraftMutation>, MediaError> {
        if port != "artifact:in" {
            return Err(MediaError::NotImplemented);
        }
        let MediaPayload::Structured { json, .. } = &media.payload else {
            return Err(MediaError::Payload(port.to_string(), "default document:in importer only accepts a Structured (base64 pack) payload".into()));
        };
        let bytes = store::pack_rt::pack_value_from_base64(json).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
        let snapshot = <ShootingSnapshot as pack::snapshot::ArtifactPack>::decode_pack(&bytes).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
        Ok(Emit { effects: vec![reset_document_effect(&snapshot)], ..Default::default() })
    }

    /// 🏷️ Maps each `ShootingCommand` variant back to the action id it was declared under in
    /// `create_shooting_app` — used by `VcsArtifactApp` for command-log labeling and the registry's
    /// View/Shell kind-discipline check.
    fn command_id(command: &ShootingCommand) -> &'static str {
        command.command_id()
    }

    /// 🕹️ The flat `assets` domain: every asset of the document at `asset` granularity, in document order — without
    /// it the framework's revalidation drops every pick, and an id-less gumball gesture finds no selection to move.
    fn interaction_topology(doc: &ArtifactView<'_, ShootingSnapshot>, _cfg: &ConfigView<'_, ShootingConfig>) -> Result<semio_framework_plugin::InteractionTopology, semio_framework_value::ValueError> {
 Ok((||{
        let ordered = doc.snapshot.assets.iter().map(|asset| semio_framework_plugin::TopologyNode { id: asset.id.clone(), granularity: "asset".into(), parent: None }).collect();
        semio_framework_plugin::InteractionTopology { domains: std::collections::BTreeMap::from([(SHOOTING_INTERACTION_DOMAIN.into(), semio_framework_plugin::DomainTopology { ordered })]) }
    
})())
}

    fn command_from_action(action: &str, args: &mut Option<DslValue>, original_fault: &mut Option<Fault>) -> Result<Self::Command, Fault> {
        if let Some(fault) = original_fault.take() { return Err(fault); }
        let args = args.as_ref();
        args_bridge::command_from_action(action, args)
    }

    fn handle(
        command: &ShootingCommand,
        doc: &ArtifactView<'_, ShootingSnapshot>,
        cfg: &ConfigView<'_, ShootingConfig>,
        interaction: &InteractionView<'_>,
        _view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<ShootingMutation, ShootingConfigMutation, Self::DraftMutation>, Fault> {
        let mut ctx = ShootingDispatchCtx { selected_asset_ids: interaction.selection(SHOOTING_INTERACTION_DOMAIN).ids.clone() };
        command.dispatch(doc, cfg, &mut ctx)
    }

    /// 🧮️ This app's typed configuration spec — mirrors `ShootingConfig`'s three sticky-default fields,
    /// each grounded in an existing `.action_args` default (see that struct's doc).
    fn config_spec() -> semio_framework_plugin::ConfigSpec {
        semio_framework_plugin::ConfigSpec {
            fields: vec![
                ActionArgDef::select(
                    "defaultShotFormat",
                    LocalizedLabel::native("Default Shot Format", "Standard-Aufnahmeformat"),
                    vec![ActionArgOption::new("svg", LocalizedLabel::native("SVG", "SVG")), ActionArgOption::new("png", LocalizedLabel::native("PNG", "PNG"))],
                )
                .default_value(&"png"),
                ActionArgDef::select(
                    "defaultShotShape",
                    LocalizedLabel::native("Default Shot Shape", "Standard-Aufnahmeform"),
                    vec![ActionArgOption::new("rectangle", LocalizedLabel::native("Rectangle", "Rechteck")), ActionArgOption::new("ellipse", LocalizedLabel::native("Ellipse", "Ellipse"))],
                )
                .default_value(&"rectangle"),
                ActionArgDef::select("defaultAssetFormat", LocalizedLabel::native("Default Asset Format", "Standard-Asset-Format"), vec![ActionArgOption::new("glb", LocalizedLabel::native("GLB", "GLB"))]).default_value(&"glb"),
            ],
        }
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, ShootingSnapshot>, cfg: &ConfigView<'_, ShootingConfig>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let snapshot = doc.snapshot;
        let labels = shooting_play_labels(view_state);
        match body_key {
            SHOOTING_PLAY_BODY_SCENE => scene_window::render(snapshot, cfg.snapshot, view_state.active_utility_id.as_deref().unwrap_or("move")),
            SHOOTING_PLAY_BODY_ICON => icon_window::render(snapshot, cfg.snapshot),
            SHOOTING_PLAY_BODY_ARTIFACT => document_panel::render(snapshot, labels, &semio_framework_plugin::TreeWindows::for_body(view_state, SHOOTING_PLAY_BODY_ARTIFACT)),
            SHOOTING_PLAY_BODY_CATALOGUE => catalogue_panel::render(labels, &semio_framework_plugin::TreeWindows::for_body(view_state, SHOOTING_PLAY_BODY_CATALOGUE)),
            SHOOTING_PLAY_BODY_INSPECTION => inspection_panel::render(snapshot, cfg.snapshot, labels),
            _ => semio_framework_plugin::built_text_node(Label::data(format!("Unknown body: {body_key}"))).map_err(|_| ui_capacity_error()),
        }
        .map(semio_framework_plugin::built_to_component_tree)
    }

    fn window_engagements(doc: &ArtifactView<'_, ShootingSnapshot>, _cfg: &ConfigView<'_, ShootingConfig>, view_state: &semio_framework_plugin::ViewModel) -> HashMap<String, WindowEngagement> {
        let labels = shooting_play_labels(view_state);
        HashMap::from([(SHOOTING_PLAY_WINDOW_SCENE.into(), scene_window::engagement(doc.snapshot, labels)), (SHOOTING_PLAY_WINDOW_ICON.into(), icon_window::engagement(doc.snapshot, labels))])
    }

    fn window_measures(doc: &ArtifactView<'_, ShootingSnapshot>, _cfg: &ConfigView<'_, ShootingConfig>, view_state: &semio_framework_plugin::ViewModel) -> HashMap<String, Vec<WindowMeasure>> {
        let labels = shooting_play_labels(view_state);
        HashMap::from([(SHOOTING_PLAY_WINDOW_SCENE.into(), scene_window::window_measures(doc.snapshot, labels)), (SHOOTING_PLAY_WINDOW_ICON.into(), icon_window::window_measures(doc.snapshot, labels))])
    }
}
//#endregion 🔖️ShootingPlayApp
