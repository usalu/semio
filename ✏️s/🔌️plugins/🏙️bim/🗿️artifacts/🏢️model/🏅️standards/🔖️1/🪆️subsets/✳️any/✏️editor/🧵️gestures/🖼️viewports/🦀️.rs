//! 🖼️ The sheet tools, in paper millimetres (y downward): `Arrange` is the select tool of the sheet window and `Place` the place view tool.
//!
//! * **Arrange.** A press on a viewport window selects it (shift adds, ctrl or meta subtracts, both invert) and a drag moves it to the whole millimetre (`set-viewport` position; shift moves it in tenths). A press on the handle at the bottom right corner of a selected viewport
//!   and a drag scales it: the window follows the pointer and the scale jumps to the nearest standard scale (1:50, 1:75, 1:100, 1:125, 1:150, 1:200, ...), written as `set-viewport` scale. A press on nothing clears the selection. Nothing is written for a press
//!   that did not travel.
//! * **Place.** A click puts the selected view (else the next view that is not on the sheet yet) on the sheet with its top left corner at the click, rounded to the millimetre, at the scale of the view (`create-viewport`), and selects it.
//!
//! Both read the layout of the sheet (`sheet-layout`) for the windows and show the window they would write as a ghost with its position or scale.

use super::plane::{dist, point2, P};
use super::session::{Mark, Pointer, Preview, Step, Style, Surface, Tool, ToolContext, ToolEvent, PICK_PIXELS};
use crate::editor::bim::entities::sheets::{next_view, scale_of};
use crate::editor::bim::modes::edit::windows::sheet;
use crate::mutations::create_viewport::CreateViewport;
use crate::mutations::set_viewport::SetViewport;
use crate::standards::v1::subsets::any::schema::inferences::sheet_layout::windows::{place, WINDOW_PADDING};
use crate::standards::v1::subsets::any::schema::inferences::sheet_layout::{PaperRect, PlacedViewport, SheetLayout};
use crate::{view_is_drawn, ModelMutation, Viewport};

/// 🚫️ The model has no view the tool could put on the sheet.
pub const VIEW_MISSING: &str = "bim.create.view-missing";
/// 🖱️ How many pixels a press must travel before a drag counts.
const DRAG_PIXELS: f64 = 4.0;
/// 📏️ The standard drawing scales a scaled viewport jumps to.
pub const SERIES: [u32; 15] = [1, 2, 5, 10, 20, 25, 50, 75, 100, 125, 150, 200, 250, 500, 1000];

fn gap(scale: u32, wanted: f64) -> f64 {
    (f64::from(scale).ln() - wanted.max(1.0).ln()).abs()
}

/// 📏️ The standard scale nearest to `wanted`, on the logarithmic scale a drawing scale is read on.
pub fn nearest_scale(wanted: f64) -> u32 {
    SERIES.iter().copied().min_by(|a, b| gap(*a, wanted).total_cmp(&gap(*b, wanted))).unwrap_or(100)
}

fn snapped(point: P, fine: bool) -> P {
    let per_millimetre = if fine { 10.0 } else { 1.0 };
    [(point[0] * per_millimetre).round() / per_millimetre, (point[1] * per_millimetre).round() / per_millimetre]
}

fn layout<'a>(ctx: &'a ToolContext<'_>) -> Option<&'a SheetLayout> {
    let Surface::Sheet { sheet } = &ctx.surface else { return None };
    ctx.inference.sheet_layouts.get(sheet)
}

fn corners(rect: &PaperRect) -> [P; 4] {
    [[rect.x, rect.y], [rect.right(), rect.y], [rect.right(), rect.bottom()], [rect.x, rect.bottom()]]
}

fn shown(value: f64) -> String {
    format!("{}", (value * 10.0).round() / 10.0)
}

fn set(id: &str) -> SetViewport {
    SetViewport { id: id.to_string(), sheet: None, view: None, position: None, scale: None, crop: None, label: None }
}

