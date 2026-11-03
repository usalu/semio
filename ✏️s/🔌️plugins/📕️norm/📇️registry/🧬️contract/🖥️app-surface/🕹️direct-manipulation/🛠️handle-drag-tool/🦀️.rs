//! 🛠️ Handle drag statechart: `Idle -> Hover -> Armed -> Dragging -> Commit | Abort`; pure, so pointer, keyboard and peers replay the same events. See [`super`].

use super::{nudge, snap, Modifiers};
use crate::{DragPreview, FieldWrite};
use semio_framework_value::{DslValue, Number};
use semio_s_plugin_norm_structural_scene::{Handle, Quantise};

/// 📍️ Where a handle gesture stands; `s` is the drag parameter in handle units.
#[derive(Clone, Debug, PartialEq)]
pub enum DragState {
    Idle,
    Hover { handle: String, s: f64 },
    Armed { handle: String, origin: f64 },
    Dragging { handle: String, origin: f64, s: f64, entry: String },
}

/// 🎬️ One input of the statechart, from the pointer, the keyboard or the host phase vocabulary.
#[derive(Clone, Debug, PartialEq)]
pub enum DragEvent {
    Hover { handle: Option<String>, s: f64 },
    Press { handle: String, s: f64 },
    Move { s: f64, modifiers: Modifiers },
    Nudge { direction: f64, modifiers: Modifiers },
    Entry(char),
    EntryBackspace,
    Commit,
    Release,
    Abort(AbortReason),
}

/// 🛑️ Why a gesture ended without committing.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AbortReason {
    Escape,
    CaptureLost,
    Blur,
    Retired,
}

/// 📤️ What the host must do after one step.
#[derive(Clone, Debug, PartialEq)]
pub enum DragEffect {
    None,
    Preview(DragPreview),
    Commit(Vec<FieldWrite>),
    Cleared,
}

/// 🌊️ Streamed phase vocabulary shared with the host gesture bindings (`stream`, `commit`, `abort` + reason).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DragPhase {
    Stream,
    Commit,
    Abort(AbortReason),
}

impl DragPhase {
    /// 🔡️ Reads a binding's `phase` and an abort's `reason` (`blur`, `captureLost`, `retired`; absent = `escape`); `None` for an unknown spelling.
    pub fn parse(phase: &str, reason: Option<&str>) -> Option<Self> {
        match (phase, reason) {
            ("stream", _) => Some(Self::Stream),
            ("commit", _) => Some(Self::Commit),
            ("abort", None | Some("tool" | "escape")) => Some(Self::Abort(AbortReason::Escape)),
            ("abort", Some("captureLost" | "baseMoved" | "frozen")) => Some(Self::Abort(AbortReason::CaptureLost)),
            ("abort", Some("blur")) => Some(Self::Abort(AbortReason::Blur)),
            ("abort", Some("retired")) => Some(Self::Abort(AbortReason::Retired)),
            _ => None,
        }
    }

    /// 🎬️ The statechart event of a phase at parameter `s`.
    pub fn event(self, s: f64, modifiers: Modifiers) -> DragEvent {
        match self {
            Self::Stream => DragEvent::Move { s, modifiers },
            Self::Commit => DragEvent::Release,
            Self::Abort(reason) => DragEvent::Abort(reason),
        }
    }
}

/// 🔢️ Parses typed numeric entry with `.` or `,` as decimal separator.
pub fn parse_entry(entry: &str) -> Option<f64> {
    entry.trim().replace(',', ".").parse::<f64>().ok().filter(|v| v.is_finite())
}

/// 🔢️ The text a freshly opened numeric field starts with: the bound field value at `s` to twelve significant digits (no float noise, no loss for tiny values); `None` for token fields, which are chosen by stepping and not typed.
pub fn entry_field_text(handle: &Handle, s: f64) -> Option<String> {
    let binding = handle.writes.first().filter(|w| !matches!(w.quantise, Some(Quantise::Token { .. })))?;
    let rounded = format!("{:.11e}", binding.value_at(s)).parse::<f64>().ok().filter(|value| value.is_finite())?;
    Some(rounded.to_string())
}

/// ✍️ Every bound field value for parameter `s` as a provisional write, quantised but not re-snapped.
pub fn writes_for(handle: &Handle, s: f64) -> Vec<FieldWrite> {
    handle.writes.iter().map(|w| FieldWrite { path: w.path.clone(), value: w.token_at(s).map_or_else(|| DslValue::Number(Number::Float(w.value_at(s))), |t| DslValue::String(t.to_string())) }).collect()
}

/// 🛠️ The statechart over the handles of the current selection; `seq` is the store sequence the drag started against.
pub struct HandleDragTool<'a> {
    pub handles: &'a [Handle],
    pub seq: u64,
}

