//! 👯️ The duplicate tools: copy, mirror and the linear and radial array. All of them act on the selected placed elements (or, when nothing is selected, the element under the first
//! press, which they select) and write one mutation that creates records with ids minted from the authoring seed, so the result is one history row and the copies end up selected.
//!
//! * Copy: press the base point and drag, or click the base point and click the target; one `copy-elements` writes the vector.
//! * Mirror: click the two points of the mirror line; one `mirror-elements` mirrors the elements where they stand, with control or command held on the second click it creates mirrored copies.
//! * Array: click the base point, click the point of the first copy (the spacing), move along that direction to show how many copies fit and click: one `array-elements` writes a linear pattern.
//! * Radial array: click the centre, click the reference point, turn the pointer to show the step (5 degrees, 15 with shift) and click: one `array-elements` fills the circle with copies.
//!
//! Every tool shows the plan outline of what it would create as ghosts, so the preview travels in the window transient like every other mark.

use super::plane::{angle, dist, point2, quantised, rotate_about, same, translate, COINCIDENT, P};
use super::session::{length_label, Mark, Pick, Pointer, Preview, Step, Style, Tool, ToolContext, ToolEvent, PICK_PIXELS};
use super::transform::{ghost, outline};
use crate::editor::bim::entities::kind_holding;
use crate::editor::bim::modes::edit::windows::plan;
use crate::mutations::modify::{self, ArrayPattern, MAX_COPIES};
use crate::ModelMutation;
use std::f64::consts::{PI, TAU};

/// 🖱️ How many pixels a press must travel before it counts as a drag.
const DRAG_PIXELS: f64 = 4.0;
/// 🧭️ The quantisation of a radial step (5 degrees), and of its shift variant (15 degrees).
pub const RADIAL_STEP: f64 = PI / 36.0;
pub const RADIAL_STEP_COARSE: f64 = PI / 12.0;
/// 👻️ The most copies a preview draws as ghosts.
const GHOSTS: u32 = 16;

/// 👯️ Which duplicate tool this is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Copy,
    Mirror,
    Array,
    Radial,
}

/// 👯️ The state of one duplicate tool: the ids it acts on, the first and the second point it was given, the pointer and whether the first press is still down.
pub struct Duplicate {
    kind: Kind,
    ids: Vec<String>,
    first: Option<P>,
    second: Option<P>,
    current: P,
    held: bool,
    pointer: Option<Pointer>,
}

//#region 🔖️Geometry
/// 🪞️ The image of a point under the reflection about the line through `a` and `b`.
pub fn reflect(p: P, a: P, b: P) -> P {
    let (dx, dy) = (b[0] - a[0], b[1] - a[1]);
    let length = dx.hypot(dy);
    if length <= COINCIDENT {
        return p;
    }
    let (ux, uy) = (dx / length, dy / length);
    let (vx, vy) = (p[0] - a[0], p[1] - a[1]);
    let along = vx * ux + vy * uy;
    [a[0] + 2.0 * along * ux - vx, a[1] + 2.0 * along * uy - vy]
}

/// 🔢️ How many copies of a linear array reach the pointer: the pointer's distance along the spacing from the origin in whole spacings, between 1 and the most an array makes.
pub fn array_count(origin: P, next: P, pointer: P) -> u32 {
    let step = dist(origin, next);
    if step <= COINCIDENT {
        return 1;
    }
    let along = ((pointer[0] - origin[0]) * (next[0] - origin[0]) + (pointer[1] - origin[1]) * (next[1] - origin[1])) / step;
    ((along / step).round().max(1.0) as u32).min(MAX_COPIES)
}

/// 🧭️ The step of a radial array: the turn from the reference point to the pointer about the centre, in half a turn either way, quantised.
pub fn radial_step(center: P, reference: P, pointer: P, coarse: bool) -> f64 {
    let turn = (angle(center, pointer) - angle(center, reference) + PI).rem_euclid(TAU) - PI;
    quantised(turn, if coarse { RADIAL_STEP_COARSE } else { RADIAL_STEP })
}

/// 🔢️ How many copies fill the circle at a step: the whole turns of the step in a full circle, less the original; none when the step does not turn.
pub fn radial_count(step: f64) -> u32 {
    if step.abs() < 1e-6 {
        return 0;
    }
    ((TAU / step.abs() + 1e-9).floor() as i64 - 1).clamp(0, i64::from(MAX_COPIES)) as u32
}
//#endregion 🔖️Geometry

