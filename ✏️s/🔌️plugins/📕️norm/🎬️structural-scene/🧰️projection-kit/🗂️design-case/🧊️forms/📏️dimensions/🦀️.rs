//! 📏️ Annotation roles of the case forms (measured on the box of the body, on the plates of a joint form): the geometric inputs that no solid sizes (an end or edge distance of a fastener, a depth under a face, the angle of a load to the grain). A linear dimension (`dim_x`, `dim_xe`, `dim_y`, `dim_ye`, `dim_z`, `dim_ze`; an optional digit tells several of a kind apart) measures the input from a face of the body along an axis, an angular dimension (`dim_angle`) the angle between the grain direction (local `+X`) and the load direction in the vertical plane through the middle of the top face. Each is drawn as a dimension line with a label, a marker at the measured point (linear) or a thin sector (angular), and carries a grip bound to its input by `change-case-number`. See [`super`].

use super::*;
use std::f64::consts::PI;

const GAP: f64 = 0.15;
const MARK: f64 = 0.03;
const SECTOR_DEPTH: f64 = 0.004;
const SECTOR_STEPS: usize = 12;
const SECTOR_RGB: (u8, u8, u8) = (40, 120, 220);
const SECTOR_OPACITY: f64 = 0.6;
const RAY_SHARE: f64 = 0.3;
const RAY_MIN: f64 = 0.25;
const RAY_MAX: f64 = 3.0;
pub const ANGLE_MIN: f64 = 0.01;
pub const ANGLE_MAX: f64 = PI - ANGLE_MIN;

/// 🧭️ Where a dimension measures from.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stem {
    FromStart,
    FromEnd,
    FromNear,
    FromFar,
    FromBase,
    FromTop,
    Angle,
}

/// 🔎️ The stem of the annotation role `role` (`dim_x`, `dim_xe`, `dim_y`, `dim_ye`, `dim_z`, `dim_ze`, `dim_angle`, each with an optional trailing number); `None` for any other role.
pub fn dimension_role(role: &str) -> Option<Stem> {
    Some(match role.trim_end_matches(|c: char| c.is_ascii_digit()) {
        "dim_x" => Stem::FromStart,
        "dim_xe" => Stem::FromEnd,
        "dim_y" => Stem::FromNear,
        "dim_ye" => Stem::FromFar,
        "dim_z" => Stem::FromBase,
        "dim_ze" => Stem::FromTop,
        "dim_angle" => Stem::Angle,
        _ => return None,
    })
}

/// 🏷️ The tag of the parts, grip and annotations of the role `role`.
pub fn dimension_tag(role: &str) -> String {
    role.replace('_', "-")
}

/// 📐️ The angle input `key` of `row` in radians, within `0 < a < π`; `None` while it is unset or not an angle.
pub fn radians<C: CaseCatalogue>(row: &CaseRow, key: &str) -> Option<f64> {
    let unit = design_case::spec::<C>(&row.entry_id, key)?.unit;
    let value = row.numbers.iter().find(|n| n.id == key)?.value;
    unit.convert(value, UnitId::Radian).ok().filter(|a| a.is_finite() && *a > 0.0).map(|a| a.clamp(ANGLE_MIN, ANGLE_MAX))
}

enum Measure {
    Length { from: [f64; 3], dir: [f64; 3], offset: [f64; 3], metres: f64 },
    Angle { centre: [f64; 3], ray: f64, radians: f64 },
}

struct Item {
    tag: String,
    key: &'static str,
    measure: Measure,
}

fn bounds<C: CaseCatalogue>(m: &Meas<'_, C>, size: Size) -> Size {
    match m.g.form {
        CaseForm::Fastener | CaseForm::Weld | CaseForm::Anchor | CaseForm::Opening => joints::build(m).map_or(size, |built| Size { dx: 2.0 * built.half.0, dy: 2.0 * built.half.1, h: built.top }),
        _ => size,
    }
}

