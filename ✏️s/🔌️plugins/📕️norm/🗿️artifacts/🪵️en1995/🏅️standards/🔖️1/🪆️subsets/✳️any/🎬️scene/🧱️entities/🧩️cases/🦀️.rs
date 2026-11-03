//! 🧩️ Case extras: the data the EN 1995 catalogue cases hand to the shared forms (`forms::draw_with`) because their picture needs the timber kernel: the char and front bands of the fire sections T-101, T-102, T-106 and of the CLT panel T-107, the first mode of the floors and footbridges T-63 to T-66, T-120 and T-124 (the ghost of a strip or the heat-mapped plate), the stacked parts of the jointed beam T-90, the spaced column T-93, the diaphragm T-94, the truss T-95, the bracing T-96, the Miner tower T-121 and the stress-laminated deck T-122, and the crowd of T-120. The forms draw them; nothing here knows a scene entity. Typed entities are drawn by [`super::rows`]. See [`super`].

use super::facts;
use super::kit::{band, Span2, FIRE_LAYERS, FIRE_MINUTES, FRONT_RGB, FRONT_THICKNESS, LAYER_FIRE, LAYER_MEMBERS};
use super::model::{CaseRow, TrussKind};
use super::norm;
use super::rows::{cached_mode, product_of, special::jointed_parts};
use super::scene::projection_kit::design_case::forms::{self, Assembly, Char, CharPart, Extras, Follow, Grip, GripKind, Mode, ModeGrid, Note, PinKind, Pins, Size, Sub, SubShape};
use super::scene::{MaterialRole, Polygon2, QuantityKind};
use super::schema::checks::timber as t;

type P = [f64; 3];
type Text = (String, String);

const GIRDER: (f64, f64, f64) = (2.0, 0.3, 0.25);
const DAMAGE_UNIT: f64 = 0.6;
const SHEATHING: f64 = 0.022;
const FLOOR_COLUMNS: u32 = 25;
const FLOOR_ROWS: u32 = 17;
const PLATE_AMPLITUDE: f64 = 0.04;
const STRIP_AMPLITUDE: f64 = 0.08;
const CHAR_RGB: (u8, u8, u8) = (60, 40, 30);
const X: P = [1.0, 0.0, 0.0];
const Y: P = [0.0, 1.0, 0.0];
const Z: P = [0.0, 0.0, 1.0];
const CROWD_MAX: f64 = 2.5;
const CROWD_PER_METRE: f64 = 0.5;

fn num(c: &CaseRow, key: &str) -> Option<f64> {
    c.numbers.iter().find(|n| n.id == key).map(|n| n.value)
}

fn mm(c: &CaseRow, key: &str) -> Option<f64> {
    num(c, key).map(|v| v / 1e3)
}

fn word<'a>(c: &'a CaseRow, key: &str) -> &'a str {
    c.texts.iter().find(|t| t.id == key).map_or("", |t| t.value.as_str())
}

fn same(text: String) -> Text {
    (text.clone(), text)
}

fn dims(w: f64, d: f64, t: f64) -> Text {
    same(format!("{:.0} × {:.0} × {:.0} mm", w * 1e3, d * 1e3, t * 1e3))
}

fn named(en: &str, de: &str, c: &CaseRow) -> Text {
    (format!("{en} of {}", c.label_en), format!("{de} von {}", c.label_de))
}

/// 🕹️ An extra grip of the kind `kind` on the input `key`.
fn grip(tag: &str, key: &str, factor: f64, kind: GripKind, axis: P, anchor: P) -> Grip {
    Grip { tag: tag.to_string(), key: key.to_string(), factor, kind, axis, anchor }
}

/// 📊️ A gauge grip: the input moves `k` stored units per metre of drag, quantised to `step`, with `s` within `limits` (metres) and `follow` inputs written in the same release.
fn gauge(tag: &str, key: &str, (k, step): (f64, f64), limits: (f64, f64), follow: Vec<Follow>, axis: P, anchor: P) -> Grip {
    grip(tag, key, 1.0, GripKind::Gauge { k, step, limits, follow }, axis, anchor)
}

/// 🧱️ A sub-form with the defaults of a timber part.
fn part(tag: &str, name: Text, detail: Text, role: MaterialRole, at: P, shape: SubShape) -> Sub {
    Sub { tag: tag.to_string(), name, detail, role, rgb: None, opacity: 1.0, at, axes: None, shape }
}

fn tint(mut sub: Sub, rgb: (u8, u8, u8), opacity: f64) -> Sub {
    sub.rgb = Some(rgb);
    sub.opacity = opacity;
    sub
}

/// 🧱️ A box standing on the plan point `(x, y)` at height `z`.
fn boxed(tag: &str, name: Text, role: MaterialRole, (x, y, z): (f64, f64, f64), size: P) -> Sub {
    part(tag, name, dims(size[0], size[1], size[2]), role, [x, y, z], SubShape::Box { size })
}

/// 🧱️ A box from `a` to `b` whose width runs along the unit vector `side` and whose depth along `(b - a) × side`.
fn beam(tag: &str, (a, b, side): (P, P, P), size: (f64, f64), role: MaterialRole, name: Text) -> Sub {
    let d = [0, 1, 2].map(|i| b[i] - a[i]);
    let len = d.iter().map(|v| v * v).sum::<f64>().sqrt().max(1e-6);
    let x = d.map(|v| v / len);
    let n = [x[1] * side[2] - x[2] * side[1], x[2] * side[0] - x[0] * side[2], x[0] * side[1] - x[1] * side[0]];
    let at = [0, 1, 2].map(|i| (a[i] + b[i]) / 2.0 - n[i] * size.1 / 2.0);
    Sub { axes: Some([x, side, n]), ..part(tag, name, dims(len, size.0, size.1), role, at, SubShape::Box { size: [len, size.0, size.1] }) }
}

