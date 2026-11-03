//! 🪵️ Profile forms: the generic solids a catalogue asks of the shared forms beyond the plain boxes and cylinders: extruded profile prisms (tapered and curved-pitched members from a side elevation), laminated plates (a ply stack with its grain hatch), charred sections (the reduced outline and the char and front bands of a fire exposure), mode-shape overlays (the ghost of a member or a heat-mapped plate from a mode vector) and composite assemblies (a list of sub-forms with transforms). Prism and laminate follow declared input roles; the char, the mode and the assembly are data an artifact computes from its kernel and hands in as [`Extras`], so the forms stay domain-neutral. See [`super`].

use super::*;

pub const MODE_FIELD: &str = "mode-field";
const ZONE_RGB: (u8, u8, u8) = (230, 150, 40);
const ZONE_RAISE: f64 = 0.001;
const GHOST_RAISE: f64 = 0.003;
const ARC_STEPS: usize = 12;

/// 📥️ Everything a catalogue hands in per case besides the declared inputs: all optional, empty by default.
#[derive(Clone, Debug, Default)]
pub struct Extras { pub char: Option<Char>, pub mode: Option<Mode>, pub assembly: Option<Assembly>, pub attachments: Vec<Sub>, pub grips: Vec<Grip>, pub notes: Vec<Note> }

/// 🔥️ The fire exposure of a section: the bands and the residual section of the keyframes.
#[derive(Clone, Debug)]
pub struct Char { pub parts: Vec<CharPart> }

/// 🔥️ One char part: an outline with holes in section coordinates (`x` across the width, `y` up from the middle of the depth) extruded along the member, how it looks and how it reads.
#[derive(Clone, Debug)]
pub struct CharPart { pub tag: String, pub layer: String, pub rgb: (u8, u8, u8), pub opacity: f64, pub outline: Vec<[f64; 2]>, pub holes: Vec<Vec<[f64; 2]>>, pub name: (String, String), pub detail: (String, String) }

/// 🌊️ A vibration mode: the frequency, the peak amplitude in metres, the shape as `(position along the length 0..1, normalised displacement)` for the ghost of the body and, for a plate, the shape on a grid.
#[derive(Clone, Debug)]
pub struct Mode { pub frequency: f64, pub amplitude: f64, pub shape: Vec<(f64, f64)>, pub grid: Option<ModeGrid>, pub detail: (String, String) }

/// 🌊️ The mode of a plate on `columns` × `rows` nodes row by row (values `-1..1`) with the heat field `peak` times the absolute value, its legend title and the name of the surface.
#[derive(Clone, Debug)]
pub struct ModeGrid { pub columns: u32, pub rows: u32, pub values: Vec<f64>, pub peak: f64, pub unit: QuantityKind, pub title: (String, String), pub name: (String, String) }

/// 🧩️ A composite assembly (replacing the parts of the form): its sub-forms (the first is the body, a solid), the half extent in plan and the height of the topmost point, in the local frame of the case.
#[derive(Clone, Debug)]
pub struct Assembly { pub half: (f64, f64), pub top: f64, pub subs: Vec<Sub> }

/// 🧩️ One sub-form of an assembly or an attachment: where it stands (`at`, base centre) and how it is turned (`axes` x, y, z; none = the axes of the case), what it is and how it looks.
#[derive(Clone, Debug)]
pub struct Sub { pub tag: String, pub name: (String, String), pub detail: (String, String), pub role: MaterialRole, pub rgb: Option<(u8, u8, u8)>, pub opacity: f64, pub at: [f64; 3], pub axes: Option<[[f64; 3]; 3]>, pub shape: SubShape }

/// 🧩️ The shape of a sub-form in its own frame: a box (size along its axes), a cylinder, a laminate of plies `(thickness, parallel to the length)` over `length` × `width`, or a pattern of pins from `at` along the vector `to`.
#[derive(Clone, Debug)]
pub enum SubShape { Box { size: [f64; 3] }, Cylinder { radius: f64, height: f64 }, Laminate { length: f64, width: f64, plies: Vec<(f64, bool)> }, Pins { kind: PinKind, diameter: f64, to: [f64; 3], pattern: Pins } }

