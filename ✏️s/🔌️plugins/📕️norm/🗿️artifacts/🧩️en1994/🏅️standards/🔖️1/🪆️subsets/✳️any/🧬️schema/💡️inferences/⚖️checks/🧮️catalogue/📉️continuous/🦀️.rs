//! 📉️ Continuous composite beams: flexibility-method statics of end, interior and cantilever spans with the cracked stiffness EI₂ over a fraction of the span at internal supports (EN 1994-1-1, 5.4.2.3), the resulting deflection curve and the station profiles of the checks.

use super::constants::E_A;
use super::SpanPosition;
use semio_s_plugin_norm_mechanics as mech;

const CELLS: usize = 400;
const EPS: f64 = 1e-9;
const REFERENCE_I: f64 = 1e-4;

/// 📏️ Spans of one continuous beam from the left: lengths, the index of the span under design and whether the last span is a free cantilever.
#[derive(Clone, Debug, PartialEq)]
pub struct Layout {
    pub lengths: Vec<f64>,
    pub own: usize,
    pub cantilever: bool,
}

impl Layout {
    /// 📏️ Layout of the designed span `span_m` at `position` with the neighbouring span `adjacent_m` (0 = equal span): simple `[L]`, end `[L, L_2]`, interior `[L_2, L, L_2]`, cantilever `[L_2, L]`.
    pub fn of(position: SpanPosition, span_m: f64, adjacent_m: f64) -> Self {
        let adjacent = if adjacent_m > EPS { adjacent_m } else { span_m };
        match position {
            SpanPosition::Simple => Self { lengths: vec![span_m], own: 0, cantilever: false },
            SpanPosition::End => Self { lengths: vec![span_m, adjacent], own: 0, cantilever: false },
            SpanPosition::Interior => Self { lengths: vec![adjacent, span_m, adjacent], own: 1, cantilever: false },
            SpanPosition::Cantilever => Self { lengths: vec![adjacent, span_m], own: 1, cantilever: true },
        }
    }

    /// 📏️ Total length of all spans.
    pub fn total(&self) -> f64 {
        self.lengths.iter().sum()
    }

    /// 📍️ Positions of all supports from the left, the free end of a cantilever excluded.
    pub fn supports(&self) -> Vec<f64> {
        let mut x = 0.0;
        let mut out = vec![0.0];
        for (index, l) in self.lengths.iter().enumerate() {
            x += l;
            if !(self.cantilever && index + 1 == self.lengths.len()) {
                out.push(x);
            }
        }
        out
    }

    /// 📍️ Left end of span `index` from the beam start.
    pub fn origin(&self, index: usize) -> f64 {
        self.lengths[..index].iter().sum()
    }

    /// 🔢️ Index of the span containing the beam position `x` and the span-local position.
    pub fn locate(&self, x: f64) -> (usize, f64) {
        let mut origin = 0.0;
        for (index, l) in self.lengths.iter().enumerate() {
            if x <= origin + l + EPS || index + 1 == self.lengths.len() {
                return (index, (x - origin).clamp(0.0, *l));
            }
            origin += l;
        }
        (0, 0.0)
    }

    fn internal_left(&self, span: usize) -> bool {
        span > 0
    }

    fn internal_right(&self, span: usize) -> bool {
        span + 1 < self.lengths.len()
    }
}

/// 🧱️ Bending stiffness: uncracked EI₁ and cracked EI₂ over `cracked_fraction` of a span at each internal support (5.4.2.3(3), recommended 0.15 of the span).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Stiffness {
    pub ei1: f64,
    pub ei2: f64,
    pub cracked_fraction: f64,
}

impl Stiffness {
    /// 🧱️ Constant stiffness over the whole beam.
    pub fn uniform(ei: f64) -> Self {
        Self { ei1: ei, ei2: ei, cracked_fraction: 0.0 }
    }

    fn at(&self, layout: &Layout, span: usize, x: f64) -> f64 {
        let l = layout.lengths[span];
        let zone = self.cracked_fraction * l;
        let cracked = (layout.internal_left(span) && x < zone - 1e-12) || (layout.internal_right(span) && x > l - zone + 1e-12);
        if cracked { self.ei2 } else { self.ei1 }
    }
}