/// 🧩️ The sub-forms of an assembly drawn from the corner of its footprint, before they are centred.
struct Draft {
    subs: Vec<Sub>,
    top: f64,
    footprint: (f64, f64),
    grips: Vec<Grip>,
    notes: Vec<Note>,
}

impl Draft {
    fn new(footprint: (f64, f64)) -> Self {
        Self { subs: Vec::new(), top: 0.0, footprint, grips: Vec::new(), notes: Vec::new() }
    }

    fn lift(&mut self, tag: &str, key: &str, anchor: P) {
        self.drag(tag, key, 1.0, GripKind::Scalar, Z, anchor);
    }

    fn drag(&mut self, tag: &str, key: &str, factor: f64, kind: GripKind, axis: P, anchor: P) {
        self.grips.push(grip(tag, key, factor, kind, axis, anchor));
    }

    fn count(&mut self, tag: &str, key: &str, range: (f64, f64), anchor: P) {
        self.drag(tag, key, 1.0, GripKind::Count { min: range.0, max: range.1 }, Z, anchor);
    }

    fn finish(self, mut extras: Extras) -> Extras {
        let (cx, cy) = (self.footprint.0 / 2.0, self.footprint.1 / 2.0);
        let shift = |p: P| [p[0] - cx, p[1] - cy, p[2]];
        extras.grips.extend(self.grips.into_iter().map(|g| Grip { anchor: shift(g.anchor), ..g }));
        extras.notes.extend(self.notes.into_iter().map(|n| Note { at: shift(n.at), ..n }));
        extras.assembly = Some(Assembly { half: (cx, cy), top: self.top, subs: self.subs.into_iter().map(|s| Sub { at: shift(s.at), ..s }).collect() });
        extras
    }
}

/// ✂️ The outline and holes of a polygon of the section as a char part.
fn char_part(tag: String, layer: &str, rgb: (u8, u8, u8), opacity: f64, polygon: &Polygon2, name: Text, detail: Text) -> CharPart {
    let ring = |points: &[super::scene::Vec2]| points.iter().map(|p| [p.x, p.y]).collect::<Vec<_>>();
    CharPart { tag, layer: layer.to_string(), rgb, opacity, outline: ring(&polygon.outer.points), holes: polygon.holes.iter().map(|h| ring(&h.points)).collect(), name, detail }
}

/// 🔥️ The exposed sides `[left, right, top, bottom]` of a section, from the case inputs.
fn faces(c: &CaseRow) -> [f64; 4] {
    let face = |key: &str, default: f64| num(c, key).unwrap_or(default).clamp(0.0, 1.0).round();
    [face("face_left", 1.0), face("face_right", 1.0), face("face_top", 0.0), face("face_bottom", 1.0)]
}

/// 🔥️ The char and front bands after 15 … 120 minutes and the residual section at `t_req` of a timber member `size.dy × size.h`.
fn fire(annex: norm::AnnexChoice, c: &CaseRow, size: Size) -> Option<Char> {
    let t_req = num(c, "t_req").filter(|t| *t > 0.0)?;
    let (product, faces, full) = (product_of(c), faces(c), Span2::full(size.dy, size.h));
    let mut parts = Vec::new();
    for (minutes, layer) in FIRE_MINUTES.iter().zip(FIRE_LAYERS) {
        let after = facts::fire_after(annex, product, size.dy, size.h, f64::from(*minutes) * 60.0);
        let (edge, behind) = (full.inset(after.d_char - FRONT_THICKNESS.min(after.d_char / 2.0), faces), full.inset(after.d_char, faces));
        let kept = ((behind.x1 - behind.x0) * 1e3, (behind.y1 - behind.y0) * 1e3);
        let text = (format!("d_char {:.1} mm, residual {:.0} × {:.0} mm", after.d_char * 1e3, kept.0, kept.1), format!("d_char {:.1} mm, Rest {:.0} × {:.0} mm", after.d_char * 1e3, kept.0, kept.1));
        parts.push(char_part(format!("char@{minutes}"), layer, CHAR_RGB, 0.5, &band(full, edge, faces), (format!("Char layer after {minutes} min"), format!("Abbrandschicht nach {minutes} min")), text));
        parts.push(char_part(format!("front@{minutes}"), layer, FRONT_RGB, 1.0, &band(edge, behind, faces), (format!("Char front after {minutes} min"), format!("Abbrandfront nach {minutes} min")), same(format!("{:.1} mm", after.d_char * 1e3))));
    }
    let after = facts::fire_after(annex, product, size.dy, size.h, t_req * 60.0);
    let (depth, word) = if c.entry_id == "T-106" { (after.d_char, "d_char,n") } else { (after.d_ef, "d_ef") };
    let left = full.inset(depth, faces);
    let kept = ((left.x1 - left.x0) * 1e3, (left.y1 - left.y0) * 1e3);
    let detail = (format!("{word} {:.1} mm, residual {:.0} × {:.0} mm", depth * 1e3, kept.0, kept.1), format!("{word} {:.1} mm, Rest {:.0} × {:.0} mm", depth * 1e3, kept.0, kept.1));
    parts.push(char_part("residual".into(), LAYER_MEMBERS, (90, 60, 40), 0.75, &left.solid(), (format!("Residual section after {t_req:.0} min"), format!("Restquerschnitt nach {t_req:.0} min")), detail));
    Some(Char { parts })
}

