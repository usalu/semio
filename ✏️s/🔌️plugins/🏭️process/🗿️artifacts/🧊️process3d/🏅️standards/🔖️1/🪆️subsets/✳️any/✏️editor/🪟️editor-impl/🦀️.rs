//! 🪟️ Process3d play app body rendering and the `ArtifactEditor` implementation.

use super::*;
use semio_framework_2d_compute::EngineHandles;
use semio_s_artifact_process_process3d_kit::Process3dDispatchCtx;
use semio_s_artifact_process_process3d_kit::Process3dInteractionSnapshot;

/// 🧱️ Every window/panel body of the process3d editor, rendered against one already-resolved
/// `"geometry"` selection — shared by `render` (empty selection) and `render_with_request_context`
/// (the live selection) so there is exactly one body-key match in the app.
pub(super) fn process3d_render_body(body_key: &str, doc: &Process3dSnapshot, config: &Process3dConfig, view_state: &semio_framework_plugin::ViewModel, selected_ids: &[String]) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
    let labels = process3d_labels(view_state);
    let active_utility = view_state.active_utility_id.as_deref().filter(|utility| !utility.is_empty()).unwrap_or(PROCESS3D_DEFAULT_UTILITY);
    let base_body_key = body_key.split_once(':').map_or(body_key, |(base, _)| base);
    // 🪟️ One `TreeWindows` per panel body: the host's open/scroll state for exactly the body being
    // rendered, read straight off `ViewModel.tree_windows` (ticket 26/09/16/ARTIFACT-TREE-VIRTUALISED-STREAMING).
    let windows = semio_framework_plugin::TreeWindows::for_body(view_state, base_body_key);
    match base_body_key {
        PROCESS_3D_PLAY_BODY_MAIN => workpiece::render(doc, config, active_utility).map(semio_framework_plugin::built_to_component_tree),
        PROCESS_3D_PLAY_BODY_ARTIFACT => document_panel::render(doc, config.resolved_up_to, labels, &windows).map(semio_framework_plugin::built_to_component_tree),
        PROCESS_3D_PLAY_BODY_CATALOGUE => catalogue::render(doc, &config.contributions_json, labels, &windows).map(semio_framework_plugin::built_to_component_tree),
        PROCESS_3D_PLAY_BODY_WORKSHOP => workshop_panel::render(doc, &config.contributions_json, labels, &windows).map(semio_framework_plugin::built_to_component_tree),
        PROCESS_3D_PLAY_BODY_INSPECTION => inspection::render(doc, selected_ids, labels, &windows).map(semio_framework_plugin::built_to_component_tree),
        _ => semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
    }
}

