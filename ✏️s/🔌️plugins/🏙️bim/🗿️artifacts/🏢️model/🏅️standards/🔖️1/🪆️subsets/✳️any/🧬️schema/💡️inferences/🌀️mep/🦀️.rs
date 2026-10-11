//! 🌀️ `mep`: the inferred value of every routed MEP element (duct, pipe, cable tray). An element stores its system, its cross-section (`MepShape`, metres) and a polyline in space whose `z` is the height above the elevation
//! of its storey; everything else is derived here and never stored: the path in the building frame, the segments, the length, the section area and perimeter, the volume (section area times length: a mitred prism per segment has exactly that
//! volume), the lateral surface, a conservative bounding box, the system colour and the issues. Its only parent in the model graph is the storey node it stands on (its level).
//!
//! Conventions. Duct and tray: `width` is horizontal and `height` vertical on a horizontal run; on a vertical run the rectangle keeps the orientation of the run it continues. Pipe: a circle, tessellated within the chord tolerance of the
//! element solids (the section area and volume here are the closed forms, the solid is the tessellation). The reach of an element is half of the larger section dimension. The system palette is fixed (see [`colour`]).
//!
//! Related: <https://en.wikipedia.org/wiki/Duct_(flow)>, <https://en.wikipedia.org/wiki/Cable_tray>.

use super::super::element_solids::{dep_value, Anonymous, SolidBounds, SolidPoint};
use super::super::storey_levels::StoreyLevel;
use crate::{MepShape, MepSystem, ModelSnapshot, Point3};
use semio_framework_value::DslValue;

#[path = "💥️clash/🦀️.rs"]
pub mod clash;
#[path = "🩺️findings/🦀️.rs"]
pub mod findings;

//#region 🔖️Palette
/// 📏️ Lengths below this (metres) are no length.
pub const EPS: f64 = 1e-9;
/// 🔌️ A terminal is connected to a run that ends within this distance (metres) of its connector.
pub const CONNECT_DISTANCE: f64 = 0.05;

/// 🌀️ Every system, in declaration order.
pub const SYSTEMS: [MepSystem; 9] = [MepSystem::Supply, MepSystem::Return, MepSystem::Exhaust, MepSystem::DomesticWater, MepSystem::Waste, MepSystem::Gas, MepSystem::Power, MepSystem::Data, MepSystem::Lighting];

/// 🏷️ The stable kebab-case key of a system (`supply`, `domestic-water`, …): the key of the totals and the name of the solid part.
pub fn key(system: MepSystem) -> &'static str {
    match system {
        MepSystem::Supply => "supply",
        MepSystem::Return => "return",
        MepSystem::Exhaust => "exhaust",
        MepSystem::DomesticWater => "domestic-water",
        MepSystem::Waste => "waste",
        MepSystem::Gas => "gas",
        MepSystem::Power => "power",
        MepSystem::Data => "data",
        MepSystem::Lighting => "lighting",
    }
}

/// 🔎️ The system a key names.
pub fn system_of_key(key_text: &str) -> Option<MepSystem> {
    SYSTEMS.into_iter().find(|system| key(*system) == key_text)
}

/// 🎨️ The sRGB colour of a system as `#rrggbb`: supply blue, return orange, exhaust brown, domestic water cyan, waste olive, gas yellow, power red, data purple, lighting amber.
pub fn colour(system: MepSystem) -> &'static str {
    match system {
        MepSystem::Supply => "#1f77d4",
        MepSystem::Return => "#f08a24",
        MepSystem::Exhaust => "#8b5a2b",
        MepSystem::DomesticWater => "#1ec8d6",
        MepSystem::Waste => "#808000",
        MepSystem::Gas => "#f2d11e",
        MepSystem::Power => "#d62728",
        MepSystem::Data => "#8e44ad",
        MepSystem::Lighting => "#ffbf00",
    }
}

/// 🎨️ The sRGB colour of a system as components in `0..1`.
pub fn rgb(system: MepSystem) -> [f32; 3] {
    let hex = colour(system).trim_start_matches('#');
    let channel = |at: usize| u8::from_str_radix(&hex[at..at + 2], 16).map_or(0.0, |byte| f32::from(byte) / 255.0);
    [channel(0), channel(2), channel(4)]
}

/// 🎨️ The colour of the solid part `part` when it names a system (the groups of a MEP solid are named after their system).
pub fn part_colour(part: &str) -> Option<[f32; 3]> {
    system_of_key(part).map(rgb)
}
//#endregion 🔖️Palette

