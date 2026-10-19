//! 🎛️ Play app editor and constructor: the equation play app with its artifact editor implementation and the equation app constructor with its manifest stitch. See [`super`].

use super::*;

//#region 🔖️EquationPlayApp
/// 🧪️ B1: unit struct — the former `MathPlayRuntime`/`self.runtime` field now lives in
/// the registered graph-window configuration owner.
#[derive(Default)]
pub struct EquationPlayApp;

impl ArtifactEditor for EquationPlayApp {
    /// 🧩️ Composes `s.stdio.semio@v1/*` children (`text`/`table`/`value`), so every bundle of this
    /// surface opens them through the same roster. The react shell's `loadDocumentPair` sends
    /// `members: []`, so an undeclared-but-derivable child makes the archive closure `Incomplete`.
    type Members = semio_s_artifact_stdio_semio::SemioMembers;
    type Snapshot = EquationSnapshot;
    type Mutation = EquationMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = semio_framework_plugin::NoPresence;
    type PresenceMutation = semio_framework_plugin::NoPresenceMutation;
    type Transient = semio_framework_plugin::NoTransient;
    type TransientMutation = semio_framework_plugin::NoTransientMutation;

    type Command = EquationCommand;

    const DIALECT: Dialect = EQUATION_DIALECT;
    /// 🧬️ The crate's one loaded-parent child projection (`crate::equation_child_restore_projection`).
    fn child_restore_projection(snapshot: &Self::Snapshot) -> Result<semio_framework_composition_admission::ChildRestoreProjection<'_>, Fault> {
        crate::equation_child_restore_projection(snapshot)
    }
    const DOCUMENT_SCHEMA: &'static str = MATH_DOCUMENT_SCHEMA;

    fn build_artifact_store_one_item_preparation_factory() -> Option<Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(Arc::new(EquationStorePreparationFactory::<Self::Snapshot, Self::Mutation>::default()))
    }

    fn build_document_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
        Some(Box::new(semio_framework_plugin::ArtifactDocumentStoreDisposer::<Self::Snapshot, Self::Mutation>::new()))
    }

    fn build_config_store_disposer() -> ArtifactDisposal<store::ConfigStore<Self::Config, Self::ConfigMutation>> {
        Some(semio_framework_plugin::no_config_store_disposer())
    }

    fn build_draft_store_disposer() -> ArtifactDisposal<store::DraftStore<Self::Draft, Self::DraftMutation>> {
        Some(semio_framework_plugin::no_draft_store_disposer())
    }

    fn build_presence_local_root_retirement_factory() -> Option<Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::no_presence_local_root_retirement_factory())
    }

    fn build_presence_peer_retirement_factory() -> Option<Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::no_presence_peer_retirement_factory())
    }

    fn build_presence_store_disposer() -> ArtifactDisposal<store::PresenceStore<Self::Presence, Self::PresenceMutation>> {
        Some(semio_framework_plugin::no_presence_store_disposer())
    }

    fn build_transient_local_root_retirement_factory() -> Option<Arc<dyn store::SnapshotRetirementFactory<Self::Transient>>> {
        Some(semio_framework_plugin::no_transient_local_root_retirement_factory())
    }

    fn build_transient_store_disposer() -> ArtifactDisposal<store::TransientStore<Self::Transient, Self::TransientMutation>> {
        Some(semio_framework_plugin::no_transient_store_disposer())
    }

    fn register_window_config_owners(registry: &mut semio_framework_plugin::WindowConfigOwnerRegistry) -> Result<(), Fault> {
        registry.register::<EquationGraphWindowConfigOwner>()
    }

    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: EditorApp<EquationPlayApp>,
        owner_file: "✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.mathematical.equation@1/*#editor",
        artifact_schema: "semio.equation/v1",
        factory: "EquationCommandJobFactory",
        factory_type: EquationCommandJobFactory,
        tools: {
            "editEquation" => ToolExecutionContract::resumable(65_536, 2_048, 1, 65_536, 7_500, 1, 1),
            "setAlgorithm" => ToolExecutionContract::resumable(65_536, 2_048, 1, 65_536, 7_500, 1, 1),
            "setDirected" => ToolExecutionContract::resumable(65_536, 2_048, 1, 65_536, 7_500, 1, 1),
            "nodeGraphEdit" => ToolExecutionContract::resumable(65_536, 2_048, 1, 65_536, 7_500, 1, 1),
            "nodeGraphViewport" => ToolExecutionContract::resumable(65_536, 2_048, 1, 65_536, 7_500, 1, 1),
            "editPoints" => ToolExecutionContract::resumable(65_536, 2_048, 1, 65_536, 7_500, 1, 1),
            "setActiveExample" => ToolExecutionContract::resumable(65_536, 2_048, 1, 65_536, 7_500, 1, 1),
            "addNode" => ToolExecutionContract::resumable(65_536, 2_048, 1, 65_536, 7_500, 1, 1),
        }
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(EquationCommandJobFactory::new(&controller))
    }

    fn build_tool_job(request: ArtifactOwnedToolJobRequest<EditorApp<Self>>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
        if !EQUATION_TOOL_IDS.contains(&request.tool_id.as_str()) {
            return Ok(None);
        }
        if request.command.command_id() != request.tool_id {
            return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("app.command.tool-mismatch"), "Equation command does not match its exact retained tool registration"));
        }
        let extent = equation_command_extent(&request.command, &request.snapshot).ok_or_else(|| Fault::from("equation-command-capacity"))?;
        let tool_id = request.command.command_id();
        let operation_context = AppOperationContext {
            app_instance_id: request.app_instance_id,
            parent_document_id: request.parent_document_id.clone(),
            operation_id: request.operation.operation.0,
            generation: request.operation.generation.0,
            canonical_base_revision: request.canonical_base_revision,
            retained: request.retained,
            authoring_seed: request.authoring_seed.clone(),
        };
        let work: Box<dyn ArtifactCommandWork<EditorApp<Self>>> = Box::new(EquationRetainedCommandWork::new(tool_id, equation_operation_identity(tool_id, &operation_context), extent));
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
            EquationCommand::command_id,
            EQUATION_RETAINED_RAW_BYTES,
            EQUATION_RETAINED_WORK_ITEMS,
            work,
        );
        Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }

    /// 🌱️ The derivable `notation`/`results`/`computed` members — see
    /// `crate::genesis_equation_child_pack`.
    fn genesis_child_pack(snapshot: &Self::Snapshot, slot: &str, child_id: &str) -> Result<Option<Vec<u8>>,semio_framework_value::ValueError> {
 Ok((||{
        crate::genesis_equation_child_pack(snapshot, slot, child_id)
    
})())
}

    fn initial_snapshot() -> EquationSnapshot {
        EquationSnapshot::default()
    }

    fn io() -> Option<semio_framework_plugin::AppIo> {
        Some(equation_io())
    }

    /// 🏷️ The manifest action id each command was declared under — supplied wholesale by
    /// not a user-facing action).
    fn command_id(command: &EquationCommand) -> &'static str {
        command.command_id()
    }

    /// 🎯️ Maps a host action id + its staged args onto `EquationCommand`. The React/wgpu shells
    /// still dispatch `{action, args}` while the guest channel is typed-only, and the trait default
    /// refuses EVERY id (`app.command.unsupported`) — without this bridge every Actions-pane row and
    /// every graph gesture died before reaching `handle`. The three block-shaped payloads
    /// (`graph`/`geometry`/`viewport`) are decoded through `semio_framework_value::FromValue::from_value`, the same codec the
    /// typed channel uses, so the shell needs no staging shim.
    fn command_from_action(action: &str, args: &mut Option<semio_framework_value::DslValue>, original_fault: &mut Option<Fault>) -> Result<EquationCommand, Fault> {
        if let Some(fault) = original_fault.take() { return Err(fault); }
        let args = args.as_ref();
        let text_arg = |keys: &[&str]| keys.iter().find_map(|key| args.and_then(|value| value.get(key)).and_then(semio_framework_value::DslValue::as_str).map(str::to_string));
        fn decode<T: semio_framework_value::FromValue>(action: &str, args: Option<&semio_framework_value::DslValue>, key: &str) -> Result<T, Fault> {
            let value = args.and_then(|value| value.get(key)).cloned().ok_or_else(|| Fault::from(format!("equation {action} requires a '{key}' block")))?;
            semio_framework_value::FromValue::from_value(value).map_err(|error| Fault::from(format!("invalid equation {action} '{key}': {error}")))
        }
        match action {
            "editEquation" => Ok(EquationCommand::EditEquation(edit_equation::EditEquation { graph: decode(action, args, "graph")?, geometry: decode(action, args, "geometry")? })),
            "setAlgorithm" => Ok(EquationCommand::SetAlgorithm(set_algorithm::SetAlgorithm { algorithm: text_arg(&["algorithm", "value"]).unwrap_or_default(), seed: text_arg(&["seed"]) })),
            "setDirected" => Ok(EquationCommand::SetDirected(set_directed::SetDirected {
                directed: args.and_then(|value| value.get("directed").or_else(|| value.get("value"))).and_then(semio_framework_value::DslValue::as_bool).unwrap_or(false),
            })),
            "nodeGraphEdit" => Ok(EquationCommand::NodeGraphEdit(node_graph_edit::NodeGraphEdit {
                operations_json: args.and_then(|value| value.get("operations")).map_or_else(|| "[]".into(), |value| value.as_str().map_or_else(|| semio_framework_pack_json::to_json_string(value), str::to_string)),
            })),
            "addNode" => {
                let coordinate = |key: &str| args.and_then(|value| value.get(key)).and_then(semio_framework_value::DslValue::as_f64).ok_or_else(|| Fault::from(format!("equation addNode requires a numeric '{key}'")));
                Ok(EquationCommand::AddNode(add_node::AddNode { x: coordinate("x")?, y: coordinate("y")? }))
            }
            "nodeGraphViewport" => Ok(EquationCommand::NodeGraphViewport(node_graph_viewport::NodeGraphViewport { viewport: decode(action, args, "viewport")? })),
            "editPoints" => Ok(EquationCommand::EditPoints(edit_points::EditPoints { geometry: decode(action, args, "geometry")? })),
            "setActiveExample" => Ok(EquationCommand::SetActiveExample(set_active_example::SetActiveExample {
                example_id: text_arg(&["exampleId", "example_id", "id", "value"]).unwrap_or_else(|| crate::examples::demo::ID.into()),
            })),
            other => Err(Fault::from(format!("equation: unhandled action id {other}"))),
        }
    }

    fn handle(
        command: &EquationCommand,
        doc: &ArtifactView<'_, EquationSnapshot>,
        cfg: &ConfigView<'_, NoConfig>,
        _interaction: &InteractionView<'_>,
        view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<EquationMutation, NoConfigMutation, Self::DraftMutation>, Fault> {
        if let EquationCommand::NodeGraphViewport(payload) = command {
            let view = view_state.ok_or_else(|| Fault::from("equation-graph-window-context-required"))?;
            let mut emit = Emit::default();
            emit.window_config_mutations.push(graph_window::config::addressed(view, EquationGraphWindowConfigMutation::SetCamera(graph_window::config::SetCamera {
                camera: EquationCamera { x: payload.viewport.x, y: payload.viewport.y, zoom: payload.viewport.zoom },
            }))?);
            return Ok(emit);
        }
        command.dispatch(doc, cfg)
    }

    /// 🎞️ `"result:out"` exports the active algorithm's per-node overlay (topo order/connected
    /// components/SCC group/BFS distance — the port recipe's `computation.equation`-kinded output);
    /// `"artifact:out"` replicates `ArtifactApp::export_media`'s default whole-document-pack behavior
    /// (unreachable once this override exists).
    fn export_media(port: &str, doc: &ArtifactView<'_, EquationSnapshot>) -> Result<Media, MediaError> {
        match port {
            "result:out" => {
                let graph = doc.snapshot.graph.clone();
                let overlay = algorithm_overlay(&graph);
                let overlay_json = semio_framework_pack_json::object(overlay.iter().map(|(id, suffix)| (id.clone(), Value::from(suffix.as_str()))));
                let json = semio_framework_pack_json::to_string(&semio_framework_pack_json::object([("algorithm".to_string(), Value::from(graph.algorithm.as_str())), ("overlay".to_string(), overlay_json)]));
                Ok(Media { media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value }, payload: MediaPayload::Structured { schema: "computation.equation".into(), json } })
            }
            "artifact:out" => {
                let media_type = Self::io().map_or(MediaType { class: MediaClass::Data, form: MediaForm::Value }, |io| io.artifact_media_type);
                let bytes = doc.snapshot.encode_pack();
                Ok(Media { media_type, payload: MediaPayload::Structured { schema: Self::DOCUMENT_SCHEMA.to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) } })
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, EquationSnapshot>, cfg: &ConfigView<'_, NoConfig>, _view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let node = match body_key {
            MATH_PLAY_BODY_GRAPH => graph_window::render(&doc.snapshot.graph.clone(), &graph_window::config::current(cfg).cloned().unwrap_or_default().camera),
            MATH_PLAY_BODY_GEOMETRY => geometry_window::render(&doc.snapshot.geometry.clone()),
            _ => return semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        }?;
        Ok(semio_framework_plugin::built_to_component_tree(node))
    }
}
//#endregion 🔖️EquationPlayApp

