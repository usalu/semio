//! 🗂️ Case rows: one case row as a group of solids drawn from its numeric inputs: a timber block with taper or apex profile, notch void, hole and char layer for beam-type rows, stacked timber layers with a fastener grid and its end and edge distances for connection rows, CLT layups, floor modes and the domain scenes of [`special`]. All inputs are editable leaves of `cases[id=…].numbers[id=…].value`; the typed entities are drawn as the row they stand for (see [`super::typed`]).

use super::kit::*;
use super::facts;
use super::modes;
use super::model::CaseRow;
use super::scene::*;

/// 🧾 The drawn dimensions of a case in metres.
#[derive(Clone, Debug, PartialEq)]
pub struct Geo {
    pub fasteners: bool,
    pub b: f64,
    pub h: f64,
    pub len: f64,
    pub spans: f64,
    pub d: f64,
    pub t1: f64,
    pub t2: f64,
    pub a1: f64,
    pub a2: f64,
    pub end: f64,
    pub edge: f64,
    pub notch: f64,
    pub notch_x: f64,
    pub notch_len: f64,
    pub t_req_min: f64,
    pub nx: u32,
    pub ny: u32,
    pub points: Vec<(f64, f64)>,
    pub explicit: bool,
    pub profile: Profile,
    pub layers: Vec<(f64, bool)>,
    pub vibration: Option<(f64, f64, f64)>,
    pub faces: [f64; 4],
    pub plate: Option<Plate>,
}

impl Geo {
    /// ✂️ The length of the notch void along the member: the notch length when the case states it, else the distance `x`, at least 20 mm.
    pub fn reach(&self) -> f64 {
        if self.notch_len > 0.0 { self.notch_len } else { self.notch_x.max(0.02) }
    }
}

/// 🏢️ The plate data of a floor case: width across the joists in metres, transverse stiffness (EI)_b in N·m²/m and the damping ratio.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Plate {
    pub width: f64,
    pub ei_b: f64,
    pub zeta: f64,
}

/// 📐 Side-elevation family of a beam-type case.
#[derive(Clone, Debug, PartialEq)]
pub enum Profile {
    Straight,
    Tapered { alpha: f64 },
    Apex { alpha: f64, r_in: f64, h_ap: f64 },
}

fn mm(c: &CaseRow, key: &str) -> Option<f64> {
    c.numbers.iter().find(|n| n.id == key).map(|n| n.value / 1e3)
}

