//! 🧾️ Retained command contracts and work: the retained tool ids, the publication contracts, the document item reducers and the retained command work. See [`super`].

use super::*;

//#region 🧵️RetainedCommands
/// 🧵️ Every `Fem2dCommand` row, without exception — fem2d declares no host-only verb, so the retained
/// route table and `create_fem2d_app`'s `Migrated` classification list are the same nineteen ids
/// (pinned by `retained_routes_cover_every_command_exactly_once`).
pub(super) const FEM2D_RETAINED_TOOL_IDS: &[&str] = &[
    "addNode",
    "addBar",
    "addBeam",
    "addMaterial",
    "addSection",
    "addSupport",
    "addNodalLoad",
    "addMemberUdl",
    "addAreaLoad",
    "addRegion",
    "addLoadCase",
    "addCombination",
    "setSelfWeight",
    "setAnalysisSettings",
    "removeSelection",
    "setActiveExample",
    "setCamera",
    "setResultDisplay",
    "canvasPointerDown",
    "canvasPointerMove",
    "canvasPointerUp",
    "patchNode",
    "patchElement",
    "patchMaterial",
    "patchSection",
    "patchSupport",
    "patchRegion",
    "patchLoad",
    "patchLoadCase",
    "patchCombination",
    "setResultAnimation",
    "resultAnimationTick",
    "focusEntity",
    "translateSelection",
    "rotateSelection",
    "scaleSelection",
    "setTransformGumballFlag",
];

pub(super) const FEM2D_RETAINED_PAYLOAD_SCHEMA: &str = "fem.2d.tool-command.v1";

pub(super) const FEM2D_RETAINED_RAW_BYTES: usize = 65_536;

pub(super) const FEM2D_RETAINED_WORK_ITEMS: usize = 4_096;

/// 🧮️ The largest document a retained fem2d route will reduce over in one bounded first step — the
/// same number the artifact-lane store preparation admits, so a document too large for the reducer is
/// rejected before any authority is claimed rather than mid-publication.
pub(super) const FEM2D_MAXIMUM_DOCUMENT_ITEMS: usize = 4_096;

/// 🛣️ Structural commands publish artifact mutations; example loading emits a host effect.
/// Camera and result display publish only to the addressed window's configuration.
pub(super) const FEM2D_PUBLICATION_CONTRACTS: &[ArtifactToolPublicationContract] = &[
    ArtifactToolPublicationContract { tool_id: "addNode", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "addBar", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "addBeam", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "addMaterial", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "addSection", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "addSupport", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "addNodalLoad", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "addMemberUdl", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "addAreaLoad", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "addRegion", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "addLoadCase", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "addCombination", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "setSelfWeight", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "setAnalysisSettings", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "removeSelection", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "setActiveExample", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: "setCamera", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
    ArtifactToolPublicationContract { tool_id: "setResultDisplay", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
    ArtifactToolPublicationContract { tool_id: "canvasPointerDown", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: "canvasPointerMove", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: "canvasPointerUp", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: "patchNode", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "patchElement", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "patchMaterial", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "patchSection", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "patchSupport", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "patchRegion", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "patchLoad", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "patchLoadCase", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "patchCombination", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "setResultAnimation", lanes: &[ArtifactToolPublicationLane::WindowConfig, ArtifactToolPublicationLane::WindowTransient] },
    ArtifactToolPublicationContract { tool_id: "resultAnimationTick", lanes: &[ArtifactToolPublicationLane::WindowConfig, ArtifactToolPublicationLane::WindowTransient] },
    ArtifactToolPublicationContract { tool_id: "focusEntity", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
    ArtifactToolPublicationContract { tool_id: "translateSelection", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Transient] },
    ArtifactToolPublicationContract { tool_id: "rotateSelection", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Transient] },
    ArtifactToolPublicationContract { tool_id: "scaleSelection", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Transient] },
    ArtifactToolPublicationContract { tool_id: "setTransformGumballFlag", lanes: &[ArtifactToolPublicationLane::HostOnly] },
];

pub(super) fn fem2d_retained_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(FEM2D_RETAINED_RAW_BYTES, FEM2D_RETAINED_WORK_ITEMS, 1, 262_144, 7_500)
}

