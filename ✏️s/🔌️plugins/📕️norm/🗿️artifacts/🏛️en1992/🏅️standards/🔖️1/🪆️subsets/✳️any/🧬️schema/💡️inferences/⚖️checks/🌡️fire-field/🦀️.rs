//! 🌡️ Fire field: the 2D transient heat conduction of any concrete section outline under the ISO 834 standard fire (enthalpy formulation, explicit finite volumes, progress and cancellation per time step), with the temperature-dependent siliceous-concrete properties of EN 1992-1-2 3.3 and the boundary conditions of EN 1991-1-2 3.2 (convection 25 and 4 W/m²K, emissivity 0,7, 1,0), the 500 °C isotherm contour (Annex B.1) and the zone-method offset contour (Annex B.2). SI units, `y` across the width, `z` up, origin at the section centroid. The single thermal solver of the EN 1992 plugin: the member checks and the scene read it from here. See [`super`].

use super::*;
use std::cell::RefCell;

pub const LEVEL_500: f64 = 500.0;
pub const MOISTURE_DEFAULT: f64 = 1.5;

/// 🔢 Number of zones of the zone method (Annex B.2), registry row `ndp.en1992.fire-zones` (unverified); EN and DE agree, so the common value is read.
pub fn zones() -> usize {
    (semio_s_plugin_norm_compliance::ndp_common("ndp.en1992.fire-zones").round() as usize).clamp(1, 20)
}

/// 🧮️ Exponent of the damage formula of the zone method, registry row `ndp.en1992.fire-zone-exponent` (unverified; the spec R-04 reads 1, EN 1992-1-2 Eq. B.6 is quoted with 1.3 in the row evidence).
pub fn zone_exponent() -> f64 {
    semio_s_plugin_norm_compliance::ndp_common("ndp.en1992.fire-zone-exponent")
}
const AMBIENT: f64 = 20.0;
const ALPHA_EXPOSED: f64 = 25.0;
const ALPHA_UNEXPOSED: f64 = 4.0;
const EMISSIVITY: f64 = 0.7;
const STEFAN_BOLTZMANN: f64 = 5.67e-8;
const CELL: f64 = 0.005;
const MAX_CELLS: usize = 100;
const MIN_CELLS: usize = 8;
const DIFFUSIVITY_MAX: f64 = 1.0e-6;
const SAFETY: f64 = 0.2;
const TABLE_STEP: f64 = 0.5;
const TABLE_MAX: f64 = 1200.0;
const ENTHALPY_BINS: usize = 8192;
const CACHE: usize = 8;

/// 🔥️ Which faces of the section `(left, right, bottom, top)` the fire reaches; the other faces are in contact with the ambient air.
pub type Faces = (bool, bool, bool, bool);

/// 🧱️ Thermal conductivity of normal-weight concrete in W/(m K), upper limit of EN 1992-1-2 3.3.3 (1).
pub fn conductivity(theta: f64) -> f64 {
    let x = theta.clamp(20.0, TABLE_MAX) / 100.0;
    2.0 - 0.2451 * x + 0.0107 * x * x
}

/// 💧️ Specific heat of siliceous concrete in J/(kg K), EN 1992-1-2 3.3.2 with the moisture peak of the water content `u` (% by mass) between 100 and 115 degC, linear in `u` between the tabulated 0, 1,5, 3 and 10 %.
pub fn specific_heat(theta: f64, moisture: f64) -> f64 {
    let peak = moisture_peak(moisture);
    match theta {
        t if t < 100.0 => 900.0,
        t if t <= 115.0 => peak,
        t if t <= 200.0 => peak + (1000.0 - peak) * (t - 115.0) / 85.0,
        t if t <= 400.0 => 1000.0 + (t - 200.0) / 2.0,
        _ => 1100.0,
    }
}

fn moisture_peak(u: f64) -> f64 {
    let table = [(0.0, 900.0), (1.5, 1470.0), (3.0, 2020.0), (10.0, 5600.0)];
    let u = u.clamp(0.0, 10.0);
    let i = table.iter().position(|&(k, _)| k >= u).unwrap_or(table.len() - 1).max(1);
    let ((k0, c0), (k1, c1)) = (table[i - 1], table[i]);
    c0 + (c1 - c0) * (u - k0) / (k1 - k0)
}

