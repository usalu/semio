//! 🪑️ The component tool: the pointer carries a ghost of a family of the library, one click writes a `create-component` and the tool goes on placing until Escape. The family is the one selected in the library (else the first
//! placeable one) and Tab cycles it; Alt+R turns the ghost by 15 degrees (Ctrl+Alt+R by a quarter turn), Alt+M mirrors it, Alt+PageUp and Alt+PageDown change its elevation, Alt+T makes it a terminal of a system. A family of
//! the plumbing, lighting, electrical or casework category clings to the wall beside the pointer (the wall becomes the `host` of the component, the ghost stands on its face with its back to it); holding Ctrl places it free-standing.
//!
//! The same options can be typed as a line: `z <length>` (the elevation), `rot <degrees>`, `mirror`, `sys <system|none>` and `fam <id or name>`; a length is in metres, or in millimetres, centimetres or metres with its unit.

use super::plane::{axis_tangent_at, point2, project, P};
use super::session::{GestureKey, Mark, Pointer, Preview, Step, Style, Tool, ToolContext, ToolEvent, ELEVATION_STEP, PICK_PIXELS, REJECTED, STOREY_MISSING};
use super::snap::SnapHit;
use super::typed::INVALID;
use crate::editor::bim::entities::components::{placeable, system_of, wall_mounted, SYSTEMS};
use crate::standards::v1::subsets::any::schema::inferences::wall_layout::thickness_of;
use crate::{Axis, Component, MepSystem, ModelMutation, ModelSnapshot};
use std::f64::consts::TAU;

/// 🚫️ The model holds no family the tool could place.
pub const FAMILY_MISSING: &str = "bim.tool.family-missing";
/// 🪑️ The utility the tool is armed as.
pub const UTILITY: &str = "component";
/// 📏️ The half side of the ghost of a family without a visible solid, in metres.
const FALLBACK_HALF: f64 = 0.2;
/// 📏️ The length of the front tick and the half length of the connector cross, in metres.
const TICK: f64 = 0.15;
const CROSS: f64 = 0.1;

//#region 🔖️Frame
/// 🧭️ Where the frame of a family instance stands in plan: the origin, the turn about the vertical axis in radians (counter-clockwise) and whether the local x axis is flipped before the turn. The local frame is x right, y depth
/// (the back of a wall-mounted family lies at y = 0 and the family stands out along +y), z up.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Frame {
    pub origin: P,
    pub yaw: f64,
    pub mirrored: bool,
}

impl Frame {
    /// 🧭️ The plan point of a point `local` of the family frame.
    pub fn world(&self, local: P) -> P {
        let x = if self.mirrored { -local[0] } else { local[0] };
        let (sin, cos) = self.yaw.sin_cos();
        [self.origin[0] + x * cos - local[1] * sin, self.origin[1] + x * sin + local[1] * cos]
    }
}

/// 🧱️ How a wall carries a component: the wall, the point of its face where the family origin stands and the turn that puts the local +y axis away from the wall.
#[derive(Clone, Debug, PartialEq)]
pub struct Mount {
    pub wall: String,
    pub origin: P,
    pub yaw: f64,
}

/// 🧱️ The face point and the turn of a component mounted on the wall `axis` of `thickness` at plan point `at`: `at` projects onto the axis, the origin stands on the face of the side `at` lies on (the left face when `at` is on
/// the axis), and the turn takes the local +y axis along the outward normal of that face.
pub fn fit(axis: &Axis, thickness: f64, at: P) -> (P, f64) {
    let found = project(axis, at);
    let tangent = axis_tangent_at(axis, found.offset);
    let sign = if found.side < 0.0 { -1.0 } else { 1.0 };
    let outward = [-tangent[1] * sign, tangent[0] * sign];
    ([found.point[0] + outward[0] * thickness / 2.0, found.point[1] + outward[1] * thickness / 2.0], (-outward[0]).atan2(outward[1]))
}

