//! ⏱️ Fem3d play app command — `result-animation-tick`: one playback clock tick — advances the phase
//! and re-arms itself through `Effect::DispatchAction` while playing.
//!
//! A wasm guest may not read a monotonic clock inside a command, so the tick advances by the FIXED
//! `ANIMATION_TICK_SECONDS` and the host's `delay_ms` is what keeps that delta honest. A tick that
//! arrives on a stopped window writes nothing and re-arms nothing: that is how a pause, a closed
//! window or a second chain that raced the first dies out instead of spinning the guest forever.

use crate::editor::fem3d::commands::set_result_animation::{playback_dirty_scope, rearm_effect, PLAYBACK_COALESCE_KEY};
use crate::editor::fem3d::modes::edit::windows::results;
use crate::editor::fem3d::modes::edit::windows::results::config::ANIMATION_TICK_SECONDS;
use crate::standards::v1::subsets::any::schema::mutations::text::Fem3dMutation;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use semio_framework_value_derive::{FromValue, ToValue};

type Fem3dSnapshot = crate::Fem3dSnapshot;

//#region 🔖️ResultAnimationTick
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "result-animation-tick")]
pub struct ResultAnimationTick {}

pub fn handle(_payload: &ResultAnimationTick, _doc: &ArtifactView<'_, Fem3dSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Fem3dMutation, NoConfigMutation>, Fault> {
    Err(Fault::from("fem3d.result-animation-tick.window-context-required"))
}

pub fn handle_window(_payload: &ResultAnimationTick, _doc: &ArtifactView<'_, Fem3dSnapshot>, cfg: &ConfigView<'_, NoConfig>, view: &semio_framework_plugin::ViewModel) -> Result<Emit<Fem3dMutation, NoConfigMutation>, Fault> {
    let current = results::config::current(cfg);
    if !current.animation.playing {
        return Ok(Emit::default());
    }
    let window_id = results::config::addressed_window_id(cfg, view, None)?;
    let mut next = current.clone();
    next.animation = current.animation.advanced(ANIMATION_TICK_SECONDS);
    let effects = if next.animation.playing { vec![rearm_effect(&window_id)] } else { Vec::new() };
    Ok(Emit { window_config_mutations: vec![results::config::addressed_to(&window_id, next)], effects, coalesce_key: Some(PLAYBACK_COALESCE_KEY.to_owned()), ui_scope: playback_dirty_scope(), ..Default::default() })
}
//#endregion 🔖️ResultAnimationTick

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
