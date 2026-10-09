//! ⌨️ Typed geometry entry: the keyboard twin of the pointer. A line typed into the window's entry field means one point, resolved against the gesture in progress, and every tool takes
//! that point as a click (press and release at the exact point, no snapping). The grammar, all in metres and degrees counter-clockwise from +X (a section takes distance along the line
//! and height instead):
//!
//! * `x, y` (or `x; y`, or `x y`): the absolute point;
//! * `@dx, dy`: the offset from the point the gesture is anchored at (the origin before the first point);
//! * `length<angle` (or `@length<angle`): the polar offset from the anchor;
//! * `length`: that far from the anchor towards where the pointer last was (towards +X when it never was);
//! * nothing: finish the gesture, as Enter does.

use super::plane::{angle, dist, polar, COINCIDENT, P};

/// 🔬️ The metres one pixel reaches for a typed point: so small that a pick and a snap only find what lies exactly there.
pub const REACH: f64 = 1e-4;

/// 🚫️ The refusal of a line that names no point.
pub const INVALID: &str = "bim.tool.input-invalid";

/// ⌨️ What a typed line means before it meets the gesture's anchor.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Entry {
    Empty,
    Absolute(P),
    Relative(P),
    Polar { length: f64, degrees: f64 },
    Length(f64),
}

fn number(text: &str) -> Option<f64> {
    text.trim().trim_end_matches('°').trim_end_matches('m').trim().parse::<f64>().ok().filter(|value| value.is_finite())
}

/// 📍️ The absolute point a text names: two numbers joined by a comma, a semicolon or white space.
pub fn point_of(text: &str) -> Option<P> {
    let text = text.trim();
    let parts: Vec<&str> = if text.contains([',', ';']) { text.split([',', ';']).map(str::trim).collect() } else { text.split_whitespace().collect() };
    match parts.as_slice() {
        [x, y] => Some([number(x)?, number(y)?]),
        _ => None,
    }
}

/// ⌨️ The entry a typed line means; none when it is no entry at all.
pub fn parse(text: &str) -> Option<Entry> {
    let text = text.trim();
    if text.is_empty() {
        return Some(Entry::Empty);
    }
    let (relative, body) = text.strip_prefix('@').map_or((false, text), |rest| (true, rest.trim()));
    if let Some((length, degrees)) = body.split_once('<') {
        return Some(Entry::Polar { length: number(length)?, degrees: number(degrees)? });
    }
    if let Some(offset) = point_of(body) {
        return Some(if relative { Entry::Relative(offset) } else { Entry::Absolute(offset) });
    }
    number(body).map(Entry::Length)
}

fn clean(value: f64) -> f64 {
    (value * 1e9).round() / 1e9 + 0.0
}

/// 📍️ The point an entry names: against the `anchor` of the gesture (the origin without one) and, for a bare length, the direction `toward` where the pointer last was.
pub fn resolve(entry: &Entry, anchor: Option<P>, toward: Option<P>) -> Option<P> {
    let from = anchor.unwrap_or([0.0, 0.0]);
    let point = match entry {
        Entry::Empty => return None,
        Entry::Absolute(at) => *at,
        Entry::Relative(offset) => [from[0] + offset[0], from[1] + offset[1]],
        Entry::Polar { length, degrees } => polar(from, degrees.to_radians(), *length),
        Entry::Length(length) => polar(from, toward.filter(|toward| dist(from, *toward) > COINCIDENT).map_or(0.0, |toward| angle(from, toward)), *length),
    };
    Some([clean(point[0]), clean(point[1])])
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
