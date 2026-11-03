//! ⛰️ Slope stability of a circular slip surface by slices: simplified Bishop, ordinary (Fellenius) and simplified Janbu, with the pseudo-static seismic forces of EN 1998-5 §4.1.3.3 (`F_H = k_h W`, `F_V = ±r F_H`) acting at the centroid of every slice as additional slice forces. Without seismic forces the factors are those of the static methods.

fn rad(deg: f64) -> f64 {
    deg.to_radians()
}

/// 📥️ Homogeneous slope with a circular slip surface: crest at x = 0 and height h, toe at x = h/tan β, horizontal ground on both sides.
#[derive(Clone, Copy, Debug)]
pub struct Circle {
    pub h: f64,
    pub beta_deg: f64,
    pub xc: f64,
    pub yc: f64,
    pub r: f64,
    pub gamma: f64,
    pub gamma_sat: f64,
    pub gwl_level: f64,
    pub ru: f64,
    pub q: f64,
}

/// 📤️ One slice of a circle (read-only record): width, weight, base angle (positive where the base falls towards the toe), pore force, centre abscissa, base ordinate and height at the centre, and the edge abscissae with base and ground ordinates.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Slice {
    pub b: f64,
    pub w: f64,
    pub alpha: f64,
    pub u: f64,
    pub x: f64,
    pub yb: f64,
    pub h: f64,
    pub x0: f64,
    pub x1: f64,
    pub base: [f64; 2],
    pub top: [f64; 2],
}

/// 🌋 Pseudo-static seismic forces of a slice of weight `W`: horizontal `F_H = horizontal · W` towards the toe and vertical `F_V = ±vertical · F_H` (both signs are examined, the smaller factor governs).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Pseudostatic {
    pub horizontal: f64,
    pub vertical: f64,
}

impl Pseudostatic {
    pub const NONE: Self = Self { horizontal: 0.0, vertical: 0.0 };

    /// 🌋 Horizontal and vertical (downward positive) force of a slice of weight `w` for the vertical sign `sign` (+1 or −1).
    pub fn forces(&self, w: f64, sign: f64) -> (f64, f64) {
        let fh = self.horizontal * w;
        (fh, sign * self.vertical * fh)
    }

    fn signs(&self) -> &'static [f64] {
        if self.horizontal * self.vertical > 0.0 { &[1.0, -1.0] } else { &[1.0] }
    }
}

/// ⛰️ The ground line of the embankment with its tangent and run evaluated once, not per scan step.
#[derive(Clone, Copy)]
struct Ground {
    h: f64,
    tan: f64,
    run: f64,
}

impl Ground {
    fn of(s: &Circle) -> Self {
        let tan = rad(s.beta_deg).tan();
        Self { h: s.h, tan, run: s.h / tan }
    }

    fn at(&self, x: f64) -> f64 {
        if x <= 0.0 { self.h } else if x >= self.run { 0.0 } else { self.h - x * self.tan }
    }
}

/// 🍰️ The `n` slices of the circle inside the ground, `None` when no sliding mass develops: the single source of the checked and the drawn slices.
pub fn slices(s: &Circle, n: usize) -> Option<Vec<Slice>> {
    let ground = Ground::of(s);
    let below = |x: f64| -> Option<f64> {
        let under = s.r * s.r - (x - s.xc).powi(2);
        if under < 0.0 { None } else { Some(s.yc - under.sqrt()) }
    };
    let (x_lo, x_hi) = (s.xc - s.r, s.xc + s.r);
    let steps = 4000;
    let at = |i: usize| x_lo + (x_hi - x_lo) * i as f64 / steps as f64;
    let inside = |x: f64| below(x).is_some_and(|y| y < ground.at(x));
    let xa = (0..=steps).map(at).find(|x| inside(*x))?;
    let xb = (0..=steps).rev().map(at).find(|x| inside(*x))?;
    if xb - xa < 1e-6 {
        return None;
    }
    let b = (xb - xa) / n as f64;
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        let x = xa + (i as f64 + 0.5) * b;
        let (x0, x1) = (xa + i as f64 * b, xa + (i as f64 + 1.0) * b);
        let yb = below(x)?;
        let edge = |e: f64| below(e).unwrap_or(yb);
        let yg = ground.at(x);
        let height = (yg - yb).max(0.0);
        let wet = (yg.min(s.gwl_level) - yb).max(0.0).min(height);
        let weight = s.gamma * (height - wet) * b + s.gamma_sat * wet * b + s.q * b;
        let u = if s.ru > 0.0 { s.ru * s.gamma * height * b } else { 9_810.0 * wet * b };
        out.push(Slice { b, w: weight, alpha: ((s.xc - x) / s.r).asin(), u, x, yb, h: height, x0, x1, base: [edge(x0), edge(x1)], top: [ground.at(x0), ground.at(x1)] });
    }
    Some(out)
}

fn worst(p: &Pseudostatic, factor: impl Fn(f64) -> Option<f64>) -> Option<f64> {
    p.signs().iter().map(|sign| factor(*sign)).try_fold(f64::INFINITY, |least, f| f.map(|f| least.min(f)))
}

