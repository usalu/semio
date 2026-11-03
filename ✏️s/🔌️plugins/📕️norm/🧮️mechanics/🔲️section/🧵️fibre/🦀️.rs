//! 🧵️ Fibre section kernel: strain-plane integration of concrete fibres and lumped reinforcement with caller-supplied stress laws, the force-balanced moment-curvature curve and its yield and ultimate points. Domain-neutral: N, mm, MPa, strains in permille, compression positive, `z` upwards, the compressed face at the top. See [`super`] and <https://concreteproperties.readthedocs.io/en/latest/user_guide/analysis.html>.

/// 📈 Stress law: strain in permille (compression positive) to stress in MPa; defined for every strain, ultimate strains are limits of the caller and never of the law.
pub type Law<'a> = Box<dyn Fn(f64) -> f64 + 'a>;

/// 🧵 One fibre: strain level `z` (mm), area `a` (mm²) and first moment `s` = ∫ z dA (mm³) about the origin.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Fibre {
    pub z: f64,
    pub a: f64,
    pub s: f64,
}

impl Fibre {
    /// 🪡 Lumped fibre of area `a` at `z`.
    pub fn lumped(z: f64, a: f64) -> Self {
        Self { z, a, s: a * z }
    }
}

/// 🟦 `slices` strips of the rectangle b × h centred on the origin.
pub fn rectangle_fibres(b: f64, h: f64, slices: usize) -> Vec<Fibre> {
    let n = slices.max(1);
    let dz = h / n as f64;
    (0..n).map(|i| Fibre::lumped(-0.5 * h + (i as f64 + 0.5) * dz, b * dz)).collect()
}

/// 📐 Parabola-rectangle stress σ(ε) with plateau `strength` from the strain `eps_peak` on and exponent `n` (EN 1992-1-1 3.1.7 Fig. 3.3); zero in tension, the plateau continues for every strain above the peak.
pub fn parabola_rectangle(strength: f64, eps_peak: f64, n: f64, eps: f64) -> f64 {
    if eps <= 0.0 {
        0.0
    } else if eps < eps_peak {
        strength * (1.0 - (1.0 - eps / eps_peak).powf(n))
    } else {
        strength
    }
}

/// 🔩 Elastic perfectly plastic stress σ(ε) = clamp(E ε, ±f_y) for ε in permille and E in MPa.
pub fn elastic_plastic(yield_stress: f64, modulus: f64, eps: f64) -> f64 {
    (modulus * eps / 1000.0).clamp(-yield_stress, yield_stress)
}

/// ⛓️ Lumped reinforcement: a fibre with its own law; `displaces` removes the concrete stress it replaces while it is compressed.
pub struct Reinforcement<'a> {
    pub fibre: Fibre,
    pub law: Law<'a>,
    pub displaces: bool,
}

/// 🎚️ Strain limits (permille) that bound the yield and ultimate points: compression at the extreme concrete fibre and tension in the most strained reinforcement.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Limits {
    pub concrete_yield: f64,
    pub concrete_ultimate: f64,
    pub steel_yield: f64,
    pub steel_ultimate: f64,
}

/// 🎯 Yield and ultimate point of a section at a normal force: curvatures in 1/m, moments in N·mm.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Characteristics {
    pub kappa_y: f64,
    pub m_y: f64,
    pub kappa_u: f64,
    pub m_u: f64,
}

/// 🧱 Fibre section: concrete fibres under one law, reinforcement under their own laws; `top` is the highest strain level of the section.
pub struct FibreSection<'a> {
    pub concrete: Vec<Fibre>,
    pub concrete_law: Law<'a>,
    pub reinforcement: Vec<Reinforcement<'a>>,
    pub top: f64,
    pub cache: SectionCache,
}

/// 🗃️ Memo of one section instance, keyed by the exact bits of the inputs: the balanced mean strain per (curvature, normal force, strain limit) and the characteristics per (normal force, limits); it lives as long as the section, i.e. one document revision.
#[derive(Default)]
pub struct SectionCache {
    balances: std::cell::RefCell<std::collections::HashMap<[u64; 3], Option<f64>>>,
    characteristics: std::cell::RefCell<std::collections::HashMap<[u64; 5], Option<Characteristics>>>,
}

