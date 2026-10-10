//! 🏗️ The frame tools: the column tilt, the curtain wall grid and the curtain panel override. The tilt tool takes a column and then the point its top should stand over, and writes the
//! `set-column-tilt` that leans the column there (a point over the foot makes it plumb). The grid tool adds a grid line at the pointer to the nearest curtain wall (along the wall in the plan
//! and the world, up the wall in a section that runs along it) and, with shift or the command key, takes the nearest line away; it writes the explicit line list of that direction into
//! the wall. The cell tool turns the cell under the pointer into an open panel (or back into the panel of the type), and with shift into a door. Every tool is one click, one history row.

use super::opening::nearest_host;
use super::plane::{angle, axis_ends, dist, P};
use super::session::{Mark, Pointer, Preview, Step, Style, Surface, Tool, ToolContext, ToolEvent, PICK_PIXELS, REJECTED};
use crate::standards::v1::subsets::any::schema::inferences::curtain_layout::CurtainLayout;
use crate::{Assigned, CurtainGrid, CurtainPanel, CurtainPanelOverride, ModelMutation, Slope};

/// 📏️ The shortest lean a tilt is written for, in metres of top offset.
const MIN_LEAN: f64 = 1e-3;
/// 📏️ How far from a column a click still takes it, beyond the pick reach, in metres.
const COLUMN_REACH: f64 = 0.3;
/// 📏️ The shortest distance between two grid lines and from a grid line to the edge of the wall, in metres.
const MIN_CELL: f64 = 0.05;
/// 📏️ How far a section line may stand from a curtain wall and still run along it, in metres.
const ALONG_REACH: f64 = 0.05;
/// 📏️ How far a click reaches a grid line to take it away, in metres.
const LINE_REACH: f64 = 0.3;

//#region 🔖️Tilt
/// 📐️ The state of the tilt tool: the column taken and its foot.
#[derive(Default)]
pub struct Tilt {
    column: Option<(String, P)>,
    hover: Option<P>,
}

impl Tilt {
    fn column_at(ctx: &ToolContext<'_>, pointer: &Pointer) -> Option<(String, P)> {
        let storey = ctx.storey()?;
        let reach = pointer.tolerance * PICK_PIXELS + COLUMN_REACH;
        ctx.snapshot
            .columns
            .iter()
            .filter(|(_, column)| column.storey == storey)
            .map(|(id, column)| (id, [column.position.x, column.position.y]))
            .map(|(id, foot)| (dist(foot, pointer.at), id, foot))
            .filter(|(distance, _, _)| *distance <= reach)
            .min_by(|a, b| a.0.total_cmp(&b.0))
            .map(|(_, id, foot)| (id.clone(), foot))
    }

    fn lean(ctx: &ToolContext<'_>, id: &str, foot: P, top: P) -> Option<Slope> {
        let height = ctx.snapshot.columns.get(id).and_then(|column| ctx.snapshot.storeys.get(&column.storey)).map_or(3.0, |storey| storey.height).max(1e-3);
        let reach = dist(foot, top);
        (reach >= MIN_LEAN).then(|| Slope { direction: angle(foot, top), angle: (reach / height).atan() })
    }
}

impl Tool for Tilt {
    fn event(&mut self, ctx: &mut ToolContext<'_>, event: &ToolEvent) -> Step {
        match event {
            ToolEvent::Move(pointer) => {
                self.hover = Some(pointer.at);
                Step::default()
            }
            ToolEvent::Down(pointer) => {
                self.hover = Some(pointer.at);
                let Some((id, foot)) = self.column.take() else {
                    self.column = Self::column_at(ctx, pointer);
                    return Step::default();
                };
                let tilt = Self::lean(ctx, &id, foot, pointer.at);
                Step::write(ctx, ModelMutation::SetColumnTilt(crate::mutations::set_column_tilt::SetColumnTilt { id, tilt }))
            }
            ToolEvent::Escape | ToolEvent::Lost => {
                *self = Self::default();
                Step::default()
            }
            ToolEvent::Up(_) | ToolEvent::Double(_) | ToolEvent::Finish => Step::default(),
        }
    }

    fn preview(&self, ctx: &ToolContext<'_>) -> Preview {
        let (Some((id, foot)), Some(top)) = (self.column.as_ref(), self.hover) else { return Preview::default() };
        let text = Self::lean(ctx, id, *foot, top).map_or_else(|| "0°".to_string(), |slope| format!("{:.1}°", slope.angle.to_degrees()));
        Preview::of(vec![Mark::dot(*foot, Style::Handle), Mark::path(&[*foot, top], false, Style::Ghost), Mark::label(top, text)])
    }

    fn anchor(&self) -> Option<P> {
        self.column.as_ref().map(|(_, foot)| *foot)
    }
}
//#endregion 🔖️Tilt

