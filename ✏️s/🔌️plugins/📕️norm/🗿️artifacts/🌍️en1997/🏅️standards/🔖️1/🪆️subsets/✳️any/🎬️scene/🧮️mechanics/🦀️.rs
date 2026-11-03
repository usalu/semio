//! 🧮️ Mechanics: the pure geotechnical arithmetic the scene draws (SI, degrees at the boundary): actions, wedges, pressures, stresses, slip circles. See [`checks::catalogue`].

use super::checks::catalogue::{earth_pressure, foundations, settlement, slopes};
use super::model::{FoundationLoadCase, Pile, PileGroup, RetainingWall, SoilLayer, SpreadFoundation};

pub const GAMMA_W: f64 = 9_810.0;

/// ⛰️ Extrusion width of a slope body and its slip surface: the stated width along the slope axis, the section length when it is zero.
pub fn slope_span(s: &super::model::Slope) -> f64 {
    if s.width > 1e-6 { s.width } else { s.length }
}
pub const SLICES: usize = 12;

pub fn rad(deg: f64) -> f64 {
    deg.to_radians()
}

/// 🪨️ The layer containing `depth`, the last one below the model and the first above it.
pub fn layer_at(layers: &[SoilLayer], depth: f64) -> Option<&SoilLayer> {
    layers.iter().find(|l| depth >= l.depth_top && depth < l.depth_bottom).or_else(|| if depth < layers.first().map_or(0.0, |l| l.depth_top) { layers.first() } else { layers.last() })
}

/// 🏋️ Characteristic resultant of one load case: vertical, horizontal, moment (N, N, N·m).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Actions {
    pub v: f64,
    pub h: f64,
    pub m: f64,
}

pub fn actions(case: &FoundationLoadCase) -> Actions {
    Actions { v: case.vertical_permanent + case.vertical_variable, h: case.horizontal_permanent + case.horizontal_variable, m: case.moment_permanent + case.moment_variable }
}

/// 🎯️ The load case with the largest eccentricity, the one the base picture shows.
pub fn governing_case(footing: &SpreadFoundation) -> Option<&FoundationLoadCase> {
    footing.load_cases.iter().max_by(|a, b| ecc(&actions(a)).abs().total_cmp(&ecc(&actions(b)).abs()))
}

pub fn ecc(a: &Actions) -> f64 {
    if a.v.abs() < 1.0 { 0.0 } else { a.m / a.v }
}

/// 📐️ Effective strip of Annex D.1 along B as `(centre_x, width)`: centred on the resultant, `B' = B - 2|e|`.
pub fn effective_strip(b: f64, e_b: f64) -> (f64, f64) {
    let e = e_b.abs().min(0.49 * b);
    (e_b.signum() * e, (b - 2.0 * e).max(1e-3))
}

pub fn base_pressure(footing: &SpreadFoundation, a: &Actions) -> f64 {
    let area = (footing.width * footing.length).max(1e-9);
    a.v / area
}

pub fn footing_thickness(footing: &SpreadFoundation) -> f64 {
    (footing.width.min(footing.length) / 4.0).max(0.4).min(footing.embedment.max(0.4))
}

pub fn bearing_factors(phi_deg: f64) -> [f64; 3] {
    [foundations::n_q(phi_deg), foundations::n_c(phi_deg), foundations::n_gamma(phi_deg)]
}

/// 🌀️ Prandtl–Terzaghi mechanism under a strip of width `b` in the plane `(x, z)` with `z = 0` at the base: active wedge, two log-spiral fans, two passive wedges.
#[derive(Clone, Debug, PartialEq)]
pub struct Mechanism {
    pub active: Vec<[f64; 2]>,
    pub fans: [Vec<[f64; 2]>; 2],
    pub passive: [Vec<[f64; 2]>; 2],
    pub depth: f64,
}