/// 🧮️ Every fem2d route reduces in exactly one bounded step, admitted only while the live document
/// stays inside `FEM2D_MAXIMUM_DOCUMENT_ITEMS` — an oversized document faults as "exceeds semantic
/// work capacity" instead of silently blowing the step budget.
pub(super) fn fem2d_document_items(snapshot: &Fem2dSnapshot) -> Option<usize> {
    [snapshot.nodes.len(), snapshot.elements.len(), snapshot.regions.len(), snapshot.materials.len(), snapshot.sections.len(), snapshot.supports.len(), snapshot.load_cases.len(), snapshot.combinations.len()]
        .into_iter()
        .try_fold(1usize, |total, count| total.checked_add(count))
}

fn fem2d_interaction_selection_ids(interaction: &protocol::InteractionState) -> Vec<String> {
    interaction.selection.get(FEM2D_INTERACTION_DOMAIN).map(|selection| selection.ids.clone()).unwrap_or_default()
}

fn fem2d_retained_extent(command: &Fem2dCommand, snapshot: &Fem2dSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    if !FEM2D_RETAINED_TOOL_IDS.contains(&command.command_id()) {
        return None;
    }
    fem2d_document_items(snapshot).filter(|items| *items <= FEM2D_MAXIMUM_DOCUMENT_ITEMS).map(|_| 1)
}

#[expect(clippy::too_many_arguments, reason = "Implements the framework ArtifactCommandReducer callback signature.")]
fn fem2d_retained_reduce(
    command: &Fem2dCommand,
    snapshot: &Fem2dSnapshot,
    config: &NoConfig,
    history: &semio_framework_plugin::HistoryView,
    _interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<Fem2dPlayApp>>>,
    operation: &AppOperationContext,
) -> Result<Emit<Fem2dMutation, NoConfigMutation, NoDraftMutation>, Fault> {
    if !FEM2D_RETAINED_TOOL_IDS.contains(&command.command_id()) {
        return Err(Fault::from("fem2d-command-retained-route-rejected"));
    }
    let doc = ArtifactView::with_operation(snapshot, history, operation.clone());
    let cfg = ConfigView { snapshot: config, window: context.and_then(|context| context.window_config.as_ref()) };
    let view = context.and_then(|context| context.view_state.as_ref());
    match command {
        Fem2dCommand::SetCamera(payload) => set_camera::handle_window(payload, &cfg, view.ok_or_else(|| Fault::from("fem2d.camera.window-context-required"))?),
        Fem2dCommand::SetResultDisplay(payload) => set_result_display::handle_window(payload, &cfg, view.ok_or_else(|| Fault::from("fem2d.results.window-context-required"))?),
        Fem2dCommand::SetResultAnimation(payload) => set_result_animation::handle_window(payload, &doc, &cfg, view.ok_or_else(|| Fault::from("fem2d.results.window-context-required"))?),
        Fem2dCommand::ResultAnimationTick(payload) => result_animation_tick::handle_window(payload, &doc, &cfg, view.ok_or_else(|| Fault::from("fem2d.results.window-context-required"))?),
        Fem2dCommand::CanvasPointerDown(payload) => canvas_pointer_down::handle_window(payload, &doc, &cfg, view.ok_or_else(|| Fault::from("fem2d.pointer.window-context-required"))?),
        Fem2dCommand::CanvasPointerMove(payload) => canvas_pointer_move::handle_window(payload, &doc, &cfg, view.ok_or_else(|| Fault::from("fem2d.pointer.window-context-required"))?),
        Fem2dCommand::CanvasPointerUp(payload) => canvas_pointer_up::handle_window(payload, &doc, &cfg, view.ok_or_else(|| Fault::from("fem2d.pointer.window-context-required"))?),
        Fem2dCommand::FocusEntity(payload) => focus_entity::handle_window(payload, &doc, &cfg, view.ok_or_else(|| Fault::from("fem2d.camera.window-context-required"))?),
        Fem2dCommand::SetTransformGumballFlag(payload) => set_transform_gumball_flag::handle_window(payload, view.ok_or_else(|| Fault::from("fem2d.gumball-flag.window-context-required"))?),
        _ => command.dispatch(&doc, &cfg),
    }
}

/// 🧵️ The one bounded work every fem2d route runs: the gumball verbs drive the gumball tool (committing a transaction
/// and publishing the window's next transient); the two playback commands publish the results window's TRANSIENT lane on
/// top of the config — the running clock the runtime captured from the command's own `window_id`
/// (`retained_window_transient_target`) — as `CompleteWithEphemeral`; every other route reduces once through
/// [`fem2d_retained_reduce`].
pub(super) struct Fem2dRetainedWork {
    pub(super) tool_id: &'static str,
    pub(super) consumed: bool,
}