/// 🪨️ Density of normal-weight concrete in kg/m³, EN 1992-1-2 3.3.2 (3) with `rho(20) = 2300`.
pub fn density(theta: f64) -> f64 {
    let r = 2300.0;
    match theta {
        t if t <= 115.0 => r,
        t if t <= 200.0 => r * (1.0 - 0.02 * (t - 115.0) / 85.0),
        t if t <= 400.0 => r * (0.98 - 0.03 * (t - 200.0) / 200.0),
        t => r * (0.95 - 0.07 * (t.min(TABLE_MAX) - 400.0) / 800.0),
    }
}

struct Enthalpy {
    bin: f64,
    theta: Vec<f64>,
    heat: Vec<f64>,
    step: f64,
}

impl Enthalpy {
    fn new(moisture: f64) -> Self {
        let n = (TABLE_MAX / TABLE_STEP) as usize + 1;
        let mut heat = Vec::with_capacity(n);
        let mut acc = 0.0;
        let volumetric = |t: f64| density(t) * specific_heat(t, moisture);
        for i in 0..n {
            if i > 0 {
                let (a, b) = ((i - 1) as f64 * TABLE_STEP + AMBIENT, i as f64 * TABLE_STEP + AMBIENT);
                acc += 0.5 * (volumetric(a) + volumetric(b)) * TABLE_STEP;
            }
            heat.push(acc);
        }
        let top = heat[n - 1];
        let bin = top / ENTHALPY_BINS as f64;
        let mut theta = Vec::with_capacity(ENTHALPY_BINS + 1);
        let mut j = 0;
        for k in 0..=ENTHALPY_BINS {
            let h = k as f64 * bin;
            while j + 1 < n - 1 && heat[j + 1] < h {
                j += 1;
            }
            let span = (heat[j + 1] - heat[j]).max(1e-9);
            theta.push(AMBIENT + (j as f64 + ((h - heat[j]) / span).clamp(0.0, 1.0)) * TABLE_STEP);
        }
        Self { bin, theta, heat, step: TABLE_STEP }
    }

    fn enthalpy(&self, t: f64) -> f64 {
        let x = ((t - AMBIENT) / self.step).clamp(0.0, (self.heat.len() - 1) as f64);
        let i = (x.floor() as usize).min(self.heat.len() - 2);
        self.heat[i] + (self.heat[i + 1] - self.heat[i]) * (x - i as f64)
    }

    fn temperature(&self, h: f64) -> f64 {
        let x = (h / self.bin).clamp(0.0, ENTHALPY_BINS as f64);
        let i = (x.floor() as usize).min(ENTHALPY_BINS - 1);
        self.theta[i] + (self.theta[i + 1] - self.theta[i]) * (x - i as f64)
    }
}

/// 🌡️ The temperature field of one section at one time (cell centres, row-major with `z` rows).
#[derive(Clone, Debug, PartialEq)]
pub struct Field {
    pub nx: usize,
    pub ny: usize,
    pub width: f64,
    pub height: f64,
    pub temperatures: Vec<f64>,
    pub origin: (f64, f64),
    pub mask: Vec<bool>,
    pub outline: Vec<(f64, f64)>,
}

impl Field {
    fn cell(&self, i: usize, j: usize) -> f64 {
        self.temperatures[j * self.nx + i]
    }

    /// 🌡️ Temperature at `(y, z)` by bilinear interpolation between the cell centres, clamped at the faces.
    pub fn at(&self, y: f64, z: f64) -> f64 {
        let (y, z) = (y - self.origin.0, z - self.origin.1);
        let fx = ((y + self.width / 2.0) / self.width * self.nx as f64 - 0.5).clamp(0.0, (self.nx - 1) as f64);
        let fy = ((z + self.height / 2.0) / self.height * self.ny as f64 - 0.5).clamp(0.0, (self.ny - 1) as f64);
        let (i, j) = ((fx.floor() as usize).min(self.nx.saturating_sub(2)), (fy.floor() as usize).min(self.ny.saturating_sub(2)));
        let (i1, j1) = ((i + 1).min(self.nx - 1), (j + 1).min(self.ny - 1));
        let (u, v) = (fx - i as f64, fy - j as f64);
        (1.0 - v) * ((1.0 - u) * self.cell(i, j) + u * self.cell(i1, j)) + v * ((1.0 - u) * self.cell(i, j1) + u * self.cell(i1, j1))
    }