pub fn mechanism(b: f64, phi_deg: f64) -> Mechanism {
    let phi = rad(phi_deg.clamp(0.0, 45.0));
    let psi = std::f64::consts::FRAC_PI_4 + phi / 2.0;
    let apex = [0.0, -(b / 2.0) * psi.tan()];
    let r0 = b / (2.0 * psi.cos());
    let steps = 12;
    let right = |sign: f64| -> ([Vec<[f64; 2]>; 2], f64) {
        let e = [sign * b / 2.0, 0.0];
        let spiral: Vec<[f64; 2]> = (0..=steps)
            .map(|i| {
                let theta = std::f64::consts::FRAC_PI_2 * i as f64 / steps as f64;
                let r = r0 * (theta * phi.tan()).exp();
                let angle = -(std::f64::consts::PI - psi) + theta;
                [e[0] + sign * r * angle.cos(), e[1] + r * angle.sin()]
            })
            .collect();
        let tip = *spiral.last().unwrap_or(&apex);
        let passive_base = std::f64::consts::FRAC_PI_4 - phi / 2.0;
        let r1 = (tip[0] - e[0]).hypot(tip[1] - e[1]);
        let ground = [e[0] + sign * 2.0 * r1 * passive_base.cos(), 0.0];
        let mut fan = vec![e];
        fan.extend(spiral);
        let depth = fan.iter().map(|p| -p[1]).fold(0.0, f64::max);
        ([fan, vec![e, tip, ground]], depth)
    };
    let (r, depth_r) = right(1.0);
    let (l, depth_l) = right(-1.0);
    Mechanism { active: vec![[-b / 2.0, 0.0], [b / 2.0, 0.0], apex], fans: [r[0].clone(), l[0].clone()], passive: [r[1].clone(), l[1].clone()], depth: depth_r.max(depth_l).max(-apex[1]) }
}

pub fn k_active(wall: &RetainingWall) -> f64 {
    earth_pressure::ka_coulomb(wall.backfill_phi_deg, wall.wall_friction_deg, 0.0, 0.0)
}

pub fn k_passive(wall: &RetainingWall) -> f64 {
    earth_pressure::kp_coulomb(wall.backfill_phi_deg, (wall.wall_friction_deg).min(wall.backfill_phi_deg / 3.0), 0.0, 0.0)
}

/// 🧱️ The coefficient behind the drawn pressure for the wall's regime: active, at rest, or increased active as the mean of both (DIN 4085 visual rule).
pub fn k_driving(wall: &RetainingWall) -> f64 {
    let (ka, k0) = (k_active(wall), earth_pressure::k0(wall.backfill_phi_deg, wall.ocr, 0.0));
    match wall.earth_pressure_mode.as_str() {
        semio_s_artifact_norm_en1997_model::EarthPressureMode::AtRest => k0,
        semio_s_artifact_norm_en1997_model::EarthPressureMode::IncreasedActive => 0.5 * (ka + k0),
        _ => ka,
    }
}

pub fn total_height(wall: &RetainingWall) -> f64 {
    wall.height + wall.embedment
}

/// 📊️ Driving pressure `e(z)` in Pa at depth `z` below the retained top.
pub fn driving_pressure(wall: &RetainingWall, z: f64) -> f64 {
    k_driving(wall) * (wall.backfill_gamma * z + wall.surcharge)
}

/// 📊️ Passive pressure in Pa at depth `z` below the dredge level of the toe side.
pub fn passive_pressure(wall: &RetainingWall, z: f64) -> f64 {
    k_passive(wall) * wall.backfill_gamma * z
}

/// ➡️ Resultant driving force per metre (N/m) and its lever arm above the base.
pub fn driving_resultant(wall: &RetainingWall) -> (f64, f64) {
    let (h, k) = (total_height(wall), k_driving(wall));
    let triangle = 0.5 * k * wall.backfill_gamma * h * h;
    let block = k * wall.surcharge * h;
    let force = triangle + block;
    if force <= 0.0 { (0.0, h / 3.0) } else { (force, (triangle * h / 3.0 + block * h / 2.0) / force) }
}

pub fn passive_resultant(wall: &RetainingWall) -> (f64, f64) {
    (0.5 * k_passive(wall) * wall.backfill_gamma * wall.embedment * wall.embedment, wall.embedment / 3.0)
}

pub fn ground_height(h: f64, beta_deg: f64, x: f64) -> f64 {
    let run = h / rad(beta_deg).tan();
    if x <= 0.0 { h } else if x >= run { 0.0 } else { h - x * rad(beta_deg).tan() }
}

pub fn slope_run(h: f64, beta_deg: f64) -> f64 {
    h / rad(beta_deg).tan()
}

/// 🍰️ One slip slice in the section plane: base chord, ground chord, weight, pore force and base angle.
#[derive(Clone, Debug, PartialEq)]
pub struct Slice {
    pub x0: f64,
    pub x1: f64,
    pub base: [f64; 2],
    pub top: [f64; 2],
    pub alpha: f64,
    pub weight: f64,
    pub pore: f64,
}

impl Slice {
    pub fn width(&self) -> f64 {
        self.x1 - self.x0
    }

