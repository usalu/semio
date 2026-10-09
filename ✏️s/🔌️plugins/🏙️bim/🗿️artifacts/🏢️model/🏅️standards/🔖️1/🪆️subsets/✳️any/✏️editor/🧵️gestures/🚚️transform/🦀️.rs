//! 🚚️ The transform tools: move and rotate the selection (or the element under the first press when nothing is selected). Move: press the base point and drag, or click the base
//! point and click the target; one `move-elements` writes the vector. Rotate: click the pivot, click the reference direction, click the target direction; one `rotate-elements`
//! writes the turn between the two directions, quantised to 15 degrees while shift is held. Both show the plan outline of what moves as a ghost; openings follow their host by
//! inference, so they are never part of the set.

use super::plane::{angle, dist, flatten, point2, quantised, rotate_about, same, translate, P};
use super::session::{length_label, Mark, Pointer, Preview, Step, Style, Tool, ToolContext, ToolEvent, PICK_PIXELS};
use crate::editor::bim::entities::kind_holding;
use crate::editor::bim::modes::edit::windows::plan;
use crate::standards::v1::subsets::any::schema::inferences::plan_linework::PlanVertex;
use crate::ModelMutation;
use std::f64::consts::PI;

/// 🖱️ How many pixels a press must travel before it counts as a drag.
const DRAG_PIXELS: f64 = 4.0;
/// 📏️ The flattening tolerance of a ghost arc, in metres.
const FLATTEN_TOLERANCE: f64 = 0.01;
/// 🧭️ The quantisation of a rotation while shift is held (15 degrees).
pub const ROTATION_STEP: f64 = PI / 12.0;

/// 🚚️ Which transform this is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Move,
    Rotate,
}

/// 🚚️ The state of one transform: the ids it moves, the base (or pivot) point, the reference direction of a rotation, the pointer and whether the press is still down.
pub struct Transform {
    kind: Kind,
    ids: Vec<String>,
    base: Option<P>,
    reference: Option<P>,
    current: P,
    held: bool,
}

impl Transform {
    pub fn new(kind: Kind) -> Self {
        Self { kind, ids: Vec::new(), base: None, reference: None, current: [0.0, 0.0], held: false }
    }

    fn reset(&mut self) {
        *self = Self::new(self.kind);
    }

    /// 🎯️ The ids that move: the placed elements of the selection, else the element under `at`.
    fn pick_ids(&self, ctx: &ToolContext<'_>, pointer: &Pointer) -> (Vec<String>, Option<(String, String)>) {
        let placed = |id: &String| crate::mutations::elements::placement(ctx.snapshot, id).is_some();
        let selected: Vec<String> = ctx.selected.iter().filter(|id| placed(id)).cloned().collect();
        if !selected.is_empty() {
            return (selected, None);
        }
        let hit = ctx.storey().and_then(|storey| ctx.inference.plan_linework.get(storey)).and_then(|linework| plan::pick(linework, (pointer.at[0], pointer.at[1]), pointer.tolerance * PICK_PIXELS)).filter(placed);
        let target = hit.as_ref().and_then(|id| kind_holding(ctx.snapshot, id).map(|row| (row.kind.to_string(), id.clone())));
        (hit.into_iter().collect(), target)
    }

    fn target(&self, ctx: &ToolContext<'_>, pointer: &Pointer) -> P {
        match (self.kind, self.base) {
            (Kind::Move, base) => ctx.snapped(pointer, base, &self.ids, &[]).point,
            (Kind::Rotate, None) => ctx.snapped(pointer, None, &[], &[]).point,
            (Kind::Rotate, Some(_)) => pointer.at,
        }
    }

    fn turn(&self, current: P, quantise: bool) -> Option<(P, f64)> {
        let (pivot, reference) = (self.base?, self.reference?);
        let turn = angle(pivot, current) - angle(pivot, reference);
        let turn = (turn + PI).rem_euclid(2.0 * PI) - PI;
        Some((pivot, if quantise { quantised(turn, ROTATION_STEP) } else { turn }))
    }

    fn commit_move(&mut self, ctx: &mut ToolContext<'_>, to: P) -> Step {
        let Some(base) = self.base else { return Step::default() };
        let vector = [to[0] - base[0], to[1] - base[1]];
        if vector[0].hypot(vector[1]) < 1e-9 {
            self.reset();
            return Step::default();
        }
        let step = Step::write(ctx, ModelMutation::MoveElements(crate::mutations::move_elements::MoveElements { ids: self.ids.clone(), vector: point2(vector) }));
        self.reset();
        step
    }

    fn commit_rotate(&mut self, ctx: &mut ToolContext<'_>, to: P, quantise: bool) -> Step {
        let Some((pivot, turn)) = self.turn(to, quantise) else { return Step::default() };
        if turn.abs() < 1e-9 {
            self.reset();
            return Step::default();
        }
        let step = Step::write(ctx, ModelMutation::RotateElements(crate::mutations::rotate_elements::RotateElements { ids: self.ids.clone(), pivot: point2(pivot), angle: turn }));
        self.reset();
        step
    }