/// 📉️ Support moments of a beam under uniform loads per span (sagging positive, hogging negative) with the integrals needed for deflections.
#[derive(Clone, Debug)]
pub struct Solution {
    pub layout: Layout,
    pub loads: Vec<f64>,
    pub stiffness: Stiffness,
    pub supports: Vec<f64>,
}

struct Cells {
    h: f64,
    xs: Vec<f64>,
    kappa: Vec<f64>,
}

impl Solution {
    fn cells(&self, span: usize, m_left: f64, m_right: f64) -> Cells {
        let l = self.layout.lengths[span];
        let h = l / CELLS as f64;
        let xs: Vec<f64> = (0..CELLS).map(|k| (k as f64 + 0.5) * h).collect();
        let kappa = xs.iter().map(|&x| moment_of(l, self.loads[span], m_left, m_right, x) / self.stiffness.at(&self.layout, span, x)).collect();
        Cells { h, xs, kappa }
    }

    fn left_slope(cells: &Cells, l: f64) -> f64 {
        -cells.xs.iter().zip(&cells.kappa).map(|(x, k)| (l - x) * k).sum::<f64>() * cells.h / l
    }

    fn turn(cells: &Cells) -> f64 {
        cells.kappa.iter().sum::<f64>() * cells.h
    }

    /// 📐️ Moment [N·m] of span `span` at the span-local position `x`.
    pub fn moment(&self, span: usize, x: f64) -> f64 {
        moment_of(self.layout.lengths[span], self.loads[span], self.supports[span], self.supports[span + 1], x)
    }

    /// 📐️ Shear force [N] of span `span` at the span-local position `x`.
    pub fn shear(&self, span: usize, x: f64) -> f64 {
        let l = self.layout.lengths[span];
        self.loads[span] * (l / 2.0 - x) + (self.supports[span + 1] - self.supports[span]) / l
    }

    /// 📐️ Moment at the beam position `x`.
    pub fn moment_at(&self, x: f64) -> f64 {
        let (span, local) = self.layout.locate(x);
        self.moment(span, local)
    }

    /// 📐️ Shear force at the beam position `x`.
    pub fn shear_at(&self, x: f64) -> f64 {
        let (span, local) = self.layout.locate(x);
        self.shear(span, local)
    }

    fn scan<F: Fn(usize, f64) -> f64>(&self, f: F) -> Vec<f64> {
        let mut out = Vec::new();
        for (span, l) in self.layout.lengths.iter().enumerate() {
            out.extend((0..=CELLS / 4).map(|k| f(span, l * k as f64 / (CELLS / 4) as f64)));
        }
        out
    }

    /// 📈️ Largest sagging moment of the designed span.
    pub fn max_sagging_own(&self) -> f64 {
        let l = self.layout.lengths[self.layout.own];
        (0..=CELLS).map(|k| self.moment(self.layout.own, l * k as f64 / CELLS as f64)).fold(0.0, f64::max)
    }

    /// 📈️ Largest hogging moment magnitude at the supports of the designed span.
    pub fn max_hogging_own(&self) -> f64 {
        let own = self.layout.own;
        self.supports[own].min(self.supports[own + 1]).min(0.0).abs()
    }

    /// 📈️ Largest shear force magnitude of the designed span.
    pub fn max_shear_own(&self) -> f64 {
        let l = self.layout.lengths[self.layout.own];
        (0..=CELLS).map(|k| self.shear(self.layout.own, l * k as f64 / CELLS as f64).abs()).fold(0.0, f64::max)
    }