impl semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<Fem2dPlayApp>> for Fem2dRetainedWork {
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    fn extent(&self, command: &Fem2dCommand, snapshot: &Fem2dSnapshot, interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<Fem2dPlayApp>>>) -> Option<usize> {
        fem2d_retained_extent(command, snapshot, interaction)
    }

    fn step(&mut self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<Fem2dPlayApp>>, _cx: &mut semio_framework_job::StepContext<'_>) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<Fem2dPlayApp>>, semio_framework_job::OriginalTurnFailure> {
        let original: Result<_, Fault> = (|| {
        use semio_s_artifact_fem_2d_editor_interaction::editor::fem2d::interaction::gumball::Fem2dGumballMotion;
        if self.consumed {
            return Err(Fault::from("fem2d-retained-work-repeated"));
        }
        self.consumed = true;
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { snapshot_owner: _, command, snapshot, config, history, interaction, hover, context, operation } = *input;
        let selected = fem2d_interaction_selection_ids(interaction);
        match command {
            Fem2dCommand::TranslateSelection(payload) => crate::editor::fem2d::commands::gumball::gumball_step("translateSelection", Fem2dGumballMotion::Translate { dx: payload.dx, dy: payload.dy }, &payload.ids, payload.phase.as_deref(), payload.reason.as_deref(), snapshot, &selected, context, operation),
            Fem2dCommand::RotateSelection(payload) => crate::editor::fem2d::commands::gumball::gumball_step("rotateSelection", Fem2dGumballMotion::Rotate { angle: payload.angle }, &payload.ids, payload.phase.as_deref(), payload.reason.as_deref(), snapshot, &selected, context, operation),
            Fem2dCommand::ScaleSelection(payload) => crate::editor::fem2d::commands::gumball::gumball_step("scaleSelection", Fem2dGumballMotion::Scale { sx: payload.sx, sy: payload.sy }, &payload.ids, payload.phase.as_deref(), payload.reason.as_deref(), snapshot, &selected, context, operation),
            Fem2dCommand::ResultAnimationTick(payload) => {
                let cfg = ConfigView { snapshot: config, window: context.and_then(|context| context.window_config.as_ref()) };
                let view = context.and_then(|context| context.view_state.as_ref()).ok_or_else(|| Fault::from("fem2d.result-animation-tick.window-context-required"))?;
                let clock = results_window::transient::required_clock::<results_window::transient::Fem2dResultsWindowTransientOwner>(context.and_then(|context| context.window_transient.as_ref()), &payload.window_id)?;
                let step = result_animation_tick::result_animation_tick_step::<set_result_animation::Fem2dResultsPlayback>(payload, &cfg, view, clock)?;
                Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::CompleteWithEphemeral { emit: step.emit, ephemeral: semio_framework_plugin::EphemeralEmit { presence: Vec::new(), transient: Vec::new(), window_transient: step.window_transient } })
            }
            Fem2dCommand::SetResultAnimation(payload) => {
                let cfg = ConfigView { snapshot: config, window: context.and_then(|context| context.window_config.as_ref()) };
                let view = context.and_then(|context| context.view_state.as_ref()).ok_or_else(|| Fault::from("fem2d.result-animation.window-context-required"))?;
                let clock = payload.window_id.as_deref().filter(|id| !id.is_empty()).and_then(|id| context.and_then(|context| results_window::transient::captured_clock::<results_window::transient::Fem2dResultsWindowTransientOwner>(context.window_transient.as_ref(), id)));
                let step = set_result_animation::set_result_animation_step::<set_result_animation::Fem2dResultsPlayback>(payload, &cfg, view, clock)?;
                Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::CompleteWithEphemeral { emit: step.emit, ephemeral: semio_framework_plugin::EphemeralEmit { presence: Vec::new(), transient: Vec::new(), window_transient: step.window_transient } })
            }
            _ => fem2d_retained_reduce(command, snapshot, config, history, interaction, hover, context, operation).map(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete),
        }
    
        })();
        original.map_err(semio_framework_job::OriginalTurnFailure::from)
    }
}
//#endregion 🧵️RetainedCommands
