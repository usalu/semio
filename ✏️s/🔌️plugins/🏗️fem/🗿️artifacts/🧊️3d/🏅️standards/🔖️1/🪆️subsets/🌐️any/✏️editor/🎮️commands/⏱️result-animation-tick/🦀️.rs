//! ⏱️ Fem3d play app command — `result-animation-tick`: one playback clock tick — advances the
//! window's running clock and re-arms itself through `Effect::DispatchAction` while the transport
//! plays.
//!
//! A wasm guest may not read a monotonic clock inside a command, so the tick advances by the FIXED
//! `ANIMATION_TICK_SECONDS` and the host's `delay_ms` is what keeps that delta honest.
//!
//! 🫧️ The frame lands in the results window's TRANSIENT partition — one `Arc` root, no history —
//! never in its config: a coalesced config amend per frame grew the tick cost with every frame
//! played and displaced three retirement owners a tick. Only the transitions touch the config:
//! a `Once` run that reaches its end stops the transport there, and a tick that finds the
//! transport stopped PARKS the clock — folds its phase and direction into the resting config and
//! clears the transient — and re-arms nothing. That is how a pause, a closed window or a second
//! chain that raced the first dies out instead of spinning the guest forever.

use crate::editor::fem3d::commands::set_result_animation::{playback_dirty_scope, rearm_effect, Fem3dPlaybackStep, PLAYBACK_COALESCE_KEY};
use crate::editor::fem3d::modes::edit::windows::results;
use crate::editor::fem3d::modes::edit::windows::results::config::{Fem3dResultsAnimation, Fem3dResultsWindowConfig, ANIMATION_TICK_SECONDS};
use crate::editor::fem3d::modes::edit::windows::results::transient::Fem3dPlaybackClock;
use crate::standards::v1::subsets::any::schema::mutations::text::Fem3dMutation;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use semio_framework_value_derive::{FromValue, ToValue};

type Fem3dSnapshot = crate::Fem3dSnapshot;

//#region 🔖️ResultAnimationTick
/// ⏱️ One frame of the playback chain of ONE results window. `window_id` is the window the chain
/// was armed for — the retained route captures that exact window's transient authority from it,
/// never from whichever pane the shell happened to focus when it redispatched the hop.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "result-animation-tick")]
pub struct ResultAnimationTick {
    pub window_id: String,
}

pub fn handle(_payload: &ResultAnimationTick, _doc: &ArtifactView<'_, Fem3dSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Fem3dMutation, NoConfigMutation>, Fault> {
    Err(Fault::from("fem3d.result-animation-tick.window-context-required"))
}

/// ⏱️ The batch route has no transient authority to land a frame in — the tick is a retained
/// command by construction (`Fem3dPlaybackWork`), and any other arrival is a routing error.
pub fn handle_window(_payload: &ResultAnimationTick, _doc: &ArtifactView<'_, Fem3dSnapshot>, _cfg: &ConfigView<'_, NoConfig>, _view: &semio_framework_plugin::ViewModel) -> Result<Emit<Fem3dMutation, NoConfigMutation>, Fault> {
    Err(Fault::from("fem3d.result-animation-tick.retained-route-required"))
}

/// ⏱️ One frame under the captured window's transport settings and running clock.
pub fn step(payload: &ResultAnimationTick, cfg: &ConfigView<'_, NoConfig>, view: &semio_framework_plugin::ViewModel, clock: Option<Fem3dPlaybackClock>) -> Result<Fem3dPlaybackStep, Fault> {
    let window_id = results::config::addressed_window_id(cfg, view, Some(&payload.window_id))?;
    let current = results::config::current(cfg);
    let settings = &current.animation;
    if !settings.playing {
        let Some(clock) = clock else { return Ok(Fem3dPlaybackStep::default()) };
        let parked = Fem3dResultsWindowConfig { animation: clock.parked_into(settings), ..current.clone() };
        return Ok(Fem3dPlaybackStep {
            emit: Emit { window_config_mutations: vec![results::config::addressed_to(&window_id, parked)], coalesce_key: Some(PLAYBACK_COALESCE_KEY.to_owned()), ui_scope: playback_dirty_scope(), ..Default::default() },
            window_transient: vec![results::transient::addressed_to(&window_id, None)],
        });
    }
    let clock = clock.unwrap_or_else(|| Fem3dPlaybackClock::from_settings(settings));
    let (next, running) = clock.advanced(settings, ANIMATION_TICK_SECONDS);
    if !running {
        let stopped = Fem3dResultsWindowConfig { animation: Fem3dResultsAnimation { playing: false, ..next.parked_into(settings) }, ..current.clone() };
        return Ok(Fem3dPlaybackStep {
            emit: Emit { window_config_mutations: vec![results::config::addressed_to(&window_id, stopped)], coalesce_key: Some(PLAYBACK_COALESCE_KEY.to_owned()), ui_scope: playback_dirty_scope(), ..Default::default() },
            window_transient: vec![results::transient::addressed_to(&window_id, None)],
        });
    }
    Ok(Fem3dPlaybackStep {
        emit: Emit { effects: vec![rearm_effect(&window_id)], ui_scope: playback_dirty_scope(), ..Default::default() },
        window_transient: vec![results::transient::addressed_to(&window_id, Some(next))],
    })
}

//#endregion 🔖️ResultAnimationTick

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