/// 🌳️ The char depth in metres of the CLT layup of `c` after `minutes`.
fn clt_depth(c: &CaseRow) -> impl Fn(f64) -> f64 {
    let thick: Vec<f64> = (1..=num(c, "n_layers").unwrap_or(0.0).min(9.0) as usize).filter_map(|i| num(c, &format!("t{i}"))).collect();
    let (beta, factor, protect, on) = (num(c, "beta_0").unwrap_or(0.65), num(c, "fall_off_factor").unwrap_or_else(|| norm::ndp_common("ndp.en1995.fire.clt-fall-off-factor")), num(c, "protect_mm").unwrap_or_else(|| norm::ndp_common("ndp.en1995.fire.clt-fall-off-protect")), num(c, "fall_off").unwrap_or(1.0) > 0.5);
    move |minutes| t::clt_char_depth(&thick, beta, minutes, on, factor, protect).0 / 1e3
}

/// 🌳️ The char of a CLT layup after 15 … 120 minutes and at `t_req` as slabs from the exposed bottom face of a panel `size.dy` wide.
fn clt_char(c: &CaseRow, size: Size) -> Option<Char> {
    let t_req = num(c, "t_req").filter(|t| *t > 0.0)?;
    let char_depth = clt_depth(c);
    let steps = FIRE_MINUTES.iter().zip(FIRE_LAYERS).map(|(m, layer)| (format!("char@{m}"), f64::from(*m), layer)).chain([("residual".to_string(), t_req, LAYER_FIRE)]);
    let parts = steps
        .filter_map(|(tag, minutes, layer)| {
            let depth = char_depth(minutes);
            let (w, y0) = (size.dy / 2.0, -size.h / 2.0);
            let slab = Span2 { x0: -w, x1: w, y0, y1: y0 + depth };
            (depth > 0.0).then(|| char_part(tag, layer, CHAR_RGB, 0.55, &slab.solid(), (format!("Char after {minutes:.0} min"), format!("Abbrand nach {minutes:.0} min")), same(format!("d_char {:.1} mm after {minutes:.0} min", depth * 1e3))))
        })
        .collect();
    Some(Char { parts })
}

/// 🛡️ The char behind the cladding of T-103 after 15 … 120 minutes as slabs on the unexposed face of a board `size.dy` wide and `size.h` thick: nothing before the start of charring t_ch, then the depth of [`t::ProtectedProbe::depth_at`] in the timber above the board.
fn protected_char(annex: norm::AnnexChoice, c: &CaseRow, size: Size) -> Option<Char> {
    let probe = t::protected_probe(&t::case_input(annex, &norm::design_case::DesignCase::from(c))).ok()?;
    let (w, y0) = (size.dy / 2.0, size.h / 2.0);
    let steps = FIRE_MINUTES.iter().zip(FIRE_LAYERS).map(|(m, layer)| (format!("char@{m}"), f64::from(*m), layer)).chain([("residual".to_string(), probe.t_min, LAYER_FIRE)]);
    let parts = steps
        .filter_map(|(tag, minutes, layer)| {
            let depth = probe.depth_at(minutes) / 1e3;
            let slab = Span2 { x0: -w, x1: w, y0, y1: y0 + depth };
            (depth > 0.0).then(|| char_part(tag, layer, CHAR_RGB, 0.55, &slab.solid(), (format!("Char behind the cladding after {minutes:.0} min"), format!("Abbrand hinter der Bekleidung nach {minutes:.0} min")), same(format!("d_char {:.1} mm after {minutes:.0} min (t_ch {:.1} min, t_f {:.1} min)", depth * 1e3, probe.t_ch, probe.t_f))))
        })
        .collect();
    Some(Char { parts })
}

/// 🔥️ The fire resistance time `t_req` as a gauge on the char front of the first exposed face (bottom, left, right, top) and the stepper of each exposed-face flag, for a section `size` whose body starts at the height `base`; `depth` is the char depth in metres after a number of minutes.
fn fire_grips(c: &CaseRow, size: Size, base: f64, faces: Option<[f64; 4]>, depth: &dyn Fn(f64) -> f64) -> Vec<Grip> {
    let Some(t_req) = num(c, "t_req").filter(|t| *t > 0.0) else { return Vec::new() };
    let (now, rate) = (depth(t_req), depth(t_req + 1.0) - depth(t_req));
    let (dy, h) = (size.dy, size.h);
    let sides: [(P, P); 4] = [([0.0, 1.0, 0.0], [0.0, -dy / 2.0 + now, base + h / 2.0]), ([0.0, -1.0, 0.0], [0.0, dy / 2.0 - now, base + h / 2.0]), ([0.0, 0.0, -1.0], [0.0, 0.0, base + h - now]), (Z, [0.0, 0.0, base + now])];
    let exposed = faces.map_or(3, |f| [3, 0, 1, 2].into_iter().find(|i| f[*i] > 0.5).unwrap_or(3));
    let mut out = Vec::new();
    if rate > 1e-9 {
        let (axis, anchor) = sides[exposed];
        out.push(gauge("t_req", "t_req", (1.0 / rate, 5.0), (-now, 0.5 * dy.min(h) - now - 0.002), Vec::new(), axis, anchor));
    }
    if faces.is_some() {
        for (tag, (_, anchor)) in ["face_left", "face_right", "face_top", "face_bottom"].into_iter().zip(sides) {
            out.push(grip(tag, tag, 1.0, GripKind::Count { min: 0.0, max: 1.0 }, Z, anchor));
        }
    }
    out
}

