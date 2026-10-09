//! ✂️ The reshape tools: offset, trim, extend, align and split. Each of them writes one mutation that computes its numbers from the authored geometry, so the history row is exact.
//!
//! * Offset: press a wall, move to the side it is copied to (the distance snaps to 5 cm) and click; one `offset-wall` creates the parallel wall.
//! * Trim and extend: press the target wall, then press the walls to meet it; trim keeps the part of the wall under the pointer and cuts the other end back to the target, extend moves
//!   the end nearest the pointer out to it. One `trim-extend-wall` per press.
//! * Align: with elements selected, press the reference element near the edge or the middle line to align to; one `align-elements` sets every selected element onto that line.
//! * Split: press a beam where it parts (`split-beam`), or press a slab and click the second point of the cut line (`split-slab`); the new half gets an id minted from the authoring seed.
//!
//! Every tool shows the result as a ghost before the click, a refused result as a warning.

use super::plane::{axis_bulge, axis_ends, dist, flatten, from_point2, point2, project, same, P};
use super::session::{length_label, Mark, Pick, Pointer, Preview, Step, Style, Tool, ToolContext, ToolEvent, PICK_PIXELS};
use super::transform::{ghost, outline};
use crate::editor::bim::modes::edit::windows::plan;
use crate::mutations::modify::cut::{locate, meeting, offset_axis, trim_extend};
use crate::mutations::modify::map::{align_gap, bounds};
use crate::mutations::modify::{AlignAxis, AlignEdge, WallEnd};
use crate::{Axis, ModelMutation};

/// 📏️ The step the distance of an offset snaps to, in metres.
pub const OFFSET_STEP: f64 = 0.05;
/// 📏️ The flattening tolerance of a previewed arc, in metres.
const FLATTEN_TOLERANCE: f64 = 0.01;
/// 📏️ The length of the mark across a beam where it would split, in metres.
const CUT_MARK: f64 = 0.6;
/// 📏️ How far a reference line mark reaches past the extent of its element, in metres.
const LINE_REACH: f64 = 1.0;

/// ✂️ Which reshape tool this is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Offset,
    Trim,
    Extend,
    Align,
    Split,
}

/// ✂️ The state of one reshape tool: the element it holds (the wall to offset, the target of a trim, the slab to cut), the point it was given and the pointer.
pub struct Reshape {
    kind: Kind,
    held: Option<String>,
    start: Option<P>,
    current: P,
    pointer: Pointer,
    moved: bool,
}

//#region 🔖️Picking
fn element_at(ctx: &ToolContext<'_>, pointer: &Pointer) -> Option<String> {
    let linework = ctx.storey().and_then(|storey| ctx.inference.plan_linework.get(storey))?;
    plan::pick(linework, (pointer.at[0], pointer.at[1]), pointer.tolerance * PICK_PIXELS)
}

fn wall_at(ctx: &ToolContext<'_>, pointer: &Pointer) -> Option<String> {
    element_at(ctx, pointer).filter(|id| ctx.snapshot.walls.contains_key(id))
}

fn target_of(ctx: &ToolContext<'_>, id: &str) -> Option<(String, String)> {
    crate::editor::bim::entities::kind_holding(ctx.snapshot, id).map(|row| (row.kind.to_string(), id.to_string()))
}

fn axis_path(axis: &Axis) -> Vec<P> {
    let (start, end) = axis_ends(axis);
    flatten(start, end, axis_bulge(axis), FLATTEN_TOLERANCE)
}

fn written(ctx: &ToolContext<'_>, mutation: ModelMutation, created: Option<(String, String)>) -> Step {
    let step = Step::write(ctx, mutation);
    match created {
        Some(target) if step.refused.is_none() => Step { pick: Some(Pick { targets: vec![target], merge: "replace" }), ..step },
        _ => step,
    }
}
//#endregion 🔖️Picking

//#region 🔖️Meaning
/// ↔️ The signed distance of an offset: how far the pointer stands from the axis, positive on the left of its direction, snapped to [`OFFSET_STEP`]; none when it rounds to zero.
pub fn offset_distance(axis: &Axis, pointer: P) -> Option<f64> {
    let on = project(axis, pointer);
    let signed = if on.side < 0.0 { -on.distance } else { on.distance };
    let snapped = (signed / OFFSET_STEP).round() * OFFSET_STEP;
    (snapped.abs() > 1e-9).then_some(snapped)
}

