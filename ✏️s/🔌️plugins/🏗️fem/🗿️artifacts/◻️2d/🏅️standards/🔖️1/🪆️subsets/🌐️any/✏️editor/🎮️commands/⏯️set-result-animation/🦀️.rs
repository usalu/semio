//! ⏯️ FEM play app command — `result-animation`: results-window playback state (phase, play/pause,
//! speed, loop mode, waveform) — window config only, never a document mutation. ONE implementation for every
//! FEM editor: fem 2d and fem 3d each declare their results window as a [`FemPlaybackTransport`] and route
//! the command through [`set_result_animation_step`].
//!
//! Two vocabularies reach this one command. The staged form and the `space` keybinding speak the
//! TYPED fields; a persistent transport control in the results panel speaks `{field, value}`, because
//! the host merges a control's own scalar under the single key `value`
//! (`🛠️ShellHelpers/🟦️.tsx`'s `uiIntentPayload`) and a slider therefore cannot name which field it
//! just moved. A dispatch that names neither — the bare `space` chord — toggles play/pause.
//!
//! Every publication is ONE plain window-config edit (design §20.1: no amend on any lane): a transport press is
//! one edit, and a dragged phase or speed slider rides the framework's config-lane press (`gesture`/`commit`
//! args), whose ticks stay provisional until the release publishes ONE edit and whose cancel leaves none.

use crate::app_surface::{FemLoopMode, FemResultsAnimation, FemWaveform, ANIMATION_SPEED_MAXIMUM, ANIMATION_SPEED_MINIMUM, ANIMATION_TICK_MS};
use crate::editor::fem2d::modes::edit::windows::results;
use crate::editor::fem2d::modes::edit::windows::results::transient::{addressed_to, FemPlaybackClock, FemResultsWindowTransientOwner};
use crate::standards::v1::subsets::any::schema::mutations::Fem2dMutation;
use semio_framework_plugin::{ArtifactView, ConfigView, Effect, Emit, Fault, NoConfig, NoConfigMutation, ViewModel, WindowConfigMutation, WindowTransientMutation};
use semio_framework_value_derive::{FromValue, ToValue};

type Fem2dSnapshot = crate::Fem2dSnapshot;

//#region 🔖️Transport
/// ⏱️ The action the playback clock re-dispatches onto itself.
pub const TICK_ACTION: &str = "resultAnimationTick";