/// 📌️ The family of a pin.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PinKind { Nail, Dowel }

/// 📌️ The repetition of a pin: explicit offsets in the axis coordinates of the pin (x along the axis, then y, z) or a grid of `nx` × `ny` at `sx`, `sy`.
#[derive(Clone, Debug)]
pub enum Pins { Points(Vec<[f64; 3]>), Grid { nx: u32, ny: u32, sx: f64, sy: f64 } }

/// 🕹️ An extra grip of a case: the input `key` it writes with `factor`, along `axis` at `anchor` (local frame), of the [`GripKind`] that turns the drag into the input (face offset, scalar drag, spacing, count stepper or gauge).
#[derive(Clone, Debug)]
pub struct Grip { pub tag: String, pub key: String, pub factor: f64, pub kind: GripKind, pub axis: [f64; 3], pub anchor: [f64; 3] }

/// 🏷️ A text note in the drawing of a case at a local point.
#[derive(Clone, Debug)]
pub struct Note { pub tag: String, pub at: [f64; 3], pub text: (String, String) }

/// 🔌️ What a catalogue hands in per case: the extras for `row` drawn at `size`.
pub type Extend<'a> = &'a dyn Fn(&CaseRow, Size) -> Extras;

/// 🔌️ The extender of a catalogue without extras.
pub fn no_extras(_: &CaseRow, _: Size) -> Extras {
    Extras::default()
}

fn polygon(outline: &[[f64; 2]]) -> Polygon2 {
    Polygon2::solid(outline.iter().map(|p| Vec2::new(p[0], p[1])).collect())
}

fn rect(width: f64, height: f64) -> Polygon2 {
    polygon(&[[-width / 2.0, -height / 2.0], [width / 2.0, -height / 2.0], [width / 2.0, height / 2.0], [-width / 2.0, height / 2.0]])
}

fn plies(items: impl Iterator<Item = (f64, bool)>) -> Vec<GrainLayer> {
    let word = |along: bool, en: &'static str, de: &'static str| if along { en } else { de };
    items.enumerate().map(|(i, (thickness, along))| GrainLayer { thickness, role: if along { MaterialRole::Timber } else { MaterialRole::Neutral }, grain: if along { 0.0 } else { std::f64::consts::FRAC_PI_2 }, label: LocalizedCopy::new(format!("Ply {} ({})", i + 1, word(along, "longitudinal", "cross")), format!("Lage {} ({})", i + 1, word(along, "längs", "quer"))) }).collect()
}

/// 📐️ The side elevation `(x, up)` of a tapered member of length `len` and depth `h` at the start (`taper` angle in radians) or of a curved-pitched member (`apex`: pitch angle, inner radius, depth of the apex zone), the polygon of the curved zone (empty when tapered) and, for the curved member, the arc centre, the soffit apex and the top end of the arc (the anchors of its grips); the elevation starts at `x = 0` and rests on `y = 0`.
pub fn elevation(len: f64, h: f64, taper: Option<f64>, apex: Option<(f64, f64, f64)>) -> Option<(Vec<[f64; 2]>, Vec<[f64; 2]>, Vec<[f64; 2]>)> {
    if let Some((alpha, r_in, h_ap)) = apex {
        let leg = 2.5 * h_ap.max(0.05);
        let inner = |theta: f64, r: f64| [r * theta.sin(), r * theta.cos() - r_in];
        let arc = |r: f64| (0..=ARC_STEPS).map(|i| inner(-alpha + 2.0 * alpha * i as f64 / ARC_STEPS as f64, r)).collect::<Vec<_>>();
        let (lower, upper) = (arc(r_in), arc(r_in + h_ap));
        let (dl, dr) = ([alpha.cos(), alpha.sin()], [alpha.cos(), -alpha.sin()]);
        let (l0, l1, r0, r1) = (lower[0], upper[0], lower[ARC_STEPS], upper[ARC_STEPS]);
        let mut ring = vec![[l0[0] - leg * dl[0], l0[1] - leg * dl[1]]];
        ring.extend(lower.iter().copied());
        ring.push([r0[0] + leg * dr[0], r0[1] + leg * dr[1]]);
        ring.push([r1[0] + leg * dr[0], r1[1] + leg * dr[1]]);
        ring.extend(upper.iter().rev().copied());
        ring.push([l1[0] - leg * dl[0], l1[1] - leg * dl[1]]);
        let (min_x, min_y) = ring.iter().fold((f64::MAX, f64::MAX), |m, p| (m.0.min(p[0]), m.1.min(p[1])));
        let shift = |p: &[f64; 2]| [p[0] - min_x, p[1] - min_y];
        let zone = lower.iter().chain(upper.iter().rev()).map(shift).collect();
        let anchors = vec![shift(&[0.0, -r_in]), shift(&lower[ARC_STEPS / 2]), shift(&upper[ARC_STEPS])];
        return Some((ring.iter().map(shift).collect(), zone, anchors));
    }
    let alpha = taper?;
    Some((vec![[0.0, 0.0], [len, 0.0], [len, h + len * alpha.tan()], [0.0, h]], Vec::new(), Vec::new()))
}

