//! 🧩️ The area tools: slab and roof by polygon (click the corners, close on the first one or finish) or by rectangle (press one corner, release the opposite one), and the slab picked
//! from the walls (click inside a closed wall loop and the loop becomes the boundary). The outline is written counter-clockwise, as the model requires; a roof takes the shape
//! of the active preset (a gable of 30 degrees along the longest edge) and its type from the library selection.

use super::plane::{angle, counter_clockwise, dist, from_point2, loop_of, rectangle, same, signed_area, P};
use super::session::{Mark, Pointer, Preview, Step, Style, Tool, ToolContext, ToolEvent, STOREY_MISSING, TYPE_MISSING};
use crate::standards::v1::subsets::any::schema::inferences::spaces::{rooms_of, SpaceStatus};
use crate::{ModelMutation, Point2, Roof, RoofShape, Slab, Space, SpaceBoundary};
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
    Roof,
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
                let name = ctx.name_of(|labels| labels.kind_roof, "Roof", count);
                let id = ctx.mint("roof");
                let roof = Roof { storey, roof_type, footprint: loop_of(&ring), shape: RoofShape::Gable { pitch: GABLE_PITCH, ridge_direction: ridge_direction(&ring) }, overhang: ROOF_OVERHANG, base_offset: 0.0, name };
                Step::write(ctx, ModelMutation::CreateRoof(crate::mutations::create_roof::CreateRoof { id, roof }))
            }
            Kind::Slab | Kind::SlabFromWalls => {
                let Some(slab_type) = ctx.library_type(&ctx.snapshot.slab_types) else { return Step::refuse(TYPE_MISSING) };
                let count = ctx.snapshot.slabs.values().filter(|slab| slab.storey == storey).count();
                let name = ctx.name_of(|labels| labels.kind_slab, "Slab", count);
                let id = ctx.mint("slab");
                let slab = Slab { storey, slab_type, boundary: loop_of(&ring), holes: Vec::new(), offset: 0.0, slope: None, name };
                Step::write(ctx, ModelMutation::CreateSlab(crate::mutations::create_slab::CreateSlab { id, slab }))
            }
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
        let Some(level) = ctx.inference.storey_levels.get(&storey).copied() else { return Step::refuse(STOREY_MISSING) };
        let mut probe = ctx.snapshot.clone();
        probe.spaces.insert("probe".into(), Space { storey: storey.clone(), number: String::new(), name: String::new(), boundary: SpaceBoundary::Bounded { seed: Point2 { x: at[0], y: at[1] } }, usage: String::new() });
        let rooms = rooms_of(&probe, &storey, &level);
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
