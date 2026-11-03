//! 🧩️ EN 1999-1-1 §8 joints: bolts, rivets, welds and their spacing rules as pure clause functions. See [`super`].

use std::f64::consts::PI;

/// 🔩 Fastener material strength f_ub / f_ur [Pa]; bolts by property class and stainless grade, rivets by `rivet-<alloy>` (unverified table).
pub fn fastener_fu_pa(grade: &str) -> f64 {
    match grade.to_ascii_lowercase().as_str() {
        "4.6" => 400.0e6,
        "5.6" => 500.0e6,
        "8.8" => 800.0e6,
        "10.9" => 1000.0e6,
        "a2-70" | "a4-70" => 700.0e6,
        "rivet-6082" => 160.0e6,
        _ => 0.0,
    }
}

pub const FASTENER_GRADES: &[&str] = &["4.6", "5.6", "8.8", "10.9", "a2-70", "a4-70", "rivet-6082"];

pub fn is_rivet(grade: &str) -> bool { grade.to_ascii_lowercase().starts_with("rivet-") }

/// 📏 Tensile stress area A_s [m²] of the metric thread nearest to the nominal diameter.
pub fn stress_area(d: f64) -> f64 {
    const TABLE: [(f64, f64); 9] = [(8.0, 36.6), (10.0, 58.0), (12.0, 84.3), (14.0, 115.0), (16.0, 157.0), (20.0, 245.0), (24.0, 353.0), (27.0, 459.0), (30.0, 561.0)];
    let mm = d * 1.0e3;
    let nearest = TABLE.iter().min_by(|a, b| (a.0 - mm).abs().partial_cmp(&(b.0 - mm).abs()).unwrap_or(std::cmp::Ordering::Equal)).copied().unwrap_or((mm, PI / 4.0 * mm * mm));
    if (nearest.0 - mm).abs() < 0.5 { nearest.1 * 1.0e-6 } else { 0.78 * PI / 4.0 * d * d }
}

/// 🕳️ Hole diameter d0 = d + 2 mm up to M24, d + 3 mm above.
pub fn hole_diameter(d: f64) -> f64 { d + if d <= 0.024 { 0.002 } else { 0.003 } }

/// 📏 Minimum distances of Table 8.2 as multiples of the hole diameter d0: end distance e1 and edge distance e2 (both 1.2) and pitch p1 along the load (2.2); the spacing p2 across the load is the registry row [`MIN_SPACING_ROW`] ([`min_spacing_d0`]).
pub const MIN_END_EDGE_D0: f64 = 1.2;
pub const MIN_PITCH_D0: f64 = 2.2;

/// 🔖 Registry row of the minimum spacing p2 of Table 8.2 as a multiple of d0 (unverified, recalled from the printed table; see the row evidence).
pub const MIN_SPACING_ROW: &str = "ndp.en1999.table-8-2.p2-over-d0";

/// 📏 Minimum spacing p2 across the load as a multiple of the hole diameter d0, read from [`MIN_SPACING_ROW`].
pub fn min_spacing_d0() -> f64 { semio_s_plugin_norm_compliance::ndp_common(MIN_SPACING_ROW) }

/// 🔪 α_v of the shear resistance: 0.6, but 0.5 for class 10.9 (unverified).
pub fn alpha_v(grade: &str) -> f64 { if grade == "10.9" { 0.5 } else { 0.6 } }

/// ✂️ Shear resistance F_v,Rd of one fastener in one shear plane: α_v·f_ub·A/γ_M2, with A_s for bolts and A_0 for rivets.
pub fn shear_resistance(grade: &str, d: f64, gamma_m2: f64) -> f64 {
    let f = fastener_fu_pa(grade);
    if is_rivet(grade) {
        let a0 = PI / 4.0 * hole_diameter(d).powi(2);
        0.6 * f * a0 / gamma_m2
    } else {
        alpha_v(grade) * f * stress_area(d) / gamma_m2
    }
}

