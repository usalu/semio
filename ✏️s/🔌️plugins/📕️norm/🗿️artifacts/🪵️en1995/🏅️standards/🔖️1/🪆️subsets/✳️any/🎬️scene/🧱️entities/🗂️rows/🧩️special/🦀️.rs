//! 🧩️ Domain scenes of the Annex B and C, fire and bridge entries: the mechanically jointed beam T-90 (stacked parts with fastener rows), the spaced column T-93, the CLT char T-107, the Miner damage tower T-121, the stress-laminated deck T-122 and the crowd of T-120. Handles write the case leaves in the unit of the vocabulary. See [`super`].

use super::*;
use crate::schema::checks::timber as t;
use super::super::scene::projection_kit::handle_kit::{field_binding, grid, handle, round, HandleSpec as HSpec};

type At<'a> = &'a dyn Fn(f64, f64, f64) -> Vec3;
type Tag<'a> = &'a dyn Fn(&[&str]) -> Vec<String>;

fn num(c: &CaseRow, key: &str) -> Option<f64> {
    c.numbers.iter().find(|n| n.id == key).map(|n| n.value)
}

fn boxed(at: At<'_>, centre_x: f64, centre_y: f64, z: f64, size: Vec3) -> Geometry {
    Geometry::Solid(SolidGeometry { shape: SolidShape::Box { frame: Frame { origin: at(centre_x, centre_y, z), x: Vec3::X, y: Vec3::Y, z: Vec3::Z }, size } })
}

/// 🔥 Prefix of the check ids that colour the fire parts of an entry.
pub(crate) fn fire_prefix(c: &CaseRow) -> &'static str {
    match c.entry_id.as_str() {
        "T-102" => "en1995.t-102",
        "T-106" => "en1995.t-106",
        "T-107" => "en1995.t-107",
        _ => "en1995.t-101",
    }
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

/// 🧱️ The parts of an Annex B case in the order of the γ-method together with the slip modulus (N/mm) and the span (mm).
pub fn jointed_parts(c: &CaseRow) -> Option<(Vec<t::Part>, f64, f64)> {
    let part = |i: u8, needs_s: bool| -> Option<t::Part> { Some(t::Part { a: num(c, &format!("a{i}"))?, i: num(c, &format!("i{i}"))?, e: num(c, &format!("e{i}"))?, h: num(c, &format!("h{i}"))?, s: if needs_s { num(c, &format!("s{i}"))? } else { 1.0 } }) };
    let mut parts = vec![part(1, true)?, part(2, false)?];
    parts.extend(part(3, true));
    Some((parts, num(c, "k")?, num(c, "l")?))
}

fn column_dims(c: &CaseRow) -> (usize, f64, f64) {
    let n = num(c, "n_parts").unwrap_or(2.0).clamp(1.0, 4.0) as usize;
    let side = ((num(c, "area").unwrap_or(40000.0) / n as f64).sqrt() / 1e3).clamp(0.03, 0.4);
    (n, mm(c, "lef").unwrap_or(2.0).clamp(0.3, 8.0), side)
}

fn deck_dims(c: &CaseRow) -> (f64, f64, f64) {
    (num(c, "span_m").unwrap_or(3.0).clamp(1.0, 12.0), num(c, "deck_width_m").unwrap_or(1.0).clamp(0.3, 6.0), mm(c, "deck_depth").unwrap_or(0.25).clamp(0.05, 0.8))
}

fn blocks(c: &CaseRow) -> Vec<(usize, f64, f64)> {
    (1..=6).filter_map(|k| Some((k, num(c, &format!("ds{k}"))?, num(c, &format!("n{k}"))?))).collect()
}

fn damage_of(c: &CaseRow, ds: f64, n: f64) -> f64 {
    if ds < num(c, "ds_cut").unwrap_or(0.0) { 0.0 } else { t::miner_damage(num(c, "sn_a").unwrap_or(15.0), num(c, "sn_b").unwrap_or(4.0), ds, n * num(c, "years").unwrap_or(1.0)) }
}

const GIRDER: (f64, f64, f64) = (2.0, 0.3, 0.25);
const DAMAGE_UNIT: f64 = 0.6;

/// 📏 Plan footprint of the special entries.
pub(crate) fn extent(c: &CaseRow, g: &Geo) -> Option<(f64, f64)> {
    match c.entry_id.as_str() {
        "T-90" => Some((g.len, slabs(c).iter().fold(0.1_f64, |m, s| m.max(s.w)))),
        "T-93" => {
            let (n, _, side) = column_dims(c);
            Some((n as f64 * side + (n - 1) as f64 * side, side))
        }
        "T-94" if num(c, "b_i").is_some() => {
            let (b, n) = (mm(c, "b_i")?, panels(c));
            Some(if wall(c) { (n * b, 0.1) } else { (mm(c, "l_dia")?, n * b) })
        }
        "T-95" if num(c, "span").is_some() => Some((mm(c, "span")?, mm(c, "chord_b").unwrap_or(0.1))),
        "T-96" if num(c, "n_braced").is_some() => Some((num(c, "l_m")?, bracing_width(c))),
        "T-121" => Some((GIRDER.0, GIRDER.1)),
        "T-122" => {
            let (len, width, _) = deck_dims(c);
            Some((len, width))
        }
        _ => None,
    }
}

/// 🎬️ The parts and the height of a special entry, `None` for every other entry.
pub(crate) fn build(c: &CaseRow, g: &Geo, at: At<'_>, binding: &Option<UtilisationBinding>, tag: Tag<'_>, input: &Input, out: &mut dyn Sink) -> Option<(Vec<EntityRef>, f64)> {
    match c.entry_id.as_str() {
        "T-90" => jointed(c, g, at, binding, tag, out),
        "T-93" => Some(spaced_column(c, at, binding, tag, out)),
        "T-94" if num(c, "b_i").is_some() => diaphragm(c, at, binding, tag, out),
        "T-95" if num(c, "span").is_some() => truss(c, at, binding, tag, out),
        "T-96" if num(c, "n_braced").is_some() => bracing(c, at, binding, tag, out),
        "T-121" => fatigue(c, at, binding, out),
        "T-122" => Some(deck(c, at, binding, tag, input, out)),
        _ => None,
    }
}