/// 🌊️ The first vertical mode of a floor or footbridge: the heat-mapped plate when it has a width and a transverse stiffness, the ghost of the strip otherwise.
fn mode(c: &CaseRow, size: Size) -> (Option<Mode>, Vec<Note>) {
    let (Some(span), Some(ei), Some(mass)) = (num(c, "span_m"), num(c, "ei_l").or_else(|| num(c, "ei_nm2")), num(c, "mass_kg_m2").or_else(|| num(c, "mass_kg_m"))) else { return (None, Vec::new()) };
    let plate = num(c, "width_m").zip(num(c, "ei_b")).filter(|(width, ei_b)| span > 0.0 && ei > 0.0 && mass > 0.0 && *width > 0.0 && *ei_b > 0.0);
    let Some((width, ei_b)) = plate else {
        let Some(found) = cached_mode(span, ei, mass) else { return (None, Vec::new()) };
        let detail = (format!("first vertical mode f₁ = {:.2} Hz", found.frequency), format!("erste vertikale Eigenform f₁ = {:.2} Hz", found.frequency));
        return (Some(Mode { frequency: found.frequency, amplitude: STRIP_AMPLITUDE * span, shape: found.shape.clone(), grid: None, detail }), Vec::new());
    };
    let f1 = t::floor_plate_f1(span, width, ei, ei_b, mass);
    let peak = t::floor_plate_peak_acceleration(span, width, ei, ei_b, mass);
    let values = (0..FLOOR_ROWS).flat_map(|j| (0..FLOOR_COLUMNS).map(move |i| (i, j))).map(|(i, j)| t::floor_plate_mode(span * f64::from(i) / f64::from(FLOOR_COLUMNS - 1), width * f64::from(j) / f64::from(FLOOR_ROWS - 1), span, width)).collect();
    let detail = (format!("first mode f₁ = {f1:.2} Hz, peak acceleration {peak:.3} m/s² per N·s at the antinode"), format!("erste Eigenform f₁ = {f1:.2} Hz, größte Beschleunigung {peak:.3} m/s² je N·s im Schwingungsbauch"));
    let grid = ModeGrid {
        columns: FLOOR_COLUMNS,
        rows: FLOOR_ROWS,
        values,
        peak,
        unit: QuantityKind::Acceleration,
        title: ("Peak acceleration per unit impulse (m/s² per N·s)".into(), "Größte Beschleunigung je Einheitsimpuls (m/s² je N·s)".into()),
        name: named("Floor mode and acceleration", "Deckenschwingform und Beschleunigung", c),
    };
    let note = Note { tag: "floor-mode".into(), at: [0.0, 0.0, forms::SUPPORT_HEIGHT + size.h + 0.25], text: (format!("f₁ = {f1:.2} Hz, a ≤ {peak:.3} m/s² per N·s"), format!("f₁ = {f1:.2} Hz, a ≤ {peak:.3} m/s² je N·s")) };
    (Some(Mode { frequency: f1, amplitude: PLATE_AMPLITUDE * span, shape: Vec::new(), grid: Some(grid), detail }), vec![note])
}

/// 🚶️ The crowd of a footbridge case: standing persons on a grid over the deck, one pin pattern.
fn crowd(c: &CaseRow, size: Size) -> Option<Sub> {
    let density = num(c, "crowd_density").filter(|d| *d > 0.0)?;
    let (len, width) = (size.dx, size.dy);
    let people = (density * len * width).round().clamp(1.0, 120.0);
    let nx = (people * width / len).sqrt().ceil().clamp(1.0, 8.0) as u32;
    let ny = (people / f64::from(nx)).ceil().clamp(1.0, 60.0) as u32;
    let (sx, sy) = (width / f64::from(nx), len / f64::from(ny));
    let text = same(format!("{} persons, {density:.2} persons/m²", nx * ny));
    let at = [len / 2.0 - sy / 2.0, sx / 2.0 - width / 2.0, forms::SUPPORT_HEIGHT + size.h];
    Some(tint(part("crowd", named("Crowd", "Menschenmenge", c), text, MaterialRole::Neutral, at, SubShape::Pins { kind: PinKind::Dowel, diameter: 0.35, to: [0.0, 0.0, 1.7], pattern: Pins::Grid { nx, ny, sx, sy } }), (200, 160, 120), 1.0))
}

struct Slab {
    index: u8,
    w: f64,
    h: f64,
    s: Option<f64>,
    e: Option<f64>,
}

impl Slab {
    fn role(&self) -> MaterialRole {
        if self.e.is_some_and(t::is_mineral_part) { MaterialRole::Concrete } else { MaterialRole::Timber }
    }
}

fn slabs(c: &CaseRow) -> Vec<Slab> {
    [3_u8, 2, 1]
        .iter()
        .filter_map(|&i| match (num(c, &format!("a{i}")), num(c, &format!("h{i}"))) {
            (Some(a), Some(h)) if a > 0.0 && h > 0.0 => Some(Slab { index: i, w: a / h / 1e3, h: h / 1e3, s: mm(c, &format!("s{i}")).filter(|s| *s > 0.0), e: num(c, &format!("e{i}")) }),
            _ => None,
        })
        .collect()
}

