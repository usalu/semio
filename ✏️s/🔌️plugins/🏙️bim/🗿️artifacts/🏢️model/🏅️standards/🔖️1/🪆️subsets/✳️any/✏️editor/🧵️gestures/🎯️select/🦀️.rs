//! 🎯️ The select tool and the handles it owns. A press picks the topmost element under the pointer and selects it (shift adds, ctrl or meta subtracts, both invert); a press on
//! nothing starts a marquee (left to right selects what it contains, right to left what it touches). With one wall selected its two end handles and its midpoint handle
//! are live: dragging an end moves the wall's end (`set-wall-axis`), dragging the midpoint bends the wall through the pointer (`set-wall-axis` with a bulge). With one slab or roof selected every corner of its outline is a handle: dragging one
//! reshapes the outline (`set-slab-boundary`, `set-roof-footprint`). Pressing an opening and
//! dragging slides it along its host or onto another wall (`move-opening`, with the new host when it lands on another wall). In the section window the top line of every storey is a handle: dragging it sets the
//! storey height (`set-storey-height`) and every wall, opening and stair that follows the storey follows by inference.

use super::opening::{cut_outline, fitted_offset, nearest_host, HostHit};
use super::plane::{axis_ends, axis_length, axis_of, axis_point_at, bounds, bulge_through, dist, flatten, from_point2, point2, same, P};
use super::session::{length_label, Mark, Pointer, Preview, Step, Style, Tool, ToolContext, ToolEvent, PICK_PIXELS};
use crate::editor::bim::entities::kind_holding;
use crate::editor::bim::modes::edit::windows::plan;
use crate::mutations::placement::{placement_issue, width_of};
use crate::standards::v1::subsets::any::schema::inferences::plan_linework::PlanLinework;
use crate::{Axis, ModelInference, ModelMutation, ModelSnapshot, Vertex};
use std::collections::BTreeMap;

/// 📏️ The flattening tolerance of a dragged arc, in metres.
const FLATTEN_TOLERANCE: f64 = 0.005;
/// 📏️ The height step of the storey height handle and the least height it sets, in metres.
pub const HEIGHT_STEP: f64 = 0.05;
pub const HEIGHT_MINIMUM: f64 = 0.5;
/// 📏️ How far past the section line the storey top handles reach, in metres.
const SECTION_REACH: f64 = 0.5;
/// 🖱️ How many pixels a press must travel before a marquee or a drag counts.
const DRAG_PIXELS: f64 = 4.0;

/// 🔧️ Which handle of a wall is dragged.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HandleKind {
    Start,
    End,
    Curve,
}

