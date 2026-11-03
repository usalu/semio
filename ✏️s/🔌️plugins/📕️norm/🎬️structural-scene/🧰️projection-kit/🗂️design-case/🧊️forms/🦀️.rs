//! 🧊️ Case forms: the generic 3D drawing of a design case from its declared [`CaseGeometry`]: the solids of its [`CaseForm`] sized by the dimension and auxiliary inputs in their stored units, a load bar or gauge pillar scaled by the load input, a utilisation colour from the live result, a label, and the grips that write the same inputs back plus the placement move; every artifact scene only decides where the cases stand and which collection and layer they belong to. The ground forms (footing, pile, walls, slope, soil block, anchors, seepage, well) live in [`ground`], the joint forms (fastener, weld, anchor, opening, symbol) in [`joints`], the profile forms (extruded profile prisms, laminated plates, charred sections, mode-shape overlays and composite assemblies, with the [`Extras`] a catalogue hands in as data) in [`profiles`] and the shared layout pass in [`layout`]. See [`super`].

use crate::*;
use super::super::handle_kit::{self, HandleSpec};
use super::super::utilisation::{binding_of_subject, tone_of, utilisation_words};
use super::super::vectors::{rotate_yaw, v3};
use super::{input_metres, input_per_metre, size_binding};
use norm::design_case::{self, CaseCatalogue, CaseForm, CaseGeometry, CaseRow};
use norm::{CheckReport, UnitId};
use std::marker::PhantomData;

pub const GRID: f64 = 0.05;
pub const ANGLE_STEP: f64 = 0.002;
pub const MIN_SIZE: f64 = 0.002;
pub const MAX_SIZE: f64 = 60.0;
pub const MARGIN: f64 = 1.0;
pub const SUPPORT_HEIGHT: f64 = 0.3;
pub const GAUGE_REFERENCE: f64 = 2.0;
pub const BAR_MIN: f64 = 0.1;
pub const BAR_MAX: f64 = 6.0;
pub const CONTEXT_OPACITY: f64 = 0.3;
pub const WATER_OPACITY: f64 = 0.55;
pub const MIN_BOTTOM_LAYER: f64 = 0.5;
pub const VOID_OPACITY: f64 = 0.85;

#[path = "📏️dimensions/🦀️.rs"]
mod dimensions;
#[path = "🕹️grips/🦀️.rs"]
mod grips;
#[path = "🌍️ground/🦀️.rs"]
mod ground;
#[path = "🔩️joints/🦀️.rs"]
mod joints;
#[path = "🪵️profiles/🦀️.rs"]
pub mod profiles;
#[path = "🗺️layout/🦀️.rs"]
pub mod layout;

pub use dimensions::{annotations as dimension_annotations, dimension_role, dimension_tag, radians as input_radians, Stem, ANGLE_MAX, ANGLE_MIN};
pub use grips::{Follow, GripKind};
pub use layout::{origins, origins_with, overlaps, PACK_WIDTH};
pub use profiles::{elevation, no_extras, Assembly, Char, CharPart, Extend, Extras, Grip, Mode, ModeGrid, Note, PinKind, Pins, Sub, SubShape};

/// 📍️ Where one case stands: origin in metres and yaw about `+Z` in radians.
#[derive(Clone, Debug, PartialEq)]
pub struct CasePose {
    pub origin: Vec3,
    pub yaw: f64,
}

impl CasePose {
    pub fn dir(&self, v: &Vec3) -> Vec3 {
        rotate_yaw(v, self.yaw)
    }

    pub fn at(&self, x: f64, y: f64, z: f64) -> Vec3 {
        let d = self.dir(&v3(x, y, z));
        v3(self.origin.x + d.x, self.origin.y + d.y, self.origin.z + d.z)
    }

    fn frame(&self, local: [f64; 3]) -> Frame {
        Frame { origin: self.at(local[0], local[1], local[2]), x: self.dir(&Vec3::X), y: self.dir(&Vec3::Y), z: Vec3::Z }
    }

    fn axes(&self, local: [f64; 3], x: [f64; 3], y: [f64; 3], z: [f64; 3]) -> Frame {
        let axis = |a: [f64; 3]| self.dir(&v3(a[0], a[1], a[2]));
        Frame { origin: self.at(local[0], local[1], local[2]), x: axis(x), y: axis(y), z: axis(z) }
    }
}

/// 📏️ The drawn size of a case in metres along its local axes: `dx` along X, `dy` along Y, `h` up.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Size {
    pub dx: f64,
    pub dy: f64,
    pub h: f64,
}

/// 📏️ The size a form takes for every dimension its entry leaves undeclared or unset: `(width, length, height)`.
pub const fn nominal(form: CaseForm) -> (f64, f64, f64) {
    match form {
        CaseForm::Block => (1.5, 1.5, 1.0),
        CaseForm::Member => (0.3, 4.0, 0.5),
        CaseForm::Column => (0.4, 0.4, 3.0),
        CaseForm::Plate => (3.0, 3.0, 0.2),
        CaseForm::Wall => (0.3, 3.0, 2.5),
        CaseForm::Cylinder => (0.6, 0.6, 3.0),
        CaseForm::Connection => (0.4, 0.8, 0.06),
        CaseForm::Load => (2.0, 2.0, 0.05),
        CaseForm::Gauge => (0.5, 0.5, GAUGE_REFERENCE),
        CaseForm::Symbol => (0.6, 0.6, 1.1),
        CaseForm::Fastener => (0.2, 0.4, 0.04),
        CaseForm::Weld => (0.2, 0.4, 0.02),
        CaseForm::Anchor => (0.6, 0.6, 0.4),
        CaseForm::Opening => (3.0, 3.0, 0.2),
        CaseForm::Footing => (2.0, 2.0, 0.5),
        CaseForm::Pile => (0.6, 0.6, 8.0),
        CaseForm::RetainingWall => (0.4, 3.0, 3.0),
        CaseForm::SheetWall => (0.3, 4.0, 4.0),
        CaseForm::Slope => (4.0, 8.0, 5.0),
        CaseForm::SoilBlock => (3.0, 3.0, 3.0),
        CaseForm::GroundAnchor => (0.15, 6.0, 0.15),
        CaseForm::Seepage => (3.0, 8.0, 2.0),
        CaseForm::Well => (15.0, 15.0, 10.0),
    }
}