/// 📐 Dimensions of a case from its numeric inputs (mm inputs become metres, `span_m` is metres already).
pub fn geo(c: &CaseRow) -> Geo {
    let d = mm(c, "d").unwrap_or(0.0);
    let fasteners = d > 0.0 && mm(c, "t1").is_some();
    let has = |key: &str| c.numbers.iter().any(|n| n.id == key);
    let b = mm(c, "b").or_else(|| c.numbers.iter().find(|n| n.id == "deck_width_m" || n.id == "width_m").map(|n| n.value)).unwrap_or(if fasteners { 0.0 } else if has("n_layers") { 1.0 } else if has("span_m") { 0.5 } else { 0.1 }).max(0.0);
    let h = mm(c, "h").or_else(|| mm(c, "h_ap")).unwrap_or(0.2).max(0.01);
    let len = mm(c, "l").or_else(|| mm(c, "lef")).or_else(|| c.numbers.iter().find(|n| n.id == "span_m").map(|n| n.value)).unwrap_or(4.0 * h);
    let spans = if mm(c, "l").is_some() && c.texts.iter().any(|t| t.id == "load_case" && t.value.starts_with("two_span")) { 2.0 } else { 1.0 };
    let len = (len * spans).clamp(h, 12.0 * spans);
    let a1 = mm(c, "a1").unwrap_or(5.0 * d);
    let a2 = mm(c, "a2").unwrap_or(3.0 * d);
    let end = mm(c, "a3t").unwrap_or(7.0 * d);
    let edge = mm(c, "a4t").unwrap_or(3.0 * d);
    let h_ef = mm(c, "h_ef").unwrap_or(h);
    let number = |key: &str| c.numbers.iter().find(|n| n.id == key).map(|n| n.value);
    let profile = match (number("alpha_ap_deg"), number("alpha_deg")) {
        (Some(a), _) => Profile::Apex { alpha: a.to_radians(), r_in: mm(c, "r_in").unwrap_or(10.0 * h), h_ap: mm(c, "h_ap").unwrap_or(h) },
        (None, Some(a)) if !fasteners => Profile::Tapered { alpha: a.to_radians() },
        _ => Profile::Straight,
    };
    let layers: Vec<(f64, bool)> = (1..=number("n_layers").unwrap_or(0.0).min(9.0) as usize).filter_map(|i| mm(c, &format!("t{i}")).map(|t| (t, number(&format!("o{i}")).unwrap_or(1.0) >= 0.5))).collect();
    let vibration = number("span_m").and_then(|span| Some((span, number("ei_l").or_else(|| number("ei_nm2"))?, number("mass_kg_m2").or_else(|| number("mass_kg_m"))?)));
    let points: Vec<(f64, f64)> = (1..=72).map_while(|i| Some((mm(c, &format!("px{i}"))?, mm(c, &format!("py{i}"))?))).collect();
    let (nx, ny) = (number("rows").map_or(2, |v| v.clamp(1.0, 6.0) as u32), number("n").map_or(3, |v| v.clamp(1.0, 12.0) as u32));
    let face = |key: &str, default: f64| number(key).unwrap_or(default).clamp(0.0, 1.0).round();
    let faces = [face("face_left", 1.0), face("face_right", 1.0), face("face_top", 0.0), face("face_bottom", 1.0)];
    let plate = vibration.zip(number("width_m").zip(number("ei_b"))).filter(|((span, ei, mass), (width, ei_b))| *span > 0.0 && *ei > 0.0 && *mass > 0.0 && *width > 0.0 && *ei_b > 0.0).map(|(_, (width, ei_b))| Plate { width, ei_b, zeta: number("zeta").unwrap_or(0.01) });
    Geo { fasteners, b: b.max(0.02), h, len, spans, d, t1: mm(c, "t1").unwrap_or(0.06), t2: mm(c, "t2").or_else(|| mm(c, "t_steel")).unwrap_or(0.06), a1, a2, end, edge, notch: (h - h_ef).max(0.0), notch_x: mm(c, "x").unwrap_or(0.0), notch_len: mm(c, "notch_l").unwrap_or(0.0), t_req_min: number("t_req").unwrap_or(0.0), nx, ny, points, explicit: number("explicit").is_some_and(|v| v >= 0.5), profile, layers, vibration, faces, plate }
}

/// 🔩️ The key of the per-fastener colour field and the demand at which the ramp ends (a violation reads as the red end).
const SPACING_FIELD: &str = "spacing-demand";
const SPACING_MAX: f64 = 1.2;

/// 🔩️ The positions `(x, y)` in metres of the fasteners of a case, row by row: the explicit points when the case lists them, else the regular pattern of `nx` rows at a₂ and `ny` columns at a₁ from the end and edge distance.
pub fn fastener_positions(g: &Geo) -> Vec<(f64, f64)> {
    if g.points.is_empty() {
        (0..g.nx).flat_map(|j| (0..g.ny).map(move |i| (g.end + f64::from(i) * g.a1, g.edge + f64::from(j) * g.a2))).collect()
    } else {
        g.points.clone()
    }
}

/// 🔩️ True when the positions are the regular pattern of `n` columns and `rows` rows (not `explicit`), so that dragging a fastener rewrites a₁ or a₂.
pub fn regular(g: &Geo) -> bool {
    !g.explicit && (g.points.is_empty() || g.points.len() == (g.nx * g.ny) as usize)
}