#[derive(Clone, Debug, PartialEq)]
enum Mode {
    Idle,
    Marquee { start: P, current: P, merge: &'static str },
    Handle { wall: String, kind: HandleKind, current: P },
    Vertex { id: String, roof: bool, index: usize, current: P },
    Sliding { id: String, from: (String, f64), current: P, reach: f64 },
    Height { storey: String, value: f64 },
}

/// 🎯️ The select tool: its mode and the pointer position.
pub struct Select {
    mode: Mode,
}

impl Default for Select {
    fn default() -> Self {
        Self { mode: Mode::Idle }
    }
}

//#region 🔖️Handles
/// 🔧️ The handle positions of the one selected wall: start, end and the midpoint of its axis.
pub fn wall_handles(snapshot: &ModelSnapshot, selected: &[String]) -> Option<(String, [P; 3])> {
    let [only] = selected else { return None };
    let wall = snapshot.walls.get(only)?;
    let (start, end) = axis_ends(&wall.axis);
    Some((only.clone(), [start, end, axis_point_at(&wall.axis, axis_length(&wall.axis) / 2.0)]))
}

/// 🔧️ The corners of the outline of the one selected slab or roof, with whether it is a roof: the handles the author drags to reshape it.
pub fn outline_handles(snapshot: &ModelSnapshot, selected: &[String]) -> Option<(String, bool, Vec<P>)> {
    let [only] = selected else { return None };
    let corners = |vertices: &[Vertex]| vertices.iter().map(|vertex| from_point2(vertex.point)).collect();
    snapshot.slabs.get(only).map(|slab| (only.clone(), false, corners(&slab.boundary))).or_else(|| snapshot.roofs.get(only).map(|roof| (only.clone(), true, corners(&roof.footprint))))
}

/// 🔧️ The marks of the live handles in a plan: a handle dot on both ends and on the midpoint of the one selected wall, and on every corner of the one selected slab or roof.
pub fn plan_marks(snapshot: &ModelSnapshot, selected: &[String]) -> Vec<Mark> {
    let wall = wall_handles(snapshot, selected).map(|(_, handles)| handles.to_vec()).unwrap_or_default();
    let outline = outline_handles(snapshot, selected).map(|(_, _, corners)| corners).unwrap_or_default();
    wall.into_iter().chain(outline).map(|at| Mark::dot(at, Style::Handle)).collect()
}

/// 🔧️ The marks of the storey top handles in a section: a guide along the top of every storey with a handle dot at its left end.
pub fn section_marks(inference: &ModelInference, start: P, end: P) -> Vec<Mark> {
    let length = dist(start, end);
    inference.storey_levels.values().flat_map(|level| [Mark::path(&[[-SECTION_REACH, level.top_elevation], [length + SECTION_REACH, level.top_elevation]], false, Style::Guide), Mark::dot([-SECTION_REACH, level.top_elevation], Style::Handle)]).collect()
}

fn handle_at(handles: &[P; 3], at: P, reach: f64) -> Option<HandleKind> {
    [(HandleKind::Start, handles[0]), (HandleKind::End, handles[1]), (HandleKind::Curve, handles[2])].into_iter().filter(|(_, handle)| dist(*handle, at) <= reach).min_by(|a, b| dist(a.1, at).total_cmp(&dist(b.1, at))).map(|(kind, _)| kind)
}

fn corner_at(corners: &[P], at: P, reach: f64) -> Option<usize> {
    corners.iter().enumerate().filter(|(_, corner)| dist(**corner, at) <= reach).min_by(|a, b| dist(*a.1, at).total_cmp(&dist(*b.1, at))).map(|(index, _)| index)
}

/// 🔷️ The outline with its corner `index` moved to `to`, every bulge kept; none for an index past the end.
fn moved_corner(vertices: &[Vertex], index: usize, to: P) -> Option<Vec<Vertex>> {
    (index < vertices.len()).then(|| vertices.iter().enumerate().map(|(position, vertex)| if position == index { Vertex { point: point2(to), bulge: vertex.bulge } } else { vertex.clone() }).collect())
}

/// 🔷️ The mutation that moves corner `index` of the slab or roof `id` to `to`; none when the corner does not exist or does not move.
fn dragged_outline(snapshot: &ModelSnapshot, id: &str, roof: bool, index: usize, to: P) -> Option<ModelMutation> {
    if roof {
        let footprint = &snapshot.roofs.get(id)?.footprint;
        let moved = moved_corner(footprint, index, to).filter(|moved| moved != footprint)?;
        Some(ModelMutation::SetRoofFootprint(crate::mutations::set_roof_footprint::SetRoofFootprint { id: id.into(), footprint: moved }))
    } else {
        let slab = snapshot.slabs.get(id)?;
        let moved = moved_corner(&slab.boundary, index, to).filter(|moved| moved != &slab.boundary)?;
        Some(ModelMutation::SetSlabBoundary(crate::mutations::set_slab_boundary::SetSlabBoundary { id: id.into(), boundary: moved, holes: slab.holes.clone() }))
    }
}

fn dragged_axis(snapshot: &ModelSnapshot, wall: &str, kind: HandleKind, to: P) -> Option<Axis> {
    let axis = &snapshot.walls.get(wall)?.axis;
    let (start, end) = axis_ends(axis);
    match kind {
        HandleKind::Start => (!same(to, end)).then(|| axis_of(to, end, super::plane::axis_bulge(axis))),
        HandleKind::End => (!same(to, start)).then(|| axis_of(start, to, super::plane::axis_bulge(axis))),
        HandleKind::Curve => Some(axis_of(start, end, bulge_through(start, to, end).unwrap_or(0.0))),
    }
}
//#endregion 🔖️Handles

//#region 🔖️Picking
fn linework<'a>(ctx: &'a ToolContext<'_>) -> Option<&'a PlanLinework> {
    ctx.storey().and_then(|storey| ctx.inference.plan_linework.get(storey))
}

fn target_of(ctx: &ToolContext<'_>, id: &str) -> Option<(String, String)> {
    kind_holding(ctx.snapshot, id).map(|row| (row.kind.to_string(), id.to_string()))
}

fn elements_bounds(linework: &PlanLinework) -> BTreeMap<String, [f64; 4]> {
    let mut found: BTreeMap<String, Vec<P>> = BTreeMap::new();
    for region in linework.regions.iter().filter(|region| !region.element.is_empty()) {
        found.entry(region.element.clone()).or_default().extend(region.outer.iter().map(|v| [v.x, v.y]));
    }
    for line in linework.polylines.iter().filter(|line| !line.element.is_empty()) {
        found.entry(line.element.clone()).or_default().extend(line.vertices.iter().map(|v| [v.x, v.y]));
    }
    found.into_iter().filter_map(|(element, points)| bounds(points).map(|bounds| (element, bounds))).collect()
}

fn in_marquee(start: P, end: P) -> [f64; 4] {
    [start[0].min(end[0]), start[1].min(end[1]), start[0].max(end[0]), start[1].max(end[1])]
}

fn marquee_targets(ctx: &ToolContext<'_>, start: P, end: P) -> Vec<(String, String)> {
    let Some(linework) = linework(ctx) else { return Vec::new() };
    let rect = in_marquee(start, end);
    let crossing = end[0] < start[0];
    elements_bounds(linework)
        .into_iter()
        .filter(|(_, b)| if crossing { b[0] <= rect[2] && b[2] >= rect[0] && b[1] <= rect[3] && b[3] >= rect[1] } else { b[0] >= rect[0] && b[2] <= rect[2] && b[1] >= rect[1] && b[3] <= rect[3] })
        .filter_map(|(element, _)| target_of(ctx, &element))
        .collect()
}
//#endregion 🔖️Picking

impl Select {
    fn down(&mut self, ctx: &mut ToolContext<'_>, pointer: &Pointer) -> Step {
        let reach = pointer.tolerance * PICK_PIXELS;
        if let super::session::Surface::Section { start, end } = &ctx.surface {
            let length = dist(*start, *end);
            let [u, v] = pointer.at;
            let hit = ctx.inference.storey_levels.iter().find(|(_, level)| (v - level.top_elevation).abs() <= reach && u >= -SECTION_REACH - reach && u <= length + SECTION_REACH + reach).map(|(storey, level)| (storey.clone(), level.top_elevation));
            if let Some((storey, value)) = hit {
                self.mode = Mode::Height { storey, value };
            }
            return Step::default();
        }
        if let Some((wall, handles)) = wall_handles(ctx.snapshot, ctx.selected) {
            if let Some(kind) = handle_at(&handles, pointer.at, reach * 1.5) {
                let current = match kind {
                    HandleKind::Start => handles[0],
                    HandleKind::End => handles[1],
                    HandleKind::Curve => handles[2],
                };
                self.mode = Mode::Handle { wall, kind, current };
                return Step::default();
            }
        }
        if let Some((id, roof, corners)) = outline_handles(ctx.snapshot, ctx.selected) {
            if let Some(index) = corner_at(&corners, pointer.at, reach * 1.5) {
                self.mode = Mode::Vertex { id, roof, index, current: corners[index] };
                return Step::default();
            }
        }
        let hit = linework(ctx).and_then(|linework| plan::pick(linework, (pointer.at[0], pointer.at[1]), reach));
        let merge = pointer.modifiers.merge();
        match hit.and_then(|id| target_of(ctx, &id)) {
            Some((kind, id)) => {
                if let Some(opening) = ctx.snapshot.openings.get(&id) {
                    self.mode = Mode::Sliding { id: id.clone(), from: (opening.host.clone(), opening.offset), current: pointer.at, reach };
                }
                Step::select(vec![(kind, id)], merge)
            }
            None => {
                self.mode = Mode::Marquee { start: pointer.at, current: pointer.at, merge };
                if merge == "replace" { Step::select(Vec::new(), "replace") } else { Step::default() }
            }
        }
    }