/// 🧱️ The wall of `storey` that carries a component put at `at`: the closest wall within half its thickness plus `reach` metres.
pub fn mount(snapshot: &ModelSnapshot, storey: &str, at: P, reach: f64) -> Option<Mount> {
    snapshot
        .walls
        .iter()
        .filter(|(_, wall)| wall.storey == storey)
        .filter_map(|(id, wall)| {
            let thickness = thickness_of(snapshot, wall);
            let distance = project(&wall.axis, at).distance;
            (distance <= thickness / 2.0 + reach).then(|| (distance, id, fit(&wall.axis, thickness, at)))
        })
        .min_by(|a, b| a.0.total_cmp(&b.0))
        .map(|(_, id, (origin, yaw))| Mount { wall: id.clone(), origin, yaw })
}
//#endregion 🔖️Frame

//#region 🔖️Options
fn clean(value: f64) -> f64 {
    (value * 1e9).round() / 1e9 + 0.0
}

/// 🧭️ An angle in radians turned into [0, 2π), without the noise of the additions.
pub fn normalized(angle: f64) -> f64 {
    let turned = clean(angle.rem_euclid(TAU));
    if (turned - TAU).abs() < 1e-9 { 0.0 } else { turned }
}

/// 📏️ A length a text names, in metres: a plain number is metres, `mm`, `cm` and `m` name their unit.
pub fn length(text: &str) -> Option<f64> {
    let text = text.trim();
    let (number, scale) = if let Some(rest) = text.strip_suffix("mm") {
        (rest, 0.001)
    } else if let Some(rest) = text.strip_suffix("cm") {
        (rest, 0.01)
    } else {
        (text.strip_suffix('m').unwrap_or(text), 1.0)
    };
    number.trim().parse::<f64>().ok().filter(|value| value.is_finite()).map(|value| clean(value * scale))
}

/// 🌀️ The system after `current` in the cycle none, the nine systems, none.
pub fn next_system(current: Option<MepSystem>) -> Option<MepSystem> {
    match current {
        None => SYSTEMS.first().copied(),
        Some(system) => SYSTEMS.iter().position(|known| *known == system).and_then(|at| SYSTEMS.get(at + 1)).copied(),
    }
}

/// 🌀️ The system a typed word names, `none` for no system.
fn system_word(word: &str) -> Option<Option<MepSystem>> {
    if word.eq_ignore_ascii_case("none") { Some(None) } else { system_of(word).map(Some) }
}
//#endregion 🔖️Options

struct Candidate {
    component: Component,
    frame: Frame,
    valid: bool,
    hit: Option<SnapHit>,
}

/// 🪑️ The state of the component tool: the family, the turn, the mirror, the elevation, the terminal system and the candidate under the pointer.
#[derive(Default)]
pub struct Place {
    family: Option<String>,
    adopted: Option<String>,
    rotation: f64,
    mirrored: bool,
    elevation: f64,
    system: Option<MepSystem>,
    hover: Option<Candidate>,
    last: Option<Pointer>,
}

impl Place {
    fn current_family(&mut self, ctx: &ToolContext<'_>) -> Option<String> {
        let list = placeable(ctx.snapshot);
        let selected = ctx.library.iter().find(|id| list.iter().any(|(family, _)| family == *id)).cloned();
        if selected.is_some() && selected != self.adopted {
            self.family.clone_from(&selected);
        }
        self.adopted = selected;
        if self.family.as_ref().is_none_or(|family| !list.iter().any(|(known, _)| known == family)) {
            self.family = list.first().map(|(id, _)| id.clone());
        }
        self.family.clone()
    }

    fn cycle(&mut self, ctx: &ToolContext<'_>, step: isize) {
        let list = placeable(ctx.snapshot);
        let Some(count) = isize::try_from(list.len()).ok().filter(|count| *count > 0) else { return };
        let at = self.family.as_ref().and_then(|family| list.iter().position(|(known, _)| known == family)).map_or(0, |at| at as isize);
        self.family = Some(list[(at + step).rem_euclid(count) as usize].0.clone());
    }

