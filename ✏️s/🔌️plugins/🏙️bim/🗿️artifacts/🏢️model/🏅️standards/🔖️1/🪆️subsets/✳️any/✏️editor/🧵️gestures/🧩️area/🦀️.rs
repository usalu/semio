//! 🧩️ The area tools: slab and roof by polygon (click the corners, close on the first one or finish) or by rectangle (press one corner, release the opposite one), and the slab picked
//! from the walls (click inside a closed wall loop and the loop becomes the boundary). The outline is written counter-clockwise, as the model requires; a roof takes the shape
//! of the active preset (a gable of 30 degrees along the longest edge) and its type from the library selection. The ceiling is drawn like the slab, by polygon or rectangle, and
//! hangs [`DEFAULT_DROP`](crate::editor::bim::entities::ceilings::DEFAULT_DROP) below the storey top; picked from a space, a click inside a room (a space of the storey, else the room the walls
//! around the click close) hangs a ceiling over its outline and islands.

use super::plane::{angle, counter_clockwise, dist, from_point2, loop_of, rectangle, same, signed_area, P};
use super::session::{Mark, Pointer, Preview, Step, Style, Tool, ToolContext, ToolEvent, STOREY_MISSING, TYPE_MISSING};
use crate::editor::bim::entities::ceilings::DEFAULT_DROP;
use crate::standards::v1::subsets::any::schema::inferences::spaces::{SpaceRoom, SpaceStatus};
use crate::{Ceiling, ModelMutation, Point2, Roof, RoofShape, Slab, Space, SpaceBoundary, Vertex};
use semio_framework_geometry::loops;
use semio_framework_geometry::Point;
use std::f64::consts::PI;

/// 🖱️ How many pixels a press must travel before a click becomes a rectangle drag.
const DRAG_PIXELS: f64 = 4.0;
/// 🏠️ The pitch of the gable preset, in radians (30 degrees).
pub const GABLE_PITCH: f64 = PI / 6.0;
/// 🏠️ The overhang of a drawn roof, in metres.
pub const ROOF_OVERHANG: f64 = 0.3;

/// 🧩️ Which area tool this is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Slab,
    SlabFromWalls,
    Ceiling,
    CeilingFromSpace,
    Roof,
}

fn covers(outline: &[Vertex], at: P) -> bool {
    let ring: Vec<loops::Vertex> = outline.iter().map(|vertex| loops::Vertex::new(Point::new(vertex.point.x, vertex.point.y), vertex.bulge)).collect();
    loops::contains(&ring, Point::new(at[0], at[1]))
}

fn encloses(room: &SpaceRoom, at: P) -> bool {
    matches!(room.status, SpaceStatus::Inferred | SpaceStatus::Explicit) && room.outline.len() >= 3 && covers(&room.outline, at) && !room.holes.iter().any(|hole| covers(hole, at))
}

/// 🧩️ The state of one area tool: the corners clicked so far, the pressed corner of a possible rectangle and the snapped pointer.
pub struct Area {
    kind: Kind,
    vertices: Vec<P>,
    pressed: Option<P>,
    hover: Option<super::snap::SnapHit>,
}

impl Area {
    pub fn new(kind: Kind) -> Self {
        Self { kind, vertices: Vec::new(), pressed: None, hover: None }
    }

    fn reset(&mut self) {
        let hover = self.hover.take();
        *self = Self { hover, ..Self::new(self.kind) };
    }

    fn snapped(&self, ctx: &ToolContext<'_>, pointer: &Pointer) -> super::snap::SnapHit {
        let extra: Vec<P> = self.vertices.first().copied().into_iter().collect();
        ctx.snapped(pointer, self.vertices.last().copied(), &[], &extra)
    }

