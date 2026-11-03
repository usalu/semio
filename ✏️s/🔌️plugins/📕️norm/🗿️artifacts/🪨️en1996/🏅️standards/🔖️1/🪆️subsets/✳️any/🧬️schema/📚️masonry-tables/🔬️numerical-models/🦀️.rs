//! 🔬️ Numerical models of the EN 1996 catalogue: the Johansen yield-line load of laterally loaded panels and the plane-section M-N capacity of reinforced masonry. See [`super`].

/// 🧱️ Condition of one panel edge: unsupported, simply supported or fully fixed (hogging capacity equal to the sagging capacity).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Edge {
    Free,
    Simple,
    Fixed,
}

impl Edge {
    fn hogging(self) -> f64 {
        if self == Edge::Fixed { 1.0 } else { 0.0 }
    }
}

/// 🧱️ Edge conditions of a laterally loaded panel; horizontal edges are top and bottom, vertical edges left and right.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Edges {
    pub top: Edge,
    pub bottom: Edge,
    pub left: Edge,
    pub right: Edge,
}

impl Edges {
    /// 🔤️ Four letters top, bottom, left, right with `F` free, `S` simple, `X` fixed, e.g. `SSSS`.
    pub fn parse(code: &str) -> Option<Self> {
        let edge = |c: char| match c.to_ascii_uppercase() {
            'F' => Some(Edge::Free),
            'S' => Some(Edge::Simple),
            'X' => Some(Edge::Fixed),
            _ => None,
        };
        let mut chars = code.chars();
        let (top, bottom, left, right) = (edge(chars.next()?)?, edge(chars.next()?)?, edge(chars.next()?)?, edge(chars.next()?)?);
        chars.next().is_none().then_some(Self { top, bottom, left, right })
    }
    fn transposed(self) -> Self {
        Self { top: self.right, bottom: self.left, left: self.bottom, right: self.top }
    }
}

fn minimise(lo: &[f64], hi: &[f64], f: &dyn Fn(&[f64]) -> f64) -> (f64, Vec<f64>) {
    let n = lo.len();
    if n == 0 {
        return (f(&[]), Vec::new());
    }
    let steps = match n { 1 => 400, 2 => 60, _ => 24 };
    let mut best = f64::INFINITY;
    let mut best_x = lo.to_vec();
    let mut idx = vec![0usize; n];
    loop {
        let x: Vec<f64> = (0..n).map(|k| lo[k] + (hi[k] - lo[k]) * idx[k] as f64 / steps as f64).collect();
        let v = f(&x);
        if v < best {
            best = v;
            best_x = x;
        }
        let mut k = 0;
        while k < n {
            idx[k] += 1;
            if idx[k] <= steps { break; }
            idx[k] = 0;
            k += 1;
        }
        if k == n { break; }
    }
    let mut width: Vec<f64> = (0..n).map(|k| (hi[k] - lo[k]) / steps as f64).collect();
    for _ in 0..80 {
        for k in 0..n {
            for dir in [-1.0, 1.0] {
                let mut x = best_x.clone();
                x[k] = (x[k] + dir * width[k]).clamp(lo[k], hi[k]);
                let v = f(&x);
                if v < best {
                    best = v;
                    best_x = x;
                }
            }
            width[k] *= 0.8;
        }
    }
    (best, best_x)
}