    /// 📏️ Position on the segment `from -> to` where the temperature falls below `level` coming from `from` (the face), by bisection; `None` when it never does.
    fn cold_start(&self, from: (f64, f64), to: (f64, f64), level: f64) -> Option<f64> {
        let len = (to.0 - from.0).hypot(to.1 - from.1);
        let steps = ((len / (CELL / 4.0)).ceil() as usize).max(4);
        let point = |t: f64| (from.0 + (to.0 - from.0) * t, from.1 + (to.1 - from.1) * t);
        let hot = |t: f64| {
            let p = point(t);
            self.at(p.0, p.1) > level
        };
        let first = (0..=steps).map(|k| k as f64 / steps as f64).find(|&t| !hot(t))?;
        if first == 0.0 {
            return Some(0.0);
        }
        let (mut lo, mut hi) = (first - 1.0 / steps as f64, first);
        for _ in 0..40 {
            let mid = 0.5 * (lo + hi);
            if hot(mid) { lo = mid } else { hi = mid }
        }
        Some(0.5 * (lo + hi) * len)
    }

    /// 🔶️ The reduced rectangle `(y0, y1, z0, z1)` inside the `level` isotherm along the two centre lines (reduced cross-section of the 500 degC isotherm method); `None` when the centre is hotter than `level`.
    pub fn reduced_rectangle(&self, level: f64) -> Option<(f64, f64, f64, f64)> {
        let (hw, hh) = (self.width / 2.0, self.height / 2.0);
        let y0 = -hw + self.cold_start((-hw, 0.0), (0.0, 0.0), level)?;
        let y1 = hw - self.cold_start((hw, 0.0), (0.0, 0.0), level)?;
        let z0 = -hh + self.cold_start((0.0, -hh), (0.0, 0.0), level)?;
        let z1 = hh - self.cold_start((0.0, hh), (0.0, 0.0), level)?;
        (y1 - y0 > 1e-3 && z1 - z0 > 1e-3).then_some((y0, y1, z0, z1))
    }

    /// 〰️ The closed contour of the `level` isotherm around the centroid as `rays` points (the cold core is star-shaped for a section heated from outside); empty when the centroid is hotter than `level`.
    pub fn isotherm_ring(&self, level: f64, rays: usize) -> Vec<(f64, f64)> {
        if self.at(0.0, 0.0) > level {
            return Vec::new();
        }
        let (hw, hh) = (self.width / 2.0, self.height / 2.0);
        (0..rays.max(8))
            .map(|k| {
                let a = std::f64::consts::TAU * k as f64 / rays.max(8) as f64;
                let (dy, dz) = (a.cos(), a.sin());
                let reach = if self.outline.len() >= 3 { exit_distance(&self.outline, (dy, dz)) } else { (hw / dy.abs().max(1e-12)).min(hh / dz.abs().max(1e-12)) };
                let to = (dy * reach, dz * reach);
                let start = (0.0, 0.0);
                let len = reach;
                let steps = ((len / (CELL / 4.0)).ceil() as usize).max(4);
                let hot_at = |t: f64| self.at(start.0 + to.0 * t, start.1 + to.1 * t) > level;
                match (0..=steps).map(|s| s as f64 / steps as f64).find(|&t| hot_at(t)) {
                    None => to,
                    Some(first) => {
                        let (mut lo, mut hi) = ((first - 1.0 / steps as f64).max(0.0), first);
                        for _ in 0..40 {
                            let mid = 0.5 * (lo + hi);
                            if hot_at(mid) { hi = mid } else { lo = mid }
                        }
                        let t = 0.5 * (lo + hi);
                        (to.0 * t, to.1 * t)
                    }
                }
            })
            .collect()
    }

    /// 📏️ The deepest advance of the `level` isotherm along `rays` rays from the centroid to the outline (any section shape).
    pub fn ring_depth(&self, level: f64, rays: usize) -> f64 {
        let ring = self.isotherm_ring(level, rays);
        if ring.is_empty() {
            return self.outline.iter().map(|p| p.0.hypot(p.1)).fold(0.0, f64::max);
        }
        let n = ring.len();
        (0..n)
            .map(|k| {
                let a = std::f64::consts::TAU * k as f64 / n as f64;
                let reach = if self.outline.len() >= 3 { exit_distance(&self.outline, (a.cos(), a.sin())) } else { f64::INFINITY };
                (reach - ring[k].0.hypot(ring[k].1)).max(0.0)
            })
            .filter(|d| d.is_finite())
            .fold(0.0, f64::max)
    }