    fn down(&mut self, ctx: &mut ToolContext<'_>, pointer: &Pointer) -> Step {
        let at = self.target(ctx, pointer);
        self.current = at;
        let mut select = None;
        if self.ids.is_empty() {
            let (ids, target) = self.pick_ids(ctx, pointer);
            self.ids = ids;
            select = target;
        }
        if self.ids.is_empty() {
            return Step::default();
        }
        let step = match (self.kind, self.base, self.reference) {
            (Kind::Move, None, _) => {
                self.base = Some(at);
                self.held = true;
                Step::default()
            }
            (Kind::Move, Some(_), _) => return self.commit_move(ctx, at),
            (Kind::Rotate, None, _) => {
                self.base = Some(at);
                Step::default()
            }
            (Kind::Rotate, Some(pivot), None) => {
                if !same(pivot, at) {
                    self.reference = Some(at);
                }
                Step::default()
            }
            (Kind::Rotate, Some(_), Some(_)) => return self.commit_rotate(ctx, at, pointer.modifiers.shift),
        };
        match select {
            Some(target) => Step::select(vec![target], "replace"),
            None => step,
        }
    }
}

/// 👻️ The plan outlines of `ids` with every point mapped by `map`, drawn as ghosts: what a tool shows of the elements it would move, copy or mirror.
pub fn ghost(ctx: &ToolContext<'_>, ids: &[String], map: &dyn Fn(P) -> P) -> Vec<Mark> {
    traced(ctx, ids, map, Style::Ghost)
}

/// 🔦️ The plan outlines of `ids` where they stand, drawn as the selection: what a tool shows of the elements it will act on before the first click.
pub fn outline(ctx: &ToolContext<'_>, ids: &[String]) -> Vec<Mark> {
    traced(ctx, ids, &|p| p, Style::Selection)
}

fn traced(ctx: &ToolContext<'_>, ids: &[String], map: &dyn Fn(P) -> P, style: Style) -> Vec<Mark> {
    let Some(linework) = ctx.storey().and_then(|storey| ctx.inference.plan_linework.get(storey)) else { return Vec::new() };
    let path = |vertices: &[PlanVertex], closed: bool| -> Vec<P> {
        let edges = if closed { vertices.len() } else { vertices.len().saturating_sub(1) };
        let mut points: Vec<P> = vertices.first().map(|v| vec![map([v.x, v.y])]).unwrap_or_default();
        for index in 0..edges {
            let (a, b) = (&vertices[index], &vertices[(index + 1) % vertices.len()]);
            points.extend(flatten([a.x, a.y], [b.x, b.y], a.bulge, FLATTEN_TOLERANCE).into_iter().skip(1).map(map));
        }
        points
    };
    let regions = linework.regions.iter().filter(|region| ids.contains(&region.element)).map(|region| Mark::path(&path(&region.outer, true), true, style));
    let lines = linework.polylines.iter().filter(|line| ids.contains(&line.element)).map(|line| Mark::path(&path(&line.vertices, line.closed), line.closed, style));
    regions.chain(lines).collect()
}

impl Tool for Transform {
    fn anchor(&self) -> Option<P> {
        self.base
    }

    fn event(&mut self, ctx: &mut ToolContext<'_>, event: &ToolEvent) -> Step {
        match event {
            ToolEvent::Down(pointer) => self.down(ctx, pointer),
            ToolEvent::Move(pointer) => {
                self.current = self.target(ctx, pointer);
                Step::default()
            }
            ToolEvent::Up(pointer) => {
                if self.kind != Kind::Move || !self.held {
                    return Step::default();
                }
                self.held = false;
                let to = self.target(ctx, pointer);
                match self.base {
                    Some(base) if dist(base, to) > DRAG_PIXELS * pointer.tolerance => self.commit_move(ctx, to),
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
        let Some(base) = self.base else { return Preview::default() };
        let mut marks = Vec::new();
        match self.kind {
            Kind::Move => {
                let vector = [self.current[0] - base[0], self.current[1] - base[1]];
                marks.extend(ghost(ctx, &self.ids, &|p| translate(p, vector)));
                marks.push(Mark::path(&[base, self.current], false, Style::Guide));
                marks.push(length_label(base, self.current));
            }
            Kind::Rotate => {
                marks.push(Mark::dot(base, Style::Handle));
                if let Some(reference) = self.reference {
                    marks.push(Mark::path(&[base, reference], false, Style::Guide));
                    marks.push(Mark::path(&[base, self.current], false, Style::Ghost));
                    if let Some((_, turn)) = self.turn(self.current, false) {
                        marks.extend(ghost(ctx, &self.ids, &|p| rotate_about(p, base, turn)));
                        marks.push(Mark::label(self.current, format!("{:.1}°", turn.to_degrees())));
                    }
                }
            }
        }
        Preview::of(marks)
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