const STEP_TOLERANCE: f64 = 1.0e-14;
const MAX_ITERATIONS: usize = 240;

/// 🎯 Root of an increasing `f` in the bracket `[lo, hi]` with `f(lo) < 0 <= f(hi)`: Illinois regula falsi, bisecting every fourth step so the bracket always halves at least that often and the iteration cannot diverge. See <https://en.wikipedia.org/wiki/Regula_falsi#The_Illinois_algorithm>.
pub fn bracketed_root(mut f: impl FnMut(f64) -> f64, mut lo: f64, mut hi: f64, mut f_lo: f64, mut f_hi: f64) -> f64 {
    let mut side = 0i8;
    let mut x = hi;
    for i in 0..MAX_ITERATIONS {
        let previous = x;
        let secant = hi - f_hi * (hi - lo) / (f_hi - f_lo);
        x = if i % 4 == 3 || !secant.is_finite() || secant <= lo || secant >= hi { 0.5 * (lo + hi) } else { secant };
        let value = f(x);
        if value == 0.0 {
            return x;
        }
        if value < 0.0 {
            lo = x;
            f_lo = value;
            if side == -1 {
                f_hi *= 0.5;
            }
            side = -1;
        } else {
            hi = x;
            f_hi = value;
            if side == 1 {
                f_lo *= 0.5;
            }
            side = 1;
        }
        if (hi - lo).abs() <= STEP_TOLERANCE * (1.0 + x.abs()) || (i > 0 && (x - previous).abs() <= STEP_TOLERANCE * (1.0 + x.abs())) {
            break;
        }
    }
    0.5 * (lo + hi)
}

const CACHE_ENTRIES: usize = 4096;
const STRAIN_FLOOR: f64 = -200.0;