    /// 📏️ The deepest advance of the `level` isotherm from any face: the largest distance between the section outline and the reduced rectangle (0 when the section is entirely hotter).
    pub fn isotherm_depth(&self, level: f64) -> f64 {
        self.reduced_rectangle(level).map_or(0.0, |(y0, y1, z0, z1)| (y0 + self.width / 2.0).max(self.width / 2.0 - y1).max(z0 + self.height / 2.0).max(self.height / 2.0 - z1))
    }
}

/// 🧮️ Transient solution after `minutes` of the standard fire on the faces `faces` of a centred `width` x `height` rectangle with the concrete water content `moisture` (% by mass).
pub fn solve(width: f64, height: f64, faces: Faces, minutes: f64, moisture: f64) -> Field {
    solve_outline(&rect_outline(width, height), faces, minutes, moisture)
}

/// ▭ The centred rectangle `width` x `height` as a counter-clockwise outline.
pub fn rect_outline(width: f64, height: f64) -> Vec<(f64, f64)> {
    let (hw, hh) = (width / 2.0, height / 2.0);
    vec![(-hw, -hh), (hw, -hh), (hw, hh), (-hw, hh)]
}

/// 🧮️ Transient solution for an arbitrary section outline (`y`, `z` in m around the centroid, one closed ring): cells whose centre lies inside carry the heat flow, an exposed face is a cell side that borders the outside in that direction (left, right, bottom, top); results are cached per outline, faces, time and moisture.
pub fn solve_outline(outline: &[(f64, f64)], faces: Faces, minutes: f64, moisture: f64) -> Field {
    thread_local! {
        static CACHE_STORE: RefCell<Vec<(Vec<u64>, Field)>> = const { RefCell::new(Vec::new()) };
    }
    let key: Vec<u64> = [minutes, moisture].iter().map(|v| v.to_bits()).chain([faces.0, faces.1, faces.2, faces.3].map(u64::from)).chain(outline.iter().flat_map(|p| [p.0.to_bits(), p.1.to_bits()])).collect();
    if let Some(hit) = CACHE_STORE.with(|c| c.borrow().iter().find(|(k, _)| *k == key).map(|(_, f)| f.clone())) {
        return hit;
    }
    let field = solve_outline_with(outline, faces, minutes, moisture, &mut |_| true).unwrap_or_else(|| unreachable!("a step that never cancels"));
    CACHE_STORE.with(|c| {
        let mut c = c.borrow_mut();
        if c.len() >= CACHE {
            c.remove(0);
        }
        c.push((key, field.clone()));
    });
    field
}

/// 🧮️ [`solve_outline`] for a worker: `step(fraction)` is called once per time step and returns `false` to cancel; a cancelled solve returns `None` and is not cached.
pub fn solve_outline_with(outline: &[(f64, f64)], faces: Faces, minutes: f64, moisture: f64, step: &mut dyn FnMut(f32) -> bool) -> Option<Field> {
    let (y0, y1) = outline.iter().fold((f64::MAX, f64::MIN), |a, p| (a.0.min(p.0), a.1.max(p.0)));
    let (z0, z1) = outline.iter().fold((f64::MAX, f64::MIN), |a, p| (a.0.min(p.1), a.1.max(p.1)));
    integrate_in(y1 - y0, z1 - z0, (0.5 * (y0 + y1), 0.5 * (z0 + z1)), outline, faces, minutes, moisture, step)
}

fn inside_polygon(outline: &[(f64, f64)], p: (f64, f64)) -> bool {
    let mut inside = false;
    for i in 0..outline.len() {
        let (a, b) = (outline[i], outline[(i + 1) % outline.len()]);
        if (a.1 > p.1) != (b.1 > p.1) && p.0 < a.0 + (p.1 - a.1) / (b.1 - a.1) * (b.0 - a.0) {
            inside = !inside;
        }
    }
    inside
}

fn ray_exit(outline: &[(f64, f64)], from: (f64, f64), dir: (f64, f64)) -> f64 {
    let mut best = f64::INFINITY;
    for i in 0..outline.len() {
        let (a, b) = (outline[i], outline[(i + 1) % outline.len()]);
        let e = (b.0 - a.0, b.1 - a.1);
        let den = dir.0 * e.1 - dir.1 * e.0;
        if den.abs() < 1e-14 {
            continue;
        }
        let (dx, dy) = (a.0 - from.0, a.1 - from.1);
        let t = (dx * e.1 - dy * e.0) / den;
        let u = (dx * dir.1 - dy * dir.0) / den;
        if t > 1e-9 && (0.0..=1.0).contains(&u) {
            best = best.min(t);
        }
    }
    best
}