//#region 🔖️Manifest
/// 🧱️ The manifest stitch: one call per taxonomy node, each sourced from that node's own `definition()`.
/// Only the leaf action/keybinding declarations (which have no dedicated `_def` passthrough) are written
/// out inline.
///
/// 🚧️ SDK GAP (contract §2.4): `EditorBuilder` has no `.example(...)`/`.workflow(...)` —
/// `PluginBuilder::editor::<E>(def: AppDefinition)` only takes the bare definition, so the old
/// `.example_source(crate::examples::demo::source())` and
/// `.workflow("equation", "Equation", "graph")` calls are dropped here (not silently: noted
/// in the migration report). The subset's own `📚️examples/🎬️demo` facet
/// (`crate::examples::...`, real content, pre-existing) is the modern,
/// role-agnostic replacement surface for example registration.
pub fn create_equation_app() -> semio_framework_plugin::AppDefinition {
    Editor::builder(EQUATION_DIALECT)
        .document(["semio", "equation"])
        .artifact_kind(crate::artifact_kind())
        .icon_id("math-app")
        .mode_def(edit::definition())
        .default_mode_id(edit::MATH_PLAY_MODE_EDIT)
        .window_kind_def(graph_window::definition())
        .window_kind_def(geometry_window::definition())
        .default_layout(edit::layout())
        .mutation("editEquation", LocalizedLabel::native("Edit Equation", "Gleichung bearbeiten"))
        .mutation("setAlgorithm", LocalizedLabel::native("Set Algorithm", "Algorithmus festlegen"))
        .mutation("setDirected", LocalizedLabel::native("Set Directed", "Gerichtet festlegen"))
        .mutation("nodeGraphEdit", LocalizedLabel::native("Node Graph Edit", "Knotengraph bearbeiten"))
        .mutation("addNode", LocalizedLabel::native("Add Node", "Knoten hinzufügen"))
        .action_with(semio_framework_plugin::ActionDefinition::new("nodeGraphViewport", LocalizedLabel::native("Node Graph Viewport", "Knotengraph-Ansicht"), semio_framework_plugin::ActionKind::View, "camera"))
        .mutation("editPoints", LocalizedLabel::native("Edit Points", "Punkte bearbeiten"))
        .action_interactive_job("editEquation", InteractiveJobClassification::Migrated)
        .action_interactive_job("setAlgorithm", InteractiveJobClassification::Migrated)
        .action_interactive_job("setDirected", InteractiveJobClassification::Migrated)
        .action_interactive_job("nodeGraphEdit", InteractiveJobClassification::Migrated)
        .action_interactive_job("addNode", InteractiveJobClassification::Migrated)
        .action_interactive_job("nodeGraphViewport", InteractiveJobClassification::Migrated)
        .action_interactive_job("editPoints", InteractiveJobClassification::Migrated)
        .action_with(semio_framework_plugin::ActionDefinition::new("setActiveExample", LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen"), semio_framework_plugin::ActionKind::Mutation, "panel-left"))
        .action_destructive("setActiveExample")
        .action_interactive_job("setActiveExample", InteractiveJobClassification::Migrated)
        .action_args("setActiveExample", vec![
            ActionArgDef::select("exampleId", LocalizedLabel::native("Example", "Beispiel"), vec![ActionArgOption::new(crate::examples::demo::ID, crate::examples::demo::label())])
                .required()
                .default_value(&crate::examples::demo::ID),
        ])
        .action_args("setAlgorithm", vec![
            ActionArgDef::select("algorithm", LocalizedLabel::native("Algorithm", "Algorithmus"), vec![
                ActionArgOption::new("topo", LocalizedLabel::native("Topological Order", "Topologische Ordnung")),
                ActionArgOption::new("components", LocalizedLabel::native("Connected Components", "Zusammenhangskomponenten")),
                ActionArgOption::new("scc", LocalizedLabel::native("Strongly Connected Components", "Starke Zusammenhangskomponenten")),
                ActionArgOption::new("bfs", LocalizedLabel::native("Breadth-First Distances", "Breitensuche-Distanzen")),
            ]).required(),
        ])
        .action_args("setDirected", vec![
            ActionArgDef::toggle("directed", LocalizedLabel::native("Directed", "Gerichtet")).default_value(&true),
        ])
        .action_args("addNode", vec![
            ActionArgDef::number("x", LocalizedLabel::native("X", "X")).required().default_value(&120.0),
            ActionArgDef::number("y", LocalizedLabel::native("Y", "Y")).required().default_value(&80.0),
        ])
        .action_args("nodeGraphEdit", vec![
            ActionArgDef::json_text("operations", LocalizedLabel::native("Operations", "Operationen"))
                .required()
                .default_value(&EQUATION_DEFAULT_EDIT_OPERATIONS),
        ])
        // WORKFLOWS-END-TO-END-TYPED-PORTS) — `equation_io()` (this file's own `🔖️Io` region) is
        .io(equation_io())
        .action_describe("editEquation", LocalizedLabel::native("Applies the supplied graph and point geometry as the concrete node, edge and point changes that differ from the equation; nothing else is touched.", "Wendet den übergebenen Graphen und die Punktgeometrie als die konkreten Knoten-, Kanten- und Punktänderungen an, die von der Gleichung abweichen; sonst wird nichts angefasst."))
        .action_describe("setAlgorithm", LocalizedLabel::native("Chooses the graph algorithm the equation evaluates: topological order, connected components, strongly connected components or breadth-first distances.", "Wählt den Graphalgorithmus, den die Gleichung auswertet: topologische Ordnung, Zusammenhangskomponenten, starke Zusammenhangskomponenten oder Breitensuche-Distanzen."))
        .action_describe("setDirected", LocalizedLabel::native("Sets whether the equation's graph is treated as directed or undirected, which changes every algorithm result.", "Legt fest, ob der Graph der Gleichung gerichtet oder ungerichtet behandelt wird, was jedes Algorithmusergebnis ändert."))
        .action_describe("nodeGraphEdit", LocalizedLabel::native("Applies a JSON list of node-graph rows (move; connect; disconnect; delete) to the equation's graph in one step.", "Wendet eine JSON-Liste von Knotengraph-Zeilen (move; connect; disconnect; delete) in einem Schritt auf den Graphen der Gleichung an."))
        .action_describe("addNode", LocalizedLabel::native("Adds one node to the equation's graph at x, y, named with the first free id.", "Fügt dem Graphen der Gleichung einen Knoten an x, y hinzu, benannt mit der ersten freien Id."))
        .action_describe("editPoints", LocalizedLabel::native("Applies the supplied points as the point repositionings, insertions and removals that differ from the equation's geometry.", "Wendet die übergebenen Punkte als die Punktverschiebungen, -einfügungen und -entfernungen an, die von der Geometrie der Gleichung abweichen."))
        .action_describe("setActiveExample", LocalizedLabel::native("Replaces the whole equation document with the bundled demo example; any other example id changes nothing.", "Ersetzt das gesamte Gleichungsdokument durch das mitgelieferte Demo-Beispiel; jede andere Beispiel-Id ändert nichts."))
        .action_audience("nodeGraphViewport", semio_framework_plugin::CapabilityAudience::Chrome)
        .build_definition()
}
//#endregion 🔖️Manifest