/// 🔗️ The auxiliary roles a form understands: the schema of the `aux` declaration of [`CaseGeometry`]; a role outside this list is refused by the catalogue tests and by the geometry generator.
pub const fn aux_roles(form: CaseForm) -> &'static [&'static str] {
    match form {
        CaseForm::Member => &["grain", "eff_height", "notch_length", "taper", "apex_angle", "apex_radius", "plies", "ply_dir"],
        CaseForm::Column => &["grain"],
        CaseForm::Plate => &["grain", "plies", "ply_dir"],
        CaseForm::Block => &["grain"],
        CaseForm::Wall => &["grain"],
        CaseForm::Fastener => &["thickness2", "diameter", "count", "spacing", "rows", "spacing2"],
        CaseForm::Weld => &["throat"],
        CaseForm::Anchor => &["embed", "diameter"],
        CaseForm::Opening => &["open_w", "open_l"],
        CaseForm::Footing => &["embed", "layers", "water"],
        CaseForm::Pile => &["rows", "columns", "spacing", "layers", "water"],
        CaseForm::RetainingWall => &["base", "base_t", "toe", "water"],
        CaseForm::SheetWall => &["embed", "prop", "water"],
        CaseForm::Slope => &["angle", "ratio", "soft", "level"],
        CaseForm::SoilBlock => &["layers", "water", "rod"],
        CaseForm::GroundAnchor => &["free", "angle"],
        CaseForm::Seepage => &["embed", "layer"],
        CaseForm::Well => &["radius", "level"],
        _ => &[],
    }
}

/// 🔢️ Whether an auxiliary role of `form` moves or counts rather than sizes a solid: counts, angles and ratios, and levels and depths measured from another edge (toe, water, prop, well level); its input changes the drawing without a fixed proportion to one solid dimension.
pub const fn aux_is_scalar(form: CaseForm, role: &str) -> bool {
    match (form, role.as_bytes()) {
        (_, b"count" | b"rows" | b"columns" | b"angle" | b"ratio") => true,
        (CaseForm::Member, b"eff_height" | b"taper" | b"apex_angle" | b"apex_radius") | (CaseForm::RetainingWall, b"toe" | b"water") | (CaseForm::SheetWall, b"water" | b"prop") | (CaseForm::Well, b"level") => true,
        _ => false,
    }
}

/// ✅️ Whether `role` is an auxiliary role of `form`: one of its own roles ([`aux_roles`]) or an annotation role every form accepts ([`dimension_role`]).
pub fn is_aux_role(form: CaseForm, role: &str) -> bool {
    aux_roles(form).contains(&role) || dimension_role(role).is_some()
}

/// 🖍️ Whether an auxiliary role only decides how the solid looks (grain hatch, ply orientation), so that no drawn size depends on its input.
pub const fn aux_is_appearance(role: &str) -> bool {
    matches!(role.as_bytes(), b"grain" | b"ply_dir")
}

/// 📐️ Whether a dimension of `geometry` (`width`, `length`, `height`) drives its drawing without a fixed proportion to one drawn scalar: the height of a curved apex is the thickness measured across the arc, not a coordinate of the profile.
pub fn dimension_is_scalar(geometry: &CaseGeometry, name: &str) -> bool {
    name == "height" && geometry.aux_key("apex_angle").is_some_and(|key| !key.is_empty())
}

/// 🔀️ Whether the length runs along X and the width across (member, wall, connection, joints, seepage, anchors) rather than the width along X.
const fn lengthwise(form: CaseForm) -> bool {
    matches!(form, CaseForm::Member | CaseForm::Wall | CaseForm::Connection | CaseForm::Fastener | CaseForm::Weld | CaseForm::Seepage | CaseForm::GroundAnchor)
}

fn dimension<C: CaseCatalogue>(row: &CaseRow, key: &str, default: f64) -> f64 {
    if key.is_empty() { default } else { input_metres::<C>(row, key).map_or(default, |m| m.clamp(MIN_SIZE, MAX_SIZE)) }
}

/// 🧊️ The geometry the entry of `row` declares.
pub fn declared<C: CaseCatalogue>(row: &CaseRow) -> Option<CaseGeometry> {
    design_case::entry::<C>(&row.entry_id)?.geometry
}

/// 📏️ The drawn size of `row`: each declared and set dimension input in metres, the nominal size of the form otherwise.
pub fn size<C: CaseCatalogue>(row: &CaseRow, geometry: &CaseGeometry) -> Size {
    let (w0, l0, h0) = nominal(geometry.form);
    let (w, l, h) = (dimension::<C>(row, geometry.width, w0), dimension::<C>(row, geometry.length, l0), dimension::<C>(row, geometry.height, h0));
    match geometry.form {
        CaseForm::Cylinder | CaseForm::Pile => Size { dx: w, dy: w, h },
        CaseForm::Gauge | CaseForm::Symbol => Size { dx: w0, dy: l0, h },
        CaseForm::Well => Size { dx: 2.0 * w, dy: 2.0 * w, h },
        form if lengthwise(form) => Size { dx: l, dy: w, h },
        _ => Size { dx: w, dy: l, h },
    }
}

/// 🎚️ The load input of a case as it is drawn: stored value, metres per stored unit of the bar (the example of its vocabulary row halved, so the example reads 2 m) and the unit symbol.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Driver {
    pub value: f64,
    pub scale: f64,
    pub unit: &'static str,
}

