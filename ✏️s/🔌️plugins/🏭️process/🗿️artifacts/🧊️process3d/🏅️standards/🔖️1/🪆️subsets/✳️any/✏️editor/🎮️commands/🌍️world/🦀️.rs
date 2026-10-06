//! 🌍️ Process 3d play app commands — 3D viewport interactions: click-to-place, push/pull face drag,
//! and face picking.
//!
//! 🛠️ Every world gesture runs through the world TOOL: a `🔄️machine` statechart whose effects are `ToolYield`s,
//! driven as a one-shot by the shared gesture runner (`drive_chart_gesture`, design §22.10). The host keeps a face drag local and dispatches its net
//! `distance`/`normal`/`startPoint` once on release (a click-to-place is one dispatch too), so one gesture is one
//! dispatch, one `ToolTransaction`, one edit and one history row whose `create-step` leaf — the step with its
//! measure and pose — time travel edits. A gesture that places nothing leaves zero trace. Tool state is never
//! history; the yielded leaf is (design `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING/📋️design.md` §5).
#![allow(unexpected_cfgs)]

use crate::editor::process3d::axis_angle_from_up_to;
use crate::editor::process3d::config::{Process3dConfig, Process3dConfigMutation};
use crate::editor::process3d::set_active_utility_effect;
use crate::editor::process3d::terminology::{process3d_labels, Process3dLabels};
use crate::schema::inferences::capability_for_measure_kind;
use crate::editor::process3d::commands::step::insert_step_emit;
use crate::schema::next_step_id;
use crate::{op::Process3dMutation, MeasureKind, Pose, Process3dSnapshot, ProcessMeasure, ProcessStep, StepOrigin, WorkingSolid};
use machine::Command;
use semio_framework_plugin::{ArtifactView, ConfigView, Effect, Emit, Fault};
use semio_framework_tool_machine::{drive_chart_gesture, GestureChart, GesturePhase, ToolRefusal, ToolYield};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🛠️WorldTool
/// 🪪️ The editor whose world tool authors every gesture transaction: `<appId>#<verb>`.
pub const PROCESS3D_EDITOR_APP_ID: &str = "s.process.process3d@1/*#editor";

/// 📨️ One world gesture: the document mutations its placement yields (the `create-step` of the placed step) — a
/// dispatch input, never tool state.
#[derive(Clone, Debug)]
pub struct WorldToolRequest {
    pub mutations: Vec<Process3dMutation>,
}

/// 🧰️ The world tool's context: nothing survives an event, because a world gesture is one event.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct WorldToolContext;

fn world_tool_context(input: WorldToolContext) -> WorldToolContext {
    input
}

fn gesture_places(_context: &WorldToolContext, event: Option<&world_tool::Event>) -> bool {
    matches!(event, Some(world_tool::Event::Gesture(request)) if !request.mutations.is_empty())
}

fn yield_gesture(_context: &mut WorldToolContext, event: Option<&world_tool::Event>, sink: &mut Vec<Command<world_tool::WorldTool>>) {
    let Some(world_tool::Event::Gesture(request)) = event else { return };
    sink.extend(request.mutations.iter().enumerate().map(|(index, mutation)| Command::Effect(ToolYield::upsert(format!("step:{index}"), mutation.clone()))));
    sink.push(Command::Effect(ToolYield::Commit));
}

machine::statechart! {
    machine world_tool {
        context: WorldToolContext;
        event Event { Gesture(WorldToolRequest) }
        input: WorldToolContext;
        output: ();
        effect: ToolYield<Process3dMutation>;
        context_from_input: world_tool_context;
        initial: idle;
        state idle {
            on Gesture if gesture_places => idle do yield_gesture;
        }
    }
}

/// 🧷️ The world tool's host: its chart declares no timer, no invoke and no foreign effect.
pub struct WorldToolHost;

