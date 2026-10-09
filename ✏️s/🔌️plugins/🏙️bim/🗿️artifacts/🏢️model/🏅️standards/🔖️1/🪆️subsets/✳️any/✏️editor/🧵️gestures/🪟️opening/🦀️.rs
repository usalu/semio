//! 🪟️ The opening tools: window, door and void. The pointer hovers a wall (or curtain wall) of the storey; the opening stands centred on the pointer's foot on the host axis, kept
//! inside the host, with the facing taken from the side of the wall the pointer is on. The ghost is the opening's cut across the wall; it turns to a warning when the model would
//! refuse the placement (it leaves the host or overlaps a neighbour). One click writes the opening. The same host search places an existing opening that is dragged along or onto
//! another wall.

use super::plane::{axis_length, axis_point_at, axis_tangent_at, project, P};
use super::session::{Mark, Pointer, Preview, Step, Style, Tool, ToolContext, ToolEvent, REJECTED, TYPE_MISSING};
use crate::standards::v1::subsets::any::schema::inferences::wall_layout::thickness_of;
use crate::{Axis, ModelMutation, ModelSnapshot, Opening, OpeningKind};

/// 🪟️ The size of a void opening, in metres.
pub const VOID_WIDTH: f64 = 1.0;
pub const VOID_HEIGHT: f64 = 2.1;
/// 📏️ The thickness a curtain wall counts as when looking for a host, in metres.
const CURTAIN_DEPTH: f64 = 0.1;

/// 🪟️ Which opening this tool places.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Window,
    Door,
    Void,
}

/// 🧱️ A host under the pointer: its id, the axis arc length of the pointer's foot, the side of the host the pointer is on (positive left of travel), its length and thickness.
#[derive(Clone, Debug, PartialEq)]
pub struct HostHit {
    pub host: String,
    pub offset: f64,
    pub side: f64,
    pub length: f64,
    pub thickness: f64,
    pub axis: Axis,
}

/// 🔎️ The wall or curtain wall of `storey` closest to `at`, within half its thickness plus `reach` metres; `except` never counts.
pub fn nearest_host(snapshot: &ModelSnapshot, storey: &str, at: P, reach: f64) -> Option<HostHit> {
    let walls = snapshot.walls.iter().filter(|(_, wall)| wall.storey == storey).map(|(id, wall)| (id, &wall.axis, thickness_of(snapshot, wall)));
    let curtains = snapshot.curtain_walls.iter().filter(|(_, wall)| wall.storey == storey).map(|(id, wall)| (id, &wall.axis, CURTAIN_DEPTH));
    walls
        .chain(curtains)
        .filter_map(|(id, axis, thickness)| {
            let found = project(axis, at);
            (found.distance <= thickness / 2.0 + reach).then(|| (found.distance, HostHit { host: id.clone(), offset: found.offset, side: found.side, length: axis_length(axis), thickness, axis: axis.clone() }))
        })
        .min_by(|a, b| a.0.total_cmp(&b.0))
        .map(|(_, hit)| hit)
}

/// 📐️ The centre offset along a host that keeps an opening of `width` inside it, nearest to `wanted`; none when the host is too short.
pub fn fitted_offset(length: f64, width: f64, wanted: f64) -> Option<f64> {
    (length >= width).then(|| wanted.clamp(width / 2.0, length - width / 2.0))
}

/// 🔷️ The cut of an opening across its host as a closed outline: `width` along the axis, a little deeper than the host.
pub fn cut_outline(host: &HostHit, offset: f64, width: f64) -> Vec<P> {
    let centre = axis_point_at(&host.axis, offset);
    let tangent = axis_tangent_at(&host.axis, offset);
    let normal = [-tangent[1], tangent[0]];
    let (half_width, half_depth) = (width / 2.0, host.thickness / 2.0 + 0.03);
    [(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)].map(|(along, across)| [centre[0] + tangent[0] * half_width * along + normal[0] * half_depth * across, centre[1] + tangent[1] * half_width * along + normal[1] * half_depth * across]).to_vec()
}