/// 📏️ A member drawn from its elevation: the body prism, the optional zone prism, the height of the highest point over the support and the length of the profile.
pub(super) struct Profiled { pub body: Part, pub zone: Option<Part>, pub top: f64, pub width: f64, pub plans: Vec<Plan> }

/// 📐️ The prism of a member that declares `taper` (angle) or `apex_angle` with `apex_radius`: the elevation of the profile extruded across the width, resting on the supports; the apex zone is an orange prism on top. `None` for a straight member.
pub(super) fn profile<C: CaseCatalogue>(m: &Meas<'_, C>, size: Size) -> Option<Profiled> {
    if m.g.form != CaseForm::Member || !(m.has("taper") || m.has("apex_angle")) {
        return None;
    }
    let Size { dx, dy, h } = size;
    let angle = |role: &str| Some(m.angle(role, 0.0)).filter(|a| m.has(role) && *a > 0.0);
    let apex = angle("apex_angle").map(|alpha| (alpha, m.aux("apex_radius", 10.0 * h), h));
    let taper = angle("taper");
    let (ring, zone, anchors) = elevation(dx, h, taper, apex)?;
    let (width, top) = ring.iter().fold((0.0_f64, 0.0_f64), |m, p| (m.0.max(p[0]), m.1.max(p[1])));
    let centred = |points: &[[f64; 2]]| points.iter().map(|p| [p[0] - width / 2.0, p[1]]).collect::<Vec<_>>();
    let extruded = |points: &[[f64; 2]], raise: f64, depth: f64| SolidShape::Prism { frame: m.pose.axes([0.0, depth / 2.0, SUPPORT_HEIGHT + raise], [1.0, 0.0, 0.0], [0.0, 0.0, 1.0], [0.0, -1.0, 0.0]), base: polygon(&centred(points)), height: depth };
    let body = Part::new("", extruded(&ring, 0.0, dy), m.role.clone(), ("Profile member", "Profilbauteil"));
    let zone = (!zone.is_empty()).then(|| Part::new("apex", extruded(&zone, ZONE_RAISE, dy * 1.02), MaterialRole::Neutral, ("Apex zone", "Firstbereich")).tinted(ZONE_RGB, 0.6));
    let at = |p: &[f64; 2]| [p[0] - width / 2.0, 0.0, SUPPORT_HEIGHT + p[1]];
    let mut plans = Vec::new();
    match (taper, apex) {
        (_, Some(_)) => {
            plans.push(super::angle("apex-angle", m.key("apex_angle"), [0.0, 1.0, 0.0], at(&anchors[0]), at(&anchors[2]), std::f64::consts::FRAC_PI_2, (-std::f64::consts::FRAC_PI_2, -std::f64::consts::FRAC_PI_2 + 0.6)));
            plans.push(radius("apex-radius", m.key("apex_radius"), at(&anchors[0]), at(&anchors[1])));
        }
        (Some(alpha), None) => plans.push(super::angle("taper-angle", m.key("taper"), [0.0, -1.0, 0.0], [-dx / 2.0, 0.0, SUPPORT_HEIGHT + h], [dx / 2.0, 0.0, SUPPORT_HEIGHT + h + dx * alpha.tan()], 0.0, (0.0, 0.6))),
        _ => {}
    }
    Some(Profiled { body, zone, top, width, plans })
}