/// 🧱️ The mechanically jointed beam T-90: the parts stacked in the order of the γ-method with a fastener row at every interface that has a spacing, and the γ label.
fn jointed(c: &CaseRow, size: Size) -> Option<Extras> {
    let stack = slabs(c);
    if stack.len() < 2 {
        return None;
    }
    let (len, wide) = (size.dx, stack.iter().fold(0.0_f64, |m, s| m.max(s.w)));
    let mut draft = Draft::new((len, wide));
    let (mut z, mut interfaces) = (0.0, Vec::new());
    for (n, s) in stack.iter().enumerate() {
        draft.subs.push(boxed(&format!("part{}", s.index), named(&format!("Part {}", s.index), &format!("Teil {}", s.index), c), s.role(), (len / 2.0, wide / 2.0, z), [len, s.w, s.h]));
        draft.lift(&format!("h{}", s.index), &format!("h{}", s.index), [len / 2.0, wide / 2.0, z + s.h]);
        let (h_mm, i_unit) = (s.h * 1e3, s.h.powi(3) * 1e9 / 12.0);
        let follow = Follow { key: format!("i{}", s.index), base: s.w * 1e3 * i_unit, k: 2e3 * i_unit, step: 1e4 };
        draft.grips.push(gauge(&format!("a{}", s.index), &format!("a{}", s.index), (2e3 * h_mm, 100.0), ((0.02 - s.w) / 2.0, (0.8 - s.w) / 2.0), vec![follow], Y, [len / 2.0, wide / 2.0 + s.w / 2.0, z + s.h / 2.0]));
        if let Some(spacing) = s.s {
            draft.drag(&format!("s{}", s.index), &format!("s{}", s.index), 2.0, GripKind::Spacing, X, [spacing / 2.0, wide / 2.0, z + s.h]);
        }
        z += s.h;
        if let Some(next) = stack.get(n + 1) {
            let owner = if s.index == 2 { next } else { s };
            if let Some(spacing) = owner.s {
                interfaces.push((z, s.h.min(next.h), owner.index, spacing));
            }
        }
    }
    for (zi, depth, owner, spacing) in interfaces {
        let xs: Vec<f64> = (0..40).map(|k| spacing / 2.0 + f64::from(k) * spacing).take_while(|x| *x <= len).collect();
        let Some(&first) = xs.first() else { continue };
        let text = same(format!("{} fasteners every {:.0} mm", xs.len(), spacing * 1e3));
        let offsets = xs.iter().map(|x| [0.0, 0.0, -(x - first)]).collect();
        draft.subs.push(part(&format!("fasteners-{owner}"), named(&format!("Fasteners of part {owner}"), &format!("Verbindungsmittel von Teil {owner}"), c), text, MaterialRole::Bolt, [first, wide / 2.0, zi - 0.4 * depth], SubShape::Pins { kind: PinKind::Nail, diameter: 0.006, to: [0.0, 0.0, 0.8 * depth], pattern: Pins::Points(offsets) }));
    }
    if let Some((parts, k, l)) = jointed_parts(c) {
        let r = t::gamma_beam(&parts, k, l);
        draft.notes.push(Note { tag: "gamma".into(), at: [len / 2.0, -0.05, z + 0.05], text: same(format!("γ₁ = {:.2}, γ₃ = {:.2}, (EI)_ef/(EI) = {:.2}", r.gamma[0], r.gamma[2], r.ei_ef / r.ei_rigid)) });
    }
    draft.top = z;
    Some(draft.finish(Extras::default()))
}

/// 🏛️ The spaced column T-93: `n_parts` posts side by side with a spacer block at every spacing.
fn spaced_column(c: &CaseRow, size: Size) -> Extras {
    let n = num(c, "n_parts").unwrap_or(2.0).clamp(1.0, 4.0) as usize;
    let side = ((num(c, "area").unwrap_or(40000.0) / n as f64).sqrt() / 1e3).clamp(0.03, 0.4);
    let (height, pitch) = (size.h, 2.0 * side);
    let mut draft = Draft::new((n as f64 * side + (n - 1) as f64 * side, side));
    for j in 0..n {
        draft.subs.push(boxed(&format!("part{}", j + 1), named(&format!("Column part {}", j + 1), &format!("Stabteil {}", j + 1), c), MaterialRole::Timber, (j as f64 * pitch + side / 2.0, side / 2.0, 0.0), [side, side, height]));
    }
    let spacing = mm(c, "l1").filter(|v| *v > 0.05).unwrap_or(height / 3.0);
    let pack = (0.5 * side).max(0.04);
    draft.lift("l1", "l1", [side / 2.0, side / 2.0, spacing]);
    draft.count("n_parts", "n_parts", (1.0, 4.0), [draft.footprint.0, side / 2.0, height / 2.0]);
    draft.grips.push(gauge("area", "area", (2e6 * n as f64 * side, 100.0), (0.03 - side, 0.4 - side), Vec::new(), X, [side, side / 2.0, height / 2.0]));
    for level in 0..=((height / spacing) as usize).min(8) {
        let z = (level as f64 * spacing).min(height - pack);
        for j in 0..n.saturating_sub(1) {
            draft.subs.push(boxed(&format!("spacer{j}-{level}"), named("Spacer block", "Distanzholz", c), MaterialRole::Neutral, (j as f64 * pitch + side + side / 2.0, side / 2.0, z), [side, side, pack]));
        }
    }
    draft.top = height;
    draft.finish(Extras::default())
}

fn panels(c: &CaseRow) -> f64 {
    num(c, "n_panels").unwrap_or(1.0).clamp(1.0, 12.0).round()
}