impl machine::Host<world_tool::WorldTool> for WorldToolHost {
    fn execute_effect(&mut self, _actor: machine::ActorId, _effect: ToolYield<Process3dMutation>) {}
    fn schedule(&mut self, _actor: machine::ActorId, _timer: machine::TimerId, _delay_ms: u64) {}
    fn cancel_timer(&mut self, _actor: machine::ActorId, _timer: machine::TimerId) {}
    fn start_task(&mut self, _actor: machine::ActorId, _invoke: machine::InvokeId) {}
    fn cancel_task(&mut self, _actor: machine::ActorId, _invoke: machine::InvokeId) {}
    fn now_ms(&self) -> u64 {
        semio_framework_job::default_now_ms().unwrap_or(0)
    }
}

/// 🧭️ The world chart on the shared gesture runner: every gesture is one event, so nothing is ever persisted or restored.
impl GestureChart for world_tool::WorldTool {
    type Tick = WorldToolRequest;
    type Host = WorldToolHost;

    fn tool(verb: &str) -> String {
        format!("{PROCESS3D_EDITOR_APP_ID}#{verb}")
    }

    fn host() -> WorldToolHost {
        WorldToolHost
    }

    fn input() -> WorldToolContext {
        WorldToolContext
    }

    fn restore(_entries: &[(String, Process3dMutation)], _context: &semio_framework_value::DslValue) -> Option<WorldToolContext> {
        None
    }

    fn event(phase: GesturePhase, _at_rest: bool, tick: Option<WorldToolRequest>) -> Option<world_tool::Event> {
        match phase {
            GesturePhase::Once => tick.map(world_tool::Event::Gesture),
            GesturePhase::Stream | GesturePhase::Commit | GesturePhase::Abort(_) => None,
        }
    }
}

/// 🛠️ Runs one gesture through the world tool at rest as ONE transaction of `<appId>#<verb>`, its ref minted from the
/// admission's `authoring_seed` and the host clock. `None` when the gesture places nothing: zero trace.
pub fn process3d_world_commit(verb: &str, authoring_seed: &str, request: WorldToolRequest) -> Result<Option<(protocol::TransactionRef, Vec<Process3dMutation>)>, ToolRefusal> {
    Ok(drive_chart_gesture::<world_tool::WorldTool>(None, verb, GesturePhase::Once, Some(request), authoring_seed, "")?.committed)
}

/// 📤️ The emission of one world gesture placing `step`: its committed transaction as ONE edit stamped with the ref
/// (plain when the view carries no admission — a render or test view), the viewer's cursor moved past the step on the
/// config lane, and `effects`. A gesture its tool refuses is the dispatch's fault.
fn world_emit(verb: &str, doc: &ArtifactView<'_, Process3dSnapshot>, cfg: &ConfigView<'_, Process3dConfig>, step: ProcessStep, effects: Vec<Effect>) -> Result<Emit<Process3dMutation, Process3dConfigMutation>, Fault> {
    let placed = insert_step_emit(doc.snapshot, cfg.snapshot, step);
    let seed = doc.operation_optional().map(|operation| operation.authoring_seed.as_str()).unwrap_or_default();
    let committed = process3d_world_commit(verb, seed, WorldToolRequest { mutations: placed.artifact_mutations })
        .map_err(|refusal| Fault::new(semio_framework_plugin::FaultOrigin::Framework, semio_framework_plugin::FaultCode::new(refusal.code()), format!("gesture tool {verb:?} refused its dispatch")))?;
    Ok(Emit { config_mutations: placed.config_mutations, effects, ..semio_framework_plugin::app::gesture_emit(committed, seed) })
}
//#endregion 🛠️WorldTool