impl Driver {
    pub fn height(&self) -> f64 {
        (self.value.abs() / self.scale).clamp(BAR_MIN, BAR_MAX)
    }
}

/// 🎚️ The driver of `row` when its entry declares a load key whose number row is set.
pub fn driver<C: CaseCatalogue>(row: &CaseRow, geometry: &CaseGeometry) -> Option<Driver> {
    if geometry.load.is_empty() {
        return None;
    }
    let value = row.numbers.iter().find(|n| n.id == geometry.load)?.value;
    let spec = design_case::spec::<C>(&row.entry_id, geometry.load)?;
    let scale = spec.example.map(f64::abs).filter(|e| *e > 0.0).map_or(1.0, |e| e / GAUGE_REFERENCE);
    value.is_finite().then_some(Driver { value, scale, unit: spec.unit.symbol() })
}

/// 🧱️ One solid of a case before it becomes a scene entity; a context part (soil, water) is translucent, coloured by role and carries no utilisation or grips.
struct Part {
    tag: String,
    geometry: Geometry,
    role: MaterialRole,
    name: (String, String),
    detail: Option<(String, String)>,
    opacity: f64,
    context: bool,
    hatch: Option<Hatch>,
    channel: Option<ColourChannel>,
    deformation: Option<DeformationOverlay>,
    layer: Option<String>,
}

impl Part {
    fn new(tag: impl Into<String>, shape: SolidShape, role: MaterialRole, name: (&str, &str)) -> Self {
        Self::of(tag, Geometry::Solid(SolidGeometry { shape }), role, name)
    }

    fn of(tag: impl Into<String>, geometry: Geometry, role: MaterialRole, name: (&str, &str)) -> Self {
        Self { tag: tag.into(), geometry, role, name: (name.0.to_string(), name.1.to_string()), detail: None, opacity: 1.0, context: false, hatch: None, channel: None, deformation: None, layer: None }
    }

    fn tinted(self, rgb: (u8, u8, u8), opacity: f64) -> Self {
        Self { channel: Some(ColourChannel::Custom { rgb: Rgb { r: rgb.0, g: rgb.1, b: rgb.2 } }), opacity, ..self }
    }

    fn detailed(self, detail: (&str, &str)) -> Self {
        Self { detail: Some((detail.0.to_string(), detail.1.to_string())), ..self }
    }

    fn on(self, layer: &str) -> Self {
        Self { layer: Some(layer.to_string()), ..self }
    }

    fn context(mut self, opacity: f64) -> Self {
        self.opacity = opacity;
        self.context = true;
        self
    }

    fn glass(mut self, opacity: f64) -> Self {
        self.opacity = opacity;
        self
    }
}

/// 🕹️ One size grip of a new-style form: the input key it writes, the drag factor (stored change per metre of drag in stored units per metre), a face offset, scalar drag, spacing, count stepper or gauge drag along `axis` ([`GripKind`]), anchored at a local point; an `extra` grip comes on top of the width, length and height grips of the plain forms instead of replacing them.
struct Plan { tag: String, key: String, factor: f64, kind: GripKind, extra: bool, axis: [f64; 3], anchor: [f64; 3], curved: Option<Curved> }

/// 🌀️ The absolute handles of a profile: the angle about `axis` through `centre` that zero is `zero` radians for (limits in radians), or the radius about `centre`.
enum Curved { Angle { centre: [f64; 3], zero: f64, limits: (f64, f64) }, Radius { centre: [f64; 3] } }

fn face(tag: &str, key: &str, factor: f64, axis: [f64; 3], anchor: [f64; 3]) -> Plan {
    Plan { tag: tag.to_string(), key: key.to_string(), factor, kind: GripKind::Face, extra: false, axis, anchor, curved: None }
}

fn scalar(tag: &str, key: &str, factor: f64, axis: [f64; 3], anchor: [f64; 3]) -> Plan {
    Plan { tag: tag.to_string(), key: key.to_string(), factor, kind: GripKind::Scalar, extra: false, axis, anchor, curved: None }
}

fn angle(tag: &str, key: &str, axis: [f64; 3], centre: [f64; 3], anchor: [f64; 3], zero: f64, limits: (f64, f64)) -> Plan {
    Plan { curved: Some(Curved::Angle { centre, zero, limits }), ..added(tag, key, 1.0, axis, anchor) }
}

fn radius(tag: &str, key: &str, centre: [f64; 3], anchor: [f64; 3]) -> Plan {
    Plan { curved: Some(Curved::Radius { centre }), ..added(tag, key, 1.0, [0.0, 1.0, 0.0], anchor) }
}

fn stepper(tag: &str, key: &str, (min, max): (f64, f64), anchor: [f64; 3]) -> Plan {
    Plan { kind: GripKind::Count { min, max }, ..scalar(tag, key, 1.0, [0.0, 0.0, 1.0], anchor) }
}

fn added(tag: &str, key: &str, factor: f64, axis: [f64; 3], anchor: [f64; 3]) -> Plan {
    Plan { extra: true, ..scalar(tag, key, factor, axis, anchor) }
}

/// 🧱️ What one form builds: its parts (the body first), the height of the topmost point, the half extent in plan and the lowest point, and the grips of the new-style forms.
struct Built { parts: Vec<Part>, top: f64, bottom: f64, half: (f64, f64), plans: Vec<Plan> }

/// 📏️ The measures of one case in metres or radians: dimension and auxiliary inputs with the nominal fallback.
struct Meas<'a, C: CaseCatalogue> {
    row: &'a CaseRow,
    g: &'a CaseGeometry,
    pose: &'a CasePose,
    role: &'a MaterialRole,
    n: (f64, f64, f64),
    catalogue: PhantomData<C>,
}