const SHEATHING: f64 = 0.022;
type P = [f64; 3];

fn word<'a>(c: &'a CaseRow, key: &str) -> &'a str {
    c.texts.iter().find(|t| t.id == key).map_or("", |t| t.value.as_str())
}

fn wall(c: &CaseRow) -> bool {
    !matches!(word(c, "kind"), "floor" | "roof")
}

fn panels(c: &CaseRow) -> f64 {
    num(c, "n_panels").unwrap_or(1.0).clamp(1.0, 12.0).round()
}

fn bracing_width(c: &CaseRow) -> f64 {
    (num(c, "n_braced").unwrap_or(1.0) - 1.0).max(0.0) * mm(c, "spacing").unwrap_or(0.8) + mm(c, "member_b").unwrap_or(0.1)
}

/// 🧱️ A box from `a` to `b` whose width runs along the unit vector `side` and whose depth along `(b - a) × side`.
fn beam(c: &CaseRow, at: At<'_>, tag: &str, (a, b, side): (P, P, P), size: (f64, f64), (role, binding, name, handles): (MaterialRole, &Option<UtilisationBinding>, LocalizedCopy, Vec<String>), out: &mut dyn Sink) -> EntityRef {
    let d = [0, 1, 2].map(|i| b[i] - a[i]);
    let len = d.iter().map(|v| v * v).sum::<f64>().sqrt().max(1e-6);
    let x = d.map(|v| v / len);
    let n = [x[1] * side[2] - x[2] * side[1], x[2] * side[0] - x[0] * side[2], x[0] * side[1] - x[1] * side[0]];
    let o = [0, 1, 2].map(|i| (a[i] + b[i]) / 2.0 - n[i] * size.1 / 2.0);
    let (m, v) = (size.0.max(size.1) / 2.0, |p: P| Vec3::new(p[0], p[1], p[2]));
    let geometry = Geometry::Solid(SolidGeometry { shape: SolidShape::Box { frame: Frame { origin: at(o[0], o[1], o[2]), x: v(x), y: v(side), z: v(n) }, size: Vec3::new(len, size.0, size.1) } });
    let (lo, hi) = ([0, 1, 2].map(|i| a[i].min(b[i]) - m), [0, 1, 2].map(|i| a[i].max(b[i]) + m));
    place(c, plain(tag, geometry, role, binding, name, dims(len, size.0, size.1), handles), &[at(lo[0], lo[1], lo[2]), at(hi[0], hi[1], hi[2])], out)
}

/// 🛡️ A diaphragm: sheathing panels side by side with a nail line along every panel edge at the spacing s (a wall stands in the xz plane, a floor or roof lies flat; the sheathing thickness is a presentation constant).
fn diaphragm(c: &CaseRow, at: At<'_>, binding: &Option<UtilisationBinding>, tag: Tag<'_>, out: &mut dyn Sink) -> Option<(Vec<EntityRef>, f64)> {
    let (b, n, flat) = (mm(c, "b_i")?, panels(c) as usize, !wall(c));
    let (s, l) = (mm(c, "s").filter(|s| *s > 0.01).unwrap_or(0.075), if flat { mm(c, "l_dia")? } else { mm(c, "h")? });
    let p = |along: f64, across: f64, t: f64| if flat { [along, across, t] } else { [across, t, along] };
    let (mut kids, mut nails) = (Vec::new(), Vec::new());
    for i in 0..n {
        let (lo, hi) = (i as f64 * b, (i + 1) as f64 * b);
        let name = loc(&format!("Sheathing panel {} of {}", i + 1, c.label_en), &format!("Beplankungsplatte {} von {}", i + 1, c.label_de));
        kids.push(beam(c, at, &format!("panel{}", i + 1), (p(0.0, (lo + hi) / 2.0, SHEATHING / 2.0), p(l, (lo + hi) / 2.0, SHEATHING / 2.0), if flat { [0.0, 1.0, 0.0] } else { [1.0, 0.0, 0.0] }), (b, SHEATHING), (MaterialRole::Timber, binding, name, tag(&["b_i", "h", "n_panels", "l_dia"])), out));
        for edge in [lo + 0.02, hi - 0.02] {
            nails.extend((0..((l - 0.1) / s).floor().max(0.0) as usize + 1).take(80).map(|k| (0.05 + k as f64 * s, edge)));
        }
    }
    let (a0, c0) = nails.first().copied()?;
    let offsets = nails.iter().map(|&(a, cc)| if flat { Vec3::new(0.0, cc - c0, -(a - a0)) } else { Vec3::new(0.0, -(a - a0), -(cc - c0)) }).collect();
    let (from, to, far) = (p(a0, c0, -0.01), p(a0, c0, SHEATHING + 0.01), p(l, b * n as f64, SHEATHING));
    let text = format!("{} nails at s = {:.0} mm along the panel edges", nails.len(), s * 1e3);
    let fastener = Geometry::Fastener(FastenerGeometry { kind: FastenerKind::Nail, axis: Segment3 { from: at(from[0], from[1], from[2]), to: at(to[0], to[1], to[2]) }, diameter: 0.003, head: HeadShape::None, pattern: Some(Pattern::Points { offsets }), hole_clearance: None });
    kids.push(place(c, plain("fastener-lines", fastener, MaterialRole::Bolt, binding, loc(&format!("Fastener lines of {}", c.label_en), &format!("Verbindungsmittellinien von {}", c.label_de)), LocalizedCopy::new(text, format!("{} Nägel im Abstand s = {:.0} mm entlang der Plattenränder", nails.len(), s * 1e3)), tag(&["s"])), &[at(0.0, 0.0, 0.0), at(far[0], far[1], far[2])], out));
    Some((kids, if flat { SHEATHING } else { l }))
}

