//! ⏯️ Playback of parameter tracks and mode shapes: the ephemeral, local-only transport state, its command reducer, the sampled track frame and the oscillation phase; reduced motion never starts or advances an animation on its own.

use super::overlays::number;
use super::scene::{LocalizedCopy, Motion, ParameterTrack, TrackKind};
use std::f64::consts::TAU;

pub const SWEEP_SECONDS: f64 = 8.0;
pub const VISUAL_HZ: f64 = 0.5;
pub const RAMP_SECONDS: f64 = 2.0;

pub const ACTION_SET_ACTIVE_FIELD: &str = "viewportSetActiveField";
pub const ACTION_SELECT_TRACK: &str = "viewportSelectTrack";
pub const ACTION_SEEK: &str = "viewportSeek";
pub const ACTION_TOGGLE_PLAY: &str = "viewportTogglePlay";
pub const ACTION_STEP: &str = "viewportStep";
pub const ACTION_SET_SPEED: &str = "viewportSetPlaybackSpeed";

/// ⏱️ Transport state of one viewport: selected track, parameter value, play flag and the animation clock in seconds; `reduced_motion` mirrors the user's system preference.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PlaybackState {
    pub track: Option<String>,
    pub value: f64,
    pub playing: bool,
    pub clock: f64,
    pub reduced_motion: bool,
}

/// 🕹️ Events that change the transport state.
#[derive(Clone, Debug, PartialEq)]
pub enum PlaybackCommand {
    Select(Option<String>),
    Seek(f64),
    Play,
    Pause,
    Toggle,
    Step(i32),
    Rewind,
    Tick(f64),
}

/// 🎞️ Animation factors of one frame: `phase` scales harmonic overlays, `ramp` scales ramped ones.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Frame {
    pub phase: f64,
    pub ramp: f64,
}

impl Default for Frame {
    fn default() -> Self {
        Self { phase: 1.0, ramp: 1.0 }
    }
}

/// 🔗️ Fields and layers a track selects at one parameter value: `fields` is `(from, to, weight)` for blending.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TrackSample {
    pub fields: Option<(String, Option<String>, f64)>,
    pub layer: Option<String>,
    pub managed_layers: Vec<String>,
}

impl PlaybackState {
    pub fn with_reduced_motion(self, reduced_motion: bool) -> Self {
        Self { reduced_motion, playing: self.playing && !reduced_motion, ..self }
    }
}

pub fn find<'a>(tracks: &'a [ParameterTrack], id: &str) -> Option<&'a ParameterTrack> {
    tracks.iter().find(|t| t.id == id)
}

fn snap(track: &ParameterTrack, value: f64) -> f64 {
    let steps = ((value - track.min) / track.step).round();
    (track.min + steps * track.step).clamp(track.min, track.max)
}

fn cyclic(track: &ParameterTrack) -> bool {
    track.looped || matches!(track.kind, TrackKind::Phase)
}

fn travel(track: &ParameterTrack, from: f64, delta: f64) -> (f64, bool) {
    let span = track.max - track.min;
    let to = from + delta;
    if cyclic(track) { (track.min + (to - track.min).rem_euclid(span), true) } else { (to.clamp(track.min, track.max), to > track.min && to < track.max) }
}

/// 🧮️ Next transport state after one command; unknown tracks are ignored and reduced motion refuses `Play`, `Toggle` to playing and `Tick`.
pub fn reduce(state: &PlaybackState, tracks: &[ParameterTrack], speed: f64, command: &PlaybackCommand) -> PlaybackState {
    let mut next = state.clone();
    let track = state.track.as_deref().and_then(|id| find(tracks, id));
    match command {
        PlaybackCommand::Select(id) => {
            let chosen = id.as_deref().and_then(|id| find(tracks, id));
            next.track = chosen.map(|t| t.id.clone());
            next.value = chosen.map_or(0.0, |t| t.min);
            next.playing = false;
        }
        PlaybackCommand::Seek(value) => {
            if let Some(t) = track { next.value = snap(t, *value); }
        }
        PlaybackCommand::Play => next.playing = !state.reduced_motion,
        PlaybackCommand::Pause => next.playing = false,
        PlaybackCommand::Toggle => next.playing = !state.playing && !state.reduced_motion,
        PlaybackCommand::Step(n) => {
            if let Some(t) = track { next.value = snap(t, travel(t, state.value, f64::from(*n) * t.step).0); }
        }
        PlaybackCommand::Rewind => {
            next.value = track.map_or(0.0, |t| t.min);
            next.clock = 0.0;
        }
        PlaybackCommand::Tick(dt) => {
            if state.playing && !state.reduced_motion && dt.is_finite() && *dt > 0.0 {
                let dt = dt * speed;
                next.clock += dt;
                if let Some(t) = track {
                    let (value, more) = travel(t, state.value, (t.max - t.min) / SWEEP_SECONDS * dt);
                    next.value = value;
                    next.playing = more;
                }
            }
        }
    }
    next
}