/// 📐️ The minimum-spacing demand of every fastener of `positions`: the largest of required ÷ provided over the spacing a₁ to its neighbour along the grain, a₂ to its neighbour across, the end distance a₃,t and the edge distance a₄,c, with the minima of Tables 8.2 – 8.5 of the kernel (`None` for a case without a fastener kind). The end and edge distances are the pattern's own, mirrored at the far side.
pub fn spacing_demands(c: &CaseRow, g: &Geo, positions: &[(f64, f64)]) -> Option<Vec<f64>> {
    let raw = c.texts.iter().find(|t| t.id == "kind").map(|t| t.value.as_str())?;
    let family = raw.split('_').next().unwrap_or(raw);
    let kind = if matches!(raw, "nail" | "nail_predrilled" | "staple" | "screw_axial" | "bolt" | "dowel") { raw } else if family == "screw" { "nail" } else { family };
    let number = |key: &str| c.numbers.iter().find(|n| n.id == key).map(|n| n.value);
    let rho = c.texts.iter().find(|t| t.id == "class").and_then(|t| facts::properties(&t.value)).map_or(350.0, |p| p.rho_k);
    let table = facts::spacing_minima(kind, g.d * 1e3, number("alpha_deg").unwrap_or(0.0), rho);
    let required = |name: &str| table.iter().find(|r| r.0 == name).map(|r| r.1 / 1e3);
    let (min_x, max_x) = positions.iter().fold((f64::MAX, f64::MIN), |m, p| (m.0.min(p.0), m.1.max(p.0)));
    let (min_y, max_y) = positions.iter().fold((f64::MAX, f64::MIN), |m, p| (m.0.min(p.1), m.1.max(p.1)));
    let same = |a: f64, b: f64| (a - b).abs() < 1e-9;
    let nearest = |at: f64, others: Vec<f64>| others.into_iter().map(|o| (o - at).abs()).filter(|gap| *gap > 1e-9).fold(f64::MAX, f64::min);
    let ratio = |req: Option<f64>, provided: f64| match req {
        Some(req) if provided > 1e-9 && provided < f64::MAX => req / provided,
        _ => 0.0,
    };
    Some(
        positions
            .iter()
            .enumerate()
            .map(|(index, (x, y))| {
                let end = (x - min_x).min(max_x - x) + g.end;
                let edge = (y - min_y).min(max_y - y) + g.edge;
                let (a1, a2) = if g.explicit {
                    (super::schema::checks::timber::pairwise_demand(index, positions, required("a1"), required("a2")), 0.0)
                } else {
                    (ratio(required("a1"), nearest(*x, positions.iter().filter(|p| same(p.1, *y)).map(|p| p.0).collect())), ratio(required("a2"), nearest(*y, positions.iter().filter(|p| same(p.0, *x)).map(|p| p.1).collect())))
                };
                [a1, a2, ratio(required("a3t"), end), ratio(required("a4c"), edge)].into_iter().fold(0.0_f64, f64::max)
            })
            .collect(),
    )
}

/// 📏 Plan footprint `(length, width)` of a case.
pub fn extent(c: &CaseRow) -> (f64, f64) {
    let g = geo(c);
    if let Some(footprint) = special::extent(c, &g) {
        return footprint;
    }
    if g.fasteners {
        let (x0, y0) = g.points.first().copied().unwrap_or((0.0, 0.0));
        let (reach_x, reach_y) = g.points.iter().fold((0.0_f64, 0.0_f64), |m, p| (m.0.max(p.0 - x0), m.1.max(p.1 - y0)));
        if g.points.is_empty() { (2.0 * g.end + f64::from(g.ny - 1) * g.a1, 2.0 * g.edge + f64::from(g.nx - 1) * g.a2) } else { (2.0 * g.end + reach_x, 2.0 * g.edge + reach_y) }
    } else { (g.len, if matches!(g.profile, Profile::Straight) && g.layers.is_empty() { g.b } else { g.b.max(0.1) }) }
}

/// 🧱️ One drawn part of a case: geometry, appearance, the verification it is bound to, its handles, scene layer and optional mode overlay.
struct Part<'a> {
    tag: &'a str,
    geometry: Geometry,
    appearance: Option<Appearance>,
    role: MaterialRole,
    binding: Option<UtilisationBinding>,
    name: LocalizedCopy,
    detail: LocalizedCopy,
    handles: Vec<String>,
    layer: &'static str,
    deformation: Option<DeformationOverlay>,
}

fn place(c: &CaseRow, part: Part<'_>, points: &[Vec3], out: &mut dyn Sink) -> EntityRef {
    let id = format!("{}:{}", c.id, part.tag);
    let kind = match &part.geometry { Geometry::Fastener(_) => EntityKind::Fastener, Geometry::Surface(_) => EntityKind::Surface, _ => EntityKind::Solid };
    let collection = out.home();
    out.push(Spec { kind, id: id.clone(), collection, geometry: part.geometry, appearance: part.appearance, role: part.role, binding: part.binding, name: part.name, detail: part.detail, layer: part.layer, handles: part.handles, deformation: part.deformation }, points);
    entity_ref(kind, &id, collection)
}