impl ArtifactEditor for Process3dPlayApp {
    /// 📢️ The localized notices of this editor's user-reachable refusals (design §20.12).
    fn fault_notices() -> &'static [(&'static str, LocalizedLabel)] {
        use semio_framework_ui_locale::LocalizedLabel;
        static NOTICES: std::sync::LazyLock<[(&str, LocalizedLabel); 2]> = std::sync::LazyLock::new(|| {
            [
            ("process3d.action.invalid", LocalizedLabel::native("This action's arguments are not valid.", "Die Argumente dieser Aktion sind ungültig.")),
            ("process3d.media.export", LocalizedLabel::native("The model cannot be exported.", "Das Modell lässt sich nicht exportieren.")),
            ]
        });
        &*NOTICES
    }

    /// 📚️ Artifact catalogue stamped by `PluginBuilder::editor` onto the navbar dropdown.
    fn examples() -> Vec<semio_framework_plugin::ExampleSource> {
        vec![semio_s_artifact_process_process3d_examples::source(), semio_s_artifact_process_process3d_examples::examples::concrete_forest::source()]
    }
    /// 🧩️ The roster the three composed `s.stdio.semio` members (`stockSolid`/`steps`/`toolSolids`)
    /// open through — a `NoMembers` editor cannot materialise the children `genesis_child_pack`
    /// derives, and every whole-document load then fails its archive closure as `Incomplete`.
    type Members = semio_s_artifact_stdio_semio::SemioMembers;
    type Snapshot = Process3dSnapshot;
    type Mutation = Process3dMutation;
    type Config = Process3dConfig;
    type ConfigMutation = Process3dConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = Process3dPresence;
    type PresenceMutation = Process3dPresenceMutation;
    type Transient = semio_framework_plugin::NoTransient;
    type TransientMutation = semio_framework_plugin::NoTransientMutation;

    type Command = Process3dCommand;

    const DIALECT: Dialect = semio_s_artifact_process_process3d_model::snapshot::PROCESS3D_DIALECT;

    const DOCUMENT_SCHEMA: &'static str = semio_s_artifact_process_process3d_model::snapshot::PROCESS_3D_SCHEMA;

    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(std::sync::Arc::new(Process3dArtifactPreparationFactory))
    }

    fn build_config_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Config, Self::ConfigMutation>>> {
        Some(std::sync::Arc::new(Process3dConfigStorePreparationFactory))
    }

    fn bounded_first_step_tool_proofs() -> Vec<semio_framework_plugin::ArtifactBoundedFirstStepProof> {
        Process3dBoundedProofs::bounded_first_step_tool_proofs().into_iter().chain(Process3dResumableProofs::bounded_first_step_tool_proofs()).collect()
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        let controller_id = registry.controller_id().to_string();
        registry.register(Process3dBoundedCommandJobFactory::new(&controller_id))?;
        registry.register(Process3dResumableCommandJobFactory::new(&controller_id))
    }

    fn build_tool_job(request: ArtifactOwnedToolJobRequest<EditorApp<Self>>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
        let Some(disposition) = process3d_command_disposition(&request.tool_id) else {
            return Ok(None);
        };
        if request.command.command_id() != request.tool_id {
            return Err(Fault::new(FaultOrigin::App, FaultCode::new("app.command.tool-mismatch"), "Process 3D command does not match its exact registered tool"));
        }
        let tool_id = request.command.command_id();
        let work: Box<dyn ArtifactCommandWork<EditorApp<Self>>> = match disposition {
            Process3dCommandDisposition::Bounded => Box::new(BoundedArtifactCommandWork::new(tool_id, process3d_retained_reduce, |_, _, _| Some(1))),
            Process3dCommandDisposition::Config => {
                let extent = process3d_resumable_extent(&request.command, &request.snapshot, &request.config, &request.interaction_state).ok_or_else(|| Fault::from("process3d-retained-work-extent-overflow"))?;
                Box::new(Process3dResumableCommandWork::new(tool_id, extent))
            }
        };
        let operation_context = AppOperationContext {
            app_instance_id: request.app_instance_id,
            parent_document_id: request.parent_document_id.clone(),
            operation_id: request.operation.operation.0,
            generation: request.operation.generation.0,
            canonical_base_revision: request.canonical_base_revision,
            authoring_seed: request.authoring_seed.clone(),
            retained: request.retained,
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
            Process3dCommand::command_id,
            process3d_retained_raw_bytes(tool_id),
            process3d_retained_work_items(tool_id),
            work,
        );
        Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }
    const REQUIRES_DOCUMENT_STORE_PUBLICATION_AUTHORITY: bool = true;

    fn build_envelope_decode_owner_bundle() -> Option<store::os_store::owned_schema_decode::ArtifactEnvelopeDecodeOwnerBundle<Self::Snapshot, Self::Mutation>> {
        Some(semio_s_artifact_process_process3d_host::process3d_envelope_decode_owner_bundle())
    }

    fn build_document_store_initialization_job(
        envelope: store::os_store::store_schema::ArtifactEnvelope<Self::Snapshot, Self::Mutation>,
        operation: semio_framework_job::OperationId,
        generation: semio_framework_job::Generation,
        actor: protocol::ActorId,
    ) -> Result<semio_framework_plugin::ArtifactStoreInitializationJob<Self::Snapshot, Self::Mutation>, store::os_store::store_schema::ArtifactEnvelope<Self::Snapshot, Self::Mutation>> {
        Ok(semio_s_artifact_process_process3d_host::process3d_document_store_initialization_job(envelope, operation, generation, actor))
    }

    /// 🔐️ The exact publication gate the host consults once, right before its non-rejecting commit
    /// — so a lease the app admitted for itself (`process3d_admit_app_publication_authority`, every
    /// host-begun `Effect::LoadDocument`) is consumed here; a host-admitted lease stays the host's.
    fn validate_document_store_publication(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, live_generation: semio_framework_job::Generation) -> Result<(), Fault> {
        semio_s_artifact_process_process3d_host::process3d_validate_atomic_publication_authority(operation, generation, live_generation).map_err(|code| Fault::new(FaultOrigin::App, FaultCode::new(code), "Process3d atomic publication authority is absent or stale"))?;
        semio_s_artifact_process_process3d_host::process3d_release_app_publication_authority(operation);
        Ok(())
    }

    fn build_document_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
        Some(Box::new(semio_framework_plugin::ArtifactDocumentStoreDisposer::<Self::Snapshot, Self::Mutation>::new()))
    }

    fn build_config_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::Config, Self::ConfigMutation>>>> {
        Some(semio_framework_plugin::bounded_config_store_disposer::<Self::Config, Self::ConfigMutation>())
    }

    /// 🧹️ Every owned store lane the close ladder drives needs its bounded disposer, the absent ones
    /// (`NoDraft`, `NoTransient`) included — `drive_artifact_owned_disposer` faults
    /// `interactive-job.close-owned-disposer-missing` on the first `None` and the app never reaches
    /// its terminal-empty shell (mirrors `📐️cad`/`🧩️puzzle`).
    fn build_draft_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::DraftStore<Self::Draft, Self::DraftMutation>>>> {
        Some(semio_framework_plugin::bounded_document_store_disposer::<NoDraft, NoDraftMutation>())
    }

    fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(std::sync::Arc::new(semio_s_artifact_process_process3d_presence::Process3dPresenceRetirementFactory))
    }

    fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(std::sync::Arc::new(semio_s_artifact_process_process3d_presence::Process3dPresenceRetirementFactory))
    }

    fn build_presence_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::PresenceStore<Self::Presence, Self::PresenceMutation>>>> {
        Some(semio_s_artifact_process_process3d_presence::process3d_presence_store_disposer())
    }

    fn build_transient_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::TransientStore<Self::Transient, Self::TransientMutation>>>> {
        Some(semio_framework_plugin::no_transient_store_disposer())
    }

    fn app_schema() -> Option<::semio_framework_schema_registry::AppSchemaDescriptor> {
        Some(semio_s_artifact_process_process3d_config::schema::app_schema_descriptor())
    }

    fn child_restore_projection(snapshot: &Self::Snapshot) -> Result<semio_framework_composition_admission::ChildRestoreProjection<'_>, Fault> {
        semio_s_artifact_process_process3d_model::snapshot::process3d_child_restore_projection(snapshot)
    }

    fn genesis_child_pack(snapshot: &Self::Snapshot, slot: &str, child_id: &str) -> Result<Option<Vec<u8>>,semio_framework_value::ValueError> {
 Ok((||{
        semio_s_artifact_process_process3d_model::snapshot::genesis_process3d_child_pack(snapshot, slot, child_id)
    
})())
}

    fn initial_snapshot() -> Process3dSnapshot {
        semio_s_artifact_process_process3d_model::standards::v1::subsets::any::io::text::snapshot::default_document()
    }

    fn io() -> Option<semio_framework_plugin::AppIo> {
        Some(process3d_io())
    }

    //#region 🔖️Media
    /// 🎞️ `brep:out` (see the artifact engine's `export_process3d_model`, STEP text) plus the inherited
    /// `document:out` default (the pack of `doc.snapshot`, replicated inline — overriding `export_media`
    /// shadows the trait's provided body for every port on this app, not just the new one).
    fn export_media(port: &str, doc: &ArtifactView<'_, Process3dSnapshot>) -> Result<semio_framework_plugin::Media, MediaError> {
        match port {
            "brep:out" => match semio_s_artifact_process_process3d_io::export_process3d_model(&semio_s_artifact_process_process3d_model::snapshot::process_working_scene_from_snapshot(doc.snapshot), None, "step").map_err(|error| MediaError::Payload("brep:out".into(), error))? {
                Some(export) => {
                    let text = match export.data {
                        DslValue::String(text) => text,
                        other => semio_framework_pack_json::to_json_string(&other),
                    };
                    Ok(semio_framework_plugin::Media { media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Brep }, payload: MediaPayload::Structured { schema: "3d.process".into(), json: text } })
                }
                None => Err(MediaError::Payload("brep:out".into(), "kernel replay failed".into())),
            },
            "artifact:out" => {
                let media_type = Self::io().map_or(MediaType { class: MediaClass::Data, form: MediaForm::Value }, |io| io.artifact_media_type);
                let bytes = doc.snapshot.encode_pack();
                Ok(semio_framework_plugin::Media { media_type, payload: MediaPayload::Structured { schema: Self::DOCUMENT_SCHEMA.to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) } })
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    /// 📥️ `geometry:in` (best-effort STEP-text import) replaces the whole document via a `Effect::LoadDocument`
    /// (`reset_process3d_document_effect`, outside undo history); there is no import mutation. Overriding `import_media`
    /// shadows the trait's provided body for every port, so `document:in` is simply unimplemented here.
    fn import_media(port: &str, media: &semio_framework_plugin::Media, _doc: &ArtifactView<'_, Process3dSnapshot>) -> Result<Emit<Process3dMutation, Process3dConfigMutation, Self::DraftMutation>, MediaError> {
        match port {
            "geometry:in" => {
                let MediaPayload::Structured { schema, json } = &media.payload else {
                    return Err(MediaError::Payload("geometry:in".into(), "expected a structured payload".into()));
                };
                if schema != semio_s_artifact_process_process3d_model::snapshot::PROCESS_3D_SCHEMA && schema != "3d.process" {
                    return Err(MediaError::Payload("geometry:in".into(), format!("unrecognized schema: {schema}")));
                }
                // 📦️ `export_process3d_model("step")` hands back raw (non-base64) STEP text — the
                // "stdio.step" format is not binary — so this re-encodes it as base64 to satisfy
                // `import_process3d_model`'s `data:...,<base64>` expectation.
                let data_url = format!("data:application/octet-stream;base64,{}", base64_codec::base64_standard_encode(json.as_bytes()));
                match semio_s_artifact_process_process3d_io::import_process3d_model("geometry-in.step", &data_url) {
                    Some(snapshot) => Ok(Emit { effects: vec![reset_process3d_document_effect(&snapshot)], ..Default::default() }),
                    None => Err(MediaError::Payload("geometry:in".into(), "STEP import failed".into())),
                }
            }
            _ => Err(MediaError::NotImplemented),
        }
    }
    //#endregion 🔖️Media

    /// 🏷️ The manifest action id each command was declared under — supplied wholesale by
    /// `app_commands!`'s generated `command_id()`.
    fn command_id(command: &Process3dCommand) -> &'static str {
        command.command_id()
    }

    /// 🎯️ Exhaustive host-action bridge into the closed `Process3dCommand` enum. React and wgpu still
    /// emit manifest action ids plus JSON arguments; only this boundary interprets that transport shape.
    fn command_from_action(action: &str, args: &mut Option<DslValue>, original_fault: &mut Option<Fault>) -> Result<Self::Command, Fault> {
        if let Some(fault) = original_fault.take() { return Err(fault); }
        let args = args.as_ref();
        let field = |key: &str| args.and_then(|value| value.get(key));
        let string_field = |key: &str| field(key).and_then(DslValue::as_str).map(str::to_string);
        let number_field = |key: &str| field(key).and_then(DslValue::as_f64);
        let unsigned_field = |key: &str| field(key).and_then(|value| value.as_f64()).filter(|number| number.is_finite() && *number >= 0.0).map(|number| number as u64);
        let signed_field = |key: &str| field(key).and_then(|value| value.as_f64()).filter(|number| number.is_finite()).map(|number| number as i64);
        let vec3_field = |key: &str| -> Option<[f64; 3]> {
            let values = field(key)?.as_array()?;
            if values.len() != 3 {
                return None;
            }
            Some([values[0].as_f64()?, values[1].as_f64()?, values[2].as_f64()?])
        };
        let vec2_field = |key: &str| -> Option<[f64; 2]> {
            let values = field(key)?.as_array()?;
            if values.len() != 2 {
                return None;
            }
            Some([values[0].as_f64()?, values[1].as_f64()?])
        };
        match action {
            "setSnapshot" => {
                let json = string_field("json").or_else(|| field("document").map(semio_framework_pack_json::to_json_string)).unwrap_or_default();
                Ok(Process3dCommand::SetDocument(set_snapshot::SetDocument { json }))
            }
            "setActiveExample" => Ok(Process3dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: string_field("exampleId").or_else(|| string_field("id")).unwrap_or_else(|| PROCESS3D_EXAMPLE_TIMBER.into()) })),
            "addStep" => Ok(Process3dCommand::AddStep(add_step::AddStep {
                measure: string_field("measure"),
                machine_id: string_field("machineId").or_else(|| string_field("machine_id")),
                capability_id: string_field("capabilityId").or_else(|| string_field("capability_id")),
                position: vec3_field("position"),
            })),
            "addWorkshopMachine" => Ok(Process3dCommand::AddWorkshopMachine(add_workshop_machine::AddWorkshopMachine {
                catalog_id: string_field("catalogId").or_else(|| string_field("catalog_id")).unwrap_or_default(),
                machine_id: string_field("machineId").or_else(|| string_field("machine_id")).unwrap_or_default(),
            })),
            "removeWorkshopMachine" => Ok(Process3dCommand::RemoveWorkshopMachine(remove_workshop_machine::RemoveWorkshopMachine { id: string_field("id").unwrap_or_default() })),
            "updateWorkshopMachine" => Ok(Process3dCommand::UpdateWorkshopMachine(update_workshop_machine::UpdateWorkshopMachine {
                machine: args
                    .and_then(|value| value.get("machine"))
                    .cloned()
                    .map(<WorkshopMachine as semio_framework_value::FromValue>::from_value)
                    .transpose()
                    .map_err(|error| process3d_action_fault(action, format!("invalid 'machine': {error}")))?
                    .unwrap_or(WorkshopMachine { id: String::new(), label: String::new(), icon_id: String::new(), catalog_id: None, capabilities: Vec::new() }),
            })),
            "removeStep" => Ok(Process3dCommand::RemoveStep(remove_step::RemoveStep { id: string_field("id").unwrap_or_default() })),
            "removeSelectedStep" => Ok(Process3dCommand::RemoveSelectedStep(remove_selected_step::RemoveSelectedStep {})),
            "moveStep" => Ok(Process3dCommand::MoveStep(move_step::MoveStep { id: string_field("id").unwrap_or_default(), index: unsigned_field("index").unwrap_or_default() as usize })),
            "updateStep" => {
                let step_json = string_field("stepJson").or_else(|| string_field("step_json")).or_else(|| field("step").map(semio_framework_pack_json::to_json_string)).unwrap_or_default();
                Ok(Process3dCommand::UpdateStep(update_step::UpdateStep { step_json }))
            }
            "setStepEnabled" => Ok(Process3dCommand::SetStepEnabled(set_step_enabled::SetStepEnabled {
                id: string_field("id").unwrap_or_default(),
                enabled: field("enabled").and_then(DslValue::as_bool).ok_or_else(|| process3d_action_fault(action, "requires the boolean 'enabled' it sets"))?,
            })),
            "setStock" => Ok(Process3dCommand::SetStock(set_stock::SetStock { kind: string_field("kind").or_else(|| string_field("value")).unwrap_or_else(|| "box".into()) })),
            "patchInspector" => Ok(Process3dCommand::PatchInspector(patch_inspector::PatchInspector {
                target: string_field("target").unwrap_or_default(),
                field: string_field("field").unwrap_or_default(),
                number: number_field("number").or_else(|| number_field("value")),
                text: string_field("text").or_else(|| string_field("value")),
            })),
            "setCursor" => Ok(Process3dCommand::SetCursor(set_cursor::SetCursor { value: unsigned_field("value") })),
            "stepCursor" => Ok(Process3dCommand::StepCursor(step_cursor::StepCursor { delta: signed_field("delta").unwrap_or_default() })),
            "stepCursorBack" => Ok(Process3dCommand::StepCursorBack(step_cursor_back::StepCursorBack {})),
            "stepCursorForward" => Ok(Process3dCommand::StepCursorForward(step_cursor_forward::StepCursorForward {})),
            "engagementSubmit" => Ok(Process3dCommand::EngagementSubmit(engagement_submit::EngagementSubmit {})),
            "worldPointerDown" => Ok(Process3dCommand::WorldPointerDown(world_pointer_down::WorldPointerDown { position: vec3_field("position").unwrap_or_default() })),
            "worldFaceDragEnd" => Ok(Process3dCommand::WorldFaceDragEnd(world_face_drag_end::WorldFaceDragEnd {
                normal: vec3_field("normal").unwrap_or([0.0, 0.0, 1.0]),
                start_point: vec3_field("startPoint").or_else(|| vec3_field("start_point")).unwrap_or_default(),
                distance: number_field("distance").unwrap_or_default(),
                face_extent: vec2_field("faceExtent").or_else(|| vec2_field("face_extent")),
            })),
            "importModelFile" => Ok(Process3dCommand::ImportModelFile(import_model_file::ImportModelFile { name: string_field("name").unwrap_or_default(), payload: string_field("payload").unwrap_or_default() })),
            "engagementInput" => Ok(Process3dCommand::EngagementInput(engagement_input::EngagementInput { value: string_field("value").unwrap_or_default() })),
            "engagementAbort" => Ok(Process3dCommand::EngagementAbort(engagement_abort::EngagementAbort {})),
            "setCamera" => Ok(Process3dCommand::SetCamera(set_camera::SetCamera { position: vec3_field("position").unwrap_or([3.0, -3.0, 2.0]), target: vec3_field("target").unwrap_or_default(), fov: number_field("fov").unwrap_or(45.0) })),
            "toggleSun" => Ok(Process3dCommand::ToggleSun(toggle_sun::ToggleSun {})),
            "setSunAzimuth" => Ok(Process3dCommand::SetSunAzimuth(set_sun_azimuth::SetSunAzimuth { value: number_field("value").unwrap_or_default() })),
            "setSunElevation" => Ok(Process3dCommand::SetSunElevation(set_sun_elevation::SetSunElevation { value: number_field("value").unwrap_or_default() })),
            "setSunIntensity" => Ok(Process3dCommand::SetSunIntensity(set_sun_intensity::SetSunIntensity { value: number_field("value").unwrap_or_default() })),
            "setContributions" => Ok(Process3dCommand::SetContributions(set_contributions::SetContributions { json: string_field("json").unwrap_or_else(|| "[]".into()) })),
            "exportModel" => Ok(Process3dCommand::ExportModel(export_model::ExportModel { format: string_field("format").unwrap_or_else(|| "step".into()) })),
            "loadModelRequest" => Ok(Process3dCommand::LoadModelRequest(load_model_request::LoadModelRequest {})),
            other => Err(process3d_action_fault(other, "not declared by the process app's typed command vocabulary")),
        }
    }

    fn host_configuration_mutation(action: &str, args: Option<&DslValue>) -> Result<Option<Self::ConfigMutation>, Fault> {
        Ok((action == "setContributions").then(|| Process3dConfigMutation::SetContributions(Process3dConfigSetContributions{
            json: installable_contributions(args.and_then(|value| value.get("json")).and_then(DslValue::as_str).unwrap_or("[]"), PROCESS3D_CONFIG_CONTRIBUTIONS_BYTES),
        })))
    }

    /// 🕹️ FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM (26/08/14): reads the framework-owned
    /// `"geometry"` domain selection once per dispatch and threads it through `Process3dDispatchCtx`
    /// — the one retained verb that operates ON the selection (`remove_selected_step`) reads it from
    /// there; every other command ignores it (mirrors `📐️cad`'s own `handle`).
    fn handle(
        command: &Process3dCommand,
        doc: &ArtifactView<'_, Process3dSnapshot>,
        cfg: &ConfigView<'_, Process3dConfig>,
        interaction: &semio_framework_plugin::app::InteractionView<'_>,
        view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<Process3dMutation, Process3dConfigMutation, Self::DraftMutation>, Fault> {
        let selection = interaction.selection(PROCESS3D_INTERACTION_DOMAIN);
        let mut ctx = Process3dDispatchCtx { interaction: Process3dInteractionSnapshot { ids: selection.ids.clone() }, view_state: view_state.cloned() };
        command.dispatch(doc, cfg, &mut ctx)
    }

    /// 🧮️ process3d exposes no genuinely settings-like sticky defaults — every `Process3dConfig` field
    /// is session-only view state, so this stays at the trait default.
    fn config_spec() -> semio_framework_plugin::ConfigSpec {
        semio_framework_plugin::ConfigSpec::default()
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Process3dSnapshot>, cfg: &ConfigView<'_, Process3dConfig>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        process3d_render_body(body_key, doc.snapshot, cfg.snapshot, view_state, &[])
    }

    /// 🕹️ FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM (26/08/14): resolves the live `"geometry"`
    /// selection once per render and threads it into `process3d_render_body`, so the inspection
    /// panel finally sees a real selection instead of always rendering empty.
    fn render_with_request_context(
        _owner: &semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
        body_key: &str,
        doc: &ArtifactView<'_, Process3dSnapshot>,
        cfg: &ConfigView<'_, Process3dConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        _transient: &semio_framework_plugin::TransientView<'_, semio_framework_plugin::NoTransient>,
        interaction: &semio_framework_plugin::app::InteractionView<'_>,
        cx:&mut semio_framework_plugin::StepContext<'_>,
    ) -> semio_framework_plugin::OriginalRenderResult<semio_framework_plugin::ComponentTree> {
        let rendered:semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree>=(||{
        let selected_ids = interaction.selection(PROCESS3D_INTERACTION_DOMAIN).ids.clone();
        process3d_render_body(body_key, doc.snapshot, cfg.snapshot, view_state, &selected_ids)

        })();
        rendered.map(Some).map_err(Into::into)
        }

    fn window_engagements(doc: &ArtifactView<'_, Process3dSnapshot>, cfg: &ConfigView<'_, Process3dConfig>, view_state: &semio_framework_plugin::ViewModel) -> HashMap<String, semio_framework_plugin::WindowEngagement> {
        let active_utility = view_state.active_utility_id.as_deref().filter(|utility| !utility.is_empty()).unwrap_or(PROCESS3D_DEFAULT_UTILITY);
        HashMap::from([(workpiece::PROCESS_3D_PLAY_WINDOW_MAIN.into(), workpiece::engagement(doc.snapshot, cfg.snapshot, active_utility, process3d_labels(view_state)))])
    }

    fn window_measures(_doc: &ArtifactView<'_, Process3dSnapshot>, cfg: &ConfigView<'_, Process3dConfig>, view_state: &semio_framework_plugin::ViewModel) -> HashMap<String, Vec<WindowMeasure>> {
        HashMap::from([(workpiece::PROCESS_3D_PLAY_WINDOW_MAIN.into(), workpiece::window_measures(cfg.snapshot, process3d_labels(view_state).is_de()))])
    }

    /// 🖱️ Interaction-less twin of [`Self::context_menu_with_request_context`] — an empty
    /// `"geometry"` domain, so the destructive row is withheld.
    fn context_menu(_request: &ContextMenuRequest, _doc: &ArtifactView<'_, Process3dSnapshot>, _cfg: &ConfigView<'_, Process3dConfig>, view_state: &semio_framework_plugin::ViewModel, registry: &AppActionRegistry) -> Vec<ContextMenuItemSpec> {
        process3d_context_menu_items(registry, view_state, &[])
    }

    /// 🕹️ `removeSelectedStep` appears only for a real `"geometry"` selection — the authoritative
    /// framework-owned one, since `ContextMenuRequest.surface.selection` only carries what the clicked
    /// surface itself painted (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM).
    fn context_menu_with_request_context(
        _request: &ContextMenuRequest,
        _doc: &ArtifactView<'_, Process3dSnapshot>,
        _cfg: &ConfigView<'_, Process3dConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        interaction: &semio_framework_plugin::app::InteractionView<'_>,
        registry: &AppActionRegistry,
    ) -> Vec<ContextMenuItemSpec> {
        process3d_context_menu_items(registry, view_state, &interaction.selection(PROCESS3D_INTERACTION_DOMAIN).ids)
    }
}