//#region 🔖️Values
/// 🔩️ The kind of a cross-section.
#[derive(semio_framework_value::RetireOwned, Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetainedClone)]
pub enum MepSectionKind {
    #[default]
    Duct,
    Pipe,
    Tray,
}

impl MepSectionKind {
    /// 🏷️ The stable key of the kind.
    pub fn key(self) -> &'static str {
        match self {
            Self::Duct => "duct",
            Self::Pipe => "pipe",
            Self::Tray => "tray",
        }
    }
}

/// 📐️ The cross-section of an element in metres and square metres: `width` and `height` (a pipe has both equal to its diameter), `area` and `perimeter` in closed form, and the size `label` (`300x200` in millimetres, `Ø100` for a pipe).
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetainedClone)]
pub struct MepSection {
    pub kind: MepSectionKind,
    pub width: f64,
    pub height: f64,
    pub area: f64,
    pub perimeter: f64,
    pub label: String,
}

impl MepSection {
    /// 📏️ Half of the larger dimension: how far the element reaches from its centre line in any direction.
    pub fn reach(&self) -> f64 {
        self.width.max(self.height) / 2.0
    }

    /// ✅️ Whether both dimensions are finite and positive.
    pub fn valid(&self) -> bool {
        self.width.is_finite() && self.height.is_finite() && self.width > EPS && self.height > EPS
    }
}

/// ➖️ One straight stretch of the centre line in the building frame (metres, `z` from the building datum).
#[derive(semio_framework_value::RetireOwned, Clone, Copy, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetainedClone)]
pub struct MepSegment {
    pub from: Point3,
    pub to: Point3,
    pub length: f64,
}

impl MepSegment {
    /// ⬆️ Whether the stretch runs vertically (no horizontal extent).
    pub fn vertical(&self) -> bool {
        (self.to.x - self.from.x).hypot(self.to.y - self.from.y) <= EPS
    }
}

/// 🚦️ What is wrong with an element.
#[derive(semio_framework_value::RetireOwned, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetainedClone)]
pub enum MepIssueCode {
    NonFinite,
    SectionDegenerate,
    PathDegenerate,
}

impl MepIssueCode {
    /// 🏷️ The stable kebab-case slug.
    pub fn slug(self) -> &'static str {
        match self {
            Self::NonFinite => "non-finite",
            Self::SectionDegenerate => "section-degenerate",
            Self::PathDegenerate => "path-degenerate",
        }
    }
}

/// 🚦️ One issue with an English fallback text.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetainedClone)]
pub struct MepIssue {
    pub code: MepIssueCode,
    pub detail: String,
}

/// 🌀️ Everything inferred about one MEP element. `path` and `segments` are in the building frame (`z` = storey elevation + authored `z`); `bounds` is the box of the centre line grown by the reach on every side (conservative).
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetainedClone)]
pub struct MepValue {
    pub storey: String,
    pub system: MepSystem,
    pub colour: String,
    pub section: MepSection,
    pub path: Vec<Point3>,
    pub segments: Vec<MepSegment>,
    pub length: f64,
    pub volume: f64,
    pub surface_area: f64,
    pub bounds: SolidBounds,
    pub issues: Vec<MepIssue>,
}

impl Default for MepValue {
    fn default() -> Self {
        Self { storey: String::new(), system: MepSystem::Supply, colour: colour(MepSystem::Supply).to_string(), section: MepSection::default(), path: Vec::new(), segments: Vec::new(), length: 0.0, volume: 0.0, surface_area: 0.0, bounds: SolidBounds::default(), issues: Vec::new() }
    }
}

impl MepValue {
    /// 🔌️ The two ends of the centre line.
    pub fn ends(&self) -> Option<(Point3, Point3)> {
        Some((*self.path.first()?, *self.path.last()?))
    }

    /// ✅️ Whether the element has a solid: a valid section and at least one segment.
    pub fn buildable(&self) -> bool {
        self.section.valid() && !self.segments.is_empty() && !self.issues.iter().any(|issue| issue.code == MepIssueCode::NonFinite)
    }
}
//#endregion 🔖️Values

//#region 🔖️Section
fn millimetres(metres: f64) -> String {
    let millimetres = metres * 1000.0;
    if (millimetres - millimetres.round()).abs() < 1e-6 {
        format!("{}", millimetres.round() as i64)
    } else {
        format!("{millimetres:.1}")
    }
}

