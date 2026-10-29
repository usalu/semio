//! 🔭️ Viewer and interaction topology: the node port ids, the viewer struct, its artifact viewer implementation and the render body. See [`super`].

use super::*;

//#region 🔖️InteractionTopology
/// 🕸️ Every node's visible port ids (`{nodeId}@{portId}`) — read-only twin of the sibling surface's
/// own projection, so a world pick in this viewer names the exact same declared target ids.
fn generation3d_view_port_ids_by_node(host_snapshot: &semio_framework_artifact_flow_flow::FlowHostSnapshot) -> std::collections::BTreeMap<String, Vec<String>> {
    let (graph_nodes, _) = semio_s_artifact_procedural_3d_model::with_host(host_snapshot, |host| semio_s_artifact_procedural_3d_preview_eval::dag_host_snapshot_to_workflow(&host.dag.host_snapshot));
    graph_nodes.into_iter().map(|node| (node.id, node.inputs.into_iter().chain(node.outputs).map(|port| port.id).collect())).collect()
}
//#endregion 🔖️InteractionTopology

//#region 🔖️Viewer
#[derive(Default, Clone, Copy)]
pub struct Generation3dViewer;

impl semio_framework_plugin::ArtifactViewer for Generation3dViewer {
    /// 📣️ The localized notices of the viewer's refusal codes (design §20.12): the document-level ones.
    fn fault_notices() -> &'static [(&'static str, LocalizedLabel)] {
        semio_s_artifact_procedural_3d_model::generation3d_document_fault_notices()
    }

    /// 🧩️ The loaded-parent child projection every archive load and maintenance swap asks for before a
    /// decoded document may replace the store. `Generation3dSnapshot` declares no child slot, so the
    /// projection is honestly empty; without it every replacement faulted with `viewer did not declare
    /// a loaded-parent child projection`.
    fn child_restore_projection(snapshot: &Self::Snapshot) -> Result<semio_framework_composition_admission::ChildRestoreProjection<'_>, Fault> {
        semio_framework_composition_admission::ChildRestoreProjection::from_snapshot(snapshot).map_err(|error| Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new(semio_s_artifact_procedural_3d_model::GENERATION3D_CHILD_PROJECTION), error.to_string()))
    }

    type Snapshot = Generation3dSnapshot;
    type Mutation = Generation3dMutation;
    type Config = Generation3dViewConfig;
    type ConfigMutation = Generation3dViewConfigMutation;
    type Presence = Generation3dViewPresence;
    type PresenceMutation = Generation3dViewPresenceMutation;
    type Transient = Generation3dViewTransient;
    type TransientMutation = Generation3dViewTransientMutation;
    type Command = Generation3dViewCommand;

    const DIALECT: Dialect = GENERATION3D_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = GENERATION_3D_SCHEMA;

    fn build_transient_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactEphemeralOneItemPreparationFactory<Self::Transient, Self::TransientMutation>>> {
        Some(semio_framework_plugin::bounded_transient_preparation_factory::<Self::Transient, Self::TransientMutation>())
    }

    /// 🎛️ The viewer's `setCamera`/`setShowMode`/`setLodMode`/sun tools all publish onto the CONFIG
    /// lane; without this authority every one of them fails closed with
    /// `interactive-job.publication-authority-missing` (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    fn build_config_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Config, Self::ConfigMutation>>> {
        Some(store::snapshot_clone_preparation::config_apply_preparation_factory::<Self::Config, Self::ConfigMutation>())
    }

    fn build_presence_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactEphemeralOneItemPreparationFactory<Self::Presence, Self::PresenceMutation>>> {
        Some(semio_framework_plugin::bounded_transient_preparation_factory::<Self::Presence, Self::PresenceMutation>())
    }

    fn build_transient_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Transient>>> {
        Some(semio_framework_plugin::bounded_transient_root_retirement_factory::<Self::Transient>())
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, ViewerApp<Self>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(Generation3dViewBoundedCommandJobFactory::new(&controller))?;
        registry.register(Generation3dViewContributionsJobFactory::new(&controller))?;
        registry.register(Generation3dViewExampleJobFactory::new(&controller))?;
        registry.register(Generation3dViewFlowEvalJobFactory::new(&controller))?;
        registry.register(Generation3dViewDocumentIoJobFactory::new(&controller))
    }

    /// 🧠️ One retained `FlowEvalSession` per viewer instance — see
    /// [`Generation3dViewInstanceOperationOwner`]. Without it every chain hop would fold its answer
    /// into a session that died with the command that built it.
    fn build_instance_operation_owner() -> Box<dyn semio_framework_plugin::ArtifactInstanceOperationOwner> {
        Box::new(Generation3dViewInstanceOperationOwner::new())
    }

    /// 🪟️ The viewer preview window's retained evaluation publication.
    fn register_window_transient_owners(registry: &mut semio_framework_plugin::WindowTransientOwnerRegistry) -> Result<(), Fault> {
        registry.register::<preview::transient::Generation3dViewPreviewWindowTransientOwner>()
    }

    /// 🎯️ `flowEvalTick` publishes the evaluation into ONE preview window's retained transient, and
    /// the whole chain is self-dispatched: an `Effect::DispatchAction` reaches the shell with no
    /// window of its own and is redispatched under whichever window is current. Naming the target
    /// off the PAYLOAD makes the runtime validate it against the trusted ViewModel roster and
    /// capture that exact window's mutation authority (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    /// 🪟️ Every route of the preview chain that may publish — the dispatched hop and both
    /// window-addressed folds, which now continue the chain inline and therefore publish the
    /// intermediate geometry and the advanced status pill the hop used to.
    fn retained_window_transient_target(command: &Self::Command) -> Option<(&str, &'static str)> {
        let (window_id, window_kind_id) = generation3d_view_flow_eval_window_address(command)?;
        if window_id.is_empty() {
            return None;
        }
        preview_eval::preview_kind(window_kind_id, GENERATION3D_VIEW_PREVIEW_KINDS).map(|kind| (window_id, kind))
    }

    /// ⏯️ Starts, finalizes or wakes the `previewEval` run for the attached preview window
    /// ([`preview_eval::preview_eval_run_effects`]). Before the first `Event::SurfaceVisible` the host
    /// has no roster (`view` is `None`) and there is nothing to publish into, so nothing starts.
    ///
    /// 📚️ Servability is asked of the graph the run will actually evaluate — the VIEWED document — not
    /// of the one this session happens to have open. They are different documents the moment the
    /// navbar names an example (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    fn pending_effects(owner: &semio_framework_plugin::ArtifactInstanceOperationOwnerHandle, doc: &ArtifactView<'_, Self::Snapshot>, cfg: &ConfigView<'_, Self::Config>, view: Option<&semio_framework_plugin::ViewModel>) -> Vec<semio_framework_plugin::Effect> {
        let windows = generation3d_view_preview_windows(view);
        let viewed = Generation3dViewedDocument::resolve(doc.snapshot, cfg.snapshot);
        let servable = preview_eval::may_rearm(&viewed.snapshot().host_snapshot);
        viewed.retire();
        let applied_edits = preview_eval::applied_document_edits_digest(doc.history);
        owner
            .with_mut::<Generation3dViewInstanceOperationOwner, _>(|owner| {
                use preview_eval::PreviewEvalRunOwner as _;
                let (session, link) = owner.preview_eval_parts().ok_or_else(|| Fault::from("generation3d-view-eval-session-closing"))?;
                Ok(preview_eval::preview_eval_run_effects(session, link, &windows, doc.tool_run(), servable, applied_edits))
            })
            .unwrap_or_default()
    }

    /// ⏯️ The `previewEval` run job over the VIEWED document of the run's base and settings.
    fn build_tool_run_job(request: ToolRunJobRequest<'_, ViewerApp<Self>>) -> Result<Option<ToolRunJob>, Fault> {
        if request.tool_id != preview_eval::PREVIEW_EVAL_TOOL_ID || request.purpose != ToolRunJobPurpose::Run {
            return Ok(None);
        }
        let viewed = Generation3dViewedDocument::resolve(&request.snapshot, &request.config);
        let preview_widget_ids = preview_eval::preview_widget_ids(&viewed.snapshot().host_snapshot);
        viewed.retire();
        Ok(Some(Box::new(preview_eval::PreviewEvalRunJob::<Generation3dViewInstanceOperationOwner>::new(request.instance_owner, request.port, request.identity, preview_widget_ids)?)))
    }

    fn build_tool_job(request: &mut Option<ArtifactOwnedToolJobRequest<ViewerApp<Self>>>, output: &mut Option<semio_framework_plugin::ErasedToolJob>, cx: &mut semio_framework_plugin::__job::StepContext<'_>) -> Result<bool, semio_framework_plugin::__job::OriginalTurnFailure> {
        let original = request.as_ref().ok_or_else(|| semio_framework_plugin::__value::ValueError::literal(semio_framework_plugin::__value::ValueRefusalKind::InvariantViolated, "original retained producer request is absent"))?;
        if !GENERATION3D_VIEW_TOOL_IDS.contains(&original.tool_id.as_str())
            && !GENERATION3D_VIEW_CONTRIBUTIONS_TOOL_IDS.contains(&original.tool_id.as_str())
            && !GENERATION3D_VIEW_EXAMPLE_TOOL_IDS.contains(&original.tool_id.as_str())
            && !GENERATION3D_VIEW_FLOW_EVAL_TOOL_IDS.contains(&original.tool_id.as_str())
            && !GENERATION3D_VIEW_DOCUMENT_IO_TOOL_IDS.contains(&original.tool_id.as_str())
        {
            return Ok(false);
        }
        if original.command.command_id() != original.tool_id {
            return Err(semio_framework_plugin::__value::ValueError::literal(semio_framework_plugin::__value::ValueRefusalKind::InvariantViolated, "the generation 3d view command does not match its exact registered tool").into());
        }
        let tool_id = original.command.command_id();
        let contributions = GENERATION3D_VIEW_CONTRIBUTIONS_TOOL_IDS.contains(&tool_id);
        let example = GENERATION3D_VIEW_EXAMPLE_TOOL_IDS.contains(&tool_id);
        let flow_eval = GENERATION3D_VIEW_FLOW_EVAL_TOOL_IDS.contains(&tool_id);
        semio_framework_plugin::retained_command::producer::admit_retained_tool_producer(request, output, cx, Generation3dViewCommand::command_id, if contributions {
                GENERATION3D_VIEW_CONTRIBUTIONS_RAW_BYTES
            } else if flow_eval {
                GENERATION3D_VIEW_FLOW_EVAL_RAW_BYTES
            } else if example {
                GENERATION3D_VIEW_EXAMPLE_RAW_BYTES
            } else if GENERATION3D_VIEW_DOCUMENT_IO_TOOL_IDS.contains(&tool_id) {
                GENERATION3D_VIEW_DOCUMENT_IO_RAW_BYTES
            } else {
                GENERATION3D_VIEW_RAW_BYTES
            }, GENERATION3D_VIEW_WORK_ITEMS, false, |request| -> Box<dyn ArtifactCommandWork<ViewerApp<Generation3dViewer>>> { if GENERATION3D_VIEW_DOCUMENT_IO_TOOL_IDS.contains(&tool_id) {
            Box::new(Generation3dViewDocumentIoWork { instance_owner: request.instance_operation_owner.clone(), consumed: false })
        } else if contributions {
            Box::new(Generation3dViewContributionsWork { instance_owner: request.instance_operation_owner.clone(), consumed: false })
        } else if GENERATION3D_VIEW_FLOW_EVAL_WINDOW_TOOL_IDS.contains(&tool_id) {
            Box::new(Generation3dViewFlowEvalWindowWork::new(tool_id, request.instance_operation_owner.clone()))
        } else if flow_eval {
            Box::new(Generation3dViewFlowResolveWork { tool_id, instance_owner: request.instance_operation_owner.clone(), consumed: false })
        } else {
            Box::new(Generation3dViewCommandWork::new(tool_id, request.instance_operation_owner.clone()))
        } })
    }

    /// 🧾️ ALL THREE factories' proofs, in registration order. The bounded rows read their contract
    /// off `generation3d_view_bounded_contract()` instead of respelling `8_192, 32, 32, …` seven
    /// times — one declared quantity, one place.
    fn bounded_first_step_tool_proofs() -> Vec<semio_framework_plugin::ArtifactBoundedFirstStepProof> {
        let mut proofs = Generation3dViewBoundedCommandJobFactoryProofs::bounded_first_step_tool_proofs();
        proofs.extend(Generation3dViewContributionsJobFactoryProofs::bounded_first_step_tool_proofs());
        proofs.extend(Generation3dViewExampleJobFactoryProofs::bounded_first_step_tool_proofs());
        proofs.extend(Generation3dViewFlowEvalJobFactoryProofs::bounded_first_step_tool_proofs());
        proofs.extend(Generation3dViewDocumentIoJobFactoryProofs::bounded_first_step_tool_proofs());
        proofs
    }

    /// 🎯️ Maps the host's declared command id + JSON args onto this viewer's typed command — the
    /// bridge `PluginApp::handle_command` takes for EVERY structurally addressed invocation
    /// (`AppCommand::Command` → `A::command_from_action`). Without it the shell's own
    /// `setContributions` push, and every chrome measure's `on_change`, fail closed with
    /// `app.command.unsupported` (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    fn command_from_action(action: &str, args: &mut Option<semio_framework_value::DslValue>, original_fault: &mut Option<semio_framework::Fault>) -> Result<Self::Command, Fault> {
        let mut original_fault=original_fault;if !matches!(action,"flowEvalResolve"|"flowTessellateResolve"|"flowTessellateCancelResolve"){if let Some(fault)=original_fault.take(){return Err(fault)}}
        let mut args = args.get_or_insert(semio_framework_value::DslValue::Null);
        let str_arg = |keys: &[&str]| -> Option<String> { keys.iter().find_map(|key| args.get(key).and_then(|value| value.as_str()).map(str::to_string)) };
        let f64_arg = |keys: &[&str]| -> Option<f64> { keys.iter().find_map(|key| args.get(key).and_then(semio_framework_value::DslValue::as_f64)) };
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
            "setShowMode" => Ok(Generation3dViewCommand::SetShowMode(set_show_mode::SetShowMode { value: str_arg(&["value", "showMode", "show_mode"]).unwrap_or_default() })),
            "setLodMode" => Ok(Generation3dViewCommand::SetLodMode(set_lod_mode::SetLodMode { value: str_arg(&["value", "lodMode", "lod_mode"]).unwrap_or_default() })),
            "setCamera" => Ok(Generation3dViewCommand::SetCamera(set_camera::SetCamera {
                camera: args
                    .get("camera")
                    .and_then(|camera| <semio_s_artifact_procedural_3d_viewer_config::Generation3dViewCamera as semio_framework_value::FromValue>::from_value(camera.clone()).ok())
                    .unwrap_or_default(),
            })),
            "toggleSun" => Ok(Generation3dViewCommand::ToggleSun(toggle_sun::ToggleSun {})),
            "setActiveExample" => Ok(Generation3dViewCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: str_arg(&["exampleId", "example_id", "value"]).unwrap_or_default() })),
            "setSunAzimuth" => Ok(Generation3dViewCommand::SetSunAzimuth(set_sun_azimuth::SetSunAzimuth { value: f64_arg(&["value"]).unwrap_or(0.0) })),
            "setSunElevation" => Ok(Generation3dViewCommand::SetSunElevation(set_sun_elevation::SetSunElevation { value: f64_arg(&["value"]).unwrap_or(0.0) })),
            "setSunIntensity" => Ok(Generation3dViewCommand::SetSunIntensity(set_sun_intensity::SetSunIntensity { value: f64_arg(&["value"]).unwrap_or(1.0) })),
            "setContributions" => Ok(Generation3dViewCommand::SetContributions(set_contributions::SetContributions {
                json: str_arg(&["json"]).unwrap_or_default(),
                page: u64_arg(&["page"]).unwrap_or_default(),
                page_count: u64_arg(&["pageCount", "page_count"]).unwrap_or(1),
            })),
            "flowEvalTick" => Ok(Generation3dViewCommand::FlowEvalTick(semio_s_artifact_procedural_3d_preview_eval::FlowEvalTick {
                window_id: str_arg(&["windowId", "window_id"]).unwrap_or_default(),
                window_kind_id: str_arg(&["windowKindId", "window_kind_id"]).unwrap_or_default(),
            })),
            "flowEvalResolve" => {
                let input_valid=original_reply_valid(&args,false,false);
                if !input_valid{return Ok(Generation3dViewCommand::FlowEvalResolve(semio_s_artifact_procedural_3d_preview_eval::FlowEvalResolve{window_id:String::new(),window_kind_id:String::new(),node_hash:0,extension_id:String::new(),output:semio_framework_value::DslValue::Null,ok:original_fault.is_none(),fault:original_fault.take(),original_input:Some(std::mem::replace(args,semio_framework_value::DslValue::Null)),input_valid}))}
                let node_hash=args.get("nodeHash").and_then(semio_framework_value::DslValue::as_u64).unwrap();
                let window_id=original_reply_text(&mut args,"windowId",true).unwrap();let window_kind_id=original_reply_text(&mut args,"windowKindId",true).unwrap();let extension_id=original_reply_text(&mut args,"extensionId",false).unwrap();let output=original_reply_output(&mut args).unwrap();
                Ok(Generation3dViewCommand::FlowEvalResolve(semio_s_artifact_procedural_3d_preview_eval::FlowEvalResolve{window_id,window_kind_id,node_hash,extension_id,output,ok:original_fault.is_none(),fault:original_fault.take(),original_input:Some(std::mem::replace(args,semio_framework_value::DslValue::Null)),input_valid}))
            },
            "flowTessellateResolve" => {
                let input_valid=original_reply_valid(&args,true,false);
                if !input_valid{return Ok(Generation3dViewCommand::FlowTessellateResolve(semio_s_artifact_procedural_3d_preview_eval::FlowTessellateResolve{window_id:String::new(),window_kind_id:String::new(),node_hash:0,handle:String::new(),tolerance:0.0,output:semio_framework_value::DslValue::Null,fault:original_fault.take(),original_input:Some(std::mem::replace(args,semio_framework_value::DslValue::Null)),input_valid}))}
                let node_hash=args.get("nodeHash").and_then(semio_framework_value::DslValue::as_u64).unwrap();let tolerance=args.get("tolerance").and_then(|value|value.as_f64()).unwrap();
                let window_id=original_reply_text(&mut args,"windowId",true).unwrap();let window_kind_id=original_reply_text(&mut args,"windowKindId",true).unwrap();let handle=original_reply_text(&mut args,"handle",true).unwrap();let output=original_reply_output(&mut args).unwrap();
                Ok(Generation3dViewCommand::FlowTessellateResolve(semio_s_artifact_procedural_3d_preview_eval::FlowTessellateResolve{window_id,window_kind_id,node_hash,handle,tolerance,output,fault:original_fault.take(),original_input:Some(std::mem::replace(args,semio_framework_value::DslValue::Null)),input_valid}))
            },
            "flowEvalRelease" => Ok(Generation3dViewCommand::FlowEvalRelease(semio_s_artifact_procedural_3d_preview_eval::FlowEvalRelease {
                window_id: str_arg(&["windowId", "window_id"]).unwrap_or_default(),
                window_kind_id: str_arg(&["windowKindId", "window_kind_id"]).unwrap_or_default(),
            })),
            "flowTessellateCancelResolve"=>{let input_valid=original_reply_valid(&args,false,true);if !input_valid{return Ok(Generation3dViewCommand::FlowTessellateCancelResolve(semio_s_artifact_procedural_3d_preview_eval::FlowTessellateCancelResolve{window_id:String::new(),window_kind_id:String::new(),output:semio_framework_value::DslValue::Null,ok:original_fault.is_none(),fault:original_fault.take(),original_input:Some(std::mem::replace(args,semio_framework_value::DslValue::Null)),input_valid}))}let window_id=original_reply_text(&mut args,"windowId",true).unwrap();let window_kind_id=original_reply_text(&mut args,"windowKindId",true).unwrap();let output=original_reply_output(&mut args).unwrap();Ok(Generation3dViewCommand::FlowTessellateCancelResolve(semio_s_artifact_procedural_3d_preview_eval::FlowTessellateCancelResolve{window_id,window_kind_id,output,ok:original_fault.is_none(),fault:original_fault.take(),original_input:Some(std::mem::replace(args,semio_framework_value::DslValue::Null)),input_valid}))},
            "exportDocument" => Ok(Generation3dViewCommand::ExportDocument(export_document::ExportDocument { format: str_arg(&["format", "value"]).unwrap_or_else(|| "stl".into()) })),
            other => Err(Fault::from(format!("action '{other}' is not declared by the generation3d viewer"))),
        }
    }

    /// 🏷️ The declared tool id of the command, NOT the trait's generic `"typed-command"` default:
    /// `qualified_tool_proof` looks the bounded first-step proof up by this verb, so leaving the
    /// default in place makes EVERY viewer action fail closed with `interactive-job.missing-factory`.
    fn command_id(command: &Generation3dViewCommand) -> &'static str {
        command.command_id()
    }

    fn reference_birth()->Option<fn()->Result<Generation3dSnapshot,semio_framework_plugin::Fault>>{Some(||{
        Ok(semio_s_artifact_procedural_3d_io_text::snapshot::default_snapshot())
    })}

    /// 👁️ Structurally read-only: `dispatch` returns `ViewEmit`, so this signature cannot widen into
    /// a document write no matter what a payload module does.
    fn handle(
        command: &Self::Command,
        doc: &ArtifactView<'_, Self::Snapshot>,
        cfg: &ConfigView<'_, Self::Config>,
        _interaction: &InteractionView<'_>,
        _view_state: Option<&semio_framework_plugin::ViewModel>,
        _engines: &EngineHandles,
    ) -> Result<ViewEmit<Self::ConfigMutation>, Fault> {
        command.dispatch(doc, cfg)
    }

    /// 🕸️ The same `graph` topology the sibling surface declares — every top-level widget is a
    /// `node`, each of its visible ports a `handle` parented to it, each synapse an `edge`, and each
    /// `Cluster`'s nested neurons `node`s parented to their cluster (the transitive-hover source).
    /// Without this a world pick would be pruned by `validate_state` and nothing would ever light up.
    fn interaction_topology(doc: &ArtifactView<'_, Self::Snapshot>, cfg: &ConfigView<'_, Self::Config>) -> Result<InteractionTopology, semio_framework_value::ValueError> {
 Ok((||{
        fn walk_neuron(neuron: &semio_framework_artifact_flow_flow::neural::Neuron, parent: String, ordered: &mut Vec<TopologyNode>) {
            ordered.push(TopologyNode { id: neuron.id.clone(), granularity: "node".into(), parent: Some(parent) });
            if let Some(tree) = &neuron.tree {
                for child in &tree.neurons {
                    walk_neuron(child, neuron.id.clone(), ordered);
                }
            }
        }
        let viewed = Generation3dViewedDocument::resolve(doc.snapshot, cfg.snapshot);
        let host_snapshot = &viewed.snapshot().host_snapshot;
        let mut ordered = Vec::new();
        let ports_by_node = generation3d_view_port_ids_by_node(host_snapshot);
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
        viewed.retire();
        InteractionTopology { domains }
    
})())
}

    /// 🕹️ The marks-free entry point the framework still offers (no owner, no transient, no
    /// interaction) — every live window goes through `render_with_request_context` instead.
    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, cfg: &ConfigView<'_, Self::Config>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let viewed = Generation3dViewedDocument::resolve(doc.snapshot, cfg.snapshot);
        let tree = generation3d_view_render_body(body_key, viewed.snapshot(), cfg.snapshot, None, None, doc.tool_run(), &preview::Generation3dViewMarks::default(), semio_s_artifact_procedural_3d_terminology::generation3d_labels(view_state));
        viewed.retire();
        tree
    }

    /// 🕹️ Resolves the live `graph` hover/selection once per render and threads it into the preview
    /// body, alongside the evaluation the ADDRESSED chain published into this preview window's own
    /// transient and the retained session holding the meshes `flowTessellateResolve` folded in.
    ///
    /// 🪟️ The window-scoped publication is read first and the app-level lane is the fallback: the
    /// chain writes the window's own transient (that is what `retained_window_transient_target`
    /// captures authority for), and a surface rendered before any tick landed still has to paint
    /// its empty world rather than evaluate itself.
    fn render_with_request_context(
        owner: &semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
        body_key: &str,
        doc: &ArtifactView<'_, Self::Snapshot>,
        cfg: &ConfigView<'_, Self::Config>,
        view_state: &semio_framework_plugin::ViewModel,
        transient: &semio_framework_plugin::TransientView<'_, Self::Transient>,
        interaction: &InteractionView<'_>,
        _cx: &mut semio_framework_job::StepContext<'_>,
    ) -> semio_framework_plugin::OriginalRenderResult<semio_framework_plugin::ComponentTree> {
        let preview_eval_text = transient
            .window::<preview::transient::Generation3dViewPreviewWindowTransientOwner>()
            .and_then(|state| state.preview_eval_text.as_deref())
            .or(transient.snapshot.preview_eval_text.as_deref());
        let marks = preview::Generation3dViewMarks::from_interaction(interaction);
        let labels = semio_s_artifact_procedural_3d_terminology::generation3d_labels(view_state);
        let viewed = Generation3dViewedDocument::resolve(doc.snapshot, cfg.snapshot);
        let tree = owner.with_mut::<Generation3dViewInstanceOperationOwner, _>(|owner| owner.with_session(|session| generation3d_view_render_body(body_key, viewed.snapshot(), cfg.snapshot, preview_eval_text, Some(session), doc.tool_run(), &marks, labels)));
        viewed.retire();
        tree.map_err(|original|semio_framework_plugin::OriginalRenderFailure::from(semio_framework_job::OriginalTurnFailure::from(original)))?.map(Some).map_err(Into::into)
    }

    /// 🎚️ The Preview window's chrome: show mode, LOD and the sun group, all bound to this viewer's
    /// own actions so the read-only surface's window controls actually do something.
    fn window_measures(_doc: &ArtifactView<'_, Self::Snapshot>, cfg: &ConfigView<'_, Self::Config>, view_state: &semio_framework_plugin::ViewModel) -> HashMap<String, Vec<WindowMeasure>> {
        let is_de = view_state.locale == semio_framework_ui_locale::Locale::De;
        HashMap::from([(preview::WINDOW_KIND_ID.to_string(), preview::preview_window_measures(cfg.snapshot, is_de, generation3d_view_action))])
    }
}

/// 🧱️ Every window body of the generation3d viewer, rendered against one already-resolved set of
/// `graph` marks — one body-key match for both render entry points.
fn generation3d_view_render_body(
    body_key: &str,
    document: &Generation3dSnapshot,
    config: &Generation3dViewConfig,
    eval_json: Option<&str>,
    session: Option<&FlowEvalSession>,
    run: Option<&semio_framework_plugin::ToolRunView>,
    marks: &preview::Generation3dViewMarks,
    labels: &semio_s_artifact_procedural_3d_terminology::Generation3dLabels,
) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
    let node = match body_key {
        preview::BODY_KEY => preview::render(document, config, eval_json, session, run, marks, labels),
        _ => semio_framework_plugin::built_text_node(Label::data(format!("Unknown body: {body_key}"))).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.unknown-body", "fixed UI unknown-body admission failed")),
    }?;
    Ok(semio_framework_plugin::built_to_component_tree(node))
}
//#endregion 🔖️Viewer