/// 🧲 Tension resistance F_t,Rd = 0.9·f_ub·A_s/γ_M2.
pub fn tension_resistance(grade: &str, d: f64, gamma_m2: f64) -> f64 { 0.9 * fastener_fu_pa(grade) * stress_area(d) / gamma_m2 }

/// 🔨 Punching resistance B_p,Rd = 0.6·π·d_m·t_p·f_u/γ_M2 with d_m = 1.5·d.
pub fn punching_resistance(d: f64, t_p: f64, f_u: f64, gamma_m2: f64) -> f64 { 0.6 * PI * 1.5 * d * t_p * f_u / gamma_m2 }

/// 🧲 Bearing resistance F_b,Rd = k1·α_b·f_u·d·t/γ_M2 of an edge bolt; `e1` is the end distance along the load, `e2` the edge distance across it and `p1` the pitch along it (EN 1999-1-1 Table 8.2; the gauge p2 is a spacing, never an edge distance).
pub fn bearing_resistance(e1: f64, p1: f64, e2: f64, d: f64, t: f64, f_u: f64, f_ub: f64, gamma_m2: f64) -> f64 {
    let d0 = hole_diameter(d);
    let alpha_b = (e1 / (3.0 * d0)).min(p1 / (3.0 * d0) - 0.25).min(f_ub / f_u).min(1.0).max(0.0);
    let k1 = (2.8 * e2 / d0 - 1.7).min(2.5).max(0.0);
    k1 * alpha_b * f_u * d * t / gamma_m2
}

/// 🔩 In-plane force `(fx, fy)` [N] on the fastener at `(x, y)` [m] from the centroid of a group; `x` runs along the load (rows, pitch p1), `y` across it (bolts per row, spacing p2).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FastenerForce { pub x: f64, pub y: f64, pub fx: f64, pub fy: f64 }

impl FastenerForce {
    pub fn resultant(&self) -> f64 { self.fx.hypot(self.fy) }
}

/// 🧭 Elastic distribution of an eccentric in-plane shear `shear` along `x` with lever arm `eccentricity` across `y` over a `rows × per_row` grid (rigid plate, equal stiff fasteners; the elastic method, cf. EN 1993-1-8 3.12 - no clause of EN 1999-1-1 is quoted for it): the direct share V/n plus the torsion T = V·e shared as `F = T·r / Σr²` perpendicular to each radius. The forces sum to `(shear, 0)` and their moment about the centroid equals `−V·e`.
pub fn group_forces(rows: u32, per_row: u32, p1: f64, p2: f64, shear: f64, eccentricity: f64) -> Vec<FastenerForce> {
    let points = grid_points(rows, per_row, p1, p2);
    let n = points.len() as f64;
    let polar: f64 = points.iter().map(|(x, y)| x * x + y * y).sum();
    let torque = shear * eccentricity;
    points.into_iter().map(|(x, y)| {
        let (tx, ty) = if polar > 0.0 { (torque * y / polar, -torque * x / polar) } else { (0.0, 0.0) };
        FastenerForce { x, y, fx: shear / n + tx, fy: ty }
    }).collect()
}

/// 🔖 Registry rows of the fastener load-deformation curve R/R_ult = (1 − e^(−μ·r/r_max))^k of the instantaneous-centre method (Crawford and Kulak; unverified, common practice rather than an EN 1999-1-1 clause).
pub const IC_CURVE_MU_ROW: &str = "ndp.en1999.fastener-ic.curve-mu";
pub const IC_CURVE_EXPONENT_ROW: &str = "ndp.en1999.fastener-ic.curve-exponent";

fn grid_points(rows: u32, per_row: u32, p1: f64, p2: f64) -> Vec<(f64, f64)> {
    let (rows, per_row) = (rows.max(1), per_row.max(1));
    let coordinate = |index: u32, count: u32, step: f64| (f64::from(index) - 0.5 * f64::from(count - 1)) * step;
    (0..rows).flat_map(|i| (0..per_row).map(move |j| (coordinate(i, rows, p1), coordinate(j, per_row, p2)))).collect()
}