/// 🔺️ A truss in the xz plane: chord and web members on the layout of `truss_layout`, a pin at every joint.
fn truss(c: &CaseRow, at: At<'_>, binding: &Option<UtilisationBinding>, tag: Tag<'_>, out: &mut dyn Sink) -> Option<(Vec<EntityRef>, f64)> {
    use super::super::model::TrussKind as K;
    let (span, height) = (mm(c, "span")?, mm(c, "height")?);
    let kind = [K::Pratt, K::Howe, K::KingPost].into_iter().find(|k| k.id() == word(c, "kind")).unwrap_or(K::Warren);
    let (joints, members) = t::truss_layout(kind, span, height, num(c, "bays")?.clamp(2.0, 24.0) as u32);
    let ((cb, ch), (wb, wh)) = ((mm(c, "chord_b").unwrap_or(0.1), mm(c, "chord_h").unwrap_or(0.2)), (mm(c, "web_b").unwrap_or(0.08), mm(c, "web_h").unwrap_or(0.12)));
    let mut kids = Vec::new();
    for (k, &(a, b, chord)) in members.iter().enumerate() {
        let (size, keys, en, de) = if chord { ((cb, ch), ["chord_b", "chord_h"], "Chord member", "Gurtstab") } else { ((wb, wh), ["web_b", "web_h"], "Web member", "Füllstab") };
        let name = loc(&format!("{en} {} of {}", k + 1, c.label_en), &format!("{de} {} von {}", k + 1, c.label_de));
        kids.push(beam(c, at, &format!("member{}", k + 1), ([joints[a].0, cb / 2.0, joints[a].1], [joints[b].0, cb / 2.0, joints[b].1], [0.0, 1.0, 0.0]), size, (MaterialRole::Timber, binding, name, tag(&keys)), out));
    }
    let pin = ch.min(cb).max(0.06);
    for (k, &(x, z)) in joints.iter().enumerate() {
        let name = loc(&format!("Joint pin {} of {}", k + 1, c.label_en), &format!("Knotenbolzen {} von {}", k + 1, c.label_de));
        kids.push(beam(c, at, &format!("joint{}", k + 1), ([x, -0.01, z], [x, cb + 0.01, z], [1.0, 0.0, 0.0]), (pin, pin), (MaterialRole::Bolt, binding, name, tag(&["span", "height", "bays"])), out));
    }
    Some((kids, height + ch))
}

/// ✖️ A bracing: the stabilised members in a row (length l, spacing) with the bracing members across them (plan bracing: two diagonals; bridging: a block between neighbours at midspan; elastic restraint: a post under every member at midspan).
fn bracing(c: &CaseRow, at: At<'_>, binding: &Option<UtilisationBinding>, tag: Tag<'_>, out: &mut dyn Sink) -> Option<(Vec<EntityRef>, f64)> {
    let (l, k, sp) = (num(c, "l_m")?, num(c, "n_braced")?.clamp(1.0, 24.0) as usize, mm(c, "spacing").filter(|s| *s > 0.01).unwrap_or(0.8));
    let ((mb, mh), (bb, bh)) = ((mm(c, "member_b").unwrap_or(0.1), mm(c, "member_h").unwrap_or(0.2)), (mm(c, "brace_b").unwrap_or(0.06), mm(c, "brace_h").unwrap_or(0.12)));
    let (y, z) = (|i: usize| i as f64 * sp + mb / 2.0, mh + bh / 2.0);
    let mut kids = Vec::new();
    for i in 0..k {
        let name = loc(&format!("Stabilised member {} of {}", i + 1, c.label_en), &format!("Ausgesteiftes Bauteil {} von {}", i + 1, c.label_de));
        kids.push(beam(c, at, &format!("member{}", i + 1), ([0.0, y(i), mh / 2.0], [l, y(i), mh / 2.0], [0.0, 1.0, 0.0]), (mb, mh), (MaterialRole::Timber, binding, name, tag(&["l_m", "n_braced", "spacing"])), out));
    }
    let named = |n: &str| (MaterialRole::Neutral, binding, loc(&format!("Bracing member {n} of {}", c.label_en), &format!("Aussteifungsstab {n} von {}", c.label_de)), tag(&["brace_b", "brace_h", "n_d"]));
    let mut bars: Vec<(String, P, P, P, (f64, f64))> = (0..k).map(|i| (format!("force{}", i + 1), [-(l / 4.0).min(0.5), y(i), mh / 2.0], [0.0, y(i), mh / 2.0], [0.0, 0.0, 1.0], (0.03, 0.03))).collect();
    match word(c, "kind") {
        "bridging" => bars.extend((0..k.saturating_sub(1)).map(|i| (format!("bridge{}", i + 1), [l / 2.0, y(i) + mb / 2.0, z], [l / 2.0, y(i + 1) - mb / 2.0, z], [0.0, 0.0, 1.0], (bh, bb)))),
        "elastic-restraint" => bars.extend((0..k).map(|i| (format!("spring{}", i + 1), [l / 2.0, y(i), -bh], [l / 2.0, y(i), 0.0], [1.0, 0.0, 0.0], (bb, bb)))),
        _ if k > 1 => bars.extend([("diagonal1", y(0), y(k - 1)), ("diagonal2", y(k - 1), y(0))].map(|(n, y0, y1)| (n.to_string(), [0.0, y0, z], [l, y1, z], [0.0, 0.0, 1.0], (bh, bb)))),
        _ => {}
    }
    for (name, a, b, side, size) in bars {
        kids.push(beam(c, at, &name, (a, b, side), size, named(&name), out));
    }
    Some((kids, mh + bh))
}

