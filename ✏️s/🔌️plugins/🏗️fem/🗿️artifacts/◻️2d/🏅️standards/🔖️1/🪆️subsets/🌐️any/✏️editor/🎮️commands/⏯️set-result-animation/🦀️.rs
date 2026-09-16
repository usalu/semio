//! ⏯️ Fem2d play app command — `result-animation`: results-window playback state (phase, play/pause,
//! speed, loop mode, waveform) — window config only, never a document mutation.
//!
//! Two vocabularies reach this one command. The staged form and the `space` keybinding speak the
//! TYPED fields; a persistent transport control in the results panel speaks `{field, value}`, because
//! the host merges a control's own scalar under the single key `value`
//! (`🛠️ShellHelpers/🟦️.tsx`'s `uiIntentPayload`) and a slider therefore cannot name which field it
//! just moved. A dispatch that names neither — the bare `space` chord — toggles play/pause.

use crate::editor::fem2d::modes::edit::windows::results;
use crate::editor::fem2d::modes::edit::windows::results::config::{Fem2dLoopMode, Fem2dResultsAnimation, Fem2dWaveform, ANIMATION_SPEED_MAXIMUM, ANIMATION_SPEED_MINIMUM, ANIMATION_TICK_MS};
use crate::standards::v1::subsets::any::schema::mutations::text::Fem2dMutation;
use semio_framework_plugin::{ArtifactView, ConfigView, Effect, Emit, Fault, NoConfig, NoConfigMutation};
use semio_framework_value_derive::{FromValue, ToValue};

type Fem2dSnapshot = crate::Fem2dSnapshot;

//#region 🔖️Clock
/// ⏱️ The action the playback clock re-dispatches onto itself.
pub const TICK_ACTION: &str = "resultAnimationTick";

/// 🪪️ The request id every re-arm of the playback chain carries.
const REARM_REQUEST: u64 = 141;

/// 🔁️ The hop that arms — or keeps — the playback clock of ONE results window.
///
/// `Effect::DispatchAction` carries no window of its own; the React ShellHost redispatches it under
/// the `resolvedTargetViewState` of the dispatch that emitted it (`🏛️ShellHost/🟦️.tsx`'s
/// `scheduleDispatchAction`), which is how the chain keeps addressing the window the user pressed
/// play in. `windowId` rides along as the address the hop was armed for.
pub fn rearm_effect(window_id: &str) -> Effect {
    Effect::DispatchAction {
        req: semio_framework_plugin::RequestId(REARM_REQUEST),
        action: TICK_ACTION.into(),
        args: Some(dsl::DslValue::object([("windowId".to_string(), dsl::DslValue::String(window_id.to_string()))])),
        delay_ms: ANIMATION_TICK_MS,
    }
}
//#endregion 🔖️Clock

//#region 🔖️SetResultAnimation
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "result-animation")]
pub struct SetResultAnimation {
    pub phase: Option<f64>,
    pub playing: Option<bool>,
    pub speed: Option<f64>,
    pub loop_mode: Option<String>,
    pub waveform: Option<String>,
    pub field: Option<String>,
    pub value: Option<String>,
}

/// 🔢️ One control's scalar, as the bridge stringified it.
fn number(value: &str) -> Result<f64, Fault> {
    value.trim().parse::<f64>().map_err(|_| Fault::from(format!("fem2d.result-animation.value: '{value}' is not a number")))
}

fn flag(value: &str) -> Result<bool, Fault> {
    match value.trim() {
        "true" | "1" | "on" => Ok(true),
        "false" | "0" | "off" | "" => Ok(false),
        other => Err(Fault::from(format!("fem2d.result-animation.value: '{other}' is not a boolean"))),
    }
}

fn set_playing(animation: &mut Fem2dResultsAnimation, playing: bool) {
    if playing {
        animation.start();
    } else {
        animation.playing = false;
    }
}

/// 🎚️ Applies ONE named transport field — the shape a persistent panel control can express.
fn apply_field(animation: &mut Fem2dResultsAnimation, field: &str, value: &str) -> Result<(), Fault> {
    match field {
        "phase" => animation.phase = number(value)?.clamp(0.0, 1.0),
        "phaseStep" => animation.phase = (animation.phase + number(value)?).rem_euclid(1.0),
        "playing" => set_playing(animation, flag(value)?),
        "speed" => animation.speed = number(value)?.clamp(ANIMATION_SPEED_MINIMUM, ANIMATION_SPEED_MAXIMUM),
        "loopMode" => animation.loop_mode = Fem2dLoopMode::try_from(value).map_err(Fault::from)?,
        "waveform" => animation.waveform = Fem2dWaveform::try_from(value).map_err(Fault::from)?,
        "reverse" => animation.reverse = flag(value)?,
        other => return Err(Fault::from(format!("fem2d.result-animation.field: '{other}' is not a playback field"))),
    }
    Ok(())
}

/// ⏯️ Merges everything the payload names into `animation`; names nothing ⇒ toggle play/pause.
fn merge(payload: &SetResultAnimation, animation: &mut Fem2dResultsAnimation) -> Result<(), Fault> {
    let field = payload.field.as_deref().filter(|field| !field.is_empty());
    let mut named = field.is_some();
    if let Some(phase) = payload.phase {
        animation.phase = phase.clamp(0.0, 1.0);
        named = true;
    }
    if let Some(speed) = payload.speed {
        animation.speed = speed.clamp(ANIMATION_SPEED_MINIMUM, ANIMATION_SPEED_MAXIMUM);
        named = true;
    }
    if let Some(mode) = payload.loop_mode.as_deref() {
        animation.loop_mode = Fem2dLoopMode::try_from(mode).map_err(Fault::from)?;
        named = true;
    }
    if let Some(waveform) = payload.waveform.as_deref() {
        animation.waveform = Fem2dWaveform::try_from(waveform).map_err(Fault::from)?;
        named = true;
    }
    if let Some(playing) = payload.playing {
        set_playing(animation, playing);
        named = true;
    }
    if let Some(field) = field {
        apply_field(animation, field, payload.value.as_deref().unwrap_or_default())?;
    }
    if !named {
        set_playing(animation, !animation.playing);
    }
    Ok(())
}

pub fn handle(_payload: &SetResultAnimation, _doc: &ArtifactView<'_, Fem2dSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Fem2dMutation, NoConfigMutation>, Fault> {
    Err(Fault::from("fem2d.result-animation.window-context-required"))
}

pub fn handle_window(payload: &SetResultAnimation, _doc: &ArtifactView<'_, Fem2dSnapshot>, cfg: &ConfigView<'_, NoConfig>, view: &semio_framework_plugin::ViewModel) -> Result<Emit<Fem2dMutation, NoConfigMutation>, Fault> {
    let window_id = results::config::addressed_window_id(cfg, view)?;
    let current = results::config::current(cfg);
    let mut next = current.clone();
    merge(payload, &mut next.animation)?;
    // 🕰️ ONE clock per window: only the transition into `playing` arms a hop. Moving the phase
    // slider mid-playback, or pressing play twice, must never leave two chains ticking the same
    // window — each would advance the phase by its own frame and the structure would run double
    // speed and never slow down again.
    let effects = if next.animation.playing && !current.animation.playing { vec![rearm_effect(&window_id)] } else { Vec::new() };
    Ok(Emit { window_config_mutations: vec![results::config::addressed_to(&window_id, next)], effects, ..Default::default() })
}
//#endregion 🔖️SetResultAnimation

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