/// ✂️ The end of `wall` that a trim or an extend to `target` moves when the pointer stands at `at`: a trim keeps the part under the pointer and moves the end on the other side of the meeting
/// point, an extend moves the end nearest the pointer. None when the axes never meet.
pub fn end_to_move(kind: Kind, wall: &Axis, target: &Axis, at: P) -> Option<WallEnd> {
    let here = locate(wall, point2(at));
    if kind == Kind::Extend {
        return Some(if here < 0.5 { WallEnd::Start } else { WallEnd::End });
    }
    let meet = meeting(wall, target, point2(at))?;
    Some(if here < meet { WallEnd::End } else { WallEnd::Start })
}

/// 🎯️ The line of an element that an alignment sets the selection onto: the one of the six lines of its extent (the two edges and the middle on each coordinate) that the pointer is nearest to.
pub fn reference_line(extent: [f64; 4], at: P) -> (AlignAxis, AlignEdge, f64) {
    let middle = [(extent[0] + extent[2]) / 2.0, (extent[1] + extent[3]) / 2.0];
    [
        (AlignAxis::X, AlignEdge::Min, extent[0], (at[0] - extent[0]).abs()),
        (AlignAxis::X, AlignEdge::Center, middle[0], (at[0] - middle[0]).abs()),
        (AlignAxis::X, AlignEdge::Max, extent[2], (at[0] - extent[2]).abs()),
        (AlignAxis::Y, AlignEdge::Min, extent[1], (at[1] - extent[1]).abs()),
        (AlignAxis::Y, AlignEdge::Center, middle[1], (at[1] - middle[1]).abs()),
        (AlignAxis::Y, AlignEdge::Max, extent[3], (at[1] - extent[3]).abs()),
    ]
    .into_iter()
    .min_by(|a, b| a.3.total_cmp(&b.3))
    .map(|(axis, edge, line, _)| (axis, edge, line))
    .unwrap_or((AlignAxis::X, AlignEdge::Min, extent[0]))
}
//#endregion 🔖️Meaning

impl Reshape {
    pub fn new(kind: Kind) -> Self {
        Self { kind, held: None, start: None, current: [0.0, 0.0], pointer: Pointer { at: [0.0, 0.0], modifiers: Default::default(), tolerance: 0.01 }, moved: false }
    }

    fn reset(&mut self) {
        *self = Self::new(self.kind);
    }

    fn offset(&mut self, ctx: &mut ToolContext<'_>, wall: &str) -> Step {
        let Some(distance) = ctx.snapshot.walls.get(wall).and_then(|row| offset_distance(&row.axis, self.current)) else { return Step::default() };
        let new_id = ctx.mint("wall");
        let mutation = ModelMutation::OffsetWall(crate::mutations::offset_wall::OffsetWall { id: wall.to_string(), new_id: new_id.clone(), distance });
        let step = written(ctx, mutation, Some(("wall".to_string(), new_id)));
        self.reset();
        step
    }

    fn trim(&mut self, ctx: &ToolContext<'_>, target: &str, pointer: &Pointer) -> Step {
        let Some(wall) = wall_at(ctx, pointer).filter(|wall| wall != target) else { return Step::default() };
        let (Some(row), Some(other)) = (ctx.snapshot.walls.get(&wall), ctx.snapshot.walls.get(target)) else { return Step::default() };
        let Some(end) = end_to_move(self.kind, &row.axis, &other.axis, pointer.at) else { return Step::refuse(super::session::REJECTED) };
        Step::write(ctx, ModelMutation::TrimExtendWall(crate::mutations::trim_extend_wall::TrimExtendWall { id: wall, end, target: target.to_string() }))
    }

    fn align(&mut self, ctx: &ToolContext<'_>, pointer: &Pointer) -> Step {
        let ids: Vec<String> = ctx.selected.iter().filter(|id| crate::mutations::elements::placement(ctx.snapshot, id).is_some()).cloned().collect();
        let Some(hit) = element_at(ctx, pointer) else { return Step::default() };
        if ids.is_empty() {
            return target_of(ctx, &hit).map_or_else(Step::default, |target| Step::select(vec![target], "replace"));
        }
        if ids.contains(&hit) {
            return Step::default();
        }
        let Some(extent) = crate::mutations::elements::placement(ctx.snapshot, &hit).and_then(|placement| bounds(&placement)) else { return Step::default() };
        let (axis, edge, target) = reference_line(extent, pointer.at);
        Step::write(ctx, ModelMutation::AlignElements(crate::mutations::align_elements::AlignElements { ids, axis, edge, target }))
    }