impl<'a, C: CaseCatalogue> Meas<'a, C> {
    fn new(row: &'a CaseRow, g: &'a CaseGeometry, pose: &'a CasePose, role: &'a MaterialRole) -> Self {
        Self { row, g, pose, role, n: nominal(g.form), catalogue: PhantomData }
    }

    fn w(&self) -> f64 {
        dimension::<C>(self.row, self.g.width, self.n.0)
    }

    fn l(&self) -> f64 {
        dimension::<C>(self.row, self.g.length, self.n.1)
    }

    fn h(&self) -> f64 {
        dimension::<C>(self.row, self.g.height, self.n.2)
    }

    fn has(&self, role: &str) -> bool {
        self.g.aux_key(role).is_some()
    }

    fn key(&self, role: &str) -> &'static str {
        self.g.aux_key(role).unwrap_or("")
    }

    fn aux(&self, role: &str, default: f64) -> f64 {
        dimension::<C>(self.row, self.key(role), default)
    }

    fn count(&self, role: &str, default: f64) -> f64 {
        let key = self.key(role);
        if key.is_empty() { default } else { self.row.numbers.iter().find(|n| n.id == key).filter(|n| n.value.is_finite()).map_or(default, |n| n.value.round().clamp(1.0, 12.0)) }
    }

    fn angle(&self, role: &str, default: f64) -> f64 {
        let key = self.key(role);
        let unit = design_case::spec::<C>(&self.row.entry_id, key).map(|spec| spec.unit);
        let value = self.row.numbers.iter().find(|n| n.id == key).map(|n| n.value);
        match (unit, value) {
            (Some(unit), Some(value)) => unit.convert(value, UnitId::Radian).ok().filter(|a| a.is_finite()).map_or(default, |a| a.clamp(0.05, 1.5)),
            _ => default,
        }
    }

    /// 🌳️ The grain direction of a solid that declares the `grain` role: lines along the length, across it when the declared input is set to 0.5 or more (an empty key draws the nominal grain along the length); `None` without the role.
    fn grain(&self) -> Option<Hatch> {
        let key = self.key("grain");
        self.has("grain").then(|| if !key.is_empty() && self.row.numbers.iter().any(|n| n.id == key && n.value >= 0.5) { Hatch::Vertical } else { Hatch::Horizontal })
    }

    fn ratio(&self, role: &str) -> Option<f64> {
        let key = self.key(role);
        self.row.numbers.iter().find(|n| n.id == key).map(|n| n.value).filter(|v| v.is_finite() && *v > 0.1)
    }

    fn series(&self, role: &str) -> Vec<f64> {
        let key = self.key(role);
        if key.is_empty() {
            return Vec::new();
        }
        (0..8).map_while(|index| input_metres::<C>(self.row, &design_case::series_key(key, index)).map(|m| m.clamp(MIN_SIZE, MAX_SIZE))).collect()
    }

    fn layers(&self) -> Vec<f64> {
        self.series("layers")
    }
}

fn solid(pose: &CasePose, at: [f64; 3], size: [f64; 3]) -> SolidShape {
    SolidShape::Box { frame: pose.frame(at), size: v3(size[0], size[1], size[2]) }
}

fn pillar(pose: &CasePose, at: [f64; 3], radius: f64, height: f64) -> SolidShape {
    SolidShape::Cylinder { frame: pose.frame(at), radius, height }
}

fn soil_role(class: &str) -> MaterialRole {
    MaterialRole::Soil { class: class.to_string() }
}

/// 🌍️ The stacked soil context of a ground form: one translucent box per layer thickness of `stack` from the ground surface down (centred at `cx` in X, half extents `hx` and `hy`) and, when `water` gives a level, the groundwater table as a thin sheet.
fn soil<C: CaseCatalogue>(m: &Meas<'_, C>, cx: f64, hx: f64, hy: f64, stack: &[f64], water: Option<f64>) -> Vec<Part> {
    let mut top = 0.0;
    let mut parts: Vec<Part> = stack
        .iter()
        .enumerate()
        .map(|(index, thickness)| {
            top -= thickness;
            Part::new(format!("soil-{index}"), solid(m.pose, [cx, 0.0, top], [2.0 * hx, 2.0 * hy, *thickness]), soil_role(&format!("layer-{}", index % 4)), ("Soil layer", "Bodenschicht")).context(CONTEXT_OPACITY)
        })
        .collect();
    if let Some(level) = water {
        parts.push(Part::new("water", solid(m.pose, [cx, 0.0, level], [2.0 * hx, 2.0 * hy, 0.03]), soil_role("water"), ("Groundwater table", "Grundwasserspiegel")).context(WATER_OPACITY));
    }
    parts
}

/// 🌍️ The thickness stack of the soil below a ground form: the declared layers and always one bottom layer that tops them up to `depth` (at least [`MIN_BOTTOM_LAYER`]), so the number of soil parts does not depend on the layer inputs.
fn strata<C: CaseCatalogue>(m: &Meas<'_, C>, depth: f64) -> Vec<f64> {
    let mut stack = m.layers();
    let sum: f64 = stack.iter().sum();
    stack.push((depth - sum).max(MIN_BOTTOM_LAYER));
    stack
}

/// 🧱️ The parts of a form in the pose, the first being the body; the height of the topmost point; the half extent in plan; the lowest point; the grips of the new-style forms; the markers, sectors and grips of the annotation roles come last.
fn build<C: CaseCatalogue>(row: &CaseRow, geometry: &CaseGeometry, pose: &CasePose, role: &MaterialRole, extras: &Extras) -> Built {
    let mut built = shape::<C>(row, geometry, pose, role, extras);
    dimensions::attach(&Meas::<C>::new(row, geometry, pose, role), size::<C>(row, geometry), &mut built);
    built
}