//#region 🔖️Picking
fn pick_ids(ctx: &ToolContext<'_>, pointer: &Pointer) -> (Vec<String>, Option<(String, String)>) {
    let placed = |id: &String| crate::mutations::elements::placement(ctx.snapshot, id).is_some();
    let selected: Vec<String> = ctx.selected.iter().filter(|id| placed(id)).cloned().collect();
    if !selected.is_empty() {
        return (selected, None);
    }
    let hit = ctx.storey().and_then(|storey| ctx.inference.plan_linework.get(storey)).and_then(|linework| plan::pick(linework, (pointer.at[0], pointer.at[1]), pointer.tolerance * PICK_PIXELS)).filter(placed);
    let target = hit.as_ref().and_then(|id| kind_holding(ctx.snapshot, id).map(|row| (row.kind.to_string(), id.clone())));
    (hit.into_iter().collect(), target)
}

/// 🎯️ The `(kind, id)` of every element the copies `1..=copies` of a mutation with `prefix` create, in creation order: the minted ids carry the kind of their source.
fn minted_targets(ctx: &ToolContext<'_>, ids: &[String], prefix: &str, copies: u32) -> Vec<(String, String)> {
    let Ok(sources) = modify::sources(ctx.snapshot, ids) else { return Vec::new() };
    let kinds: Vec<Option<&'static str>> = sources.elements.iter().map(|source| kind_holding(ctx.snapshot, source).map(|row| row.kind)).collect();
    let mut targets = Vec::new();
    for copy in 1..=copies {
        for (ordinal, kind) in kinds.iter().enumerate() {
            if let Some(kind) = kind {
                targets.push((kind.to_string(), modify::mint(prefix, copy, ordinal)));
            }
        }
    }
    targets
}
//#endregion 🔖️Picking

impl Duplicate {
    pub fn new(kind: Kind) -> Self {
        Self { kind, ids: Vec::new(), first: None, second: None, current: [0.0, 0.0], held: false, pointer: None }
    }

    fn reset(&mut self) {
        *self = Self::new(self.kind);
    }

    /// 🔢️ Whether the next press reads the pointer itself (the count of an array, the step of a radial one) and not a snapped point.
    fn counting(&self) -> bool {
        matches!(self.kind, Kind::Array | Kind::Radial) && self.second.is_some()
    }

    fn finish(&mut self, ctx: &ToolContext<'_>, mutation: ModelMutation, targets: Vec<(String, String)>) -> Step {
        let step = Step::write(ctx, mutation);
        self.reset();
        if step.refused.is_some() || targets.is_empty() {
            return step;
        }
        Step { pick: Some(Pick { targets, merge: "replace" }), ..step }
    }

    fn copy(&mut self, ctx: &mut ToolContext<'_>, base: P, to: P) -> Step {
        let vector = [to[0] - base[0], to[1] - base[1]];
        if vector[0].hypot(vector[1]) < 1e-9 {
            self.reset();
            return Step::default();
        }
        let prefix = ctx.mint("copy");
        let targets = minted_targets(ctx, &self.ids, &prefix, 1);
        let mutation = ModelMutation::CopyElements(crate::mutations::copy_elements::CopyElements { ids: self.ids.clone(), vector: point2(vector), prefix });
        self.finish(ctx, mutation, targets)
    }

    fn mirror(&mut self, ctx: &mut ToolContext<'_>, start: P, end: P, copies: bool) -> Step {
        if same(start, end) {
            self.reset();
            return Step::default();
        }
        let prefix = copies.then(|| ctx.mint("mirror"));
        let targets = prefix.as_ref().map_or_else(Vec::new, |prefix| minted_targets(ctx, &self.ids, prefix, 1));
        let mutation = ModelMutation::MirrorElements(crate::mutations::mirror_elements::MirrorElements { ids: self.ids.clone(), line_start: point2(start), line_end: point2(end), prefix });
        self.finish(ctx, mutation, targets)
    }

    fn array(&mut self, ctx: &mut ToolContext<'_>, pattern: ArrayPattern) -> Step {
        let prefix = ctx.mint("array");
        let targets = minted_targets(ctx, &self.ids, &prefix, pattern.count());
        let mutation = ModelMutation::ArrayElements(crate::mutations::array_elements::ArrayElements { ids: self.ids.clone(), prefix, pattern });
        self.finish(ctx, mutation, targets)
    }