impl FibreSection<'_> {
    /// ⚙️ Axial force and moment (N, N·mm; compression positive, compression at +z) of the strain plane ε(z) = `eps_m` + `kappa` z, `kappa` in permille per mm (numerically 1/m).
    pub fn integrate(&self, eps_m: f64, kappa: f64) -> (f64, f64) {
        let (mut n, mut m) = (0.0, 0.0);
        for f in &self.concrete {
            let s = (self.concrete_law)(eps_m + kappa * f.z);
            n += s * f.a;
            m += s * f.s;
        }
        for r in &self.reinforcement {
            let e = eps_m + kappa * r.fibre.z;
            let mut s = (r.law)(e);
            if r.displaces && e > 0.0 {
                s -= (self.concrete_law)(e);
            }
            n += s * r.fibre.a;
            m += s * r.fibre.s;
        }
        (n, m)
    }

    /// 🧮 Mean strain that balances `n_ed` at the curvature `kappa` with the extreme fibre at most at `eps_top_max`; `None` when no strain plane below that limit balances it. Memoised per section.
    pub fn balance(&self, kappa: f64, n_ed: f64, eps_top_max: f64) -> Option<f64> {
        let key = [kappa.to_bits(), n_ed.to_bits(), eps_top_max.to_bits()];
        if let Some(hit) = self.cache.balances.borrow().get(&key) {
            return *hit;
        }
        let solved = self.solve_balance(kappa, n_ed, eps_top_max);
        let mut balances = self.cache.balances.borrow_mut();
        if balances.len() >= CACHE_ENTRIES {
            balances.clear();
        }
        balances.insert(key, solved);
        solved
    }

    fn solve_balance(&self, kappa: f64, n_ed: f64, eps_top_max: f64) -> Option<f64> {
        let (lo, hi) = (STRAIN_FLOOR, eps_top_max - kappa * self.top);
        if hi <= lo {
            return None;
        }
        let (f_lo, f_hi) = (self.integrate(lo, kappa).0 - n_ed, self.integrate(hi, kappa).0 - n_ed);
        if f_lo > 0.0 || f_hi < 0.0 {
            return None;
        }
        if f_lo == 0.0 {
            return Some(lo);
        }
        Some(bracketed_root(|e| self.integrate(e, kappa).0 - n_ed, lo, hi, f_lo, f_hi))
    }

    /// 📍 Moment (N·mm) at the curvature `kappa` (1/m) and normal force `n_ed`, `None` when `balance` fails.
    pub fn moment_at(&self, kappa: f64, n_ed: f64, eps_top_max: f64) -> Option<f64> {
        let eps_m = self.balance(kappa, n_ed, eps_top_max)?;
        Some(self.integrate(eps_m, kappa).1)
    }

    /// 📉 Moment-curvature curve at `n_ed`: (κ in 1/m, M in N·mm) for `points` curvatures up to `kappa_max`, dense towards zero, starting at the origin; curvatures that cannot be balanced are skipped.
    pub fn curve(&self, n_ed: f64, kappa_max: f64, points: usize, eps_top_max: f64) -> Vec<(f64, f64)> {
        let mut out = vec![(0.0, 0.0)];
        for k in 1..=points {
            let kappa = kappa_max * (k as f64 / points as f64).powf(1.5);
            if let Some(m) = self.moment_at(kappa, n_ed, eps_top_max) {
                out.push((kappa, m));
            }
        }
        out
    }

    fn demand_ratio(&self, kappa: f64, n_ed: f64, concrete_limit: f64, steel_limit: f64) -> Option<f64> {
        let eps_m = self.balance(kappa, n_ed, 10.0 * concrete_limit)?;
        let tension = self.reinforcement.iter().map(|r| -(eps_m + kappa * r.fibre.z)).fold(f64::NEG_INFINITY, f64::max);
        Some(((eps_m + kappa * self.top) / concrete_limit).max(tension / steel_limit))
    }

    fn curvature_at_ratio_one(&self, n_ed: f64, concrete_limit: f64, steel_limit: f64) -> Option<f64> {
        if concrete_limit <= 0.0 || steel_limit <= 0.0 || self.demand_ratio(0.0, n_ed, concrete_limit, steel_limit)? >= 1.0 {
            return None;
        }
        let depth = (self.top - self.concrete.iter().map(|f| f.z).fold(f64::INFINITY, f64::min)).max(1.0);
        let (mut lo, mut hi) = (0.0, (concrete_limit + steel_limit) / depth);
        for _ in 0..40 {
            if self.demand_ratio(hi, n_ed, concrete_limit, steel_limit).map_or(true, |r| r >= 1.0) {
                break;
            }
            lo = hi;
            hi *= 2.0;
        }
        let g = |k: f64| self.demand_ratio(k, n_ed, concrete_limit, steel_limit).map_or(1.0, |r| r - 1.0);
        let f_hi = g(hi);
        if f_hi < 0.0 {
            return Some(hi);
        }
        Some(bracketed_root(g, lo, hi, g(lo), f_hi))
    }

    /// 🏁 Yield point (first of the concrete reaching `concrete_yield` or the tension reinforcement `steel_yield`) and ultimate point (first of `concrete_ultimate` or `steel_ultimate`) at `n_ed`; `None` when `n_ed` is outside the range the section balances below its first limit.
    pub fn characteristics(&self, n_ed: f64, limits: &Limits) -> Option<Characteristics> {
        let key = [n_ed.to_bits(), limits.concrete_yield.to_bits(), limits.concrete_ultimate.to_bits(), limits.steel_yield.to_bits(), limits.steel_ultimate.to_bits()];
        if let Some(hit) = self.cache.characteristics.borrow().get(&key) {
            return *hit;
        }
        let solved = self.solve_characteristics(n_ed, limits);
        let mut memo = self.cache.characteristics.borrow_mut();
        if memo.len() >= CACHE_ENTRIES {
            memo.clear();
        }
        memo.insert(key, solved);
        solved
    }

    fn solve_characteristics(&self, n_ed: f64, limits: &Limits) -> Option<Characteristics> {
        let kappa_y = self.curvature_at_ratio_one(n_ed, limits.concrete_yield, limits.steel_yield)?;
        let kappa_u = self.curvature_at_ratio_one(n_ed, limits.concrete_ultimate, limits.steel_ultimate)?.max(kappa_y);
        let at = |kappa: f64| self.balance(kappa, n_ed, 10.0 * limits.concrete_ultimate).map(|e| self.integrate(e, kappa).1);
        Some(Characteristics { kappa_y, m_y: at(kappa_y)?, kappa_u, m_u: at(kappa_u)? })
    }
}