/// 🎛️ One FEM editor's results window as a playback transport: the window config that carries its
/// [`FemResultsAnimation`], the owner of its clock partition, and the lanes a playback step publishes into.
pub trait FemPlaybackTransport {
    type Mutation;
    type Config: Clone;
    type Owner: FemResultsWindowTransientOwner;
    /// 🪪️ The request id every re-arm of this window kind's playback chain carries.
    const REARM_REQUEST: u64;
    fn current(cfg: &ConfigView<'_, NoConfig>) -> Self::Config;
    fn animation(config: &Self::Config) -> FemResultsAnimation;
    fn with_animation(config: &Self::Config, animation: FemResultsAnimation) -> Self::Config;
    fn addressed_window_id(cfg: &ConfigView<'_, NoConfig>, view: &ViewModel, requested: Option<&str>) -> Result<String, Fault>;
    fn addressed_to(window_id: &str, config: Self::Config) -> WindowConfigMutation;
    fn dirty_scope() -> semio_framework::kernel::UiDirtyScope;
}

/// 🫧️ What one playback command publishes: the config lane (transport settings, resting phase) and
/// the results window's transient lane (the running clock) — the retained route folds both into
/// one `CompleteWithEphemeral` step.
pub struct FemPlaybackStep<M> {
    pub emit: Emit<M, NoConfigMutation>,
    pub window_transient: Vec<WindowTransientMutation>,
}

impl<M> Default for FemPlaybackStep<M> {
    fn default() -> Self {
        Self { emit: Emit::default(), window_transient: Vec::new() }
    }
}

/// 🔁️ The hop that arms — or keeps — the playback clock of ONE results window.
///
/// `Effect::DispatchAction` carries no window of its own; the React ShellHost redispatches it under
/// the `resolvedTargetViewState` of the dispatch that emitted it, which is how the chain keeps
/// addressing the window the user pressed play in. `windowId` rides along as the address the hop was
/// armed for.
pub fn rearm_effect<T: FemPlaybackTransport>(window_id: &str) -> Effect {
    Effect::DispatchAction {
        req: semio_framework_plugin::RequestId(T::REARM_REQUEST),
        action: TICK_ACTION.into(),
        args: Some(semio_framework_value::DslValue::object([("windowId".to_string(), semio_framework_value::DslValue::String(window_id.to_string()))])),
        delay_ms: ANIMATION_TICK_MS,
    }
}

/// 🧾️ The step that publishes `animation` as the window's resting transport and clears its clock.
pub fn resting_step<T: FemPlaybackTransport>(window_id: &str, current: &T::Config, animation: FemResultsAnimation) -> FemPlaybackStep<T::Mutation> {
    FemPlaybackStep {
        emit: Emit { window_config_mutations: vec![T::addressed_to(window_id, T::with_animation(current, animation))], ui_scope: T::dirty_scope(), ..Default::default() },
        window_transient: vec![addressed_to::<T::Owner>(window_id, None)],
    }
}
//#endregion 🔖️Transport

//#region 🔖️SetResultAnimation
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "result-animation")]
pub struct SetResultAnimation {
    pub phase: Option<f64>,
    pub playing: Option<bool>,
    pub speed: Option<f64>,
    pub loop_mode: Option<String>,
    pub waveform: Option<String>,
    pub field: Option<String>,
    pub value: Option<String>,
    /// 🪟️ The results window this gesture speaks for, as the panel control tagged it. A panel
    /// projection carries no `window_id` of its own, so the tag is the ONLY thing that keeps a split
    /// layout from retuning the pane the user is not looking at.
    pub window_id: Option<String>,
}

fn number(value: &str) -> Result<f64, Fault> {
    value.trim().parse::<f64>().map_err(|_| Fault::from(format!("fem.result-animation.value: '{value}' is not a number")))
}

fn flag(value: &str) -> Result<bool, Fault> {
    match value.trim() {
        "true" | "1" | "on" => Ok(true),
        "false" | "0" | "off" | "" => Ok(false),
        other => Err(Fault::from(format!("fem.result-animation.value: '{other}' is not a boolean"))),
    }
}

fn set_playing(animation: &mut FemResultsAnimation, playing: bool) {
    if playing {
        animation.start();
    } else {
        animation.playing = false;
    }
}

/// 🎚️ Applies ONE named transport field — the shape a persistent panel control can express.
pub fn apply_field(animation: &mut FemResultsAnimation, field: &str, value: &str) -> Result<(), Fault> {
    match field {
        "phase" => animation.phase = number(value)?.clamp(0.0, 1.0),
        "phaseStep" => animation.phase = (animation.phase + number(value)?).rem_euclid(1.0),
        "playing" => set_playing(animation, flag(value)?),
        "speed" => animation.speed = number(value)?.clamp(ANIMATION_SPEED_MINIMUM, ANIMATION_SPEED_MAXIMUM),
        "loopMode" => animation.loop_mode = FemLoopMode::try_from(value).map_err(Fault::from)?,
        "waveform" => animation.waveform = FemWaveform::try_from(value).map_err(Fault::from)?,
        "reverse" => animation.reverse = flag(value)?,
        other => return Err(Fault::from(format!("fem.result-animation.field: '{other}' is not a playback field"))),
    }
    Ok(())
}

