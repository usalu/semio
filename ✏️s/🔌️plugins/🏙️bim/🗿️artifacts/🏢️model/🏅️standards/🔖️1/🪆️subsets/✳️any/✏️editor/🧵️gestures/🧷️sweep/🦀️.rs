//! 🧷️ The wall sweep tool. The pointer hovers a wall of the storey; the sweep runs along the face of the wall the pointer is on (the left face looking along the axis when the pointer is left of it, else the right one), standing on the
//! wall base. Its profile is the section of the selected beam or column type of the library, else a baseboard; its material is the selected material, else the first of the project. The ghost is the band the sweep covers along the face; it turns to
//! a warning when the model would refuse it. One click writes the sweep.

use super::plane::{axis_length, axis_point_at, axis_tangent_at, project, P};
use super::session::{Mark, Pointer, Preview, Step, Style, Tool, ToolContext, ToolEvent, PICK_PIXELS, REJECTED, TYPE_MISSING};
use crate::editor::bim::entities::wall_sweeps::baseboard;
use crate::mutations::create_wall_sweep::CreateWallSweep;
use crate::standards::v1::subsets::any::schema::inferences::element_solids::wall_sweeps::extents_of;
use crate::standards::v1::subsets::any::schema::inferences::wall_layout::thickness_of;
use crate::{ModelMutation, ModelSnapshot, Profile, Wall, WallSide, WallSweep};

/// 📏️ The metres between two samples of the ghost along an arc wall.
const STEP: f64 = 0.25;
/// 🔢️ The most samples the ghost takes along a wall.
const MAX_STEPS: usize = 64;

/// 🔎️ The wall of `storey` closest to `at`, within half its thickness plus `reach` metres, with the side of the pointer (positive left of travel).
pub fn nearest_wall<'a>(snapshot: &'a ModelSnapshot, storey: &str, at: P, reach: f64) -> Option<(&'a String, &'a Wall, f64)> {
    snapshot
        .walls
        .iter()
        .filter(|(_, wall)| wall.storey == storey)
        .filter_map(|(id, wall)| {
            let found = project(&wall.axis, at);
            (found.distance <= thickness_of(snapshot, wall) / 2.0 + reach).then_some((found.distance, id, wall, found.side))
        })
        .min_by(|a, b| a.0.total_cmp(&b.0))
        .map(|(_, id, wall, side)| (id, wall, side))
}

/// ▭️ The profile of a new sweep: the section of the selected beam or column type, else a baseboard.
fn profile_of(ctx: &ToolContext<'_>) -> Profile {
    ctx.library
        .iter()
        .find_map(|id| ctx.snapshot.beam_types.get(id).map(|row| row.profile.clone()).or_else(|| ctx.snapshot.column_types.get(id).map(|row| row.profile.clone())))
        .unwrap_or_else(baseboard)
}

/// 📐️ The samples of the line `offset` metres left of the axis of `wall` (negative: right of it).
fn face_line(wall: &Wall, offset: f64) -> Vec<P> {
    let length = axis_length(&wall.axis);
    let steps = ((length / STEP).ceil() as usize).clamp(1, MAX_STEPS);
    (0..=steps)
        .map(|k| {
            let s = length * k as f64 / steps as f64;
            let (point, tangent) = (axis_point_at(&wall.axis, s), axis_tangent_at(&wall.axis, s));
            [point[0] - tangent[1] * offset, point[1] + tangent[0] * offset]
        })
        .collect()
}

/// 🧷️ A candidate sweep under the pointer: the record that would be written, the distance of its face from the axis and whether the model accepts it.
struct Candidate {
    sweep: WallSweep,
    face: f64,
    valid: bool,
}

/// 🧷️ The state of the sweep tool: the candidate under the pointer.
#[derive(Default)]
pub struct Sweeping {
    hover: Option<Candidate>,
}

impl Sweeping {
    fn candidate(&self, ctx: &ToolContext<'_>, pointer: &Pointer) -> Option<Candidate> {
        let storey = ctx.storey()?;
        let (host, wall, side) = nearest_wall(ctx.snapshot, storey, pointer.at, pointer.tolerance * PICK_PIXELS)?;
        let side = if side < 0.0 { WallSide::Right } else { WallSide::Left };
        let material = ctx.library_type(&ctx.snapshot.materials);
        let sweep = WallSweep { host: host.clone(), side, profile: profile_of(ctx), height: 0.0, inset: 0.0, material: material.clone().unwrap_or_default(), name: ctx.name_of(|labels| labels.kind_wall_sweep, ctx.snapshot.wall_sweeps.len()) };
        let layout = ctx.inference.wall_layout.get(host);
        let half = thickness_of(ctx.snapshot, wall) / 2.0;
        let face = match side {
            WallSide::Left => layout.map_or(half, |layout| layout.offset_left),
            WallSide::Right => -layout.map_or(half, |layout| layout.offset_right),
        };
        let valid = material.is_some() && ctx.accepts(&ModelMutation::CreateWallSweep(CreateWallSweep { id: "probe".into(), wall_sweep: sweep.clone() }));
        Some(Candidate { sweep, face, valid })
    }
}

impl Tool for Sweeping {
    fn event(&mut self, ctx: &mut ToolContext<'_>, event: &ToolEvent) -> Step {
        match event {
            ToolEvent::Move(pointer) => {
                self.hover = self.candidate(ctx, pointer);
                Step::default()
            }
            ToolEvent::Down(pointer) => {
                if ctx.library_type(&ctx.snapshot.materials).is_none() {
                    return Step::refuse(TYPE_MISSING);
                }
                self.hover = self.candidate(ctx, pointer);
                match self.hover.as_ref() {
                    None => Step::default(),
                    Some(Candidate { valid: true, sweep, .. }) => {
                        let wall_sweep = sweep.clone();
                        let id = ctx.mint("sweep");
                        Step::write(ctx, ModelMutation::CreateWallSweep(CreateWallSweep { id, wall_sweep }))
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

    fn preview(&self, ctx: &ToolContext<'_>) -> Preview {
        let Some(candidate) = self.hover.as_ref() else { return Preview::default() };
        let Some(wall) = ctx.snapshot.walls.get(&candidate.sweep.host) else { return Preview::default() };
        let style = if candidate.valid { Style::Ghost } else { Style::Warning };
        let outward = if candidate.face < 0.0 { -1.0 } else { 1.0 };
        let shown = (extents_of(&candidate.sweep).0 - candidate.sweep.inset).max(0.0) * outward;
        let mut ring = face_line(wall, candidate.face);
        ring.extend(face_line(wall, candidate.face + shown).into_iter().rev());
        Preview::of(vec![Mark::path(&ring, true, style)])
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