fn jointed(c: &CaseRow, g: &Geo, at: At<'_>, binding: &Option<UtilisationBinding>, tag: Tag<'_>, out: &mut dyn Sink) -> Option<(Vec<EntityRef>, f64)> {
    let stack = slabs(c);
    if stack.len() < 2 {
        return None;
    }
    let wide = stack.iter().fold(0.0_f64, |m, s| m.max(s.w));
    let (mut z, mut kids) = (0.0, Vec::new());
    let mut interfaces = Vec::new();
    for (n, s) in stack.iter().enumerate() {
        let (ka, kh) = (format!("a{}", s.index), format!("h{}", s.index));
        let name = loc(&format!("Part {} of {}", s.index, c.label_en), &format!("Teil {} von {}", s.index, c.label_de));
        kids.push(place(c, plain(&format!("part{}", s.index), boxed(at, g.len / 2.0, wide / 2.0, z, Vec3::new(g.len, s.w, s.h)), s.role(), binding, name, dims(g.len, s.w, s.h), tag(&[&ka, &kh, "l"])), &[at(0.0, 0.0, z), at(g.len, wide, z + s.h)], out));
        z += s.h;
        if let Some(next) = stack.get(n + 1) {
            let owner = if s.index == 2 { next } else { s };
            if let Some(spacing) = owner.s {
                interfaces.push((z, s.h.min(next.h), owner.index, spacing));
            }
        }
    }
    for (zi, depth, owner, spacing) in interfaces {
        let xs: Vec<f64> = (0..40).map(|k| spacing / 2.0 + f64::from(k) * spacing).take_while(|x| *x <= g.len).collect();
        let Some(&first) = xs.first() else { continue };
        let (from, to) = (at(first, wide / 2.0, zi - 0.4 * depth), at(first, wide / 2.0, zi + 0.4 * depth));
        let offsets = xs.iter().map(|x| Vec3::new(0.0, 0.0, -(x - first))).collect();
        let key = format!("s{owner}");
        let text = format!("{} fasteners every {:.0} mm", xs.len(), spacing * 1e3);
        kids.push(place(c, plain(&format!("fasteners-{owner}"), Geometry::Fastener(FastenerGeometry { kind: FastenerKind::Nail, axis: Segment3 { from: from.clone(), to: to.clone() }, diameter: 0.006, head: HeadShape::None, pattern: Some(Pattern::Points { offsets }), hole_clearance: None }), MaterialRole::Bolt, binding, loc(&format!("Fasteners of part {owner} of {}", c.label_en), &format!("Verbindungsmittel von Teil {owner} von {}", c.label_de)), LocalizedCopy::new(text.clone(), text), tag(&[&key])), &[from, to], out));
    }
    if let Some((parts, k, l)) = jointed_parts(c) {
        let r = t::gamma_beam(&parts, k, l);
        let text = format!("γ₁ = {:.2}, γ₃ = {:.2}, (EI)_ef/(EI) = {:.2}", r.gamma[0], r.gamma[2], r.ei_ef / r.ei_rigid);
        let anchor = at(g.len / 2.0, -0.05, z + 0.05);
        out.annotate(Annotation::Label { id: format!("{}:gamma", c.id), anchor: anchor.clone(), text: loc(&text, &text), tone: tone_of(binding) }, &anchor);
    }
    Some((kids, z))
}

fn spaced_column(c: &CaseRow, at: At<'_>, binding: &Option<UtilisationBinding>, tag: Tag<'_>, out: &mut dyn Sink) -> (Vec<EntityRef>, f64) {
    let (n, height, side) = column_dims(c);
    let pitch = 2.0 * side;
    let mut kids = Vec::new();
    for j in 0..n {
        kids.push(place(c, plain(&format!("part{}", j + 1), boxed(at, j as f64 * pitch + side / 2.0, side / 2.0, 0.0, Vec3::new(side, side, height)), MaterialRole::Timber, binding, loc(&format!("Column part {} of {}", j + 1, c.label_en), &format!("Stabteil {} von {}", j + 1, c.label_de)), dims(side, side, height), tag(&["lef", "n_parts"])), &[at(j as f64 * pitch, 0.0, 0.0), at(j as f64 * pitch + side, side, height)], out));
    }
    let spacing = mm(c, "l1").filter(|v| *v > 0.05).unwrap_or(height / 3.0);
    let pack = (0.5 * side).max(0.04);
    for level in 0..=((height / spacing) as usize).min(8) {
        let z = (level as f64 * spacing).min(height - pack);
        for j in 0..n.saturating_sub(1) {
            let x = j as f64 * pitch + side + side / 2.0;
            kids.push(place(c, plain(&format!("spacer{j}-{level}"), boxed(at, x, side / 2.0, z, Vec3::new(side, side, pack)), MaterialRole::Neutral, binding, loc(&format!("Spacer block of {}", c.label_en), &format!("Distanzholz von {}", c.label_de)), dims(side, side, pack), tag(&["l1"])), &[at(x - side / 2.0, 0.0, z), at(x + side / 2.0, side, z + pack)], out));
        }
    }
    (kids, height)
}

fn fatigue(c: &CaseRow, at: At<'_>, binding: &Option<UtilisationBinding>, out: &mut dyn Sink) -> Option<(Vec<EntityRef>, f64)> {
    let list = blocks(c);
    if list.is_empty() {
        return None;
    }
    let (len, width, depth) = GIRDER;
    let mut kids = vec![place(c, plain("girder", boxed(at, len / 2.0, width / 2.0, 0.0, Vec3::new(len, width, depth)), MaterialRole::Timber, binding, loc(&format!("Girder of {}", c.label_en), &format!("Träger von {}", c.label_de)), dims(len, width, depth), Vec::new()), &[at(0.0, 0.0, 0.0), at(len, width, depth)], out)];
    let mut top = depth + DAMAGE_UNIT;
    for (j, (k, ds, n)) in list.iter().enumerate() {
        let d = damage_of(c, *ds, *n);
        let height = (d * DAMAGE_UNIT).clamp(0.005, 1.5);
        top = top.max(depth + height);
        let x = 0.3 + j as f64 * 0.35;
        let text = format!("block {k}: Δσ {ds:.1} N/mm², D = {d:.3}");
        kids.push(place(c, Part { appearance: Some(tinted((200, 120, 60), 0.85, StyleState::Normal)), ..plain(&format!("block{k}"), boxed(at, x, width / 2.0, depth, Vec3::new(0.2, 0.2, height)), MaterialRole::Neutral, binding, loc(&format!("Damage of block {k} of {}", c.label_en), &format!("Schädigung von Block {k} von {}", c.label_de)), LocalizedCopy::new(text.clone(), text), Vec::new()) }, &[at(x - 0.1, 0.0, depth), at(x + 0.1, width, depth + height)], out));
    }
    kids.push(place(c, Part { appearance: Some(tinted((200, 40, 40), 0.35, StyleState::Normal)), ..plain("limit", boxed(at, len / 2.0, width / 2.0, depth + DAMAGE_UNIT, Vec3::new(len, width, 0.004)), MaterialRole::Neutral, binding, loc("Damage limit D = 1", "Schädigungsgrenze D = 1"), loc("Miner sum limit", "Grenze der Miner-Summe"), Vec::new()) }, &[at(0.0, 0.0, depth + DAMAGE_UNIT), at(len, width, depth + DAMAGE_UNIT)], out));
    Some((kids, top))
}