/// 🔀️ Merges everything the payload names into `animation`; names nothing ⇒ toggle play/pause.
pub fn merge(payload: &SetResultAnimation, animation: &mut FemResultsAnimation) -> Result<(), Fault> {
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
        animation.loop_mode = FemLoopMode::try_from(mode).map_err(Fault::from)?;
        named = true;
    }
    if let Some(waveform) = payload.waveform.as_deref() {
        animation.waveform = FemWaveform::try_from(waveform).map_err(Fault::from)?;
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

/// 🎬️ The retained route: the gesture lands on the transport with the window's RUNNING clock folded
/// in first — a pause rests exactly where the animation was, a seek or a retune mid-playback
/// starts the next frame from the phase the user sees — and a clock that existed is cleared so the
/// chain restarts from the published config instead of a stale frame. `clock` is `None` both when
/// the gesture carried no window tag (the keyboard chord; the next tick parks the clock itself) and
/// when the window is not playing.
///
/// 🕰️ ONE clock per window: only the TRANSITION into `playing` arms a hop. Moving the phase slider
/// mid-playback, or pressing play twice, must never leave two chains ticking the same window — each
/// would advance the phase by its own frame, so the structure would run at double speed and never
/// slow down again.
pub fn set_result_animation_step<T: FemPlaybackTransport>(payload: &SetResultAnimation, cfg: &ConfigView<'_, NoConfig>, view: &ViewModel, clock: Option<FemPlaybackClock>) -> Result<FemPlaybackStep<T::Mutation>, Fault> {
    let window_id = T::addressed_window_id(cfg, view, payload.window_id.as_deref())?;
    let current = T::current(cfg);
    let resting = T::animation(&current);
    let mut animation = clock.map_or(resting, |clock| clock.parked_into(&resting));
    merge(payload, &mut animation)?;
    let effects = if animation.playing && !resting.playing { vec![rearm_effect::<T>(&window_id)] } else { Vec::new() };
    let window_transient = clock.map(|_| addressed_to::<T::Owner>(&window_id, None)).into_iter().collect();
    Ok(FemPlaybackStep { emit: Emit { window_config_mutations: vec![T::addressed_to(&window_id, T::with_animation(&current, animation))], effects, ui_scope: T::dirty_scope(), ..Default::default() }, window_transient })
}
//#endregion 🔖️SetResultAnimation

//#region 🔖️Fem2d
/// 📊️ The fem 2d results window as a playback transport.
pub struct Fem2dResultsPlayback;

impl FemPlaybackTransport for Fem2dResultsPlayback {
    type Mutation = Fem2dMutation;
    type Config = results::config::Fem2dResultsWindowConfig;
    type Owner = results::transient::Fem2dResultsWindowTransientOwner;
    const REARM_REQUEST: u64 = 141;

    fn current(cfg: &ConfigView<'_, NoConfig>) -> Self::Config {
        results::config::current(cfg)
    }

    fn animation(config: &Self::Config) -> FemResultsAnimation {
        config.animation
    }

    fn with_animation(config: &Self::Config, animation: FemResultsAnimation) -> Self::Config {
        Self::Config { animation, ..config.clone() }
    }

    fn addressed_window_id(cfg: &ConfigView<'_, NoConfig>, view: &ViewModel, requested: Option<&str>) -> Result<String, Fault> {
        results::config::addressed_window_id(cfg, view, requested)
    }

    fn addressed_to(window_id: &str, config: Self::Config) -> WindowConfigMutation {
        results::config::addressed_to(window_id, config)
    }

    /// 🧹️ The results body and the panel that reads its transport.
    fn dirty_scope() -> semio_framework::kernel::UiDirtyScope {
        semio_framework::kernel::UiDirtyScope::Partial { window_bodies: vec![results::BODY_KEY.to_owned()], panel_bodies: vec![crate::editor::fem2d::panels::results::BODY_KEY.to_owned()], utilities: false, tools: false, engagements: false, measures: false, labels: false }
    }
}

pub fn handle(_payload: &SetResultAnimation, _doc: &ArtifactView<'_, Fem2dSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Fem2dMutation, NoConfigMutation>, Fault> {
    Err(Fault::from("fem.result-animation.window-context-required"))
}

/// 🗂️ Merges the gesture into the addressed fem 2d results window's playback state (batch route: no clock).
pub fn handle_window(payload: &SetResultAnimation, _doc: &ArtifactView<'_, Fem2dSnapshot>, cfg: &ConfigView<'_, NoConfig>, view: &ViewModel) -> Result<Emit<Fem2dMutation, NoConfigMutation>, Fault> {
    set_result_animation_step::<Fem2dResultsPlayback>(payload, cfg, view, None).map(|step| step.emit)
}
//#endregion 🔖️Fem2d

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