    fn down(&mut self, ctx: &mut ToolContext<'_>, pointer: &Pointer) -> Step {
        let at = if self.counting() { pointer.at } else { ctx.snapped(pointer, self.first, &self.ids, &[]).point };
        self.current = at;
        let mut select = None;
        if self.ids.is_empty() {
            let (ids, target) = pick_ids(ctx, pointer);
            self.ids = ids;
            select = target;
        }
        if self.ids.is_empty() {
            return Step::default();
        }
        let step = match (self.kind, self.first, self.second) {
            (Kind::Copy, None, _) => {
                self.first = Some(at);
                self.held = true;
                Step::default()
            }
            (Kind::Copy, Some(base), _) => return self.copy(ctx, base, at),
            (Kind::Mirror, None, _) | (Kind::Array | Kind::Radial, None, _) => {
                self.first = Some(at);
                Step::default()
            }
            (Kind::Mirror, Some(start), _) => return self.mirror(ctx, start, at, pointer.modifiers.subtractive()),
            (Kind::Array | Kind::Radial, Some(origin), None) => {
                if !same(origin, at) {
                    self.second = Some(at);
                }
                Step::default()
            }
            (Kind::Array, Some(origin), Some(next)) => {
                let spacing = point2([next[0] - origin[0], next[1] - origin[1]]);
                return self.array(ctx, ArrayPattern::Linear { count: array_count(origin, next, at), spacing });
            }
            (Kind::Radial, Some(center), Some(reference)) => {
                let step = radial_step(center, reference, at, pointer.modifiers.shift);
                let count = radial_count(step);
                if count == 0 {
                    self.reset();
                    return Step::default();
                }
                return self.array(ctx, ArrayPattern::Radial { count, center: point2(center), step });
            }
        };
        match select {
            Some(target) => Step::select(vec![target], "replace"),
            None => step,
        }
    }

    fn marks(&self, ctx: &ToolContext<'_>, first: P) -> Vec<Mark> {
        let current = self.current;
        let mut marks = Vec::new();
        match (self.kind, self.second) {
            (Kind::Copy, _) => {
                let vector = [current[0] - first[0], current[1] - first[1]];
                marks.extend(ghost(ctx, &self.ids, &|p| translate(p, vector)));
                marks.push(Mark::path(&[first, current], false, Style::Guide));
                marks.push(length_label(first, current));
            }
            (Kind::Mirror, _) => {
                marks.push(Mark::path(&[first, current], false, Style::Guide));
                marks.extend(ghost(ctx, &self.ids, &|p| reflect(p, first, current)));
            }
            (Kind::Array, None) => {
                marks.push(Mark::path(&[first, current], false, Style::Guide));
                marks.push(length_label(first, current));
            }
            (Kind::Array, Some(next)) => {
                let (count, step) = (array_count(first, next, current), [next[0] - first[0], next[1] - first[1]]);
                for copy in 1..=count.min(GHOSTS) {
                    let vector = [step[0] * f64::from(copy), step[1] * f64::from(copy)];
                    marks.extend(ghost(ctx, &self.ids, &|p| translate(p, vector)));
                }
                marks.push(Mark::path(&[first, next], false, Style::Guide));
                marks.push(Mark::label(current, format!("× {count}")));
            }
            (Kind::Radial, None) => {
                marks.push(Mark::dot(first, Style::Handle));
                marks.push(Mark::path(&[first, current], false, Style::Guide));
            }
            (Kind::Radial, Some(reference)) => {
                let step = radial_step(first, reference, current, false);
                let count = radial_count(step);
                for copy in 1..=count.min(GHOSTS) {
                    let turn = step * f64::from(copy);
                    marks.extend(ghost(ctx, &self.ids, &|p| rotate_about(p, first, turn)));
                }
                marks.push(Mark::dot(first, Style::Handle));
                marks.push(Mark::path(&[first, reference], false, Style::Guide));
                marks.push(Mark::path(&[first, current], false, Style::Ghost));
                marks.push(Mark::label(current, format!("× {count} · {:.0}°", step.to_degrees())));
            }
        }
        marks
    }
}

impl Tool for Duplicate {
    fn anchor(&self) -> Option<P> {
        self.first
    }

    fn event(&mut self, ctx: &mut ToolContext<'_>, event: &ToolEvent) -> Step {
        match event {
            ToolEvent::Down(pointer) => {
                let step = self.down(ctx, pointer);
                self.pointer = Some(*pointer);
                step
            }
            ToolEvent::Move(pointer) => {
                self.pointer = Some(*pointer);
                self.current = if self.counting() { pointer.at } else { ctx.snapped(pointer, self.first, &self.ids, &[]).point };
                Step::default()
            }
            ToolEvent::Up(pointer) => {
                if self.kind != Kind::Copy || !self.held {
                    return Step::default();
                }
                self.held = false;
                let to = ctx.snapped(pointer, self.first, &self.ids, &[]).point;
                match self.first {
                    Some(base) if dist(base, to) > DRAG_PIXELS * pointer.tolerance => self.copy(ctx, base, to),
                    _ => Step::default(),
                }
            }
            ToolEvent::Escape | ToolEvent::Lost => {
                self.reset();
                Step::default()
            }
            ToolEvent::Double(_) | ToolEvent::Finish => Step::default(),
        }
    }

    fn preview(&self, ctx: &ToolContext<'_>) -> Preview {
        match (self.first, self.pointer) {
            (Some(first), _) => Preview::of(self.marks(ctx, first)),
            (None, Some(pointer)) if self.ids.is_empty() => Preview::of(outline(ctx, &pick_ids(ctx, &pointer).0)),
            (None, _) => Preview::of(outline(ctx, &self.ids)),
        }
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