/// 🌳️ A plate or member drawn as a ply stack.
pub(super) struct Laminated { pub body: Part, pub plans: Vec<Plan>, pub base: f64, pub total: f64 }

/// 🌳️ The ply stack of a plate or member that declares the `plies` series (thicknesses `t1…tn`) and optionally `ply_dir` (orientation `o1…on`, 0.5 or more = parallel to the length, the grain of a cross ply lies at a right angle): a laminate over the plan, with a grip on the top of every ply after the first.
pub(super) fn laminate<C: CaseCatalogue>(m: &Meas<'_, C>, size: Size) -> Option<Laminated> {
    let thicknesses = m.series("plies");
    if !matches!(m.g.form, CaseForm::Member | CaseForm::Plate) || thicknesses.is_empty() {
        return None;
    }
    let dir = m.key("ply_dir");
    let parallel = |index: usize| dir.is_empty() || m.row.numbers.iter().find(|n| n.id == design_case::series_key(dir, index)).map_or(true, |n| n.value >= 0.5);
    let layers = plies(thicknesses.iter().enumerate().map(|(index, thickness)| (*thickness, parallel(index))));
    let base = if m.g.form == CaseForm::Member { SUPPORT_HEIGHT } else { 0.0 };
    let body = Part::new("", SolidShape::Laminate { frame: m.pose.frame([0.0, 0.0, base]), footprint: rect(size.dx, size.dy), layers }, m.role.clone(), ("Ply stack", "Lagenaufbau"));
    let tops: Vec<f64> = thicknesses.iter().scan(0.0, |top, thickness| { *top += thickness; Some(*top) }).collect();
    let plans = (1..thicknesses.len()).map(|index| added(&format!("ply-{}", index + 1), &design_case::series_key(m.key("plies"), index), 1.0, [0.0, 0.0, 1.0], [0.0, 0.0, base + tops[index]])).collect();
    Some(Laminated { body, plans, base, total: thicknesses.iter().sum() })
}

fn frame_of(pose: &CasePose, at: [f64; 3], axes: Option<[[f64; 3]; 3]>) -> Frame {
    axes.map_or_else(|| pose.frame(at), |[x, y, z]| pose.axes(at, x, y, z))
}

fn sub_part(pose: &CasePose, sub: &Sub, tag: &str) -> Part {
    let frame = frame_of(pose, sub.at, sub.axes);
    let name = (sub.name.0.as_str(), sub.name.1.as_str());
    let mut part = match &sub.shape {
        SubShape::Box { size } => Part::new(tag, SolidShape::Box { frame, size: v3(size[0], size[1], size[2]) }, sub.role.clone(), name),
        SubShape::Cylinder { radius, height } => Part::new(tag, SolidShape::Cylinder { frame, radius: *radius, height: *height }, sub.role.clone(), name),
        SubShape::Laminate { length, width, plies: items } => Part::new(tag, SolidShape::Laminate { frame, footprint: rect(*length, *width), layers: plies(items.iter().copied()) }, sub.role.clone(), name),
        SubShape::Pins { kind, diameter, to, pattern } => {
            let pattern = match pattern {
                Pins::Points(offsets) => Pattern::Points { offsets: offsets.iter().map(|o| v3(o[0], o[1], o[2])).collect() },
                Pins::Grid { nx, ny, sx, sy } => Pattern::Grid { nx: *nx, ny: *ny, sx: *sx, sy: *sy },
            };
            let kind = if *kind == PinKind::Nail { FastenerKind::Nail } else { FastenerKind::Dowel };
            let axis = Segment3 { from: pose.at(sub.at[0], sub.at[1], sub.at[2]), to: pose.at(sub.at[0] + to[0], sub.at[1] + to[1], sub.at[2] + to[2]) };
            Part::of(tag, Geometry::Fastener(FastenerGeometry { kind, axis, diameter: *diameter, head: HeadShape::None, pattern: Some(pattern), hole_clearance: None }), sub.role.clone(), name)
        }
    };
    part.detail = Some(sub.detail.clone());
    part.opacity = sub.opacity;
    match sub.rgb {
        Some(rgb) => part.tinted(rgb, sub.opacity),
        None => part,
    }
}

