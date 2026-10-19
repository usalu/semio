//! 🖼️ Play app and retained command contracts: the play app, the viewport and camera parsers, the render body, the retained tool contracts and the preview command work. See [`super`].

use super::*;

//#region 🔖️Generation3dPlayApp
/// 🧪️ Unit struct: the retained evaluation session lives in [`Generation3dInstanceOperationOwner`]
/// and every other former runtime field lives in [`Generation3dConfig`], written through
/// [`Generation3dConfigMutation`]s.
#[derive(Default)]
pub struct Generation3dPlayApp;

/// 🎥️ Parses the flow-graph camera out of `command_from_action`'s JSON args' nested `{viewport: {x, y, zoom}}`
/// object (`nodeGraphViewportActionArgs`, `🧱️elements/🕸️NodeGraph/🟦️.tsx`). An ABSENT `viewport` decodes to
/// the identity camera, because `ActionDefinition::new("nodeGraphViewport", …)` declares no args and every
/// declared action must bridge from its own id under the shell's staged args alone — the same contract
/// `parse_preview_camera_json` keeps for `setCamera`. A PRESENT but malformed one still faults: a camera the
/// graph cannot express must never be silently replaced by one it can.
pub(super) fn parse_flow_viewport(args: &semio_framework_value::DslValue) -> Result<semio_framework_os_kernel::Viewport2d, Fault> {
    let Some(value) = args.get("viewport").cloned() else { return Ok(semio_framework_os_kernel::Viewport2d::default()) };
    semio_framework_value::FromValue::from_value(value).map_err(|error| Fault::from(format!("invalid nodeGraphViewport viewport: {error}")))
}

/// 🎥️ Parses the 3D preview camera out of `command_from_action`'s JSON args; falls back to the default
/// camera on any malformed/missing `camera` object.
pub(super) fn parse_preview_camera_json(args: &semio_framework_value::DslValue) -> semio_s_artifact_procedural_3d_editor_config::Generation3dPreviewCamera {
    if let Some(camera) = args.get("camera") {
        if let Ok(parsed) = <semio_s_artifact_procedural_3d_editor_config::Generation3dPreviewCamera as semio_framework_value::FromValue>::from_value(camera.clone()) {
            return parsed;
        }
    }
    semio_s_artifact_procedural_3d_editor_config::Generation3dPreviewCamera::default()
}

/// 🕸️ Every node's visible port ids (`{nodeId}@{portId}`), read from the SAME
/// `dag_host_snapshot_to_workflow` projection the node-graph window paints — so an interaction target and a
/// graph pick can never drift apart.
pub fn generation3d_port_ids_by_node(host_snapshot: &semio_framework_artifact_flow_flow::FlowHostSnapshot) -> std::collections::BTreeMap<String, Vec<String>> {
    let (graph_nodes, _) = semio_s_artifact_procedural_3d_model::with_host(host_snapshot, |host| semio_s_artifact_procedural_3d_preview_eval::dag_host_snapshot_to_workflow(&host.dag.host_snapshot));
    graph_nodes.into_iter().map(|node| (node.id, node.inputs.into_iter().chain(node.outputs).map(|port| port.id).collect())).collect()
}

pub(super) fn generation3d_addressed_preview_eval(window: &semio_framework_plugin::WindowTransientSnapshot, eval_text: Option<String>) -> Result<semio_framework_plugin::WindowTransientMutation, Fault> {
    if window.window_kind_id() == generate_preview::GENERATION_3D_PLAY_WINDOW_GENERATE_PREVIEW {
        generate_preview::transient::addressed(window, eval_text)
    } else {
        edit_preview::transient::addressed(window, eval_text)
    }
}

