//! 🧱️ Reinforcement: bar positions in section coordinates (`y` across the width, `z` up, origin at the centroid) derived from the layer records; the rule is documented and golden-tested.
//! A layer with explicit bars (`bars[]` of the member) uses their positions and diameters instead. Beams, slabs and ties stack rows from the face named by the layer position (`top`/`compression` up, anything else down); columns and walls spread the bars evenly on the stirrup perimeter.

use super::layout::{class_of, Class};
use super::model::{BarBend, BarPosition, RcMember};
use super::scene::Vec3;

pub const MIN_GAP: f64 = 0.020;

/// 🧱️ The bar centres of one longitudinal layer.
#[derive(Clone, Debug, PartialEq)]
pub struct LayerBars {
    pub layer: usize,
    pub points: Vec<(f64, f64)>,
    pub diameters: Vec<f64>,
    pub explicit: bool,
}

pub fn link_diameter(member: &RcMember) -> f64 {
    member.stirrups.as_ref().map_or(0.0, |s| s.diameter)
}

pub fn is_top(position: BarPosition) -> bool {
    matches!(position, BarPosition::Top | BarPosition::Compression)
}

/// 🏷️ The lower-case name of a layer position as the bar labels spell it.
pub fn position_name(position: BarPosition) -> &'static str {
    match position {
        BarPosition::Bottom => "bottom",
        BarPosition::Top => "top",
        BarPosition::Tension => "tension",
        BarPosition::Compression => "compression",
    }
}

fn perimeter(width: f64, height: f64, inset: f64, count: usize) -> Vec<(f64, f64)> {
    let (a, e) = (width / 2.0 - inset, height / 2.0 - inset);
    if a <= 0.0 || e <= 0.0 {
        return vec![(0.0, 0.0); count.min(1)];
    }
    let total = 4.0 * (a + e);
    (0..count)
        .map(|k| {
            let s = total * k as f64 / count as f64;
            let (bottom, right, top) = (2.0 * a, 2.0 * a + 2.0 * e, 4.0 * a + 2.0 * e);
            if s < bottom {
                (-a + s, -e)
            } else if s < right {
                (a, -e + (s - bottom))
            } else if s < top {
                (a - (s - right), e)
            } else {
                (-a, e - (s - top))
            }
        })
        .collect()
}

/// 📐️ Bar centres of every layer for a section `width` wide (the plate width for slabs).
pub fn bar_layout(member: &RcMember, width: f64) -> Vec<LayerBars> {
    let link = link_diameter(member);
    let upright = class_of(member.kind) == Class::Upright;
    let (mut rows_top, mut rows_bottom) = (0usize, 0usize);
    member
        .longitudinal
        .iter()
        .enumerate()
        .map(|(layer, l)| {
            let own: Vec<_> = member.bars.iter().filter(|b| b.layer_id == l.id).collect();
            if !own.is_empty() {
                return LayerBars { layer, points: own.iter().map(|b| member.bar_position(b)).collect(), diameters: own.iter().map(|b| b.diameter).collect(), explicit: true };
            }
            let inset = member.cover + link + l.diameter / 2.0;
            let count = l.count as usize;
            if upright {
                let points = perimeter(width, member.height, inset, count);
                return LayerBars { layer, diameters: vec![l.diameter; points.len()], points, explicit: false };
            }
            let top = is_top(l.position);
            let gap = l.diameter.max(MIN_GAP);
            let capacity = (((width - 2.0 * inset + gap) / (l.diameter + gap)).floor() as usize).max(1);
            let used = if top { &mut rows_top } else { &mut rows_bottom };
            let first = *used;
            *used += count.div_ceil(capacity);
            let sign = if top { 1.0 } else { -1.0 };
            let points = (0..count)
                .map(|k| {
                    let (row, col) = (k / capacity, k % capacity);
                    let in_row = (count - row * capacity).min(capacity);
                    let y = if in_row == 1 { 0.0 } else { -width / 2.0 + inset + (width - 2.0 * inset) * col as f64 / (in_row - 1) as f64 };
                    (y, sign * (member.height / 2.0 - inset - (first + row) as f64 * (l.diameter + gap)))
                })
                .collect();
            LayerBars { layer, diameters: vec![l.diameter; points.len()], points, explicit: false }
        })
        .collect()
}

/// 📏️ Assumed stock length of straight bars (m) after which a bar is lapped; an unverified modelling assumption of the detailing view, not a normative value (EN 1992-1-1 8.7 gives only the lap lengths).
pub const STOCK_LENGTH: f64 = 12.0;

/// 🪝️ Standard hook of a bar: bend angle, centre-line bend radius and straight leg (m), after EN 1992-1-1 8.3 Table 8.1N (mandrel 4 phi up to 16 mm, 7 phi above) and Figure 8.1 (hook 150 deg with a 5 phi leg); the values are quoted from memory and unverified.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Hook {
    pub angle: f64,
    pub radius: f64,
    pub leg: f64,
}

pub fn hook(diameter: f64) -> Hook {
    let mandrel = if diameter <= 0.016 { 4.0 * diameter } else { 7.0 * diameter };
    Hook { angle: 5.0 * std::f64::consts::PI / 6.0, radius: (mandrel + diameter) / 2.0, leg: 5.0 * diameter }
}