/// 🧭️ Command of a toolbar action id with its text and number arguments.
pub fn command_of(action: &str, text: Option<&str>, number: Option<f64>) -> Option<PlaybackCommand> {
    match action {
        ACTION_SELECT_TRACK => Some(PlaybackCommand::Select(text.filter(|t| !t.is_empty()).map(str::to_string))),
        ACTION_SEEK => number.map(PlaybackCommand::Seek),
        ACTION_TOGGLE_PLAY => Some(PlaybackCommand::Toggle),
        ACTION_STEP => number.map(|n| PlaybackCommand::Step(n.round() as i32)),
        _ => None,
    }
}

/// 🌊️ Oscillation phase and ramp of the current transport state; the manual phase applies while paused and under reduced motion.
pub fn frame(state: &PlaybackState, tracks: &[ParameterTrack], manual_phase: f64) -> Frame {
    let track = state.track.as_deref().and_then(|id| find(tracks, id));
    let animating = state.playing && !state.reduced_motion;
    let phase = match track {
        Some(t) if matches!(t.kind, TrackKind::Phase) => (TAU * (state.value - t.min) / (t.max - t.min)).sin(),
        _ if animating => (TAU * VISUAL_HZ * state.clock).sin(),
        _ => manual_phase,
    };
    Frame { phase, ramp: if animating { (state.clock / RAMP_SECONDS).clamp(0.0, 1.0) } else { 1.0 } }
}

/// 📐️ Displacement multiplier of an overlay's motion in a frame.
pub fn motion_factor(motion: Motion, frame: Frame) -> f64 {
    match motion {
        Motion::Static => 1.0,
        Motion::Harmonic => frame.phase,
        Motion::Ramp => frame.ramp,
    }
}

/// 🔖️ Fields and layers a track selects at a parameter value.
pub fn sample(track: &ParameterTrack, value: f64) -> TrackSample {
    let managed_layers: Vec<String> = track.keyframes.iter().filter_map(|k| k.layer.clone()).collect();
    let discrete = matches!(track.kind, TrackKind::Mode | TrackKind::Combination);
    let nearest = || track.keyframes.iter().min_by(|a, b| (a.at - value).abs().total_cmp(&(b.at - value).abs()));
    let stepped = || track.keyframes.iter().rev().find(|k| k.at <= value && k.layer.is_some()).or(track.keyframes.iter().find(|k| k.layer.is_some()));
    if matches!(track.kind, TrackKind::Phase) || track.keyframes.is_empty() { return TrackSample::default(); }
    if discrete {
        let key = nearest();
        return TrackSample { fields: key.and_then(|k| k.field.clone()).map(|f| (f, None, 0.0)), layer: key.and_then(|k| k.layer.clone()), managed_layers };
    }
    let fielded: Vec<_> = track.keyframes.iter().filter(|k| k.field.is_some()).collect();
    let fields = match fielded.iter().position(|k| k.at > value) {
        _ if fielded.is_empty() => None,
        Some(0) => fielded[0].field.clone().map(|f| (f, None, 0.0)),
        None => fielded[fielded.len() - 1].field.clone().map(|f| (f, None, 0.0)),
        Some(i) => {
            let (a, b) = (fielded[i - 1], fielded[i]);
            a.field.clone().map(|f| (f, b.field.clone(), (value - a.at) / (b.at - a.at)))
        }
    };
    TrackSample { fields, layer: stepped().and_then(|k| k.layer.clone()), managed_layers }
}

fn clock_text(seconds: f64, german: bool) -> String {
    let (value, unit) = match seconds.abs() {
        s if s < 120.0 => (seconds, "s"),
        s if s < 7200.0 => (seconds / 60.0, "min"),
        s if s < 172_800.0 => (seconds / 3600.0, "h"),
        s if s < 63_072_000.0 => (seconds / 86_400.0, "d"),
        _ => (seconds / 31_557_600.0, "a"),
    };
    format!("{} {unit}", number(value, german, 1))
}

/// 🏷️ Readout of the current parameter value, for the scrubber label and the live region.
pub fn readout(track: &ParameterTrack, value: f64) -> LocalizedCopy {
    let key = track.keyframes.iter().min_by(|a, b| (a.at - value).abs().total_cmp(&(b.at - value).abs()));
    let text = |german: bool| match track.kind {
        TrackKind::Time | TrackKind::Age => clock_text(value, german),
        TrackKind::Phase => format!("{}°", number(360.0 * (value - track.min) / (track.max - track.min), german, 0)),
        TrackKind::Mode | TrackKind::Combination => key.map_or_else(|| number(value, german, 0), |k| if german { k.caption.de.clone() } else { k.caption.en.clone() }),
    };
    LocalizedCopy::new(format!("{}: {}", track.label.en, text(false)), format!("{}: {}", track.label.de, text(true)))
}