/// 🛡️ The diaphragm T-94: sheathing panels side by side with a nail line along every panel edge at the spacing s (a wall stands in the xz plane, a floor or roof lies flat).
fn diaphragm(c: &CaseRow) -> Option<Extras> {
    let (b, n, flat) = (mm(c, "b_i")?, panels(c) as usize, matches!(word(c, "kind"), "floor" | "roof"));
    let (s, l) = (mm(c, "s").filter(|s| *s > 0.01).unwrap_or(0.075), if flat { mm(c, "l_dia")? } else { mm(c, "h")? });
    let p = |along: f64, across: f64, t: f64| if flat { [along, across, t] } else { [across, t, along] };
    let mut draft = Draft::new(if flat { (l, n as f64 * b) } else { (n as f64 * b, 0.1) });
    let mut nails = Vec::new();
    for i in 0..n {
        let (lo, hi) = (i as f64 * b, (i + 1) as f64 * b);
        let side = if flat { [0.0, 1.0, 0.0] } else { [1.0, 0.0, 0.0] };
        draft.subs.push(beam(&format!("panel{}", i + 1), (p(0.0, (lo + hi) / 2.0, SHEATHING / 2.0), p(l, (lo + hi) / 2.0, SHEATHING / 2.0), side), (b, SHEATHING), MaterialRole::Timber, named(&format!("Sheathing panel {}", i + 1), &format!("Beplankungsplatte {}", i + 1), c)));
        for edge in [lo + 0.02, hi - 0.02] {
            nails.extend((0..((l - 0.1) / s).floor().max(0.0) as usize + 1).take(80).map(|k| (0.05 + k as f64 * s, edge)));
        }
    }
    let (a0, c0) = nails.first().copied()?;
    let offsets = nails.iter().map(|&(a, cc)| if flat { [0.0, cc - c0, -(a - a0)] } else { [0.0, -(a - a0), -(cc - c0)] }).collect();
    let (from, to) = (p(a0, c0, -0.01), p(a0, c0, SHEATHING + 0.01));
    let text = (format!("{} nails at s = {:.0} mm along the panel edges", nails.len(), s * 1e3), format!("{} Nägel im Abstand s = {:.0} mm entlang der Plattenränder", nails.len(), s * 1e3));
    draft.subs.push(part("fastener-lines", named("Fastener lines", "Verbindungsmittellinien", c), text, MaterialRole::Bolt, from, SubShape::Pins { kind: PinKind::Nail, diameter: 0.003, to: [0, 1, 2].map(|i| to[i] - from[i]), pattern: Pins::Points(offsets) }));
    draft.top = if flat { SHEATHING } else { l };
    draft.drag("s", "s", 1.0, GripKind::Spacing, if flat { X } else { Z }, p(0.05 + s, 0.02, SHEATHING));
    draft.count("n_panels", "n_panels", (1.0, 12.0), p(l / 2.0, n as f64 * b, SHEATHING));
    if !flat {
        draft.lift("h", "h", p(l, n as f64 * b / 2.0, SHEATHING));
    }
    Some(draft.finish(Extras::default()))
}

/// 🔺️ The truss T-95 in the xz plane: chord and web members on the layout of `truss_layout`, a pin at every joint.
fn truss(c: &CaseRow) -> Option<Extras> {
    let (span, height) = (mm(c, "span")?, mm(c, "height")?);
    let kind = [TrussKind::Pratt, TrussKind::Howe, TrussKind::KingPost].into_iter().find(|k| k.id() == word(c, "kind")).unwrap_or(TrussKind::Warren);
    let (joints, members) = t::truss_layout(kind, span, height, num(c, "bays")?.clamp(2.0, 24.0) as u32);
    let ((cb, ch), (wb, wh)) = ((mm(c, "chord_b").unwrap_or(0.1), mm(c, "chord_h").unwrap_or(0.2)), (mm(c, "web_b").unwrap_or(0.08), mm(c, "web_h").unwrap_or(0.12)));
    let mut draft = Draft::new((span, cb));
    for (k, &(a, b, chord)) in members.iter().enumerate() {
        let (size, en, de) = if chord { ((cb, ch), "Chord member", "Gurtstab") } else { ((wb, wh), "Web member", "Füllstab") };
        draft.subs.push(beam(&format!("member{}", k + 1), ([joints[a].0, cb / 2.0, joints[a].1], [joints[b].0, cb / 2.0, joints[b].1], [0.0, 1.0, 0.0]), size, MaterialRole::Timber, named(&format!("{en} {}", k + 1), &format!("{de} {}", k + 1), c)));
    }
    let pin = ch.min(cb).max(0.06);
    for (k, &(x, z)) in joints.iter().enumerate() {
        draft.subs.push(beam(&format!("joint{}", k + 1), ([x, -0.01, z], [x, cb + 0.01, z], [1.0, 0.0, 0.0]), (pin, pin), MaterialRole::Bolt, named(&format!("Joint pin {}", k + 1), &format!("Knotenbolzen {}", k + 1), c)));
    }
    draft.top = height + ch;
    draft.lift("span", "span", [span, cb / 2.0, 0.0]);
    draft.lift("height", "height", [span / 2.0, cb / 2.0, height]);
    draft.count("bays", "bays", (2.0, 24.0), [span / 4.0, cb / 2.0, height]);
    draft.drag("chord_h", "chord_h", 2.0, GripKind::Scalar, Z, [0.0, cb / 2.0, ch / 2.0]);
    draft.drag("chord_b", "chord_b", 1.0, GripKind::Face, Y, [0.0, cb, 0.0]);
    if let Some(&(a, b, _)) = members.iter().find(|m| !m.2) {
        let (x, z) = ((joints[a].0 + joints[b].0) / 2.0, (joints[a].1 + joints[b].1) / 2.0);
        draft.drag("web_h", "web_h", 2.0, GripKind::Scalar, Z, [x, cb / 2.0, z + wh / 2.0]);
        draft.drag("web_b", "web_b", 2.0, GripKind::Face, Y, [x, cb / 2.0 + wb / 2.0, z]);
    }
    Some(draft.finish(Extras::default()))
}