/// ✂️ The straight pieces `(from, to)` along the axis of a bar from `from` to `to` that is lapped every `stock` metres with the lap length `lap` (EN 1992-1-1 8.7.3); one piece when the bar fits the stock length or has no lap.
pub fn lap_spans(from: f64, to: f64, stock: f64, lap: f64) -> Vec<(f64, f64)> {
    if lap <= 0.0 || stock <= lap || to - from <= stock {
        return vec![(from, to)];
    }
    let mut pieces = Vec::new();
    let mut start = from;
    loop {
        let end = (start + stock).min(to);
        pieces.push((start, end));
        if end >= to - 1e-9 {
            return pieces;
        }
        start = end - lap;
    }
}

/// 🧭️ The side the tessellator bends a rebar towards, `perpendicular` of its direction: the cross product with the world X (or Y for axes within 25 deg of X).
pub fn bend_normal(axis: &Vec3) -> Vec3 {
    let length = axis.length().max(1e-12);
    let t = Vec3::new(axis.x / length, axis.y / length, axis.z / length);
    let helper = if t.x.abs() < 0.9 { Vec3::X } else { Vec3::Y };
    let n = t.cross(&helper);
    let l = n.length().max(1e-12);
    Vec3::new(n.x / l, n.y / l, n.z / l)
}

/// 🪝️ The end of a bar along `axis` whose hook turns towards `inward` (into the section): the far end when the tessellator's bend side points inward, the near end of the reversed bar when it points outward, `None` when the bend side is across the section.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HookEnd {
    Far,
    Near,
    Neither,
}

pub fn hook_end(axis: &Vec3, inward: &Vec3) -> HookEnd {
    let side = bend_normal(axis).dot(inward);
    if side > 0.3 {
        HookEnd::Far
    } else if side < -0.3 {
        HookEnd::Near
    } else {
        HookEnd::Neither
    }
}

/// 🪝️ How the persisted `bend` text of an explicit bar shapes its ends: `Auto` keeps the derived rule (a hook only where anchorage fails), `Straight` none, `Hook` one hook at the end bending into the section, `UHook` both ends.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BendMode {
    Auto,
    Straight,
    Hook,
    UHook,
}

pub fn bend_mode(bend: BarBend) -> BendMode {
    match bend {
        BarBend::Auto => BendMode::Auto,
        BarBend::Straight => BendMode::Straight,
        BarBend::Hook => BendMode::Hook,
        BarBend::UHook => BendMode::UHook,
    }
}

/// 📏️ The span `(from, to)` of an explicit bar of persisted `length` centred in `[from, to]` and clipped to it; `None` for a length of zero or less, so the derived stock and lap pieces apply.
pub fn centred_span(from: f64, to: f64, length: f64) -> Option<(f64, f64)> {
    (length > 0.0).then(|| {
        let used = length.min(to - from).max(0.0);
        let start = (from + to - used) / 2.0;
        (start, start + used)
    })
}

/// 🪝️ The hook layout of one bar `(end carrying the hook, hooks at both ends)`: `natural` is the end the derived rule picks (anchorage fails), `side` the end whose bend side points into the section (a forced hook falls back to the far end when the bend side is across the section).
pub fn bar_hooks(mode: BendMode, natural: HookEnd, side: HookEnd, single_piece: bool) -> (HookEnd, bool) {
    let forced = if side == HookEnd::Neither { HookEnd::Far } else { side };
    match mode {
        BendMode::Auto => (natural, single_piece),
        BendMode::Straight => (HookEnd::Neither, false),
        BendMode::Hook => (forced, false),
        BendMode::UHook => (forced, true),
    }
}

/// 📐️ A closed polygon moved inward by `distance` (miter joins, long spikes cut back): the link centre line of a section outline at cover plus half the link diameter. Returns the input for fewer than three points.
pub fn offset_inward(points: &[(f64, f64)], distance: f64) -> Vec<(f64, f64)> {
    let n = points.len();
    if n < 3 {
        return points.to_vec();
    }
    let area: f64 = (0..n).map(|i| points[i].0 * points[(i + 1) % n].1 - points[(i + 1) % n].0 * points[i].1).sum();
    let side = if area >= 0.0 { 1.0 } else { -1.0 };
    let normal = |a: (f64, f64), b: (f64, f64)| {
        let (dx, dy) = (b.0 - a.0, b.1 - a.1);
        let l = dx.hypot(dy).max(1e-12);
        (-dy / l * side, dx / l * side)
    };
    (0..n)
        .map(|i| {
            let (prev, cur, next) = (points[(i + n - 1) % n], points[i], points[(i + 1) % n]);
            let (n1, n2) = (normal(prev, cur), normal(cur, next));
            let denominator = 1.0 + n1.0 * n2.0 + n1.1 * n2.1;
            let (mx, my) = if denominator < 0.2 { (n1.0, n1.1) } else { ((n1.0 + n2.0) / denominator, (n1.1 + n2.1) / denominator) };
            (cur.0 + mx * distance, cur.1 + my * distance)
        })
        .collect()
}
