//! 🧭️ Shooting play app commands — the transform gumball: translate/rotate/scale the selected assets.
//!
//! 🛠️ Every gumball gesture runs through the gumball TOOL: a `🔄️machine` statechart whose effects are
//! `ToolYield`s, driven by the `🛠️tool-machine` runner. Both hosts accumulate a drag locally (`World3dHost`'s
//! instance preview, the wgpu world engine's gesture) and dispatch its NET pose delta once on release, so one
//! gesture is one dispatch, one `ToolTransaction`, one edit and one history row whose relative leaf
//! (`drag-`/`rotate-`/`scale-assets`, literal targets and parameters) time travel edits. A cancelled drag
//! dispatches nothing and leaves zero trace. Tool state is never history; the yielded leaf is
//! (design `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING/📋️design.md` §5).
#![allow(unexpected_cfgs)]

use crate::editor::shooting::config::{ShootingConfig, ShootingConfigMutation};
use crate::editor::shooting::{ShootingDispatchCtx, SHOOTING_PLAY_APP_ID};
use crate::standards::v1::subsets::any::schema::mutations::ShootingMutation;
use crate::ShootingSnapshot;
use machine::Command;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_tool_machine::{ToolMachineRunner, ToolStep, ToolYield};
use semio_framework_value_derive::{FromValue, ToValue};

/// 🎯️ Falls back to the current `"assets"` interaction-domain selection (read once per dispatch into
/// `ShootingDispatchCtx::selected_asset_ids` — see that struct's doc comment) when the command carries
/// no explicit ids.
fn mesh_selection_ids_typed(ids: &[String], fallback: &[String]) -> Vec<String> {
    if ids.is_empty() {
        fallback.to_vec()
    } else {
        ids.to_vec()
    }
}

//#region 🛠️GumballTool
/// 🔑️ The transaction key of a gesture's leaf — one gesture upserts exactly one relative leaf.
pub const SHOOTING_GUMBALL_TOOL_KEY: &str = "assets:0";

/// 📨️ One gumball gesture: its net relative leaf and whether any asset it names is on the base it applies to —
/// a dispatch input, never tool state.
#[derive(Clone, Debug)]
pub struct GumballToolRequest {
    pub leaf: ShootingMutation,
    pub targets_present: bool,
}

impl GumballToolRequest {
    /// 🧮️ The request for `leaf` on `base`.
    pub fn on(base: &ShootingSnapshot, leaf: ShootingMutation) -> Self {
        let targets: &[String] = match &leaf {
            ShootingMutation::DragAssets(leaf) => &leaf.asset_ids,
            ShootingMutation::RotateAssets(leaf) => &leaf.asset_ids,
            ShootingMutation::ScaleAssets(leaf) => &leaf.asset_ids,
            _ => &[],
        };
        Self { targets_present: base.assets.iter().any(|asset| targets.contains(&asset.id)), leaf }
    }

    /// 🎚️ Whether the gesture moves anything: a present target and a finite, admissible, non-identity motion.
    pub fn moves(&self) -> bool {
        self.targets_present
            && match &self.leaf {
                ShootingMutation::DragAssets(leaf) => [leaf.dx, leaf.dy, leaf.dz].iter().all(|value| value.is_finite()) && (leaf.dx, leaf.dy, leaf.dz) != (0.0, 0.0, 0.0),
                ShootingMutation::RotateAssets(leaf) => [leaf.ax, leaf.ay, leaf.az, leaf.angle].iter().all(|value| value.is_finite()) && leaf.angle != 0.0 && (leaf.ax, leaf.ay, leaf.az) != (0.0, 0.0, 0.0),
                ShootingMutation::ScaleAssets(leaf) => [leaf.sx, leaf.sy, leaf.sz].iter().all(|value| value.is_finite() && *value > 0.0) && (leaf.sx, leaf.sy, leaf.sz) != (1.0, 1.0, 1.0),
                _ => false,
            }
    }
}

/// 🧰️ The gumball tool's context: nothing survives an event, because a gumball gesture is one event.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct GumballToolContext;

fn gumball_tool_context(input: GumballToolContext) -> GumballToolContext {
    input
}

fn gesture_moves(_context: &GumballToolContext, event: Option<&gumball_tool::Event>) -> bool {
    matches!(event, Some(gumball_tool::Event::Gesture(request)) if request.moves())
}

fn yield_gesture(_context: &mut GumballToolContext, event: Option<&gumball_tool::Event>, sink: &mut Vec<Command<gumball_tool::GumballTool>>) {
    let Some(gumball_tool::Event::Gesture(request)) = event else { return };
    sink.push(Command::Effect(ToolYield::upsert(SHOOTING_GUMBALL_TOOL_KEY, request.leaf.clone())));
    sink.push(Command::Effect(ToolYield::Commit));
}

machine::statechart! {
    machine gumball_tool {
        context: GumballToolContext;
        event Event { Gesture(GumballToolRequest) }
        input: GumballToolContext;
        output: ();
        effect: ToolYield<ShootingMutation>;
        context_from_input: gumball_tool_context;
        initial: idle;
        state idle {
            on Gesture if gesture_moves => idle do yield_gesture;
        }
    }
}

/// 🧷️ The gumball tool's host: its chart declares no timer, no invoke and no foreign effect.
pub struct GumballToolHost;