fn plain<'a>(tag: &'a str, geometry: Geometry, role: MaterialRole, binding: &Option<UtilisationBinding>, name: LocalizedCopy, detail: LocalizedCopy, handles: Vec<String>) -> Part<'a> {
    Part { tag, geometry, appearance: None, role, binding: binding.clone(), name, detail, handles, layer: LAYER_CONNECTIONS, deformation: None }
}

fn dims(w: f64, d: f64, t: f64) -> LocalizedCopy {
    let text = format!("{:.0} × {:.0} × {:.0} mm", w * 1e3, d * 1e3, t * 1e3);
    LocalizedCopy::new(text.clone(), text)
}

/// 📐 Side elevation `(x, up)` of a tapered or curved-pitched beam, the polygon of its apex zone and (apex only) the arc centre, the soffit apex and the top end of the arc.
pub fn elevation(g: &Geo) -> Option<(Vec<Vec2>, Vec<Vec2>, Vec<Vec2>)> {
    match g.profile {
        Profile::Straight => None,
        Profile::Tapered { alpha } => Some((vec![Vec2::new(0.0, 0.0), Vec2::new(g.len, 0.0), Vec2::new(g.len, g.h + g.len * alpha.tan()), Vec2::new(0.0, g.h)], Vec::new(), Vec::new())),
        Profile::Apex { alpha, r_in, h_ap } => {
            let (steps, leg) = (12, 2.5 * h_ap.max(0.05));
            let inner = |theta: f64, r: f64| Vec2::new(r * theta.sin(), r * theta.cos() - r_in);
            let arc = |r: f64| (0..=steps).map(|i| inner(-alpha + 2.0 * alpha * f64::from(i) / f64::from(steps), r)).collect::<Vec<_>>();
            let (lower, upper) = (arc(r_in), arc(r_in + h_ap));
            let (dl, dr) = (Vec2::new(alpha.cos(), alpha.sin()), Vec2::new(alpha.cos(), -alpha.sin()));
            let (l0, l1, r0, r1) = (&lower[0], &upper[0], &lower[steps as usize], &upper[steps as usize]);
            let mut ring = vec![Vec2::new(l0.x - leg * dl.x, l0.y - leg * dl.y)];
            ring.extend(lower.iter().cloned());
            ring.push(Vec2::new(r0.x + leg * dr.x, r0.y + leg * dr.y));
            ring.push(Vec2::new(r1.x + leg * dr.x, r1.y + leg * dr.y));
            ring.extend(upper.iter().rev().cloned());
            ring.push(Vec2::new(l1.x - leg * dl.x, l1.y - leg * dl.y));
            let (min_x, min_y) = ring.iter().fold((f64::MAX, f64::MAX), |m, p| (m.0.min(p.x), m.1.min(p.y)));
            let shift = |p: &Vec2| Vec2::new(p.x - min_x, p.y - min_y);
            let zone: Vec<Vec2> = lower.iter().chain(upper.iter().rev()).map(shift).collect();
            let anchors = vec![shift(&Vec2::new(0.0, -r_in)), shift(&lower[steps as usize / 2]), shift(&upper[steps as usize])];
            Some((ring.iter().map(shift).collect(), zone, anchors))
        }
    }
}

fn elevation_prism(at: &dyn Fn(f64, f64, f64) -> Vec3, polygon: Vec<Vec2>, width: f64) -> Geometry {
    Geometry::Solid(SolidGeometry { shape: SolidShape::Prism { frame: Frame { origin: at(0.0, width, 0.0), x: Vec3::X, y: Vec3::Z, z: Vec3::new(0.0, -1.0, 0.0) }, base: Polygon2::solid(polygon), height: width } })
}

fn section_prism(at: &dyn Fn(f64, f64, f64) -> Vec3, g: &Geo, base: Polygon2) -> Geometry {
    Geometry::Solid(SolidGeometry { shape: SolidShape::Prism { frame: Frame { origin: at(0.0, g.b / 2.0, g.h / 2.0), x: Vec3::Y, y: Vec3::Z, z: Vec3::X }, base, height: g.len } })
}

#[path = "🧩️special/🦀️.rs"]
pub mod special;