fn deck(c: &CaseRow, at: At<'_>, binding: &Option<UtilisationBinding>, tag: Tag<'_>, input: &Input, out: &mut dyn Sink) -> (Vec<EntityRef>, f64) {
    let (len, width, depth) = deck_dims(c);
    let lamination = mm(c, "lam_b").filter(|b| *b > 0.005).unwrap_or(0.045);
    let count = (width / lamination).ceil().clamp(1.0, 60.0) as usize;
    let layers = (0..count).map(|i| GrainLayer { thickness: width / count as f64, role: MaterialRole::Timber, grain: 0.0, label: loc(&format!("Lamination {}", i + 1), &format!("Brettlage {}", i + 1)) }).collect();
    let frame = Frame { origin: at(len / 2.0, width, depth / 2.0), x: Vec3::X, y: Vec3::Z, z: Vec3::new(0.0, -1.0, 0.0) };
    let mut kids = vec![place(c, plain("laminations", Geometry::Solid(SolidGeometry { shape: SolidShape::Laminate { frame, footprint: rect(len, depth), layers } }), MaterialRole::Timber, &binding_of(input.report, &c.id, "en1995.t-122.prestress_min"), loc(&format!("Laminations of {}", c.label_en), &format!("Brettlagen von {}", c.label_de)), dims(len, width, depth), tag(&["span_m", "deck_width_m", "deck_depth"])), &[at(0.0, 0.0, 0.0), at(len, width, depth)], out)];
    let spacing = mm(c, "bar_spacing").filter(|s| *s > 0.05).unwrap_or(0.8);
    let side = (num(c, "plate_a").unwrap_or(10000.0).sqrt() / 1e3).clamp(0.04, 0.6);
    let bar_d = mm(c, "bar_d").unwrap_or(0.02).clamp(0.005, 0.1);
    for (i, x) in (0..40).map(|k| spacing / 2.0 + f64::from(k) * spacing).take_while(|x| *x <= len).enumerate() {
        let (from, to) = (at(x, -0.05, depth / 2.0), at(x, width + 0.05, depth / 2.0));
        kids.push(place(c, plain(&format!("bar{i}"), Geometry::Fastener(FastenerGeometry { kind: FastenerKind::Dowel, axis: Segment3 { from: from.clone(), to: to.clone() }, diameter: bar_d, head: HeadShape::None, pattern: None, hole_clearance: None }), MaterialRole::Bolt, &binding_of(input.report, &c.id, "en1995.t-122.bar_force"), loc(&format!("Prestress bar {} of {}", i + 1, c.label_en), &format!("Spannstab {} von {}", i + 1, c.label_de)), loc("prestress bar", "Spannstab"), tag(&["bar_spacing"])), &[from, to], out));
        for (side_y, label) in [(-0.06, "a"), (width + 0.06, "b")] {
            kids.push(place(c, Part { appearance: Some(tinted((90, 90, 100), 1.0, StyleState::Normal)), ..plain(&format!("plate{i}{label}"), boxed(at, x, side_y, depth / 2.0 - side / 2.0, Vec3::new(side, 0.02, side)), MaterialRole::Neutral, &binding_of(input.report, &c.id, "en1995.t-122.plate_bearing"), loc("Anchor plate", "Ankerplatte"), dims(side, 0.02, side), Vec::new()) }, &[at(x - side / 2.0, side_y, 0.0), at(x + side / 2.0, side_y, depth)], out));
        }
    }
    if let (Some(wheel), Some(contact)) = (num(c, "wheel_n"), mm(c, "contact_mm").filter(|a| *a > 0.0)) {
        let (x, y, height) = (len / 2.0, width / 2.0, 0.01);
        let text = format!("wheel load Q = {:.0} kN on a patch of {:.0} × {:.0} mm", wheel / 1e3, contact * 1e3, contact * 1e3);
        let text_de = format!("Radlast Q = {:.0} kN auf einer Fläche von {:.0} × {:.0} mm", wheel / 1e3, contact * 1e3, contact * 1e3);
        kids.push(place(c, Part { appearance: Some(tinted((200, 60, 60), 0.6, StyleState::Normal)), ..plain("wheel-patch", boxed(at, x, y, depth, Vec3::new(contact, contact, height)), MaterialRole::Neutral, &binding_of(input.report, &c.id, "en1995.t-122.wheel_spread"), loc(&format!("Wheel load patch of {}", c.label_en), &format!("Radlastfläche von {}", c.label_de)), loc(&text, &text_de), tag(&["wheel_n", "contact_mm"])) }, &[at(x - contact / 2.0, y - contact / 2.0, depth), at(x + contact / 2.0, y + contact / 2.0, depth + height)], out));
    }
    (kids, depth)
}