/// 🧱️ Every window body of the generation3d editor, rendered against one already-resolved set of
/// `graph` marks. Shared by `render` (marks-free) and `render_with_request_context` (live marks) so
/// there is exactly one body-key match in the app.
pub fn generation3d_render_body(
    body_key: &str,
    document: &Generation3dSnapshot,
    config: &Generation3dConfig,
    preview_eval_text: Option<&str>,
    view_state: &semio_framework_plugin::ViewModel,
    marks: &PreviewInteractionMarks,
    session: &FlowEvalSession,
    run: Option<&semio_framework_plugin::ToolRunView>,
) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
    let labels = generation3d_labels(view_state);
    let active_utility = view_state.active_utility_id.as_deref().unwrap_or("move");
    let selected_generation_id = config.selected_generation_id.as_deref();
    if semio_framework_job::runtime_diagnostics_enabled() {
        eprintln!("[TRACE] gen3d render body={body_key} generations={} selected={selected_generation_id:?}", document.generation.as_state().generations.len());
    }
    let node = match body_key {
        flow_window::GENERATION_3D_PLAY_BODY_MAIN => flow_window::render(document, config, session, marks, labels),
        edit_preview::GENERATION_3D_PLAY_BODY_PREVIEW => edit_preview::render(document, config, preview_eval_text, session, run, active_utility, marks, labels),
        generations::GENERATION_3D_PLAY_BODY_GENERATIONS => generations::render(
            &document.generation,
            selected_generation_id,
            view_state.locale,
            view_state.terminology,
            &semio_framework_plugin::TreeWindows::for_body(view_state, generations::GENERATION_3D_PLAY_BODY_GENERATIONS),
        ),
        form::GENERATION_3D_PLAY_BODY_GENERATE_FORM => form::render(&document.host_snapshot, &document.generation, selected_generation_id, labels),
        generate_preview::GENERATION_3D_PLAY_BODY_GENERATE_PREVIEW => generate_preview::render(&document.host_snapshot, &document.generation, selected_generation_id, preview_eval_text, config, labels, active_utility, marks, session, run),
        artifact_panel::GENERATION_3D_PLAY_BODY_ARTIFACT => artifact_panel::render(document, config, session, labels, &semio_framework_plugin::TreeWindows::for_body(view_state, artifact_panel::GENERATION_3D_PLAY_BODY_ARTIFACT)),
        catalogue_panel::GENERATION_3D_PLAY_BODY_CATALOGUE => catalogue_panel::render(labels, &semio_framework_plugin::TreeWindows::for_body(view_state, catalogue_panel::GENERATION_3D_PLAY_BODY_CATALOGUE)),
        inspection_panel::GENERATION_3D_PLAY_BODY_INSPECTION => {
            let evaluation = session.eval_json();
            let selected = marks.graph_selection_ids();
            let export_widget = selected.first().filter(|id| document.host_snapshot.widgets.iter().any(|widget| matches!(widget, semio_framework_artifact_flow_flow::Widget::OutputExport { id: target, .. } if target == *id)));
            let exported = match export_widget { Some(widget) => export_meshes_from_session(document, config, session, Some(widget.as_str())), None => Ok(Vec::new()) };
            let export_ready = exported.is_ok();
            let meshes = exported.unwrap_or_default();
            inspection_panel::render(&document.host_snapshot, &marks.graph_selection_ids(), labels, &semio_framework_plugin::TreeWindows::for_body(view_state, inspection_panel::GENERATION_3D_PLAY_BODY_INSPECTION), view_state.locale, view_state.terminology, &meshes, evaluation, session.status_json(), export_ready,marks.source_revision.as_deref())
        },
        _ => semio_framework_plugin::built_text_node(Label::data(format!("Unknown body: {body_key}"))).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.unknown-body", "fixed UI unknown-body admission failed")),
    }?;
    Ok(semio_framework_plugin::built_to_component_tree(node))
}

//#region 🧵️RetainedCommands
/// 🧾️ Every gen3d GESTURE tool id, in `Generation3dCommand` declaration order. Together with
/// [`GENERATION3D_FLOW_EVAL_TOOL_IDS`] and [`GENERATION3D_CONTRIBUTIONS_TOOL_IDS`] this is a
/// bijection with `Generation3dCommand`'s 34 rows (asserted by
/// `retained_route_dispositions_are_exact_and_exhaustive` below) and with
/// `Generation3dBoundedCommandJobFactory::PUBLICATION_CONTRACTS`.
pub const GENERATION3D_RETAINED_TOOL_IDS: &[&str] = &[
    "setActiveExample",
    "nodeGraphEdit",
    "deleteSelection",
    "removeWidget",
    "addWidget",
    "patchFlowWidgets",
    "setWidgetInput",
    "reorganize",
    "translateSelection",
    "rotateSelection",
    "scaleSelection",
    "addGeneration",
    "removeGeneration",
    "renameGeneration",
    "updateGenerationValues",
    "nodeGraphViewport",
    "setLodMode",
    "setShowMode",
    "toggleSun",
    "setSunAzimuth",
    "setSunElevation",
    "setSunIntensity",
    "setCamera",
    "selectGeneration",
    "cycleShowMode",
    "cycleLodMode",
    "selectNextNode",
    "selectPreviousNode",
    "selectUpstreamNode",
    "selectDownstreamNode",
    "activateSelection",
    "editMeshSelection",
    "knifeMeshSelection",
];