/// 🪟️ The window the viewport `id` would have at `position` and `scale` (each when given, else its own).
fn recompute(ctx: &ToolContext<'_>, id: &str, position: Option<P>, scale: Option<u32>) -> Option<PlacedViewport> {
    let row = ctx.snapshot.viewports.get(id)?;
    let view = ctx.snapshot.views.get(&row.view)?;
    let edited = Viewport { position: position.map_or(row.position, point2), scale: scale.unwrap_or(row.scale), ..row.clone() };
    Some(place(id, &edited, view, ctx.inference.view_linework.get(&row.view)))
}

fn ghost(placed: &PlacedViewport, text: String) -> Preview {
    Preview::of(vec![Mark::path(&corners(&placed.window), true, Style::Ghost), Mark::label([placed.window.x, placed.window.y - 1.0], text)])
}

/// 🔧️ The scale handles of the selected viewports of a sheet: a dot on the bottom right corner of each window.
pub fn handles(layout: &SheetLayout, selected: &[String]) -> Vec<Mark> {
    layout.viewports.iter().filter(|placed| selected.contains(&placed.viewport)).map(|placed| Mark::dot(sheet::handle_of(&placed.window), Style::Handle)).collect()
}

//#region 🔖️Arrange
#[derive(Clone, Debug, Default, PartialEq)]
enum Drag {
    #[default]
    Idle,
    Move { id: String, origin: P, grab: P, to: P },
    Scale { id: String, window: PaperRect, pad: f64, scale: u32, to: u32 },
}

/// 🖱️ The select tool of the sheet window: pick, move and scale viewports.
#[derive(Default)]
pub struct Arrange {
    drag: Drag,
}

impl Arrange {
    fn press(&mut self, ctx: &ToolContext<'_>, pointer: &Pointer) -> Step {
        let Some(layout) = layout(ctx) else { return Step::default() };
        let reach = pointer.tolerance * PICK_PIXELS;
        let merge = pointer.modifiers.merge();
        let handle = layout.viewports.iter().filter(|placed| ctx.selected.contains(&placed.viewport)).find(|placed| dist(sheet::handle_of(&placed.window), pointer.at) <= reach);
        if let Some(placed) = handle {
            let pad = if placed.cropped { 0.0 } else { 2.0 * WINDOW_PADDING };
            self.drag = Drag::Scale { id: placed.viewport.clone(), window: placed.window, pad, scale: placed.scale, to: placed.scale };
            return Step::default();
        }
        match sheet::pick(layout, pointer.at, 0.0) {
            Some(placed) => {
                self.drag = Drag::Move { id: placed.viewport.clone(), origin: pointer.at, grab: [pointer.at[0] - placed.window.x, pointer.at[1] - placed.window.y], to: [placed.window.x, placed.window.y] };
                Step::select(vec![("viewport".to_string(), placed.viewport.clone())], merge)
            }
            None if merge == "replace" => Step::select(Vec::new(), "replace"),
            None => Step::default(),
        }
    }

    fn follow(&mut self, pointer: &Pointer) {
        match &mut self.drag {
            Drag::Move { grab, to, .. } => *to = snapped([pointer.at[0] - grab[0], pointer.at[1] - grab[1]], pointer.modifiers.shift),
            Drag::Scale { window, pad, scale, to, .. } => {
                let (old, new) = ((window.width - *pad).max(1.0), (pointer.at[0] - window.x - *pad).max(1.0));
                *to = nearest_scale(f64::from(*scale) * old / new);
            }
            Drag::Idle => {}
        }
    }

    fn release(&mut self, ctx: &ToolContext<'_>, pointer: &Pointer) -> Step {
        self.follow(pointer);
        match std::mem::take(&mut self.drag) {
            Drag::Move { id, origin, to, .. } if dist(origin, pointer.at) >= DRAG_PIXELS * pointer.tolerance => {
                let Some(row) = ctx.snapshot.viewports.get(&id) else { return Step::default() };
                if (row.position.x - to[0]).abs() < 1e-9 && (row.position.y - to[1]).abs() < 1e-9 {
                    return Step::default();
                }
                Step::write(ctx, ModelMutation::SetViewport(SetViewport { position: Some(point2(to)), ..set(&id) }))
            }
            Drag::Scale { id, scale, to, .. } if to != scale => Step::write(ctx, ModelMutation::SetViewport(SetViewport { scale: Some(to), ..set(&id) })),
            _ => Step::default(),
        }
    }
}

