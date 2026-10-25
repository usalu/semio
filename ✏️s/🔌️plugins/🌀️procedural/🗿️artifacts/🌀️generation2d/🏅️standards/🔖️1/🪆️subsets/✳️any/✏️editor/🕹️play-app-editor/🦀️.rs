//! 🕹️ Play app of the generation2d editor: the play app type and its `ArtifactEditor` implementation. See [`super`].

use super::*;
use semio_framework_2d_compute::EngineHandles;

//#region 🔖️Generation2dPlayApp
/// 🧪️ Unit struct apart from `eval_session`: every former runtime field lives in [`Generation2dConfig`],
/// written through [`Generation2dConfigMutation`]s. The eval session is the one piece of state that is
/// neither document nor view — it is threaded into every command handler as the `app_commands!`
/// dispatch context.
#[derive(Default)]
pub struct Generation2dPlayApp;

/// 🎥️ Parses the flow-graph camera out of `command_from_action`'s nested `{viewport: {x, y, zoom}}` args
/// (`nodeGraphViewportActionArgs`, `🧱️elements/🕸️NodeGraph/🟦️.tsx`). An absent `viewport` decodes to the
/// identity camera, because `ActionDefinition::new("nodeGraphViewport", …)` declares no args and every
/// declared action must bridge from its own id under the shell's staged args alone. A present but malformed
/// one still faults: a camera the graph cannot express is never silently replaced by one it can.
fn parse_flow_viewport(args: &semio_framework_value::DslValue) -> Result<semio_framework_os_kernel::Viewport2d, Fault> {
    let Some(value) = args.get("viewport").cloned() else { return Ok(semio_framework_os_kernel::Viewport2d::default()) };
    semio_framework_value::FromValue::from_value(value).map_err(|error| Fault::from(format!("invalid nodeGraphViewport viewport: {error}")))
}