fn shape<C: CaseCatalogue>(row: &CaseRow, geometry: &CaseGeometry, pose: &CasePose, role: &MaterialRole, extras: &Extras) -> Built {
    let m = Meas::<C>::new(row, geometry, pose, role);
    if let Some(built) = ground::build(&m).or_else(|| joints::build(&m)) {
        return with_load(&m, built);
    }
    let Size { dx, dy, h } = size::<C>(row, geometry);
    let driver = driver::<C>(row, geometry);
    let part = |tag: &str, shape, role: &MaterialRole, name| Part::new(tag, shape, role.clone(), name);
    let steel = MaterialRole::Steel;
    let (mut list, top) = match geometry.form {
        CaseForm::Member => (
            vec![
                part("", solid(pose, [0.0, 0.0, SUPPORT_HEIGHT], [dx, dy, h]), role, ("Member", "Bauteil")),
                part("support-a", solid(pose, [-dx / 2.0 + SUPPORT_HEIGHT / 2.0, 0.0, 0.0], [SUPPORT_HEIGHT, dy, SUPPORT_HEIGHT]), &MaterialRole::Support, ("Support", "Lager")),
                part("support-b", solid(pose, [dx / 2.0 - SUPPORT_HEIGHT / 2.0, 0.0, 0.0], [SUPPORT_HEIGHT, dy, SUPPORT_HEIGHT]), &MaterialRole::Support, ("Support", "Lager")),
            ],
            SUPPORT_HEIGHT + h,
        ),
        CaseForm::Cylinder => (vec![part("", pillar(pose, [0.0, 0.0, 0.0], dx / 2.0, h), role, ("Cylinder", "Zylinder"))], h),
        CaseForm::Connection => (
            vec![
                part("", solid(pose, [-dx * 0.125, 0.0, 0.0], [dx * 0.75, dy, h]), role, ("Plate", "Blech")),
                part("plate-b", solid(pose, [dx * 0.125, 0.0, h], [dx * 0.75, dy, h]), role, ("Lapped plate", "Laschenblech")),
                part("fastener", pillar(pose, [0.0, 0.0, -0.01], (dx.min(dy) / 10.0).max(MIN_SIZE / 2.0), 2.0 * h + 0.02), &steel, ("Fastener", "Verbindungsmittel")),
            ],
            2.0 * h,
        ),
        CaseForm::Gauge => {
            let height = driver.map_or(1.0, |d| d.height());
            (
                vec![
                    part("", pillar(pose, [0.0, 0.0, 0.0], 0.25, height), role, ("Gauge pillar", "Messsäule")),
                    part("limit", solid(pose, [0.0, 0.0, GAUGE_REFERENCE], [1.0, 1.0, 0.02]), &steel, ("Reference mark", "Referenzmarke")),
                ],
                height.max(GAUGE_REFERENCE + 0.02),
            )
        }
        _ => (vec![part("", solid(pose, [0.0, 0.0, 0.0], [dx, dy, h]), role, ("Body", "Körper"))], h),
    };
    let mut plans = Vec::new();
    let (mut top, mut half) = (top, (dx / 2.0, dy / 2.0));
    let size = Size { dx, dy, h };
    if geometry.form == CaseForm::Member {
        notch(&m, size, &mut list, &mut plans);
    }
    if let Some(profile) = profiles::profile(&m, size) {
        list[0] = profile.body;
        list.extend(profile.zone);
        plans.extend(profile.plans);
        top = SUPPORT_HEIGHT + profile.top;
        half.0 = half.0.max(profile.width / 2.0);
    }
    if let Some(laminate) = profiles::laminate(&m, size) {
        list[0] = laminate.body;
        plans.extend(laminate.plans);
        top = laminate.base + laminate.total;
    }
    if let Some(hatch) = m.grain() {
        list[0].hatch = Some(hatch);
    }
    let mut built = Built { parts: list, top, bottom: 0.0, half, plans };
    profiles::apply(&m, size, extras, &mut built);
    if let (Some(driver), false) = (driver, geometry.form == CaseForm::Gauge) {
        let bar = driver.height();
        built.parts.push(part("load", pillar(pose, [0.0, 0.0, built.top], 0.08, bar), &MaterialRole::Load { kind: LoadKind::Variable }, ("Load", "Last")));
        built.top += bar;
    }
    built
}

/// 🪚️ The notch of a member that declares `eff_height` (the depth left over the notch, so the void is `h - h_ef` deep) and `notch_length`: a translucent void box at the start of the top face, with the grips of the effective depth and the notch length; no void while the input is unset or the effective depth is not smaller than the depth.
fn notch<C: CaseCatalogue>(m: &Meas<'_, C>, size: Size, list: &mut Vec<Part>, plans: &mut Vec<Plan>) {
    if !m.has("eff_height") {
        return;
    }
    let Size { dx, dy, h } = size;
    let depth = h - m.aux("eff_height", h);
    let reach = m.aux("notch_length", 0.02);
    let (x, top) = (-dx / 2.0 + reach / 2.0, SUPPORT_HEIGHT + h);
    if depth > MIN_SIZE {
        list.push(Part::new("notch", solid(m.pose, [x, 0.0, top - depth], [reach, dy * 1.02, depth]), MaterialRole::Neutral, ("Notch", "Ausklinkung")).context(VOID_OPACITY));
    }
    plans.push(added("effective-height", m.key("eff_height"), 1.0, [0.0, 0.0, 1.0], [x, 0.0, top - depth.max(0.0)]));
    plans.push(added("notch-length", m.key("notch_length"), 1.0, [1.0, 0.0, 0.0], [-dx / 2.0 + reach, 0.0, top]));
}

