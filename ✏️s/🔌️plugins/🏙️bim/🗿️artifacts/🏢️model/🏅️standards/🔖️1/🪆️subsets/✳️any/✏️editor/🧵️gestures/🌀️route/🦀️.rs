//! 🌀️ The MEP route tool: each click adds a vertex of a duct, pipe or cable tray at the current elevation, Enter or a double click writes the element as one `create-mep-element`, Backspace takes the last vertex back, Escape
//! drops the route (a second Escape leaves the tool). Tab changes the kind of section (duct, pipe, tray), Alt+T the system, Alt+PageUp and Alt+PageDown the elevation of the vertices to come; a click at the same plan point
//! with another elevation makes a riser. A new system brings the section that usually goes with it until a section is chosen.
//!
//! The same options can be typed as a line: `z <length>` (the elevation), `duct <width> x <height>`, `tray <width> x <height>`, `pipe <diameter>` and `sys <system>`; a length is in metres, or with its unit `mm`, `cm` or `m`.

use super::place::{length, next_system};
use super::plane::P;
use super::session::{GestureKey, Mark, Pointer, Preview, Step, Style, Tool, ToolContext, ToolEvent, ELEVATION_STEP, STOREY_MISSING};
use super::snap::SnapHit;
use super::typed::INVALID;
use crate::editor::bim::entities::components::{system_label, system_of, SYSTEMS};
use crate::editor::bim::entities::mep::{default_shape, shape_token, size_text, DUCT, ELEVATION, PIPE, TRAY};
use crate::{MepElement, MepShape, MepSystem, ModelMutation, Point3};

/// 🔬️ Two vertices closer than this (metres) are the same vertex.
const SAME: f64 = 1e-6;
/// 🌀️ The utility the tool is armed as.
pub const UTILITY: &str = "route";

/// 🌀️ The distance between two vertices in space.
fn span(a: &Point3, b: &Point3) -> f64 {
    ((b.x - a.x).powi(2) + (b.y - a.y).powi(2) + (b.z - a.z).powi(2)).sqrt()
}

fn section_of(token: &str) -> MepShape {
    match token {
        "duct" => MepShape::Duct { width: DUCT.0, height: DUCT.1 },
        "tray" => MepShape::Tray { width: TRAY.0, height: TRAY.1 },
        _ => MepShape::Pipe { diameter: PIPE },
    }
}

const TOKENS: [&str; 3] = ["duct", "pipe", "tray"];

/// 🌀️ The state of the route tool: the vertices so far, the system and section of the element, the elevation of the next vertex and the snapped pointer.
pub struct Route {
    vertices: Vec<Point3>,
    system: MepSystem,
    shape: MepShape,
    chosen: bool,
    elevation: f64,
    hover: Option<SnapHit>,
    last: Option<Pointer>,
}

impl Default for Route {
    fn default() -> Self {
        let system = SYSTEMS[0];
        Self { vertices: Vec::new(), system, shape: default_shape(system), chosen: false, elevation: ELEVATION, hover: None, last: None }
    }
}

impl Route {
    fn plan(&self) -> Vec<P> {
        self.vertices.iter().map(|vertex| [vertex.x, vertex.y]).collect()
    }

    fn anchor_point(&self) -> Option<P> {
        self.vertices.last().map(|vertex| [vertex.x, vertex.y])
    }

    fn snap(&self, ctx: &ToolContext<'_>, pointer: &Pointer) -> SnapHit {
        let extra: Vec<P> = self.vertices.first().map(|vertex| [vertex.x, vertex.y]).into_iter().collect();
        ctx.snapped(pointer, self.anchor_point(), &[], &extra)
    }

    fn commit(&mut self, ctx: &mut ToolContext<'_>) -> Step {
        let path = std::mem::take(&mut self.vertices);
        if path.len() < 2 {
            return Step::default();
        }
        let Some(storey) = ctx.storey().map(str::to_string) else { return Step::refuse(STOREY_MISSING) };
        let name = ctx.name_of(|labels| labels.kind_mep_element, ctx.snapshot.mep_elements.values().filter(|element| element.storey == storey).count());
        let id = ctx.mint("mep");
        let mep = MepElement { storey, system: self.system, shape: self.shape.clone(), path: path.clone(), name };
        let step = Step::write(ctx, ModelMutation::CreateMepElement(crate::mutations::create_mep_element::CreateMepElement { id, mep }));
        if step.refused.is_some() {
            self.vertices = path;
        }
        step
    }

    fn add(&mut self, at: P) {
        let vertex = Point3 { x: at[0], y: at[1], z: self.elevation };
        if self.vertices.last().is_none_or(|last| span(last, &vertex) > SAME) {
            self.vertices.push(vertex);
        }
    }

    fn describe(&self, ctx: &ToolContext<'_>) -> String {
        let section = format!("{} {}", shape_token(&self.shape), size_text(&self.shape));
        let system = ctx.labels.map_or_else(|| format!("{:?}", self.system), |labels| system_label(labels, self.system));
        format!("{section} · {system} · z {:.2} m", self.elevation)
    }