pub fn product_of(c: &CaseRow) -> super::model::TimberProduct {
    c.texts.iter().find(|t| t.id == "class").and_then(|t| facts::properties(&t.value)).map_or(super::model::TimberProduct::Glulam, |p| p.product)
}

/// 🧠️ The first vertical mode of a simply supported strip, memoised per (span, stiffness, mass) so that drag frames and re-evaluations reuse the eigen-solution of unchanged inputs.
pub fn cached_mode(span: f64, ei: f64, mass: f64) -> Option<modes::Vibration> {
    use std::collections::BTreeMap;
    use std::sync::{Mutex, OnceLock};
    static MEMO: OnceLock<Mutex<BTreeMap<[u64; 3], Option<modes::Vibration>>>> = OnceLock::new();
    let key = [span.to_bits(), ei.to_bits(), mass.to_bits()];
    let mut memo = MEMO.get_or_init(Default::default).lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    if memo.len() > 256 {
        memo.clear();
    }
    memo.entry(key).or_insert_with(|| modes::first_vertical(super::model::SupportType::SimplySupported, span, ei, mass)).clone()
}

fn mode_overlay(g: &Geo) -> Option<DeformationOverlay> {
    let (span, ei, mass) = g.vibration?;
    let mode = cached_mode(span, ei, mass)?;
    let samples = mode.shape.iter().map(|&(t, u)| DisplacementSample { at: t, delta: Vec3::new(0.0, 0.0, 0.08 * span * u) }).collect();
    Some(DeformationOverlay { surface: Vec::new(), motion: Motion::Harmonic, load_factor: None, kind: DeformationKind::Mode, index: 1, frequency: Some(Quantity::new(QuantityKind::Frequency, mode.frequency)), participation: 1.0, scale: 1.0, samples })
}

/// 🌳️ The CLT layup as a laminate: parallel layers carry the grain along the span, cross layers (rolling shear) lie at a right angle.
fn laminate(g: &Geo, at: &dyn Fn(f64, f64, f64) -> Vec3) -> Geometry {
    let layers = g.layers.iter().enumerate().map(|(i, (t, parallel))| GrainLayer { thickness: *t, role: if *parallel { MaterialRole::Timber } else { MaterialRole::Neutral }, grain: if *parallel { 0.0 } else { std::f64::consts::FRAC_PI_2 }, label: loc(&format!("Layer {} {}", i + 1, if *parallel { "(longitudinal)" } else { "(cross, rolling shear)" }), &format!("Lage {} {}", i + 1, if *parallel { "(längs)" } else { "(quer, Rollschub)" })) }).collect();
    Geometry::Solid(SolidGeometry { shape: SolidShape::Laminate { frame: Frame { origin: at(g.len / 2.0, g.b / 2.0, 0.0), x: Vec3::X, y: Vec3::Y, z: Vec3::Z }, footprint: rect(g.len, g.b), layers } })
}