/// 🌀 Unit forces `(fx, fy)` of the fastener `points` rotating about the instantaneous centre `(0, centre_y)`: perpendicular to the radius, magnitude from the load-deformation curve with the deformation proportional to the radius; returns the sums `(Σfx, Σ(x·fy − y·fx))` and the unit forces.
fn centre_unit_forces(points: &[(f64, f64)], centre_y: f64, mu: f64, exponent: f64) -> (f64, f64, Vec<(f64, f64)>) {
    let radius = |p: &(f64, f64)| p.0.hypot(p.1 - centre_y);
    let r_max = points.iter().map(radius).fold(0.0, f64::max).max(1e-300);
    let units: Vec<(f64, f64)> = points.iter().map(|p| {
        let r = radius(p);
        if r < 1e-15 { return (0.0, 0.0); }
        let w = (1.0 - (-mu * r / r_max).exp()).powf(exponent);
        (-(p.1 - centre_y) / r * w, p.0 / r * w)
    }).collect();
    let force = units.iter().map(|u| u.0).sum();
    let moment = points.iter().zip(&units).map(|(p, u)| p.0 * u.1 - p.1 * u.0).sum();
    (force, moment, units)
}

/// 🌀 Instantaneous-centre distribution of an eccentric in-plane shear over a `rows × per_row` grid of ductile fasteners (same geometry and sign convention as [`group_forces`]): the plate rotates about the centre `(0, −sign(e)·t)` on the symmetry axis, every fastener resists perpendicular to its radius with R/R_ult = (1 − e^(−μ·r/r_max))^k (registry [`IC_CURVE_MU_ROW`], [`IC_CURVE_EXPONENT_ROW`]); `t` is found by bisection on the equilibrium ratio e = −Σ(x·fy − y·fx)/Σfx and the scale by Σfx = `shear`. A concentric shear, a single fastener or a group that cannot take the lever arm falls back to the elastic [`group_forces`].
pub fn instantaneous_centre_forces(rows: u32, per_row: u32, p1: f64, p2: f64, shear: f64, eccentricity: f64) -> Vec<FastenerForce> {
    let points = grid_points(rows, per_row, p1, p2);
    let (mu, exponent) = (semio_s_plugin_norm_compliance::ndp_common(IC_CURVE_MU_ROW), semio_s_plugin_norm_compliance::ndp_common(IC_CURVE_EXPONENT_ROW));
    let sign = eccentricity.signum();
    let reach = points.iter().map(|p| p.0.hypot(p.1)).fold(0.0, f64::max);
    if points.len() < 2 || eccentricity == 0.0 || reach <= 0.0 || !(mu.is_finite() && exponent.is_finite()) {
        return group_forces(rows, per_row, p1, p2, shear, eccentricity);
    }
    let ratio = |t: f64| { let (f, m, _) = centre_unit_forces(&points, -sign * t, mu, exponent); -m / f };
    let target = eccentricity.abs();
    let (mut lo, mut hi) = ((1e-9 * reach).ln(), (1e9 * reach).ln());
    if !(sign * ratio(lo.exp()) >= target && sign * ratio(hi.exp()) <= target) {
        return group_forces(rows, per_row, p1, p2, shear, eccentricity);
    }
    for _ in 0..200 {
        let mid = 0.5 * (lo + hi);
        if sign * ratio(mid.exp()) > target { lo = mid; } else { hi = mid; }
    }
    let (force, _, units) = centre_unit_forces(&points, -sign * (0.5 * (lo + hi)).exp(), mu, exponent);
    let scale = shear / force;
    points.iter().zip(units).map(|(p, u)| FastenerForce { x: p.0, y: p.1, fx: u.0 * scale, fy: u.1 * scale }).collect()
}

/// 🔩 Fastener forces by the selected method: [`group_forces`] (elastic) or [`instantaneous_centre_forces`].
pub fn fastener_forces(method: semio_s_artifact_norm_en1999_model::FastenerForceMethod, rows: u32, per_row: u32, p1: f64, p2: f64, shear: f64, eccentricity: f64) -> Vec<FastenerForce> {
    use semio_s_artifact_norm_en1999_model::FastenerForceMethod as M;
    match method {
        M::Elastic => group_forces(rows, per_row, p1, p2, shear, eccentricity),
        M::InstantaneousCentre => instantaneous_centre_forces(rows, per_row, p1, p2, shear, eccentricity),
    }
}

