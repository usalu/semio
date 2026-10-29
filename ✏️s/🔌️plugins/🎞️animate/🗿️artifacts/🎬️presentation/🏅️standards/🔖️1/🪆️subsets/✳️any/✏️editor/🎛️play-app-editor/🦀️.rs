//! 🎛️ Play app editor: the animate presentation play app and its artifact editor implementation. See [`super`].

use super::*;

//#region 🔖️AnimatePresentationPlayApp
/// 🧪️ B1: unit struct — every former `AnimatePresentationPlayRuntime` field now lives in
/// `crate::editor::animate::config::PresentationConfig` (see `ArtifactApp::Config`), written through
/// `PresentationConfigMutation`s.
#[derive(Default)]
pub struct AnimatePresentationPlayApp;

impl ArtifactEditor for AnimatePresentationPlayApp {
    /// 🧩️ Composes `s.stdio.semio@v1/*` children, so every bundle of this surface opens them through the same roster.
    type Members = semio_s_artifact_stdio_semio::SemioMembers;
    /// 🧬️ The loaded-parent child projection, read straight off the snapshot's own `#[child]` fields.
    /// Without it every live envelope load faults with `editor did not declare a loaded-parent child
    /// projection` before the decoded document can replace the store.
    fn child_restore_projection(snapshot: &Self::Snapshot) -> Result<semio_framework_composition_admission::ChildRestoreProjection<'_>, Fault> {
        semio_framework_composition_admission::ChildRestoreProjection::from_snapshot(snapshot).map_err(|error| Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("animate.child-projection"), error.to_string()))
    }

    type Snapshot = PresentationSnapshot;
    type Mutation = PresentationMutation;
    type Config = PresentationConfig;
    type ConfigMutation = PresentationConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = crate::editor::animate::presence::PresentationPresence;
    type PresenceMutation = crate::editor::animate::presence::PresentationPresenceMutation;
    type Transient = semio_framework_plugin::NoTransient;
    type TransientMutation = semio_framework_plugin::NoTransientMutation;

    type Command = PresentationCommand;

    const DIALECT: Dialect = crate::ANIMATE_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = PRESENTATION_DOCUMENT_SCHEMA;

    fn genesis_child_pack(snapshot: &Self::Snapshot, slot: &str, child_id: &str) -> Result<Option<Vec<u8>>,semio_framework_value::ValueError> {
 Ok((||{
        crate::genesis_presentation_child_pack(snapshot, slot, child_id)
    
})())
}

    /// 🧺️ Without these owners the document store holds no `initial_snapshot_retirement_factory`, so
    /// the boot `setActiveExample` archive load is refused with `module.vcs: validation failed:
    /// returned snapshot read requires its exact owned-snapshot retirement factory` — measured
    /// against the live React playground.

    fn build_document_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
        Some(semio_framework_plugin::bounded_document_store_disposer::<Self::Snapshot, Self::Mutation>())
    }

    /// 📬️ Exact one-item PUBLICATION authority for the DOCUMENT store. Without it every tool
    /// declaring the `artifact` lane is marked `unsupported-publication-contract` at boot and fails
    /// closed with `interactive-job.publication-authority-missing`, which is why this app could own
    /// no retained document verb at all (B1a: "promoting a document verb is a two-part change").
    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(store::cold_mutation_apply_preparation_factory::<Self::Snapshot, Self::Mutation>())
    }

    fn build_config_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Config, Self::ConfigMutation>>> {
        Some(std::sync::Arc::new(AnimatePresentationConfigPreparationFactory))
    }

    /// 🧹️ Every store an instance owns closes through its bounded disposer before drop; without these the
    /// instance close faults `interactive-job.close-owned-disposer-missing` at the config-store lane.

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

    fn build_presence_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::PresenceStore<Self::Presence, Self::PresenceMutation>>>> {
        Some(Box::new(
            semio_framework_plugin::PresenceStoreOwnedDisposer::new(std::sync::Arc::new(Self::Presence::default()), |value| value == &Self::Presence::default())
                .expect("default presentation presence is the exact empty terminal"),
        ))
    }

    fn build_transient_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Transient>>> {
        Some(semio_framework_plugin::no_transient_local_root_retirement_factory())
    }

    fn build_transient_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::TransientStore<Self::Transient, Self::TransientMutation>>>> {
        Some(semio_framework_plugin::no_transient_store_disposer())
    }

    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: EditorApp<AnimatePresentationPlayApp>,
        owner_file: "✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.animate.presentation@1/*#editor",
        artifact_schema: "animate.presentation",
        factory: "AnimatePresentationRetainedCommandJobFactory",
        factory_type: AnimatePresentationRetainedCommandJobFactory,
        contract: animate_presentation_retained_contract(),
        tools: [
            "seedGrid",
            "addTile",
            "deleteTile",
            "deleteSelection",
            "renameTiles",
            "patchTileCrops",
            "setSource",
            "setFrame",
            "setActiveExample",
            "clearTiles",
            "engagementSubmit",
            "resetGrid",
            "engagementInput",
            "canvasPointerDown",
            "noMutation",
            "copyPrompt",
            "exportVideoFromDeck"
        ]
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(AnimatePresentationRetainedCommandJobFactory::new(&controller))
    }

    fn build_tool_job(request: &mut Option<ArtifactOwnedToolJobRequest<EditorApp<Self>>>, output: &mut Option<semio_framework_plugin::ErasedToolJob>, cx: &mut semio_framework_plugin::__job::StepContext<'_>) -> Result<bool, semio_framework_plugin::__job::OriginalTurnFailure> {
        let original = request.as_ref().ok_or_else(|| semio_framework_plugin::__value::ValueError::literal(semio_framework_plugin::__value::ValueRefusalKind::InvariantViolated, "original retained producer request is absent"))?;
        if !ANIMATE_PRESENTATION_RETAINED_TOOL_IDS.contains(&original.tool_id.as_str()) {
            return Ok(false);
        }
        if original.command.command_id() != original.tool_id {
            return Err(semio_framework_plugin::__value::ValueError::literal(semio_framework_plugin::__value::ValueRefusalKind::InvariantViolated, "animate-presentation-command-tool-mismatch").into());
        }
        if animate_presentation_retained_extent(&original.command, &original.snapshot, &original.interaction_state).is_none() {
            return Err(Fault::from("animate-presentation-command-payload-too-large"));
        }
        let tool_id = original.command.command_id();
        semio_framework_plugin::retained_command::producer::admit_retained_tool_producer(request, output, cx, PresentationCommand::command_id, ANIMATE_PRESENTATION_RETAINED_RAW_BYTES, ANIMATE_PRESENTATION_RETAINED_WORK_ITEMS, false, |request| semio_framework_plugin::retained_command::BoundedArtifactCommandWork::new(tool_id, animate_presentation_retained_reduce, animate_presentation_retained_extent))
    }

    /// 🎞️ `import-media` is the only reserved route presentation owns: every inbound `frames:in`
    /// delivery is routed through this builder (`dispatch_import_media` → `build_artifact_reserved_media_job`).
    fn build_reserved_tool_job(request: ArtifactReservedToolJobRequest<EditorApp<Self>>) -> Result<Option<ArtifactReservedToolJob>, Fault> {
        if request.tool_id.as_str() != PRESENTATION_IMPORT_TOOL_ID {
            return Ok(None);
        }
        if !request.raw_wire.is_empty() {
            return Err(Fault::from("presentation import-media admits a decoded media value, never a wire payload"));
        }
        let ArtifactReservedToolInput::Media { port, media } = &request.input else {
            return Err(Fault::from("presentation import-media requires media input"));
        };
        let (port, media) = (port.clone(), media.clone());
        Ok(Some(ArtifactReservedToolJob::new(PresentationImportJob::new(request, port, media))))
    }

    fn build_envelope_decode_owner_bundle() -> Option<store::os_store::owned_schema_decode::ArtifactEnvelopeDecodeOwnerBundle<Self::Snapshot, Self::Mutation>> {
        Some(crate::host::owned::presentation_envelope_decode_owner_bundle())
    }

    fn app_schema() -> Option<::semio_framework_schema_registry::AppSchemaDescriptor> {
        Some(crate::editor::animate::config::schema::app_schema_descriptor())
    }

    fn reference_birth()->Option<fn()->Result<PresentationSnapshot,semio_framework_plugin::Fault>>{Some(||{
        Ok(default_presentation_snapshot())
    })}

    fn io() -> Option<AppIo> {
        Some(presentation_io())
    }

    /// 🎞️ `frames:in` (Wave-2 port recipe): inserts an incoming raster frame as a new tile in a
    /// deterministic contact-sheet grid (see `next_frame_tile_crop`'s doc comment below for why this
    /// schema's single shared `source` means tiles, not `source`, are the natural insertion point).
    /// Never mutates anything directly: the caller applies the returned `Tiles(Add)` through the
    /// ordinary, undoable document store.
    fn import_media(port: &str, media: &Media, doc: &ArtifactView<'_, PresentationSnapshot>) -> Result<Emit<PresentationMutation, PresentationConfigMutation, Self::DraftMutation>, MediaError> {
        if port != "frames:in" {
            return Err(MediaError::NotImplemented);
        }
        let deck = doc.snapshot;
        let count = crate::presentation_working_scene(deck).1.len();
        let id = next_frame_tile_id(count);
        let crop = next_frame_tile_crop(count);
        let name = frame_media_name(port, media)?;
        let tile = FigureTileDraft { id, name, crop };
        Ok(Emit::mutations(vec![PresentationMutation::CreateTile(CreateTile { index: count, tile })]))
    }

    /// 🏷️ The manifest action id each command was declared under — supplied wholesale by
    /// `app_commands!`'s generated `command_id()`.
    fn command_id(command: &PresentationCommand) -> &'static str {
        command.command_id()
    }

    /// 🎯️ Maps a host action id + its staged args onto `PresentationCommand`. The React/wgpu shells
    /// still dispatch `{action, args}` while the guest channel is typed-only, and the trait default
    /// refuses EVERY id (`app.command.unsupported`) — without this bridge the boot
    /// `setActiveExample`, every Actions-pane row and every canvas pick died before reaching
    /// `handle`. The `#[dsl(block)]` payloads (`crop`/`frame`/`source`) decode through
    /// `semio_framework_value::FromValue::from_value`, the same codec the typed channel uses.
    fn command_from_action(action: &str, args: &mut Option<semio_framework_value::DslValue>, original_fault: &mut Option<Fault>) -> Result<PresentationCommand, Fault> {
        if let Some(fault) = original_fault.take() { return Err(fault); }
        let args = args.as_ref();
        let text_arg = |keys: &[&str]| keys.iter().find_map(|key| args.and_then(|value| value.get(key)).and_then(semio_framework_value::DslValue::as_str).map(str::to_string));
        let number_arg = |keys: &[&str]| keys.iter().find_map(|key| args.and_then(|value| value.get(key)).and_then(semio_framework_value::DslValue::as_f64)).unwrap_or_default();
        let id_list = |keys: &[&str]| match keys.iter().find_map(|key| args.and_then(|value| value.get(key))) {
            Some(semio_framework_value::DslValue::Array(items)) => items.iter().filter_map(|item| item.as_str().map(str::to_string)).collect(),
            Some(semio_framework_value::DslValue::String(raw)) if !raw.is_empty() => vec![raw.clone()],
            _ => Vec::new(),
        };
        fn decode<T: semio_framework_value::FromValue>(action: &str, args: Option<&semio_framework_value::DslValue>, key: &str) -> Result<T, Fault> {
            let value = args.and_then(|value| value.get(key)).cloned().ok_or_else(|| Fault::from(format!("presentation {action} requires a '{key}' block")))?;
            semio_framework_value::FromValue::from_value(value).map_err(|error| Fault::from(format!("invalid presentation {action} '{key}': {error}")))
        }
        match action {
            "seedGrid" => Ok(PresentationCommand::SeedGrid(seed_grid::SeedGrid { rows: number_arg(&["rows"]) as u32, columns: number_arg(&["columns", "cols"]) as u32 })),
            "addTile" => Ok(PresentationCommand::AddTile(add_tile::AddTile { crop: args.and_then(|value| value.get("crop")).cloned().map(semio_framework_value::FromValue::from_value).transpose().map_err(|error| Fault::from(format!("invalid presentation addTile 'crop': {error}")))? })),
            "deleteTile" => Ok(PresentationCommand::DeleteTile(delete_tile::DeleteTile { id: text_arg(&["id", "tileId", "value"]).unwrap_or_default() })),
            "deleteSelection" => Ok(PresentationCommand::DeleteSelection(delete_selection::DeleteSelection {})),
            "renameTiles" => Ok(PresentationCommand::RenameTiles(rename_tiles::RenameTiles { ids: id_list(&["ids", "id"]), value: text_arg(&["value", "name"]).unwrap_or_default() })),
            "patchTileCrops" => Ok(PresentationCommand::PatchTileCrops(patch_tile_crops::PatchTileCrops { ids: id_list(&["ids", "id"]), field: text_arg(&["field"]).unwrap_or_default(), value: number_arg(&["value"]) })),
            "setSource" => Ok(PresentationCommand::SetSource(set_source::SetSource { source: decode(action, args, "source")? })),
            "setFrame" => Ok(PresentationCommand::SetFrame(set_frame::SetFrame { frame: decode(action, args, "frame")? })),
            "setActiveExample" => Ok(PresentationCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: text_arg(&["exampleId", "example_id", "id", "value"]).unwrap_or_else(|| crate::examples::demo::ID.into()) })),
            "clearTiles" => Ok(PresentationCommand::ClearTiles(clear_tiles::ClearTiles {})),
            "engagementSubmit" => Ok(PresentationCommand::EngagementSubmit(engagement_submit::EngagementSubmit { value: text_arg(&["value", "text"]).unwrap_or_default() })),
            "resetGrid" => Ok(PresentationCommand::ResetGrid(reset_grid::ResetGrid {})),
            "engagementInput" => Ok(PresentationCommand::EngagementInput(engagement_input::EngagementInput { value: text_arg(&["value", "text"]).unwrap_or_default() })),
            "canvasPointerDown" => Ok(PresentationCommand::CanvasPointerDown(canvas_pointer_down::CanvasPointerDown { layer_id: text_arg(&["layerId", "layer_id", "id"]) })),
            "noMutation" => Ok(PresentationCommand::NoOperation(no_operation::NoOperation {})),
            "copyPrompt" => Ok(PresentationCommand::CopyPrompt(copy_prompt::CopyPrompt {})),
            "exportVideoFromDeck" => Ok(PresentationCommand::ExportVideoFromDeck(export_video_from_deck::ExportVideoFromDeck {
                scene_json: match args.and_then(|value| value.get("scene")) {
                    Some(semio_framework_value::DslValue::String(text)) => text.clone(),
                    Some(semio_framework_value::DslValue::Null) | None => String::new(),
                    Some(value) => semio_framework_pack_json::to_json_string(value),
                },
            })),
            other => Err(Fault::from(format!("presentation: unhandled action id {other}"))),
        }
    }

    fn handle(
        command: &PresentationCommand,
        doc: &ArtifactView<'_, PresentationSnapshot>,
        cfg: &ConfigView<'_, PresentationConfig>,
        interaction: &InteractionView<'_>,
        _view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<PresentationMutation, PresentationConfigMutation, Self::DraftMutation>, Fault> {
        let mut ctx = PresentationDispatchCtx { selected_ids: interaction.selection(PRESENTATION_INTERACTION_DOMAIN).ids.clone() };
        command.dispatch(doc, cfg, &mut ctx)
    }

    /// 🕹️ `render(body_key, doc, cfg)` is never given an `InteractionView` (ticket
    /// 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM — only `handle`/`copy_fragment`/
    /// `cut_operations` are), so the per-tile crop/name editors this panel used to build from
    /// `config.selected_ids` are gone from `inspection::render`; the client renders the tile-selected
    /// canvas highlight itself from the framework's own interaction state now (matches `🖍️draw`'s
    /// canvas render, same reason).
    fn render(body_key: &str, doc: &ArtifactView<'_, PresentationSnapshot>, _cfg: &ConfigView<'_, PresentationConfig>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<ComponentTree> {
        let deck = doc.snapshot;
        let labels = animate_presentation_labels(view_state);
        (match body_key {
            PRESENTATION_PLAY_BODY_MAIN => tile_editor::render(deck),
            PRESENTATION_PLAY_BODY_ARTIFACT => artifact::render(deck, labels, &semio_framework_plugin::TreeWindows::for_body(view_state, PRESENTATION_PLAY_BODY_ARTIFACT)),
            PRESENTATION_PLAY_BODY_CATALOGUE => catalogue::render(deck, labels),
            PRESENTATION_PLAY_BODY_DETAILS => inspection::render(deck, labels),
            _ => semio_framework_plugin::built_text_node(Label::data(format!("Unknown body: {body_key}"))).map_err(|_| ui_capacity_error()),
        })
        .map(semio_framework_plugin::built_to_component_tree)
    }
}
//#endregion 🔖️AnimatePresentationPlayApp