    /// 🌀️ Downward deflection curve `(x, w)` over all spans every `CELLS / points` cells, `w` positive downward.
    pub fn deflection(&self, points: usize) -> Vec<(f64, f64)> {
        let stride = (CELLS / points.clamp(1, CELLS)).max(1);
        let mut out = Vec::new();
        let mut slope_in = 0.0;
        for span in 0..self.layout.lengths.len() {
            let l = self.layout.lengths[span];
            let cells = self.cells(span, self.supports[span], self.supports[span + 1]);
            let free = self.layout.cantilever && span + 1 == self.layout.lengths.len();
            let theta = if free { slope_in } else { Self::left_slope(&cells, l) };
            let (mut a, mut b) = (0.0, 0.0);
            let origin = self.layout.origin(span);
            for j in 0..=CELLS {
                if j % stride == 0 || j == CELLS {
                    let x = j as f64 * cells.h;
                    out.push((origin + x, -(theta * x + x * a - b)));
                }
                if j < CELLS {
                    a += cells.kappa[j] * cells.h;
                    b += cells.kappa[j] * cells.xs[j] * cells.h;
                }
            }
            slope_in = theta + Self::turn(&cells);
        }
        out
    }

    /// 🌀️ Signed largest downward deflection of the designed span (negative = upward, e.g. a cantilever tip lifted by loaded neighbours).
    pub fn deflection_own(&self) -> f64 {
        let (from, to) = (self.layout.origin(self.layout.own), self.layout.origin(self.layout.own) + self.layout.lengths[self.layout.own]);
        self.deflection(CELLS).into_iter().filter(|(x, _)| *x >= from - EPS && *x <= to + EPS).map(|p| p.1).fold(0.0, |best: f64, w| if w.abs() > best.abs() { w } else { best })
    }

    /// 〰️ Beam intervals `(x_from, x_to)` of hogging moment around the supports, from the contraflexure points.
    pub fn hogging_zones(&self) -> Vec<(f64, f64)> {
        let samples = self.scan(|span, x| self.moment(span, x));
        let xs: Vec<f64> = self.layout.lengths.iter().enumerate().flat_map(|(span, l)| (0..=CELLS / 4).map(move |k| (span, l * k as f64 / (CELLS / 4) as f64))).map(|(span, x)| self.layout.origin(span) + x).collect();
        let mut zones = Vec::new();
        let mut start: Option<f64> = None;
        let threshold = -1e-6 * samples.iter().fold(0.0_f64, |m, v| m.max(v.abs())).max(1.0);
        for i in 0..samples.len() {
            let hogging = samples[i] < threshold;
            match (hogging, start) {
                (true, None) => start = Some(if i > 0 { cross(xs[i - 1], samples[i - 1], xs[i], samples[i]) } else { xs[i] }),
                (false, Some(from)) => {
                    zones.push((from, cross(xs[i - 1], samples[i - 1], xs[i], samples[i])));
                    start = None;
                }
                _ => {}
            }
        }
        if let Some(from) = start {
            zones.push((from, *xs.last().unwrap_or(&from)));
        }
        zones
    }
}

fn cross(x0: f64, v0: f64, x1: f64, v1: f64) -> f64 {
    if (v1 - v0).abs() < 1e-12 { x0 } else { x0 + (0.0 - v0) * (x1 - x0) / (v1 - v0) }
}

fn moment_of(l: f64, q: f64, m_left: f64, m_right: f64, x: f64) -> f64 {
    q * x * (l - x) / 2.0 + m_left * (1.0 - x / l) + m_right * x / l
}

fn solve_linear(mut a: Vec<Vec<f64>>, mut b: Vec<f64>) -> Vec<f64> {
    let n = b.len();
    for col in 0..n {
        let pivot = (col..n).max_by(|&i, &j| a[i][col].abs().total_cmp(&a[j][col].abs())).unwrap_or(col);
        a.swap(col, pivot);
        b.swap(col, pivot);
        let d = a[col][col];
        if d.abs() < 1e-300 {
            continue;
        }
        for row in col + 1..n {
            let f = a[row][col] / d;
            for k in col..n {
                a[row][k] -= f * a[col][k];
            }
            b[row] -= f * b[col];
        }
    }
    let mut x = vec![0.0; n];
    for row in (0..n).rev() {
        let tail: f64 = (row + 1..n).map(|k| a[row][k] * x[k]).sum();
        x[row] = if a[row][row].abs() < 1e-300 { 0.0 } else { (b[row] - tail) / a[row][row] };
    }
    x
}