/// 🌳️ The char of a CLT layup after 30, 60, 90, 120 minutes and at `t_req` as boxes from the exposed face.
pub(crate) fn clt_char(c: &CaseRow, g: &Geo, at: At<'_>, input: &Input, tag: Tag<'_>, out: &mut dyn Sink) -> Vec<EntityRef> {
    if g.t_req_min <= 0.0 {
        return Vec::new();
    }
    let thick: Vec<f64> = g.layers.iter().map(|l| l.0 * 1e3).collect();
    let (beta, factor, protect, on) = (num(c, "beta_0").unwrap_or(0.65), num(c, "fall_off_factor").unwrap_or_else(|| norm::ndp_common("ndp.en1995.fire.clt-fall-off-factor")), num(c, "protect_mm").unwrap_or_else(|| norm::ndp_common("ndp.en1995.fire.clt-fall-off-protect")), num(c, "fall_off").unwrap_or(1.0) > 0.5);
    let binding = binding_of(input.report, &c.id, "en1995.t-107");
    let mut kids = Vec::new();
    let steps = FIRE_MINUTES.iter().zip(FIRE_LAYERS).map(|(m, layer)| (format!("char@{m}"), f64::from(*m), layer)).chain([("residual".to_string(), g.t_req_min, LAYER_FIRE)]);
    for (id, minutes, layer) in steps {
        let depth = t::clt_char_depth(&thick, beta, minutes, on, factor, protect).0 / 1e3;
        if depth <= 0.0 {
            continue;
        }
        let text = format!("d_char {:.1} mm after {minutes:.0} min", depth * 1e3);
        kids.push(place(c, Part { appearance: Some(tinted((60, 40, 30), 0.55, StyleState::Normal)), layer, ..plain(&id, boxed(at, g.len / 2.0, g.b / 2.0, 0.0, Vec3::new(g.len * 1.01, g.b * 1.01, depth)), MaterialRole::Neutral, &binding, loc(&format!("Char of {} after {minutes:.0} min", c.label_en), &format!("Abbrand von {} nach {minutes:.0} min", c.label_de)), LocalizedCopy::new(text.clone(), text), tag(&["t_req"])) }, &[at(0.0, 0.0, 0.0), at(g.len, g.b, depth)], out));
    }
    kids
}

pub const FLOOR_COLUMNS: u32 = 25;
pub const FLOOR_ROWS: u32 = 17;
const FLOOR_AMPLITUDE: f64 = 0.04;

/// 🏢️ The floor plate of a case with width and transverse stiffness: its first mode as an animated surface over the slab (exact orthotropic plate mode) coloured by the peak acceleration per unit impulse, a heat map of where the floor is lively.
pub(crate) fn floor_plate(c: &CaseRow, g: &Geo, at: At<'_>, binding: &Option<UtilisationBinding>, out: &mut dyn Sink) -> Vec<EntityRef> {
    let (Some(plate), Some((span, ei_l, mass))) = (g.plate, g.vibration) else { return Vec::new() };
    let f1 = t::floor_plate_f1(span, plate.width, ei_l, plate.ei_b, mass);
    let peak = t::floor_plate_peak_acceleration(span, plate.width, ei_l, plate.ei_b, mass);
    let (mut points, mut samples, mut values) = (Vec::new(), Vec::new(), Vec::new());
    for j in 0..FLOOR_ROWS {
        for i in 0..FLOOR_COLUMNS {
            let (x, y) = (span * f64::from(i) / f64::from(FLOOR_COLUMNS - 1), plate.width * f64::from(j) / f64::from(FLOOR_ROWS - 1));
            let phi = t::floor_plate_mode(x, y, span, plate.width);
            points.push(at(x, y, g.h + 0.003));
            samples.push(SurfaceSample { at: Vec2::new(x, y), delta: Vec3::new(0.0, 0.0, FLOOR_AMPLITUDE * span * phi) });
            values.push(peak * phi.abs());
        }
    }
    let overlay = DeformationOverlay { kind: DeformationKind::Mode, index: 1, frequency: Some(Quantity::new(QuantityKind::Frequency, f1)), participation: 1.0, scale: 1.0, samples: Vec::new(), surface: samples, motion: Motion::Harmonic, load_factor: None };
    let field = ScalarField { id: "acceleration".into(), unit: QuantityKind::Acceleration, samples: FieldSamples::Vertex { values } };
    let appearance = Appearance { role: MaterialRole::Timber, channel: ColourChannel::Fields { active: "acceleration".into(), fields: vec![field] }, style: StyleState::Normal, opacity: 0.95, hatch: None, deformation: Some(overlay) };
    let detail = loc(&format!("first mode f₁ = {f1:.2} Hz, peak acceleration {peak:.3} m/s² per N·s at the antinode"), &format!("erste Eigenform f₁ = {f1:.2} Hz, größte Beschleunigung {peak:.3} m/s² je N·s im Schwingungsbauch"));
    let mode = place(c, Part { appearance: Some(appearance), layer: LAYER_MEMBERS, ..plain("mode", Geometry::Surface(SurfaceGeometry { shape: SurfaceShape::Grid { grid: PointGrid { rows: FLOOR_ROWS, columns: FLOOR_COLUMNS, points: points.clone() } } }), MaterialRole::Timber, binding, loc(&format!("Floor mode and acceleration of {}", c.label_en), &format!("Deckenschwingform und Beschleunigung von {}", c.label_de)), detail, Vec::new()) }, &points, out);
    out.legend(Legend { id: format!("acceleration:{}", c.id), title: loc("Peak acceleration per unit impulse (m/s² per N·s)", "Größte Beschleunigung je Einheitsimpuls (m/s² je N·s)"), scale: LegendScale::Scalar { field_id: "acceleration".into(), map: ColourMapRef { id: "warm".into(), min: 0.0, max: peak.max(1e-9) }, unit: QuantityKind::Acceleration } });
    let anchor = at(span / 2.0, plate.width / 2.0, g.h + 0.25);
    out.annotate(Annotation::Label { id: format!("{}:floor-mode", c.id), anchor: anchor.clone(), text: loc(&format!("f₁ = {f1:.2} Hz, a ≤ {peak:.3} m/s² per N·s"), &format!("f₁ = {f1:.2} Hz, a ≤ {peak:.3} m/s² je N·s")), tone: tone_of(binding) }, &anchor);
    vec![mode]
}

