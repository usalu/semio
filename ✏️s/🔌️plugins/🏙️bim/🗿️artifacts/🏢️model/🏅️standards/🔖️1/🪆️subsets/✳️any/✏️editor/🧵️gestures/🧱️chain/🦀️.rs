//! 🧱️ The point-chain tools: wall (straight and arc), curtain wall, beam (straight and arc), railing, grid line and the measure. A click sets a point; the rubber band follows the pointer to the
//! next snapped point; a wall segment, a curtain wall segment, a beam or a grid line is written the moment its last point is clicked, so each is one history row, and a chain
//! of walls goes on from the end of the last one until it closes on its first point, is finished or escapes. A railing and a ramp are written whole when the chain finishes. The measure
//! writes nothing, ever.

use super::plane::{axis_of, bulge_through, dist, flatten, point2, same, P};
use super::session::{length_label, Mark, Pointer, Preview, Step, Style, Tool, ToolContext, ToolEvent, REJECTED, STOREY_MISSING, TYPE_MISSING};
use crate::{Beam, CurtainWall, GridLine, LocationLine, Phase, Railing, Ramp, TopConstraint, Vertex, Wall};
use crate::ModelMutation;

/// 📏️ The shortest segment a chain writes, in metres.
const MIN_LENGTH: f64 = 1e-3;
/// 📏️ The chord tolerance an arc preview is flattened to, in metres.
const FLATTEN_TOLERANCE: f64 = 0.005;
/// 🛤️ The default height of a railing and the spacing of its posts, in metres.
const RAILING_HEIGHT: f64 = 1.0;
const RAILING_POSTS: f64 = 1.2;
/// 🛝️ The rise of a drawn ramp until its top constraint is edited, in metres.
const RAMP_RISE: f64 = 0.5;

/// 🧱️ Which chain tool this is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Wall,
    WallArc,
    CurtainWall,
    Beam,
    BeamArc,
    Railing,
    Ramp,
    Grid,
    Measure,
}

/// 🧱️ The state of one chain: where it started, where the rubber band is anchored, the end of an arc waiting for its bulge, the vertices of a railing and the last measure.
pub struct Chain {
    kind: Kind,
    first: Option<P>,
    anchor: Option<P>,
    pending: Option<P>,
    vertices: Vec<P>,
    segments: usize,
    measured: Option<(P, P)>,
    hover: Option<super::snap::SnapHit>,
}

impl Chain {
    pub fn new(kind: Kind) -> Self {
        Self { kind, first: None, anchor: None, pending: None, vertices: Vec::new(), segments: 0, measured: None, hover: None }
    }

    fn reset(&mut self) {
        let hover = self.hover.take();
        *self = Self { hover, ..Self::new(self.kind) };
    }

    fn extra(&self) -> Vec<P> {
        self.first.filter(|_| self.segments >= 1 || !self.vertices.is_empty()).into_iter().collect()
    }

    fn wall(&self, ctx: &mut ToolContext<'_>, start: P, end: P, bulge: f64) -> Step {
        let Some(storey) = ctx.storey().map(str::to_string) else { return Step::refuse(STOREY_MISSING) };
        let Some(wall_type) = ctx.library_type(&ctx.snapshot.wall_types) else { return Step::refuse(TYPE_MISSING) };
        let count = ctx.snapshot.walls.values().filter(|wall| wall.storey == storey).count();
        let name = ctx.name_of(|labels| labels.kind_wall, count);
        let id = ctx.mint("wall");
        let wall = Wall { storey, wall_type, axis: axis_of(start, end, bulge), location: LocationLine::Center, base_offset: 0.0, top: TopConstraint::StoreyTop { offset: 0.0 }, phase: Phase::New, start_join: None, end_join: None, base_slab: None, name };
        Step::write(ctx, ModelMutation::CreateWall(crate::mutations::create_wall::CreateWall { id, wall }))
    }

    fn curtain(&self, ctx: &mut ToolContext<'_>, start: P, end: P) -> Step {
        let Some(storey) = ctx.storey().map(str::to_string) else { return Step::refuse(STOREY_MISSING) };
        let Some(curtain_wall_type) = ctx.library_type(&ctx.snapshot.curtain_wall_types) else { return Step::refuse(TYPE_MISSING) };
        let count = ctx.snapshot.curtain_walls.values().filter(|wall| wall.storey == storey).count();
        let name = ctx.name_of(|labels| labels.kind_curtain_wall, count);
        let id = ctx.mint("curtain-wall");
        let curtain_wall = CurtainWall { storey, curtain_wall_type, axis: axis_of(start, end, 0.0), base_offset: 0.0, top: TopConstraint::StoreyTop { offset: 0.0 }, u_grid: None, v_grid: None, phase: Phase::New, name };
        Step::write(ctx, ModelMutation::CreateCurtainWall(crate::mutations::create_curtain_wall::CreateCurtainWall { id, curtain_wall }))
    }