//#region 🔖️Hit
/// 🎯️ A curtain wall under the pointer: its id, its resolved layout, the distance along it and, in a section, the height above its base.
struct Hit<'a> {
    wall: String,
    layout: &'a CurtainLayout,
    u: f64,
    v: Option<f64>,
}

fn hit<'a>(ctx: &'a ToolContext<'_>, pointer: &Pointer) -> Option<Hit<'a>> {
    match &ctx.surface {
        Surface::Plan { .. } | Surface::World { .. } => {
            let found = nearest_host(ctx.snapshot, ctx.storey()?, pointer.at, pointer.tolerance * PICK_PIXELS).filter(|host| ctx.snapshot.curtain_walls.contains_key(&host.host))?;
            Some(Hit { layout: ctx.inference.curtain_layout.get(&found.host)?, wall: found.host, u: found.offset, v: None })
        }
        Surface::Section { start, end } => {
            let length = dist(*start, *end);
            if length < 1e-9 {
                return None;
            }
            let along = ((end[0] - start[0]) / length, (end[1] - start[1]) / length);
            let offset_of = |p: P| (p[0] - start[0]) * along.0 + (p[1] - start[1]) * along.1;
            let apart = |p: P| ((p[0] - start[0]) * along.1 - (p[1] - start[1]) * along.0).abs();
            ctx.snapshot.curtain_walls.iter().find_map(|(id, wall)| {
                let (a, b) = axis_ends(&wall.axis);
                let layout = ctx.inference.curtain_layout.get(id)?;
                let parallel = apart(a) <= ALONG_REACH && apart(b) <= ALONG_REACH && (offset_of(a) - offset_of(b)).abs() > 1e-9;
                let sign = if offset_of(b) >= offset_of(a) { 1.0 } else { -1.0 };
                let (u, v) = ((pointer.at[0] - offset_of(a)) * sign, pointer.at[1] - layout.base_z);
                (parallel && (0.0..=layout.length).contains(&u) && (0.0..=layout.height).contains(&v)).then(|| Hit { wall: id.clone(), layout, u, v: Some(v) })
            })
        }
        Surface::Sheet { .. } => None,
    }
}

//#endregion 🔖️Hit

//#region 🔖️Grid
/// 🕸️ The state of the grid tool: what the pointer hovers.
#[derive(Default)]
pub struct Lines {
    hover: Option<(String, f64, Option<f64>)>,
}

impl Lines {
    fn write(ctx: &ToolContext<'_>, found: &Hit<'_>, remove: bool) -> Step {
        let (edges, at) = match found.v {
            Some(v) => (&found.layout.v_edges, v),
            None => (&found.layout.u_edges, found.u),
        };
        let mut lines: Vec<f64> = edges.iter().copied().filter(|edge| *edge > MIN_CELL && *edge < edges.last().copied().unwrap_or(0.0) - MIN_CELL).collect();
        if remove {
            let Some(index) = lines.iter().enumerate().filter(|(_, line)| (**line - at).abs() <= LINE_REACH).min_by(|a, b| (a.1 - at).abs().total_cmp(&(b.1 - at).abs())).map(|(index, _)| index) else { return Step::default() };
            lines.remove(index);
        } else {
            let extent = edges.last().copied().unwrap_or(0.0);
            let at = (at * 1000.0).round() / 1000.0;
            if at <= MIN_CELL || at >= extent - MIN_CELL || lines.iter().any(|line| (*line - at).abs() < MIN_CELL) {
                return Step::default();
            }
            lines.push(at);
            lines.sort_by(f64::total_cmp);
        }
        let rule = Some(Assigned::new(Some(CurtainGrid::Lines { positions: lines })));
        let (u_grid, v_grid) = if found.v.is_some() { (None, rule) } else { (rule, None) };
        Step::write(ctx, ModelMutation::SetCurtainWallGrid(crate::mutations::set_curtain_wall_grid::SetCurtainWallGrid { id: found.wall.clone(), u_grid, v_grid }))
    }
}

impl Tool for Lines {
    fn event(&mut self, ctx: &mut ToolContext<'_>, event: &ToolEvent) -> Step {
        match event {
            ToolEvent::Move(pointer) => {
                self.hover = hit(ctx, pointer).map(|found| (found.wall, found.u, found.v));
                Step::default()
            }
            ToolEvent::Down(pointer) => {
                let step = hit(ctx, pointer).map_or_else(Step::default, |found| Self::write(ctx, &found, pointer.modifiers.shift || pointer.modifiers.subtractive()));
                if step.refused == Some(REJECTED) {
                    self.hover = None;
                }
                step
            }
            ToolEvent::Escape | ToolEvent::Lost => {
                self.hover = None;
                Step::default()
            }
            ToolEvent::Up(_) | ToolEvent::Double(_) | ToolEvent::Finish => Step::default(),
        }
    }