    /// 📉️ Local driving over resisting ratio of the slice (Fellenius), 0 for slices that resist.
    pub fn demand(&self, tan_phi: f64, c: f64) -> f64 {
        let drive = self.weight * self.alpha.sin();
        if drive <= 0.0 {
            return 0.0;
        }
        drive / (c * self.width() / self.alpha.cos() + (self.weight * self.alpha.cos() - self.pore / self.alpha.cos()).max(0.0) * tan_phi).max(1e-9)
    }
}

/// 🍰️ The slices of the checked circle ([`slopes::slices`]), so the drawn slices are the checked ones; empty when no sliding mass develops.
pub fn slip_slices(s: &slopes::Circle, n: usize) -> Vec<Slice> {
    slopes::slices(s, n)
        .unwrap_or_default()
        .into_iter()
        .map(|k| Slice { x0: k.x0, x1: k.x1, base: k.base, top: k.top, alpha: k.alpha, weight: k.w, pore: k.u })
        .collect()
}

/// 🔎️ Coarse deterministic search of the circle with the smallest Bishop factor of safety, the shared catalogue search with the scene's slice count.
pub fn critical_circle(proto: &slopes::Circle, tan_phi: f64, c: f64) -> Option<(slopes::Circle, f64)> {
    slopes::critical_circle(proto, tan_phi, c, SLICES)
}

/// 📉️ Vertical stress of a uniform `b × l` rectangle at offset `(px, py)` from its centre and depth `z`, the shared catalogue corner solution.
pub fn sigma_z_at(sigma0: f64, b: f64, l: f64, px: f64, py: f64, z: f64) -> f64 {
    settlement::sigma_z_rect_at(sigma0, b, l, px, py, z)
}

/// 🧮️ Oedometric settlement of a footing at offset `(px, py)` from its centre, the shared catalogue stress-strain integral.
pub fn settlement_at(footing: &SpreadFoundation, layers: &[SoilLayer], q_net: f64, px: f64, py: f64) -> f64 {
    settlement::stress_strain_settlement_at(layers, footing.width, footing.length, footing.embedment, q_net, px, py)
}

/// 📐️ Characteristic serviceability pressure of a footing: the largest `(G + Q)/A` of its load cases, the pressure of the SLS pair checks.
pub fn service_pressure(footing: &SpreadFoundation) -> f64 {
    let area = (footing.width * footing.length).max(1e-9);
    footing.load_cases.iter().map(|lc| (lc.vertical_permanent + lc.vertical_variable) / area).fold(0.0, f64::max)
}

/// 📉️ Elastic settlement of the half-space at offset `(px, py)` from the centre of a footing: Love's flexible rectangle ([`settlement::displacement_rect_at`]) on the equivalent half-space of the ground below the base, zero without ground.
pub fn elastic_settlement_at(footing: &SpreadFoundation, layers: &[SoilLayer], px: f64, py: f64) -> f64 {
    super::checks::part_1::elastic_equivalent(layers, footing.width, footing.embedment).map_or(0.0, |(nu, e, _)| settlement::displacement_rect_at(service_pressure(footing), footing.width, footing.length, px, py, nu, e))
}

pub fn net_pressure(footing: &SpreadFoundation, layers: &[SoilLayer]) -> f64 {
    let a = governing_case(footing).map(actions).unwrap_or(Actions { v: 0.0, h: 0.0, m: 0.0 });
    let gamma = layer_at(layers, footing.embedment).map_or(0.0, |l| l.gamma);
    (base_pressure(footing, &a) - gamma * footing.embedment).max(0.0)
}

/// 🏗️ Pile group layout: `count` piles on a near-square grid of spacing `3 D`, centred on the origin.
pub fn pile_grid(pile: &Pile) -> Vec<[f64; 2]> {
    let n = pile.count.max(1) as usize;
    let columns = (n as f64).sqrt().ceil() as usize;
    let rows = n.div_ceil(columns);
    let s = 3.0 * pile.diameter;
    (0..n).map(|i| [((i % columns) as f64 - (columns as f64 - 1.0) / 2.0) * s, ((i / columns) as f64 - (rows as f64 - 1.0) / 2.0) * s]).collect()
}

pub fn pile_shaft_resistance(pile: &Pile) -> f64 {
    std::f64::consts::PI * pile.diameter * pile.length * pile.unit_shaft_resistance * f64::from(pile.count.max(1))
}

pub fn pile_base_resistance(pile: &Pile) -> f64 {
    std::f64::consts::PI * pile.diameter * pile.diameter / 4.0 * pile.unit_base_resistance * f64::from(pile.count.max(1))
}