//#region 🔖️FaceDrag
/// 🖱️➡️ Builds a push/pull step from a face-drag gesture: dragging into the solid (negative `distance`
/// along the face's outward `normal`) removes material (Cut); dragging outward (positive) adds material
/// (Attach). A `Pose.position` is the tool's CENTRE (`inferences::solid_for_spec` centres every kernel
/// primitive before posing it), so the box is centred half a drag along the normal from the picked
/// point — it then spans exactly the dragged region, flush with the picked face, in both directions.
fn process3d_step_from_face_drag(fixture: &Process3dSnapshot, normal: [f64; 3], point: [f64; 3], distance: f64, face_extent: Option<[f64; 2]>, labels: &Process3dLabels) -> Option<ProcessStep> {
    if distance.abs() < 1e-6 {
        return None;
    }
    let (width, depth) = face_extent.map_or((0.2, 0.2), |[w, d]| (w.max(0.02), d.max(0.02)));
    let height = distance.abs();
    let (axis, angle) = axis_angle_from_up_to(normal);
    let offset = distance / 2.0;
    let position = [point[0] + normal[0] * offset, point[1] + normal[1] * offset, point[2] + normal[2] * offset];
    let pose = Pose { position, axis, angle };
    let (measure, label, machine_id, capability_id) = if distance < 0.0 {
        (ProcessMeasure::Cut { tool: WorkingSolid::Box { width, depth, height }, pose }, labels.push_cut, "saw", "cut")
    } else {
        (ProcessMeasure::Attach { component: WorkingSolid::Box { width, depth, height }, pose }, labels.pull_attach, "attacher", "attach")
    };
    let origin = StepOrigin { machine_id: machine_id.to_string(), capability_id: capability_id.to_string() };
    Some(ProcessStep { id: next_step_id(fixture), label: label.as_str().to_string(), enabled: true, origin: Some(origin), measure })
}
//#endregion 🔖️FaceDrag

//#region 🔖️WorldPointerDown
pub mod world_pointer_down {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
    #[dsl(keyword = "world-pointer-down")]
    pub struct WorldPointerDown {
        #[dsl(coord)]
        pub position: [f64; 3],
    }

    pub fn handle(
        payload: &WorldPointerDown,
        doc: &ArtifactView<'_, Process3dSnapshot>,
        cfg: &ConfigView<'_, Process3dConfig>,
        ctx: &mut crate::editor::process3d::Process3dDispatchCtx,
    ) -> Result<Emit<Process3dMutation, Process3dConfigMutation>, Fault> {
        let fixture = doc.snapshot;
        let utility = ctx.active_utility()?;
        if utility == "select" {
            return Ok(Emit::default());
        }
        let measure_kind = match utility {
            "drill" => MeasureKind::Drill,
            "attach" => MeasureKind::Attach,
            _ => MeasureKind::Cut,
        };
        let (machine, capability) = capability_for_measure_kind(&fixture.workshop, measure_kind);
        let origin = StepOrigin { machine_id: machine.id, capability_id: capability.id.clone() };
        let step = ProcessStep { id: next_step_id(fixture), label: capability.label.clone(), enabled: true, origin: Some(origin), measure: crate::schema::inferences::measure_for_capability(&capability, Some(payload.position)) };
        world_emit("worldPointerDown", doc, cfg, step, vec![set_active_utility_effect("select")])
    }
}
//#endregion 🔖️WorldPointerDown

//#region 🔖️WorldFaceDragEnd
pub mod world_face_drag_end {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
    #[dsl(keyword = "world-face-drag-end")]
    pub struct WorldFaceDragEnd {
        #[dsl(coord)]
        pub normal: [f64; 3],
        #[dsl(coord)]
        pub start_point: [f64; 3],
        pub distance: f64,
        pub face_extent: Option<[f64; 2]>,
    }

    pub fn handle(
        payload: &WorldFaceDragEnd,
        doc: &ArtifactView<'_, Process3dSnapshot>,
        cfg: &ConfigView<'_, Process3dConfig>,
        ctx: &mut crate::editor::process3d::Process3dDispatchCtx,
    ) -> Result<Emit<Process3dMutation, Process3dConfigMutation>, Fault> {
        let fixture = doc.snapshot;
        if ctx.active_utility()? != "select" {
            return Ok(Emit::default());
        }
        match process3d_step_from_face_drag(fixture, payload.normal, payload.start_point, payload.distance, payload.face_extent, process3d_labels(ctx.view_state()?)) {
            Some(step) => world_emit("worldFaceDragEnd", doc, cfg, step, Vec::new()),
            None => Ok(Emit::default()),
        }
    }
}
//#endregion 🔖️WorldFaceDragEnd

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