    fn split(&mut self, ctx: &mut ToolContext<'_>, pointer: &Pointer) -> Step {
        if let Some(slab) = self.held.clone() {
            let at = ctx.snapped(pointer, self.start, &[slab.clone()], &[]).point;
            let Some(start) = self.start.filter(|start| !same(*start, at)) else { return Step::default() };
            let new_id = ctx.mint("slab");
            let mutation = ModelMutation::SplitSlab(crate::mutations::split_slab::SplitSlab { id: slab, new_id: new_id.clone(), line_start: point2(start), line_end: point2(at) });
            let step = written(ctx, mutation, Some(("slab".to_string(), new_id)));
            self.reset();
            return step;
        }
        let Some(hit) = element_at(ctx, pointer) else { return Step::default() };
        if ctx.snapshot.slabs.contains_key(&hit) {
            self.held = Some(hit.clone());
            self.start = Some(ctx.snapped(pointer, None, &[hit], &[]).point);
            return Step::default();
        }
        let Some(t) = beam_fraction(ctx, &hit, pointer.at) else { return Step::default() };
        let new_id = ctx.mint("beam");
        written(ctx, ModelMutation::SplitBeam(crate::mutations::split_beam::SplitBeam { id: hit, t, new_id: new_id.clone() }), Some(("beam".to_string(), new_id)))
    }

    fn down(&mut self, ctx: &mut ToolContext<'_>, pointer: &Pointer) -> Step {
        self.moved = true;
        self.pointer = *pointer;
        self.current = pointer.at;
        match self.kind {
            Kind::Offset => match self.held.clone() {
                Some(wall) => self.offset(ctx, &wall),
                None => {
                    self.held = wall_at(ctx, pointer);
                    Step::default()
                }
            },
            Kind::Trim | Kind::Extend => match self.held.clone() {
                Some(target) => self.trim(ctx, &target, pointer),
                None => {
                    self.held = wall_at(ctx, pointer);
                    match self.held.as_ref().and_then(|wall| target_of(ctx, wall)) {
                        Some(target) => Step::select(vec![target], "replace"),
                        None => Step::default(),
                    }
                }
            },
            Kind::Align => self.align(ctx, pointer),
            Kind::Split => self.split(ctx, pointer),
        }
    }
}

/// 🔢️ The fraction of a beam at which the pointer stands, rounded to a thousandth; none for an element that is no beam.
pub fn beam_fraction(ctx: &ToolContext<'_>, id: &str, at: P) -> Option<f64> {
    let beam = ctx.snapshot.beams.get(id)?;
    let axis = Axis::Line { start: beam.start, end: beam.end };
    let length = dist(from_point2(beam.start), from_point2(beam.end));
    (length > 0.0).then(|| (project(&axis, at).offset / length * 1000.0).round() / 1000.0)
}

//#region 🔖️Marks
fn highlighted(ctx: &ToolContext<'_>, id: &str) -> Vec<Mark> {
    match ctx.snapshot.walls.get(id) {
        Some(wall) => vec![Mark::path(&axis_path(&wall.axis), false, Style::Selection)],
        None => outline(ctx, &[id.to_string()]),
    }
}

fn offset_marks(ctx: &ToolContext<'_>, wall: &str, at: P) -> Vec<Mark> {
    let mut marks = highlighted(ctx, wall);
    let Some(row) = ctx.snapshot.walls.get(wall) else { return marks };
    if let Some(distance) = offset_distance(&row.axis, at) {
        let probe = ModelMutation::OffsetWall(crate::mutations::offset_wall::OffsetWall { id: wall.to_string(), new_id: "probe".into(), distance });
        if let Some(shifted) = offset_axis(&row.axis, distance) {
            marks.push(Mark::path(&axis_path(&shifted), false, if ctx.accepts(&probe) { Style::Ghost } else { Style::Warning }));
        }
        marks.push(Mark::label(at, format!("{distance:.2} m")));
    }
    marks
}

fn trim_marks(ctx: &ToolContext<'_>, kind: Kind, target: &str, at: P, pointer: &Pointer) -> Vec<Mark> {
    let mut marks = highlighted(ctx, target);
    let Some(wall) = wall_at(ctx, pointer).filter(|wall| wall != target) else { return marks };
    let (Some(row), Some(other)) = (ctx.snapshot.walls.get(&wall), ctx.snapshot.walls.get(target)) else { return marks };
    if let Some(end) = end_to_move(kind, &row.axis, &other.axis, at) {
        let result = trim_extend(&row.axis, end, &other.axis);
        match result {
            Ok(trimmed) => marks.push(Mark::path(&axis_path(&trimmed.axis), false, Style::Ghost)),
            Err(_) => marks.push(Mark::path(&axis_path(&row.axis), false, Style::Warning)),
        }
    }
    marks
}