fn items<C: CaseCatalogue>(m: &Meas<'_, C>, size: Size) -> Vec<Item> {
    let Size { dx, dy, h } = bounds(m, size);
    let base = if m.g.form == CaseForm::Member { SUPPORT_HEIGHT } else { 0.0 };
    let top = base + h;
    let ray = (dx.max(dy).max(h) * RAY_SHARE).clamp(RAY_MIN, RAY_MAX);
    m.g.aux
        .iter()
        .filter_map(|(role, key)| Some((dimension_role(role)?, *role, *key)))
        .enumerate()
        .filter_map(|(slot, (stem, role, key))| {
            let gap = GAP * (slot % 6 + 1) as f64;
            let line = |from: [f64; 3], dir: [f64; 3], offset: [f64; 3]| Some((from, dir, offset));
            let placed = match stem {
                Stem::FromStart => line([-dx / 2.0, 0.0, top], [1.0, 0.0, 0.0], [0.0, -(dy / 2.0 + gap), 0.0]),
                Stem::FromEnd => line([dx / 2.0, 0.0, top], [-1.0, 0.0, 0.0], [0.0, -(dy / 2.0 + gap), 0.0]),
                Stem::FromNear => line([0.0, -dy / 2.0, top], [0.0, 1.0, 0.0], [-(dx / 2.0 + gap), 0.0, 0.0]),
                Stem::FromFar => line([0.0, dy / 2.0, top], [0.0, -1.0, 0.0], [dx / 2.0 + gap, 0.0, 0.0]),
                Stem::FromBase => line([dx / 2.0, -dy / 2.0, base], [0.0, 0.0, 1.0], [gap, 0.0, 0.0]),
                Stem::FromTop => line([dx / 2.0, -dy / 2.0, top], [0.0, 0.0, -1.0], [gap, 0.0, 0.0]),
                Stem::Angle => None,
            };
            let measure = match placed {
                Some((from, dir, offset)) => Measure::Length { from, dir, offset, metres: input_metres::<C>(m.row, key)? },
                None => Measure::Angle { centre: [0.0, 0.0, top], ray, radians: radians::<C>(m.row, key)? },
            };
            Some(Item { tag: dimension_tag(role), key, measure })
        })
        .collect()
}

fn along(from: [f64; 3], dir: [f64; 3], s: f64) -> [f64; 3] {
    [from[0] + s * dir[0], from[1] + s * dir[1], from[2] + s * dir[2]]
}

fn tip(centre: [f64; 3], ray: f64, radians: f64) -> [f64; 3] {
    [centre[0] + ray * radians.cos(), centre[1], centre[2] + ray * radians.sin()]
}

fn sector(pose: &CasePose, centre: [f64; 3], ray: f64, radians: f64) -> SolidShape {
    let rim = (0..=SECTOR_STEPS).map(|i| {
        let t = radians * i as f64 / SECTOR_STEPS as f64;
        Vec2::new(ray * t.cos(), ray * t.sin())
    });
    let base = Polygon2::solid(std::iter::once(Vec2::new(0.0, 0.0)).chain(rim).collect());
    SolidShape::Prism { frame: pose.axes([centre[0], SECTOR_DEPTH / 2.0, centre[2]], [1.0, 0.0, 0.0], [0.0, 0.0, 1.0], [0.0, -1.0, 0.0]), base, height: SECTOR_DEPTH }
}

fn part(m: &Meas<'_, impl CaseCatalogue>, item: &Item) -> Part {
    let (suffix, shape, name) = match &item.measure {
        Measure::Length { from, dir, metres, .. } => {
            let at = along(*from, *dir, *metres);
            ("mark", solid(m.pose, at, [MARK, MARK, MARK]), ("Dimension marker", "Maßmarke"))
        }
        Measure::Angle { centre, ray, radians } => ("arc", sector(m.pose, *centre, *ray, *radians), ("Angle sector", "Winkelsektor")),
    };
    Part::new(format!("{}-{suffix}", item.tag), shape, MaterialRole::Neutral, name).tinted(SECTOR_RGB, SECTOR_OPACITY).context(SECTOR_OPACITY)
}