impl ArtifactEditor for Generation2dPlayApp {
    /// 📚️ Artifact catalogue stamped by `PluginBuilder::editor` onto the navbar dropdown.
    fn examples() -> Vec<semio_framework_plugin::ExampleSource> {
        vec![semio_s_artifact_procedural_2d_examples::demo::source()]
    }
    /// 📢️ The localized notices of the editor's own refusal codes (design §20.12): the preview evaluation's and the document-level
    /// ones.
    fn fault_notices() -> &'static [(&'static str, LocalizedLabel)] {
        static NOTICES: std::sync::LazyLock<Vec<(&'static str, LocalizedLabel)>> = std::sync::LazyLock::new(|| semio_s_artifact_procedural_2d_preview_eval::preview_eval_fault_notices().iter().chain(semio_s_artifact_procedural_2d_model::generation2d_document_fault_notices()).cloned().collect());
        NOTICES.as_slice()
    }

    /// 🧩️ The loaded-parent child projection every archive load and maintenance swap asks for before a
    /// decoded document may replace the store. `Generation2dSnapshot` declares no child slot, so the
    /// projection is honestly empty; without it every replacement faulted with `editor did not declare
    /// a loaded-parent child projection`.
    fn child_restore_projection(snapshot: &Self::Snapshot) -> Result<semio_framework_composition_admission::ChildRestoreProjection<'_>, Fault> {
        semio_framework_composition_admission::ChildRestoreProjection::from_snapshot(snapshot).map_err(|error| Fault::new(FaultOrigin::App, FaultCode::new(semio_s_artifact_procedural_2d_model::GENERATION2D_CHILD_PROJECTION), error.to_string()))
    }

    /// 🛍️ Publishes the whole registered flow operator catalogue once per app instance on the reserved
    /// `framework.section.catalogue` retained surface — never on the node-graph scene, whose fixed
    /// `UI_FIXED_BYTES` admission it exceeds threefold with the real `brep`/`math` sets installed
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END §3.1).
    fn app_catalogue_json() -> String {
        semio_framework_os_flow::flow_app_catalogue_json()
    }

    type Snapshot = Generation2dSnapshot;
    type Mutation = Generation2dMutation;
    type Config = Generation2dConfig;
    type ConfigMutation = Generation2dConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = semio_s_artifact_procedural_2d_editor_presence::Generation2dPresence;
    type PresenceMutation = semio_s_artifact_procedural_2d_editor_presence::Generation2dPresenceMutation;
    type Transient = Generation2dTransient;
    type TransientMutation = Generation2dTransientMutation;

    type Command = Generation2dCommand;

    const REQUIRES_DOCUMENT_STORE_PUBLICATION_AUTHORITY: bool = true;

    fn build_envelope_decode_owner_bundle() -> Option<store::os_store::owned_schema_decode::ArtifactEnvelopeDecodeOwnerBundle<Self::Snapshot, Self::Mutation>> {
        Some(semio_s_artifact_procedural_2d_host::generation2d_envelope_decode_owner_bundle())
    }

    /// 🧠️ One retained `FlowEvalSession` per app instance — see [`Generation2dInstanceOperationOwner`].
    fn build_instance_operation_owner() -> Box<dyn semio_framework_plugin::ArtifactInstanceOperationOwner> {
        Box::new(Generation2dInstanceOperationOwner::new())
    }

    fn build_document_store_initialization_job(
        envelope: store::os_store::store_schema::ArtifactEnvelope<Self::Snapshot, Self::Mutation>,
        operation: semio_framework_job::OperationId,
        generation: semio_framework_job::Generation,
        actor: protocol::ActorId,
    ) -> Result<semio_framework_plugin::ArtifactStoreInitializationJob<Self::Snapshot, Self::Mutation>, store::os_store::store_schema::ArtifactEnvelope<Self::Snapshot, Self::Mutation>> {
        Ok(semio_s_artifact_procedural_2d_host::generation2d_document_store_initialization_job(envelope, operation, generation, actor))
    }

    fn validate_document_store_publication(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, live_generation: semio_framework_job::Generation) -> Result<(), Fault> {
        semio_s_artifact_procedural_2d_host::generation2d_validate_atomic_publication_authority(operation, generation, live_generation)
            .map_err(|code| Fault::new(FaultOrigin::App, FaultCode::new(code), "Generation2d atomic publication authority is absent or stale"))?;
        semio_s_artifact_procedural_2d_host::generation2d_release_app_publication_authority(operation);
        Ok(())
    }

    fn build_draft_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::DraftStore<Self::Draft, Self::DraftMutation>>>> {
        Some(semio_framework_plugin::no_draft_store_disposer())
    }

    const DIALECT: Dialect = GENERATION2D_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = GENERATION_2D_SCHEMA;

    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(std::sync::Arc::new(Generation2dArtifactStorePreparationFactory))
    }

    fn build_config_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Config, Self::ConfigMutation>>> {
        Some(std::sync::Arc::new(Generation2dConfigPreparationFactory))
    }

    fn build_transient_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactEphemeralOneItemPreparationFactory<Self::Transient, Self::TransientMutation>>> {
        Some(semio_framework_plugin::bounded_transient_preparation_factory::<Self::Transient, Self::TransientMutation>())
    }

    fn build_transient_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Transient>>> {
        Some(semio_framework_plugin::bounded_transient_root_retirement_factory::<Self::Transient>())
    }

    fn register_window_config_owners(registry: &mut semio_framework_plugin::WindowConfigOwnerRegistry) -> Result<(), Fault> {
        flow_window::config::register(registry)?;
        edit_preview::config::register(registry)?;
        generate_preview::config::register(registry)
    }

    /// 🧾️ BOTH factories' proofs, in registration order — `validate_tool_job_rows` matches this
    /// catalogue against the registered factories exactly.
    fn bounded_first_step_tool_proofs() -> Vec<semio_framework_plugin::ArtifactBoundedFirstStepProof> {
        let mut proofs = Generation2dBoundedCommandJobFactoryProofs::bounded_first_step_tool_proofs();
        proofs.extend(Generation2dContributionsJobFactoryProofs::bounded_first_step_tool_proofs());
        proofs
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(Generation2dBoundedCommandJobFactory::new(&controller))?;
        registry.register(Generation2dContributionsJobFactory::new(&controller))
    }

    /// 🎞️ `import-media` is the only reserved route generation2d owns. The framework registers the
    /// reserved factory but never a concrete importer, so every inbound `params:in` delivery is
    /// routed exclusively through this builder (`dispatch_import_media` →
    /// `build_artifact_reserved_media_job`); `copy`/`cut`/`paste` stay on the framework's own
    /// reserved factories.
    fn build_reserved_tool_job(request: ArtifactReservedToolJobRequest<EditorApp<Self>>) -> Result<Option<ArtifactReservedToolJob>, Fault> {
        if request.tool_id.as_str() != GENERATION2D_IMPORT_TOOL_ID {
            return Ok(None);
        }
        if !request.raw_wire.is_empty() {
            return Err(Fault::from("generation2d import-media admits a decoded media value, never a wire payload"));
        }
        let ArtifactReservedToolInput::Media { port, media } = &request.input else {
            return Err(Fault::from("generation2d import-media requires media input"));
        };
        let (port, media) = (port.clone(), media.clone());
        Ok(Some(ArtifactReservedToolJob::new(Generation2dImportJob::new(request, port, media))))
    }

    fn build_tool_job(request: &mut Option<ArtifactOwnedToolJobRequest<EditorApp<Self>>>, output: &mut Option<semio_framework_plugin::ErasedToolJob>, cx: &mut semio_framework_plugin::__job::StepContext<'_>) -> Result<bool, semio_framework_plugin::__job::OriginalTurnFailure> {
        let original = request.as_ref().ok_or_else(|| semio_framework_plugin::__value::ValueError::literal(semio_framework_plugin::__value::ValueRefusalKind::InvariantViolated, "original retained producer request is absent"))?;
        if !GENERATION2D_BOUNDED_TOOL_IDS.contains(&original.tool_id.as_str()) && !GENERATION2D_CONTRIBUTIONS_TOOL_IDS.contains(&original.tool_id.as_str()) {
            return Ok(false);
        }
        if original.command.command_id() != original.tool_id {
            return Err(semio_framework_plugin::__value::ValueError::literal(semio_framework_plugin::__value::ValueRefusalKind::InvariantViolated, "the generation 2d command does not match its exact registered tool").into());
        }
        let tool_id = original.command.command_id();
        let contributions = GENERATION2D_CONTRIBUTIONS_TOOL_IDS.contains(&tool_id);
        semio_framework_plugin::retained_command::producer::admit_retained_tool_producer(request, output, cx, Generation2dCommand::command_id, if contributions { GENERATION2D_CONTRIBUTIONS_RAW_BYTES } else { GENERATION2D_RETAINED_RAW_BYTES }, GENERATION2D_RETAINED_WORK_ITEMS, false, |request| -> Box<dyn ArtifactCommandWork<EditorApp<Generation2dPlayApp>>> { if contributions {
            Box::new(Generation2dContributionsWork { instance_owner: request.instance_operation_owner.clone(), consumed: false })
        } else if GENERATION2D_FLOW_EVAL_TOOL_IDS.contains(&tool_id) {
            Box::new(Generation2dFlowEvalWork { tool_id, instance_owner: request.instance_operation_owner.clone(), consumed: false,receiver:None,closing:false })
        } else if GENERATION2D_PREVIEW_TOOL_IDS.contains(&tool_id) {
            Box::new(Generation2dGenerationCommandWork { tool_id, instance_owner: request.instance_operation_owner.clone(), consumed: false })
        } else {
            Box::new(Generation2dSessionCommandWork::new(tool_id, request.instance_operation_owner.clone()))
        } })
    }

    fn app_schema() -> Option<::semio_framework_schema_registry::AppSchemaDescriptor> {
        Some(semio_s_artifact_procedural_2d_editor_config::schema::app_schema_descriptor())
    }

    fn initial_snapshot() -> Generation2dSnapshot {
        semio_s_artifact_procedural_2d_io_text::snapshot::default_snapshot()
    }

    fn io() -> Option<semio_framework_plugin::AppIo> {
        Some(::semio_framework_async::poll::resolve_ready(generation2d_io()))
    }

    fn command_id(command: &Generation2dCommand) -> &'static str {
        command.command_id()
    }

    /// 🎯️ Maps host action id + JSON args onto `Generation2dCommand` — preserved verbatim from the
    /// pre-migration hand-rolled dispatch so React/wgpu callers that still speak the stringly
    /// `{action,args}` wire (rather than `OpBinary` bytes) keep working unchanged.
    fn command_from_action(action: &str, args: &mut Option<semio_framework_value::DslValue>, original_fault: &mut Option<semio_framework::Fault>) -> Result<Self::Command, Fault> {
        let mut original_fault=original_fault;if !matches!(action,"flowEvalResolve"|"flowTessellateResolve"){if let Some(fault)=original_fault.take(){return Err(fault)}}
        let mut args=args.get_or_insert(semio_framework_value::DslValue::Null);
        let str_arg = |keys: &[&str]| -> Option<String> { keys.iter().find_map(|key| args.get(key).and_then(|value| value.as_str()).map(str::to_string)) };
        let f64_arg = |keys: &[&str]| -> Option<f64> { keys.iter().find_map(|key| args.get(key).and_then(|value| value.as_f64())) };
        let u64_arg = |keys: &[&str]| -> Option<u64> { keys.iter().find_map(|key| args.get(key).and_then(|value| value.as_u64().or_else(|| value.as_f64().map(|number| number as u64)))) };
        let pointer_samples = || {
            args.get("samples")
                .and_then(semio_framework_value::DslValue::as_array)
                .map(|items| items.iter().filter_map(|item| { let pair = item.as_array()?; Some([pair.first()?.as_f64()?, pair.get(1)?.as_f64()?]) }).collect::<Vec<[f64; 2]>>())
                .unwrap_or_default()
        };
        let pointer_cancelled = || args.get("cancelled").and_then(semio_framework_value::DslValue::as_bool).unwrap_or(false);
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
            "nodeGraphEdit" => {
                semio_framework_tool_machine::node_graph_edit_rows(&args).map_err(Fault::from)?;
                Ok(Generation2dCommand::NodeGraphEdit(node_graph_edit::NodeGraphEdit { operations_json: args.get("operations").map(semio_framework_pack_json::to_json_string).unwrap_or_else(|| "[]".into()) }))
            }
            "moveMediaNode" => Ok(Generation2dCommand::MoveMediaNode(move_media_node::MoveMediaNode { node_id: str_arg(&["nodeId", "node_id", "id"]).unwrap_or_default(), x: f64_arg(&["x"]).unwrap_or(0.0), y: f64_arg(&["y"]).unwrap_or(0.0) })),
            "addWidget" => Ok(Generation2dCommand::AddWidget(add_widget::AddWidget { kind: str_arg(&["kind"]).unwrap_or_else(|| "inputSlider".into()), neuron_kind: str_arg(&["neuronKind", "neuron_kind"]), format: str_arg(&["format"]), action: str_arg(&["action"]), x: f64_arg(&["x"]), y: f64_arg(&["y"]) })),
            "removeWidget" => Ok(Generation2dCommand::RemoveWidget(remove_widget::RemoveWidget { widget_id: str_arg(&["widgetId", "widget_id", "id"]).unwrap_or_default() })),
            "connectMediaPorts" => Ok(Generation2dCommand::ConnectMediaPorts(connect_media_ports::ConnectMediaPorts {
                source_node_id: str_arg(&["sourceNodeId", "source_node_id"]).unwrap_or_default(),
                source_port_id: str_arg(&["sourcePortId", "source_port_id"]).unwrap_or_default(),
                target_node_id: str_arg(&["targetNodeId", "target_node_id"]).unwrap_or_default(),
                target_port_id: str_arg(&["targetPortId", "target_port_id"]).unwrap_or_default(),
            })),
            "reorganize" => Ok(Generation2dCommand::Reorganize(reorganize::Reorganize {})),
            "addGeneration" => Ok(Generation2dCommand::AddGeneration(add_generation::AddGeneration {})),
            "removeGeneration" => Ok(Generation2dCommand::RemoveGeneration(remove_generation::RemoveGeneration { id: str_arg(&["id"]).unwrap_or_default() })),
            "renameGeneration" => Ok(Generation2dCommand::RenameGeneration(rename_generation::RenameGeneration { id: str_arg(&["id"]).unwrap_or_default(), name: str_arg(&["name"]).unwrap_or_default() })),
            "updateGenerationValues" => {
                let value = args.get("value").map_or(semio_framework_value::DslValue::Null, |entry| entry.clone());
                Ok(Generation2dCommand::UpdateGenerationValues(update_generation_values::UpdateGenerationValues {
                    generation_id: str_arg(&["generationId", "generation_id"]),
                    question_id: str_arg(&["questionId", "question_id"]).unwrap_or_default(),
                    value,
                }))
            }
            "nodeGraphViewport" => Ok(Generation2dCommand::NodeGraphViewport(node_graph_viewport::NodeGraphViewport { viewport: parse_flow_viewport(&args)? })),
            "setShowMode" => Ok(Generation2dCommand::SetShowMode(set_show_mode::SetShowMode { value: str_arg(&["value", "showMode"]).unwrap_or_default() })),
            "setActiveExample" => Ok(Generation2dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: str_arg(&["exampleId", "example_id", "id"]).unwrap_or_default() })),
            "generate" => Ok(Generation2dCommand::Generate(enter_generate::Generate {})),
            "setEvalOutputs" => Ok(Generation2dCommand::SetEvalOutputs(set_eval_outputs::SetEvalOutputs { outputs_json: str_arg(&["outputsJson", "outputs_json", "evalJson"]).unwrap_or_else(|| "{}".into()) })),
            "canvasPointerDown" => Ok(Generation2dCommand::CanvasPointerDown(canvas_pointer_down::CanvasPointerDown {})),
            "canvasPointerMove" => Ok(Generation2dCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove { samples: pointer_samples() })),
            "canvasPointerUp" => Ok(Generation2dCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp { cancelled: pointer_cancelled() })),
            "canvasWheel" => Ok(Generation2dCommand::CanvasWheel(canvas_wheel::CanvasWheel {})),
            "selectGeneration" => Ok(Generation2dCommand::SelectGeneration(select_generation::SelectGeneration { id: str_arg(&["id"]) })),
            "flowEvalTick" => Ok(Generation2dCommand::FlowEvalTick(semio_s_artifact_procedural_2d_preview_eval::FlowEvalTick {
                window_id: str_arg(&["windowId", "window_id"]).unwrap_or_default(),
                window_kind_id: str_arg(&["windowKindId", "window_kind_id"]).unwrap_or_default(),
            })),
            "flowEvalResolve" => {
                let input_valid=original_reply_valid(&args,false,false);
                if !input_valid{return Ok(Generation2dCommand::FlowEvalResolve(semio_s_artifact_procedural_2d_preview_eval::FlowEvalResolve{window_id:String::new(),window_kind_id:String::new(),node_hash:0,extension_id:String::new(),output:semio_framework_value::DslValue::Null,ok:original_fault.is_none(),fault:original_fault.take(),original_input:Some(std::mem::replace(args,semio_framework_value::DslValue::Null)),input_valid}))}
                let node_hash=args.get("nodeHash").and_then(semio_framework_value::DslValue::as_u64).unwrap();
                let window_id=original_reply_text(&mut args,"windowId",true).unwrap();let window_kind_id=original_reply_text(&mut args,"windowKindId",true).unwrap();let extension_id=original_reply_text(&mut args,"extensionId",false).unwrap();let output=original_reply_output(&mut args).unwrap();
                Ok(Generation2dCommand::FlowEvalResolve(semio_s_artifact_procedural_2d_preview_eval::FlowEvalResolve{window_id,window_kind_id,node_hash,extension_id,output,ok:original_fault.is_none(),fault:original_fault.take(),original_input:Some(std::mem::replace(args,semio_framework_value::DslValue::Null)),input_valid}))
            },
            "setContributions" => Ok(Generation2dCommand::SetContributions(set_contributions::SetContributions {
                json: str_arg(&["json"]).unwrap_or_default(),
                page: u64_arg(&["page"]).unwrap_or_default(),
                page_count: u64_arg(&["pageCount", "page_count"]).unwrap_or(1),
            })),
            other => Err(Fault::from(format!(
                "action '{other}' is not a framework-reserved action (history/clipboard/revert/filter/noteShellCommand) — \
                 app actions are dispatched exclusively through the typed command channel now (see `dispatch_typed_command`)"
            ))),
        }
    }

    /// 🕹️ Every command runs its generated dispatch over a scratch session: no generation2d verb reads an ambient
    /// selection (`nodeGraphEdit` rows name every entity they edit, design §13.3).
    fn handle(
        command: &Generation2dCommand,
        doc: &ArtifactView<'_, Generation2dSnapshot>,
        cfg: &ConfigView<'_, Generation2dConfig>,
        _interaction: &InteractionView<'_>,
        _view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<Generation2dMutation, Generation2dConfigMutation, Self::DraftMutation>, Fault> {
        let mut session = FlowEvalSession::new();
        let result = command.dispatch(doc, cfg, &mut session);
        close_flow_session(&mut session);
        result
    }

    /// 🕹️ `graph`'s `HierarchyProvider::Topology` — every top-level widget is a "node" (root unless
    /// nested in a `Widget::Cluster`'s own `tree.neurons`, where each nested `Neuron` becomes a "node"
    /// parented to its owning cluster's widget id — the DAG-parent-links transitive-hover source: hovering
    /// a Cluster's own tree item transitively covers every widget nested inside it). Synapses become
    /// "edge" targets, parented to nothing (edges are leaves, not containers).
    fn interaction_topology(doc: &ArtifactView<'_, Generation2dSnapshot>, _cfg: &ConfigView<'_, Generation2dConfig>) -> Result<InteractionTopology, semio_framework_value::ValueError> {
 Ok((||{
        fn walk_neuron(neuron: &semio_framework_artifact_flow_flow::neural::Neuron, parent: String, ordered: &mut Vec<TopologyNode>) {
            ordered.push(TopologyNode { id: neuron.id.clone(), granularity: "node".into(), parent: Some(parent) });
            if let Some(tree) = &neuron.tree {
                for child in &tree.neurons {
                    walk_neuron(child, neuron.id.clone(), ordered);
                }
            }
        }
        let fixture = &doc.snapshot.host_snapshot;
        let mut ordered = Vec::new();
        for widget in &fixture.widgets {
            let id = semio_s_artifact_procedural_2d_model::widget_id(widget).to_string();
            ordered.push(TopologyNode { id: id.clone(), granularity: "node".into(), parent: None });
            if let semio_framework_artifact_flow_flow::Widget::Cluster { tree, .. } = widget {
                for child in &tree.neurons {
                    walk_neuron(child, id.clone(), &mut ordered);
                }
            }
        }
        for synapse in &fixture.synapses {
            ordered.push(TopologyNode { id: synapse.id.clone(), granularity: "edge".into(), parent: None });
        }
        let mut domains = std::collections::BTreeMap::new();
        domains.insert("graph".to_string(), DomainTopology { ordered });
        InteractionTopology { domains }
    
})())
}

    /// ⏯️ Starts, finalizes or wakes the `previewEval` run for the attached preview windows
    /// ([`crate::preview_eval::preview_eval_run_effects`]), and owes every window whose target moved
    /// since it last evaluated — an undo, a redo or a remote edit lands with no local gesture to owe it.
    /// Before the first `Event::SurfaceVisible` the host has no roster (`view` is `None`), so nothing
    /// starts; a graph the live registry cannot serve starts nothing either, because only
    /// `setContributions` can move it.
    fn pending_effects(owner: &semio_framework_plugin::ArtifactInstanceOperationOwnerHandle, doc: &ArtifactView<'_, Generation2dSnapshot>, cfg: &ConfigView<'_, Generation2dConfig>, view: Option<&semio_framework_plugin::ViewModel>) -> Vec<Effect> {
        let windows = generation2d_preview_windows(view);
        let servable = semio_s_artifact_procedural_2d_preview_eval::may_rearm(&doc.snapshot.host_snapshot);
        let mut targets: Vec<PreviewEvalTarget> = windows.iter().map(|(_, _, target)| *target).collect();
        targets.sort_unstable();
        targets.dedup();
        let current: std::collections::BTreeMap<PreviewEvalTarget, u64> = targets.into_iter().map(|target| (target, generation2d_preview_digest(target, doc, cfg, doc.provisional_generation()))).collect();
        owner
            .with_mut::<Generation2dInstanceOperationOwner, _>(|owner| {
                let (sessions, link) = owner.parts()?;
                Ok(semio_s_artifact_procedural_2d_preview_eval::preview_eval_run_effects(sessions, link, &windows, &current, doc.tool_run(), servable))
            })
            .unwrap_or_default()
    }

    /// ⏯️ The `previewEval` run job: its hops read the retained sessions this instance's commands fold into.
    fn build_tool_run_job(request: ToolRunJobRequest<'_, EditorApp<Self>>) -> Result<Option<ToolRunJob>, Fault> {
        if request.tool_id != semio_s_artifact_procedural_2d_preview_eval::PREVIEW_EVAL_TOOL_ID || request.purpose != ToolRunJobPurpose::Run {
            return Ok(None);
        }
        Ok(Some(Box::new(semio_s_artifact_procedural_2d_preview_eval::PreviewEvalRunJob::<Generation2dInstanceOperationOwner>::new(request.instance_owner, request.port, request.identity)?)))
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Generation2dSnapshot>, cfg: &ConfigView<'_, Generation2dConfig>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let mut session = FlowEvalSession::new();
        let rendered = generation2d_render_body(body_key, doc, cfg, view_state, &GraphInteractionMarks::default(), &session);
        close_flow_session(&mut session);
        rendered
    }

    /// 🕹️ Resolves the live `graph` hover/selection once per render and threads it into every
    /// window body — the node graph paints the selected nodes.
    fn render_with_request_context(
        owner: &semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
        body_key: &str,
        doc: &ArtifactView<'_, Generation2dSnapshot>,
        cfg: &ConfigView<'_, Generation2dConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        transient: &semio_framework_plugin::TransientView<'_, Self::Transient>,
        interaction: &InteractionView<'_>,
        cx:&mut semio_framework_plugin::StepContext<'_>,
    ) -> semio_framework_plugin::OriginalRenderResult<semio_framework_plugin::ComponentTree> {
        let rendered:semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree>=(||{
        if body_key == generate_preview::GENERATION2D_PLAY_BODY_GENERATE_PREVIEW {
            let labels = generation2d_labels(view_state);
            let window_config = generate_preview::config::current(cfg);
            let node = generate_preview::render(&window_config, transient.snapshot.generation_preview_text.as_deref(), labels)?;
            return Ok(semio_framework_plugin::built_to_component_tree(node));
        }
        let marks = GraphInteractionMarks::from_interaction(interaction);
        owner
            .with_mut::<Generation2dInstanceOperationOwner, _>(|owner| owner.with_session(|session| generation2d_render_body(body_key, doc, cfg, view_state, &marks, session)))
            .map_err(|error| semio_framework_plugin::PluginAssemblyError::new("generation2d.eval-session-owner", error.message))?

        })();
        rendered.map(Some).map_err(Into::into)
        }
    /// 🗂️ The marks-free entry point the framework still offers — every live right-click goes
    /// through `context_menu_with_request_context` instead, so this delegate builds the same menu
    /// against an empty selection.
    fn context_menu(
        request: &semio_framework_plugin::ContextMenuRequest,
        doc: &ArtifactView<'_, Generation2dSnapshot>,
        cfg: &ConfigView<'_, Generation2dConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        registry: &semio_framework_plugin::AppActionRegistry,
    ) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {
        Self::context_menu_body(request, doc, cfg, view_state, &GraphInteractionMarks::default(), registry)
    }

    /// 🕹️ `context_menu`'s interaction-aware twin — the one the runtime actually calls.
    /// `interaction` is the authoritative framework-owned `graph` selection, the same one
    /// `render_with_request_context` paints from.
    fn context_menu_with_request_context(
        request: &semio_framework_plugin::ContextMenuRequest,
        doc: &ArtifactView<'_, Generation2dSnapshot>,
        cfg: &ConfigView<'_, Generation2dConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        interaction: &InteractionView<'_>,
        registry: &semio_framework_plugin::AppActionRegistry,
    ) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {
        Self::context_menu_body(request, doc, cfg, view_state, &GraphInteractionMarks::from_interaction(interaction), registry)
    }

    /// 🎞️ Declares `export_media`'s default document schema — pack-encodes `doc.snapshot`, wrapped
    /// `Structured{schema: Self::DOCUMENT_SCHEMA, json: base64}` — plus `"drawing:out"`.
    fn export_media(port: &str, doc: &ArtifactView<'_, Generation2dSnapshot>) -> Result<semio_framework_plugin::Media, semio_framework_plugin::MediaError> {
        match port {
            "drawing:out" => {
                let eval_json = semio_s_artifact_procedural_2d_io_text::snapshot::evaluate_generation_preview(&doc.snapshot.host_snapshot, &semio_framework_artifact_playbook_playbook::PlaybookValues::new());
                let layers_json = semio_s_artifact_procedural_2d_io_text::snapshot::generation_output_layers(&doc.snapshot.host_snapshot, &eval_json);
                Ok(semio_framework_plugin::Media { media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Vector }, payload: semio_framework_plugin::MediaPayload::Structured { schema: "2d.drawing".into(), json: layers_json } })
            }
            "artifact:out" => {
                let bytes = pack::snapshot::ArtifactPack::encode_pack(doc.snapshot);
                Ok(semio_framework_plugin::Media {
                    media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Flow },
                    payload: semio_framework_plugin::MediaPayload::Structured { schema: Self::DOCUMENT_SCHEMA.to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) },
                })
            }
            _ => Err(semio_framework_plugin::MediaError::NotImplemented),
        }
    }

    /// 🎞️ `"params:in"`: a generic Data×Value JSON object `{widgetId: number}` — patches matching
    /// `InputSlider` widgets' `value` field, leaving unmatched keys/widget kinds untouched.
    fn import_media(port: &str, media: &semio_framework_plugin::Media, doc: &ArtifactView<'_, Generation2dSnapshot>) -> Result<Emit<Generation2dMutation, Generation2dConfigMutation, Self::DraftMutation>, semio_framework_plugin::MediaError> {
        if port != "params:in" {
            return Err(semio_framework_plugin::MediaError::NotImplemented);
        }
        let semio_framework_plugin::MediaPayload::Structured { json, .. } = &media.payload else {
            return Err(semio_framework_plugin::MediaError::Payload(port.to_string(), "params:in expects a Structured JSON object payload".into()));
        };
        let parsed = semio_framework_pack_json::parse(json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| semio_framework_plugin::MediaError::Payload(port.to_string(), error.to_string()))?;
        let Some(object) = parsed.as_object() else {
            return Err(semio_framework_plugin::MediaError::Payload(port.to_string(), "params:in payload must be a JSON object".into()));
        };
        let mut operations = Vec::new();
        for (widget_id_key, value) in object.iter() {
            let Some(number) = value.as_f64() else { continue };
            let Some(widget) = doc.snapshot.host_snapshot.widgets.iter().find(|widget| semio_s_artifact_procedural_2d_model::widget_id(widget) == widget_id_key) else { continue };
            if let semio_framework_artifact_flow_flow::Widget::InputSlider { id, label, min, max, step, .. } = widget {
                operations.push(semio_s_artifact_procedural_2d_mut_widgets::replace_widget(semio_framework_artifact_flow_flow::Widget::InputSlider {
                    id: id.clone(),
                    label: label.clone(),
                    value: number,
                    min: *min,
                    max: *max,
                    step: *step,
                }));
            }
        }
        Ok(Emit::mutations(operations))
    }
}