    fn beam(&self, ctx: &mut ToolContext<'_>, start: P, end: P, bulge: f64) -> Step {
        let Some(storey) = ctx.storey().map(str::to_string) else { return Step::refuse(STOREY_MISSING) };
        let Some(beam_type) = ctx.library_type(&ctx.snapshot.beam_types) else { return Step::refuse(TYPE_MISSING) };
        let count = ctx.snapshot.beams.values().filter(|beam| beam.storey == storey).count();
        let name = ctx.name_of(|labels| labels.kind_beam, count);
        let id = ctx.mint("beam");
        let beam = Beam { storey, beam_type, axis: axis_of(start, end, bulge), top_offset: 0.0, end_top_offset: None, phase: Phase::New, name };
        Step::write(ctx, ModelMutation::CreateBeam(crate::mutations::create_beam::CreateBeam { id, beam }))
    }

    fn grid(&self, ctx: &mut ToolContext<'_>, start: P, end: P) -> Step {
        let Some(building) = ctx.building().map(str::to_string) else { return Step::refuse(STOREY_MISSING) };
        let vertical = (end[1] - start[1]).abs() > (end[0] - start[0]).abs();
        let label = next_grid_label(ctx, &building, vertical);
        let id = ctx.mint("grid");
        let grid_line = GridLine { building, label, start: point2(start), end: point2(end) };
        Step::write(ctx, ModelMutation::CreateGridLine(crate::mutations::create_grid_line::CreateGridLine { id, grid_line }))
    }

    fn railing(&self, ctx: &mut ToolContext<'_>) -> Step {
        let Some(storey) = ctx.storey().map(str::to_string) else { return Step::refuse(STOREY_MISSING) };
        let snapshot = ctx.snapshot;
        let Some(material) = snapshot.materials.iter().find(|(_, material)| material.category == crate::MaterialCategory::Metal).or_else(|| snapshot.materials.iter().next()).map(|(id, _)| id.clone()) else { return Step::refuse(TYPE_MISSING) };
        let count = snapshot.railings.values().filter(|railing| railing.storey == storey).count();
        let name = ctx.name_of(|labels| labels.kind_railing, count);
        let id = ctx.mint("railing");
        let railing = Railing { storey, path: self.vertices.iter().map(|p| point2(*p)).collect(), height: RAILING_HEIGHT, post_spacing: RAILING_POSTS, profile: crate::standard_rail_profile(), post_profile: crate::standard_post_profile(), baluster: None, infill: crate::STANDARD_INFILL, material, base_offset: 0.0, host: None, phase: crate::Phase::New, name };
        Step::write(ctx, ModelMutation::CreateRailing(crate::mutations::create_railing::CreateRailing { id, railing }))
    }

    fn ramp(&self, ctx: &mut ToolContext<'_>) -> Step {
        let Some(storey) = ctx.storey().map(str::to_string) else { return Step::refuse(STOREY_MISSING) };
        let snapshot = ctx.snapshot;
        let Some(material) = snapshot.materials.iter().find(|(_, material)| material.category == crate::MaterialCategory::Concrete).or_else(|| snapshot.materials.iter().next()).map(|(id, _)| id.clone()) else { return Step::refuse(TYPE_MISSING) };
        let count = snapshot.ramps.values().filter(|ramp| ramp.storey == storey).count();
        let name = ctx.name_of(|labels| labels.kind_ramp, count);
        let id = ctx.mint("ramp");
        let path = self.vertices.iter().map(|p| Vertex { point: point2(*p), bulge: 0.0 }).collect();
        let ramp = Ramp {
            storey,
            path,
            width: crate::STANDARD_RAMP_WIDTH,
            landing_start: crate::STANDARD_RAMP_LANDING,
            landing_end: crate::STANDARD_RAMP_LANDING,
            landing_turn: crate::STANDARD_RAMP_LANDING,
            max_slope: crate::STANDARD_RAMP_MAX_SLOPE,
            thickness: crate::STANDARD_RAMP_THICKNESS,
            material,
            base_offset: 0.0,
            top: TopConstraint::Unconnected { height: RAMP_RISE },
            railing_left: false,
            railing_right: false,
            name,
        };
        Step::write(ctx, ModelMutation::CreateRamp(crate::mutations::create_ramp::CreateRamp { id, ramp }))
    }

    fn down(&mut self, ctx: &mut ToolContext<'_>, pointer: &Pointer) -> Step {
        let extra = self.extra();
        let hit = ctx.snapped(pointer, self.anchor.or(self.vertices.last().copied()), &[], &extra);
        let at = hit.point;
        self.hover = Some(hit);
        match self.kind {
            Kind::Railing | Kind::Ramp => {
                if self.vertices.last().is_none_or(|last| !same(*last, at)) {
                    self.vertices.push(at);
                }
                Step::default()
            }
            Kind::Measure => {
                match (self.anchor, self.measured) {
                    (Some(start), _) if !same(start, at) => {
                        self.measured = Some((start, at));
                        self.anchor = None;
                    }
                    (None, _) => {
                        self.measured = None;
                        self.anchor = Some(at);
                    }
                    _ => {}
                }
                Step::default()
            }
            Kind::Wall | Kind::CurtainWall | Kind::Beam | Kind::Grid | Kind::WallArc | Kind::BeamArc => self.advance(ctx, at),
        }
    }