/// 🧅 Radius [m] from the base centre of a `b` x `l` footing under the base pressure `q` along the downward ray (polar angle `beta` from the vertical, azimuth `phi`, both radians) where the Boussinesq stress falls to `ratio·q`: first crossing of a marching search, refined by bisection.
pub fn bulb_radius(q: f64, b: f64, l: f64, beta: f64, phi: f64, ratio: f64) -> f64 {
    let sigma = |r: f64| settlement::sigma_z_rect_at(q, b, l, r * beta.sin() * phi.cos(), r * beta.sin() * phi.sin(), (r * beta.cos()).max(1e-4));
    let (step, reach) = (0.1 * b.max(l), 8.0 * b.max(l));
    let mut hi = step;
    while hi < reach && sigma(hi) >= ratio * q {
        hi += step;
    }
    let mut lo = (hi - step).max(0.0);
    for _ in 0..24 {
        let mid = 0.5 * (lo + hi);
        if sigma(mid) >= ratio * q { lo = mid } else { hi = mid }
    }
    0.5 * (lo + hi)
}

/// 🧅 Iso-stress surface `σ_z = ratio·q` below a footing as a closed triangle mesh in footing coordinates `(x, y, z)` with `z` negative below the base plane: the apex under the centre, `rings` rings of `segments` rays down to the base plane and the base fan; `(vertices, triangles)`.
pub fn bulb_mesh(q: f64, b: f64, l: f64, ratio: f64, segments: usize, rings: usize) -> (Vec<[f64; 3]>, Vec<[u32; 3]>) {
    let point = |beta: f64, phi: f64| {
        let r = bulb_radius(q, b, l, beta, phi, ratio);
        [r * beta.sin() * phi.cos(), r * beta.sin() * phi.sin(), -r * beta.cos()]
    };
    let mut vertices = vec![point(0.0, 0.0)];
    for ring in 1..=rings {
        let beta = std::f64::consts::FRAC_PI_2 * ring as f64 / rings as f64;
        vertices.extend((0..segments).map(|j| point(beta, std::f64::consts::TAU * j as f64 / segments as f64)));
    }
    let at = |ring: usize, j: usize| (1 + (ring - 1) * segments + j % segments) as u32;
    let mut triangles: Vec<[u32; 3]> = (0..segments).map(|j| [0, at(1, j), at(1, j + 1)]).collect();
    for ring in 1..rings {
        for j in 0..segments {
            triangles.push([at(ring, j), at(ring + 1, j), at(ring + 1, j + 1)]);
            triangles.push([at(ring, j), at(ring + 1, j + 1), at(ring, j + 1)]);
        }
    }
    let cap = vertices.len() as u32;
    vertices.push([0.0, 0.0, 0.0]);
    triangles.extend((0..segments).map(|j| [cap, at(rings, j + 1), at(rings, j)]));
    (vertices, triangles)
}

/// 🪵️ Load spread of the equivalent raft of a pile group, horizontal per vertical (1:4): the textbook convention of the drawing and of PILE-12, not an EN 1997 value.
pub const RAFT_SPREAD: f64 = 0.25;

/// 🪵️ Depth of the equivalent raft below the pile head as a fraction of the pile length (2/3): the same textbook convention as [`RAFT_SPREAD`].
pub const RAFT_DEPTH_FACTOR: f64 = 2.0 / 3.0;

/// 🪵️ Equivalent raft of a pile group: plan size and depth below the pile head; the outer pile axes spread by [`RAFT_SPREAD`] over [`RAFT_DEPTH_FACTOR`] of the pile length on each side, as the catalogue entry PILE-12.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Raft {
    pub width: f64,
    pub length: f64,
    pub depth: f64,
}

/// 🪵️ The equivalent raft of `group` of `pile`.
pub fn equivalent_raft(group: &PileGroup, pile: &Pile) -> Raft {
    let grow = 2.0 * RAFT_SPREAD * RAFT_DEPTH_FACTOR * pile.length;
    Raft { width: (f64::from(group.columns) - 1.0) * group.spacing + pile.diameter + grow, length: (f64::from(group.rows) - 1.0) * group.spacing + pile.diameter + grow, depth: RAFT_DEPTH_FACTOR * pile.length }
}

/// 📉️ Settlement of a pile group under its cap load: the shared catalogue stress-strain integral under the centre of its equivalent raft, zero without a load.
pub fn group_settlement(group: &PileGroup, pile: &Pile, layers: &[SoilLayer]) -> f64 {
    let raft = equivalent_raft(group, pile);
    settlement::stress_strain_settlement_at(layers, raft.width, raft.length, raft.depth, group.cap_vertical / (raft.width * raft.length).max(1e-9), 0.0, 0.0)
}