fn exit_distance(outline: &[(f64, f64)], dir: (f64, f64)) -> f64 {
    ray_exit(outline, (0.0, 0.0), dir)
}

#[allow(clippy::too_many_arguments)]
fn integrate_in(width: f64, height: f64, origin: (f64, f64), outline: &[(f64, f64)], faces: Faces, minutes: f64, moisture: f64, progress: &mut dyn FnMut(f32) -> bool) -> Option<Field> {
    let nx = ((width / CELL).ceil() as usize).clamp(MIN_CELLS, MAX_CELLS);
    let ny = ((height / CELL).ceil() as usize).clamp(MIN_CELLS, MAX_CELLS);
    let (dx, dy) = (width / nx as f64, height / ny as f64);
    let mask: Vec<bool> = if outline.len() >= 3 {
        (0..nx * ny).map(|c| inside_polygon(outline, (origin.0 - width / 2.0 + ((c % nx) as f64 + 0.5) * dx, origin.1 - height / 2.0 + ((c / nx) as f64 + 0.5) * dy))).collect()
    } else {
        Vec::new()
    };
    let live = |i: usize, j: usize| mask.is_empty() || mask[j * nx + i];
    let table = Enthalpy::new(moisture);
    let h_ambient = table.enthalpy(AMBIENT);
    let mut h = vec![h_ambient; nx * ny];
    let mut t = vec![AMBIENT; nx * ny];
    let dt = (SAFETY / (DIFFUSIVITY_MAX * (1.0 / (dx * dx) + 1.0 / (dy * dy)))).min(5.0);
    let total = minutes.max(0.0) * 60.0;
    let steps = (total / dt).ceil() as usize;
    let dt = if steps > 0 { total / steps as f64 } else { dt };
    let (area_x, area_y, volume) = (dy, dx, dx * dy);
    let mut flow = vec![0.0; nx * ny];
    for step in 0..steps {
        if !progress(step as f32 / steps as f32) {
            return None;
        }
        let gas = iso834((step as f64 + 0.5) * dt / 60.0);
        flow.iter_mut().for_each(|q| *q = 0.0);
        for j in 0..ny {
            for i in 0..nx {
                let c = j * nx + i;
                if !live(i, j) {
                    continue;
                }
                let lc = conductivity(t[c]);
                if i + 1 < nx && live(i + 1, j) {
                    let g = 0.5 * (lc + conductivity(t[c + 1])) * area_x / dx;
                    let q = g * (t[c + 1] - t[c]);
                    flow[c] += q;
                    flow[c + 1] -= q;
                }
                if j + 1 < ny && live(i, j + 1) {
                    let g = 0.5 * (lc + conductivity(t[c + nx])) * area_y / dy;
                    let q = g * (t[c + nx] - t[c]);
                    flow[c] += q;
                    flow[c + nx] -= q;
                }
                let sides = [(i == 0 || !live(i - 1, j), faces.0, area_x, dx), (i + 1 == nx || !live(i + 1, j), faces.1, area_x, dx), (j == 0 || !live(i, j - 1), faces.2, area_y, dy), (j + 1 == ny || !live(i, j + 1), faces.3, area_y, dy)];
                for (on_face, exposed, area, size) in sides {
                    if on_face {
                        let (gas_t, alpha) = if exposed { (gas, ALPHA_EXPOSED + radiation(gas, t[c])) } else { (AMBIENT, ALPHA_UNEXPOSED) };
                        flow[c] += area * (gas_t - t[c]) / (1.0 / alpha + 0.5 * size / lc);
                    }
                }
            }
        }
        for c in 0..nx * ny {
            if !mask.is_empty() && !mask[c] {
                continue;
            }
            h[c] += flow[c] * dt / volume;
            t[c] = table.temperature(h[c]);
        }
    }
    Some(Field { nx, ny, width, height, temperatures: t, origin, mask, outline: outline.to_vec() })
}

fn radiation(gas: f64, surface: f64) -> f64 {
    let (g, s) = (gas + 273.15, surface + 273.15);
    EMISSIVITY * STEFAN_BOLTZMANN * (g * g + s * s) * (g + s)
}

