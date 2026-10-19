//! 🗃️ Retained commands of the FEM 3D editor: the tool ids, the publication contracts, the command work and the retained command job factory. See [`super`].

use super::*;

//#region 🧵️RetainedCommands
/// 🧾️ Every fem3d tool id, in `Fem3dCommand` declaration order — a bijection with the enum's rows,
/// with `FEM3D_RETAINED_PUBLICATION_CONTRACTS`, and with the `.action_interactive_job(…, Migrated)` set
/// `create_fem3d_app` declares. `AppActionRegistry::tool_job_registration` enforces exactly that set
/// equality at construction time: a row missing here, or an action left `BatchOnlyPendingRewrite`,
/// faults the whole app with `interactive-job.catalog-incomplete` instead of silently going
/// dispatch-dead at the UI gate — which is what the pre-migration two-row list did to the other 16.
pub(super) const FEM3D_RETAINED_TOOL_IDS: &[&str] = &[
    "addNode",
    "addBar",
    "addFrame",
    "addMaterial",
    "addSection",
    "addSupport",
    "addNodalLoad",
    "addMemberUdl",
    "addAreaLoad",
    "addSolid",
    "addLoadCase",
    "addCombination",
    "setSelfWeight",
    "setAnalysisSettings",
    "removeSelection",
    "setActiveExample",
    "setCamera",
    "setResultDisplay",
    "patchNode",
    "patchElement",
    "patchMaterial",
    "patchSection",
    "patchSupport",
    "patchSolid",
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

const FEM3D_RETAINED_PAYLOAD_SCHEMA: &str = "fem.3d.tool-command.v1";

pub(super) const FEM3D_RETAINED_RAW_BYTES: usize = 65_536;

pub(super) const FEM3D_RETAINED_DECODED_ITEMS: usize = 4_096;

pub(super) const FEM3D_RETAINED_OUTPUT_BYTES: usize = 262_144;

const FEM3D_RETAINED_STEP_MICROS: u32 = 7_500;

/// 🎒️ Semantic work ceiling for one retained fem3d command: the document's own id-keyed collections
/// (`removeSelection` walks every one of them), so a model beyond this size falls out of the retained
/// path with `fem3d-command-payload-too-large` rather than blocking the interactive step budget.
pub(super) const FEM3D_RETAINED_WORK_ITEMS: usize = 4_096;

pub(super) const FEM3D_WINDOW_COMMAND_VALUE_BYTES: usize = 512;

/// 🎒️ Real bound for one Artifact-lane edit: the largest single `Fem3dMutation` any of the 15 document
/// tools emits is `CreateSolid` (outline + holes polygons) — 64 KiB is a genuine ceiling for that
/// encoded op, not a rubber stamp.
pub(super) const FEM3D_ARTIFACT_STORE_MAXIMUM_BYTES: usize = 65_536;

/// 🧾️ Document commands publish artifact mutations; example loading emits a host effect.
/// Camera and result display publish only to the addressed window's configuration. The gumball verbs
/// publish a committed transaction into the artifact and the dispatching window's open gesture into the
/// FEM gumball transient.
pub(super) const FEM3D_RETAINED_PUBLICATION_CONTRACTS: &[ArtifactToolPublicationContract] = &[
    ArtifactToolPublicationContract { tool_id: "addNode", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "addBar", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "addFrame", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "addMaterial", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "addSection", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "addSupport", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "addNodalLoad", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "addMemberUdl", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "addAreaLoad", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "addSolid", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "addLoadCase", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "addCombination", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "setSelfWeight", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "setAnalysisSettings", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "removeSelection", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "setActiveExample", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: "setCamera", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
    ArtifactToolPublicationContract { tool_id: "setResultDisplay", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
    ArtifactToolPublicationContract { tool_id: "patchNode", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "patchElement", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "patchMaterial", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "patchSection", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "patchSupport", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "patchSolid", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "patchLoad", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "patchLoadCase", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "patchCombination", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "setResultAnimation", lanes: &[ArtifactToolPublicationLane::WindowConfig, ArtifactToolPublicationLane::WindowTransient] },
    ArtifactToolPublicationContract { tool_id: "resultAnimationTick", lanes: &[ArtifactToolPublicationLane::WindowConfig, ArtifactToolPublicationLane::WindowTransient] },
    ArtifactToolPublicationContract { tool_id: "focusEntity", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
    ArtifactToolPublicationContract { tool_id: "translateSelection", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Transient] },
    ArtifactToolPublicationContract { tool_id: "rotateSelection", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Transient] },
    ArtifactToolPublicationContract { tool_id: "scaleSelection", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Transient] },
    ArtifactToolPublicationContract { tool_id: "setTransformGumballFlag", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
];

/// 🧷️ The single source of truth for this app's tool execution contract — `bounded_first_step_tool_proofs!`
/// calls this same fn, so the declared proof row and the registered factory can never disagree (they did
/// before: the proof declared `bounded_first_step` while the factory returned `resumable`, which made
/// `validate_tool_job_rows`' `registration.contract == row.contract` check reject even `setCamera`).
pub(super) fn fem3d_retained_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(FEM3D_RETAINED_RAW_BYTES, FEM3D_RETAINED_DECODED_ITEMS, 1, FEM3D_RETAINED_OUTPUT_BYTES, FEM3D_RETAINED_STEP_MICROS)
}

/// 📏️ Fixed orbit pose bytes or result-selection text carried by exact-window commands.
fn fem3d_retained_window_value_bytes(command: &Fem3dCommand) -> usize {
    match command {
        Fem3dCommand::SetCamera(payload) => size_of_val(&payload.camera),
        Fem3dCommand::SetResultDisplay(payload) => payload.mode.len().saturating_add(payload.source_id.as_ref().map_or(0, String::len)).saturating_add(payload.value.as_ref().map_or(0, String::len)),
        Fem3dCommand::SetResultAnimation(payload) => payload.value.as_ref().map_or(0, String::len).saturating_add(payload.loop_mode.as_ref().map_or(0, String::len)).saturating_add(payload.waveform.as_ref().map_or(0, String::len)),
        Fem3dCommand::SetTransformGumballFlag(payload) => payload.flag.len(),
        Fem3dCommand::ResultAnimationTick(payload) => payload.window_id.len(),
        _ => 0,
    }
}

/// 🕹️ The live framework-owned `fem3d` selection a transform or delete falls back to when its payload
/// names no ids of its own — the host's gumball and the delete keybinding both speak for "what is selected".
fn fem3d_interaction_selection_ids(interaction: &protocol::InteractionState) -> Vec<String> {
    interaction.selection.get(FEM3D_INTERACTION_DOMAIN).map(|selection| selection.ids.clone()).unwrap_or_default()
}

/// 🕹️ Routes the window-scoped and selection-scoped rows, and every other row to its own handler —
/// the ONE dispatch table `handle` and the retained reducer share, so the two paths cannot drift.
pub(crate) fn fem3d_route(command: &Fem3dCommand, doc: &ArtifactView<'_, Fem3dSnapshot>, cfg: &ConfigView<'_, NoConfig>, selection: impl FnOnce() -> Vec<String>, view: Option<&ViewModel>) -> Result<Emit<Fem3dMutation, NoConfigMutation>, Fault> {
    let window = |fault: &str| view.ok_or_else(|| Fault::from(format!("fem3d.{fault}.window-context-required")));
    match command {
        Fem3dCommand::SetCamera(payload) => set_camera::handle_window(payload, cfg, window("camera")?),
        Fem3dCommand::SetResultDisplay(payload) => set_result_display::handle_window(payload, cfg, window("results")?),
        Fem3dCommand::SetResultAnimation(payload) => set_result_animation::handle_window(payload, doc, cfg, window("results")?),
        Fem3dCommand::ResultAnimationTick(payload) => result_animation_tick::handle_window(payload, doc, cfg, window("results")?),
        Fem3dCommand::FocusEntity(payload) => focus_entity::handle_window(payload, doc, cfg, window("focus-entity")?),
        Fem3dCommand::SetTransformGumballFlag(payload) => set_transform_gumball_flag::handle_window(payload, cfg, window("gumball-flag")?),
        Fem3dCommand::RemoveSelection(payload) if payload.ids.is_empty() => remove_selection::handle(&remove_selection::RemoveSelection { ids: selection() }, doc, cfg),
        Fem3dCommand::TranslateSelection(payload) if payload.ids.is_empty() => translate_selection::handle(&translate_selection::TranslateSelection { ids: selection(), ..payload.clone() }, doc, cfg),
        Fem3dCommand::RotateSelection(payload) if payload.ids.is_empty() => rotate_selection::handle(&rotate_selection::RotateSelection { ids: selection(), ..payload.clone() }, doc, cfg),
        Fem3dCommand::ScaleSelection(payload) if payload.ids.is_empty() => scale_selection::handle(&scale_selection::ScaleSelection { ids: selection(), ..payload.clone() }, doc, cfg),
        _ => command.dispatch(doc, cfg),
    }
}

pub(super) fn fem3d_retained_extent(command: &Fem3dCommand, snapshot: &Fem3dSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    if !FEM3D_RETAINED_TOOL_IDS.contains(&command.command_id()) || fem3d_retained_window_value_bytes(command) > FEM3D_WINDOW_COMMAND_VALUE_BYTES {
        return None;
    }
    let collections = [snapshot.nodes.len(), snapshot.elements.len(), snapshot.materials.len(), snapshot.sections.len(), snapshot.solids.len(), snapshot.supports.len(), snapshot.load_cases.len(), snapshot.combinations.len()];
    let items = collections.into_iter().try_fold(1usize, |total, count| total.checked_add(count))?;
    (items <= FEM3D_RETAINED_WORK_ITEMS).then_some(1)
}

/// 🎯️ One reducer for every row: `Fem3dCommand::dispatch` already routes each row to its own
/// `🎮️commands/*` handler, so the retained job reuses the exact same owned reducers the batch path
/// used — no second, drifting copy of any command body.
#[expect(clippy::too_many_arguments, reason = "The retained command reducer implements the framework's eight-argument callback contract.")]
fn fem3d_retained_reduce(
    command: &Fem3dCommand,
    snapshot: &Fem3dSnapshot,
    config: &NoConfig,
    history: &semio_framework_plugin::HistoryView,
    interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<Fem3dPlayApp>>>,
    operation: &AppOperationContext,
) -> Result<Emit<Fem3dMutation, NoConfigMutation, NoDraftMutation>, Fault> {
    let doc = ArtifactView::with_operation(snapshot, history, operation.clone());
    let cfg = ConfigView { snapshot: config, window: context.and_then(|context| context.window_config.as_ref()) };
    let view = context.and_then(|context| context.view_state.as_ref());
    fem3d_route(command, &doc, &cfg, || fem3d_interaction_selection_ids(interaction), view)
}

/// 🧵️ The one retained work every fem3d row runs: the shared reducer for every command; for the
/// two playback commands the results window's TRANSIENT lane on top — the running clock the
/// runtime captured from the command's own `window_id` (`retained_window_transient_target`) is
/// read here and the frame is published as `CompleteWithEphemeral`; and the gumball verbs drive the
/// gumball tool, committing a transaction and publishing the window's next FEM gumball transient.
pub(super) struct Fem3dCommandWork {
    pub(super) tool_id: &'static str,
    pub(super) consumed: bool,
}

impl Fem3dCommandWork {
    pub(super) fn new(tool_id: &'static str) -> Self {
        Self { tool_id, consumed: false }
    }
}

impl ArtifactCommandWork<EditorApp<Fem3dPlayApp>> for Fem3dCommandWork {
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    fn extent(&self, command: &Fem3dCommand, snapshot: &Fem3dSnapshot, interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<Fem3dPlayApp>>>) -> Option<usize> {
        fem3d_retained_extent(command, snapshot, interaction)
    }

    fn step(&mut self, input: &ArtifactCommandInputs<'_, EditorApp<Fem3dPlayApp>>, _cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactCommandWorkStep<EditorApp<Fem3dPlayApp>>, semio_framework_job::OriginalTurnFailure> {
        let original: Result<_, Fault> = (|| {
        if self.consumed {
            return Err(Fault::from("retained-command-bounded-work-repeated"));
        }
        self.consumed = true;
        let ArtifactCommandInputs { snapshot_owner: _, snapshot_read: _, command, snapshot, config, history, interaction, hover, context, operation } = *input;
        let selected = || fem3d_interaction_selection_ids(interaction);
        match command {
            Fem3dCommand::TranslateSelection(payload) => {
                return crate::editor::fem3d::commands::gumball::gumball_step("translateSelection", Fem3dGumballMotion::Translate { dx: payload.dx, dy: payload.dy, dz: payload.dz }, &payload.ids, payload.phase.as_deref(), payload.reason.as_deref(), snapshot, &selected(), context, operation);
            }
            Fem3dCommand::RotateSelection(payload) => {
                return crate::editor::fem3d::commands::gumball::gumball_step("rotateSelection", Fem3dGumballMotion::Rotate { axis: [payload.ax, payload.ay, payload.az], angle: payload.angle }, &payload.ids, payload.phase.as_deref(), payload.reason.as_deref(), snapshot, &selected(), context, operation);
            }
            Fem3dCommand::ScaleSelection(payload) => {
                return crate::editor::fem3d::commands::gumball::gumball_step("scaleSelection", Fem3dGumballMotion::Scale { sx: payload.sx, sy: payload.sy, sz: payload.sz }, &payload.ids, payload.phase.as_deref(), payload.reason.as_deref(), snapshot, &selected(), context, operation);
            }
            _ => {}
        }
        let playback = match command {
            Fem3dCommand::ResultAnimationTick(payload) => {
                let cfg = ConfigView { snapshot: config, window: context.and_then(|context| context.window_config.as_ref()) };
                let view = context.and_then(|context| context.view_state.as_ref()).ok_or_else(|| Fault::from("fem3d.result-animation-tick.window-context-required"))?;
                let clock = window_results::transient::required_clock::<window_results::transient::Fem3dResultsWindowTransientOwner>(context.and_then(|context| context.window_transient.as_ref()), &payload.window_id)?;
                Some(result_animation_tick::result_animation_tick_step::<set_result_animation::Fem3dResultsPlayback>(payload, &cfg, view, clock)?)
            }
            Fem3dCommand::SetResultAnimation(payload) => {
                let cfg = ConfigView { snapshot: config, window: context.and_then(|context| context.window_config.as_ref()) };
                let view = context.and_then(|context| context.view_state.as_ref()).ok_or_else(|| Fault::from("fem3d.result-animation.window-context-required"))?;
                let clock = payload.window_id.as_deref().filter(|id| !id.is_empty()).and_then(|id| context.and_then(|context| window_results::transient::captured_clock::<window_results::transient::Fem3dResultsWindowTransientOwner>(context.window_transient.as_ref(), id)));
                Some(set_result_animation::set_result_animation_step::<set_result_animation::Fem3dResultsPlayback>(payload, &cfg, view, clock)?)
            }
            _ => None,
        };
        match playback {
            Some(step) => Ok(ArtifactCommandWorkStep::CompleteWithEphemeral { emit: step.emit, ephemeral: EphemeralEmit { presence: Vec::new(), transient: Vec::new(), window_transient: step.window_transient } }),
            None => fem3d_retained_reduce(command, snapshot, config, history, interaction, hover, context, operation).map(ArtifactCommandWorkStep::Complete),
        }
    
        })();
        original.map_err(semio_framework_job::OriginalTurnFailure::from)
    }
}

pub(super) struct Fem3dRetainedCommandJobFactory {
    pub(super) keys: Vec<ToolFactoryKey>,
}

impl Fem3dRetainedCommandJobFactory {
    pub(super) fn new(controller_id: &str) -> Self {
        Self { keys: FEM3D_RETAINED_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl ToolJobFactory for Fem3dRetainedCommandJobFactory {
    type Payload = semio_framework_plugin::retained_command::ArtifactRetainedCommandPayload<EditorApp<Fem3dPlayApp>>;
    type Job = semio_framework_plugin::retained_command::ArtifactRetainedCommandJob<EditorApp<Fem3dPlayApp>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        FEM3D_RETAINED_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> ToolExecutionContract {
        fem3d_retained_contract()
    }

    semio_framework_plugin::retained_command_factory_admission!(maximum_raw_bytes: FEM3D_RETAINED_RAW_BYTES, checkpoint_resume: true);

    
}

impl ArtifactOwnedToolJobFactory for Fem3dRetainedCommandJobFactory {
    type Owner = EditorApp<Fem3dPlayApp>;
    const TOOL_IDS: &'static [&'static str] = FEM3D_RETAINED_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::FEM_3D_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = FEM3D_RETAINED_PUBLICATION_CONTRACTS;
}
//#endregion 🧵️RetainedCommands