/// 📉️ Solves the continuous beam of `layout` under the uniform span loads `loads` [N/m] (downward positive) with the stiffness `stiffness` by the compatibility of the rotations at the internal supports (three-moment equation for variable EI); the cantilever moment is statically determinate.
pub fn solve(layout: &Layout, loads: &[f64], stiffness: Stiffness) -> Solution {
    let k = layout.lengths.len();
    let mut solution = Solution { layout: layout.clone(), loads: loads.to_vec(), stiffness, supports: vec![0.0; k + 1] };
    if layout.cantilever {
        let l = layout.lengths[k - 1];
        solution.supports[k - 1] = -loads[k - 1] * l * l / 2.0;
    }
    let unknown: Vec<usize> = (1..k).filter(|&i| !(layout.cantilever && i == k - 1)).collect();
    if unknown.is_empty() {
        return solution;
    }
    let residual = |trial: &[f64]| -> Vec<f64> {
        let mut s = solution.clone();
        for (slot, &i) in unknown.iter().enumerate() {
            s.supports[i] = trial[slot];
        }
        unknown
            .iter()
            .map(|&i| {
                let left = s.cells(i - 1, s.supports[i - 1], s.supports[i]);
                let right = s.cells(i, s.supports[i], s.supports[i + 1]);
                let t_right = Solution::left_slope(&left, layout.lengths[i - 1]) + Solution::turn(&left);
                t_right - Solution::left_slope(&right, layout.lengths[i])
            })
            .collect()
    };
    let n = unknown.len();
    let base = residual(&vec![0.0; n]);
    let mut matrix = vec![vec![0.0; n]; n];
    for j in 0..n {
        let mut unit = vec![0.0; n];
        unit[j] = 1.0;
        let r = residual(&unit);
        for i in 0..n {
            matrix[i][j] = r[i] - base[i];
        }
    }
    let moments = solve_linear(matrix, base.iter().map(|r| -r).collect());
    for (slot, &i) in unknown.iter().enumerate() {
        solution.supports[i] = moments[slot];
    }
    solution
}

/// 🏋️ One load of an action on a span: a uniform line load in N/m or a concentrated force in N at the fraction `xi` of the span from its first support.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Load {
    Udl(f64),
    Point(f64, f64),
}

impl Load {
    /// 🧮️ The load times `factor`.
    pub fn scaled(self, factor: f64) -> Self {
        match self {
            Load::Udl(q) => Load::Udl(q * factor),
            Load::Point(p, xi) => Load::Point(p * factor, xi),
        }
    }

    /// 🧮️ Resultant of the load on a span of length `l`.
    pub fn total(self, l: f64) -> f64 {
        match self {
            Load::Udl(q) => q * l,
            Load::Point(p, _) => p,
        }
    }

    /// 🧮️ Largest support shear of the load on a simply supported span.
    pub fn simple_shear(self, l: f64) -> f64 {
        match self {
            Load::Udl(q) => q * l / 2.0,
            Load::Point(p, xi) => p * xi.max(1.0 - xi),
        }
    }

    /// 🧮️ Largest sagging moment of the load on a simply supported span.
    pub fn simple_sagging(self, l: f64) -> f64 {
        match self {
            Load::Udl(q) => q * l * l / 8.0,
            Load::Point(p, xi) => p * xi * (1.0 - xi) * l,
        }
    }
}

/// 📊️ Extreme internal forces of the designed span: sagging and hogging moment magnitudes in N·m and the largest shear force magnitude in N.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Internals {
    pub sagging: f64,
    pub hogging: f64,
    pub shear: f64,
}

/// 🌉️ Spans carrying the action for the sagging of the designed span and for the hogging at its governing support: a permanent action loads every span, a variable action the designed span alone resp. the two spans next to the first internal support (EN 1990 6.4.3.2 arrangement).
fn arrangements(layout: &Layout, permanent: bool) -> (Vec<bool>, Vec<bool>) {
    let count = layout.lengths.len();
    if permanent {
        return (vec![true; count], vec![true; count]);
    }
    let support = if layout.own == 0 { 1 } else { layout.own };
    ((0..count).map(|i| i == layout.own).collect(), (0..count).map(|i| i + 1 == support || i == support).collect())
}

