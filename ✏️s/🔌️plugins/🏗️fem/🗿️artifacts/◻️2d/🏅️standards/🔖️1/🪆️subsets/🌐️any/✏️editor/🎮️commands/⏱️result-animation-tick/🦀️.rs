//! ⏱️ FEM play app command — `result-animation-tick`: one playback clock tick — advances the
//! window's running clock and re-arms itself through `Effect::DispatchAction` while the transport
//! plays. ONE implementation for every FEM editor ([`result_animation_tick_step`] over a
//! [`FemPlaybackTransport`]).
//!
//! A wasm guest may not read a monotonic clock inside a command, so the tick advances by the FIXED
//! `ANIMATION_TICK_SECONDS` and the host's `delay_ms` is what keeps that delta honest.
//!
//! 🫧️ The frame lands in the results window's TRANSIENT partition — one `Arc` root, no history —
//! never in its config: a coalesced config amend per frame grew the tick cost with every frame
//! played and displaced three retirement owners a tick. Only the transitions touch the config, each as
//! ONE plain config edit (design §20.1: no amend on any lane): a `Once` run that reaches its end stops
//! the transport there, and a tick that finds the transport stopped PARKS the clock — folds its phase and direction into the resting config and
//! clears the transient — and re-arms nothing. That is how a pause, a closed window or a second
//! chain that raced the first dies out instead of spinning the guest forever.

use crate::app_surface::{FemResultsAnimation, ANIMATION_TICK_SECONDS};
use crate::editor::fem2d::commands::set_result_animation::{rearm_effect, resting_step, FemPlaybackStep, FemPlaybackTransport};
use crate::editor::fem2d::modes::edit::windows::results::transient::{addressed_to, FemPlaybackClock};
use crate::standards::v1::subsets::any::schema::mutations::Fem2dMutation;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation, ViewModel};
use semio_framework_value_derive::{FromValue, ToValue};

type Fem2dSnapshot = crate::Fem2dSnapshot;

//#region 🔖️ResultAnimationTick
/// 🕐️ One frame of the playback chain of ONE results window. `window_id` is the window the chain
/// was armed for — the retained route captures that exact window's transient authority from it,
/// never from whichever pane the shell happened to focus when it redispatched the hop.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "result-animation-tick")]
pub struct ResultAnimationTick {
    pub window_id: String,
}

/// 🎞️ One frame under the captured window's transport settings and running clock.
pub fn result_animation_tick_step<T: FemPlaybackTransport>(payload: &ResultAnimationTick, cfg: &ConfigView<'_, NoConfig>, view: &ViewModel, clock: Option<FemPlaybackClock>) -> Result<FemPlaybackStep<T::Mutation>, Fault> {
    let window_id = T::addressed_window_id(cfg, view, Some(&payload.window_id))?;
    let current = T::current(cfg);
    let settings = T::animation(&current);
    if !settings.playing {
        return Ok(clock.map_or_else(FemPlaybackStep::default, |clock| resting_step::<T>(&window_id, &current, clock.parked_into(&settings))));
    }
    let (next, running) = clock.unwrap_or_else(|| FemPlaybackClock::from_settings(&settings)).advanced(&settings, ANIMATION_TICK_SECONDS);
    if !running {
        return Ok(resting_step::<T>(&window_id, &current, FemResultsAnimation { playing: false, ..next.parked_into(&settings) }));
    }
    Ok(FemPlaybackStep { emit: Emit { effects: vec![rearm_effect::<T>(&window_id)], ui_scope: T::dirty_scope(), ..Default::default() }, window_transient: vec![addressed_to::<T::Owner>(&window_id, Some(next))] })
}

pub fn handle(_payload: &ResultAnimationTick, _doc: &ArtifactView<'_, Fem2dSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Fem2dMutation, NoConfigMutation>, Fault> {
    Err(Fault::from("fem.result-animation-tick.window-context-required"))
}

/// 🚧️ The batch route has no transient authority to land a frame in — the tick is a retained
/// command by construction, and any other arrival is a routing error.
pub fn handle_window(_payload: &ResultAnimationTick, _doc: &ArtifactView<'_, Fem2dSnapshot>, _cfg: &ConfigView<'_, NoConfig>, _view: &ViewModel) -> Result<Emit<Fem2dMutation, NoConfigMutation>, Fault> {
    Err(Fault::from("fem.result-animation-tick.retained-route-required"))
}
//#endregion 🔖️ResultAnimationTick

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