fn align_marks(ctx: &ToolContext<'_>, pointer: &Pointer) -> Vec<Mark> {
    let ids: Vec<String> = ctx.selected.iter().filter(|id| crate::mutations::elements::placement(ctx.snapshot, id).is_some()).cloned().collect();
    let Some(hit) = element_at(ctx, pointer).filter(|hit| !ids.contains(hit)) else { return Vec::new() };
    if ids.is_empty() {
        return highlighted(ctx, &hit);
    }
    let Some(extent) = crate::mutations::elements::placement(ctx.snapshot, &hit).and_then(|placement| bounds(&placement)) else { return Vec::new() };
    let (axis, edge, line) = reference_line(extent, pointer.at);
    let mut marks = vec![if axis == AlignAxis::X {
        Mark::path(&[[line, extent[1] - LINE_REACH], [line, extent[3] + LINE_REACH]], false, Style::Guide)
    } else {
        Mark::path(&[[extent[0] - LINE_REACH, line], [extent[2] + LINE_REACH, line]], false, Style::Guide)
    }];
    for id in &ids {
        let Some(own) = crate::mutations::elements::placement(ctx.snapshot, id).and_then(|placement| bounds(&placement)) else { continue };
        let gap = align_gap(own, axis, edge, line);
        let vector = if axis == AlignAxis::X { [gap, 0.0] } else { [0.0, gap] };
        marks.extend(ghost(ctx, std::slice::from_ref(id), &|p| [p[0] + vector[0], p[1] + vector[1]]));
    }
    marks
}

fn split_marks(ctx: &ToolContext<'_>, held: Option<&String>, start: Option<P>, pointer: &Pointer) -> Vec<Mark> {
    if let (Some(slab), Some(start)) = (held, start) {
        let at = ctx.snapped(pointer, Some(start), std::slice::from_ref(slab), &[]).point;
        let mut marks = highlighted(ctx, slab);
        marks.push(Mark::path(&[start, at], false, Style::Guide));
        marks.push(length_label(start, at));
        return marks;
    }
    let Some(hit) = element_at(ctx, pointer) else { return Vec::new() };
    if ctx.snapshot.slabs.contains_key(&hit) {
        return highlighted(ctx, &hit);
    }
    let (Some(beam), Some(t)) = (ctx.snapshot.beams.get(&hit), beam_fraction(ctx, &hit, pointer.at)) else { return Vec::new() };
    let (start, end) = (from_point2(beam.start), from_point2(beam.end));
    let length = dist(start, end);
    let (along, across) = ([(end[0] - start[0]) / length, (end[1] - start[1]) / length], CUT_MARK / 2.0);
    let centre = [start[0] + (end[0] - start[0]) * t, start[1] + (end[1] - start[1]) * t];
    let mark = [[centre[0] - along[1] * across, centre[1] + along[0] * across], [centre[0] + along[1] * across, centre[1] - along[0] * across]];
    let probe = ModelMutation::SplitBeam(crate::mutations::split_beam::SplitBeam { id: hit, t, new_id: "probe".into() });
    vec![Mark::path(&mark, false, if ctx.accepts(&probe) { Style::Ghost } else { Style::Warning })]
}
//#endregion 🔖️Marks

impl Tool for Reshape {
    fn anchor(&self) -> Option<P> {
        self.start
    }

    fn event(&mut self, ctx: &mut ToolContext<'_>, event: &ToolEvent) -> Step {
        match event {
            ToolEvent::Down(pointer) => self.down(ctx, pointer),
            ToolEvent::Move(pointer) => {
                self.moved = true;
                self.pointer = *pointer;
                self.current = pointer.at;
                Step::default()
            }
            ToolEvent::Escape | ToolEvent::Lost => {
                self.reset();
                Step::default()
            }
            ToolEvent::Up(_) | ToolEvent::Double(_) | ToolEvent::Finish => Step::default(),
        }
    }

    fn preview(&self, ctx: &ToolContext<'_>) -> Preview {
        if !self.moved {
            return Preview::default();
        }
        let marks = match (self.kind, self.held.as_ref()) {
            (Kind::Offset, None) | (Kind::Trim | Kind::Extend, None) => wall_at(ctx, &self.pointer).map_or_else(Vec::new, |wall| highlighted(ctx, &wall)),
            (Kind::Offset, Some(wall)) => offset_marks(ctx, wall, self.current),
            (Kind::Trim | Kind::Extend, Some(target)) => trim_marks(ctx, self.kind, target, self.current, &self.pointer),
            (Kind::Align, _) => align_marks(ctx, &self.pointer),
            (Kind::Split, held) => split_marks(ctx, held, self.start, &self.pointer),
        };
        Preview::of(marks)
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