/// ✖️ The bracing T-96: the stabilised members in a row with the bracing members across them (plan bracing: two diagonals; bridging: a block between neighbours at midspan; elastic restraint: a post under every member at midspan).
fn bracing(c: &CaseRow) -> Option<Extras> {
    let (l, k, sp) = (num(c, "l_m")?, num(c, "n_braced")?.clamp(1.0, 24.0) as usize, mm(c, "spacing").filter(|s| *s > 0.01).unwrap_or(0.8));
    let ((mb, mh), (bb, bh)) = ((mm(c, "member_b").unwrap_or(0.1), mm(c, "member_h").unwrap_or(0.2)), (mm(c, "brace_b").unwrap_or(0.06), mm(c, "brace_h").unwrap_or(0.12)));
    let (y, z) = (|i: usize| i as f64 * sp + mb / 2.0, mh + bh / 2.0);
    let mut draft = Draft::new((l, (k as f64 - 1.0) * sp + mb));
    for i in 0..k {
        draft.subs.push(beam(&format!("member{}", i + 1), ([0.0, y(i), mh / 2.0], [l, y(i), mh / 2.0], [0.0, 1.0, 0.0]), (mb, mh), MaterialRole::Timber, named(&format!("Stabilised member {}", i + 1), &format!("Ausgesteiftes Bauteil {}", i + 1), c)));
    }
    let mut bars: Vec<(String, P, P, P, (f64, f64))> = (0..k).map(|i| (format!("force{}", i + 1), [-(l / 4.0).min(0.5), y(i), mh / 2.0], [0.0, y(i), mh / 2.0], [0.0, 0.0, 1.0], (0.03, 0.03))).collect();
    match word(c, "kind") {
        "bridging" => bars.extend((0..k.saturating_sub(1)).map(|i| (format!("bridge{}", i + 1), [l / 2.0, y(i) + mb / 2.0, z], [l / 2.0, y(i + 1) - mb / 2.0, z], [0.0, 0.0, 1.0], (bh, bb)))),
        "elastic-restraint" => bars.extend((0..k).map(|i| (format!("spring{}", i + 1), [l / 2.0, y(i), -bh], [l / 2.0, y(i), 0.0], [1.0, 0.0, 0.0], (bb, bb)))),
        _ if k > 1 => bars.extend([("diagonal1", y(0), y(k - 1)), ("diagonal2", y(k - 1), y(0))].map(|(n, y0, y1)| (n.to_string(), [0.0, y0, z], [l, y1, z], [0.0, 0.0, 1.0], (bh, bb)))),
        _ => {}
    }
    for (name, a, b, side, size) in bars {
        draft.subs.push(beam(&name, (a, b, side), size, MaterialRole::Neutral, named(&format!("Bracing member {name}"), &format!("Aussteifungsstab {name}"), c)));
    }
    draft.top = mh + bh;
    draft.count("n_braced", "n_braced", (1.0, 24.0), [l / 2.0, y(k - 1) + mb, mh / 2.0]);
    draft.drag("spacing", "spacing", 1.0, GripKind::Spacing, Y, [l / 2.0, y(k.min(2) - 1), mh]);
    draft.drag("member_b", "member_b", 1.0, GripKind::Face, Y, [l / 2.0, y(0) + mb / 2.0, mh / 2.0]);
    draft.lift("member_h", "member_h", [l / 2.0, y(0), mh]);
    Some(draft.finish(Extras::default()))
}

/// 🗼️ The Miner damage tower T-121: a girder with one tower per stress block, as high as its damage, under the limit sheet D = 1.
fn fatigue(c: &CaseRow) -> Option<Extras> {
    let list: Vec<(usize, f64, f64)> = (1..=6).filter_map(|k| Some((k, num(c, &format!("ds{k}"))?, num(c, &format!("n{k}"))?))).collect();
    if list.is_empty() {
        return None;
    }
    let (len, width, depth) = GIRDER;
    let mut draft = Draft::new((len, width));
    draft.subs.push(boxed("girder", named("Girder", "Träger", c), MaterialRole::Timber, (len / 2.0, width / 2.0, 0.0), [len, width, depth]));
    let mut top = depth + DAMAGE_UNIT;
    let mut tallest: Option<(f64, f64)> = None;
    for (j, (k, ds, n)) in list.iter().enumerate() {
        let damage = if *ds < num(c, "ds_cut").unwrap_or(0.0) { 0.0 } else { t::miner_damage(num(c, "sn_a").unwrap_or(15.0), num(c, "sn_b").unwrap_or(4.0), *ds, n * num(c, "years").unwrap_or(1.0)) };
        let height = (damage * DAMAGE_UNIT).clamp(0.005, 1.5);
        top = top.max(depth + height);
        let text = same(format!("block {k}: Δσ {ds:.1} N/mm², D = {damage:.3}"));
        let block = part(&format!("block{k}"), named(&format!("Damage of block {k}"), &format!("Schädigung von Block {k}"), c), text, MaterialRole::Neutral, [0.3 + j as f64 * 0.35, width / 2.0, depth], SubShape::Box { size: [0.2, 0.2, height] });
        draft.subs.push(tint(block, (200, 120, 60), 0.85));
        let years = num(c, "years").unwrap_or(1.0);
        if damage > 0.0 && years > 0.0 {
            let (a, b) = (num(c, "sn_a").unwrap_or(15.0), num(c, "sn_b").unwrap_or(4.0));
            let per_metre = t::miner_damage(a, b, *ds, 1.0).recip() / (years * DAMAGE_UNIT);
            draft.grips.push(gauge(&format!("n{k}"), &format!("n{k}"), (per_metre, 1000.0), ((0.005 - height).max(-damage * DAMAGE_UNIT), 1.5 - height), Vec::new(), Z, [0.3 + j as f64 * 0.35, width / 2.0, depth + height]));
            if tallest.map_or(true, |(_, h)| damage * DAMAGE_UNIT > h) {
                tallest = Some((years, damage * DAMAGE_UNIT));
            }
        }
    }
    if let Some((years, reach)) = tallest {
        let per_metre = years / reach;
        draft.grips.push(gauge("years", "years", (per_metre, 1.0), ((1.0 - years) / per_metre, (100.0 - years) / per_metre), Vec::new(), Z, [len / 2.0, width / 2.0, depth + DAMAGE_UNIT]));
    }
    let limit = part("limit", ("Damage limit D = 1".into(), "Schädigungsgrenze D = 1".into()), ("Miner sum limit".into(), "Grenze der Miner-Summe".into()), MaterialRole::Neutral, [len / 2.0, width / 2.0, depth + DAMAGE_UNIT], SubShape::Box { size: [len, width, 0.004] });
    draft.subs.push(tint(limit, (200, 40, 40), 0.35));
    draft.top = top;
    Some(draft.finish(Extras::default()))
}