struct Mesh {
    xs: Vec<f64>,
    points: Vec<(f64, f64)>,
}

fn mesh(layout: &Layout, stiffness: &Stiffness, loads: &[Vec<Load>]) -> Mesh {
    let mut xs = vec![0.0];
    let mut points = Vec::new();
    for (span, l) in layout.lengths.iter().enumerate() {
        let origin = layout.origin(span);
        let zone = stiffness.cracked_fraction * l;
        let mut cuts = vec![*l];
        if stiffness.ei2 < stiffness.ei1 && zone > EPS {
            cuts.extend([zone.min(*l), (l - zone).max(0.0)]);
        }
        for load in &loads[span] {
            if let Load::Point(p, xi) = load {
                let at = xi.clamp(0.0, 1.0) * l;
                cuts.push(at);
                points.push((origin + at, *p));
            }
        }
        cuts.sort_by(f64::total_cmp);
        for cut in cuts {
            if origin + cut > xs.last().copied().unwrap_or(0.0) + EPS {
                xs.push(origin + cut);
            }
        }
    }
    Mesh { xs, points }
}

fn solve_beam(layout: &Layout, stiffness: &Stiffness, loads: &[Vec<Load>]) -> mech::MechanicsResult<(Mesh, mech::StaticResult)> {
    let mesh = mesh(layout, stiffness, loads);
    let nodes: Vec<mech::Node> = mesh.xs.iter().enumerate().map(|(k, x)| mech::Node { id: format!("n{k}"), x: *x, y: 0.0, z: 0.0 }).collect();
    let node_at = |x: f64| mesh.xs.iter().position(|v| (v - x).abs() <= EPS).unwrap_or(0);
    let middle = |k: usize| 0.5 * (mesh.xs[k] + mesh.xs[k + 1]);
    let ei_of = |k: usize| {
        let (span, local) = layout.locate(middle(k));
        stiffness.at(layout, span, local)
    };
    let members: Vec<mech::Member> = (0..mesh.xs.len() - 1).map(|k| mech::Member { id: format!("m{k}"), node_i: format!("n{k}"), node_j: format!("n{}", k + 1), material: "S".into(), section: if ei_of(k) < stiffness.ei1 { "2" } else { "1" }.into(), ..Default::default() }).collect();
    let section = |id: &str, ei: f64| {
        let i = REFERENCE_I * ei / stiffness.ei1;
        mech::Section { id: id.into(), a: 1e-2, iy: i, iz: i, j: i, ..Default::default() }
    };
    let supports: Vec<mech::Support> = layout.supports().iter().enumerate().map(|(k, x)| mech::Support { node: format!("n{}", node_at(*x)), fixed: [k == 0, true, true, k == 0, false, false], ..Default::default() }).collect();
    let distributed = members
        .iter()
        .enumerate()
        .filter_map(|(k, member)| {
            let q: f64 = loads[layout.locate(middle(k)).0].iter().map(|load| if let Load::Udl(q) = load { *q } else { 0.0 }).sum();
            (q != 0.0).then(|| mech::DistributedLoad { member: member.id.clone(), frame: mech::LoadFrame::Global, start: [0.0, 0.0, -q], end: [0.0, 0.0, -q] })
        })
        .collect();
    let nodal = mesh.points.iter().map(|(x, p)| mech::NodalLoad { node: format!("n{}", node_at(*x)), force: [0.0, 0.0, -p, 0.0, 0.0, 0.0] }).collect();
    let model = mech::Model {
        nodes,
        materials: vec![mech::Material { id: "S".into(), e: E_A, g: E_A / 2.6, rho: 0.0 }],
        sections: vec![section("1", stiffness.ei1), section("2", stiffness.ei2.min(stiffness.ei1))],
        members,
        supports,
        load_cases: vec![mech::LoadCase { id: "action".into(), nodal, distributed, ..Default::default() }],
        ..Default::default()
    };
    let control = mech::Control::none();
    let mut analysis = mech::Analysis::new(&model, &control)?;
    analysis.stations = 3;
    let result = analysis.solve_case(&model.load_cases[0], &control)?;
    Ok((mesh, result))
}

