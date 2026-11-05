//! 🎛️ Flow play app: the flow play app and its `ArtifactEditor` implementation. See [`super`].

use super::*;

//#region 🔖️FlowPlayApp
/// 🧪️ Stateless app definition; the evaluation session is retained by the exact app-instance owner.
#[derive(Default)]
pub struct FlowPlayApp;

impl ArtifactEditor for FlowPlayApp {
    /// 📚️ Artifact catalogue stamped by `PluginBuilder::editor` onto the navbar dropdown.
    fn examples() -> Vec<semio_plugin::ExampleSource> {
        vec![crate::examples::demo::source()]
    }
    /// 🧩️ Composes `s.stdio.semio@v1/*` children, so every bundle of this surface opens them through the same roster.
    type Members = semio_s_artifact_stdio_semio::SemioMembers;
    /// 🛍️ Publishes the whole registered flow operator catalogue once per app instance on the reserved
    /// `framework.section.catalogue` retained surface — never on the node-graph scene, whose fixed
    /// `UI_FIXED_BYTES` admission it exceeds threefold with the real `brep`/`math` sets installed
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END §3.1).
    fn app_catalogue_json() -> String {
        flow::flow_app_catalogue_json()
    }

    type Snapshot = FlowSnapshot;
    type Mutation = FlowMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = FlowPresence;
    type PresenceMutation = FlowPresenceMutation;
    type Transient = semio_plugin::NoTransient;
    type TransientMutation = semio_plugin::NoTransientMutation;

    type Command = FlowCommand;

    const DIALECT: Dialect = crate::FLOW_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = FLOW_DOCUMENT_SCHEMA;

    fn child_restore_projection(snapshot: &Self::Snapshot) -> Result<semio_framework_composition_admission::ChildRestoreProjection<'_>, Fault> {
        semio_framework_composition_admission::ChildRestoreProjection::from_snapshot(snapshot).map_err(|error| Fault::new(semio_plugin::FaultOrigin::App, semio_plugin::FaultCode::new("flow.child-projection"), error.to_string()))
    }

    fn document_store_owners_source_demands() -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        crate::schema::retirement::store_owners_source_demands()
    }

    fn build_document_store_owners(grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Option<Result<(store::os_store::document_store_owners::DocumentStoreOwners<Self::Snapshot, Self::Mutation>, semio_framework_value::retained_clone::RetainedCloneProgress), store::os_store::document_store_owners::DocumentStoreOwnersAdmissionError<Self::Snapshot, Self::Mutation>>> {
        Some(crate::schema::retirement::store_owners(grant))
    }

    fn build_document_store_disposer() -> Option<Box<dyn semio_plugin::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
        Some(Box::new(semio_plugin::ArtifactDocumentStoreDisposer::<Self::Snapshot, Self::Mutation>::new()))
    }

    fn build_config_store_disposer() -> Option<Box<dyn semio_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::Config, Self::ConfigMutation>>>> {
        Some(semio_plugin::no_config_store_disposer())
    }

    fn build_draft_store_disposer() -> Option<Box<dyn semio_plugin::ArtifactOwnedDisposer<store::DraftStore<Self::Draft, Self::DraftMutation>>>> {
        Some(semio_plugin::no_draft_store_disposer())
    }

    fn build_presence_local_root_retirement_factory() -> Option<Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(Arc::new(crate::editor::flow::presence::retirement::FlowPresenceRetirementFactory))
    }

    fn build_presence_peer_retirement_factory() -> Option<Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(Arc::new(crate::editor::flow::presence::retirement::FlowPresenceRetirementFactory))
    }

    fn build_presence_store_disposer() -> Option<Box<dyn semio_plugin::ArtifactOwnedDisposer<store::PresenceStore<Self::Presence, Self::PresenceMutation>>>> {
        Some(crate::editor::flow::presence::retirement::store_disposer())
    }

    fn build_transient_store_disposer() -> Option<Box<dyn semio_plugin::ArtifactOwnedDisposer<store::TransientStore<Self::Transient, Self::TransientMutation>>>> {
        Some(semio_plugin::no_transient_store_disposer())
    }

    fn register_window_config_owners(registry: &mut semio_plugin::WindowConfigOwnerRegistry) -> Result<(), Fault> {
        registry.register::<main::config::FlowMainWindowConfigOwner>()
    }

    fn register_window_transient_owners(registry: &mut semio_plugin::WindowTransientOwnerRegistry) -> Result<(), Fault> {
        main::transient::register(registry)
    }

    /// 🧭️ The leaf that names a node-drag release (design §19.1): a release that also drew a wire lands its `insert-edge`
    /// before the relative `drag-nodes`, and the history row is still labelled by the drag.
    fn tool_intent_kinds(tool: &str) -> &'static [&'static str] {
        match tool.strip_prefix(edit::tools::drag::FLOW_EDITOR_APP_ID).and_then(|verb| verb.strip_prefix('#')) {
            Some(node_graph_edit::NODE_GRAPH_EDIT_VERB | "moveMediaNode") => &["drag-nodes"],
            _ => &[],
        }
    }

    fn bounded_first_step_tool_proofs() -> Vec<semio_plugin::ArtifactBoundedFirstStepProof> {
        let mut proofs = FlowDirectStoreJobFactoryProofs::proofs();
        proofs.extend(FlowHostEffectJobFactoryProofs::proofs());
        proofs.extend(FlowEvaluationJobFactoryProofs::proofs());
        proofs.extend(FlowChildGroupJobFactoryProofs::proofs());
        proofs.extend(FlowGraphOperationJobFactoryProofs::proofs());
        proofs.extend(FlowContributionsJobFactoryProofs::proofs());
        proofs
    }

    fn register_tool_job_factories(registry: &mut semio_plugin::ArtifactToolFactoryRegistry<'_, semio_plugin::EditorApp<Self>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(FlowChildGroupJobFactory::new(&controller))?;
        registry.register(FlowHostEffectJobFactory::new(&controller))?;
        registry.register(FlowEvaluationJobFactory::new(&controller))?;
        registry.register(FlowGraphOperationJobFactory::new(&controller))?;
        registry.register(FlowContributionsJobFactory::new(&controller))?;
        registry.register(FlowDirectStoreJobFactory::new(&controller))
    }

    fn build_tool_job(request: &mut Option<semio_plugin::ArtifactOwnedToolJobRequest<semio_plugin::EditorApp<Self>>>, output: &mut Option<semio_plugin::ErasedToolJob>, cx: &mut semio_plugin::__job::StepContext<'_>) -> Result<bool, semio_plugin::__job::OriginalTurnFailure> {
        let original = request.as_ref().ok_or_else(|| semio_plugin::__value::ValueError::literal(semio_plugin::__value::ValueRefusalKind::InvariantViolated, "original retained producer request is absent"))?;
        if !FLOW_CHILD_GROUP_TOOL_IDS.contains(&original.tool_id.as_str())
            && !FLOW_HOST_ONLY_TOOL_IDS.contains(&original.tool_id.as_str())
            && !FLOW_EVALUATION_TOOL_IDS.contains(&original.tool_id.as_str())
            && !FLOW_GRAPH_OPERATION_TOOL_IDS.contains(&original.tool_id.as_str())
            && !FLOW_DIRECT_STORE_TOOL_IDS.contains(&original.tool_id.as_str())
            && !FLOW_CONTRIBUTIONS_TOOL_IDS.contains(&original.tool_id.as_str())
        {
            return Ok(false);
        }
        if original.command.command_id() != original.tool_id {
            return Err(semio_plugin::__value::ValueError::literal(semio_plugin::__value::ValueRefusalKind::InvariantViolated, "Flow command does not match its exact retained tool registration").into());
        }
        if original.tool_id.as_str()!="flowEvalResolve"&&!FLOW_CONTRIBUTIONS_TOOL_IDS.contains(&original.tool_id.as_str())&&!original.instance_operation_owner.with_mut::<FlowInstanceOperationOwner,_>(|owner|owner.catalogue_ready())?{return Ok(false)}
        if FLOW_CHILD_GROUP_TOOL_IDS.contains(&original.tool_id.as_str())
            || FLOW_GRAPH_OPERATION_TOOL_IDS.contains(&original.tool_id.as_str())
            || FLOW_DIRECT_STORE_TOOL_IDS.contains(&original.tool_id.as_str())
            || FLOW_CONTRIBUTIONS_TOOL_IDS.contains(&original.tool_id.as_str())
        {
            return flow_build_retained_job(request,output,cx);
        }
        flow_build_host_job(request, output, cx)
    }

    fn build_instance_operation_owner() -> Box<dyn semio_plugin::ArtifactInstanceOperationOwner> {
        Box::new(FlowInstanceOperationOwner::new())
    }

    fn reference_birth()->Option<fn()->Result<FlowSnapshot,semio_plugin::Fault>>{Some(||{
        Ok(FlowSnapshot::default())
    })}

    /// 🌱️ Derives the `content` child at boot and on every archive load, so a live shell composes the
    /// child that every verb edits and every window reads — see [`crate::flow_genesis_content_pack`] and
    /// [`crate::flow_composed_snapshot`].
    fn genesis_child_pack(snapshot: &Self::Snapshot, slot: &str, child_id: &str) -> Result<Option<Vec<u8>>,semio_framework_value::ValueError> {
 Ok((||{
        crate::flow_genesis_content_pack(snapshot, slot, child_id)
    
})())
}

    /// 🏷️ The manifest action id each command was declared under — supplied wholesale by
    /// manifest declaration (host-pushed/internally-chained, not user-facing actions).
    fn command_id(command: &FlowCommand) -> &'static str {
        command.command_id()
    }

    /// 🎯️ Maps host action id + JSON args onto `FlowCommand`, the bridge every app owes the shell's
    /// `{action, args}` wire. Without it the trait's default refuses EVERY id, and the one caller that
    /// cannot avoid this channel is the host's own re-arm: `Effect::dispatchAction` is delivered through
    /// `handle_action`, so flow's `flowEvalTick` chain died on its first hop with
    /// `action 'flowEvalTick' is not a framework-reserved action` and the node graph never evaluated
    /// (16 refusal lines per boot, measured on :6016, ticket 26/09/18 slice B3c). Mirrors
    /// `Generation2dPlayApp::command_from_action`, which is why that sibling's identical eval chain runs.
    fn command_from_action(action: &str, args: &mut Option<semio_framework_value::DslValue>, original_fault: &mut Option<Fault>) -> Result<Self::Command, Fault> {
        if let Some(fault) = original_fault.take() { return Err(fault); }
        if action=="flowEvalResolve"{
            let original=args.as_ref().ok_or_else(||Fault::from("flow-eval-resolve-original-args-required"))?;
            if original.get("windowId").and_then(semio_framework_value::DslValue::as_str).is_none_or(str::is_empty){return Err(Fault::from("flow-eval-resolve-window-required"))}
            let node_hash=original.get("nodeHash").and_then(semio_framework_value::DslValue::as_u64).ok_or_else(||Fault::from("flow-eval-resolve-node-hash-required"))?;
            let Some(semio_framework_value::DslValue::Object(fields))=args.as_mut()else{return Err(Fault::from("flow-eval-resolve-object-required"))};
            let window_id=match fields.iter_mut().find(|(key,_)|key=="windowId").map(|(_,value)|value){Some(semio_framework_value::DslValue::String(value))=>std::mem::take(value),_=>unreachable!("validated original Flow resolve window")};
            let output=fields.iter_mut().find(|(key,_)|key=="output").and_then(|(_,value)|if matches!(value,semio_framework_value::DslValue::Null){None}else{Some(std::mem::replace(value,semio_framework_value::DslValue::Null))});
            return Ok(FlowCommand::FlowEvalResolve(flow_eval_resolve::FlowEvalResolve{window_id,node_hash,output}));
        }
        let args = args.as_ref();
        let args = args.cloned().unwrap_or(semio_framework_value::DslValue::Null);
        let str_arg = |keys: &[&str]| -> Option<String> { keys.iter().find_map(|key| args.get(key).and_then(|value| value.as_str()).map(str::to_string)) };
        let f64_arg = |keys: &[&str]| -> Option<f64> { keys.iter().find_map(|key| args.get(key).and_then(|value| value.as_f64())) };
        let u64_arg = |keys: &[&str]| -> Option<u64> { keys.iter().find_map(|key| args.get(key).and_then(|value| value.as_u64().or_else(|| value.as_f64().map(|number| number as u64)))) };
        let bool_arg = |keys: &[&str]| -> Option<bool> { keys.iter().find_map(|key| args.get(key).and_then(semio_framework_value::DslValue::as_bool)) };
        let string_list = |keys: &[&str]| -> Vec<String> {
            keys.iter()
                .find_map(|key| args.get(key).and_then(semio_framework_value::DslValue::as_array))
                .map_or_else(Vec::new, |items| items.iter().filter_map(|item| item.as_str().map(str::to_string)).collect())
        };
        match action {
            "addWidget" => Ok(FlowCommand::AddWidget(add_widget::AddWidget {
                kind: str_arg(&["kind"]).unwrap_or_else(|| "inputSlider".into()),
                neuron_kind: str_arg(&["neuronKind", "neuron_kind"]),
                x: f64_arg(&["x"]),
                y: f64_arg(&["y"]),
                label: str_arg(&["label"]),
                action: str_arg(&["action"]),
                format: str_arg(&["format"]),
            })),
            "removeWidget" => Ok(FlowCommand::RemoveWidget(remove_widget::RemoveWidget { widget_id: str_arg(&["widgetId", "widget_id", "id"]).unwrap_or_default() })),
            "duplicateWidget" => Ok(FlowCommand::DuplicateWidget(duplicate_widget::DuplicateWidget { widget_id: str_arg(&["widgetId", "widget_id", "id"]).unwrap_or_default() })),
            "deleteSelection" => Ok(FlowCommand::DeleteSelection(delete_selection::DeleteSelection {})),
            "disconnect" => Ok(FlowCommand::Disconnect(disconnect::Disconnect { synapse_id: str_arg(&["synapseId", "synapse_id", "id"]).unwrap_or_default() })),
            "connectMediaPorts" => Ok(FlowCommand::ConnectMediaPorts(connect_media_ports::ConnectMediaPorts {
                source_node_id: str_arg(&["sourceNodeId", "source_node_id"]).unwrap_or_default(),
                source_port_id: str_arg(&["sourcePortId", "source_port_id"]).unwrap_or_default(),
                target_node_id: str_arg(&["targetNodeId", "target_node_id"]).unwrap_or_default(),
                target_port_id: str_arg(&["targetPortId", "target_port_id"]).unwrap_or_default(),
            })),
            "moveMediaNode" => Ok(FlowCommand::MoveMediaNode(move_media_node::MoveMediaNode { node_id: str_arg(&["nodeId", "node_id", "id"]).unwrap_or_default(), x: f64_arg(&["x"]).unwrap_or(0.0), y: f64_arg(&["y"]).unwrap_or(0.0) })),
            "reorganize" => Ok(FlowCommand::Reorganize(reorganize::Reorganize {})),
            "patchFlowWidgets" => Ok(FlowCommand::PatchFlowWidgets(patch_flow_widgets::PatchFlowWidgets {
                widget_ids: string_list(&["widgetIds", "widget_ids", "ids"]),
                field: str_arg(&["field"]).unwrap_or_default(),
                value: str_arg(&["value"]).unwrap_or_default(),
            })),
            "renameFlowWidget" => Ok(FlowCommand::RenameFlowWidget(rename_flow_widget::RenameFlowWidget { old_id: str_arg(&["oldId", "old_id", "id"]).unwrap_or_default(), value: str_arg(&["value", "name"]).unwrap_or_default() })),
            "nodeGraphEdit" => Ok(FlowCommand::NodeGraphEdit(node_graph_edit::NodeGraphEdit { operations: node_graph_edit::operations_from_action(&args)? })),
            "spotlightCommit" => Ok(FlowCommand::SpotlightCommit(spotlight_commit::SpotlightCommit { operations: node_graph_edit::operations_from_action(&args)? })),
            "runExtensionAction" => Ok(FlowCommand::RunExtensionAction(run_extension_action::RunExtensionAction { action_id: str_arg(&["actionId", "action_id", "id"]).unwrap_or_default() })),
            "setActiveExample" => Ok(FlowCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: str_arg(&["exampleId", "example_id"]).unwrap_or_default() })),
            "evaluate" => Ok(FlowCommand::Evaluate(evaluate::Evaluate {})),
            "focusSelection" => Ok(FlowCommand::FocusSelection(focus_selection::FocusSelection {})),
            "nodeGraphViewport" => Ok(FlowCommand::NodeGraphViewport(node_graph_viewport::NodeGraphViewport {
                viewport: match args.get("viewport").cloned() {
                    None => semio_os_kernel::Viewport2d::default(),
                    Some(value) => semio_framework_value::FromValue::from_value(value).map_err(|error| Fault::from(format!("invalid nodeGraphViewport viewport: {error}")))?,
                },
            })),
            "setLodMode" => Ok(FlowCommand::SetLodMode(set_lod_mode::SetLodMode { value: str_arg(&["value", "mode"]).unwrap_or_default() })),
            "setProximityDistance" => Ok(FlowCommand::SetProximityDistance(set_proximity_distance::SetProximityDistance { value: f64_arg(&["value", "distance"]).unwrap_or_default() })),
            "setGridVisible" => Ok(FlowCommand::SetGridVisible(set_grid_visible::SetGridVisible { pressed: bool_arg(&["pressed", "value", "visible"]) })),
            "setGridSnapEnabled" => Ok(FlowCommand::SetGridSnapEnabled(set_grid_snap_enabled::SetGridSnapEnabled { pressed: bool_arg(&["pressed", "value", "enabled"]) })),
            "setGridFactor" => Ok(FlowCommand::SetGridFactor(set_grid_factor::SetGridFactor { value: f64_arg(&["value", "factor"]).unwrap_or_default() })),
            "contextMenuAt" => Ok(FlowCommand::ContextMenuAt(context_menu_at::ContextMenuAt { id: str_arg(&["id"]).unwrap_or_default() })),
            "setPreviewOff" => Ok(FlowCommand::SetPreviewOff(set_preview_off::SetPreviewOff { ids: string_list(&["ids", "widgetIds", "widget_ids"]), value: bool_arg(&["value", "off"]).unwrap_or(false) })),
            "openSpotlight" => Ok(FlowCommand::OpenSpotlight(open_spotlight::OpenSpotlight {})),
            "replaceImage" => Ok(FlowCommand::ReplaceImage(replace_image::ReplaceImage { id: str_arg(&["id", "widgetId", "widget_id"]).unwrap_or_default() })),
            "setCatalogueSections" => Ok(FlowCommand::SetCatalogueSections(set_catalogue_sections::SetCatalogueSections { sections_json: str_arg(&["sectionsJson", "sections_json"]).or_else(|| args.get("sections").map(semio_framework_pack_json::to_json_string)).unwrap_or_else(|| "[]".into()) })),
            "toggleExtension" => Ok(FlowCommand::ToggleExtension(toggle_extension::ToggleExtension { id: str_arg(&["id", "extensionId", "extension_id"]).unwrap_or_default(), enabled: bool_arg(&["enabled", "value"]).unwrap_or(false) })),
            "addGeneration" => Ok(FlowCommand::AddGeneration(add_generation::AddGeneration {})),
            "removeGeneration" => Ok(FlowCommand::RemoveGeneration(remove_generation::RemoveGeneration { id: str_arg(&["id"]).unwrap_or_default() })),
            "selectGeneration" => Ok(FlowCommand::SelectGeneration(select_generation::SelectGeneration { id: str_arg(&["id"]).unwrap_or_default() })),
            "renameGeneration" => Ok(FlowCommand::RenameGeneration(rename_generation::RenameGeneration { id: str_arg(&["id"]).unwrap_or_default(), name: str_arg(&["name", "value"]).unwrap_or_default() })),
            "updateGenerationValues" => Ok(FlowCommand::UpdateGenerationValues(update_generation_values::UpdateGenerationValues {
                generation_id: str_arg(&["generationId", "generation_id"]),
                question_id: str_arg(&["questionId", "question_id"]).unwrap_or_default(),
                value: args.get("value").cloned().unwrap_or(semio_framework_value::DslValue::Null),
            })),
            "flowEvalTick" => Ok(FlowCommand::FlowEvalTick(flow_eval_tick::FlowEvalTick {
                window_id: str_arg(&["windowId", "window_id"]).unwrap_or_else(|| FLOW_PLAY_WINDOW_MAIN.into()),
                window_kind_id: str_arg(&["windowKindId", "window_kind_id"]).unwrap_or_else(|| FLOW_PLAY_WINDOW_MAIN.into()),
            })),
            "setContributions" => Ok(FlowCommand::SetContributions(set_contributions::SetContributions {
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

    /// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `deleteSelection`/`focusSelection`/
    /// `nodeGraphEdit`/`spotlightCommit` read the "graph" interaction domain directly (bypassing the
    /// `app_commands!`-generated `dispatch`, whose per-row `$module::handle(payload, doc, cfg, session)`
    /// signature is framework-fixed and has no `interaction` slot) — mirrors `space`'s equivalent routing.
    fn handle(
        command: &FlowCommand,
        doc: &ArtifactView<'_, FlowSnapshot>,
        cfg: &ConfigView<'_, NoConfig>,
        interaction: &InteractionView<'_>,
        _view_state: Option<&semio_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<FlowMutation, NoConfigMutation, Self::DraftMutation>, Fault> {
        if FLOW_CHILD_GROUP_TOOL_IDS.contains(&command.command_id())
            || FLOW_HOST_ONLY_TOOL_IDS.contains(&command.command_id())
            || FLOW_GRAPH_OPERATION_TOOL_IDS.contains(&command.command_id())
            || FLOW_DIRECT_STORE_TOOL_IDS.contains(&command.command_id())
        {
            return Err(Fault::new(semio_plugin::FaultOrigin::App, semio_plugin::FaultCode::new("flow.retained.legacy-dispatch"), "Flow retained routes execute only through their exact app-owned job factory"));
        }
        let mut session = FlowEvalSession::new();
        let emitted = match command {
            FlowCommand::DeleteSelection(payload) => delete_selection::apply(payload, doc, cfg, &mut session, interaction),
            FlowCommand::FocusSelection(payload) => focus_selection::apply(payload, doc, cfg, &mut session, interaction),
            _ => command.dispatch(doc, cfg, &mut session),
        };
        session.retire_cold();
        emitted
    }

    /// 🕹️ `graph`'s `HierarchyProvider::Topology`: every widget/synapse is registered at its own
    /// granularity, every one a root — the outer widget list has no real parent/child membership (a
    /// `Widget::Cluster`'s own `tree` is a private, self-contained nested sub-graph, not exposed at this
    /// domain), so this deliberately does NOT declare transitive hover/selection (see `🔖️Manifest`'s
    /// `.interaction(...)` doc comment for why that diverges from the ticket's headline example).
    /// `Topology` (rather than `Flat`) is still the right choice purely for the pruning it buys:
    /// `validate_state` drops stale ids of a domain it has membership info for, and `Flat` domains are
    /// skipped entirely (see the design doc's `HierarchyProvider::Flat` note). "handle" targets have no
    /// persisted document data to register — see `flow_graph_selection_domains`'s doc comment.
    fn interaction_topology(doc: &ArtifactView<'_, FlowSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<InteractionTopology, semio_framework_value::ValueError> {
 Ok((||{
        let mut domains = std::collections::BTreeMap::new();
        let Ok(composed) = crate::flow_composed_snapshot(doc.snapshot, &doc.children) else {
            domains.insert(FLOW_INTERACTION_GRAPH.to_string(), DomainTopology { ordered: Vec::new() });
            return InteractionTopology { domains };
        };
        let live = composed.to_host_snapshot();
        let mut ordered: Vec<TopologyNode> = live.widgets.iter().map(|widget| TopologyNode { id: flow_graph_node_target_id(crate::schema::widget_id(widget)), granularity: "node".into(), parent: None }).collect();
        ordered.extend(live.synapses.iter().map(|synapse| TopologyNode { id: flow_graph_edge_target_id(&synapse.id), granularity: "edge".into(), parent: None }));
        live.retire_cold();
        domains.insert(FLOW_INTERACTION_GRAPH.to_string(), DomainTopology { ordered });
        InteractionTopology { domains }
    
})())
}

    /// 🧵️ Arms a `flowEvalTick` chain for every ATTACHED main window whose evaluation the snapshot
    /// still owes — covers every mutation path (edits, undo/redo, example load, remote operations) in
    /// one place.
    ///
    /// 🔒️ The probe runs against the app instance's RETAINED session, never a throwaway one. A
    /// throwaway session has an empty tick latch by construction, so every host refresh answered
    /// "nothing owes a hop yet" and minted another one for the identical snapshot: 2015 ×
    /// `transient read registry is busy or exhausted` in ~3 s and the shell down with it, measured on
    /// :6016 (ticket 26/09/18 §5.3). Latches of windows that have left the roster are dropped here,
    /// because a detached window's latch can never be discharged by a hop.
    fn pending_effects(owner: &semio_plugin::ArtifactInstanceOperationOwnerHandle, doc: &semio_plugin::ArtifactPendingView<'_, FlowSnapshot>, cfg: &ConfigView<'_, NoConfig>, view: Option<&semio_plugin::ViewModel>) -> Vec<Effect> {
        let windows: Vec<(String, String)> = view
            .map(|view| view.window_instances.iter().filter(|window| window.window_kind_id == FLOW_PLAY_WINDOW_MAIN).map(|window| (window.id.clone(), window.window_kind_id.clone())).collect())
            .unwrap_or_default();
        if windows.is_empty() {
            return Vec::new();
        }
        let config = main::config::current(cfg);
        let Ok(composed) = crate::flow_composed_snapshot(doc.snapshot, &doc.children) else { return Vec::new() };
        owner
            .with_mut::<FlowInstanceOperationOwner, _>(|instance| {
                instance.with_session(|session| {
                    let live: Vec<&str> = windows.iter().map(|(id, _)| id.as_str()).collect();
                    let mut cursor = 0;
                    while !matches!(session.retain_window_tick_latches(&live, &mut cursor, cold_grant()).expect("flow cold session grant"), semio_framework_value::RetainedCloneStep::Complete(_)) {}
                    windows.iter().flat_map(|(window_id, window_kind_id)| match evaluate::evaluate_result(&composed, &config, session, window_id, window_kind_id) { Ok(emit) => emit.effects, Err(fault) => vec![Effect::Notify { message: fault.message }] }).collect::<Vec<Effect>>()
                })
            })
            .unwrap_or_default()
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, FlowSnapshot>, cfg: &ConfigView<'_, NoConfig>, view_state: &semio_plugin::ViewModel) -> semio_plugin::UiAssemblyResult<semio_plugin::ComponentTree> {
        let config = main::config::current(cfg);
        let transient = main::transient::FlowWindowTransient::default();
        let labels = flow_play_labels(view_state);
        let render=|source:&dyn semio_framework_artifact_flow_flow::FlowSnapshotSource,session:&mut FlowEvalSession| match body_key {
            FLOW_PLAY_BODY_MAIN => main::render_source(source, &config, session, &[]).map(semio_plugin::built_to_component_tree),
            FLOW_PLAY_BODY_COMPILED => compiled::render_source(source, &config, session).map(semio_plugin::built_to_component_tree),
            FLOW_PLAY_BODY_GENERATIONS => generations::render(&transient, view_state.locale, view_state.terminology, &semio_plugin::TreeWindows::for_body(view_state, FLOW_PLAY_BODY_GENERATIONS))
                .map(semio_plugin::built_to_component_tree),
            FLOW_PLAY_BODY_GENERATE_FORM => form::render_source(source, &config, &transient, labels).map(semio_plugin::built_to_component_tree),
            FLOW_PLAY_BODY_GENERATE_PREVIEW => preview::render(&transient).map(semio_plugin::built_to_component_tree),
            FLOW_PLAY_BODY_ARTIFACT => document_panel::render_live(source, labels, &semio_plugin::TreeWindows::for_body(view_state, FLOW_PLAY_BODY_ARTIFACT)).map(semio_plugin::built_to_component_tree),
            FLOW_PLAY_BODY_CATALOGUE => catalogue_panel::render(&config, labels, &semio_plugin::TreeWindows::for_body(view_state, FLOW_PLAY_BODY_CATALOGUE)).map(semio_plugin::built_to_component_tree),
            FLOW_PLAY_BODY_INSPECTOR => inspection_panel::render(labels, &semio_plugin::TreeWindows::for_body(view_state, FLOW_PLAY_BODY_INSPECTOR)).map(semio_plugin::built_to_component_tree),
            _ => semio_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        };
        if let Some(instance)=doc.instance(){
            return instance.with_original_mut::<FlowInstanceOperationOwner,_>(|owner|owner.with_scene_session(doc,|source,session|render(source,session)))
                .map_err(|error|semio_plugin::PluginAssemblyError::new("flow.eval-session-owner",error.primary.message()))?
                .flatten().unwrap_or_else(||Err(semio_plugin::PluginAssemblyError::new("flow.content-pending","Flow execution scene is still being prepared")));
        }
        let source=doc.snapshot.read_source().map_err(|error|semio_plugin::PluginAssemblyError::new("flow.source",error.into_message()))?;
        let mut session=FlowEvalSession::new();let rendered=render(&source,&mut session);session.retire_cold();rendered
    }

    fn render_with_request_context(
        owner: &semio_plugin::ArtifactInstanceOperationOwnerHandle,
        body_key: &str,
        doc: &ArtifactView<'_, FlowSnapshot>,
        cfg: &ConfigView<'_, NoConfig>,
        view_state: &semio_plugin::ViewModel,
        transient: &semio_plugin::TransientView<'_, semio_plugin::NoTransient>,
        interaction: &InteractionView<'_>,
        cx:&mut semio_plugin::StepContext<'_>,
    ) -> semio_plugin::OriginalRenderResult<semio_plugin::ComponentTree> {
        let rendered:semio_plugin::OriginalRenderResult<semio_plugin::ComponentTree>=(||{
        let config = main::config::current(cfg);
        let transient = main::transient::current(transient);
        let (graph_selection, _) = flow_graph_selection_domains(&interaction.selection(FLOW_INTERACTION_GRAPH).ids);

        owner
            .with_original_mut::<FlowInstanceOperationOwner, _>(|owner| {
                owner.with_scene_session(doc,|source,session| match body_key {
                    FLOW_PLAY_BODY_MAIN => main::render_source(source, &config, session, &graph_selection).map(semio_plugin::built_to_component_tree),
                    FLOW_PLAY_BODY_COMPILED => compiled::render_source(source, &config, session).map(semio_plugin::built_to_component_tree),
                    FLOW_PLAY_BODY_GENERATIONS => {
                        generations::render(&transient, view_state.locale, view_state.terminology, &semio_plugin::TreeWindows::for_body(view_state, FLOW_PLAY_BODY_GENERATIONS))
                            .map(semio_plugin::built_to_component_tree)
                    }
                    FLOW_PLAY_BODY_GENERATE_FORM => form::render_source(source, &config, &transient, flow_play_labels(view_state)).map(semio_plugin::built_to_component_tree),
                    FLOW_PLAY_BODY_GENERATE_PREVIEW => preview::render(&transient).map(semio_plugin::built_to_component_tree),
                    FLOW_PLAY_BODY_ARTIFACT => document_panel::render_live(source, flow_play_labels(view_state), &semio_plugin::TreeWindows::for_body(view_state, FLOW_PLAY_BODY_ARTIFACT)).map(semio_plugin::built_to_component_tree),
                    FLOW_PLAY_BODY_CATALOGUE => {
                        catalogue_panel::render(&config, flow_play_labels(view_state), &semio_plugin::TreeWindows::for_body(view_state, FLOW_PLAY_BODY_CATALOGUE)).map(semio_plugin::built_to_component_tree)
                    }
                    FLOW_PLAY_BODY_INSPECTOR => inspection_panel::render(flow_play_labels(view_state), &semio_plugin::TreeWindows::for_body(view_state, FLOW_PLAY_BODY_INSPECTOR)).map(semio_plugin::built_to_component_tree),
                    _ => semio_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
                })
            })
            ?
            .flatten().transpose().map_err(Into::into)

        })();
        rendered
        }

    fn window_measures(_doc: &ArtifactView<'_, FlowSnapshot>, cfg: &ConfigView<'_, NoConfig>, view_state: &semio_plugin::ViewModel) -> HashMap<String, Vec<WindowMeasure>> {
        let Some(window_id) = view_state.window_id.clone() else { return HashMap::new() };
        let is_main = view_state.window_instances.iter().any(|window| window.id == window_id && window.window_kind_id == FLOW_PLAY_WINDOW_MAIN);
        if !is_main {
            return HashMap::new();
        }
        let config = main::config::current(cfg);
        HashMap::from([(window_id, main::window_measures(&config, flow_play_labels(view_state)))])
    }

    fn context_menu(request: &ContextMenuRequest, doc: &ArtifactView<'_, FlowSnapshot>, cfg: &ConfigView<'_, NoConfig>, view_state: &semio_plugin::ViewModel, registry: &AppActionRegistry) -> Vec<ContextMenuItemSpec> {
        let config=main::config::current(cfg);
        let render=|source:&dyn semio_framework_artifact_flow_flow::FlowSnapshotSource|flow_context_menu_items(registry,source,&config,flow_play_labels(view_state),view_state,request.surface.as_ref());
        if let Some(instance)=doc.instance(){return instance.with_original_mut::<FlowInstanceOperationOwner,_>(|owner|owner.with_scene_session(doc,|source,_|render(source))).ok().flatten().flatten().unwrap_or_default()}
        doc.snapshot.read_source().map(|source|render(&source)).unwrap_or_default()
    }
}
//#endregion 🔖️FlowPlayApp
