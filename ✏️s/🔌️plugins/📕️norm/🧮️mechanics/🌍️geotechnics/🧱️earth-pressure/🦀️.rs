//! 🧱️ Mononobe–Okabe earth pressure of EN 1998-5 Annex E on a wall of height `h`: active and passive coefficients for any wall inclination, wall friction and backfill slope, the more onerous sign of the vertical acceleration, the partially submerged backfill and the hydrodynamic water pressure of Westergaard. Without acceleration the coefficients are those of Coulomb.

fn rad(deg: f64) -> f64 {
    deg.to_radians()
}

/// 🌊 Resultant of the Westergaard pressure `p(z) = 7/8 k_h γ_w √(h z)` divided by `k_h γ_w h²`: the integral of `7/8 √(h z)` over the depth is `7/12 h²`.
pub const WESTERGAARD_COEFFICIENT: f64 = 7.0 / 12.0;

/// 🌊 Height of the Westergaard resultant above the base in shares of the water height: the depth centroid of `√z` is `3/5 h`.
pub const WESTERGAARD_ARM: f64 = 0.4;

/// 🌊 Unit weight of water [N/m³].
pub const UNIT_WEIGHT_WATER: f64 = 9_810.0;

/// 🧱 Which limit state of the backfill is mobilised.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Side {
    Active,
    Passive,
}

/// 📐 Backfill and wall geometry of Annex E: friction angle φ, wall friction δ, wall back inclination ψ against the horizontal (90 for a vertical wall) and backfill slope β, all in degrees.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Interface {
    pub phi_deg: f64,
    pub delta_deg: f64,
    pub wall_deg: f64,
    pub slope_deg: f64,
}

impl Interface {
    /// 📐 Vertical wall with a horizontal backfill.
    pub const fn vertical(phi_deg: f64, delta_deg: f64) -> Self {
        Self { phi_deg, delta_deg, wall_deg: 90.0, slope_deg: 0.0 }
    }
}

/// 🌋 Pseudo-static acceleration of the backfill: horizontal coefficient `k_h` and the magnitude `k_v` of the vertical one; both signs `1 ± k_v` are examined.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Acceleration {
    pub kh: f64,
    pub kv: f64,
}

impl Acceleration {
    pub const NONE: Self = Self { kh: 0.0, kv: 0.0 };
}

/// 🌊 Water in the backfill: height `height_m` above the wall base, saturated unit weight (0 takes the dry unit weight) and the permeability class. A permeable backfill (water free to move against the grains) takes the dry weight as inertia and the Westergaard pressure of the water; a backfill of low permeability moves with its water, the saturated weight is the inertia and no separate water pressure arises.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Water {
    pub height_m: f64,
    pub saturated_unit_weight: f64,
    pub permeable: bool,
}

impl Water {
    pub const NONE: Self = Self { height_m: 0.0, saturated_unit_weight: 0.0, permeable: false };
}

/// 🧱 Thrust per metre of wall [N/m] with its parts and the vertical factor `1 ± k_v` that governed.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Thrust {
    pub coefficient: f64,
    pub vertical: f64,
    pub soil: f64,
    pub water_static: f64,
    pub water_dynamic: f64,
}

impl Thrust {
    pub fn total(&self) -> f64 {
        self.soil + self.water_static + self.water_dynamic
    }

    pub fn water(&self) -> f64 {
        self.water_static + self.water_dynamic
    }
}

/// 🌋 Seismic angle θ with `tan θ = k_h / (1 ± k_v)` (Annex E), `vertical` being the factor `1 ± k_v`.
pub fn seismic_angle(kh: f64, vertical: f64) -> f64 {
    (kh / vertical).atan()
}

/// 📐 Active coefficient `K_AE` for the seismic angle `theta` [rad] (Annex E, Eq. E.4); `None` where φ − θ − β < 0, the backfill has no limit equilibrium.
pub fn active_coefficient(i: &Interface, theta: f64) -> Option<f64> {
    let (phi, delta, psi, beta) = (rad(i.phi_deg), rad(i.delta_deg), rad(i.wall_deg), rad(i.slope_deg));
    if phi - theta - beta < 0.0 {
        return None;
    }
    let root = ((phi + delta).sin() * (phi - theta - beta).sin() / ((psi - theta - delta).sin() * (psi + beta).sin())).sqrt();
    Some((psi + phi - theta).sin().powi(2) / (theta.cos() * psi.sin().powi(2) * (psi - theta - delta).sin() * (1.0 + root).powi(2)))
}

/// 📐 Passive coefficient `K_PE` for the seismic angle `theta` [rad] (Annex E, Eq. E.5) with δ the magnitude of the wall friction and the inertia force pointing away from the wall (the less resisting direction); verified against the force polygon of the sliding wedge. `None` where φ + β − θ < 0 or the root reaches 1 (no passive wedge).
pub fn passive_coefficient(i: &Interface, theta: f64) -> Option<f64> {
    let (phi, delta, psi, beta) = (rad(i.phi_deg), rad(i.delta_deg), rad(i.wall_deg), rad(i.slope_deg));
    if phi + beta - theta < 0.0 {
        return None;
    }
    let root = ((phi + delta).sin() * (phi + beta - theta).sin() / ((psi + theta + delta).sin() * (psi + beta).sin())).sqrt();
    (root < 1.0).then(|| (psi + phi - theta).sin().powi(2) / (theta.cos() * psi.sin().powi(2) * (psi + theta + delta).sin() * (1.0 - root).powi(2)))
}