    fn ring_step(&self, ctx: &mut ToolContext<'_>, ring: &[P]) -> Step {
        let Some(storey) = ctx.storey().map(str::to_string) else { return Step::refuse(STOREY_MISSING) };
        let ring = counter_clockwise(ring.to_vec());
        match self.kind {
            Kind::Roof => {
                let Some(roof_type) = ctx.library_type(&ctx.snapshot.roof_types) else { return Step::refuse(TYPE_MISSING) };
                let count = ctx.snapshot.roofs.values().filter(|roof| roof.storey == storey).count();
                let name = ctx.name_of(|labels| labels.kind_roof, count);
                let id = ctx.mint("roof");
                let roof = Roof { storey, roof_type, footprint: loop_of(&ring), shape: RoofShape::Gable { pitch: GABLE_PITCH, ridge_direction: ridge_direction(&ring) }, overhang: ROOF_OVERHANG, base_offset: 0.0, phase: crate::Phase::New, name };
                Step::write(ctx, ModelMutation::CreateRoof(crate::mutations::create_roof::CreateRoof { id, roof }))
            }
            Kind::Ceiling | Kind::CeilingFromSpace => self.ceiling_step(ctx, storey, loop_of(&ring), Vec::new()),
            Kind::Slab | Kind::SlabFromWalls => {
                let Some(slab_type) = ctx.library_type(&ctx.snapshot.slab_types) else { return Step::refuse(TYPE_MISSING) };
                let count = ctx.snapshot.slabs.values().filter(|slab| slab.storey == storey).count();
                let name = ctx.name_of(|labels| labels.kind_slab, count);
                let id = ctx.mint("slab");
                let slab = Slab { storey, slab_type, boundary: loop_of(&ring), holes: Vec::new(), offset: 0.0, slope: None, phase: crate::Phase::New, name };
                Step::write(ctx, ModelMutation::CreateSlab(crate::mutations::create_slab::CreateSlab { id, slab }))
            }
        }
    }

    fn ceiling_step(&self, ctx: &mut ToolContext<'_>, storey: String, boundary: Vec<Vertex>, holes: Vec<Vec<Vertex>>) -> Step {
        let Some(ceiling_type) = ctx.library_type(&ctx.snapshot.ceiling_types) else { return Step::refuse(TYPE_MISSING) };
        let count = ctx.snapshot.ceilings.values().filter(|ceiling| ceiling.storey == storey).count();
        let name = ctx.name_of(|labels| labels.kind_ceiling, count);
        let id = ctx.mint("ceiling");
        let ceiling = Ceiling { storey, ceiling_type, boundary, holes, offset: DEFAULT_DROP, slope: None, name };
        Step::write(ctx, ModelMutation::CreateCeiling(crate::mutations::create_ceiling::CreateCeiling { id, ceiling }))
    }

    fn space_at(&self, ctx: &mut ToolContext<'_>, at: P) -> Step {
        let Some(storey) = ctx.storey().map(str::to_string) else { return Step::refuse(STOREY_MISSING) };
        let listed = ctx.snapshot.spaces.iter().filter(|(_, space)| space.storey == storey).filter_map(|(id, _)| ctx.inference.spaces.get(id)).find(|room| encloses(room, at));
        let room = match listed {
            Some(room) => Some(room.clone()),
            None => {
                if !ctx.inference.storey_levels.contains_key(&storey) {
                    return Step::refuse(STOREY_MISSING);
                }
                let mut probe = ctx.snapshot.clone();
                probe.spaces.insert("probe".into(), Space { storey: storey.clone(), number: String::new(), name: String::new(), boundary: SpaceBoundary::Bounded { seed: Point2 { x: at[0], y: at[1] } }, usage: String::new(), phase: crate::Phase::New, zone: None, floor_finish: None, wall_finish: None, ceiling_finish: None });
                let mut rooms = crate::standards::v1::subsets::any::schema::inferences::model_graph::instance::probe_rooms(ctx.instance, &probe);
                rooms.remove("probe").filter(|room| room.status == SpaceStatus::Inferred)
            }
        };
        match room.filter(|room| room.outline.len() >= 3) {
            Some(room) => self.ceiling_step(ctx, storey, room.outline, room.holes),
            None => Step::refuse(super::session::REJECTED),
        }
    }

    fn commit_polygon(&mut self, ctx: &mut ToolContext<'_>) -> Step {
        if self.vertices.len() < 3 {
            return Step::default();
        }
        let step = self.ring_step(ctx, &self.vertices.clone());
        if step.refused.is_none() {
            self.reset();
        }
        step
    }