/// 🧮️ Internal forces of the designed span of `layout` under `loads` (the loads of each span) from the linear statics of the mechanics kernel: pinned supports at the layout supports, member stiffness EI₁ resp. EI₂ in the cracked zones, exact diagram extremes from three stations per member; only the stiffness ratio enters the forces, so the section is scaled to a well-conditioned reference inertia.
fn envelope(layout: &Layout, stiffness: &Stiffness, loads: &[Vec<Load>]) -> mech::MechanicsResult<Internals> {
    let (mesh, result) = solve_beam(layout, stiffness, loads)?;
    let (from, to) = (layout.origin(layout.own), layout.origin(layout.own) + layout.lengths[layout.own]);
    let (mut sagging, mut hogging, mut shear, mut peak) = (0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64);
    for (k, member) in result.members.iter().enumerate() {
        if mesh.xs[k] < from - EPS || mesh.xs[k + 1] > to + EPS {
            continue;
        }
        let (m0, mh, m1) = (member.diagram[0].my, member.diagram[1].my, member.diagram[2].my);
        let l = member.length;
        let c = 2.0 * (m0 - 2.0 * mh + m1) / (l * l);
        let b = (m1 - m0) / l - c * l;
        let vertex = if c.abs() > 1e-12 * m0.abs().max(m1.abs()).max(1.0) / (l * l) { -b / (2.0 * c) } else { -1.0 };
        let extreme = if vertex > 0.0 && vertex < l { m0 + b * vertex + c * vertex * vertex } else { m0 };
        for m in [m0, m1, extreme] {
            sagging = sagging.max(-m);
            hogging = hogging.max(m);
            peak = peak.max(m.abs());
        }
        shear = member.diagram.iter().fold(shear, |s, point| s.max(point.vz.abs()));
    }
    let snap = |v: f64| if v < 1e-9 * peak { 0.0 } else { v };
    Ok(Internals { sagging: snap(sagging), hogging: snap(hogging), shear: snap(shear) })
}

/// 📈️ Shear force diagram `(x, V)` along the whole beam under `loads` (one list per span) by the mechanics statics: two points per member, so a jump at a concentrated force or a support appears as two points at the same `x`. Empty for a zero-length span.
pub fn shear_diagram(position: SpanPosition, span_m: f64, adjacent_m: f64, loads: &[Vec<Load>], stiffness: Stiffness) -> Vec<(f64, f64)> {
    let layout = Layout::of(position, span_m, adjacent_m);
    if layout.lengths.iter().any(|l| *l <= EPS) || loads.len() != layout.lengths.len() {
        return Vec::new();
    }
    let Ok((mesh, result)) = solve_beam(&layout, &stiffness, loads) else {
        return Vec::new();
    };
    result.members.iter().enumerate().flat_map(|(k, member)| [(mesh.xs[k], member.diagram[0].vz), (mesh.xs[k + 1], member.diagram[2].vz)]).collect()
}

/// 📈️ Shear force at the beam position `x` of a diagram from [`shear_diagram`] (linear between points, the later value at a jump), 0 for an empty diagram.
pub fn shear_at(diagram: &[(f64, f64)], x: f64) -> f64 {
    let segment = diagram.windows(2).rev().find(|w| w[1].0 - w[0].0 > EPS && x >= w[0].0 - EPS && x <= w[1].0 + EPS);
    match segment {
        Some(w) => w[0].1 + (w[1].1 - w[0].1) * ((x - w[0].0) / (w[1].0 - w[0].0)).clamp(0.0, 1.0),
        None => diagram.last().map_or(0.0, |p| p.1),
    }
}