fn fixed_point(mut f: f64, step: impl Fn(f64) -> f64) -> f64 {
    for _ in 0..200 {
        let next = step(f);
        if (next - f).abs() < 1e-9 {
            return next;
        }
        f = next;
    }
    f
}

/// 📐️ Moment of the horizontal force about the circle centre per radius: `F_H (y_c − y_cg) / R` of the slice.
fn lever(s: &Circle, k: &Slice, fh: f64) -> f64 {
    fh * (s.yc - (k.yb + 0.5 * k.h)) / s.r
}

/// 📐️ Simplified Bishop factor of safety F for design parameters tan φ_d and c_d (fixed-point iteration to 1e-9) under the pseudo-static forces `p`: the vertical force enters the weight, the horizontal force the driving moment about the centre.
pub fn bishop_with(s: &Circle, tan_phi: f64, c: f64, n: usize, p: &Pseudostatic) -> Option<f64> {
    let sl = slices(s, n)?;
    worst(p, |sign| {
        let forces: Vec<(f64, f64)> = sl.iter().map(|k| p.forces(k.w, sign)).collect();
        let drive: f64 = sl.iter().zip(&forces).map(|(k, (fh, fv))| (k.w + fv) * k.alpha.sin() + lever(s, k, *fh)).sum();
        if drive <= 0.0 {
            return None;
        }
        Some(fixed_point(1.5, |f| sl.iter().zip(&forces).map(|(k, (_, fv))| (c * k.b + (k.w + fv - k.u).max(0.0) * tan_phi) / (k.alpha.cos() * (1.0 + k.alpha.tan() * tan_phi / f))).sum::<f64>() / drive))
    })
}

/// 📐️ Ordinary (Fellenius) factor of safety under the pseudo-static forces `p`: the base normal force is `(W + F_V) cos α − F_H sin α − u/cos α`.
pub fn fellenius_with(s: &Circle, tan_phi: f64, c: f64, n: usize, p: &Pseudostatic) -> Option<f64> {
    let sl = slices(s, n)?;
    worst(p, |sign| {
        let forces: Vec<(f64, f64)> = sl.iter().map(|k| p.forces(k.w, sign)).collect();
        let drive: f64 = sl.iter().zip(&forces).map(|(k, (fh, fv))| (k.w + fv) * k.alpha.sin() + lever(s, k, *fh)).sum();
        let resist: f64 = sl.iter().zip(&forces).map(|(k, (fh, fv))| c * k.b / k.alpha.cos() + ((k.w + fv) * k.alpha.cos() - fh * k.alpha.sin() - k.u / k.alpha.cos()).max(0.0) * tan_phi).sum();
        (drive > 0.0).then(|| resist / drive)
    })
}

/// 📐️ Janbu simplified factor with the correction f0 = 1 + b1·(d/L − 1.4·(d/L)²) under the pseudo-static forces `p`: the horizontal equilibrium sums `(W + F_V) tan α + F_H`.
pub fn janbu_with(s: &Circle, tan_phi: f64, c: f64, n: usize, p: &Pseudostatic) -> Option<f64> {
    let sl = slices(s, n)?;
    let ground = Ground::of(s);
    let chord = sl.last()?.x - sl.first()?.x + sl[0].b;
    let depth = sl.iter().map(|k| ground.at(k.x) - (s.yc - (s.r * s.r - (k.x - s.xc).powi(2)).max(0.0).sqrt())).fold(0.0, f64::max);
    let b1 = if c > 0.0 && tan_phi > 0.0 { 0.69 } else if tan_phi > 0.0 { 0.31 } else { 0.5 };
    let ratio = depth / chord;
    let f0 = 1.0 + b1 * (ratio - 1.4 * ratio * ratio);
    worst(p, |sign| {
        let forces: Vec<(f64, f64)> = sl.iter().map(|k| p.forces(k.w, sign)).collect();
        let drive: f64 = sl.iter().zip(&forces).map(|(k, (fh, fv))| (k.w + fv) * k.alpha.tan() + fh).sum();
        if drive <= 0.0 {
            return None;
        }
        Some(fixed_point(1.5, |f| f0 * sl.iter().zip(&forces).map(|(k, (_, fv))| (c * k.b + (k.w + fv - k.u).max(0.0) * tan_phi) / (k.alpha.cos().powi(2) * (1.0 + k.alpha.tan() * tan_phi / f))).sum::<f64>() / drive))
    })
}

/// 📐️ Static simplified Bishop factor of safety, see [`bishop_with`].
pub fn bishop(s: &Circle, tan_phi: f64, c: f64, n: usize) -> Option<f64> {
    bishop_with(s, tan_phi, c, n, &Pseudostatic::NONE)
}

/// 📐️ Static ordinary (Fellenius) factor of safety, see [`fellenius_with`].
pub fn fellenius(s: &Circle, tan_phi: f64, c: f64, n: usize) -> Option<f64> {
    fellenius_with(s, tan_phi, c, n, &Pseudostatic::NONE)
}

/// 📐️ Static simplified Janbu factor of safety, see [`janbu_with`].
pub fn janbu(s: &Circle, tan_phi: f64, c: f64, n: usize) -> Option<f64> {
    janbu_with(s, tan_phi, c, n, &Pseudostatic::NONE)
}