fn coefficient(side: Side, i: &Interface, theta: f64) -> Option<f64> {
    match side {
        Side::Active => active_coefficient(i, theta),
        Side::Passive => passive_coefficient(i, theta),
    }
}

/// 🧱 Thrust of the backfill of unit weight `gamma` against a wall of height `h` for one vertical factor `1 ± k_v`: the dry part above the water with `K(θ)`, the submerged part with the effective weight `γ_sat − γ_w` and its own angle `θ_w` (`tan θ_w = γ_inertia/(γ_sat − γ_w) · k_h/(1 ± k_v)`), the hydrostatic water thrust and, for a permeable backfill, the Westergaard thrust. `None` without limit equilibrium or with a non-positive effective weight.
pub fn thrust_for(side: Side, i: &Interface, gamma: f64, h: f64, water: Water, kh: f64, vertical: f64) -> Option<Thrust> {
    let wet = water.height_m.clamp(0.0, h);
    let dry = h - wet;
    let k_dry = coefficient(side, i, seismic_angle(kh, vertical))?;
    let mut soil = 0.5 * gamma * vertical * dry * dry * k_dry;
    let (mut water_static, mut water_dynamic) = (0.0, 0.0);
    if wet > 0.0 {
        let saturated = if water.saturated_unit_weight > 0.0 { water.saturated_unit_weight } else { gamma };
        let effective = saturated - UNIT_WEIGHT_WATER;
        if effective <= 0.0 {
            return None;
        }
        let inertia = if water.permeable { gamma } else { saturated };
        let k_wet = coefficient(side, i, seismic_angle(kh * inertia / effective, vertical))?;
        soil += k_wet * vertical * (gamma * dry * wet + 0.5 * effective * wet * wet);
        water_static = 0.5 * UNIT_WEIGHT_WATER * wet * wet;
        if water.permeable {
            water_dynamic = WESTERGAARD_COEFFICIENT * kh * UNIT_WEIGHT_WATER * wet * wet;
        }
    }
    Some(Thrust { coefficient: k_dry, vertical, soil, water_static, water_dynamic })
}

/// 🌋 Thrust under the more onerous sign of the vertical acceleration: the larger total for the active side, the smaller for the passive side.
pub fn thrust(side: Side, i: &Interface, gamma: f64, h: f64, water: Water, a: Acceleration) -> Option<Thrust> {
    let plus = thrust_for(side, i, gamma, h, water, a.kh, 1.0 + a.kv);
    let minus = thrust_for(side, i, gamma, h, water, a.kh, 1.0 - a.kv);
    let onerous = |x: Thrust, y: Thrust| match side {
        Side::Active => if y.total() > x.total() { y } else { x },
        Side::Passive => if y.total() < x.total() { y } else { x },
    };
    match (plus, minus) {
        (Some(x), Some(y)) => Some(onerous(x, y)),
        (x, y) => x.or(y).filter(|_| side == Side::Passive),
    }
}

/// 🌋 Thrust `P` [N/m] and angle of the critical wedge plane (degrees from horizontal) of a vertical wall with a horizontal backfill, found by maximising the force polygon of the sliding wedge over the plane angle: weight `W = ½γH²·cot ρ` times the vertical factor, the inertia force `k_h·W` towards the wall, the wall reaction at δ and the plane reaction at φ. Independent of the closed form [`active_coefficient`].
pub fn critical_wedge(gamma: f64, h: f64, phi_deg: f64, delta_deg: f64, kh: f64, vertical: f64) -> (f64, f64) {
    let (phi, delta) = (rad(phi_deg), rad(delta_deg));
    let force = |rho: f64| {
        let a = -rho.sin() + phi.tan() * rho.cos();
        let b = rho.cos() + phi.tan() * rho.sin();
        let det = delta.cos() * b - delta.sin() * a;
        0.5 * gamma * h * h / rho.tan() * (kh * b - vertical * a) / det
    };
    let coarse = (1..360).map(|i| rad(0.25 * f64::from(i))).max_by(|x, y| force(*x).total_cmp(&force(*y))).unwrap_or(rad(45.0));
    let (mut lo, mut hi) = (coarse - rad(0.25), coarse + rad(0.25));
    for _ in 0..60 {
        let (m1, m2) = (lo + (hi - lo) / 3.0, hi - (hi - lo) / 3.0);
        if force(m1) < force(m2) {
            lo = m1;
        } else {
            hi = m2;
        }
    }
    let rho = 0.5 * (lo + hi);
    (rho.to_degrees(), force(rho))
}