/// 🔩 English and German name of a fastener force method for scene texts.
pub fn force_method_label(method: semio_s_artifact_norm_en1999_model::FastenerForceMethod) -> (&'static str, &'static str) {
    use semio_s_artifact_norm_en1999_model::FastenerForceMethod as M;
    match method {
        M::Elastic => ("elastic method", "elastisches Verfahren"),
        M::InstantaneousCentre => ("instantaneous-centre method", "Momentanpolverfahren"),
    }
}

/// 🎯 Index of the most loaded fastener of `forces` (the first one on a tie).
pub fn governing_fastener(forces: &[FastenerForce]) -> usize {
    let top = forces.iter().map(FastenerForce::resultant).fold(0.0, f64::max);
    forces.iter().position(|f| f.resultant() >= top * (1.0 - 1e-12)).unwrap_or(0)
}

/// 📏 Long-joint reduction β_Lf = 1 − (L_j − 15·d)/(200·d), limited to [0.75, 1].
pub fn long_joint_factor(l_j: f64, d: f64) -> f64 {
    if l_j <= 15.0 * d { 1.0 } else { (1.0 - (l_j - 15.0 * d) / (200.0 * d)).clamp(0.75, 1.0) }
}

/// 🧱 Block tearing resistance of a single-line group in concentric loading (`e1` end distance along the load, `e2` edge distance across it): f_u·A_nt/γ_M2 + f_o·A_nv/(√3·γ_M1).
pub fn block_tearing(n: u32, e1: f64, p1: f64, e2: f64, d: f64, t: f64, f_u: f64, f_o: f64, gamma_m1: f64, gamma_m2: f64) -> f64 {
    let d0 = hole_diameter(d);
    let nf = f64::from(n.max(1));
    let a_nt = ((e2 - 0.5 * d0) * t).max(0.0);
    let a_nv = ((e1 + (nf - 1.0) * p1 - (nf - 0.5) * d0) * t).max(0.0);
    f_u * a_nt / gamma_m2 + f_o * a_nv / (3.0f64.sqrt() * gamma_m1)
}

/// 🕳️ Best net width and predecessor hole line of every path across a plate of `width` that ends at each of `lines` hole lines of diameter `d0` at the lateral spacing `gauge`; the holes of every second line are offset by `stagger` along the load (0 = aligned), a diagonal between two visited holes adds s²/(4·p) with the offset s and the lateral distance p (EN 1999-1-1 6.2.2).
fn net_paths(width: f64, d0: f64, lines: u32, gauge: f64, stagger: f64) -> (Vec<f64>, Vec<Option<usize>>) {
    let offset = |j: usize| if j % 2 == 1 { stagger } else { 0.0 };
    let (mut best, mut from): (Vec<f64>, Vec<Option<usize>>) = (Vec::new(), Vec::new());
    for k in 0..lines as usize {
        let (mut value, mut origin) = (width - d0, None);
        for j in 0..k {
            let via = best[j] - d0 + (offset(k) - offset(j)).powi(2) / (4.0 * ((k - j) as f64 * gauge).max(1e-12));
            if via < value { (value, origin) = (via, Some(j)); }
        }
        best.push(value);
        from.push(origin);
    }
    (best, from)
}

/// 🕳️ Net width [m] of the most critical path across the plate: the minimum over all paths through any subset of the holes ([`net_paths`]); the plate width when there is no hole.
pub fn net_width(width: f64, d0: f64, lines: u32, gauge: f64, stagger: f64) -> f64 {
    net_paths(width, d0, lines, gauge, stagger).0.into_iter().fold(width, f64::min)
}