    fn typed(&mut self, word: &str, rest: &str) -> Step {
        let sizes = || -> Option<Vec<f64>> { rest.split(['x', '×']).map(length).collect::<Option<Vec<_>>>().filter(|sizes| sizes.iter().all(|size| *size > 0.0)) };
        let shape = match (word, sizes().as_deref()) {
            ("duct", Some([width, height])) => Some(MepShape::Duct { width: *width, height: *height }),
            ("tray", Some([width, height])) => Some(MepShape::Tray { width: *width, height: *height }),
            ("pipe", Some([diameter])) => Some(MepShape::Pipe { diameter: *diameter }),
            _ => None,
        };
        match (word, shape) {
            ("z", _) => length(rest).map_or_else(|| Step::refuse(INVALID), |elevation| {
                self.elevation = elevation;
                Step::default()
            }),
            ("sys", _) => system_of(rest).map_or_else(|| Step::refuse(INVALID), |system| {
                self.system = system;
                if !self.chosen {
                    self.shape = default_shape(system);
                }
                Step::default()
            }),
            (_, Some(shape)) => {
                self.shape = shape;
                self.chosen = true;
                Step::default()
            }
            _ => Step::refuse(INVALID),
        }
    }
}

impl Tool for Route {
    fn anchor(&self) -> Option<P> {
        self.anchor_point()
    }

    fn event(&mut self, ctx: &mut ToolContext<'_>, event: &ToolEvent) -> Step {
        match event {
            ToolEvent::Move(pointer) => {
                self.last = Some(*pointer);
                self.hover = Some(self.snap(ctx, pointer));
                Step::default()
            }
            ToolEvent::Down(pointer) => {
                self.last = Some(*pointer);
                let hit = self.snap(ctx, pointer);
                self.add(hit.point);
                self.hover = Some(hit);
                Step::default()
            }
            ToolEvent::Double(_) | ToolEvent::Finish => self.commit(ctx),
            ToolEvent::Escape => {
                if self.vertices.is_empty() {
                    self.hover = None;
                    return Step { arm: Some(crate::editor::bim::utilities::DEFAULT_UTILITY.to_string()), ..Step::default() };
                }
                self.vertices.clear();
                Step::default()
            }
            ToolEvent::Lost => {
                self.vertices.clear();
                self.hover = None;
                Step::default()
            }
            ToolEvent::Up(_) => Step::default(),
        }
    }

    fn key(&mut self, _ctx: &mut ToolContext<'_>, key: GestureKey) -> Option<Step> {
        match key {
            GestureKey::Back => {
                self.vertices.pop()?;
            }
            GestureKey::Next | GestureKey::Previous => {
                let at = TOKENS.iter().position(|token| *token == shape_token(&self.shape)).unwrap_or(0);
                let step = if key == GestureKey::Next { 1 } else { TOKENS.len() - 1 };
                self.shape = section_of(TOKENS[(at + step) % TOKENS.len()]);
                self.chosen = true;
            }
            GestureKey::Raise => self.elevation = ((self.elevation + ELEVATION_STEP) * 1e9).round() / 1e9,
            GestureKey::Lower => self.elevation = ((self.elevation - ELEVATION_STEP) * 1e9).round() / 1e9,
            GestureKey::System => {
                self.system = next_system(Some(self.system)).unwrap_or(SYSTEMS[0]);
                if !self.chosen {
                    self.shape = default_shape(self.system);
                }
            }
            GestureKey::Turn(_) | GestureKey::Mirror => return None,
        }
        Some(Step::default())
    }

    fn line(&mut self, _ctx: &mut ToolContext<'_>, line: &str) -> Option<Step> {
        let (word, rest) = line.trim().split_once(char::is_whitespace).map_or((line.trim(), ""), |(word, rest)| (word, rest));
        let word = word.to_ascii_lowercase();
        matches!(word.as_str(), "z" | "duct" | "tray" | "pipe" | "sys").then(|| self.typed(&word, rest))
    }

    fn preview(&self, ctx: &ToolContext<'_>) -> Preview {
        let mut marks = Vec::new();
        let mut run = self.plan();
        if let Some(hit) = self.hover.as_ref() {
            if !self.vertices.is_empty() {
                run.push(hit.point);
            }
            marks.push(Mark::label(hit.point, self.describe(ctx)));
            marks.push(Mark::snap(hit));
        }
        if run.len() >= 2 {
            marks.push(Mark::path(&run, false, Style::Ghost));
        }
        marks.extend(self.plan().into_iter().map(|at| Mark::dot(at, Style::Handle)));
        let total: f64 = self.vertices.windows(2).map(|pair| span(&pair[0], &pair[1])).sum();
        if let (Some(last), true) = (self.anchor_point(), total > 0.0) {
            marks.push(Mark::label(last, format!("{total:.2} m")));
        }
        Preview::of(marks)
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