/// 🪟️ A candidate placement of the tool's opening: the host, the fitted offset, the facing, the opening that would be written and whether the model accepts it.
struct Candidate {
    host: HostHit,
    offset: f64,
    width: f64,
    opening: Option<Opening>,
    valid: bool,
}

/// 🪟️ The state of one opening tool: the candidate under the pointer.
pub struct Placing {
    kind: Kind,
    hover: Option<Candidate>,
}

impl Placing {
    pub fn new(kind: Kind) -> Self {
        Self { kind, hover: None }
    }

    fn opening_kind(&self, ctx: &ToolContext<'_>) -> Option<(OpeningKind, f64)> {
        match self.kind {
            Kind::Window => ctx.library_type(&ctx.snapshot.window_types).and_then(|id| ctx.snapshot.window_types.get(&id).map(|row| (OpeningKind::Window { window_type: id.clone() }, row.width))),
            Kind::Door => ctx.library_type(&ctx.snapshot.door_types).and_then(|id| ctx.snapshot.door_types.get(&id).map(|row| (OpeningKind::Door { door_type: id.clone() }, row.width))),
            Kind::Void => Some((OpeningKind::Void { width: VOID_WIDTH, height: VOID_HEIGHT }, VOID_WIDTH)),
        }
    }

    fn candidate(&self, ctx: &ToolContext<'_>, pointer: &Pointer) -> Option<Candidate> {
        let storey = ctx.storey()?;
        let host = nearest_host(ctx.snapshot, storey, pointer.at, pointer.tolerance * super::session::PICK_PIXELS)?;
        let Some((kind, width)) = self.opening_kind(ctx) else { return Some(Candidate { offset: host.offset, host, width: 0.0, opening: None, valid: false }) };
        let Some(offset) = fitted_offset(host.length, width, host.offset) else { return Some(Candidate { offset: host.offset, host, width, opening: None, valid: false }) };
        let name = ctx.name_of(|labels| labels.kind_opening, ctx.snapshot.openings.len());
        let opening = Opening { host: host.host.clone(), kind, offset, sill_override: None, width: None, height: None, flip_hand: false, flip_facing: host.side < 0.0, reveal_depth: None, reveal_material: None, name };
        let valid = ctx.accepts(&ModelMutation::CreateOpening(crate::mutations::create_opening::CreateOpening { id: "probe".into(), opening: opening.clone() }));
        Some(Candidate { host, offset, width, opening: Some(opening), valid })
    }
}

impl Tool for Placing {
    fn event(&mut self, ctx: &mut ToolContext<'_>, event: &ToolEvent) -> Step {
        match event {
            ToolEvent::Move(pointer) => {
                self.hover = self.candidate(ctx, pointer);
                Step::default()
            }
            ToolEvent::Down(pointer) => {
                if self.opening_kind(ctx).is_none() {
                    return Step::refuse(TYPE_MISSING);
                }
                self.hover = self.candidate(ctx, pointer);
                match self.hover.as_ref() {
                    None => Step::default(),
                    Some(Candidate { valid: true, opening: Some(opening), .. }) => {
                        let opening = opening.clone();
                        let id = ctx.mint("opening");
                        Step::write(ctx, ModelMutation::CreateOpening(crate::mutations::create_opening::CreateOpening { id, opening }))
                    }
                    Some(_) => Step::refuse(REJECTED),
                }
            }
            ToolEvent::Escape | ToolEvent::Lost => {
                self.hover = None;
                Step::default()
            }
            ToolEvent::Up(_) | ToolEvent::Double(_) | ToolEvent::Finish => Step::default(),
        }
    }

    fn preview(&self, _ctx: &ToolContext<'_>) -> Preview {
        let Some(candidate) = self.hover.as_ref() else { return Preview::default() };
        let style = if candidate.valid { Style::Ghost } else { Style::Warning };
        let ring = if candidate.width > 0.0 { cut_outline(&candidate.host, candidate.offset, candidate.width) } else { Vec::new() };
        Preview::of(if ring.is_empty() { Vec::new() } else { vec![Mark::path(&ring, true, style)] })
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