impl machine::Host<gumball_tool::GumballTool> for GumballToolHost {
    fn execute_effect(&mut self, _actor: machine::ActorId, _effect: ToolYield<ShootingMutation>) {}
    fn schedule(&mut self, _actor: machine::ActorId, _timer: machine::TimerId, _delay_ms: u64) {}
    fn cancel_timer(&mut self, _actor: machine::ActorId, _timer: machine::TimerId) {}
    fn start_task(&mut self, _actor: machine::ActorId, _invoke: machine::InvokeId) {}
    fn cancel_task(&mut self, _actor: machine::ActorId, _invoke: machine::InvokeId) {}
    fn now_ms(&self) -> u64 {
        semio_framework_job::default_now_ms().unwrap_or(0)
    }
}

/// 🛠️ Runs one gesture through a gumball tool at rest as ONE transaction of `<appId>#<verb>`, its ref minted from
/// the admission's `authoring_seed` and the host clock. `None` when the gesture moves nothing: zero trace.
pub fn shooting_gumball_commit(verb: &str, authoring_seed: &str, request: GumballToolRequest) -> Option<(protocol::TransactionRef, Vec<ShootingMutation>)> {
    let mut runner = ToolMachineRunner::<gumball_tool::GumballTool, GumballToolHost>::start(format!("{SHOOTING_PLAY_APP_ID}#{verb}"), protocol::ActorId(authoring_seed.to_string()), GumballToolContext, GumballToolHost).ok()?;
    let clock = semio_framework_tool_machine::authoring_clock(0);
    match runner.send(gumball_tool::Event::Gesture(request), clock).ok()? {
        ToolStep::Committed(transaction, mutations) => Some((transaction, mutations)),
        ToolStep::Idle | ToolStep::Open | ToolStep::Aborted(..) | ToolStep::Empty(_) => None,
    }
}

/// 📤️ The emission of one gumball gesture: its committed transaction as ONE edit stamped with the ref — plain
/// when the view carries no admission (a render or test view without command authority) — or nothing.
fn gumball_emit(verb: &str, doc: &ArtifactView<'_, ShootingSnapshot>, leaf: ShootingMutation) -> Emit<ShootingMutation, ShootingConfigMutation> {
    let seed = doc.operation_optional().map(|operation| operation.authoring_seed.as_str()).unwrap_or_default();
    match shooting_gumball_commit(verb, seed, GumballToolRequest::on(doc.snapshot, leaf)) {
        Some((transaction, mutations)) if !seed.is_empty() => Emit::commit_transaction(transaction, mutations),
        Some((_, mutations)) => Emit::mutations(mutations),
        None => Emit::default(),
    }
}
//#endregion 🛠️GumballTool

//#region 🔖️TranslateSelection
pub mod translate_selection {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
    #[dsl(keyword = "translate-selection")]
    pub struct TranslateSelection {
        pub asset_ids: Vec<String>,
        pub dx: f64,
        pub dy: f64,
        pub dz: f64,
    }

    pub fn handle(payload: &TranslateSelection, doc: &ArtifactView<'_, ShootingSnapshot>, _cfg: &ConfigView<'_, ShootingConfig>, ctx: &mut ShootingDispatchCtx) -> Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault> {
        let asset_ids = mesh_selection_ids_typed(&payload.asset_ids, &ctx.selected_asset_ids);
        Ok(gumball_emit("translateSelection", doc, ShootingMutation::DragAssets(crate::mutations::drag_assets::DragAssets { asset_ids, dx: payload.dx, dy: payload.dy, dz: payload.dz })))
    }
}
//#endregion 🔖️TranslateSelection

//#region 🔖️RotateSelection
pub mod rotate_selection {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
    #[dsl(keyword = "rotate-selection")]
    pub struct RotateSelection {
        pub asset_ids: Vec<String>,
        pub ax: f64,
        pub ay: f64,
        pub az: f64,
        pub angle: f64,
    }

    pub fn handle(payload: &RotateSelection, doc: &ArtifactView<'_, ShootingSnapshot>, _cfg: &ConfigView<'_, ShootingConfig>, ctx: &mut ShootingDispatchCtx) -> Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault> {
        let asset_ids = mesh_selection_ids_typed(&payload.asset_ids, &ctx.selected_asset_ids);
        Ok(gumball_emit("rotateSelection", doc, ShootingMutation::RotateAssets(crate::mutations::rotate_assets::RotateAssets { asset_ids, ax: payload.ax, ay: payload.ay, az: payload.az, angle: payload.angle })))
    }
}
//#endregion 🔖️RotateSelection

//#region 🔖️ScaleSelection
pub mod scale_selection {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
    #[dsl(keyword = "scale-selection")]
    pub struct ScaleSelection {
        pub asset_ids: Vec<String>,
        pub sx: f64,
        pub sy: f64,
        pub sz: f64,
    }

    pub fn handle(payload: &ScaleSelection, doc: &ArtifactView<'_, ShootingSnapshot>, _cfg: &ConfigView<'_, ShootingConfig>, ctx: &mut ShootingDispatchCtx) -> Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault> {
        let asset_ids = mesh_selection_ids_typed(&payload.asset_ids, &ctx.selected_asset_ids);
        Ok(gumball_emit("scaleSelection", doc, ShootingMutation::ScaleAssets(crate::mutations::scale_assets::ScaleAssets { asset_ids, sx: payload.sx, sy: payload.sy, sz: payload.sz })))
    }
}
//#endregion 🔖️ScaleSelection

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