/// 📐️ Extreme internal forces of the designed span of a continuous beam (end, interior or cantilever span, neighbour `adjacent_m`, 0 = equal) under one action `load` at its position, by the shared mechanics statics with the stiffness `stiffness` (uniform = elastic uncracked, EI₂ in the cracked zones = cl. 5.4.2.3 global analysis). The neighbouring spans carry the same load at the same relative position when the arrangement of EN 1990 6.4.3.2 loads them; a zero-length span carries no force.
pub fn action_internals(position: SpanPosition, span_m: f64, adjacent_m: f64, load: Load, permanent: bool, stiffness: Stiffness) -> Internals {
    let layout = Layout::of(position, span_m, adjacent_m);
    if layout.lengths.iter().any(|l| *l <= EPS) {
        return Internals::default();
    }
    let loaded = |spans: &[bool]| -> Vec<Vec<Load>> { spans.iter().map(|on| if *on { vec![load] } else { Vec::new() }).collect() };
    let (sag_spans, hog_spans) = arrangements(&layout, permanent);
    let sagging = envelope(&layout, &stiffness, &loaded(&sag_spans)).unwrap_or_default();
    let hogging = if permanent { sagging } else { envelope(&layout, &stiffness, &loaded(&hog_spans)).unwrap_or_default() };
    Internals { sagging: sagging.sagging, hogging: hogging.hogging, shear: sagging.shear.max(hogging.shear) }
}

/// 🪜️ Factored extreme internal forces of the designed span under a permanent action: the action is individual per span (EN 1990 A1.3.1), so every arrangement of the shared `permanent_arrangements` (γG,sup on the unfavourable spans, γG,inf on the others) is solved and the largest sagging, hogging and shear are taken.
pub fn permanent_internals(position: SpanPosition, span_m: f64, adjacent_m: f64, load: Load, stiffness: Stiffness, factors: &mech::SpanFactors) -> Internals {
    let layout = Layout::of(position, span_m, adjacent_m);
    if layout.lengths.iter().any(|l| *l <= EPS) {
        return Internals::default();
    }
    mech::permanent_arrangements(layout.lengths.len(), factors).iter().fold(Internals::default(), |peak, factor| {
        let loads: Vec<Vec<Load>> = factor.iter().map(|f| vec![load.scaled(*f)]).collect();
        let found = envelope(&layout, &stiffness, &loads).unwrap_or_default();
        Internals { sagging: peak.sagging.max(found.sagging), hogging: peak.hogging.max(found.hogging), shear: peak.shear.max(found.shear) }
    })
}

/// 📉️ Loading pattern of a deflection component: every span loaded (permanent) or the designed span only (variable, the unfavourable arrangement for its own deflection).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pattern {
    AllSpans,
    OwnSpan,
}

/// 🌀️ Signed downward deflection [m] of the designed span under the uniform load `q` [N/m] in `pattern`.
pub fn span_deflection_m(position: SpanPosition, span_m: f64, adjacent_m: f64, q: f64, pattern: Pattern, stiffness: Stiffness) -> f64 {
    let layout = Layout::of(position, span_m, adjacent_m);
    let loads: Vec<f64> = (0..layout.lengths.len()).map(|i| if pattern == Pattern::AllSpans || i == layout.own { q } else { 0.0 }).collect();
    solve(&layout, &loads, stiffness).deflection_own()
}

/// 📉️ Ratios cracked/uncracked of the largest hogging moment (all spans loaded) and of the largest sagging moment (designed span loaded) of the designed span, for the global-analysis check of cl. 5.4.2.3.
pub fn cracking_ratios(position: SpanPosition, span_m: f64, adjacent_m: f64, cracked: Stiffness, uncracked: Stiffness) -> (f64, f64) {
    let layout = Layout::of(position, span_m, adjacent_m);
    let loads = |pattern: Pattern| -> Vec<f64> { (0..layout.lengths.len()).map(|i| if pattern == Pattern::AllSpans || i == layout.own { 1.0 } else { 0.0 }).collect() };
    let ratio = |cr: f64, un: f64| if un.abs() > EPS { cr / un } else { 1.0 };
    let (hog_cr, hog_un) = (solve(&layout, &loads(Pattern::AllSpans), cracked).max_hogging_own(), solve(&layout, &loads(Pattern::AllSpans), uncracked).max_hogging_own());
    let (sag_cr, sag_un) = (solve(&layout, &loads(Pattern::OwnSpan), cracked).max_sagging_own(), solve(&layout, &loads(Pattern::OwnSpan), uncracked).max_sagging_own());
    (ratio(hog_cr, hog_un), ratio(sag_cr, sag_un))
}