/// 🧱️ Collapse pressure and geometry `[ridge height, left offset, right offset]` of the pattern with a horizontal ridge: the ridge height and the corner offsets of the ridge ends are the minimised parameters.
fn ridge_pattern(e: Edges, l: f64, h: f64, m1: f64, m2: f64) -> (f64, [f64; 3]) {
    if e.top == Edge::Free && e.bottom == Edge::Free {
        return (f64::INFINITY, [0.0; 3]);
    }
    let eps = 1e-3;
    let (left_on, right_on) = (e.left != Edge::Free, e.right != Edge::Free);
    let y_free = e.top == Edge::Free || e.bottom == Edge::Free;
    let mut lo = Vec::new();
    let mut hi = Vec::new();
    if !y_free {
        lo.push(eps * h);
        hi.push((1.0 - eps) * h);
    }
    for on in [left_on, right_on] {
        if on {
            lo.push(eps * l);
            hi.push(l);
        }
    }
    let decode = |x: &[f64]| -> [f64; 3] {
        let mut k = 0;
        let y_r = if e.top == Edge::Free { h } else if e.bottom == Edge::Free { 0.0 } else { k += 1; x[0] };
        let a_l = if left_on { k += 1; x[k - 1] } else { 0.0 };
        let a_r = if right_on { k += 1; x[k - 1] } else { 0.0 };
        [y_r, a_l, a_r]
    };
    let f = |x: &[f64]| -> f64 {
        let [y_r, a_l, a_r] = decode(x);
        if a_l + a_r > l {
            return f64::INFINITY;
        }
        let mut work = 0.0;
        if e.bottom != Edge::Free {
            let positive = if e.top != Edge::Free { l } else { a_l + a_r };
            work += m1 * (positive + l * e.bottom.hogging()) / y_r.max(1e-12);
        }
        if e.top != Edge::Free {
            let positive = if e.bottom != Edge::Free { l } else { a_l + a_r };
            work += m1 * (positive + l * e.top.hogging()) / (h - y_r).max(1e-12);
        }
        if left_on {
            work += m2 * h * (1.0 + e.left.hogging()) / a_l;
        }
        if right_on {
            work += m2 * h * (1.0 + e.right.hogging()) / a_r;
        }
        let volume_factor = (l / 2.0 - (a_l + a_r) / 6.0) * h;
        work / volume_factor
    };
    let (pressure, x) = minimise(&lo, &hi, &f);
    (pressure, decode(&x))
}

/// 🧱️ Collapse pressure of the ridge pattern alone, see [`ridge_pattern`].
fn ridge_pressure(e: Edges, l: f64, h: f64, m1: f64, m2: f64) -> f64 {
    ridge_pattern(e, l, h, m1, m2).0
}

/// 🧱️ Johansen yield-line collapse pressure [Pa] of an L × H panel with the given edge conditions; `m1` resists cracks along the bed joints, `m2` vertical cracks [N·m/m], hogging capacity equals sagging capacity on fixed edges.
pub fn yield_line_pressure(edges: Edges, l: f64, h: f64, m1: f64, m2: f64) -> f64 {
    ridge_pressure(edges, l, h, m1, m2).min(ridge_pressure(edges.transposed(), h, l, m2, m1))
}

/// 📐️ The governing yield-line pattern of a panel: collapse pressure, whether the ridge runs vertically (`transposed`) and the ridge position with the corner offsets of its ends, in the frame of the orientation that governs.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct YieldPattern {
    pub pressure: f64,
    pub transposed: bool,
    pub ridge: f64,
    pub left: f64,
    pub right: f64,
}

/// 📐️ The governing pattern of [`yield_line_pressure`] (same minimisation, same pressure).
pub fn yield_line_pattern(edges: Edges, l: f64, h: f64, m1: f64, m2: f64) -> YieldPattern {
    let (p, [ridge, left, right]) = ridge_pattern(edges, l, h, m1, m2);
    let (pt, [ridge_t, left_t, right_t]) = ridge_pattern(edges.transposed(), h, l, m2, m1);
    if pt < p { YieldPattern { pressure: pt, transposed: true, ridge: ridge_t, left: left_t, right: right_t } } else { YieldPattern { pressure: p, transposed: false, ridge, left, right } }
}

impl YieldPattern {
    /// 📐️ The fracture lines in the panel frame (x along the length, y up from the bottom edge): the ridge and the diagonals from its ends to the corners of the supported edges; degenerate lines are dropped.
    pub fn lines(&self, l: f64, h: f64) -> Vec<((f64, f64), (f64, f64))> {
        let (w, v) = if self.transposed { (h, l) } else { (l, h) };
        let (x0, x1) = (self.left, w - self.right);
        let mut out = vec![((x0, self.ridge), (x1, self.ridge))];
        if self.left > 0.0 {
            out.push(((x0, self.ridge), (0.0, 0.0)));
            out.push(((x0, self.ridge), (0.0, v)));
        }
        if self.right > 0.0 {
            out.push(((x1, self.ridge), (w, 0.0)));
            out.push(((x1, self.ridge), (w, v)));
        }
        let keep = |(a, b): &((f64, f64), (f64, f64))| (a.0 - b.0).hypot(a.1 - b.1) > 1e-9;
        let flip = |(x, y): (f64, f64)| if self.transposed { (y, x) } else { (x, y) };
        out.into_iter().filter(keep).map(|(a, b)| (flip(a), flip(b))).collect()
    }

