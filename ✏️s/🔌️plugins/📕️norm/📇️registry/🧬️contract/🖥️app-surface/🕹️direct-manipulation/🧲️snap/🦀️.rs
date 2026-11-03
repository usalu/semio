//! 🧲️ Snapping of the drag parameter: explains which rule moved `s` and derives the keyboard nudge step. See [`super`].

use semio_s_plugin_norm_structural_scene::{Handle, HandleKind, Quantise};

/// 🏷️ The rule that decided a snapped value.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SnapSource {
    Free,
    Target,
    Catalogue,
    Increment,
    Grid,
    Limit,
}

/// 🧲️ A drag parameter after limits and snapping, with the rule that produced it.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Snapped {
    pub s: f64,
    pub source: SnapSource,
}

/// ⌨️ Modifier keys of one pointer or key event; `ctrl` frees the drag from snapping, `shift`/`alt` scale nudges.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Modifiers {
    pub shift: bool,
    pub alt: bool,
    pub ctrl: bool,
}

impl Modifiers {
    pub const NONE: Self = Self { shift: false, alt: false, ctrl: false };
    pub const SHIFT: Self = Self { shift: true, ..Self::NONE };
    pub const ALT: Self = Self { alt: true, ..Self::NONE };
    pub const CTRL: Self = Self { ctrl: true, ..Self::NONE };

    /// ✖️ Nudge multiplier: Shift x10, Alt x0.1, both cancel.
    pub fn scale(self) -> f64 {
        match (self.shift, self.alt) {
            (true, false) => 10.0,
            (false, true) => 0.1,
            _ => 1.0,
        }
    }
}

fn nearest(values: &[f64], s: f64, tolerance: f64) -> Option<f64> {
    values.iter().copied().filter(|v| (v - s).abs() <= tolerance).min_by(|a, b| (a - s).abs().total_cmp(&(b - s).abs()))
}

fn outside(handle: &Handle, value: f64) -> bool {
    handle.limits.as_ref().is_some_and(|limits| !limits.contains(value))
}

fn clamp(handle: &Handle, value: f64) -> f64 {
    handle.limits.as_ref().map_or(value, |limits| limits.clamp(value))
}

/// 🧲️ Limits first, then the nearest target or catalogue value, else increment or grid; `ctrl` keeps only the limits. Equals [`Handle::resolve_s`] without `ctrl`.
pub fn snap(handle: &Handle, raw: f64, modifiers: Modifiers) -> Snapped {
    let bounded = clamp(handle, raw);
    let rule = &handle.snap;
    let (moved, source) = if modifiers.ctrl {
        (bounded, SnapSource::Free)
    } else {
        let all: Vec<f64> = rule.targets.iter().chain(&rule.catalogue).copied().collect();
        match (nearest(&all, bounded, rule.tolerance), rule.increment, rule.grid) {
            (Some(v), _, _) => (v, if rule.targets.contains(&v) { SnapSource::Target } else { SnapSource::Catalogue }),
            (None, Some(step), _) => ((bounded / step).round() * step, SnapSource::Increment),
            (None, None, Some(step)) => ((bounded / step).round() * step, SnapSource::Grid),
            _ => (bounded, SnapSource::Free),
        }
    };
    Snapped { s: clamp(handle, moved), source: if outside(handle, raw) || outside(handle, moved) { SnapSource::Limit } else { source } }
}

/// 📐️ The schema step of the first bound field in handle units: its quantisation step (a round step, one for an integer, the smallest gap of a choice list, one decade fraction) divided by the slope of the affine map; `None` for token choices and unquantised fields.
pub fn schema_step(handle: &Handle) -> Option<f64> {
    let binding = handle.writes.first()?;
    let k = binding.map.k.abs();
    let step = match binding.quantise.as_ref()? {
        Quantise::Round { step } => *step,
        Quantise::Int => 1.0,
        Quantise::Choice { values } => {
            let mut sorted = values.clone();
            sorted.sort_by(f64::total_cmp);
            sorted.windows(2).map(|pair| pair[1] - pair[0]).filter(|gap| *gap > 0.0).min_by(f64::total_cmp)?
        }
        Quantise::Decades { per_decade } => 1.0 / per_decade,
        Quantise::Token { .. } => return None,
    };
    (step.is_finite() && step > 0.0 && k > 0.0).then_some(step / k)
}

/// 👣️ The base nudge step in handle units: the schema step of the bound field, else the increment, else the grid, else one percent of the limit span, else 1; Shift multiplies by ten, Alt divides by ten.
pub fn nudge_step(handle: &Handle, modifiers: Modifiers) -> f64 {
    let base = schema_step(handle).or(handle.snap.increment).or(handle.snap.grid).or_else(|| handle.limits.as_ref().map(|l| (l.max - l.min) / 100.0).filter(|span| *span > 0.0)).unwrap_or(1.0);
    base * modifiers.scale()
}

/// ⌨️ One keyboard nudge of `s` by `direction` (+1 or -1) steps, clamped to the limits and never snapped away again; a choice stepper moves one token and cycles past either end.
pub fn nudge(handle: &Handle, s: f64, direction: f64, modifiers: Modifiers) -> Snapped {
    if let (HandleKind::ChoiceStepper, Some(limits)) = (&handle.kind, &handle.limits) {
        let count = limits.max.round() - limits.min.round() + 1.0;
        let next = (s.round() - limits.min.round() + direction.signum()).rem_euclid(count) + limits.min.round();
        return Snapped { s: next, source: SnapSource::Increment };
    }
    let raw = s + direction.signum() * nudge_step(handle, modifiers);
    Snapped { s: clamp(handle, raw), source: if outside(handle, raw) { SnapSource::Limit } else { SnapSource::Free } }
}