/// ⏱️ The preview run's hop tool ids — a HOST route, never a user gesture: nobody clicks a tick,
/// and the mesh body an extension answer carries is nothing like a gesture's wire payload. Split
/// out of [`GENERATION3D_RETAINED_TOOL_IDS`] for exactly the reason
/// [`GENERATION3D_CONTRIBUTIONS_TOOL_IDS`] was, and in exactly the shape the sibling viewer surface
/// already declares (`GENERATION3D_VIEW_FLOW_EVAL_TOOL_IDS`): one factory registers ONE execution
/// contract for all its keys, so a chain that must carry 48 KiB of `pack` body cannot live under
/// the 8 KiB gesture quota without widening 24 unrelated routes with it
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️preview-mesh-delivery-2026-09-12.md`).
pub const GENERATION3D_FLOW_EVAL_TOOL_IDS: &[&str] = &["flowEvalTick", "flowEvalResolve", "flowTessellateResolve", "flowEvalRelease", "flowTessellateCancelResolve"];

/// 🪟️ The chain routes that are ADDRESSED AT A PREVIEW WINDOW and therefore share one publication
/// layer ([`Generation3dFlowEvalWindowWork`]): the dispatched hop and the two extension folds that
/// continue the chain inline. `flowEvalRelease` and `flowTessellateCancelResolve` stay off it on
/// purpose — a release must reach the geometry extension even once the window it names is gone, and
/// `retained_window_transient_target` refuses a window that has left the roster.
pub(super) const GENERATION3D_FLOW_EVAL_WINDOW_TOOL_IDS: &[&str] = &["flowEvalTick", "flowEvalResolve", "flowTessellateResolve"];

/// 📄️ The user's OWN import/export route: its own tool ids, its own factory and its own execution
/// contract, for the same reason the preview chain has one — what it carries is nothing like a
/// gesture. `importDocument` hands over one WHOLE picked file — reassembled by the framework
/// (`semio_framework::kernel::ImportStaging`) and bounded by one Artifact-lane edit. Widening the
/// 8 KiB gesture quota to admit it would widen 23 unrelated interactive routes with it
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, io-surface lane).
pub const GENERATION3D_DOCUMENT_IO_TOOL_IDS: &[&str] = &["importDocumentRequest", "importDocument", "exportDocument"];

pub(super) const GENERATION3D_DOCUMENT_IO_PAYLOAD_SCHEMA: &str = "generation.3d.document-io-command.v1";

/// 🎒️ Real bound for one document-IO hop's wire payload: one whole import
/// ([`import_document::GENERATION3D_IMPORT_TOTAL_BYTES`]) plus the addressed envelope and the command
/// id — the whole public-invocation body the host may send, pinned to
/// `PUBLIC_INVOCATION_BODY_BYTES` by `document_io_route_declares_a_reachable_wire_ceiling` rather
/// than guessed.
pub const GENERATION3D_DOCUMENT_IO_RAW_BYTES: usize = semio_framework::PUBLIC_INVOCATION_BODY_BYTES;

pub(super) const GENERATION3D_RETAINED_PAYLOAD_SCHEMA: &str = "generation.3d.tool-command.v1";

pub(super) const GENERATION3D_FLOW_EVAL_PAYLOAD_SCHEMA: &str = "generation.3d.flow-eval-command.v1";

pub const GENERATION3D_RETAINED_RAW_BYTES: usize = 8_192;

/// 🎒️ Real bound for one preview-chain hop's wire payload: a `flowTessellateResolve` carries one
/// whole `tessellate` envelope, i.e. one [`MESH_PACK_CHUNK_BASE64_CHARS`] mesh-body chunk plus its
/// accounting header — pinned to `brep_geometry::tessellate_envelope_maximum_bytes()` by
/// `tessellate_envelope_fits_the_declared_wire_bound`, so the transfer unit and this bound are one
/// fact. 64 KiB is the guest-contiguous page the answer already crosses in, never a rubber stamp.
pub const GENERATION3D_FLOW_EVAL_RAW_BYTES: usize = 65_536;

pub const GENERATION3D_PREVIEW_TOOL_IDS: &[&str] = &["setActiveExample", "addGeneration", "removeGeneration", "renameGeneration", "updateGenerationValues", "selectGeneration"];

/// 🧮️ The ONE declared quantity of every generation3d retained route: at most 32 point-invertible
/// durable items per gesture. Everything the runtime compares is read off it and nothing is ever
/// spelled again — the preflight ceiling ([`GENERATION3D_RETAINED_WORK_ITEMS`], handed to
/// `ArtifactRetainedCommandPayload::new`), the extent every `ArtifactCommandWork` answers, the
/// one-item store footprint both durable preflights declare, and the bounded proof's work units.
/// They used to be three literals in two units — a two-row footprint, a one-ITEM extent and a
/// 32-ITEM ceiling — so nothing could compare them and only the store ever measured rows
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
pub const GENERATION3D_RETAINED_CAPACITY: ArtifactRetainedWorkCapacity = ArtifactRetainedWorkCapacity::for_invertible_items(32);

pub const GENERATION3D_RETAINED_WORK_ITEMS: usize = GENERATION3D_RETAINED_CAPACITY.work_items();

/// 🧾️ The ONE execution contract all 29 routes publish — the factory's `execution_contract()` and
/// every row of `bounded_first_step_tool_proofs!` alike, so the proof catalogue can never drift from
/// the factory it proves. Its work-unit budget is the route capacity itself; `max_decoded_items`
/// stays its own quantity (decoded JSON items on the wire, not staged edit rows).
pub fn generation3d_bounded_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(GENERATION3D_RETAINED_RAW_BYTES, GENERATION3D_RETAINED_DECODED_ITEMS, GENERATION3D_RETAINED_WORK_ITEMS as u64, 16_384, 7_500)
}

/// 🧾️ Decoded JSON items one retained command's args may carry — a WIRE bound, deliberately not the
/// work capacity.
pub(super) const GENERATION3D_RETAINED_DECODED_ITEMS: usize = 32;

/// 🪟️ Every attached preview window, in roster order — the windows a `flowEvalTick` chain can be
/// addressed to. Each holds its OWN retained evaluation publication
/// (`Generation3dPreviewWindowTransientOwner`), so each gets its own armed chain; an empty answer
/// means no surface is mounted yet and nothing may be armed at all.
pub(super) fn generation3d_preview_kind(kind: &str) -> Option<&'static str> {
    if kind == edit_preview::GENERATION_3D_PLAY_WINDOW_PREVIEW {
        Some(edit_preview::GENERATION_3D_PLAY_WINDOW_PREVIEW)
    } else if kind == generate_preview::GENERATION_3D_PLAY_WINDOW_GENERATE_PREVIEW {
        Some(generate_preview::GENERATION_3D_PLAY_WINDOW_GENERATE_PREVIEW)
    } else {
        None
    }
}

pub(super) fn generation3d_preview_windows(view: Option<&semio_framework_plugin::ViewModel>) -> Vec<(&str, &'static str)> {
    view.map(|view| {
        view.window_instances
            .iter()
            .filter_map(|window| generation3d_preview_kind(&window.window_kind_id).map(|kind| (window.id.as_str(), kind)))
            .collect()
    })
    .unwrap_or_default()
}

pub fn generation3d_bounded_extent(_command: &Generation3dCommand, _snapshot: &Generation3dSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    GENERATION3D_RETAINED_CAPACITY.rows_for_items(1)
}

pub struct Generation3dPreviewCommandWork {
    pub(super) tool_id: &'static str,
    /// 🔒️ The app instance's retained evaluation session, reached ONLY to arm the preview chain
    /// through its per-window latch — a gesture that changed what the evaluation would produce owes
    /// every attached preview a tick, and owes it at most once.
    pub(super) instance_owner: semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
    pub(super) emit: Option<Emit<Generation3dMutation, Generation3dConfigMutation, NoDraftMutation>>,
    pub(super) host: Option<FlowHost>,
    /// 🧹️ The host's explicit retirement ladder, armed by `close_step` when the evaluation host is
    /// handed over. A bare `Option<FlowHost>::take()`-and-drop panics on the fixture's
    /// `OrderedMap<WidgetLayout>` root, so the host is drained under the caller's grant instead.
    pub(super) host_retirement: Option<semio_framework_os_flow::FlowHostRetirement>,
    pub(super) session: Option<FlowEvalSession>,
    pub(super) emit_retirement: Option<semio_framework_value::retirement::controlled::ControlledRetirement<Emit<Generation3dMutation, Generation3dConfigMutation, NoDraftMutation>>>,
    pub(super) started: bool,
    pub(super) complete: bool,
    pub(super) closing: bool,
}

impl Generation3dPreviewCommandWork {
    pub(super) fn new(tool_id: &'static str, instance_owner: semio_framework_plugin::ArtifactInstanceOperationOwnerHandle) -> Self {
        Self { tool_id, instance_owner, emit: None, host: None, host_retirement: None, session: None, emit_retirement: None, started: false, complete: false, closing: false }
    }

    /// 🔒️ Owes every attached preview window an evaluation on the RETAINED session's latch, so two
    /// gestures in a row still leave exactly one debt per window for the run to pay.
    pub(super) fn owe_attached_previews(&self, view: Option<&semio_framework_plugin::ViewModel>, servable: bool, emit: &mut Emit<Generation3dMutation, Generation3dConfigMutation>, grant: semio_framework_value::RetainedCloneGrant) -> Result<(), Fault> {
        let windows = generation3d_preview_windows(view);
        self.instance_owner.with_mut::<Generation3dInstanceOperationOwner, _>(|owner| owner.owe_attached_previews_carrying(&windows, servable, emit, grant))
    }

    pub(super) fn close_demands(&self, copy: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        use semio_framework_value::RetirementDemand;
        let leaf = RetirementDemand { depth: 1, ..Default::default() };
        if let Some(session) = self.session.as_ref() {
            return if session.terminal_is_empty() { Ok(leaf) } else { Ok(RetirementDemand { copy_bytes: session.next_close_copy_byte_demand()?, capacity_bytes: session.next_close_capacity_byte_demand(copy)?, release_bytes: session.next_close_release_byte_demand()?, depth: session.next_close_depth_demand()? }) };
        }
        if self.host.is_some() || self.emit.is_some() {
            return Ok(leaf);
        }
        if let Some(retirement) = self.host_retirement.as_ref() {
            return if retirement.terminal_is_empty() { Ok(leaf) } else { Ok(RetirementDemand { copy_bytes: retirement.next_close_copy_byte_demand()?, capacity_bytes: retirement.next_close_capacity_byte_demand(copy)?, release_bytes: retirement.next_close_release_byte_demand()?, depth: retirement.next_close_depth_demand()? }) };
        }
        if let Some(retirement) = self.emit_retirement.as_ref() {
            return Ok(RetirementDemand { copy_bytes: retirement.next_copy_byte_demand()?, capacity_bytes: retirement.next_capacity_byte_demand(copy)?, release_bytes: retirement.next_release_byte_demand()?, depth: retirement.next_depth_demand()? });
        }
        Ok(Default::default())
    }

    pub(super) fn complete(&mut self) -> Result<ArtifactCommandWorkStep<EditorApp<Generation3dPlayApp>>, Fault> {
        self.complete = true;
        let emit = self.emit.take().ok_or_else(|| Fault::from("generation3d-preview-emit-owner-absent"))?;
        Ok(ArtifactCommandWorkStep::Complete(emit))
    }
}

/// 🧮️ The transient bytes one work step may materialize: the work's own frame plus the one emit it can stage.
pub(super) fn command_work_step_demands<A: semio_framework_plugin::ArtifactApp>(work_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
    Ok(semio_framework_value::RetirementDemand { copy_bytes: work_bytes.saturating_add(size_of::<Emit<A::Mutation, A::ConfigMutation, A::DraftMutation>>()), depth: 1, ..Default::default() })
}
//#endregion 🧵️RetainedCommands
//#endregion 🔖️Generation3dPlayApp