/// 📈️ Relative internal forces along the whole beam for the stations of the checks, normalised by the governing values of the designed span.
#[derive(Clone, Debug)]
pub struct Profile {
    solution: Solution,
    sagging: f64,
    hogging: f64,
    shear: f64,
}

impl Profile {
    /// 📈️ Profile of the all-spans-loaded uncracked statics of the layout.
    pub fn of(position: SpanPosition, span_m: f64, adjacent_m: f64) -> Self {
        let layout = Layout::of(position, span_m, adjacent_m);
        let solution = solve(&layout, &vec![1.0; layout.lengths.len()], Stiffness::uniform(1.0));
        let sagging = solution.max_sagging_own().max(EPS);
        let hogging = solution.supports.iter().fold(0.0_f64, |m, v| m.max(-v)).max(EPS);
        let shear = (0..solution.layout.lengths.len()).map(|span| solution.shear(span, 0.0).abs().max(solution.shear(span, solution.layout.lengths[span]).abs())).fold(0.0, f64::max).max(EPS);
        Self { solution, sagging, hogging, shear }
    }

    /// 📏️ Total length of the profile in metres.
    pub fn total(&self) -> f64 {
        self.solution.layout.total()
    }

    /// 📈️ Relative sagging moment at `s ∈ [0, 1]` of the total length (1 at the largest sagging moment of the designed span).
    pub fn sagging(&self, s: f64) -> f64 {
        (self.solution.moment_at(s.clamp(0.0, 1.0) * self.total()) / self.sagging).max(0.0)
    }

    /// 📈️ Relative hogging moment at `s` (1 at the largest support moment).
    pub fn hogging(&self, s: f64) -> f64 {
        (-self.solution.moment_at(s.clamp(0.0, 1.0) * self.total()) / self.hogging).max(0.0)
    }

    /// 📈️ Relative shear force magnitude at `s` (1 at the largest shear force).
    pub fn shear(&self, s: f64) -> f64 {
        self.solution.shear_at(s.clamp(0.0, 1.0) * self.total()).abs() / self.shear
    }

    /// 📈️ Signed moment per unit load at `s`, for combined moment-shear envelopes.
    pub fn moment_per_load(&self, s: f64) -> f64 {
        self.solution.moment_at(s.clamp(0.0, 1.0) * self.total())
    }

    /// 📈️ Signed shear force per unit load at `s`.
    pub fn shear_per_load(&self, s: f64) -> f64 {
        self.solution.shear_at(s.clamp(0.0, 1.0) * self.total())
    }
}

/// 🌈️ The crack-width profile of every hogging zone of a continuous beam under a uniform load on all spans: `(x, |M(x)|/|M|max)` at `points` stations per zone, the support moment of the elastic analysis with the cracked stiffness being 1.
pub fn hogging_band_profile(layout: &Layout, stiffness: Stiffness, points: usize) -> Vec<Vec<(f64, f64)>> {
    let solution = solve(layout, &vec![1.0; layout.lengths.len()], stiffness);
    let zones: Vec<Vec<(f64, f64)>> = solution.hogging_zones().into_iter().map(|(a, b)| (0..points).map(|k| a + (b - a) * k as f64 / (points - 1) as f64).map(|x| (x, solution.moment_at(x).abs())).collect()).collect();
    let peak = zones.iter().flatten().map(|p| p.1).fold(0.0, f64::max);
    if peak <= 1e-12 {
        return Vec::new();
    }
    zones.into_iter().map(|zone| zone.into_iter().map(|(x, m)| (x, m / peak)).collect()).collect()
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