/// 🚶️ The crowd of a footbridge case: standing persons on a grid over the deck, one entity.
pub(crate) fn crowd(c: &CaseRow, g: &Geo, at: At<'_>, binding: &Option<UtilisationBinding>, out: &mut dyn Sink) -> Vec<EntityRef> {
    let density = num(c, "crowd_density").unwrap_or(0.0);
    if density <= 0.0 || c.entry_id != "T-120" {
        return Vec::new();
    }
    let people = (density * g.len * g.b).round().clamp(1.0, 120.0);
    let nx = (people * g.b / g.len).sqrt().ceil().clamp(1.0, 8.0) as u32;
    let ny = (people / f64::from(nx)).ceil().clamp(1.0, 60.0) as u32;
    let (sx, sy) = (g.b / f64::from(nx), g.len / f64::from(ny));
    let (from, to) = (at(g.len - sy / 2.0, sx / 2.0, g.h), at(g.len - sy / 2.0, sx / 2.0, g.h + 1.7));
    let text = format!("{} persons, {density:.2} persons/m²", nx * ny);
    vec![place(c, Part { appearance: Some(tinted((200, 160, 120), 1.0, StyleState::Normal)), ..plain("crowd", Geometry::Fastener(FastenerGeometry { kind: FastenerKind::Dowel, axis: Segment3 { from: from.clone(), to: to.clone() }, diameter: 0.35, head: HeadShape::None, pattern: Some(Pattern::Grid { nx, ny, sx, sy }), hole_clearance: None }), MaterialRole::Neutral, binding, loc(&format!("Crowd on {}", c.label_en), &format!("Menschenmenge auf {}", c.label_de)), LocalizedCopy::new(text.clone(), text), Vec::new()) }, &[from, to], out)]
}

fn grip(entity: &EntityRef, id: &str, tag: &str, kind: HandleKind, anchor: Vec3, writes: Vec<FieldBinding>, step: f64, limits: (f64, f64), name: (String, String)) -> Handle {
    handle(HSpec { entity, tag, kind, anchor, grip: GripStyle::Diamond, writes, snap: grid(step), limits, name: loc(&format!("{} of {id}", name.0), &format!("{} von {id}", name.1)) })
}

fn count_grip(entity: &EntityRef, id: &str, tag: &str, path: String, anchor: Vec3, limits: (f64, f64), name: (&str, &str)) -> Handle {
    grip(entity, id, tag, HandleKind::CountStepper, anchor, vec![FieldBinding { path, map: Affine::IDENTITY, unit: "1".into(), quantise: Some(Quantise::Int) }], 1.0, limits, (name.0.into(), name.1.into()))
}