    /// 📐️ The rigid facets between the fracture lines in the panel frame of [`YieldPattern::lines`]: the sheets above and below the ridge and the triangles at the ends of the ridge; empty facets are dropped, together they tile the panel.
    pub fn facets(&self, l: f64, h: f64) -> Vec<Vec<(f64, f64)>> {
        let (w, v) = if self.transposed { (h, l) } else { (l, h) };
        let (x0, x1, r) = (self.left, w - self.right, self.ridge);
        let flip = |(x, y): (f64, f64)| if self.transposed { (y, x) } else { (x, y) };
        let mut out = Vec::new();
        if r > 1e-9 {
            out.push(vec![(0.0, 0.0), (w, 0.0), (x1, r), (x0, r)]);
        }
        if v - r > 1e-9 {
            out.push(vec![(0.0, v), (w, v), (x1, r), (x0, r)]);
        }
        if self.left > 1e-9 {
            out.push(vec![(0.0, 0.0), (x0, r), (0.0, v)]);
        }
        if self.right > 1e-9 {
            out.push(vec![(w, 0.0), (w, v), (x1, r)]);
        }
        out.into_iter().map(|f| f.into_iter().map(flip).collect()).collect()
    }

    /// 📐️ The relative deflection 0..1 of the collapse mechanism at a point of the panel (rigid facets rotating about the held edges, 1 on the ridge): the lowest of the facet planes.
    pub fn deflection(&self, l: f64, h: f64, x: f64, y: f64) -> f64 {
        let (w, v) = if self.transposed { (h, l) } else { (l, h) };
        let (s, t) = if self.transposed { (y, x) } else { (x, y) };
        let (x0, r) = (self.left, self.ridge);
        let mut d: f64 = 1.0;
        if r > 1e-9 {
            d = d.min(t / r);
        }
        if v - r > 1e-9 {
            d = d.min((v - t) / (v - r));
        }
        if self.left > 1e-9 {
            d = d.min(s / x0);
        }
        if self.right > 1e-9 {
            d = d.min((w - s) / self.right);
        }
        d.clamp(0.0, 1.0)
    }
}

/// 🏹️ Compression block depth `d_c` of the arch that reproduces the catalogue capacity: the three-hinge arch of span `ℓ_a` and thickness `t` resists `8 f_d d_c (t − d_c)/ℓ_a²`, equal to [`arching_pressure`] for `d_c (t − d_c) = t²/8`, the smaller root.
pub fn arch_block_depth(t: f64) -> f64 {
    0.5 * t * (1.0 - 0.5_f64.sqrt())
}

/// 🏹️ Thrust line of the three-hinge arch in a wall of thickness `t` and span `span`: `samples + 1` points `(z, y)` with `z` up from the lower support and `y` the depth from the face on which the supports bear; the thrust acts at `d_c/2` at the supports and `t − d_c/2` at mid-span.
pub fn arch_thrust_line(t: f64, span: f64, samples: usize) -> Vec<(f64, f64)> {
    let block = arch_block_depth(t);
    let rise = t - block;
    (0..=samples)
        .map(|i| {
            let u = i as f64 / samples.max(1) as f64;
            (u * span, 0.5 * block + 4.0 * rise * u * (1.0 - u))
        })
        .collect()
}

/// 📍️ One position along a row of supports (ties, anchors, tie columns) with the information whether its document row may move.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Support {
    pub at: f64,
    pub movable: bool,
}