    fn footprint(ctx: &ToolContext<'_>, family: &str) -> [f64; 4] {
        let boxes = ctx.inference.families.get(family).into_iter().flat_map(|value| value.visible()).map(|(_, mesh)| [mesh.bounds.min.x, mesh.bounds.min.y, mesh.bounds.max.x, mesh.bounds.max.y]);
        boxes.reduce(|a, b| [a[0].min(b[0]), a[1].min(b[1]), a[2].max(b[2]), a[3].max(b[3])]).unwrap_or([-FALLBACK_HALF, -FALLBACK_HALF, FALLBACK_HALF, FALLBACK_HALF])
    }

    fn candidate(&mut self, ctx: &mut ToolContext<'_>, pointer: &Pointer) -> Option<Candidate> {
        let storey = ctx.storey()?.to_string();
        let family = self.current_family(ctx)?;
        let category = ctx.snapshot.families.get(&family)?.category;
        let mounted = (wall_mounted(category) && !pointer.modifiers.subtractive()).then(|| mount(ctx.snapshot, &storey, pointer.at, pointer.tolerance * PICK_PIXELS)).flatten();
        let hit = mounted.is_none().then(|| ctx.snapped(pointer, None, &[], &[]));
        let at = hit.as_ref().map_or(pointer.at, |hit| hit.point);
        let frame = match &mounted {
            Some(mount) => Frame { origin: mount.origin, yaw: mount.yaw + self.rotation, mirrored: self.mirrored },
            None => Frame { origin: at, yaw: self.rotation, mirrored: self.mirrored },
        };
        let count = ctx.snapshot.components.values().filter(|component| component.family == family).count();
        let name = format!("{} {}", ctx.snapshot.families.get(&family).map_or("", |row| row.name.as_str()), count + 1);
        let component = Component { storey, family, position: point2(at), elevation: self.elevation, rotation: self.rotation, mirrored: self.mirrored, host: mounted.map(|mount| mount.wall), system: self.system, name };
        let probe = ModelMutation::CreateComponent(crate::mutations::create_component::CreateComponent { id: "probe".into(), component: component.clone() });
        Some(Candidate { valid: ctx.accepts(&probe), component, frame, hit })
    }

    fn down(&mut self, ctx: &mut ToolContext<'_>, pointer: &Pointer) -> Step {
        if ctx.storey().is_none() {
            return Step::refuse(STOREY_MISSING);
        }
        self.hover = self.candidate(ctx, pointer);
        match self.hover.as_ref() {
            None => Step::refuse(FAMILY_MISSING),
            Some(candidate) if !candidate.valid => Step::refuse(REJECTED),
            Some(candidate) => {
                let component = candidate.component.clone();
                let id = ctx.mint("component");
                Step::write(ctx, ModelMutation::CreateComponent(crate::mutations::create_component::CreateComponent { id, component }))
            }
        }
    }

    fn options(&self, ctx: &ToolContext<'_>) -> String {
        let family = self.family.as_ref().and_then(|id| ctx.snapshot.families.get(id)).map_or("", |row| row.name.as_str());
        let system = self.system.map(|system| format!(" · {}", ctx.labels.map_or_else(|| format!("{system:?}"), |labels| crate::editor::bim::entities::components::system_label(labels, system)))).unwrap_or_default();
        format!("{family} · {}°{} · z {:.2} m{system}", clean(self.rotation.to_degrees().round()), if self.mirrored { " ⇋" } else { "" }, self.elevation)
    }

    fn typed(&mut self, ctx: &mut ToolContext<'_>, word: &str, rest: &str) -> Step {
        match word {
            "z" => length(rest).map_or_else(|| Step::refuse(INVALID), |elevation| {
                self.elevation = elevation;
                Step::default()
            }),
            "rot" => rest.trim().parse::<f64>().ok().filter(|degrees| degrees.is_finite()).map_or_else(|| Step::refuse(INVALID), |degrees| {
                self.rotation = normalized(degrees.to_radians());
                Step::default()
            }),
            "mirror" => {
                self.mirrored = !self.mirrored;
                Step::default()
            }
            "sys" => system_word(rest.trim()).map_or_else(|| Step::refuse(INVALID), |system| {
                self.system = system;
                Step::default()
            }),
            _ => {
                let needle = rest.trim().to_lowercase();
                placeable(ctx.snapshot)
                    .into_iter()
                    .find(|(id, _)| id.to_lowercase() == needle || ctx.snapshot.families.get(id).is_some_and(|row| row.name.to_lowercase().contains(&needle)))
                    .filter(|_| !needle.is_empty())
                    .map_or_else(|| Step::refuse(INVALID), |(id, _)| {
                        self.family = Some(id);
                        Step::default()
                    })
            }
        }
    }
}