fn plan(item: &Item) -> Plan {
    match &item.measure {
        Measure::Length { from, dir, metres, .. } => Plan { kind: GripKind::Spacing, ..added(&item.tag, item.key, 1.0, *dir, along(*from, *dir, *metres)) },
        Measure::Angle { centre, ray, radians } => angle(&item.tag, item.key, [0.0, -1.0, 0.0], *centre, tip(*centre, *ray, *radians), 0.0, (ANGLE_MIN, ANGLE_MAX)),
    }
}

/// 📏️ Adds the markers, sectors and grips of the annotation roles of the declaration of `m` to `built`; the roles whose input is unset draw and grip nothing.
pub(super) fn attach<C: CaseCatalogue>(m: &Meas<'_, C>, size: Size, built: &mut Built) {
    for item in items(m, size) {
        built.parts.push(part(m, &item));
        built.plans.push(plan(&item));
    }
}

/// 📝️ The dimension lines and labels of the annotation roles of `row` in the pose, `tone` coloured: per role a [`Annotation::Dimension`] (`ROW:dim-x`, bound to the input like its grip; an angle is the chord between the grain ray and the load ray with an angle quantity) and an [`Annotation::Label`] (`ROW:dim-x:label`) reading the input name and value.
pub fn annotations<C: CaseCatalogue>(row: &CaseRow, pose: &CasePose, tone: Tone) -> Vec<Annotation> {
    let Some(geometry) = declared::<C>(row) else { return Vec::new() };
    let role = MaterialRole::Neutral;
    let m = Meas::<C>::new(row, &geometry, pose, &role);
    items(&m, size::<C>(row, &geometry))
        .into_iter()
        .flat_map(|item| {
            let (en, de) = label_of::<C>(row, item.key);
            let Some(spec) = design_case::spec::<C>(&row.entry_id, item.key) else { return Vec::new() };
            let value = row.numbers.iter().find(|n| n.id == item.key).map_or(0.0, |n| n.value);
            let at = |p: [f64; 3]| pose.at(p[0], p[1], p[2]);
            let id = format!("{}:{}", row.id, item.tag);
            let (from, to, offset, quantity, binding) = match &item.measure {
                Measure::Length { from, dir, offset, metres } => {
                    let to = along(*from, *dir, *metres);
                    (at(*from), at(to), pose.dir(&v3(offset[0], offset[1], offset[2])), Quantity::new(QuantityKind::Length, *metres), size_binding::<C>(row, item.key, 1.0, grid_for(*metres)))
                }
                Measure::Angle { centre, ray, radians } => {
                    let binding = UnitId::Radian.convert(1.0, spec.unit).ok().map(|per_radian| handle_kit::field_binding(design_case::input_path(&row.id, item.key, true), Affine { base: value, k: per_radian }, spec.unit.symbol(), handle_kit::round(ANGLE_STEP * per_radian)));
                    (at(tip(*centre, *ray, 0.0)), at(tip(*centre, *ray, *radians)), Vec3::ZERO, Quantity::new(QuantityKind::Angle, *radians), binding)
                }
            };
            let anchor = Vec3 { x: (from.x + to.x) / 2.0 + offset.x, y: (from.y + to.y) / 2.0 + offset.y, z: (from.z + to.z) / 2.0 + offset.z };
            let text = LocalizedCopy::new(format!("{en} {value} {}", spec.unit.symbol()), format!("{de} {value} {}", spec.unit.symbol()));
            vec![Annotation::Dimension { id: id.clone(), from, to, offset, value: quantity, binding }, Annotation::Label { id: format!("{id}:label"), anchor, text, tone }]
        })
        .collect()
}