/// 🧱️ The load bar of a new-style form: added by the dispatcher of [`build`] for forms that return their own parts.
fn with_load<C: CaseCatalogue>(m: &Meas<'_, C>, mut built: Built) -> Built {
    if let Some(driver) = driver::<C>(m.row, m.g) {
        let bar = driver.height();
        built.parts.push(Part::new("load", pillar(m.pose, [0.0, 0.0, built.top], 0.08, bar), MaterialRole::Load { kind: LoadKind::Variable }, ("Load", "Last")));
        built.top += bar;
    }
    built
}

/// 📍️ The plan rectangle `[x0, x1, y0, y1]` a case occupies around its origin, with margin.
pub fn extent<C: CaseCatalogue>(row: &CaseRow) -> Option<[f64; 4]> { extent_with::<C>(row, &no_extras) }

/// 📍️ [`extent`] of a case whose catalogue hands in `ext` extras (an assembly changes the plan extent).
pub fn extent_with<C: CaseCatalogue>(row: &CaseRow, ext: Extend<'_>) -> Option<[f64; 4]> {
    let geometry = declared::<C>(row)?;
    let extras = ext(row, size::<C>(row, &geometry));
    let (hx, hy) = build::<C>(row, &geometry, &CasePose { origin: v3(0.0, 0.0, 0.0), yaw: 0.0 }, &MaterialRole::Neutral, &extras).half;
    Some([-hx - MARGIN, hx + MARGIN, -hy - MARGIN, hy + MARGIN])
}

fn range(base: f64, k: f64, lo: f64, hi: f64) -> (f64, f64) {
    let (a, b) = ((lo - base) / k, (hi - base) / k);
    (a.min(b), a.max(b))
}

fn named(en: &str, de: &str, id: &str) -> LocalizedCopy {
    LocalizedCopy::new(format!("{en} {id}"), format!("{de} {id}"))
}

fn label_of<C: CaseCatalogue>(row: &CaseRow, key: &str) -> (String, String) {
    design_case::spec::<C>(&row.entry_id, key).map_or_else(|| (key.to_string(), key.to_string()), |s| (s.en.to_string(), s.de.to_string()))
}

/// 🧲️ The drag step in metres for a dimension of `metres`: 5 cm for metre-sized dimensions, finer for centimetre and millimetre ones.
pub fn grid_for(metres: f64) -> f64 {
    if metres >= 0.5 { GRID } else if metres >= 0.05 { 0.01 } else { 0.001 }
}

/// 🕹️ The grips of one case, in the pose: width, length, height, auxiliary and load grips that write the declared inputs in their stored units (one release, one `change-case-number`), and the placement move; a dimension input that is not set has no grip.
pub fn handles<C: CaseCatalogue>(row: &CaseRow, pose: &CasePose, collection: &str) -> Vec<Handle> { handles_with::<C>(row, pose, collection, &no_extras) }