/// 🗂️ The drawing of one case row standing at `o`: the engine of the typed entities that are drawn as the case row they stand for (`out.home` names their collection).
pub fn draw_row(input: &Input, c: &CaseRow, o: &Vec3, tag: &dyn Fn(&[&str]) -> Vec<String>, out: &mut dyn Sink) {
    {
        let g = geo(c);
        let binding = binding_of(input.report, &c.id, "");
        let at = |x: f64, y: f64, z: f64| v3(o.x + x, o.y + y, o.z + z);
        let mut children = Vec::new();
        let mut top = g.h;
        if let Some((parts, height)) = special::build(c, &g, &at, &binding, tag, input, out) {
            children = parts;
            top = height;
        } else if g.fasteners {
            let (length, width) = extent(c);
            let mut z = 0.0;
            for (thickness, key) in [(g.t1, "t1"), (g.t2, "t2")] {
                children.push(place(c, plain(&format!("layer-{key}"), Geometry::Solid(SolidGeometry { shape: SolidShape::Box { frame: Frame { origin: at(length / 2.0, width / 2.0, z), x: Vec3::X, y: Vec3::Y, z: Vec3::Z }, size: Vec3::new(length, width, thickness) } }), MaterialRole::Timber, &binding, loc(&format!("Timber layer {key} of {}", c.label_en), &format!("Holzlage {key} von {}", c.label_de)), dims(length, width, thickness), tag(&[key])), &[at(0.0, 0.0, z), at(length, width, z + thickness)], out));
                z += thickness;
            }
            top = z;
            let positions = fastener_positions(&g);
            let (from, to) = (at(g.end, g.edge, 0.0), at(g.end, g.edge, z));
            let (x0, y0) = positions.first().copied().unwrap_or((g.end, g.edge));
            let pattern = Pattern::Points { offsets: positions.iter().map(|(x, y)| Vec3::new(0.0, *y - y0, -(*x - x0))).collect() };
            let demands = spacing_demands(c, &g, &positions);
            let worst = demands.as_ref().map_or(0.0, |d| d.iter().copied().fold(0.0_f64, f64::max));
            let verdict = |en: bool| match (&demands, worst > 1.0) {
                (None, _) => String::new(),
                (Some(_), true) => if en { format!(", minimum spacing violated (demand {worst:.2})") } else { format!(", Mindestabstand unterschritten (Ausnutzung {worst:.2})") },
                (Some(_), false) => if en { format!(", minimum spacings kept (demand {worst:.2})") } else { format!(", Mindestabstände eingehalten (Ausnutzung {worst:.2})") },
            };
            let head = format!("{} fasteners ⌀{:.1} mm, a₁ {:.0} mm, a₂ {:.0} mm, a₃ {:.0} mm, a₄ {:.0} mm", positions.len(), g.d * 1e3, g.a1 * 1e3, g.a2 * 1e3, g.end * 1e3, g.edge * 1e3);
            let head_de = format!("{} Verbindungsmittel ⌀{:.1} mm, a₁ {:.0} mm, a₂ {:.0} mm, a₃ {:.0} mm, a₄ {:.0} mm", positions.len(), g.d * 1e3, g.a1 * 1e3, g.a2 * 1e3, g.end * 1e3, g.edge * 1e3);
            let grips: Vec<String> = ["d", "a1", "a2", "a3t", "a4t", "n", "rows"].into_iter().map(String::from).chain((1..g.ny).filter(|_| regular(&g)).map(|i| format!("col{i}"))).chain((1..g.nx).filter(|_| regular(&g)).map(|j| format!("row{j}"))).collect();
            let mut fasteners = plain("fasteners", Geometry::Fastener(FastenerGeometry { kind: FastenerKind::Dowel, axis: Segment3 { from: from.clone(), to: to.clone() }, diameter: g.d, head: HeadShape::None, pattern: Some(pattern), hole_clearance: None }), MaterialRole::Bolt, &binding, loc(&format!("Fasteners of {}", c.label_en), &format!("Verbindungsmittel von {}", c.label_de)), LocalizedCopy::new(format!("{head}{}", verdict(true)), format!("{head_de}{}", verdict(false))), tag(&grips.iter().map(String::as_str).collect::<Vec<_>>()));
            if let Some(values) = demands {
                fasteners.appearance = Some(Appearance { role: MaterialRole::Bolt, channel: ColourChannel::Fields { active: SPACING_FIELD.into(), fields: vec![ScalarField { id: SPACING_FIELD.into(), unit: QuantityKind::Dimensionless, samples: FieldSamples::Instance { values } }] }, style: StyleState::Normal, opacity: 1.0, hatch: None, deformation: None });
                out.legend(Legend { id: format!("spacing:{}", c.id), title: loc("Minimum spacing demand per fastener (required ÷ provided)", "Mindestabstandsausnutzung je Verbindungsmittel (erforderlich ÷ vorhanden)"), scale: LegendScale::Scalar { field_id: SPACING_FIELD.into(), map: ColourMapRef { id: "failure".into(), min: 0.0, max: SPACING_MAX }, unit: QuantityKind::Dimensionless } });
            }
            children.push(place(c, fasteners, &[from, to], out));
        } else if !g.layers.is_empty() {
            let total: f64 = g.layers.iter().map(|l| l.0).sum();
            top = total;
            children.push(place(c, plain("laminate", laminate(&g, &at), MaterialRole::Timber, &binding, loc(&format!("CLT layup of {}", c.label_en), &format!("BSP-Aufbau von {}", c.label_de)), dims(g.len, g.b, total), tag(&["l", "t1", "t2", "t3", "t4", "t5", "t6", "t7", "t8", "t9"])), &[at(0.0, 0.0, 0.0), at(g.len, g.b, total)], out));
            children.extend(special::clt_char(c, &g, &at, input, tag, out));
            let anchor = at(g.len / 2.0, 0.0, total + 0.05);
            out.annotate(Annotation::Label { id: format!("{}:rolling-shear", c.id), anchor: anchor.clone(), text: loc("Cross layers carry rolling shear", "Querlagen tragen den Rollschub"), tone: tone_of(&binding_of(input.report, &c.id, "en1995.t-97.rolling_shear")) }, &anchor);
        } else {
            let body_handles = tag(&["b", "h", "l", "h_ap", "alpha_deg", "alpha_ap_deg", "r_in", "t_lam"]);
            let (geometry, extent_points) = match elevation(&g) {
                Some((ring, _, _)) => {
                    let height = ring.iter().fold(0.0_f64, |m, p| m.max(p.y));
                    top = height;
                    (elevation_prism(&at, ring, g.b), vec![at(0.0, 0.0, 0.0), at(g.len.max(2.0 * g.h), g.b, height)])
                }
                None => (Geometry::Solid(SolidGeometry { shape: SolidShape::Box { frame: Frame { origin: at(g.len / 2.0, g.b / 2.0, 0.0), x: Vec3::X, y: Vec3::Y, z: Vec3::Z }, size: Vec3::new(g.len, g.b, g.h) } }), vec![at(0.0, 0.0, 0.0), at(g.len, g.b, g.h)]),
            };
            let mut body = plain("body", geometry, MaterialRole::Timber, &binding, loc(&format!("Timber of {}", c.label_en), &format!("Holz von {}", c.label_de)), dims(g.len, g.b, g.h), body_handles);
            body.deformation = if g.plate.is_some() { None } else { mode_overlay(&g) };
            if let Some(overlay) = &body.deformation {
                let hz = overlay.frequency.as_ref().map_or(0.0, |q| q.value);
                body.detail = LocalizedCopy::new(format!("first vertical mode f₁ = {hz:.2} Hz"), format!("erste vertikale Eigenform f₁ = {hz:.2} Hz"));
            }
            children.push(place(c, body, &extent_points, out));
            children.extend(special::floor_plate(c, &g, &at, &binding, out));
            children.extend(special::crowd(c, &g, &at, &binding, out));
            if let Some((_, zone, _)) = elevation(&g).filter(|(_, z, _)| !z.is_empty()) {
                children.push(place(c, Part { appearance: Some(tinted((230, 150, 40), 0.6, StyleState::Normal)), ..plain("apex", elevation_prism(&|x, y, z| at(x, y, z + 0.001), zone, g.b * 1.02), MaterialRole::Neutral, &binding_of(input.report, &c.id, "en1995.t-31"), loc(&format!("Apex zone of {} (k_l, k_p, σ_t,90)", c.label_en), &format!("Firstbereich von {} (k_l, k_p, σ_t,90)", c.label_de)), loc("curved apex zone with the stress factors k_l and k_p", "gekrümmter Firstbereich mit den Beiwerten k_l und k_p"), tag(&["h_ap"])) }, &[at(0.0, 0.0, 0.0), at(g.len, g.b, top)], out));
            }
            if g.notch > 1e-6 {
                let reach = g.reach();
                children.push(place(c, Part { appearance: Some(tinted((250, 250, 250), 0.9, StyleState::Normal)), ..plain("notch", Geometry::Solid(SolidGeometry { shape: SolidShape::Box { frame: Frame { origin: at(reach / 2.0, g.b / 2.0, g.h - g.notch), x: Vec3::X, y: Vec3::Y, z: Vec3::Z }, size: Vec3::new(reach, g.b * 1.02, g.notch) } }), MaterialRole::Neutral, &binding_of(input.report, &c.id, "en1995.t-45"), loc(&format!("Notch of {}", c.label_en), &format!("Ausklinkung von {}", c.label_de)), loc(&format!("notch depth {:.0} mm", g.notch * 1e3), &format!("Ausklinkungstiefe {:.0} mm", g.notch * 1e3)), tag(&["h_ef", "x", "notch_l", "incl", "n_rods"])) }, &[at(0.0, 0.0, g.h - g.notch), at(reach, g.b, g.h)], out));
            }
            if g.t_req_min > 0.0 {
                let product = product_of(c);
                let full = Span2::full(g.b, g.h);
                let fire_binding = binding_of(input.report, &c.id, special::fire_prefix(c));
                let extent_points = [at(0.0, 0.0, 0.0), at(g.len, g.b, g.h)];
                for (minutes, layer) in FIRE_MINUTES.iter().zip(FIRE_LAYERS) {
                    let fire = facts::fire_after(input.document.annex, product, g.b, g.h, f64::from(*minutes) * 60.0);
                    let front = FRONT_THICKNESS.min(fire.d_char / 2.0);
                    let (edge, behind) = (full.inset(fire.d_char - front, g.faces), full.inset(fire.d_char, g.faces));
                    let kept = (behind.x1 - behind.x0, behind.y1 - behind.y0);
                    let char_text = loc(&format!("d_char {:.1} mm, residual {:.0} × {:.0} mm", fire.d_char * 1e3, kept.0 * 1e3, kept.1 * 1e3), &format!("d_char {:.1} mm, Rest {:.0} × {:.0} mm", fire.d_char * 1e3, kept.0 * 1e3, kept.1 * 1e3));
                    children.push(place(c, Part { appearance: Some(tinted((60, 40, 30), 0.5, StyleState::Normal)), layer, ..plain(&format!("char@{minutes}"), section_prism(&at, &g, band(full, edge, g.faces)), MaterialRole::Neutral, &fire_binding, loc(&format!("Char layer after {minutes} min of {}", c.label_en), &format!("Abbrandschicht nach {minutes} min von {}", c.label_de)), char_text, tag(&["t_req"])) }, &extent_points, out));
                    children.push(place(c, Part { appearance: Some(tinted(FRONT_RGB, 1.0, StyleState::Normal)), layer, ..plain(&format!("front@{minutes}"), section_prism(&at, &g, band(edge, behind, g.faces)), MaterialRole::Neutral, &fire_binding, loc(&format!("Char front after {minutes} min of {}", c.label_en), &format!("Abbrandfront nach {minutes} min von {}", c.label_de)), loc(&format!("front at {:.1} mm", fire.d_char * 1e3), &format!("Front bei {:.1} mm", fire.d_char * 1e3)), Vec::new()) }, &extent_points, out));
                }
                let fire = facts::fire_after(input.document.annex, product, g.b, g.h, g.t_req_min * 60.0);
                let (depth, word_en, word_de) = if c.entry_id == "T-106" { (fire.d_char, "d_char,n", "d_char,n") } else { (fire.d_ef, "d_ef", "d_ef") };
                let left = full.inset(depth, g.faces);
                children.push(place(c, Part { appearance: Some(tinted((90, 60, 40), 0.75, StyleState::Normal)), ..plain("residual", section_prism(&at, &g, left.solid()), MaterialRole::Neutral, &fire_binding, loc(&format!("Residual section of {} after {:.0} min", c.label_en, g.t_req_min), &format!("Restquerschnitt von {} nach {:.0} min", c.label_de, g.t_req_min)), loc(&format!("{word_en} {:.1} mm, residual {:.0} × {:.0} mm", depth * 1e3, (left.x1 - left.x0) * 1e3, (left.y1 - left.y0) * 1e3), &format!("{word_de} {:.1} mm, Rest {:.0} × {:.0} mm", depth * 1e3, (left.x1 - left.x0) * 1e3, (left.y1 - left.y0) * 1e3)), tag(&["t_req"])) }, &extent_points, out));
            }
        }
        out.push(
            Spec { kind: EntityKind::Group, id: c.id.clone(), collection: out.home(), geometry: Geometry::Group { members: children }, appearance: None, role: MaterialRole::Timber, binding: binding.clone(), name: loc(&c.label_en, &c.label_de), detail: loc(&format!("catalogue case {}", c.entry_id), &format!("Katalogfall {}", c.entry_id)), layer: LAYER_MEMBERS, handles: Vec::new(), deformation: None },
            &[o.clone()],
        );
        let label_at = at(0.0, -0.1, top + 0.1);
        out.annotate(Annotation::Label { id: format!("{}:entry", c.id), anchor: label_at.clone(), text: loc(&format!("{} {}", c.entry_id, c.label_en), &format!("{} {}", c.entry_id, c.label_de)), tone: tone_of(&binding) }, &label_at);
    }
}