/// 📐️ The cross-section of a shape.
pub fn section_of(shape: &MepShape) -> MepSection {
    match shape {
        MepShape::Duct { width, height } => MepSection { kind: MepSectionKind::Duct, width: *width, height: *height, area: width * height, perimeter: 2.0 * (width + height), label: format!("{}x{}", millimetres(*width), millimetres(*height)) },
        MepShape::Tray { width, height } => MepSection { kind: MepSectionKind::Tray, width: *width, height: *height, area: width * height, perimeter: 2.0 * (width + height), label: format!("{}x{}", millimetres(*width), millimetres(*height)) },
        MepShape::Pipe { diameter } => MepSection { kind: MepSectionKind::Pipe, width: *diameter, height: *diameter, area: std::f64::consts::PI * diameter * diameter / 4.0, perimeter: std::f64::consts::PI * diameter, label: format!("Ø{}", millimetres(*diameter)) },
    }
}
//#endregion 🔖️Section

//#region 🔖️Value
fn finite(point: &Point3) -> bool {
    point.x.is_finite() && point.y.is_finite() && point.z.is_finite()
}

fn distance(a: &Point3, b: &Point3) -> f64 {
    (b.x - a.x).hypot(b.y - a.y).hypot(b.z - a.z)
}

/// 🌀️ The value of element `id` on the storey whose level is `level`.
pub fn mep_of(snapshot: &ModelSnapshot, id: &str, level: &StoreyLevel) -> MepValue {
    let Some(element) = snapshot.mep_elements.get(id) else { return MepValue::default() };
    let section = section_of(&element.shape);
    let mut issues = Vec::new();
    let sound = element.path.iter().all(finite) && section.width.is_finite() && section.height.is_finite();
    if !sound {
        issues.push(MepIssue { code: MepIssueCode::NonFinite, detail: "a coordinate or a dimension is not a finite number".to_string() });
    } else if !section.valid() {
        issues.push(MepIssue { code: MepIssueCode::SectionDegenerate, detail: format!("the section is {} by {} m", section.width, section.height) });
    }
    let mut path: Vec<Point3> = Vec::new();
    let mut repeated = false;
    if sound {
        for point in element.path.iter().map(|point| Point3 { x: point.x, y: point.y, z: level.elevation + point.z }) {
            match path.last() {
                Some(last) if distance(last, &point) <= EPS => repeated = true,
                _ => path.push(point),
            }
        }
        if path.len() < 2 || repeated {
            issues.push(MepIssue { code: MepIssueCode::PathDegenerate, detail: if path.len() < 2 { "the run has no length".to_string() } else { "consecutive points of the run coincide".to_string() } });
        }
    }
    let segments: Vec<MepSegment> = path.windows(2).map(|pair| MepSegment { from: pair[0], to: pair[1], length: distance(&pair[0], &pair[1]) }).collect();
    let length: f64 = segments.iter().map(|segment| segment.length).sum();
    let reach = if section.valid() { section.reach() } else { 0.0 };
    let bounds = path.iter().fold(None, |bounds: Option<([f64; 3], [f64; 3])>, point| {
        let at = [point.x, point.y, point.z];
        Some(bounds.map_or((at, at), |(lo, hi)| ([lo[0].min(at[0]), lo[1].min(at[1]), lo[2].min(at[2])], [hi[0].max(at[0]), hi[1].max(at[1]), hi[2].max(at[2])])))
    });
    let bounds = bounds.map_or_else(SolidBounds::default, |(lo, hi)| SolidBounds { min: SolidPoint { x: lo[0] - reach, y: lo[1] - reach, z: lo[2] - reach }, max: SolidPoint { x: hi[0] + reach, y: hi[1] + reach, z: hi[2] + reach } });
    let buildable = section.valid() && !segments.is_empty();
    MepValue {
        storey: element.storey.clone(),
        system: element.system,
        colour: colour(element.system).to_string(),
        volume: if buildable { section.area * length } else { 0.0 },
        surface_area: if buildable { section.perimeter * length } else { 0.0 },
        section,
        path,
        segments,
        length,
        bounds,
        issues,
    }
}

/// 🔑️ Everything `mep_of` reads of the snapshot besides the level of its storey: the element record without its name.
pub fn dependency(snapshot: &ModelSnapshot, id: &str) -> DslValue {
    dep_value(&snapshot.mep_elements.get(id).map(Anonymous::anonymous))
}

/// 🗺️ The snapshot collections a MEP element reads.
pub const READS: &[&str] = &["mep_elements", "storeys", "buildings", "sites"];
//#endregion 🔖️Value

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