/// 🕹️ [`handles`] of a case whose catalogue hands in `ext` extras (its extra grips come after the size grips).
pub fn handles_with<C: CaseCatalogue>(row: &CaseRow, pose: &CasePose, collection: &str, ext: Extend<'_>) -> Vec<Handle> {
    let Some(geometry) = declared::<C>(row) else { return Vec::new() };
    let Size { dx, dy, h } = size::<C>(row, &geometry);
    let built = build::<C>(row, &geometry, pose, &MaterialRole::Neutral, &ext(row, Size { dx, dy, h }));
    let top = built.top;
    let entity = entity_ref(EntityKind::Solid, &row.id, collection);
    let base_z = if geometry.form == CaseForm::Member { SUPPORT_HEIGHT } else { 0.0 };
    let mid = base_z + h / 2.0;
    let mut out = Vec::new();
    let mut size_grip = |tag: &str, key: &str, factor: f64, kind: HandleKind, anchor: Vec3, style: GripStyle| {
        if key.is_empty() {
            return;
        }
        let Some((per_metre, _)) = input_per_metre::<C>(row, key) else { return };
        let grid = row.numbers.iter().find(|n| n.id == key).map_or(GRID, |n| grid_for((n.value / per_metre).abs()));
        let Some(binding) = size_binding::<C>(row, key, factor, grid) else { return };
        let (en, de) = label_of::<C>(row, key);
        let limits = range(binding.map.base, binding.map.k, MIN_SIZE * per_metre, MAX_SIZE * per_metre);
        out.push(handle_kit::handle(HandleSpec { entity: &entity, tag, kind, anchor, grip: style, writes: vec![binding], snap: handle_kit::grid(grid), limits, name: named(&en, &de, &row.id) }));
    };
    if built.plans.iter().all(|plan| plan.extra) {
        let (across, along) = (Vec3::X, Vec3::Y);
        let (width_axis, length_axis, width_reach, length_reach) = if lengthwise(geometry.form) { (along, across, dy, dx) } else { (across, along, dx, dy) };
        size_grip("width", geometry.width, 2.0, HandleKind::FaceOffset { normal: pose.dir(&width_axis) }, pose.at(width_axis.x * width_reach / 2.0, width_axis.y * width_reach / 2.0, mid), GripStyle::Arrow);
        if !matches!(geometry.form, CaseForm::Cylinder | CaseForm::Gauge) {
            size_grip("length", geometry.length, 2.0, HandleKind::FaceOffset { normal: pose.dir(&length_axis) }, pose.at(length_axis.x * length_reach / 2.0, length_axis.y * length_reach / 2.0, mid), GripStyle::Arrow);
        }
        if geometry.form != CaseForm::Gauge {
            size_grip("height", geometry.height, 1.0, HandleKind::ScalarAlong { axis: Vec3::Z }, pose.at(0.0, 0.0, base_z + h), GripStyle::Diamond);
        }
    }
    let mut curved_handles = Vec::new();
    for plan in &built.plans {
        let axis = v3(plan.axis[0], plan.axis[1], plan.axis[2]);
        if let Some(curved) = &plan.curved {
            let at = |p: &[f64; 3]| pose.at(p[0], p[1], p[2]);
            let Some(spec) = design_case::spec::<C>(&row.entry_id, &plan.key) else { continue };
            if !row.numbers.iter().any(|n| n.id == plan.key) {
                continue;
            }
            let (en, de) = label_of::<C>(row, &plan.key);
            let (kind, map, snap, limits, unit) = match curved {
                Curved::Angle { centre, zero, limits } => {
                    let Ok(per_radian) = UnitId::Radian.convert(1.0, spec.unit) else { continue };
                    (HandleKind::Angular { axis: pose.dir(&axis), centre: at(centre) }, Affine { base: zero * per_radian, k: per_radian }, ANGLE_STEP * per_radian, *limits, spec.unit.symbol())
                }
                Curved::Radius { centre } => {
                    let Some((per_metre, unit)) = input_per_metre::<C>(row, &plan.key) else { continue };
                    (HandleKind::Radial { centre: at(centre) }, Affine { base: 0.0, k: per_metre }, GRID * per_metre, (MIN_SIZE, MAX_SIZE), unit)
                }
            };
            curved_handles.push(handle_kit::handle(HandleSpec { entity: &entity, tag: &plan.tag, kind, anchor: at(&plan.anchor), grip: GripStyle::Ring, writes: vec![handle_kit::field_binding(design_case::input_path(&row.id, &plan.key, true), map, unit, handle_kit::round(snap))], snap: handle_kit::grid(snap), limits, name: named(&en, &de, &row.id) }));
            continue;
        }
        if plan.kind.quantity() {
            curved_handles.extend(grips::quantity::<C>(row, plan, pose, &entity));
            continue;
        }
        let (kind, style) = match plan.kind {
            GripKind::Face => (HandleKind::FaceOffset { normal: pose.dir(&axis) }, GripStyle::Arrow),
            GripKind::Spacing => (HandleKind::ScalarAlong { axis: pose.dir(&axis) }, GripStyle::Arrow),
            _ => (HandleKind::ScalarAlong { axis: pose.dir(&axis) }, GripStyle::Diamond),
        };
        size_grip(&plan.tag, &plan.key, plan.factor, kind, pose.at(plan.anchor[0], plan.anchor[1], plan.anchor[2]), style);
    }
    out.extend(curved_handles);
    if let (Some(driver), Some(value)) = (driver::<C>(row, &geometry), row.numbers.iter().find(|n| n.id == geometry.load)) {
        let (unit, sign) = (driver.unit, if driver.value < 0.0 { -1.0 } else { 1.0 });
        let binding = handle_kit::field_binding(design_case::input_path(&row.id, geometry.load, true), Affine { base: value.value, k: sign * driver.scale }, unit, handle_kit::round(driver.scale * GRID));
        let (en, de) = label_of::<C>(row, geometry.load);
        let limits = range(binding.map.base, binding.map.k, 0.0, sign * driver.scale * BAR_MAX);
        out.push(handle_kit::handle(HandleSpec { entity: &entity, tag: "load", kind: HandleKind::ScalarAlong { axis: Vec3::Z }, anchor: pose.at(0.0, 0.0, top), grip: GripStyle::Diamond, writes: vec![binding], snap: handle_kit::grid(GRID), limits, name: named(&en, &de, &row.id) }));
    }
    for (tag, axis, base, field, en, de) in [("move-x", Vec3::X, pose.origin.x, "x", "Move along X", "Entlang X verschieben"), ("move-y", Vec3::Y, pose.origin.y, "y", "Move along Y", "Entlang Y verschieben")] {
        let write = handle_kit::field_binding(format!("placements[id={}].{field}", row.id), Affine { base, k: 1.0 }, "m", handle_kit::round(GRID));
        out.push(handle_kit::handle(HandleSpec { entity: &entity, tag, kind: HandleKind::Translate { axis }, anchor: pose.at(0.0, 0.0, top), grip: GripStyle::Ring, writes: vec![write], snap: handle_kit::grid(GRID), limits: (-1.0e4, 1.0e4), name: named(en, de, &row.id) }));
    }
    out
}

/// 🎬️ Everything a scene needs to show one case: its scene entities (the body carries the grip ids), the points that bound it and its label.
pub struct CaseDraw {
    pub entities: Vec<SceneEntity>,
    pub points: Vec<Vec3>,
    pub label: Annotation,
    pub anchor: Vec3,
    pub notes: Vec<Annotation>,
    pub legends: Vec<Legend>,
}

#[allow(clippy::too_many_arguments)]
fn entity(id: String, collection: &str, part: Part, binding: &Option<UtilisationBinding>, name: LocalizedCopy, detail: LocalizedCopy, layer: &str, handles: Vec<String>) -> SceneEntity {
    let binding = binding.as_ref().filter(|_| !part.context);
    let (more_en, more_de) = utilisation_words(&binding.cloned());
    let failing = binding.is_some_and(|b| b.status == CheckStatus::Fail);
    let kind = match &part.geometry {
        Geometry::Fastener(_) => EntityKind::Fastener,
        Geometry::Surface(_) => EntityKind::Surface,
        _ => EntityKind::Solid,
    };
    SceneEntity {
        entity: entity_ref(kind, &id, collection),
        geometry: part.geometry,
        appearance: Appearance {
            role: part.role,
            channel: part.channel.unwrap_or(if binding.is_some() { ColourChannel::Utilisation } else { ColourChannel::Role }),
            style: if failing { StyleState::Error } else { StyleState::Normal },
            opacity: part.opacity,
            hatch: if failing { Some(Hatch::Diagonal) } else { part.hatch },
            deformation: part.deformation,
        },
        utilisation: binding.cloned(),
        label: Some(name.clone()),
        accessible_name: name,
        accessible_detail: LocalizedCopy::new(format!("{}{more_en}", detail.en), format!("{}{more_de}", detail.de)),
        layer: layer.to_string(),
        pick: PickPolicy::Whole,
        handles,
        content_hash: String::new(),
    }
    .sealed()
}