impl Tool for Arrange {
    fn event(&mut self, ctx: &mut ToolContext<'_>, event: &ToolEvent) -> Step {
        match event {
            ToolEvent::Down(pointer) => self.press(ctx, pointer),
            ToolEvent::Move(pointer) => {
                self.follow(pointer);
                Step::default()
            }
            ToolEvent::Up(pointer) => self.release(ctx, pointer),
            ToolEvent::Escape | ToolEvent::Lost => {
                self.drag = Drag::Idle;
                Step::default()
            }
            ToolEvent::Double(_) | ToolEvent::Finish => Step::default(),
        }
    }

    fn preview(&self, ctx: &ToolContext<'_>) -> Preview {
        match &self.drag {
            Drag::Move { id, to, origin, .. } if *to != *origin => recompute(ctx, id, Some(*to), None).map_or_else(Preview::default, |placed| ghost(&placed, format!("{}, {}", shown(to[0]), shown(to[1])))),
            Drag::Scale { id, to, scale, .. } if to != scale => recompute(ctx, id, None, Some(*to)).map_or_else(Preview::default, |placed| ghost(&placed, format!("1:{to}"))),
            _ => Preview::default(),
        }
    }
}
//#endregion 🔖️Arrange

//#region 🔖️Place
/// 🖼️ The place view tool: a click puts the selected view on the sheet.
#[derive(Default)]
pub struct Place {
    hover: Option<P>,
}

impl Place {
    fn target(ctx: &ToolContext<'_>) -> Option<(String, String)> {
        let Surface::Sheet { sheet } = &ctx.surface else { return None };
        let chosen = ctx.selected.iter().find(|id| ctx.snapshot.views.get(*id).is_some_and(|view| view_is_drawn(view.kind))).cloned();
        chosen.or_else(|| next_view(ctx.snapshot, sheet)).map(|view| (sheet.clone(), view))
    }
}

impl Tool for Place {
    fn event(&mut self, ctx: &mut ToolContext<'_>, event: &ToolEvent) -> Step {
        match event {
            ToolEvent::Move(pointer) => {
                self.hover = Some(pointer.at);
                Step::default()
            }
            ToolEvent::Down(pointer) => {
                let Some((sheet, view)) = Self::target(ctx) else { return Step::refuse(VIEW_MISSING) };
                let at = snapped(pointer.at, false);
                let id = ctx.mint("viewport");
                let viewport = Viewport { scale: scale_of(ctx.snapshot, &view), ..Viewport::standard(&sheet, &view, point2(at)) };
                Step::write(ctx, ModelMutation::CreateViewport(CreateViewport { id: id.clone(), viewport })).then(Step::select(vec![("viewport".to_string(), id)], "replace"))
            }
            ToolEvent::Escape | ToolEvent::Lost => {
                self.hover = None;
                Step::default()
            }
            ToolEvent::Up(_) | ToolEvent::Double(_) | ToolEvent::Finish => Step::default(),
        }
    }

    fn preview(&self, ctx: &ToolContext<'_>) -> Preview {
        let (Some(at), Some((sheet, view))) = (self.hover, Self::target(ctx)) else { return Preview::default() };
        let Some(row) = ctx.snapshot.views.get(&view) else { return Preview::default() };
        let at = snapped(at, false);
        let viewport = Viewport { scale: scale_of(ctx.snapshot, &view), ..Viewport::standard(&sheet, &view, point2(at)) };
        ghost(&place("preview", &viewport, row, ctx.inference.view_linework.get(&view)), format!("{}, {}", shown(at[0]), shown(at[1])))
    }

    fn anchor(&self) -> Option<P> {
        self.hover
    }
}
//#endregion 🔖️Place

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