impl Tool for Place {
    fn event(&mut self, ctx: &mut ToolContext<'_>, event: &ToolEvent) -> Step {
        match event {
            ToolEvent::Move(pointer) => {
                self.last = Some(*pointer);
                self.hover = self.candidate(ctx, pointer);
                Step::default()
            }
            ToolEvent::Down(pointer) => {
                self.last = Some(*pointer);
                self.down(ctx, pointer)
            }
            ToolEvent::Escape => {
                self.hover = None;
                Step { arm: Some(crate::editor::bim::utilities::DEFAULT_UTILITY.to_string()), ..Step::default() }
            }
            ToolEvent::Lost => {
                self.hover = None;
                Step::default()
            }
            ToolEvent::Up(_) | ToolEvent::Double(_) | ToolEvent::Finish => Step::default(),
        }
    }

    fn key(&mut self, ctx: &mut ToolContext<'_>, key: GestureKey) -> Option<Step> {
        match key {
            GestureKey::Turn(radians) => self.rotation = normalized(self.rotation + radians),
            GestureKey::Mirror => self.mirrored = !self.mirrored,
            GestureKey::Next => self.cycle(ctx, 1),
            GestureKey::Previous => self.cycle(ctx, -1),
            GestureKey::Raise => self.elevation = clean(self.elevation + ELEVATION_STEP),
            GestureKey::Lower => self.elevation = clean(self.elevation - ELEVATION_STEP),
            GestureKey::System => self.system = next_system(self.system),
            GestureKey::Back => return None,
        }
        if let Some(pointer) = self.last {
            self.hover = self.candidate(ctx, &pointer);
        }
        Some(Step::default())
    }

    fn line(&mut self, ctx: &mut ToolContext<'_>, line: &str) -> Option<Step> {
        let (word, rest) = line.trim().split_once(char::is_whitespace).map_or((line.trim(), ""), |(word, rest)| (word, rest));
        let word = word.to_ascii_lowercase();
        let step = matches!(word.as_str(), "z" | "rot" | "mirror" | "sys" | "fam").then(|| self.typed(ctx, &word, rest))?;
        if let Some(pointer) = self.last {
            self.hover = self.candidate(ctx, &pointer);
        }
        Some(step)
    }

    fn preview(&self, ctx: &ToolContext<'_>) -> Preview {
        let Some(candidate) = self.hover.as_ref() else { return Preview::default() };
        let style = if candidate.valid { Style::Ghost } else { Style::Warning };
        let [x0, y0, x1, y1] = Self::footprint(ctx, &candidate.component.family);
        let ring: Vec<P> = [[x0, y0], [x1, y0], [x1, y1], [x0, y1]].iter().map(|corner| candidate.frame.world(*corner)).collect();
        let front = (x0 + x1) / 2.0;
        let origin = candidate.frame.origin;
        let mut marks = vec![Mark::path(&ring, true, style), Mark::path(&[candidate.frame.world([front, y1]), candidate.frame.world([front, y1 + TICK])], false, style), Mark::label(origin, self.options(ctx))];
        if candidate.component.system.is_some() {
            marks.push(Mark::path(&[candidate.frame.world([-CROSS, 0.0]), candidate.frame.world([CROSS, 0.0])], false, style));
            marks.push(Mark::path(&[candidate.frame.world([0.0, -CROSS]), candidate.frame.world([0.0, CROSS])], false, style));
        }
        marks.extend(candidate.hit.as_ref().map(Mark::snap));
        Preview::of(marks)
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
