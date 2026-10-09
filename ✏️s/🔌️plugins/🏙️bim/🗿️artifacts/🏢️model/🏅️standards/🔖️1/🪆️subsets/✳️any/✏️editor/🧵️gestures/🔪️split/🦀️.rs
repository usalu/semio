//! ✂️ The split tool: the pointer hovers a wall of the storey, the cut across it shows where the wall would part, and one click writes the `split-wall` that parts it there, the new
//! half getting an id minted from the authoring seed. A cut at an end of the wall (or one the model would refuse) shows as a warning and writes nothing. Curtain walls do not split.

use super::opening::nearest_host;
use super::plane::{axis_point_at, axis_tangent_at, P};
use super::session::{Mark, Pointer, Preview, Step, Style, Tool, ToolContext, ToolEvent, PICK_PIXELS, REJECTED};
use crate::ModelMutation;

/// 📏️ How far past the face of the wall the cut mark reaches, in metres.
const CUT_REACH: f64 = 0.15;

/// ✂️ A cut under the pointer: the wall, the fraction of its axis, the two ends of the mark across it and whether the model accepts it.
struct Cut {
    wall: String,
    t: f64,
    mark: [P; 2],
    valid: bool,
}

/// ✂️ The state of the split tool: the cut under the pointer.
#[derive(Default)]
pub struct Split {
    hover: Option<Cut>,
}

impl Split {
    fn cut(ctx: &ToolContext<'_>, pointer: &Pointer) -> Option<Cut> {
        let host = nearest_host(ctx.snapshot, ctx.storey()?, pointer.at, pointer.tolerance * PICK_PIXELS).filter(|host| ctx.snapshot.walls.contains_key(&host.host))?;
        let t = if host.length > 0.0 { host.offset / host.length } else { 0.0 };
        let (at, tangent) = (axis_point_at(&host.axis, host.offset), axis_tangent_at(&host.axis, host.offset));
        let reach = host.thickness / 2.0 + CUT_REACH;
        let mark = [[at[0] - tangent[1] * reach, at[1] + tangent[0] * reach], [at[0] + tangent[1] * reach, at[1] - tangent[0] * reach]];
        let probe = ModelMutation::SplitWall(crate::mutations::split_wall::SplitWall { id: host.host.clone(), t, new_id: "probe".into() });
        Some(Cut { valid: ctx.accepts(&probe), wall: host.host, t, mark })
    }
}

impl Tool for Split {
    fn event(&mut self, ctx: &mut ToolContext<'_>, event: &ToolEvent) -> Step {
        match event {
            ToolEvent::Move(pointer) => {
                self.hover = Self::cut(ctx, pointer);
                Step::default()
            }
            ToolEvent::Down(pointer) => {
                self.hover = Self::cut(ctx, pointer);
                match self.hover.as_ref() {
                    None => Step::default(),
                    Some(Cut { valid: false, .. }) => Step::refuse(REJECTED),
                    Some(cut) => {
                        let (id, t) = (cut.wall.clone(), cut.t);
                        let new_id = ctx.mint("wall");
                        Step::write(ctx, ModelMutation::SplitWall(crate::mutations::split_wall::SplitWall { id, t, new_id }))
                    }
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
        let Some(cut) = self.hover.as_ref() else { return Preview::default() };
        Preview::of(vec![Mark::path(&cut.mark, false, if cut.valid { Style::Ghost } else { Style::Warning })])
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
