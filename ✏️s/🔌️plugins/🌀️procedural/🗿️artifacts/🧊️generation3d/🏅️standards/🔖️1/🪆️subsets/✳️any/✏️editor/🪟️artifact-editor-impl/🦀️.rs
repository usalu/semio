//! 🪟️ Artifact editor impl: the artifact editor implementation of the play app, the dispatch-only editor surface. See [`super`].

use super::*;

//#region 🔖️Generation3dPlayApp
impl ArtifactEditor for Generation3dPlayApp {
    /// 📚️ Artifact catalogue stamped by `PluginBuilder::editor` onto the navbar dropdown.
    fn examples() -> Vec<ExampleSource> {
        examples()
    }
    /// 🧩️ The loaded-parent child projection every archive load and maintenance swap asks for before a
    /// decoded document may replace the store. `Generation3dSnapshot` declares no child slot, so the
    /// projection is honestly empty; without it every replacement faulted with `editor did not declare
    /// a loaded-parent child projection`.
    fn child_restore_projection(snapshot: &Self::Snapshot) -> Result<semio_framework_composition_admission::ChildRestoreProjection<'_>, Fault> {
        semio_framework_composition_admission::ChildRestoreProjection::from_snapshot(snapshot).map_err(|error| Fault::new(FaultOrigin::App, FaultCode::new(semio_s_artifact_procedural_3d_model::GENERATION3D_CHILD_PROJECTION), error.to_string()))
    }

    /// 🛍️ Publishes the whole registered flow operator catalogue once per app instance on the reserved
    /// `framework.section.catalogue` retained surface — never on the node-graph scene, whose fixed
    /// `UI_FIXED_BYTES` admission it exceeds threefold with the real `brep`/`math` sets installed
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END §3.1).
    fn app_catalogue_json() -> String {
        semio_framework_pack_json::to_json_string(&catalogue_panel::catalogue())
    }

    type Snapshot = Generation3dSnapshot;
    type Mutation = Generation3dMutation;
    type Config = Generation3dConfig;
    type ConfigMutation = Generation3dConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = semio_s_artifact_procedural_3d_editor_presence::Generation3dPresence;
    type PresenceMutation = semio_s_artifact_procedural_3d_editor_presence::Generation3dPresenceMutation;
    type Transient = Generation3dTransient;
    type TransientMutation = Generation3dTransientMutation;

    type Command = Generation3dCommand;

    const REQUIRES_DOCUMENT_STORE_PUBLICATION_AUTHORITY: bool = true;

    fn build_envelope_decode_owner_bundle() -> Option<store::os_store::owned_schema_decode::ArtifactEnvelopeDecodeOwnerBundle<Self::Snapshot, Self::Mutation>> {
        Some(semio_s_artifact_procedural_3d_envelope::generation3d_envelope_decode_owner_bundle())
    }

    /// 🧠️ One retained `FlowEvalSession` per app instance — see [`Generation3dInstanceOperationOwner`].
    fn build_instance_operation_owner() -> Box<dyn semio_framework_plugin::ArtifactInstanceOperationOwner> {
        Box::new(Generation3dInstanceOperationOwner::new())
    }

    fn build_transient_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactEphemeralOneItemPreparationFactory<Self::Transient, Self::TransientMutation>>> {
        Some(semio_framework_plugin::bounded_transient_preparation_factory::<Self::Transient, Self::TransientMutation>())
    }

    fn build_transient_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Transient>>> {
        Some(semio_framework_plugin::bounded_transient_root_retirement_factory::<Self::Transient>())
    }

    fn build_document_store_initialization_job(
        envelope: store::os_store::store_schema::ArtifactEnvelope<Self::Snapshot, Self::Mutation>,
        operation: semio_framework_job::OperationId,
        generation: semio_framework_job::Generation,
        actor: protocol::ActorId,
    ) -> Result<semio_framework_plugin::ArtifactStoreInitializationJob<Self::Snapshot, Self::Mutation>, store::os_store::store_schema::ArtifactEnvelope<Self::Snapshot, Self::Mutation>> {
        Ok(semio_s_artifact_procedural_3d_initialization::generation3d_document_store_initialization_job(envelope, operation, generation, actor))
    }

    fn validate_document_store_publication(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, live_generation: semio_framework_job::Generation) -> Result<(), Fault> {
        semio_s_artifact_procedural_3d_host::generation3d_validate_atomic_publication_authority(operation, generation, live_generation)
            .map_err(|code| Fault::new(FaultOrigin::App, FaultCode::new(code), "Generation3d atomic publication authority is absent or stale"))?;
        semio_s_artifact_procedural_3d_host::generation3d_release_app_publication_authority(operation);
        Ok(())
    }

    fn build_draft_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::DraftStore<Self::Draft, Self::DraftMutation>>>> {
        Some(semio_framework_plugin::no_draft_store_disposer())
    }

    fn register_window_transient_owners(registry: &mut semio_framework_plugin::WindowTransientOwnerRegistry) -> Result<(), Fault> {
        registry.register::<edit_preview::transient::Generation3dPreviewWindowTransientOwner>()?;
        registry.register::<generate_preview::transient::Generation3dGeneratePreviewWindowTransientOwner>()
    }

    /// 🧭️ The leaf that names each tool's history row (design §19.1): a first grab's transaction splices the transform
    /// operator in before its relative leaf, and the row is still labelled by that leaf; a mesh edit's row is labelled by the
    /// `create-widget` of the operator it inserts ("Insert …", audit P4).
    fn tool_intent_kinds(tool: &str) -> &'static [&'static str] {
        match tool.strip_prefix(GENERATION3D_EDITOR_APP_ID).and_then(|verb| verb.strip_prefix('#')) {
            Some("translateSelection") => &["drag-transforms"],
            Some("rotateSelection") => &["rotate-transforms"],
            Some("scaleSelection") => &["scale-transforms"],
            Some("editMeshSelection" | "knifeMeshSelection" | "deleteSelection") => &["create-widget"],
            _ => &[],
        }
    }

    /// 📢️ The localized notices of the editor's own refusal codes (design §20.12): the gumball's `generation3d.gumball.*`, the
    /// preview evaluation's and the document-level ones.
    fn fault_notices() -> &'static [(&'static str, LocalizedLabel)] {
        static NOTICES: std::sync::LazyLock<Vec<(&'static str, LocalizedLabel)>> = std::sync::LazyLock::new(|| {
            semio_s_artifact_procedural_3d_transforms::gumball_fault_notices().iter().chain(add_widget::source_fault_notices()).chain(semio_s_artifact_procedural_3d_preview_eval::preview_eval_fault_notices()).chain(semio_s_artifact_procedural_3d_model::generation3d_document_fault_notices()).cloned().collect()
        });
        NOTICES.as_slice()
    }

    /// 🎯️ `flowEvalTick` publishes the evaluation into ONE preview window's retained transient, and
    /// the whole chain is self-dispatched: an `Effect::DispatchAction` reaches the shell with no
    /// window of its own and is redispatched under whichever window is current — the flow window in
    /// the served app. Naming the target off the PAYLOAD makes the runtime validate it against the
    /// trusted ViewModel roster and capture that exact window's mutation authority, instead of the
    /// current window's (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    /// 🪟️ Every route of the preview chain that may publish — the dispatched hop and both
    /// window-addressed folds, which now continue the chain inline and therefore publish the
    /// intermediate geometry and the advanced status pill the hop used to.
    fn retained_window_transient_target(command: &Self::Command) -> Option<(&str, &'static str)> {
        let (window_id, window_kind_id) = generation3d_flow_eval_window_address(command)?;
        if window_id.is_empty() {
            return None;
        }
        generation3d_preview_kind(window_kind_id).map(|kind| (window_id, kind))
    }

    const DIALECT: Dialect = semio_s_artifact_procedural_3d_model::GENERATION3D_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = GENERATION_3D_SCHEMA;

    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(std::sync::Arc::new(Generation3dArtifactStorePreparationFactory))
    }

    fn build_config_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Config, Self::ConfigMutation>>> {
        Some(store::snapshot_clone_preparation::config_apply_preparation_factory::<Generation3dConfig, Generation3dConfigMutation>())
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(Generation3dBoundedCommandJobFactory::new(&controller))?;
        registry.register(Generation3dFlowEvalJobFactory::new(&controller))?;
        registry.register(Generation3dContributionsJobFactory::new(&controller))?;
        registry.register(Generation3dDocumentIoJobFactory::new(&controller))
    }

    fn build_tool_job(request: ArtifactOwnedToolJobRequest<EditorApp<Self>>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
        if !GENERATION3D_RETAINED_TOOL_IDS.contains(&request.tool_id.as_str())
            && !GENERATION3D_FLOW_EVAL_TOOL_IDS.contains(&request.tool_id.as_str())
            && !GENERATION3D_CONTRIBUTIONS_TOOL_IDS.contains(&request.tool_id.as_str())
            && !GENERATION3D_DOCUMENT_IO_TOOL_IDS.contains(&request.tool_id.as_str())
        {
            return Ok(None);
        }
        if request.command.command_id() != request.tool_id {
            return Err(Fault::new(FaultOrigin::App, FaultCode::new("app.command.tool-mismatch"), "the generation 3d command does not match its exact registered tool"));
        }
        let tool_id = request.command.command_id();
        let contributions = GENERATION3D_CONTRIBUTIONS_TOOL_IDS.contains(&tool_id);
        let work: Box<dyn ArtifactCommandWork<EditorApp<Generation3dPlayApp>>> =
            if contributions {
                Box::new(Generation3dContributionsWork { instance_owner: request.instance_operation_owner, consumed: false })
            } else if GENERATION3D_DOCUMENT_IO_TOOL_IDS.contains(&tool_id) {
                Box::new(Generation3dDocumentIoWork::new(tool_id, request.instance_operation_owner))
            } else if GENERATION3D_FLOW_EVAL_WINDOW_TOOL_IDS.contains(&tool_id) {
                Box::new(Generation3dFlowEvalWindowWork::new(tool_id, request.instance_operation_owner))
            } else if GENERATION3D_PREVIEW_TOOL_IDS.contains(&tool_id) {
                Box::new(Generation3dPreviewCommandWork::new(tool_id, request.instance_operation_owner))
            } else {
                Box::new(Generation3dSessionCommandWork::new(tool_id, request.instance_operation_owner))
            };
        let operation_context = AppOperationContext {
            retained: request.retained,
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
            Generation3dCommand::command_id,
            if contributions {
                GENERATION3D_CONTRIBUTIONS_RAW_BYTES
            } else if GENERATION3D_DOCUMENT_IO_TOOL_IDS.contains(&tool_id) {
                GENERATION3D_DOCUMENT_IO_RAW_BYTES
            } else if GENERATION3D_FLOW_EVAL_TOOL_IDS.contains(&tool_id) {
                GENERATION3D_FLOW_EVAL_RAW_BYTES
            } else {
                GENERATION3D_RETAINED_RAW_BYTES
            },
            GENERATION3D_RETAINED_WORK_ITEMS,
            work,
        );
        Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }

    /// 🧾️ BOTH factories' proofs, in registration order — `validate_tool_job_rows` matches this
    /// catalogue against the registered factories exactly, so a second factory that is registered
    /// and not proved (or proved and not registered) fails the app closed at boot.
    fn bounded_first_step_tool_proofs() -> Vec<semio_framework_plugin::ArtifactBoundedFirstStepProof> {
        let mut proofs = Generation3dBoundedCommandJobFactoryProofs::bounded_first_step_tool_proofs();
        proofs.extend(Generation3dFlowEvalJobFactoryProofs::bounded_first_step_tool_proofs());
        proofs.extend(Generation3dContributionsJobFactoryProofs::bounded_first_step_tool_proofs());
        proofs.extend(Generation3dDocumentIoJobFactoryProofs::bounded_first_step_tool_proofs());
        proofs
    }

    fn app_schema() -> Option<::semio_framework_schema_registry::AppSchemaDescriptor> {
        Some(semio_s_artifact_procedural_3d_editor_config::schema::app_schema_descriptor())
    }

    fn initial_snapshot() -> Generation3dSnapshot {
        semio_s_artifact_procedural_3d_io_text::snapshot::default_snapshot()
    }

    fn io() -> Option<semio_framework_plugin::AppIo> {
        Some(::semio_framework_async::poll::resolve_ready(generation3d_io()))
    }

    /// 📤️ Exports the declared document without evaluating its flow graph.
    fn export_media(port: &str, doc: &ArtifactView<'_, Generation3dSnapshot>) -> Result<semio_framework_plugin::Media, MediaError> {
        if port != "artifact:out" {
            return Err(MediaError::NotImplemented);
        }
        let media_type = Self::io().map_or(MediaType { class: MediaClass::Data, form: MediaForm::Value }, |io| io.artifact_media_type);
        let bytes = pack::snapshot::ArtifactPack::encode_pack(doc.snapshot);
        Ok(semio_framework_plugin::Media { media_type, payload: semio_framework_plugin::MediaPayload::Structured { schema: Self::DOCUMENT_SCHEMA.to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) } })
    }

    /// 🪪️ Reads geometry from the explicitly supplied instance's retained evaluation session.
    fn export_media_with_request_context(
        owner: &semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
        port: &str,
        doc: &ArtifactView<'_, Generation3dSnapshot>,
        _transient: &semio_framework_plugin::TransientView<'_, Generation3dTransient>,
    ) -> Result<semio_framework_plugin::Media, MediaError> {
        if port != "geometry:out" {
            return Self::export_media(port, doc);
        }
        let mesh = owner.with_mut::<Generation3dInstanceOperationOwner, _>(|owner| {
            owner.with_session(|session| export_mesh_from_session(doc.snapshot, &Generation3dConfig::default(), session))
        }).map_err(|error| MediaError::Payload(port.into(), error.message))?.map_err(|error| MediaError::Payload(port.into(), error.to_string()))?;
        semio_s_artifact_procedural_3d_mesh_bridge::semio_mesh_from_mesh_data(&mesh)
            .map_err(|error| MediaError::Payload(port.into(), error.to_string()))?;
        Ok(semio_framework_plugin::Media {
            media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Mesh },
            payload: semio_framework_plugin::MediaPayload::Structured { schema: "3d.mesh".into(), json: semio_framework_pack_json::to_json_string(&mesh) },
        })
    }

    /// 🎞️ `"params:in"` — sets matching `InputSlider` widgets from a `{widgetId: number}` JSON object as ABSOLUTE
    /// `change-slider-value` leaves; unmatched keys, non-slider widgets and unchanged values are ignored.
    fn import_media(port: &str, media: &semio_framework_plugin::Media, doc: &ArtifactView<'_, Generation3dSnapshot>) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation, Self::DraftMutation>, MediaError> {
        match port {
            "params:in" => {
                let semio_framework_plugin::MediaPayload::Structured { json, .. } = &media.payload else {
                    return Err(MediaError::Payload(port.to_string(), "params:in importer only accepts a Structured JSON object payload".into()));
                };
                let parsed = semio_framework_pack_json::parse(json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
                let object = parsed.as_object().cloned().ok_or_else(|| MediaError::Payload(port.to_string(), "params:in payload must be a JSON object".into()))?;
                let host_snapshot = &doc.snapshot.host_snapshot;
                let operations = object
                    .iter()
                    .filter_map(|(target_id, value)| {
                        let number = value.as_f64().filter(|number| number.is_finite())?;
                        host_snapshot
                            .widgets
                            .iter()
                            .any(|widget| matches!(widget, semio_framework_artifact_flow_flow::Widget::InputSlider { id, value: current, .. } if id == target_id && *current != number))
                            .then(|| semio_s_artifact_procedural_3d_mut_widgets::change_slider_value::change_slider_value(target_id.to_string(), number))
                    })
                    .collect();
                Ok(Emit::mutations(operations))
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    fn command_id(command: &Generation3dCommand) -> &'static str {
        command.command_id()
    }

    /// 🎯️ Maps host action id + JSON args onto `Generation3dCommand` — preserved verbatim from the
    /// pre-migration hand-rolled dispatch so React/wgpu callers that still speak the stringly
    /// `{action,args}` wire (rather than `OpBinary` bytes) keep working unchanged.
    fn command_from_action(action: &str, args: &mut Option<semio_framework_value::DslValue>, original_fault: &mut Option<semio_framework::Fault>) -> Result<Self::Command, Fault> {
        let mut original_fault=original_fault;if !matches!(action,"flowEvalResolve"|"flowTessellateResolve"|"flowTessellateCancelResolve"){if let Some(fault)=original_fault.take(){return Err(fault)}}
        let mut args = args.get_or_insert(semio_framework_value::DslValue::Null);
        let str_arg = |keys: &[&str]| -> Option<String> { keys.iter().find_map(|key| args.get(key).and_then(|value| value.as_str()).map(str::to_string)) };
        let string_list = |key: &str| -> Vec<String> { args.get(key).and_then(|value| value.as_array()).map(|rows| rows.iter().filter_map(|row| row.as_str().map(str::to_string)).collect()).unwrap_or_default() };
        let f64_arg = |keys: &[&str]| -> Option<f64> { keys.iter().find_map(|key| args.get(key).and_then(|value| value.as_f64())) };
        let u64_arg = |keys: &[&str]| -> Option<u64> { keys.iter().find_map(|key| args.get(key).and_then(|value| value.as_u64().or_else(|| value.as_f64().map(|number| number as u64)))) };
        fn original_reply_valid(args:&semio_framework_value::DslValue,tessellate:bool,cancel:bool)->bool{
            let semio_framework_value::DslValue::Object(rows)=args else{return false};
            let field=|key:&str|{let mut values=rows.iter().filter(|(name,_)|name==key);let value=values.next().map(|(_,value)|value);if values.next().is_some(){None}else{value}};
            if !["windowId","windowKindId"].iter().all(|key|field(key).and_then(|value|value.as_str()).is_some_and(|value|!value.is_empty()))||field("output").is_none(){return false}
            if cancel{return true}
            if field("nodeHash").and_then(semio_framework_value::DslValue::as_u64).is_none(){return false}
            if tessellate{return field("handle").and_then(|value|value.as_str()).is_some_and(|value|!value.is_empty())&&field("tolerance").and_then(|value|value.as_f64()).is_some_and(|value|value.is_finite()&&value>0.0)}
            let extensions=rows.iter().filter(|(key,_)|key=="extensionId").count();extensions==0||(extensions==1&&field("extensionId").and_then(|value|value.as_str()).is_some())
        }
        fn original_reply_text(args:&mut semio_framework_value::DslValue,key:&str,required:bool)->Result<String,Fault>{
            let semio_framework_value::DslValue::Object(rows)=args else{return Err(Fault::from("original reply argument object is absent"))};let Some((_,source))=rows.iter_mut().find(|(name,_)|name==key)else{return if required{Err(Fault::from("original reply text is absent"))}else{Ok(String::new())}};if !matches!(source,semio_framework_value::DslValue::String(_)){return Err(Fault::from("original reply text has another type"))}let semio_framework_value::DslValue::String(value)=std::mem::replace(source,semio_framework_value::DslValue::Null)else{unreachable!()};Ok(value)
        }
        fn original_reply_output(args:&mut semio_framework_value::DslValue)->Result<semio_framework_value::DslValue,Fault>{let semio_framework_value::DslValue::Object(rows)=args else{return Err(Fault::from("original reply argument object is absent"))};rows.iter_mut().find(|(key,_)|key=="output").map(|(_,source)|std::mem::replace(source,semio_framework_value::DslValue::Null)).ok_or_else(||Fault::from("original reply output is absent"))}

        match action {
            "setActiveExample" => Ok(Generation3dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: str_arg(&["exampleId", "example_id", "value"]).unwrap_or_default() })),
            "nodeGraphEdit" => {
                semio_framework_tool_machine::node_graph_edit_rows(&args).map_err(Fault::from)?;
                Ok(Generation3dCommand::NodeGraphEdit(node_graph_edit::NodeGraphEdit { operations_json: args.get("operations").map(semio_framework_pack_json::to_json_string).unwrap_or_else(|| "[]".into()) }))
            }
            "deleteSelection" => Ok(Generation3dCommand::DeleteSelection(delete_selection::DeleteSelection {})),
            "removeWidget" => Ok(Generation3dCommand::RemoveWidget(remove_widget::RemoveWidget { widget_id: str_arg(&["widgetId", "widget_id", "id"]).unwrap_or_default() })),
            "addWidget" => Ok(Generation3dCommand::AddWidget(add_widget::AddWidget { kind: str_arg(&["kind"]).unwrap_or_else(|| "inputSlider".into()), neuron_kind: str_arg(&["neuronKind"]), format: str_arg(&["format"]), action: str_arg(&["action"]), x: f64_arg(&["x"]), y: f64_arg(&["y"]),source_widget:str_arg(&["sourceWidget"]),source_port:str_arg(&["sourcePort"]),target_port:str_arg(&["targetPort"]),source_revision:str_arg(&["sourceRevision"]) })),
            "setWidgetInput" => {
                let mesh_source = str_arg(&["facet"]).as_deref() == Some("meshSource");
                let index = |key: &str| -> Result<Option<u32>, Fault> {
                    let maximum = if mesh_source && key == "destination" { 599999.0 } else { 1023.0 };
                    args.get(key).map(|value| value.as_f64().filter(|value| value.is_finite() && *value >= 0.0 && *value <= maximum && value.fract() == 0.0).map(|value| value as u32).ok_or_else(|| Fault::from("Choose an integer within the input bounds"))).transpose()
                };
                let operation = args.get("operation").map(|value| value.as_str().filter(|value| matches!(*value,"set"|"add"|"remove"|"move")).map(str::to_string).ok_or_else(|| Fault::from("Unknown list operation"))).transpose()?;
                Ok(Generation3dCommand::SetWidgetInput(set_widget_input::SetWidgetInput {
                widget_id: str_arg(&["widgetId"]).ok_or_else(|| Fault::from("Choose a widget"))?,
                channel: str_arg(&["channel"]).ok_or_else(|| Fault::from("Choose an input"))?,
                value: args.get("value").map(|value| value.as_str().map(str::to_string).unwrap_or_else(|| semio_framework_pack_json::to_json_string(value))).ok_or_else(|| Fault::from("Input value is missing"))?,
                component: str_arg(&["component"]),
                operation,
                index: index("index")?,
                destination: index("destination")?,
                facet: args.get("facet").map(|value| value.as_str().filter(|value| matches!(*value, "port" | "variableName" | "variableSchema" | "exportFormat" | "meshSource")).map(str::to_string).ok_or_else(|| Fault::from("Unknown widget facet"))).transpose()?,
                path: args.get("path").map(|value| value.as_array().filter(|parts| !parts.is_empty() && parts.len() <= 16).ok_or_else(|| Fault::from("Choose a structured mesh field"))?.iter().map(|part| part.as_str().filter(|part| !part.is_empty() && part.chars().count() <= 128).map(str::to_owned).ok_or_else(|| Fault::from("Choose a bounded field name"))).collect::<Result<Vec<_>, _>>()).transpose()?,
            }))
            }
            "patchFlowWidgets" => Ok(Generation3dCommand::PatchFlowWidgets(patch_flow_widgets::PatchFlowWidgets {
                widget_ids: {
                    let mut ids = string_list("widgetIds");
                    if ids.is_empty() {
                        ids = string_list("widget_ids");
                    }
                    ids
                },
                field: str_arg(&["field"]).unwrap_or_default(),
                value: f64_arg(&["value"]),
            })),
            "editMeshSelection" => {
                let number = |key: &str, default: f64| -> Result<f64, Fault> {
                    match args.get(key) {
                        None => Ok(default),
                        Some(value) => value.as_f64().filter(|value| value.is_finite()).ok_or_else(|| Fault::from(format!("{key} must be a finite number"))),
                    }
                };
                let text = |key: &str, default: &str| -> Result<String, Fault> {
                    match args.get(key) {
                        None => Ok(default.into()),
                        Some(value) => value.as_str().map(str::to_string).ok_or_else(|| Fault::from(format!("{key} must be text"))),
                    }
                };
                let integer = |key: &str, maximum: f64| -> Result<u32, Fault> {
                    let value = number(key, 1.0)?;
                    if !value.is_finite() || value.fract() != 0.0 || !(1.0..=maximum).contains(&value) { return Err(Fault::from(format!("{key} must be an integer from 1 to {maximum}"))); }
                    Ok(value as u32)
                };
                let center = match args.get("center") {
                    None => [0.0; 3],
                    Some(value) => {
                        let values = value.as_array().filter(|values| values.len() == 3).ok_or_else(|| Fault::from("Center requires three coordinates"))?;
                        let mut center = [0.0; 3];
                        for axis in 0..3 { center[axis] = values[axis].as_f64().filter(|value| value.is_finite()).ok_or_else(|| Fault::from("Center coordinates must be finite"))?; }
                        center
                    }
                };
                let point=|key:&str,default:[f64;3]|->Result<[f64;3],Fault>{let Some(value)=args.get(key)else{return Ok(default)};let values=value.as_array().filter(|values|values.len()==3).ok_or_else(||Fault::from("Point inputs require three coordinates"))?;let mut point=[0.0;3];for axis in 0..3{point[axis]=values[axis].as_f64().filter(|value|value.is_finite()).ok_or_else(||Fault::from("Point inputs must be finite"))?;}Ok(point)};
                Ok(Generation3dCommand::EditMeshSelection(semio_s_artifact_procedural_3d_editor_commands::edit_mesh_selection::EditMeshSelection {
                    operation: text("operation", "extrude")?, amount: number("amount", 0.1)?, cuts: integer("cuts", 256.0)?,
                    dx: number("dx", 0.0)?, dy: number("dy", 0.0)?, dz: number("dz", 0.0)?,
                    width: number("width", 0.1)?, segments: integer("segments", 64.0)?, merge_mode: text("mergeMode", "center")?,
                    tolerance: number("tolerance", 0.0001)?, radius: number("radius", 1.0)?, grid: number("grid", 1.0)?, center,angle:number("angle",0.1)?,pull_direction:point("pullDirection",[0.0,0.0,1.0])?,neutral_point:point("neutralPoint",[0.0;3])?,
                }))
            }
            "knifeMeshSelection" => {
                let point = |key: &str, default: [f64; 3]| -> Result<[f64; 3], Fault> {
                    let Some(value) = args.get(key) else { return Ok(default); };
                    let values = value.as_array().filter(|values| values.len() == 3).ok_or_else(|| Fault::from("Knife points require three coordinates"))?;
                    let mut point = [0.0; 3];
                    for axis in 0..3 {
                        point[axis] = values[axis].as_f64().filter(|value| value.is_finite() && value.abs() <= f32::MAX as f64).ok_or_else(|| Fault::from("Knife points must be finite mesh coordinates"))?;
                    }
                    Ok(point)
                };
                Ok(Generation3dCommand::KnifeMeshSelection(semio_s_artifact_procedural_3d_editor_commands::knife_mesh_selection::KnifeMeshSelection { start: point("start", [0.0, -1.0, 0.0])?, end: point("end", [0.0, 1.0, 0.0])? }))
            }
            "reorganize" => Ok(Generation3dCommand::Reorganize(reorganize::Reorganize {})),
            "translateSelection" => {
                let mut node_ids = string_list("nodeIds");
                if node_ids.is_empty() {
                    node_ids = string_list("node_ids");
                }
                if node_ids.is_empty() {
                    node_ids = string_list("ids");
                }
                Ok(Generation3dCommand::TranslateSelection(translate_selection::TranslateSelection { node_ids, dx: f64_arg(&["dx"]).unwrap_or(0.0), dy: f64_arg(&["dy"]).unwrap_or(0.0), dz: f64_arg(&["dz"]).unwrap_or(0.0), phase: str_arg(&["phase"]), reason: str_arg(&["reason"]), window_id: str_arg(&["windowId"]) }))
            }
            "rotateSelection" => {
                let mut node_ids = string_list("nodeIds");
                if node_ids.is_empty() {
                    node_ids = string_list("node_ids");
                }
                if node_ids.is_empty() {
                    node_ids = string_list("ids");
                }
                Ok(Generation3dCommand::RotateSelection(rotate_selection::RotateSelection {
                    node_ids,
                    ax: f64_arg(&["ax"]).unwrap_or(0.0),
                    ay: f64_arg(&["ay"]).unwrap_or(0.0),
                    az: f64_arg(&["az"]).unwrap_or(0.0),
                    angle: f64_arg(&["angle"]).unwrap_or(0.0),
                    phase: str_arg(&["phase"]),
                    reason: str_arg(&["reason"]),
                    window_id: str_arg(&["windowId"]),
                }))
            }
            "scaleSelection" => {
                let mut node_ids = string_list("nodeIds");
                if node_ids.is_empty() {
                    node_ids = string_list("node_ids");
                }
                if node_ids.is_empty() {
                    node_ids = string_list("ids");
                }
                Ok(Generation3dCommand::ScaleSelection(scale_selection::ScaleSelection { node_ids, sx: f64_arg(&["sx"]).unwrap_or(1.0), sy: f64_arg(&["sy"]).unwrap_or(1.0), sz: f64_arg(&["sz"]).unwrap_or(1.0), phase: str_arg(&["phase"]), reason: str_arg(&["reason"]), window_id: str_arg(&["windowId"]) }))
            }
            "addGeneration" => Ok(Generation3dCommand::AddGeneration(add_generation::AddGeneration {})),
            "removeGeneration" => Ok(Generation3dCommand::RemoveGeneration(remove_generation::RemoveGeneration { id: str_arg(&["id"]).unwrap_or_default() })),
            "renameGeneration" => Ok(Generation3dCommand::RenameGeneration(rename_generation::RenameGeneration { id: str_arg(&["id"]).unwrap_or_default(), name: str_arg(&["name", "value"]).unwrap_or_default() })),
            "updateGenerationValues" => {
                let value = args.get("value").cloned().unwrap_or(semio_framework_value::DslValue::Null);
                Ok(Generation3dCommand::UpdateGenerationValues(update_generation_values::UpdateGenerationValues {
                    generation_id: str_arg(&["generationId", "generation_id"]),
                    question_id: str_arg(&["questionId", "question_id"]).unwrap_or_default(),
                    value,
                }))
            }
            "nodeGraphViewport" => Ok(Generation3dCommand::NodeGraphViewport(node_graph_viewport::NodeGraphViewport { viewport: parse_flow_viewport(&args)? })),
            "setLodMode" => Ok(Generation3dCommand::SetLodMode(set_lod_mode::SetLodMode { value: str_arg(&["value", "lodMode", "lod_mode"]).unwrap_or_default() })),
            "setShowMode" => Ok(Generation3dCommand::SetShowMode(set_show_mode::SetShowMode { value: str_arg(&["value", "showMode", "show_mode"]).unwrap_or_default() })),
            "cycleShowMode" => Ok(Generation3dCommand::CycleShowMode(cycle_show_mode::CycleShowMode {})),
            "cycleLodMode" => Ok(Generation3dCommand::CycleLodMode(cycle_lod_mode::CycleLodMode {})),
            "selectNextNode" => Ok(Generation3dCommand::SelectNextNode(select_next_node::SelectNextNode {})),
            "selectPreviousNode" => Ok(Generation3dCommand::SelectPreviousNode(select_previous_node::SelectPreviousNode {})),
            "selectUpstreamNode" => Ok(Generation3dCommand::SelectUpstreamNode(select_upstream_node::SelectUpstreamNode {})),
            "selectDownstreamNode" => Ok(Generation3dCommand::SelectDownstreamNode(select_downstream_node::SelectDownstreamNode {})),
            "activateSelection" => Ok(Generation3dCommand::ActivateSelection(activate_selection::ActivateSelection {})),
            "toggleSun" => Ok(Generation3dCommand::ToggleSun(toggle_sun::ToggleSun {})),
            "setSunAzimuth" => Ok(Generation3dCommand::SetSunAzimuth(set_sun_azimuth::SetSunAzimuth { value: f64_arg(&["value"]).unwrap_or(0.0) })),
            "setSunElevation" => Ok(Generation3dCommand::SetSunElevation(set_sun_elevation::SetSunElevation { value: f64_arg(&["value"]).unwrap_or(0.0) })),
            "setSunIntensity" => Ok(Generation3dCommand::SetSunIntensity(set_sun_intensity::SetSunIntensity { value: f64_arg(&["value"]).unwrap_or(1.0) })),
            "setCamera" => Ok(Generation3dCommand::SetCamera(set_camera::SetCamera { camera: parse_preview_camera_json(&args) })),
            "selectGeneration" => Ok(Generation3dCommand::SelectGeneration(select_generation::SelectGeneration { id: str_arg(&["id"]).unwrap_or_default() })),
            "flowEvalTick" => Ok(Generation3dCommand::FlowEvalTick(semio_s_artifact_procedural_3d_preview_eval::FlowEvalTick {
                window_id: str_arg(&["windowId", "window_id"]).unwrap_or_default(),
                window_kind_id: str_arg(&["windowKindId", "window_kind_id"]).unwrap_or_default(),
            })),
            "flowEvalResolve" => {
                let input_valid=original_reply_valid(&args,false,false);
                if !input_valid{return Ok(Generation3dCommand::FlowEvalResolve(semio_s_artifact_procedural_3d_preview_eval::FlowEvalResolve{window_id:String::new(),window_kind_id:String::new(),node_hash:0,extension_id:String::new(),output:semio_framework_value::DslValue::Null,ok:original_fault.is_none(),fault:original_fault.take(),original_input:Some(std::mem::replace(args,semio_framework_value::DslValue::Null)),input_valid}))}
                let node_hash=args.get("nodeHash").and_then(semio_framework_value::DslValue::as_u64).unwrap();
                let window_id=original_reply_text(&mut args,"windowId",true).unwrap();let window_kind_id=original_reply_text(&mut args,"windowKindId",true).unwrap();let extension_id=original_reply_text(&mut args,"extensionId",false).unwrap();let output=original_reply_output(&mut args).unwrap();
                Ok(Generation3dCommand::FlowEvalResolve(semio_s_artifact_procedural_3d_preview_eval::FlowEvalResolve{window_id,window_kind_id,node_hash,extension_id,output,ok:original_fault.is_none(),fault:original_fault.take(),original_input:Some(std::mem::replace(args,semio_framework_value::DslValue::Null)),input_valid}))
            },
            "flowTessellateResolve" => {
                let input_valid=original_reply_valid(&args,true,false);
                if !input_valid{return Ok(Generation3dCommand::FlowTessellateResolve(semio_s_artifact_procedural_3d_preview_eval::FlowTessellateResolve{window_id:String::new(),window_kind_id:String::new(),node_hash:0,handle:String::new(),tolerance:0.0,output:semio_framework_value::DslValue::Null,fault:original_fault.take(),original_input:Some(std::mem::replace(args,semio_framework_value::DslValue::Null)),input_valid}))}
                let node_hash=args.get("nodeHash").and_then(semio_framework_value::DslValue::as_u64).unwrap();let tolerance=args.get("tolerance").and_then(|value|value.as_f64()).unwrap();
                let window_id=original_reply_text(&mut args,"windowId",true).unwrap();let window_kind_id=original_reply_text(&mut args,"windowKindId",true).unwrap();let handle=original_reply_text(&mut args,"handle",true).unwrap();let output=original_reply_output(&mut args).unwrap();
                Ok(Generation3dCommand::FlowTessellateResolve(semio_s_artifact_procedural_3d_preview_eval::FlowTessellateResolve{window_id,window_kind_id,node_hash,handle,tolerance,output,fault:original_fault.take(),original_input:Some(std::mem::replace(args,semio_framework_value::DslValue::Null)),input_valid}))
            },
            "flowEvalRelease" => Ok(Generation3dCommand::FlowEvalRelease(semio_s_artifact_procedural_3d_preview_eval::FlowEvalRelease {
                window_id: str_arg(&["windowId", "window_id"]).unwrap_or_default(),
                window_kind_id: str_arg(&["windowKindId", "window_kind_id"]).unwrap_or_default(),
            })),
            "flowTessellateCancelResolve"=>{let input_valid=original_reply_valid(&args,false,true);if !input_valid{return Ok(Generation3dCommand::FlowTessellateCancelResolve(semio_s_artifact_procedural_3d_preview_eval::FlowTessellateCancelResolve{window_id:String::new(),window_kind_id:String::new(),output:semio_framework_value::DslValue::Null,ok:original_fault.is_none(),fault:original_fault.take(),original_input:Some(std::mem::replace(args,semio_framework_value::DslValue::Null)),input_valid}))}let window_id=original_reply_text(&mut args,"windowId",true).unwrap();let window_kind_id=original_reply_text(&mut args,"windowKindId",true).unwrap();let output=original_reply_output(&mut args).unwrap();Ok(Generation3dCommand::FlowTessellateCancelResolve(semio_s_artifact_procedural_3d_preview_eval::FlowTessellateCancelResolve{window_id,window_kind_id,output,ok:original_fault.is_none(),fault:original_fault.take(),original_input:Some(std::mem::replace(args,semio_framework_value::DslValue::Null)),input_valid}))},
            "setContributions" => Ok(Generation3dCommand::SetContributions(set_contributions::SetContributions {
                json: str_arg(&["json"]).unwrap_or_default(),
                page: u64_arg(&["page"]).unwrap_or_default(),
                page_count: u64_arg(&["pageCount", "page_count"]).unwrap_or(1),
            })),
            "importDocumentRequest" => Ok(Generation3dCommand::ImportDocumentRequest(import_document_request::ImportDocumentRequest { widget_id: str_arg(&["widgetId"]), channel: str_arg(&["channel"]), texture_id: str_arg(&["textureId"]) })),
            "importDocument" => Ok(Generation3dCommand::ImportDocument(import_document::ImportDocument {
                name: str_arg(&["name"]).unwrap_or_default(),
                payload: str_arg(&["payload", "contents"]).unwrap_or_default(),
                widget_id: str_arg(&["widgetId"]), channel: str_arg(&["channel"]), texture_id: str_arg(&["textureId"]),
            })),
            "exportDocument" => Ok(Generation3dCommand::ExportDocument(export_document::ExportDocument { format: str_arg(&["format", "value"]).unwrap_or_else(|| "stl".into()), widget_id: str_arg(&["widgetId"]) })),
            other => Err(Fault::from(format!(
                "action '{other}' is not a framework-reserved action (history/clipboard/revert/filter/noteShellCommand) — \
                 app actions are dispatched exclusively through the typed command channel now (see `dispatch_typed_command`)"
            ))),
        }
    }

    /// 📨️ Every host event ends the window's open gumball gesture with zero trace under the reason the tool records: a blur
    /// `blur`, a lost pointer capture `captureLost`, a utility switch or a closing window `retired`, an opened history edit
    /// `frozen` and a remote edit `baseMoved` — the typed `translateSelection{phase: "abort"}` of that window.
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
        Some(Generation3dCommand::TranslateSelection(translate_selection::TranslateSelection {
            node_ids: Vec::new(),
            dx: 0.0,
            dy: 0.0,
            dz: 0.0,
            phase: Some("abort".into()),
            reason: Some(reason.as_str().into()),
            window_id: Some(event.window_id().to_string()),
        }))
    }

    /// 🕹️ `deleteSelection`/`nodeGraphEdit`/`{translate,rotate,scale}Selection` read the `graph`
    /// interaction domain directly (bypassing the `app_commands!`-generated `dispatch`, whose
    /// per-row `$module::handle(payload, doc, cfg, ctx)` signature is framework-fixed and has no
    /// `interaction` slot) — ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM.
    fn handle(
        command: &Generation3dCommand,
        doc: &ArtifactView<'_, Generation3dSnapshot>,
        cfg: &ConfigView<'_, Generation3dConfig>,
        interaction: &InteractionView<'_>,
        view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation, Self::DraftMutation>, Fault> {
        with_scratch_session(|session| {
            let component_ids = &interaction.selection(semio_s_artifact_procedural_3d_editor_selection::DOMAIN).ids;
            if let Some(ids) = generation3d_component_targets(command, view_state, interaction.active_granularity(semio_s_artifact_procedural_3d_editor_selection::DOMAIN), component_ids, &interaction.selection("graph").ids).filter(|ids| !ids.is_empty()) {
                semio_s_artifact_procedural_3d_editor_selection::validate_cached_components(&doc.snapshot.host_snapshot, session, ids, doc.operation().ok().map(|operation| &operation.canonical_base_revision), doc.retained_grant()?).map_err(Fault::from)?;
            }
            match command {
            Generation3dCommand::EditMeshSelection(payload) => semio_s_artifact_procedural_3d_editor_commands::edit_mesh_selection::apply_selected("editMeshSelection", payload, doc, &interaction.selection(semio_s_artifact_procedural_3d_editor_selection::DOMAIN).ids),
            Generation3dCommand::KnifeMeshSelection(payload) => semio_s_artifact_procedural_3d_editor_commands::knife_mesh_selection::apply_selected(payload, doc, &interaction.selection(semio_s_artifact_procedural_3d_editor_selection::DOMAIN).ids),
            Generation3dCommand::DeleteSelection(_) if semio_s_artifact_procedural_3d_mode_edit::windows::preview::edits_components(view_state, interaction.active_granularity(semio_s_artifact_procedural_3d_editor_selection::DOMAIN)) => semio_s_artifact_procedural_3d_editor_commands::edit_mesh_selection::delete_selected(doc, &interaction.selection(semio_s_artifact_procedural_3d_editor_selection::DOMAIN).ids),
            Generation3dCommand::DeleteSelection(payload) => delete_selection::apply(payload, doc, cfg, interaction, session),
            _ if generation3d_gumball_command(command) => {
                let gesture = generation3d_gumball_gesture(command).ok_or_else(|| Fault::from("generation3d-gumball-command"))?;
                let components = semio_s_artifact_procedural_3d_mode_edit::windows::preview::edits_components(view_state, interaction.active_granularity(semio_s_artifact_procedural_3d_editor_selection::DOMAIN)).then(|| interaction.selection(semio_s_artifact_procedural_3d_editor_selection::DOMAIN).ids.as_slice());
                let ids = semio_s_artifact_procedural_3d_transforms::gumball_ids(&doc.snapshot.host_snapshot, gesture.ids, components, &interaction.selection("graph").ids)?;
                semio_s_artifact_procedural_3d_transforms::gumball_once(gesture.verb, ids, gesture.motion, doc)
            }
            Generation3dCommand::SelectNextNode(_payload) => Ok(select_next_node::apply_selected(doc, &interaction.selection("graph").ids)),
            Generation3dCommand::SelectPreviousNode(_payload) => Ok(select_previous_node::apply_selected(doc, &interaction.selection("graph").ids)),
            Generation3dCommand::SelectUpstreamNode(_payload) => Ok(select_upstream_node::apply_selected(doc, &interaction.selection("graph").ids)),
            Generation3dCommand::SelectDownstreamNode(_payload) => Ok(select_downstream_node::apply_selected(doc, &interaction.selection("graph").ids)),
            Generation3dCommand::ActivateSelection(_payload) => Ok(activate_selection::apply_ports(doc, &generation3d_port_ids_by_node(&doc.snapshot.host_snapshot), &interaction.selection("graph").ids)),
            _ => command.dispatch(doc, cfg, session),
            }
        })
    }

    /// 🕹️ `graph`'s `HierarchyProvider::Topology` — every widget's visible ports become `handle`
    /// targets (`{nodeId}@{portId}`, byte-identical to the node-graph's own pick ids) parented to
    /// their widget, which is what makes `HoverSpec { transitive: true }` light up every channel's
    /// preview geometry from a single node hover, and resolve a preview-instance hover back to its
    /// node. Every top-level widget is a "node" (root unless
    /// nested in a `Widget::Cluster`'s own `tree.neurons`, where each nested `Neuron` becomes a "node"
    /// parented to its owning cluster's widget id — the DAG-parent-links transitive-hover source: hovering
    /// a Cluster's own tree item transitively covers every widget nested inside it). Synapses become
    /// "edge" targets, parented to nothing (edges are leaves, not containers).
    fn interaction_topology(doc: &ArtifactView<'_, Generation3dSnapshot>, _cfg: &ConfigView<'_, Generation3dConfig>) -> Result<InteractionTopology, semio_framework_value::ValueError> {
 Ok((||{
        fn walk_neuron(neuron: &semio_framework_artifact_flow_flow::neural::Neuron, parent: String, ordered: &mut Vec<TopologyNode>) {
            ordered.push(TopologyNode { id: neuron.id.clone(), granularity: "node".into(), parent: Some(parent) });
            if let Some(tree) = &neuron.tree {
                for child in &tree.neurons {
                    walk_neuron(child, neuron.id.clone(), ordered);
                }
            }
        }
        let host_snapshot = &doc.snapshot.host_snapshot;
        let mut ordered = Vec::new();
        let ports_by_node = generation3d_port_ids_by_node(host_snapshot);
        for widget in &host_snapshot.widgets {
            let id = semio_s_artifact_procedural_3d_model::widget_id(widget).to_string();
            ordered.push(TopologyNode { id: id.clone(), granularity: "node".into(), parent: None });
            for port in ports_by_node.get(&id).into_iter().flatten() {
                ordered.push(TopologyNode { id: port.clone(), granularity: "handle".into(), parent: Some(id.clone()) });
            }
            if let semio_framework_artifact_flow_flow::Widget::Cluster { tree, .. } = widget {
                for child in &tree.neurons {
                    walk_neuron(child, id.clone(), &mut ordered);
                }
            }
        }
        for synapse in &host_snapshot.synapses {
            ordered.push(TopologyNode { id: synapse.id.clone(), granularity: "edge".into(), parent: None });
        }
        let mut domains = std::collections::BTreeMap::new();
        domains.insert("graph".to_string(), DomainTopology { ordered });
        InteractionTopology { domains }
    
})())
}

    /// ⏯️ Starts, finalizes or wakes the `previewEval` run for the attached preview windows
    /// ([`crate::preview_eval::preview_eval_run_effects`]).
    ///
    /// 🪟️ Every hop the run dispatches names its window: an unaddressed `Effect::DispatchAction` is
    /// redispatched by the shell under the CURRENT window — the flow window `procedural-main` in the
    /// served app — and the route's own work then refuses it (`📓️work-capacity-2026-09-10.md` §7).
    /// Before the first `Event::SurfaceVisible` the host has no roster at all (`view` is `None`) and
    /// there is nothing to publish into, so nothing starts.
    ///
    /// 🚧️ A graph the live registry cannot serve is not slow work: a hop would recompute the identical
    /// `flow.extension-not-contributed` miss. Only `setContributions` can move it, and it owes every
    /// attached preview an evaluation itself.
    fn pending_effects(owner: &semio_framework_plugin::ArtifactInstanceOperationOwnerHandle, doc: &ArtifactView<'_, Generation3dSnapshot>, _cfg: &ConfigView<'_, Generation3dConfig>, view: Option<&semio_framework_plugin::ViewModel>) -> Vec<Effect> {
        let windows = generation3d_preview_windows(view);
        let servable = flow_eval_tick::may_rearm(&doc.snapshot.host_snapshot);
        let applied_edits = semio_s_artifact_procedural_3d_preview_eval::applied_document_edits_digest(doc.history) ^ doc.provisional_generation().wrapping_mul(0x9e37_79b9_7f4a_7c15);
        owner
            .with_mut::<Generation3dInstanceOperationOwner, _>(|owner| {
                use semio_s_artifact_procedural_3d_preview_eval::PreviewEvalRunOwner as _;
                let (session, link) = owner.preview_eval_parts().ok_or_else(|| Fault::from("generation3d-eval-session-closing"))?;
                Ok(semio_s_artifact_procedural_3d_preview_eval::preview_eval_run_effects(session, link, &windows, doc.tool_run(), servable, applied_edits))
            })
            .unwrap_or_default()
    }

    /// ⏯️ The `previewEval` run job over the run's base document: the preview widgets it observes are
    /// the document's own, and its hops read the retained session this instance's commands fold into.
    fn build_tool_run_job(request: ToolRunJobRequest<'_, EditorApp<Self>>) -> Result<Option<ToolRunJob>, Fault> {
        if request.tool_id != semio_s_artifact_procedural_3d_preview_eval::PREVIEW_EVAL_TOOL_ID || request.purpose != ToolRunJobPurpose::Run {
            return Ok(None);
        }
        let preview_widget_ids = semio_s_artifact_procedural_3d_preview_eval::preview_widget_ids(&request.snapshot.host_snapshot);
        Ok(Some(Box::new(semio_s_artifact_procedural_3d_preview_eval::PreviewEvalRunJob::<Generation3dInstanceOperationOwner>::new(request.instance_owner, request.port, request.identity, preview_widget_ids)?)))
    }

    /// 🕹️ The marks-free entry point the framework still offers (no owner, no transient, no
    /// interaction) — every live window goes through `render_with_request_context` instead.
    fn render(body_key: &str, doc: &ArtifactView<'_, Generation3dSnapshot>, cfg: &ConfigView<'_, Generation3dConfig>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        with_scratch_session(|session| generation3d_render_body(body_key, doc.snapshot, cfg.snapshot, None, view_state, &PreviewInteractionMarks::default(), session, doc.tool_run()))
    }

    /// 🕹️ Resolves the live `graph` hover/selection once per render and threads it into every
    /// window body — the node graph paints the hovered node/port, the world previews paint the
    /// hovered/selected instances, and the inspection panel finally sees a real selection.
    fn render_with_request_context(
        owner: &semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
        body_key: &str,
        doc: &ArtifactView<'_, Generation3dSnapshot>,
        cfg: &ConfigView<'_, Generation3dConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        transient: &semio_framework_plugin::TransientView<'_, Generation3dTransient>,
        interaction: &InteractionView<'_>,
        _cx: &mut semio_framework_job::StepContext<'_>,
    ) -> semio_framework_plugin::OriginalRenderResult<semio_framework_plugin::ComponentTree> {
        let preview_eval_text = transient
            .window::<edit_preview::transient::Generation3dPreviewWindowTransientOwner>()
            .or_else(|| transient.window::<generate_preview::transient::Generation3dGeneratePreviewWindowTransientOwner>())
            .and_then(|state| state.preview_eval_text.as_deref());
        let mut marks = PreviewInteractionMarks::from_interaction(interaction);
        marks.source_revision = doc.render_operation().map(|operation| operation.canonical_base_revision.iter().map(|byte| format!("{byte:02x}")).collect());
        let previews = matches!(body_key, edit_preview::GENERATION_3D_PLAY_BODY_PREVIEW | generate_preview::GENERATION_3D_PLAY_BODY_GENERATE_PREVIEW);
        let rendered=owner
            .with_mut::<Generation3dInstanceOperationOwner, _>(|owner| {
                let gesture = previews.then(|| generation3d_gumball_preview(doc.snapshot, &marks, &owner.gumball)).flatten();
                let (document, marks) = gesture.as_ref().map_or((doc.snapshot, &marks), |(document, marks)| (document, marks));
                let rendered = owner.with_session(|session| generation3d_render_body(body_key, document, cfg.snapshot, preview_eval_text, view_state, marks, session, doc.tool_run()));
                if let Some((document, _)) = gesture {
                    document.retire_cold();
                }
                rendered
            })
            .map_err(|original|semio_framework_plugin::OriginalRenderFailure::from(semio_framework_job::OriginalTurnFailure::from(original)))?;
        rendered.map(Some).map_err(Into::into)
    }

    fn window_engagements_with_request_context(_doc: &ArtifactView<'_, Generation3dSnapshot>, _cfg: &ConfigView<'_, Generation3dConfig>, view_state: &semio_framework_plugin::ViewModel, _transient: &semio_framework_plugin::TransientView<'_, Generation3dTransient>, interaction: &InteractionView<'_>) -> HashMap<String, semio_framework_plugin::WindowEngagement> {
        let engagement = semio_s_artifact_procedural_3d_editor_selection::engagement(&semio_s_artifact_procedural_3d_editor_selection::ComponentSelection::from_interaction(interaction), view_state.locale == semio_framework_ui_locale::Locale::De);
        HashMap::from([(edit_preview::GENERATION_3D_PLAY_WINDOW_PREVIEW.into(), engagement)])
    }

    fn window_measures(_doc: &ArtifactView<'_, Generation3dSnapshot>, cfg: &ConfigView<'_, Generation3dConfig>, view_state: &semio_framework_plugin::ViewModel) -> HashMap<String, Vec<WindowMeasure>> {
        let config = cfg.snapshot;
        let is_de = view_state.locale == semio_framework_ui_locale::Locale::De;
        let measures = edit_preview::preview_window_measures(config, is_de, generation3d_action);
        HashMap::from([
            (flow_window::GENERATION_3D_PLAY_WINDOW_MAIN.to_string(), flow_window::window_measures(&config.lod_mode, is_de, generation3d_action)),
            (edit_preview::GENERATION_3D_PLAY_WINDOW_PREVIEW.to_string(), measures.clone()),
            (generate_preview::GENERATION_3D_PLAY_WINDOW_GENERATE_PREVIEW.to_string(), measures),
        ])
    }

    /// 🗂️ The marks-free entry point the framework still offers — every live right-click goes
    /// through `context_menu_with_request_context` instead, so this delegate builds the same menu
    /// against an empty selection.
    fn context_menu(
        request: &semio_framework_plugin::ContextMenuRequest,
        doc: &ArtifactView<'_, Generation3dSnapshot>,
        cfg: &ConfigView<'_, Generation3dConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        registry: &semio_framework_plugin::AppActionRegistry,
    ) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {
        Self::context_menu_body(request, doc, cfg, view_state, &PreviewInteractionMarks::default(), registry)
    }

    /// 🕹️ `context_menu`'s interaction-aware twin — the one the runtime actually calls
    /// (`VcsArtifactApp::context_menu`). `ContextMenuRequest.surface.selection` only ever carries what
    /// the clicked surface itself painted, so a world-preview instance selected from the node graph
    /// (or vice versa) never reaches a menu row through it; `interaction` is the authoritative
    /// framework-owned `graph` selection, the same one `render_with_request_context` paints from.
    fn context_menu_with_request_context(
        request: &semio_framework_plugin::ContextMenuRequest,
        doc: &ArtifactView<'_, Generation3dSnapshot>,
        cfg: &ConfigView<'_, Generation3dConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        interaction: &InteractionView<'_>,
        registry: &semio_framework_plugin::AppActionRegistry,
    ) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {
        Self::context_menu_body(request, doc, cfg, view_state, &PreviewInteractionMarks::from_interaction(interaction), registry)
    }
}
//#endregion 🔖️Generation3dPlayApp