    fn slide(&self, ctx: &ToolContext<'_>, id: &str, from: &(String, f64), at: P, reach: f64) -> Option<(HostHit, f64, ModelMutation, bool)> {
        let opening = ctx.snapshot.openings.get(id)?;
        let host = nearest_host(ctx.snapshot, ctx.storey()?, at, reach)?;
        let width = width_of(ctx.snapshot, opening)?;
        let offset = fitted_offset(host.length, width, host.offset)?;
        let mutation = ModelMutation::MoveOpening(crate::mutations::move_opening::MoveOpening { id: id.to_string(), offset, host: (host.host != opening.host).then(|| host.host.clone()) });
        let moved = host.host != from.0 || (offset - from.1).abs() > 1e-9;
        let fits = placement_issue(ctx.snapshot, Some(id), &host.host, host.length, offset, width).is_none();
        Some((host, offset, mutation, moved && fits))
    }

    fn release(&mut self, ctx: &mut ToolContext<'_>, pointer: &Pointer) -> Step {
        let reach = pointer.tolerance * PICK_PIXELS;
        let mode = std::mem::replace(&mut self.mode, Mode::Idle);
        match mode {
            Mode::Idle => Step::default(),
            Mode::Marquee { start, merge, .. } => {
                if dist(start, pointer.at) <= DRAG_PIXELS * pointer.tolerance {
                    return Step::default();
                }
                Step::select(marquee_targets(ctx, start, pointer.at), merge)
            }
            Mode::Handle { wall, kind, .. } => {
                let anchor = ctx.snapshot.walls.get(&wall).map(|row| axis_ends(&row.axis)).map(|(start, end)| if kind == HandleKind::Start { end } else { start });
                let to = if kind == HandleKind::Curve { pointer.at } else { ctx.snapped(pointer, anchor, std::slice::from_ref(&wall), &[]).point };
                match dragged_axis(ctx.snapshot, &wall, kind, to) {
                    Some(axis) if ctx.snapshot.walls.get(&wall).is_some_and(|row| row.axis != axis) => Step::write(ctx, ModelMutation::SetWallAxis(crate::mutations::set_wall_axis::SetWallAxis { id: wall, axis })),
                    _ => Step::default(),
                }
            }
            Mode::Vertex { id, roof, index, .. } => {
                let to = ctx.snapped(pointer, None, std::slice::from_ref(&id), &[]).point;
                dragged_outline(ctx.snapshot, &id, roof, index, to).map_or_else(Step::default, |mutation| Step::write(ctx, mutation))
            }
            Mode::Sliding { id, from, .. } => match self.slide(ctx, &id, &from, pointer.at, reach) {
                Some((_, _, mutation, true)) => Step::write(ctx, mutation),
                Some((_, _, _, false)) | None => Step::default(),
            },
            Mode::Height { storey, .. } => {
                let (Some(level), Some(row)) = (ctx.inference.storey_levels.get(&storey), ctx.snapshot.storeys.get(&storey)) else { return Step::default() };
                let height = (((pointer.at[1] - level.elevation) / HEIGHT_STEP).round() * HEIGHT_STEP).max(HEIGHT_MINIMUM);
                if (height - row.height).abs() < 1e-9 {
                    return Step::default();
                }
                Step::write(ctx, ModelMutation::SetStoreyHeight(crate::mutations::set_storey_height::SetStoreyHeight { id: storey, height }))
            }
        }
    }
}

impl Tool for Select {
    fn event(&mut self, ctx: &mut ToolContext<'_>, event: &ToolEvent) -> Step {
        match event {
            ToolEvent::Down(pointer) => {
                self.mode = Mode::Idle;
                self.down(ctx, pointer)
            }
            ToolEvent::Move(pointer) => {
                match &mut self.mode {
                    Mode::Marquee { current, .. } | Mode::Sliding { current, .. } => *current = pointer.at,
                    Mode::Handle { wall, kind, current } => {
                        let anchor = ctx.snapshot.walls.get(wall.as_str()).map(|row| axis_ends(&row.axis)).map(|(start, end)| if *kind == HandleKind::Start { end } else { start });
                        *current = if *kind == HandleKind::Curve { pointer.at } else { ctx.snapped(pointer, anchor, std::slice::from_ref(wall), &[]).point };
                    }
                    Mode::Vertex { id, current, .. } => *current = ctx.snapped(pointer, None, std::slice::from_ref(id), &[]).point,
                    Mode::Height { value, .. } => *value = pointer.at[1],
                    Mode::Idle => {}
                }
                Step::default()
            }
            ToolEvent::Up(pointer) => self.release(ctx, pointer),
            ToolEvent::Escape | ToolEvent::Lost => {
                self.mode = Mode::Idle;
                Step::default()
            }
            ToolEvent::Double(_) | ToolEvent::Finish => Step::default(),
        }
    }