/// 🕳️ Indices of the hole lines on the most critical path of [`net_width`], in lateral order; empty without holes.
pub fn net_path(width: f64, d0: f64, lines: u32, gauge: f64, stagger: f64) -> Vec<usize> {
    let (best, from) = net_paths(width, d0, lines, gauge, stagger);
    let Some(mut k) = (0..best.len()).min_by(|a, b| best[*a].total_cmp(&best[*b])) else { return Vec::new() };
    let mut path = vec![k];
    while let Some(j) = from[k] { path.push(j); k = j; }
    path.reverse();
    path
}

/// 🕳️ Plate width [m] across the load of a bolt group: the bolt lines at `gauge` and the edge distance `edge` on both sides.
pub fn plate_width(lines: u32, gauge: f64, edge: f64) -> f64 { f64::from(lines.saturating_sub(1)) * gauge + 2.0 * edge }

/// 🕳️ Tension resistance of the plate at the bolt lines: the smaller of the general yielding N_o,Rd = b·t·f_o/γ_M1 and the net-section fracture N_u,Rd = 0.9·A_net·f_u/γ_M2 (EN 1999-1-1 6.2.3); returns `(N_o,Rd, N_u,Rd)`.
pub fn plate_tension_resistances(width: f64, net: f64, t: f64, f_o: f64, f_u: f64, gamma_m1: f64, gamma_m2: f64) -> (f64, f64) {
    (width * t * f_o / gamma_m1, 0.9 * net * t * f_u / gamma_m2)
}

/// 🔥 Weld filler strength f_w [Pa] by alloy (unverified Table 8.8 values carried over from the repo).
pub fn filler_fw_pa(filler: &str, parent_fu: f64) -> f64 {
    match filler.to_ascii_lowercase().as_str() {
        "4043" | "alsi5" => 190.0e6,
        "5356" | "almg5" => 240.0e6,
        "5183" => 270.0e6,
        _ => parent_fu.min(190.0e6),
    }
}

/// 🔥 Fillet weld resistance per unit length: the smaller of the weld metal f_w·a/(β_w·γ_Mw) and the HAZ fusion boundary ρ_u,haz·f_u·(a/0.7)/(√3·γ_M2).
pub fn weld_resistance_per_length(throat: f64, beta_w: f64, f_w: f64, parent_fu: f64, rho_u_haz: f64, gamma_mw: f64, gamma_m2: f64) -> (f64, f64) {
    let metal = f_w * throat / (beta_w.max(0.01) * gamma_mw);
    let haz = rho_u_haz * parent_fu * (throat / 0.7) / (3.0f64.sqrt() * gamma_m2);
    (metal, haz)
}

/// 📏 Minimum effective weld length max(8·a, 40 mm).
pub fn min_weld_length(throat: f64) -> f64 { (8.0 * throat).max(0.040) }

/// 📌 Pin shear resistance of one shear plane F_v,Rd = 0.6·f_up·A/γ_M2 (EN 1993-1-8 Table 3.10 transferred, unverified for EN 1999-1-1 §8.5.10).
pub fn pin_shear_resistance(grade: &str, d: f64, gamma_m2: f64) -> f64 { 0.6 * fastener_fu_pa(grade) * PI / 4.0 * d * d / gamma_m2 }

/// 📌 Pin bending resistance M_Rd = 1.5·W_el·f_yp/γ_M2 with f_yp = 0.8·f_up (unverified).
pub fn pin_bending_resistance(grade: &str, d: f64, gamma_m2: f64) -> f64 { 1.5 * PI * d.powi(3) / 32.0 * 0.8 * fastener_fu_pa(grade) / gamma_m2 }

/// 📌 Plate bearing resistance at a pin F_b,Rd = 1.5·t·d·f_o/γ_M1 (unverified).
pub fn pin_bearing_resistance(t: f64, d: f64, f_o: f64, gamma_m1: f64) -> f64 { 1.5 * t * d * f_o / gamma_m1 }

/// 📌 Pin bending moment M_Ed = F_Ed·b/8 of a double-shear pin without gaps (a = c = 0, b = plate thickness).
pub fn pin_bending_moment(f_ed: f64, t_p: f64) -> f64 { f_ed * t_p / 8.0 }
