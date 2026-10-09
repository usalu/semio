//! 📍️ The point tools: the column (one click, a column of the selected library type standing on the storey), the space (one click inside a room: the click is the seed of a
//! bounded space and the number is the next free one of the storey) and the stair (press the foot, drag the direction and release, or click the foot and click the direction).

use super::plane::{angle, dist, point2, same, P};
use super::session::{length_label, Mark, Pointer, Preview, Step, Style, Tool, ToolContext, ToolEvent, STOREY_MISSING, TYPE_MISSING};
use crate::{Column, ModelMutation, Space, SpaceBoundary, Stair, StairFlight, TopConstraint};

/// 🖱️ How many pixels a press must travel before it counts as a drag.
const DRAG_PIXELS: f64 = 4.0;
/// 🪜️ The clear width of a drawn stair, its largest riser and its smallest tread, in metres.
pub const STAIR_WIDTH: f64 = 1.0;
pub const STAIR_MAX_RISER: f64 = 0.18;
pub const STAIR_MIN_TREAD: f64 = 0.27;

/// 📍️ Which point tool this is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Column,
    Space,
    Stair,
}

/// 📍️ The state of one point tool: the snapped pointer and, for a stair, the foot already chosen and whether the press is still down.
pub struct Point {
    kind: Kind,
    hover: Option<super::snap::SnapHit>,
    foot: Option<P>,
    held: bool,
}

impl Point {
    pub fn new(kind: Kind) -> Self {
        Self { kind, hover: None, foot: None, held: false }
    }

    fn column(&self, ctx: &mut ToolContext<'_>, at: P) -> Step {
        let Some(storey) = ctx.storey().map(str::to_string) else { return Step::refuse(STOREY_MISSING) };
        let Some(column_type) = ctx.library_type(&ctx.snapshot.column_types) else { return Step::refuse(TYPE_MISSING) };
        let count = ctx.snapshot.columns.values().filter(|column| column.storey == storey).count();
        let name = ctx.name_of(|labels| labels.kind_column, count);
        let id = ctx.mint("column");
        let column = Column { storey, column_type, position: point2(at), rotation: 0.0, base_offset: 0.0, top: TopConstraint::StoreyTop { offset: 0.0 }, phase: crate::Phase::New, name };
        Step::write(ctx, ModelMutation::CreateColumn(crate::mutations::create_column::CreateColumn { id, column }))
    }

    fn space(&self, ctx: &mut ToolContext<'_>, at: P) -> Step {
        let Some(storey) = ctx.storey().map(str::to_string) else { return Step::refuse(STOREY_MISSING) };
        let number = next_space_number(ctx, &storey);
        let name = ctx.name_of(|labels| labels.kind_space, number.parse::<usize>().unwrap_or_default().saturating_sub(1));
        let id = ctx.mint("space");
        let space = Space { storey, number, name, boundary: SpaceBoundary::Bounded { seed: point2(at) }, usage: String::new(), phase: crate::Phase::New, zone: None, floor_finish: None, wall_finish: None, ceiling_finish: None };
        Step::write(ctx, ModelMutation::CreateSpace(crate::mutations::create_space::CreateSpace { id, space }))
    }

    fn stair(&mut self, ctx: &mut ToolContext<'_>, foot: P, end: P) -> Step {
        let Some(storey) = ctx.storey().map(str::to_string) else { return Step::refuse(STOREY_MISSING) };
        let count = ctx.snapshot.stairs.values().filter(|stair| stair.storey == storey).count();
        let name = ctx.name_of(|labels| labels.kind_stair, count);
        let id = ctx.mint("stair");
        let stair = Stair { storey, start: point2(foot), direction: angle(foot, end), width: STAIR_WIDTH, flight: StairFlight::Straight, top: TopConstraint::StoreyTop { offset: 0.0 }, max_riser: STAIR_MAX_RISER, min_tread: STAIR_MIN_TREAD, stringer: crate::STANDARD_STRINGER, nosing: 0.0, tread_thickness: crate::STANDARD_TREAD_THICKNESS, riser: crate::STANDARD_RISER, landing_depth: STAIR_WIDTH, phase: crate::Phase::New, name };
        let step = Step::write(ctx, ModelMutation::CreateStair(crate::mutations::create_stair::CreateStair { id, stair }));
        if step.refused.is_none() {
            self.foot = None;
            self.held = false;
        }
        step
    }

    fn down(&mut self, ctx: &mut ToolContext<'_>, pointer: &Pointer) -> Step {
        let hit = ctx.snapped(pointer, self.foot, &[], &[]);
        let at = hit.point;
        self.hover = Some(hit);
        match (self.kind, self.foot) {
            (Kind::Column, _) => self.column(ctx, at),
            (Kind::Space, _) => self.space(ctx, at),
            (Kind::Stair, Some(foot)) if !self.held => {
                if same(foot, at) { Step::default() } else { self.stair(ctx, foot, at) }
            }
            (Kind::Stair, _) => {
                self.foot = Some(at);
                self.held = true;
                Step::default()
            }
        }
    }
}

/// 🔢️ The next free number of a storey's spaces: one above the highest numeric one, starting at 1.
pub fn next_space_number(ctx: &ToolContext<'_>, storey: &str) -> String {
    let highest = ctx.snapshot.spaces.values().filter(|space| space.storey == storey).filter_map(|space| space.number.trim().parse::<u64>().ok()).max();
    highest.map_or(1, |highest| highest + 1).to_string()
}

impl Tool for Point {
    fn anchor(&self) -> Option<P> {
        self.foot
    }

    fn event(&mut self, ctx: &mut ToolContext<'_>, event: &ToolEvent) -> Step {
        match event {
            ToolEvent::Move(pointer) => {
                self.hover = Some(ctx.snapped(pointer, self.foot, &[], &[]));
                Step::default()
            }
            ToolEvent::Down(pointer) => self.down(ctx, pointer),
            ToolEvent::Up(pointer) => {
                if self.kind != Kind::Stair || !self.held {
                    return Step::default();
                }
                self.held = false;
                let hit = ctx.snapped(pointer, self.foot, &[], &[]);
                let at = hit.point;
                self.hover = Some(hit);
                match self.foot {
                    Some(foot) if dist(foot, at) > DRAG_PIXELS * pointer.tolerance => self.stair(ctx, foot, at),
                    _ => Step::default(),
                }
            }
            ToolEvent::Double(_) | ToolEvent::Finish => Step::default(),
            ToolEvent::Escape | ToolEvent::Lost => {
                self.foot = None;
                self.held = false;
                Step::default()
            }
        }
    }

    fn preview(&self, _ctx: &ToolContext<'_>) -> Preview {
        let mut marks = Vec::new();
        let hover = self.hover.as_ref().map(|hit| hit.point);
        if let (Some(foot), Some(at)) = (self.foot, hover) {
            marks.push(Mark::path(&[foot, at], false, Style::Ghost));
            marks.push(length_label(foot, at));
        }
        marks.extend(self.hover.as_ref().map(Mark::snap));
        Preview::of(marks)
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