fn section(pose: &CasePose, size: Size, base: f64, outline: Polygon2) -> SolidShape {
    SolidShape::Prism { frame: pose.axes([-size.dx / 2.0, 0.0, base + size.h / 2.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0]), base: outline, height: size.dx }
}

fn charred<C: CaseCatalogue>(m: &Meas<'_, C>, size: Size, char: &Char, built: &mut Built) {
    let base = if m.g.form == CaseForm::Member { SUPPORT_HEIGHT } else { 0.0 };
    let ring = |points: &Vec<[f64; 2]>| Ring2 { points: points.iter().map(|p| Vec2::new(p[0], p[1])).collect() };
    for part in &char.parts {
        let shape = section(m.pose, size, base, Polygon2 { outer: ring(&part.outline), holes: part.holes.iter().map(ring).collect() });
        built.parts.push(Part::new(part.tag.as_str(), shape, MaterialRole::Neutral, (part.name.0.as_str(), part.name.1.as_str())).tinted(part.rgb, part.opacity).detailed((part.detail.0.as_str(), part.detail.1.as_str())).on(&part.layer));
    }
}

fn ghost(mode: &Mode, samples: Vec<DisplacementSample>, surface: Vec<SurfaceSample>) -> DeformationOverlay {
    DeformationOverlay { kind: DeformationKind::Mode, index: 1, frequency: Some(Quantity::new(QuantityKind::Frequency, mode.frequency)), participation: 1.0, scale: 1.0, samples, surface, motion: Motion::Harmonic, load_factor: None }
}

fn overlay<C: CaseCatalogue>(m: &Meas<'_, C>, size: Size, mode: &Mode, built: &mut Built) {
    if !mode.shape.is_empty() {
        let samples = mode.shape.iter().map(|(at, u)| DisplacementSample { at: *at, delta: v3(0.0, 0.0, mode.amplitude * u) }).collect();
        built.parts[0].deformation = Some(ghost(mode, samples, Vec::new()));
        built.parts[0].detail = Some(mode.detail.clone());
    }
    let Some(grid) = mode.grid.as_ref().filter(|g| g.columns > 1 && g.rows > 1 && g.values.len() == (g.columns * g.rows) as usize) else { return };
    let (mut points, mut surface, mut values) = (Vec::new(), Vec::new(), Vec::new());
    for j in 0..grid.rows {
        for i in 0..grid.columns {
            let (u, v) = (size.dx * f64::from(i) / f64::from(grid.columns - 1), size.dy * f64::from(j) / f64::from(grid.rows - 1));
            let value = grid.values[(j * grid.columns + i) as usize];
            points.push(m.pose.at(u - size.dx / 2.0, v - size.dy / 2.0, built.top + GHOST_RAISE));
            surface.push(SurfaceSample { at: Vec2::new(u, v), delta: v3(0.0, 0.0, mode.amplitude * value) });
            values.push(grid.peak * value.abs());
        }
    }
    let field = ScalarField { id: MODE_FIELD.into(), unit: grid.unit, samples: FieldSamples::Vertex { values } };
    let mut plate = Part::of("mode", Geometry::Surface(SurfaceGeometry { shape: SurfaceShape::Grid { grid: PointGrid { rows: grid.rows, columns: grid.columns, points } } }), MaterialRole::Timber, (grid.name.0.as_str(), grid.name.1.as_str()));
    plate.channel = Some(ColourChannel::Fields { active: MODE_FIELD.into(), fields: vec![field] });
    plate.deformation = Some(ghost(mode, Vec::new(), surface));
    plate.opacity = 0.95;
    plate.detail = Some(mode.detail.clone());
    built.parts.push(plate);
}