/// 🎬️ The drawing of one case: the solids of its form with the utilisation colour of the worst check addressed to the case, the load bar or gauge pillar of its load input, and the label `id entry` with the load value; `None` when the entry declares no geometry.
pub fn draw<C: CaseCatalogue>(row: &CaseRow, pose: &CasePose, report: &CheckReport, collection: &str, layer: &str, role: &MaterialRole) -> Option<CaseDraw> { draw_with::<C>(row, pose, report, collection, layer, role, &no_extras) }

/// 🎬️ [`draw`] of a case whose catalogue hands in `ext` extras: the char of a fire section, the mode of a floor, the sub-forms of an assembly, extra grips and notes.
pub fn draw_with<C: CaseCatalogue>(row: &CaseRow, pose: &CasePose, report: &CheckReport, collection: &str, layer: &str, role: &MaterialRole, ext: Extend<'_>) -> Option<CaseDraw> {
    let geometry = declared::<C>(row)?;
    let binding = binding_of_subject(report, &row.id, &[]);
    let Size { dx, dy, h } = size::<C>(row, &geometry);
    let extras = ext(row, Size { dx, dy, h });
    let built = build::<C>(row, &geometry, pose, role, &extras);
    let grips: Vec<String> = handles_with::<C>(row, pose, collection, ext).into_iter().map(|h| h.id).collect();
    let title = |text: &str| if text.is_empty() { row.id.clone() } else { text.to_string() };
    let (title_en, title_de) = (title(&row.label_en), title(&row.label_de));
    let mut points = vec![pose.at(-built.half.0, -built.half.1, built.bottom), pose.at(built.half.0, built.half.1, built.top)];
    let top = built.top;
    let entities = built
        .parts
        .into_iter()
        .map(|part| {
            let body = part.tag.is_empty();
            let id = if body { row.id.clone() } else { format!("{}:{}", row.id, part.tag) };
            let (name, detail) = if body {
                (LocalizedCopy::new(title_en.clone(), title_de.clone()), LocalizedCopy::new(format!("Design case {}: {dx:.2} x {dy:.2} x {h:.2} m", row.entry_id), format!("Bemessungsfall {}: {dx:.2} x {dy:.2} x {h:.2} m", row.entry_id)))
            } else {
                (LocalizedCopy::new(format!("{} {}", part.name.0, row.id), format!("{} {}", part.name.1, row.id)), LocalizedCopy::new(format!("{} of design case {}", part.name.0, row.id), format!("{} von Bemessungsfall {}", part.name.1, row.id)))
            };
            let detail = part.detail.as_ref().map_or(detail, |(en, de)| LocalizedCopy::new(en.clone(), de.clone()));
            let home = part.layer.clone().unwrap_or_else(|| layer.to_string());
            entity(id, collection, part, &binding, name, detail, &home, if body { grips.clone() } else { Vec::new() })
        })
        .collect();
    let anchor = pose.at(0.0, 0.0, top);
    points.push(anchor.clone());
    let load = driver::<C>(row, &geometry).map(|d| {
        let (en, de) = label_of::<C>(row, geometry.load);
        (format!(" - {en} {} {}", d.value, d.unit), format!(" - {de} {} {}", d.value, d.unit))
    });
    let (load_en, load_de) = load.unwrap_or_default();
    let label = Annotation::Label { id: format!("{}:label", row.id), anchor: anchor.clone(), text: LocalizedCopy::new(format!("{} {}{load_en}", row.id, row.entry_id), format!("{} {}{load_de}", row.id, row.entry_id)), tone: tone_of(&binding) };
    let notes = extras.notes.iter().map(|note| Annotation::Label { id: format!("{}:{}", row.id, note.tag), anchor: pose.at(note.at[0], note.at[1], note.at[2]), text: LocalizedCopy::new(note.text.0.clone(), note.text.1.clone()), tone: tone_of(&binding) }).chain(dimension_annotations::<C>(row, pose, tone_of(&binding))).collect();
    let legends = extras.mode.iter().filter_map(|mode| mode.grid.as_ref()).map(|grid| Legend { id: format!("mode:{}", row.id), title: LocalizedCopy::new(grid.title.0.clone(), grid.title.1.clone()), scale: LegendScale::Scalar { field_id: profiles::MODE_FIELD.into(), map: ColourMapRef { id: "warm".into(), min: 0.0, max: grid.peak.max(1e-9) }, unit: grid.unit } }).collect();
    Some(CaseDraw { entities, points, label, anchor, notes, legends })
}

/// 📐️ The solids of the drawing of `row` as the scalars that size them: box sizes, cylinder diameters and heights, wedge, prism and laminate extents and coordinates, surface and pin extents, mode amplitudes and the offsets of every frame from the pose origin; the law harness perturbs a driving input and looks for the scalar that follows it.
pub fn signature<C: CaseCatalogue>(row: &CaseRow, role: &MaterialRole) -> Vec<f64> { signature_with::<C>(row, role, &no_extras) }

/// 📐️ [`signature`] of a case whose catalogue hands in `ext` extras.
pub fn signature_with<C: CaseCatalogue>(row: &CaseRow, role: &MaterialRole, ext: Extend<'_>) -> Vec<f64> {
    let Some(geometry) = declared::<C>(row) else { return Vec::new() };
    let pose = CasePose { origin: v3(0.0, 0.0, 0.0), yaw: 0.0 };
    let built = build::<C>(row, &geometry, &pose, role, &ext(row, size::<C>(row, &geometry)));
    let mut out: Vec<f64> = built.parts.iter().flat_map(profiles::scalars).collect();
    out.extend([built.top, built.half.0, built.half.1]);
    out
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