/// 🔥️ Which faces of the section the standard fire reaches for the member kind.
pub fn exposed_faces(kind: MemberKind) -> Faces {
    match kind {
        MemberKind::Column => (true, true, true, true),
        MemberKind::Slab | MemberKind::FlatSlab => (false, false, true, false),
        MemberKind::Wall | MemberKind::LiquidRetaining => (true, false, false, false),
        _ => (true, true, true, false),
    }
}

impl Field {
    /// ⬆️ Depth of the `level` isotherm from the bottom face along the vertical centre line (m); the section height when it never cools below `level`.
    pub fn bottom_depth(&self, level: f64) -> f64 {
        let hh = self.height / 2.0;
        self.cold_start((0.0, -hh), (0.0, 0.0), level).unwrap_or(hh)
    }

    /// ⬅️ Depth of the `level` isotherm from the left face along the horizontal centre line (m); half the width when it never cools below `level`.
    pub fn side_depth(&self, level: f64) -> f64 {
        let hw = self.width / 2.0;
        self.cold_start((-hw, 0.0), (0.0, 0.0), level).unwrap_or(hw)
    }

    /// 🌡️ Hottest temperature of the section (°C).
    pub fn peak(&self) -> f64 {
        self.temperatures.iter().copied().fold(AMBIENT, f64::max)
    }

    /// 🎞️ Temperatures at the mid-points of `zones` equal zones between `from` and `from + len * dir` (°C).
    pub fn profile(&self, from: (f64, f64), dir: (f64, f64), len: f64, zones: usize) -> Vec<f64> {
        (0..zones).map(|i| len * (i as f64 + 0.5) / zones as f64).map(|t| self.at(from.0 + dir.0 * t, from.1 + dir.1 * t)).collect()
    }
}

/// 🧱️ Strength factor k_c(θ) of siliceous concrete in compression, Table 3.1 of EN 1992-1-2.
fn kc(theta: f64) -> f64 {
    kc_theta(theta)
}

/// 〰️ The mean zone factor of the zone method (Annex B.2): `k_c,m = (1 - 0.2/n) (1/n) Σ k_c(θ_i)` over the zone temperatures.
pub fn zone_factor(zone_theta: &[f64]) -> f64 {
    let n = zone_theta.len() as f64;
    (1.0 - 0.2 / n) / n * zone_theta.iter().map(|t| kc(*t)).sum::<f64>()
}

/// 📏 Damaged zone `a_z = w (1 - (k_c,m / k_c(θ_M))^e)` of the zone method for the half thickness `w`, the zone temperatures, the temperature `θ_M` at the mid-plane and the registry exponent `e` (spec R-04); clamped to `[0, w]`.
pub fn zone_damage(w: f64, zone_theta: &[f64], theta_m: f64, exponent: f64) -> f64 {
    let km = kc(theta_m);
    if km <= 1e-9 || zone_theta.is_empty() {
        return w;
    }
    (w * (1.0 - (zone_factor(zone_theta) / km).powf(exponent))).clamp(0.0, w)
}

/// 🧭️ The direction of an outward edge normal as the face it belongs to: 0 left, 1 right, 2 bottom, 3 top.
fn face_of(normal: (f64, f64)) -> usize {
    if normal.0.abs() >= normal.1.abs() {
        if normal.0 < 0.0 { 0 } else { 1 }
    } else if normal.1 < 0.0 {
        2
    } else {
        3
    }
}

/// 📐️ The polygon moved inwards by `distance[i]` along edge `i` (edge `i` runs from vertex `i` to vertex `i + 1`), corners by the intersection of the moved edges; a counter-clockwise outline is expected. Empty when the offsets exceed the section.
pub fn offset_polygon(p: &[(f64, f64)], distance: &[f64]) -> Vec<(f64, f64)> {
    let n = p.len();
    if n < 3 || distance.len() != n {
        return Vec::new();
    }
    let edge = |i: usize| -> ((f64, f64), (f64, f64)) {
        let (a, b) = (p[i], p[(i + 1) % n]);
        let len = (b.0 - a.0).hypot(b.1 - a.1).max(1e-12);
        let e = ((b.0 - a.0) / len, (b.1 - a.1) / len);
        ((a.0 - e.1 * distance[i], a.1 + e.0 * distance[i]), e)
    };
    let out: Vec<(f64, f64)> = (0..n)
        .map(|i| {
            let ((q0, e0), (q1, e1)) = (edge((i + n - 1) % n), edge(i));
            let den = e0.0 * e1.1 - e0.1 * e1.0;
            if den.abs() < 1e-9 {
                return q1;
            }
            let s = ((q1.0 - q0.0) * e1.1 - (q1.1 - q0.1) * e1.0) / den;
            (q0.0 + s * e0.0, q0.1 + s * e0.1)
        })
        .collect();
    let area = |q: &[(f64, f64)]| (0..q.len()).map(|i| q[i].0 * q[(i + 1) % q.len()].1 - q[(i + 1) % q.len()].0 * q[i].1).sum::<f64>() / 2.0;
    let (a0, a1) = (area(p), area(&out));
    if a1 <= 1e-12 || a1 > a0 + 1e-12 { Vec::new() } else { out }
}