    fn advance(&mut self, ctx: &mut ToolContext<'_>, at: P) -> Step {
        let Some(start) = self.anchor else {
            self.first = Some(at);
            self.anchor = Some(at);
            return Step::default();
        };
        if matches!(self.kind, Kind::WallArc | Kind::BeamArc) {
            let Some(end) = self.pending else {
                if dist(start, at) >= MIN_LENGTH {
                    self.pending = Some(at);
                }
                return Step::default();
            };
            let Some(bulge) = bulge_through(start, at, end) else { return Step::default() };
            let step = if self.kind == Kind::BeamArc { self.beam(ctx, start, end, bulge) } else { self.wall(ctx, start, end, bulge) };
            self.after(&step, end);
            return step;
        }
        if dist(start, at) < MIN_LENGTH {
            return Step::default();
        }
        let step = match self.kind {
            Kind::Wall => self.wall(ctx, start, at, 0.0),
            Kind::CurtainWall => self.curtain(ctx, start, at),
            Kind::Beam => self.beam(ctx, start, at, 0.0),
            _ => self.grid(ctx, start, at),
        };
        self.after(&step, at);
        step
    }

    fn after(&mut self, step: &Step, end: P) {
        if step.refused.is_some() {
            return;
        }
        self.segments += 1;
        self.pending = None;
        let closed = self.first.is_some_and(|first| same(first, end)) && self.segments >= 2;
        if closed || !matches!(self.kind, Kind::Wall | Kind::WallArc | Kind::CurtainWall) {
            self.reset();
        } else {
            self.anchor = Some(end);
        }
    }

    fn finish(&mut self, ctx: &mut ToolContext<'_>) -> Step {
        let step = match self.kind {
            Kind::Railing if self.vertices.len() >= 2 => self.railing(ctx),
            Kind::Ramp if self.vertices.len() >= 2 => self.ramp(ctx),
            _ => Step::default(),
        };
        if step.refused != Some(REJECTED) {
            self.reset();
        }
        step
    }

    fn arc_marks(&self, start: P, end: P, through: Option<P>) -> Vec<P> {
        let bulge = through.and_then(|through| bulge_through(start, through, end)).unwrap_or(0.0);
        flatten(start, end, bulge, FLATTEN_TOLERANCE)
    }
}

fn next_grid_label(ctx: &ToolContext<'_>, building: &str, numbers: bool) -> String {
    let taken: Vec<&str> = ctx.snapshot.grids.values().filter(|grid| grid.building == building).map(|grid| grid.label.as_str()).collect();
    (0..).map(|ordinal| if numbers { (ordinal + 1).to_string() } else { letters(ordinal) }).find(|label| !taken.contains(&label.as_str())).unwrap_or_default()
}

fn letters(ordinal: usize) -> String {
    let (head, tail) = (ordinal / 26, ordinal % 26);
    let letter = char::from(b'A' + tail as u8);
    if head == 0 { letter.to_string() } else { format!("{}{letter}", letters(head - 1)) }
}

impl Tool for Chain {
    fn anchor(&self) -> Option<P> {
        self.anchor.or(self.vertices.last().copied())
    }

    fn event(&mut self, ctx: &mut ToolContext<'_>, event: &ToolEvent) -> Step {
        match event {
            ToolEvent::Move(pointer) => {
                let extra = self.extra();
                self.hover = Some(ctx.snapped(pointer, self.anchor.or(self.vertices.last().copied()), &[], &extra));
                Step::default()
            }
            ToolEvent::Down(pointer) => self.down(ctx, pointer),
            ToolEvent::Double(_) | ToolEvent::Finish => self.finish(ctx),
            ToolEvent::Escape | ToolEvent::Lost => {
                self.reset();
                Step::default()
            }
            ToolEvent::Up(_) => Step::default(),
        }
    }

    fn preview(&self, _ctx: &ToolContext<'_>) -> Preview {
        let mut marks = Vec::new();
        let hover = self.hover.as_ref().map(|hit| hit.point);
        if let Some((start, end)) = self.measured {
            marks.push(Mark::path(&[start, end], false, Style::Guide));
            marks.push(length_label(start, end));
        }
        match (self.kind, self.anchor, self.pending, hover) {
            (Kind::WallArc | Kind::BeamArc, Some(start), Some(end), _) => {
                marks.push(Mark::path(&self.arc_marks(start, end, hover), false, Style::Ghost));
                marks.push(Mark::path(&[start, end], false, Style::Guide));
            }
            (_, Some(start), _, Some(end)) => {
                marks.push(Mark::path(&[start, end], false, Style::Ghost));
                marks.push(length_label(start, end));
            }
            _ => {}
        }
        if matches!(self.kind, Kind::Railing | Kind::Ramp) && !self.vertices.is_empty() {
            let mut path = self.vertices.clone();
            path.extend(hover);
            marks.push(Mark::path(&path, false, Style::Ghost));
        }
        marks.extend(self.hover.as_ref().map(Mark::snap));
        Preview::of(marks)
    }
}


#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