impl Generation2dPlayApp {
    /// 🗂️ The ONE context-menu implementation — both `ArtifactEditor::context_menu` and
    /// `context_menu_with_request_context` funnel here.
    fn context_menu_body(
        request: &semio_framework_plugin::ContextMenuRequest,
        doc: &ArtifactView<'_, Generation2dSnapshot>,
        _cfg: &ConfigView<'_, Generation2dConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        marks: &GraphInteractionMarks,
        registry: &semio_framework_plugin::AppActionRegistry,
    ) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {
        use semio_framework_plugin::{node_graph_delete_selection_spec, selection_domains_from_surface, Menu, NodeGraphDeleteDispatch};
        let labels = semio_framework_plugin::resolve_labels::<Generation2dLabels>(view_state);
        let (selected_nodes, selected_edges) = marks.graph_selection_domains(&doc.snapshot.host_snapshot);
        let (nodes, edges) = selection_domains_from_surface(request.surface.as_ref(), &selected_nodes, &selected_edges);
        let mut menu = Menu::of(registry, view_state).action("addWidget").action("reorganize").action("generate");
        menu = menu.group("mode", |m| m.action("setShowMode"));
        menu = menu.group("create", |m| m.action("addGeneration"));
        menu = menu.group("methods", |m| m.action("selectGeneration"));
        menu.item(node_graph_delete_selection_spec(labels.delete_selection.as_str(), view_state, &nodes, &edges, NodeGraphDeleteDispatch::ViaNodeGraphEdit)).build()
    }
}
//#endregion 🔖️Generation2dPlayApp