impl HandleDragTool<'_> {
    fn find(&self, id: &str) -> Option<&Handle> {
        self.handles.iter().find(|h| h.id == id)
    }

    fn dragging(&self, handle: &Handle, origin: f64, s: f64, entry: String) -> (DragState, DragEffect) {
        (DragState::Dragging { handle: handle.id.clone(), origin, s, entry: entry.clone() }, DragEffect::Preview(DragPreview { handle: handle.id.clone(), writes: writes_for(handle, s), started_seq: self.seq, origin, s, entry }))
    }

    fn settle(&self, handle: &Handle, origin: f64, s: f64) -> (DragState, DragEffect) {
        let writes = writes_for(handle, s);
        let effect = if writes == writes_for(handle, origin) { DragEffect::Cleared } else { DragEffect::Commit(writes) };
        (DragState::Hover { handle: handle.id.clone(), s }, effect)
    }

    /// 🎬️ One transition. Unknown handles and events that do not apply keep the state; a missing handle ends the gesture without trace.
    pub fn step(&self, state: DragState, event: DragEvent) -> (DragState, DragEffect) {
        use DragEvent as E;
        use DragState as S;
        match (state, event) {
            (S::Dragging { handle, origin, .. }, E::Abort(reason)) => {
                let kept = matches!(reason, AbortReason::Escape);
                (if kept { S::Hover { handle, s: origin } } else { S::Idle }, DragEffect::Cleared)
            }
            (S::Armed { .. }, E::Abort(_)) | (S::Hover { .. }, E::Abort(AbortReason::Retired)) => (S::Idle, DragEffect::None),
            (state, E::Abort(_)) => (state, DragEffect::None),
            (S::Idle | S::Hover { .. }, E::Hover { handle: Some(id), s }) if self.find(&id).is_some() => (S::Hover { handle: id, s }, DragEffect::None),
            (S::Idle | S::Hover { .. }, E::Hover { handle: None, .. }) => (S::Idle, DragEffect::None),
            (S::Idle | S::Hover { .. }, E::Press { handle, s }) if self.find(&handle).is_some() => (S::Armed { handle, origin: s }, DragEffect::None),
            (S::Armed { handle, origin }, E::Move { s, modifiers }) => match self.find(&handle) {
                Some(h) => self.dragging(h, origin, snap(h, s, modifiers).s, String::new()),
                None => (S::Idle, DragEffect::None),
            },
            (S::Armed { handle, origin }, E::Release) => (S::Hover { handle, s: origin }, DragEffect::None),
            (S::Hover { handle, s: current }, E::Nudge { direction, modifiers }) => match self.find(&handle) {
                Some(h) => self.dragging(h, current, nudge(h, current, direction, modifiers).s, String::new()),
                None => (S::Idle, DragEffect::None),
            },
            (S::Hover { handle, s: current }, E::Commit) => match self.find(&handle).and_then(|h| entry_field_text(h, current).map(|text| (h, text))) {
                Some((h, text)) => self.typed(h, current, current, text),
                None => (S::Hover { handle, s: current }, DragEffect::None),
            },
            (S::Hover { handle, s: current }, E::Entry(c)) if c.is_ascii_digit() || matches!(c, '-' | '.' | ',') => match self.find(&handle) {
                Some(h) => self.typed(h, current, current, c.to_string()),
                None => (S::Idle, DragEffect::None),
            },
            (S::Dragging { handle, origin, entry, .. }, E::Move { s: raw, modifiers }) if entry.is_empty() => match self.find(&handle) {
                Some(h) => self.dragging(h, origin, snap(h, raw, modifiers).s, entry),
                None => (S::Idle, DragEffect::Cleared),
            },
            (S::Dragging { handle, origin, s, .. }, E::Nudge { direction, modifiers }) => match self.find(&handle) {
                Some(h) => self.dragging(h, origin, nudge(h, s, direction, modifiers).s, String::new()),
                None => (S::Idle, DragEffect::Cleared),
            },
            (S::Dragging { handle, origin, s, mut entry }, E::Entry(c)) if c.is_ascii_digit() || matches!(c, '-' | '.' | ',') => match self.find(&handle) {
                Some(h) => {
                    entry.push(c);
                    self.typed(h, origin, s, entry)
                }
                None => (S::Idle, DragEffect::Cleared),
            },
            (S::Dragging { handle, origin, s, mut entry }, E::EntryBackspace) => match self.find(&handle) {
                Some(h) => {
                    entry.pop();
                    self.typed(h, origin, s, entry)
                }
                None => (S::Idle, DragEffect::Cleared),
            },
            (S::Dragging { handle, origin, s, .. }, E::Commit | E::Release) => match self.find(&handle) {
                Some(h) => self.settle(h, origin, s),
                None => (S::Idle, DragEffect::Cleared),
            },
            (state, _) => (state, DragEffect::None),
        }
    }

    fn typed(&self, handle: &Handle, origin: f64, s: f64, entry: String) -> (DragState, DragEffect) {
        let first = handle.writes.first();
        let exact = parse_entry(&entry).zip(first).map(|(value, w)| handle.limits.as_ref().map_or(w.map.invert(value), |l| l.clamp(w.map.invert(value))));
        self.dragging(handle, origin, exact.unwrap_or(s), entry)
    }
}