/// 🌉️ The stress-laminated deck T-122: laminations side by side across the width, a prestress bar and two anchor plates every bar spacing; the span runs along Y.
fn deck(c: &CaseRow, size: Size) -> Extras {
    let (width, len, depth) = (size.dx, size.dy, size.h);
    let count = (width / 0.045).ceil().clamp(1.0, 60.0) as usize;
    let mut draft = Draft::new((width, len));
    let plies = vec![(width / count as f64, true); count];
    let lamination = Sub { axes: Some([[0.0, 1.0, 0.0], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0]]), ..part("", named("Laminations", "Brettlagen", c), dims(width, len, depth), MaterialRole::Timber, [0.0, len / 2.0, depth / 2.0], SubShape::Laminate { length: len, width: depth, plies }) };
    draft.subs.push(lamination);
    let spacing = mm(c, "bar_spacing").filter(|s| *s > 0.05).unwrap_or(0.8);
    let side = (num(c, "plate_a").unwrap_or(10000.0).sqrt() / 1e3).clamp(0.04, 0.3);
    for (i, y) in (0..12).map(|k| spacing / 2.0 + f64::from(k) * spacing).take_while(|y| *y <= len).enumerate() {
        draft.subs.push(part(&format!("bar{i}"), named(&format!("Prestress bar {}", i + 1), &format!("Spannstab {}", i + 1), c), ("prestress bar".into(), "Spannstab".into()), MaterialRole::Bolt, [-0.05, y, depth / 2.0], SubShape::Pins { kind: PinKind::Dowel, diameter: 0.02, to: [width + 0.1, 0.0, 0.0], pattern: Pins::Points(vec![[0.0, 0.0, 0.0]]) }));
        for (x, label) in [(-0.06, "a"), (width + 0.06, "b")] {
            draft.subs.push(tint(boxed(&format!("plate{i}{label}"), ("Anchor plate".into(), "Ankerplatte".into()), MaterialRole::Neutral, (x, y, depth / 2.0 - side / 2.0), [0.02, side, side]), (90, 90, 100), 1.0));
        }
    }
    draft.lift("deck_depth", "deck_depth", [width / 2.0, len / 2.0, depth]);
    draft.drag("bar_spacing", "bar_spacing", 2.0, GripKind::Spacing, Y, [-0.05, spacing / 2.0, depth / 2.0]);
    draft.grips.push(gauge("plate_a", "plate_a", (4e6 * side, 100.0), ((0.04 - side) / 2.0, (0.3 - side) / 2.0), Vec::new(), Z, [width + 0.07, spacing / 2.0, depth / 2.0 + side / 2.0]));
    draft.top = depth;
    draft.finish(Extras::default())
}

/// 🧩️ The extras of one catalogue case drawn by the shared forms at `size`: the char, mode, assembly, grips and notes of the entries that need the timber kernel, nothing for every other entry.
pub fn case_extras(annex: norm::AnnexChoice, c: &CaseRow, size: Size) -> Extras {
    let rolling = |total: f64| Note { tag: "rolling-shear".into(), at: [0.0, -size.dy / 2.0, total + 0.05], text: ("Cross layers carry rolling shear".into(), "Querlagen tragen den Rollschub".into()) };
    let plies: f64 = (1..=num(c, "n_layers").unwrap_or(0.0).min(9.0) as usize).filter_map(|i| mm(c, &format!("t{i}"))).sum();
    match c.entry_id.as_str() {
        "T-101" | "T-102" | "T-106" => {
            let char_depth = |minutes: f64| {
                let after = facts::fire_after(annex, product_of(c), size.dy, size.h, minutes * 60.0);
                if c.entry_id == "T-106" { after.d_char } else { after.d_ef }
            };
            Extras { char: fire(annex, c, size), grips: fire_grips(c, size, forms::SUPPORT_HEIGHT, Some(faces(c)), &char_depth), ..Extras::default() }
        }
        "T-107" => {
            let mut grips = fire_grips(c, size, 0.0, None, &clt_depth(c));
            grips.push(grip("n_layers", "n_layers", 1.0, GripKind::Count { min: 1.0, max: 9.0 }, Z, [0.0, 0.0, plies + 0.1]));
            Extras { char: clt_char(c, size), notes: vec![rolling(plies)], grips, ..Extras::default() }
        }
        "T-103" => Extras { char: protected_char(annex, c, size), ..Extras::default() },
        "T-97" => Extras { notes: vec![rolling(plies)], ..Extras::default() },
        "T-63" | "T-64" | "T-65" | "T-66" | "T-124" => {
            let (mode, notes) = mode(c, size);
            Extras { mode, notes, ..Extras::default() }
        }
        "T-120" => {
            let (mode, notes) = mode(c, size);
            let density = num(c, "crowd_density").unwrap_or(0.0);
            let limits = (-density / CROWD_PER_METRE, (CROWD_MAX - density) / CROWD_PER_METRE);
            let grips = vec![gauge("crowd_density", "crowd_density", (CROWD_PER_METRE, 0.05), limits, Vec::new(), Z, [0.0, 0.0, forms::SUPPORT_HEIGHT + size.h + 1.9])];
            Extras { mode, notes, attachments: crowd(c, size).into_iter().collect(), grips, ..Extras::default() }
        }
        "T-90" => jointed(c, size).unwrap_or_default(),
        "T-93" => spaced_column(c, size),
        "T-94" if num(c, "b_i").is_some() => diaphragm(c).unwrap_or_default(),
        "T-95" if num(c, "span").is_some() => truss(c).unwrap_or_default(),
        "T-96" if num(c, "n_braced").is_some() => bracing(c).unwrap_or_default(),
        "T-121" => fatigue(c).unwrap_or_default(),
        "T-122" => deck(c, size),
        _ => Extras::default(),
    }
}