/// ✂️ The zone-method section of Annex B.2: the damage `a_z` per edge (0 on unexposed edges) and the outline offset by it, a counter-clockwise ring in m.
#[derive(Clone, Debug, PartialEq)]
pub struct Zone {
    pub outline: Vec<(f64, f64)>,
    pub damage: Vec<f64>,
    pub contour: Vec<(f64, f64)>,
}

/// 🧮️ Zone method (Annex B.2, spec R-04) on every exposed edge of `outline`: `w` is half the thickness of the section along the inward normal at the edge mid-point, `zones` equal zones between the edge and the mid-plane give the temperatures θ_i, θ_M is the temperature at the mid-plane; the reduced section is the outline moved inwards by `a_z` per edge.
pub fn zone_section(outline: &[(f64, f64)], faces: Faces, field: &Field, zones: usize) -> Zone {
    let n = outline.len();
    let area: f64 = (0..n).map(|i| outline[i].0 * outline[(i + 1) % n].1 - outline[(i + 1) % n].0 * outline[i].1).sum();
    let ring: Vec<(f64, f64)> = if area < 0.0 { outline.iter().rev().copied().collect() } else { outline.to_vec() };
    let exposed = [faces.0, faces.1, faces.2, faces.3];
    let exponent = zone_exponent();
    let damage: Vec<f64> = (0..n)
        .map(|i| {
            let (a, b) = (ring[i], ring[(i + 1) % n]);
            let len = (b.0 - a.0).hypot(b.1 - a.1).max(1e-12);
            let normal = ((b.1 - a.1) / len, -(b.0 - a.0) / len);
            if !exposed[face_of(normal)] {
                return 0.0;
            }
            let (mid, inward) = (((a.0 + b.0) / 2.0, (a.1 + b.1) / 2.0), (-normal.0, -normal.1));
            let start = (mid.0 + inward.0 * 1e-6, mid.1 + inward.1 * 1e-6);
            let thickness = ray_exit(&ring, start, inward);
            if !thickness.is_finite() {
                return 0.0;
            }
            let w = thickness / 2.0;
            let temperatures = field.profile(mid, inward, w, zones.max(1));
            zone_damage(w, &temperatures, field.at(mid.0 + inward.0 * w, mid.1 + inward.1 * w), exponent)
        })
        .collect();
    let contour = offset_polygon(&ring, &damage);
    Zone { outline: ring, damage, contour }
}

/// 🔥️ Standard fire duration in minutes of a rating.
pub fn rating_minutes(rating: FireRating) -> f64 {
    match rating {
        FireRating::R30 => 30.0,
        FireRating::R60 => 60.0,
        FireRating::R90 => 90.0,
        FireRating::R120 => 120.0,
    }
}

/// 🧱️ The section reduced for a fire check: `ring_mm` is the cold concrete (mm), the concrete keeps `f_ck` (γ_c,fi = 1, α_cc = 1) and every bar contributes `k_s(θ_s) A_s` with θ_s read from the field at its position (m); the tendons are dropped and the steel is at `f_yk` (γ_s,fi = 1).
pub fn reduced_section(sec: &Sec, ring_mm: Vec<(f64, f64)>, field: &Field, f_yk_mpa: f64) -> Sec {
    let mut reduced = sec.clone();
    reduced.loops = vec![Loop { pts: ring_mm, hole: false }];
    reduced.fcd = sec.c.fck;
    reduced.fyd = f_yk_mpa;
    reduced.tendons.clear();
    reduced.bars = sec.bars.iter().map(|b| Bar { a: b.a * ks_theta(field.at(b.y / 1000.0, b.z / 1000.0)), ..*b }).collect();
    reduced
}