/// 🕹️ The drag grips of the special entries; `o` is the case origin of the layout.
pub(crate) fn handles(c: &CaseRow, g: &Geo, o: &Vec3, entity: &EntityRef) -> Vec<Handle> {
    let at = |x: f64, y: f64, z: f64| v3(o.x + x, o.y + y, o.z + z);
    let path = |key: &str| format!("cases[id={}].numbers[id={key}].value", c.id);
    let write = |key: &str, base: f64, k: f64, unit: &str, step: f64| field_binding(path(key), Affine { base, k }, unit, round(step));
    let id = c.label_en.as_str();
    let mut out = Vec::new();
    match c.entry_id.as_str() {
        "T-90" => {
            let stack = slabs(c);
            let wide = stack.iter().fold(0.0_f64, |m, s| m.max(s.w));
            let mut z = 0.0;
            for s in &stack {
                let h_mm = s.h * 1e3;
                out.push(grip(entity, id, &format!("h{}", s.index), HandleKind::FaceOffset { normal: Vec3::Z }, at(g.len / 2.0, wide / 2.0, z + s.h), vec![write(&format!("h{}", s.index), -z * 1e3, 1e3, "mm", 1.0)], 0.005, (z + 0.005, z + 0.5), (format!("Depth of part {}", s.index), format!("Höhe von Teil {}", s.index))));
                out.push(grip(entity, id, &format!("a{}", s.index), HandleKind::FaceOffset { normal: Vec3::Y }, at(g.len / 2.0, wide / 2.0 + s.w / 2.0, z + s.h / 2.0), vec![write(&format!("a{}", s.index), -wide * 1e3 * h_mm, 2e3 * h_mm, "mm²", 100.0), write(&format!("i{}", s.index), -wide * 1e3 * h_mm.powi(3) / 12.0, 2e3 * h_mm.powi(3) / 12.0, "mm⁴", 1e4)], 0.005, (wide / 2.0 + 0.01, wide / 2.0 + 0.4), (format!("Width of part {}", s.index), format!("Breite von Teil {}", s.index))));
                if let Some(spacing) = s.s {
                    out.push(grip(entity, id, &format!("s{}", s.index), HandleKind::DimensionLine, at(spacing / 2.0, wide / 2.0, z + s.h), vec![write(&format!("s{}", s.index), 0.0, 1e3, "mm", 5.0)], 0.005, (0.01, g.len), (format!("Fastener spacing of part {}", s.index), format!("Verbindungsmittelabstand von Teil {}", s.index))));
                }
                z += s.h;
            }
        }
        "T-93" => {
            let (n, height, side) = column_dims(c);
            let spacing = mm(c, "l1").filter(|v| *v > 0.05).unwrap_or(height / 3.0);
            out.push(grip(entity, id, "lef", HandleKind::ScalarAlong { axis: Vec3::Z }, at(side / 2.0, side / 2.0, height), vec![write("lef", 0.0, 1e3, "mm", 10.0)], 0.01, (0.3, 8.0), ("Effective length".into(), "Ersatzstablänge".into())));
            out.push(grip(entity, id, "l1", HandleKind::DimensionLine, at(side / 2.0, side / 2.0, spacing), vec![write("l1", 0.0, 1e3, "mm", 10.0)], 0.01, (0.05, height), ("Spacer distance".into(), "Distanzholzabstand".into())));
            out.push(grip(entity, id, "n_parts", HandleKind::CountStepper, at(n as f64 * side, side / 2.0, height / 2.0), vec![FieldBinding { path: path("n_parts"), map: Affine::IDENTITY, unit: "1".into(), quantise: Some(Quantise::Int) }], 1.0, (1.0, 4.0), ("Number of parts".into(), "Anzahl Stabteile".into())));
        }
        "T-95" if num(c, "span").is_some() => {
            let (span, height) = (mm(c, "span").unwrap_or(0.0), mm(c, "height").unwrap_or(0.0));
            out.push(grip(entity, id, "span", HandleKind::ScalarAlong { axis: Vec3::X }, at(span, 0.0, 0.0), vec![write("span", 0.0, 1e3, "mm", 10.0)], 0.01, (1.0, 40.0), ("Span".into(), "Spannweite".into())));
            out.push(grip(entity, id, "height", HandleKind::FaceOffset { normal: Vec3::Z }, at(span / 2.0, 0.0, height), vec![write("height", 0.0, 1e3, "mm", 10.0)], 0.01, (0.2, 6.0), ("Truss height".into(), "Trägerhöhe".into())));
            out.push(count_grip(entity, id, "bays", path("bays"), at(span / 4.0, 0.0, height), (2.0, 24.0), ("Number of bays", "Anzahl der Felder")));
        }
        "T-96" if num(c, "n_braced").is_some() => {
            let (l, spacing) = (num(c, "l_m").unwrap_or(0.0), mm(c, "spacing").unwrap_or(0.8));
            out.push(grip(entity, id, "l_m", HandleKind::ScalarAlong { axis: Vec3::X }, at(l, 0.0, 0.0), vec![write("l_m", 0.0, 1.0, "m", 0.05)], 0.05, (1.0, 40.0), ("Bracing length".into(), "Aussteifungslänge".into())));
            out.push(grip(entity, id, "spacing", HandleKind::DimensionLine, at(l / 2.0, spacing, 0.0), vec![write("spacing", 0.0, 1e3, "mm", 10.0)], 0.01, (0.1, 3.0), ("Member spacing".into(), "Bauteilabstand".into())));
            out.push(count_grip(entity, id, "n_braced", path("n_braced"), at(l / 2.0, 0.0, 0.5), (1.0, 24.0), ("Stabilised members", "Ausgesteifte Bauteile")));
        }
        "T-94" if num(c, "b_i").is_some() => {
            let (b, s) = (mm(c, "b_i").unwrap_or(1.2), mm(c, "s").unwrap_or(0.075));
            out.push(grip(entity, id, "b_i", HandleKind::DimensionLine, at(b, 0.0, 0.0), vec![write("b_i", 0.0, 1e3, "mm", 10.0)], 0.01, (0.3, 2.5), ("Panel width".into(), "Plattenbreite".into())));
            out.push(grip(entity, id, "s", HandleKind::DimensionLine, at(0.02, 0.0, 0.05 + s / 2.0), vec![write("s", 0.0, 1e3, "mm", 5.0)], 0.005, (0.02, 0.4), ("Fastener spacing".into(), "Verbindungsmittelabstand".into())));
            out.push(count_grip(entity, id, "n_panels", path("n_panels"), at(b * panels(c), 0.0, 0.0), (1.0, 12.0), ("Sheathing panels", "Beplankungsplatten")));
        }
        "T-121" => {
            for (k, ds, n) in blocks(c) {
                let years = num(c, "years").unwrap_or(1.0);
                let cycles_to_failure = t::miner_damage(num(c, "sn_a").unwrap_or(15.0), num(c, "sn_b").unwrap_or(4.0), ds, 1.0).recip();
                let k_map = cycles_to_failure / (years * DAMAGE_UNIT);
                let height = damage_of(c, ds, n) * DAMAGE_UNIT;
                let x = 0.3 + (k - 1) as f64 * 0.35;
                out.push(grip(entity, id, &format!("n{k}"), HandleKind::FaceOffset { normal: Vec3::Z }, at(x, GIRDER.1 / 2.0, GIRDER.2 + height.clamp(0.005, 1.5)), vec![write(&format!("n{k}"), -GIRDER.2 * k_map, k_map, "1", 1000.0)], 0.005, (GIRDER.2, GIRDER.2 + 1.5), (format!("Cycles of block {k}"), format!("Lastspiele von Block {k}"))));
            }
        }
        "T-122" => {
            let (len, width, depth) = deck_dims(c);
            let spacing = mm(c, "bar_spacing").filter(|s| *s > 0.05).unwrap_or(0.8);
            out.push(grip(entity, id, "bar_spacing", HandleKind::DimensionLine, at(spacing / 2.0, -0.1, depth / 2.0), vec![write("bar_spacing", 0.0, 1e3, "mm", 10.0)], 0.01, (0.1, len), ("Bar spacing".into(), "Stababstand".into())));
            out.push(grip(entity, id, "deck_depth", HandleKind::FaceOffset { normal: Vec3::Z }, at(len / 2.0, width / 2.0, depth), vec![write("deck_depth", 0.0, 1e3, "mm", 10.0)], 0.01, (0.05, 0.8), ("Deck depth".into(), "Deckhöhe".into())));
            out.push(grip(entity, id, "deck_width_m", HandleKind::FaceOffset { normal: Vec3::Y }, at(len / 2.0, width, depth / 2.0), vec![write("deck_width_m", 0.0, 1.0, "m", 0.05)], 0.05, (0.3, 6.0), ("Deck width".into(), "Deckbreite".into())));
            out.push(grip(entity, id, "span_m", HandleKind::ScalarAlong { axis: Vec3::X }, at(len, width / 2.0, depth / 2.0), vec![write("span_m", 0.0, 1.0, "m", 0.05)], 0.05, (1.0, 12.0), ("Span".into(), "Spannweite".into())));
            if let Some(contact) = mm(c, "contact_mm").filter(|a| *a > 0.0) {
                out.push(grip(entity, id, "contact_mm", HandleKind::DimensionLine, at(len / 2.0 + contact / 2.0, width / 2.0, depth + 0.01), vec![write("contact_mm", 0.0, 1e3, "mm", 10.0)], 0.01, (0.05, 1.5), ("Wheel contact width".into(), "Aufstandsbreite des Rades".into())));
            }
        }
        "T-120" if num(c, "crowd_density").is_some() => {
            out.push(grip(entity, id, "crowd_density", HandleKind::CountStepper, at(g.len / 2.0, g.b / 2.0, g.h + 1.9), vec![write("crowd_density", 0.0, 0.1, "1", 0.05)], 1.0, (0.0, 15.0), ("Crowd density".into(), "Personendichte".into())));
            out.push(grip(entity, id, "deck_width_m", HandleKind::FaceOffset { normal: Vec3::Y }, at(g.len / 2.0, g.b, g.h / 2.0), vec![write("deck_width_m", 0.0, 1.0, "m", 0.05)], 0.05, (0.3, 6.0), ("Deck width".into(), "Deckbreite".into())));
        }
        _ => {}
    }
    out
}