    fn preview(&self, ctx: &ToolContext<'_>) -> Preview {
        let mut marks = Vec::new();
        match &self.mode {
            Mode::Idle => {}
            Mode::Marquee { start, current, .. } => {
                let rect = in_marquee(*start, *current);
                marks.push(Mark::path(&[[rect[0], rect[1]], [rect[2], rect[1]], [rect[2], rect[3]], [rect[0], rect[3]]], true, Style::Selection));
            }
            Mode::Handle { wall, kind, current } => {
                if let Some(axis) = dragged_axis(ctx.snapshot, wall, *kind, *current) {
                    let (start, end) = axis_ends(&axis);
                    marks.push(Mark::path(&flatten(start, end, super::plane::axis_bulge(&axis), FLATTEN_TOLERANCE), false, Style::Ghost));
                    marks.push(Mark::dot(*current, Style::Handle));
                }
            }
            Mode::Vertex { id, index, current, .. } => {
                let corners = outline_handles(ctx.snapshot, std::slice::from_ref(id)).map(|(_, _, corners)| corners).unwrap_or_default();
                if *index < corners.len() {
                    let mut ring = corners;
                    ring[*index] = *current;
                    marks.push(Mark::path(&ring, true, Style::Ghost));
                    marks.push(Mark::dot(*current, Style::Handle));
                }
            }
            Mode::Sliding { id, from, current, reach } => {
                if let Some((host, offset, _, fits)) = self.slide(ctx, id, from, *current, *reach) {
                    let width = ctx.snapshot.openings.get(id).and_then(|opening| width_of(ctx.snapshot, opening)).unwrap_or_default();
                    marks.push(Mark::path(&cut_outline(&host, offset, width), true, if fits { Style::Ghost } else { Style::Warning }));
                }
            }
            Mode::Height { storey, value } => {
                if let (Some(level), Some(length)) = (ctx.inference.storey_levels.get(storey), section_length(ctx)) {
                    let height = (((value - level.elevation) / HEIGHT_STEP).round() * HEIGHT_STEP).max(HEIGHT_MINIMUM);
                    let top = level.elevation + height;
                    marks.push(Mark::path(&[[-SECTION_REACH, top], [length + SECTION_REACH, top]], false, Style::Ghost));
                    marks.push(length_label([-SECTION_REACH, level.elevation], [-SECTION_REACH, top]));
                }
            }
        }
        Preview::of(marks)
    }
}

fn section_length(ctx: &ToolContext<'_>) -> Option<f64> {
    match &ctx.surface {
        super::session::Surface::Section { start, end } => Some(dist(*start, *end)),
        _ => None,
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