    fn preview(&self, ctx: &ToolContext<'_>) -> Preview {
        let Some((wall, u, _)) = self.hover.as_ref() else { return Preview::default() };
        let Some(curtain) = ctx.snapshot.curtain_walls.get(wall) else { return Preview::default() };
        let (a, b) = axis_ends(&curtain.axis);
        let length = dist(a, b).max(1e-9);
        let t = (u / length).clamp(0.0, 1.0);
        let at = [a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t];
        let normal = [-(b[1] - a[1]) / length * 0.15, (b[0] - a[0]) / length * 0.15];
        Preview::of(vec![Mark::path(&[a, b], false, Style::Guide), Mark::path(&[[at[0] - normal[0], at[1] - normal[1]], [at[0] + normal[0], at[1] + normal[1]]], false, Style::Ghost), Mark::label(at, format!("{u:.2} m"))])
    }
}
//#endregion 🔖️Grid

//#region 🔖️Cell
/// 🪟️ The state of the cell tool: the cell under the pointer.
#[derive(Default)]
pub struct Cell {
    hover: Option<(String, u32, u32)>,
}

impl Cell {
    fn cell_of(found: &Hit<'_>) -> Option<(u32, u32)> {
        let find = |edges: &[f64], at: f64| edges.windows(2).position(|pair| at >= pair[0] && at <= pair[1]).map(|index| index as u32);
        Some((find(&found.layout.u_edges, found.u)?, find(&found.layout.v_edges, found.v.unwrap_or(0.0))?))
    }

    fn write(ctx: &mut ToolContext<'_>, wall: &str, u: u32, v: u32, door: bool) -> Step {
        let existing = ctx.snapshot.curtain_panel_overrides.iter().find(|(_, row)| row.curtain == wall && row.u == u && row.v == v).map(|(id, row)| (id.clone(), row.panel.clone()));
        let panel = if door {
            let Some(door_type) = ctx.library_type(&ctx.snapshot.door_types) else { return Step::refuse(super::session::TYPE_MISSING) };
            CurtainPanel::Door { door_type }
        } else {
            CurtainPanel::Empty
        };
        match existing {
            Some((id, _)) if !door => Step::write(ctx, ModelMutation::DeleteCurtainPanelOverride(crate::mutations::delete_curtain_panel_override::DeleteCurtainPanelOverride { id })),
            Some((id, current)) if current == panel => Step::write(ctx, ModelMutation::DeleteCurtainPanelOverride(crate::mutations::delete_curtain_panel_override::DeleteCurtainPanelOverride { id })),
            Some((id, _)) => Step::write(ctx, ModelMutation::SetCurtainPanelOverride(crate::mutations::set_curtain_panel_override::SetCurtainPanelOverride { id, panel })),
            None => {
                let id = ctx.mint("curtain-panel-override");
                let curtain_panel_override = CurtainPanelOverride { curtain: wall.to_string(), u, v, panel };
                Step::write(ctx, ModelMutation::CreateCurtainPanelOverride(crate::mutations::create_curtain_panel_override::CreateCurtainPanelOverride { id, curtain_panel_override }))
            }
        }
    }
}

impl Tool for Cell {
    fn event(&mut self, ctx: &mut ToolContext<'_>, event: &ToolEvent) -> Step {
        match event {
            ToolEvent::Move(pointer) => {
                self.hover = hit(ctx, pointer).and_then(|found| Self::cell_of(&found).map(|(u, v)| (found.wall, u, v)));
                Step::default()
            }
            ToolEvent::Down(pointer) => {
                let Some((wall, u, v)) = hit(ctx, pointer).and_then(|found| Self::cell_of(&found).map(|(u, v)| (found.wall, u, v))) else { return Step::default() };
                self.hover = Some((wall.clone(), u, v));
                Self::write(ctx, &wall, u, v, pointer.modifiers.shift)
            }
            ToolEvent::Escape | ToolEvent::Lost => {
                self.hover = None;
                Step::default()
            }
            ToolEvent::Up(_) | ToolEvent::Double(_) | ToolEvent::Finish => Step::default(),
        }
    }

    fn preview(&self, ctx: &ToolContext<'_>) -> Preview {
        let Some((wall, u, v)) = self.hover.as_ref() else { return Preview::default() };
        let (Some(curtain), Some(layout)) = (ctx.snapshot.curtain_walls.get(wall), ctx.inference.curtain_layout.get(wall)) else { return Preview::default() };
        let (Some(from), Some(to)) = (layout.u_edges.get(*u as usize), layout.u_edges.get(*u as usize + 1)) else { return Preview::default() };
        let (a, b) = axis_ends(&curtain.axis);
        let length = dist(a, b).max(1e-9);
        let at = |offset: f64| [a[0] + (b[0] - a[0]) * offset / length, a[1] + (b[1] - a[1]) * offset / length];
        Preview::of(vec![Mark::path(&[at(*from), at(*to)], false, Style::Selection), Mark::label(at((from + to) / 2.0), format!("{u}, {v}"))])
    }
}
//#endregion 🔖️Cell

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