/// 📍️ The single move that brings the interval of the largest overshoot within its limit: the index of the support and its new position, rounded to the millimetre so that the interval keeps its limit; `None` when the rows hold or no single move keeps both neighbouring intervals within their limits.
pub fn relocation(points: &[Support], limits: &[f64]) -> Option<(usize, f64)> {
    if points.len() < 2 || limits.len() + 1 != points.len() {
        return None;
    }
    const STEP: f64 = 1e-3;
    const TOLERANCE: f64 = 1e-9;
    let (k, ratio) = (0..limits.len()).map(|k| (k, (points[k + 1].at - points[k].at) / limits[k])).fold((0, f64::MIN), |a, r| if r.1 > a.1 { r } else { a });
    if ratio <= 1.0 + TOLERANCE {
        return None;
    }
    let floor = |x: f64| ((x / STEP) + TOLERANCE).floor() * STEP;
    let ceil = |x: f64| ((x / STEP) - TOLERANCE).ceil() * STEP;
    let mut options: Vec<(usize, f64)> = Vec::new();
    if points[k].movable && k >= 1 {
        let to = ceil(points[k + 1].at - limits[k]);
        if to - points[k - 1].at <= limits[k - 1] + TOLERANCE && to < points[k + 1].at {
            options.push((k, to));
        }
    }
    if points[k + 1].movable && k + 1 < limits.len() {
        let to = floor(points[k].at + limits[k]);
        if points[k + 2].at - to <= limits[k + 1] + TOLERANCE && to > points[k].at {
            options.push((k + 1, to));
        }
    }
    let distance = |option: &(usize, f64)| (option.1 - points[option.0].at).abs();
    options.into_iter().fold(None, |best: Option<(usize, f64)>, option| match best {
        Some(b) if distance(&option) >= distance(&b) - TOLERANCE => Some(b),
        _ => Some(option),
    })
}

/// 🏛️ Arching capacity q_lat,d = f_d (t/ℓ_a)² [Pa].
pub fn arching_pressure(f_d: f64, t: f64, span: f64) -> f64 {
    f_d * (t / span).powi(2)
}

/// 🧱️ Rectangular masonry section with discrete reinforcement bars for the plane-section M-N analysis.
#[derive(Clone, Debug)]
pub struct MnSection {
    pub b: f64,
    pub depth: f64,
    pub f_d: f64,
    pub f_yd: f64,
    pub e_s: f64,
    pub bars: Vec<(f64, f64)>,
}

pub const EPS_MU: f64 = 0.0035;
pub const BLOCK_RATIO: f64 = 0.8;

impl MnSection {
    /// ➕ Axial force [N] (compression positive) and moment about mid-depth [N·m] for a neutral-axis depth `c` from the compressed edge.
    pub fn at(&self, c: f64) -> (f64, f64) {
        let x = (BLOCK_RATIO * c).min(self.depth);
        let cm = self.f_d * self.b * x;
        let mut n = cm;
        let mut m = cm * (self.depth / 2.0 - x / 2.0);
        for (y, a) in &self.bars {
            let strain = EPS_MU * (c - y) / c;
            let sigma = (self.e_s * strain).clamp(-self.f_yd, self.f_yd);
            n += sigma * a;
            m += sigma * a * (self.depth / 2.0 - y);
        }
        (n, m)
    }
    /// 📈 Moment capacity [N·m] at the axial force `n_ed`, zero when the force lies outside the section capacity.
    pub fn m_rd(&self, n_ed: f64) -> f64 {
        let (lo_c, hi_c) = (1e-4 * self.depth, 60.0 * self.depth);
        if n_ed < self.at(lo_c).0 || n_ed > self.at(hi_c).0 {
            return 0.0;
        }
        let (mut a, mut b) = (lo_c, hi_c);
        for _ in 0..100 {
            let mid = 0.5 * (a + b);
            if self.at(mid).0 < n_ed { a = mid } else { b = mid }
        }
        self.at(0.5 * (a + b)).1
    }
    /// 📈 Points (N, M) of the interaction curve for viewers.
    pub fn curve(&self, points: usize) -> Vec<(f64, f64)> {
        (0..points)
            .map(|i| {
                let c = self.depth * 1e-3 * (60.0 * 1e3f64).powf(i as f64 / (points - 1).max(1) as f64);
                self.at(c)
            })
            .collect()
    }
}