    fn room_at(&self, ctx: &mut ToolContext<'_>, at: P) -> Step {
        let Some(storey) = ctx.storey().map(str::to_string) else { return Step::refuse(STOREY_MISSING) };
        if !ctx.inference.storey_levels.contains_key(&storey) {
            return Step::refuse(STOREY_MISSING);
        }
        let mut probe = ctx.snapshot.clone();
        probe.spaces.insert("probe".into(), Space { storey: storey.clone(), number: String::new(), name: String::new(), boundary: SpaceBoundary::Bounded { seed: Point2 { x: at[0], y: at[1] } }, usage: String::new(), phase: crate::Phase::New, zone: None, floor_finish: None, wall_finish: None, ceiling_finish: None });
        let rooms = crate::standards::v1::subsets::any::schema::inferences::model_graph::instance::probe_rooms(ctx.instance, &probe);
        let ring: Vec<P> = rooms.get("probe").filter(|room| room.status == SpaceStatus::Inferred && room.outline.len() >= 3).map(|room| room.outline.iter().map(|vertex| from_point2(vertex.point)).collect()).unwrap_or_default();
        if ring.is_empty() { Step::refuse(super::session::REJECTED) } else { self.ring_step(ctx, &ring) }
    }
}

/// 🏠️ The ridge direction of a footprint: along its longest edge.
pub fn ridge_direction(ring: &[P]) -> f64 {
    let longest = ring.iter().zip(ring.iter().cycle().skip(1)).max_by(|a, b| dist(*a.0, *a.1).total_cmp(&dist(*b.0, *b.1)));
    longest.map_or(0.0, |(from, to)| angle(*from, *to))
}

impl Tool for Area {
    fn anchor(&self) -> Option<P> {
        self.vertices.last().copied()
    }

    fn event(&mut self, ctx: &mut ToolContext<'_>, event: &ToolEvent) -> Step {
        match event {
            ToolEvent::Move(pointer) => {
                self.hover = Some(self.snapped(ctx, pointer));
                Step::default()
            }
            ToolEvent::Down(pointer) => {
                let hit = self.snapped(ctx, pointer);
                let at = hit.point;
                self.hover = Some(hit);
                if self.kind == Kind::SlabFromWalls {
                    return self.room_at(ctx, at);
                }
                if self.kind == Kind::CeilingFromSpace {
                    return self.space_at(ctx, at);
                }
                self.pressed = Some(at);
                Step::default()
            }
            ToolEvent::Up(pointer) => {
                let Some(start) = self.pressed.take() else { return Step::default() };
                let hit = self.snapped(ctx, pointer);
                let at = hit.point;
                self.hover = Some(hit);
                if self.vertices.is_empty() && dist(start, at) > DRAG_PIXELS * pointer.tolerance {
                    let ring = rectangle(start, at);
                    if signed_area(&ring).abs() < 1e-6 {
                        return Step::default();
                    }
                    let step = self.ring_step(ctx, &ring);
                    if step.refused.is_none() {
                        self.reset();
                    }
                    return step;
                }
                if self.vertices.len() >= 3 && self.vertices.first().is_some_and(|first| same(*first, start)) {
                    return self.commit_polygon(ctx);
                }
                if self.vertices.last().is_none_or(|last| !same(*last, start)) {
                    self.vertices.push(start);
                }
                Step::default()
            }
            ToolEvent::Double(_) | ToolEvent::Finish => self.commit_polygon(ctx),
            ToolEvent::Escape | ToolEvent::Lost => {
                self.reset();
                Step::default()
            }
        }
    }

    fn preview(&self, _ctx: &ToolContext<'_>) -> Preview {
        let hover = self.hover.as_ref().map(|hit| hit.point);
        let mut marks = Vec::new();
        match (self.pressed, hover) {
            (Some(start), Some(at)) if self.vertices.is_empty() && start != at => marks.push(Mark::path(&rectangle(start, at), true, Style::Ghost)),
            _ if !self.vertices.is_empty() => {
                let mut path = self.vertices.clone();
                path.extend(hover);
                marks.push(Mark::path(&path, false, Style::Ghost));
                if self.vertices.len() >= 2 {
                    marks.push(Mark::path(&[*self.vertices.last().unwrap_or(&[0.0, 0.0]), self.vertices[0]], false, Style::Guide));
                }
            }
            _ => {}
        }
        marks.extend(self.hover.as_ref().map(Mark::snap));
        Preview::of(marks)
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