/// 🧩️ The extras on the built parts of a form: the assembly replaces the parts of the form (its first sub-form is the body), the char and the mode add parts, the extra grips come after the size grips.
pub(super) fn apply<C: CaseCatalogue>(m: &Meas<'_, C>, size: Size, extras: &Extras, built: &mut Built) {
    if let Some(assembly) = &extras.assembly {
        built.parts = assembly.subs.iter().enumerate().map(|(index, sub)| sub_part(m.pose, sub, if index == 0 { "" } else { sub.tag.as_str() })).collect();
        built.top = assembly.top;
        built.half = assembly.half;
    }
    built.parts.extend(extras.attachments.iter().map(|sub| sub_part(m.pose, sub, &sub.tag)));
    if let Some(char) = &extras.char {
        charred(m, size, char, built);
    }
    if let Some(mode) = &extras.mode {
        overlay(m, size, mode, built);
    }
    built.plans.extend(extras.grips.iter().map(|g| Plan { tag: g.tag.clone(), key: g.key.clone(), factor: g.factor, kind: g.kind.clone(), extra: true, axis: g.axis, anchor: g.anchor, curved: None }));
}

/// 📐️ The scalars that size one part: box, cylinder, wedge, prism, laminate, surface and pin sizes, the coordinates of a profile, the offsets of the frame and the displacements of its overlay.
pub(super) fn scalars(part: &Part) -> Vec<f64> {
    let offsets = |f: &Frame| [f.origin.x.abs(), f.origin.y.abs(), f.origin.z.abs()];
    let spread = |values: Vec<f64>| values.iter().copied().fold(f64::MIN, f64::max) - values.iter().copied().fold(f64::MAX, f64::min);
    let mut out = match &part.geometry {
        Geometry::Solid(solid) => match &solid.shape {
            SolidShape::Box { frame, size } => [vec![size.x, size.y, size.z], offsets(frame).to_vec()].concat(),
            SolidShape::Cylinder { frame, radius, height } => [vec![2.0 * radius, *height], offsets(frame).to_vec()].concat(),
            SolidShape::Wedge { frame, base, height } => [vec![base.x, base.y, *height], offsets(frame).to_vec()].concat(),
            SolidShape::Prism { frame, base, height } => [vec![*height], base.outer.points.iter().flat_map(|p| [p.x.abs(), p.y.abs()]).collect(), offsets(frame).to_vec()].concat(),
            SolidShape::Laminate { frame, footprint, layers } => [layers.iter().map(|l| l.thickness).collect(), vec![spread(footprint.outer.points.iter().map(|p| p.x).collect()), spread(footprint.outer.points.iter().map(|p| p.y).collect())], offsets(frame).to_vec()].concat(),
            _ => Vec::new(),
        },
        Geometry::Surface(SurfaceGeometry { shape: SurfaceShape::Grid { grid } }) => vec![spread(grid.points.iter().map(|p| p.x).collect()), spread(grid.points.iter().map(|p| p.y).collect())],
        Geometry::Fastener(pins) => {
            let (from, to) = (&pins.axis.from, &pins.axis.to);
            let length = ((to.x - from.x).powi(2) + (to.y - from.y).powi(2) + (to.z - from.z).powi(2)).sqrt();
            let pattern = match &pins.pattern {
                Some(Pattern::Points { offsets }) => offsets.iter().flat_map(|o| [o.x.abs(), o.y.abs(), o.z.abs()]).collect(),
                Some(Pattern::Grid { sx, sy, .. }) => vec![*sx, *sy],
                _ => Vec::new(),
            };
            [vec![pins.diameter, length, from.x.abs(), from.y.abs(), from.z.abs()], pattern].concat()
        }
        _ => Vec::new(),
    };
    if let Some(overlay) = &part.deformation {
        out.extend(overlay.samples.iter().map(|s| s.delta.z.abs()).chain(overlay.surface.iter().map(|s| s.delta.z.abs())));
    }
    out
}
