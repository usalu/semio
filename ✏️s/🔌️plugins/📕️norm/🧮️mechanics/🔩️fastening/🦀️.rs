//! 🔩️ Domain-neutral fastening kernel: concrete cone breakout, pull-out, splitting, steel failure, edge breakout, pry-out and interaction of headed fasteners and anchors in N, mm and MPa. Every annex-driven parameter (k_1, k_2, k_9, partial factors) is an argument; EN 1992-4, EN 1993-1-8 base plates and EN 1994 studs call the same functions. See [EN 1992-4](https://eurocodes.jrc.ec.europa.eu/).

use std::f64::consts::PI;

type Rect = (f64, f64, f64, f64);

/// 📐 Free concrete region around an anchor group (x_min, x_max, y_min, y_max in mm); unbounded sides are infinite.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Region {
    pub x_min: f64,
    pub x_max: f64,
    pub y_min: f64,
    pub y_max: f64,
}

impl Region {
    pub const FREE: Region = Region { x_min: -1e9, x_max: 1e9, y_min: -1e9, y_max: 1e9 };

    /// 🧱 Anchor at the origin with one free edge `c1` away in -x; none when `c1 <= 0`.
    pub fn single_edge(c1: f64) -> Region {
        if c1 > 0.0 { Region { x_min: -c1, ..Region::FREE } } else { Region::FREE }
    }
}

/// 🧱 Inputs of the concrete cone breakout of the tension anchors `anchors` (mm positions); `k1` comes from the annex row of the anchor type and crack state.
#[derive(Clone, Copy, Debug)]
pub struct ConeInput<'a> {
    pub f_ck: f64,
    pub h_ef: f64,
    pub k1: f64,
    pub anchors: &'a [[f64; 2]],
    pub region: Region,
    pub dense_reinforcement: bool,
    pub e_nx: f64,
    pub e_ny: f64,
    pub z_compression: f64,
}

impl ConeInput<'static> {
    /// 🧱 Isolated anchor at the origin with an optional free edge `c1`, no eccentricity, no dense reinforcement.
    pub fn single(f_ck: f64, h_ef: f64, k1: f64, c1: f64) -> ConeInput<'static> {
        ConeInput { f_ck, h_ef, k1, anchors: &[[0.0, 0.0]], region: Region::single_edge(c1), dense_reinforcement: false, e_nx: 0.0, e_ny: 0.0, z_compression: 0.0 }
    }
}

/// 🍦 Concrete cone breakout `N_Rk,c = N⁰_Rk,c A_c,N/A⁰_c,N ψ_s,N ψ_re,N ψ_ec,N ψ_M,N` (EN 1992-4 7.2.1.4).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Cone {
    pub c_cr: f64,
    pub n0_rk_c: f64,
    pub ratio: f64,
    pub psi_s: f64,
    pub psi_re: f64,
    pub psi_ec: f64,
    pub psi_m: f64,
    pub n_rk_c: f64,
}

/// 📐 Area of the union of axis-aligned rectangles (xmin, xmax, ymin, ymax) by coordinate compression.
pub fn union_area(rects: &[Rect]) -> f64 {
    let mut xs: Vec<f64> = rects.iter().flat_map(|r| [r.0, r.1]).collect();
    let mut ys: Vec<f64> = rects.iter().flat_map(|r| [r.2, r.3]).collect();
    xs.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    ys.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let mut a = 0.0;
    for i in 0..xs.len().saturating_sub(1) {
        for j in 0..ys.len().saturating_sub(1) {
            let (cx, cy) = (0.5 * (xs[i] + xs[i + 1]), 0.5 * (ys[j] + ys[j + 1]));
            if rects.iter().any(|r| cx > r.0 && cx < r.1 && cy > r.2 && cy < r.3) {
                a += (xs[i + 1] - xs[i]) * (ys[j + 1] - ys[j]);
            }
        }
    }
    a
}

/// 🧱 Ratio `A_c,N/A⁰_c,N` of the union of the cone footprints (side 2 c_cr) clipped to the free region.
pub fn cone_area_ratio(anchors: &[[f64; 2]], c_cr: f64, region: Region) -> f64 {
    let rects: Vec<Rect> = anchors
        .iter()
        .map(|p| (p[0] - c_cr, p[0] + c_cr, p[1] - c_cr, p[1] + c_cr))
        .map(|r| (r.0.max(region.x_min), r.1.min(region.x_max), r.2.max(region.y_min), r.3.min(region.y_max)))
        .collect();
    union_area(&rects) / (2.0 * c_cr).powi(2)
}

/// 🧱 Smallest edge distance of the anchors to the free region.
pub fn edge_distance(anchors: &[[f64; 2]], region: Region) -> f64 {
    anchors.iter().map(|p| (p[0] - region.x_min).min(region.x_max - p[0]).min(p[1] - region.y_min).min(region.y_max - p[1])).fold(f64::MAX, f64::min)
}

/// 🧱 `N⁰_Rk,c = k_1 √f_ck h_ef^1.5` of a single anchor without edge and spacing effects.
pub fn cone_n0_rk_c(k1: f64, f_ck: f64, h_ef: f64) -> f64 {
    k1 * f_ck.sqrt() * h_ef.powf(1.5)
}

/// 🧱 `ψ_s,N = min(0.7 + 0.3 c/c_cr, 1)` of the smallest edge distance `c`.
pub fn psi_s_n(c_min: f64, c_cr: f64) -> f64 {
    (0.7 + 0.3 * c_min / c_cr).min(1.0)
}

/// 🧱 `ψ_re,N = min(0.5 + h_ef/200, 1)`, one for a dense reinforcement.
pub fn psi_re_n(h_ef: f64, dense_reinforcement: bool) -> f64 {
    if dense_reinforcement { 1.0 } else { (0.5 + h_ef / 200.0).min(1.0) }
}

/// 🧱 `ψ_ec,N = Π min(1/(1 + 2 e/s_cr,N), 1)` with `s_cr,N = 3 h_ef` for the load eccentricities of both axes.
pub fn psi_ec_n(e_nx: f64, e_ny: f64, h_ef: f64) -> f64 {
    (1.0 / (1.0 + 2.0 * e_nx / (3.0 * h_ef))).min(1.0) * (1.0 / (1.0 + 2.0 * e_ny / (3.0 * h_ef))).min(1.0)
}

/// 🧱 `ψ_M,N = max(2 − z/(1.5 h_ef), 1)` for a compression zone at lever `z`; one without.
pub fn psi_m_n(z_compression: f64, h_ef: f64) -> f64 {
    if z_compression > 0.0 { (2.0 - z_compression / (1.5 * h_ef)).max(1.0) } else { 1.0 }
}

/// 🍦 Concrete cone breakout of the tension anchors of `input` with `c_cr = 1.5 h_ef`.
pub fn cone_breakout(input: &ConeInput<'_>) -> Cone {
    let c_cr = 1.5 * input.h_ef;
    let n0_rk_c = cone_n0_rk_c(input.k1, input.f_ck, input.h_ef);
    let ratio = cone_area_ratio(input.anchors, c_cr, input.region);
    let psi_s = psi_s_n(edge_distance(input.anchors, input.region), c_cr);
    let psi_re = psi_re_n(input.h_ef, input.dense_reinforcement);
    let psi_ec = psi_ec_n(input.e_nx, input.e_ny, input.h_ef);
    let psi_m = psi_m_n(input.z_compression, input.h_ef);
    Cone { c_cr, n0_rk_c, ratio, psi_s, psi_re, psi_ec, psi_m, n_rk_c: n0_rk_c * ratio * psi_s * psi_re * psi_ec * psi_m }
}

/// 🧲 Head bearing area `A_h = π/4 (d_h² − d²)` of a headed fastener.
pub fn head_bearing_area(d: f64, d_h: f64) -> f64 {
    PI / 4.0 * (d_h * d_h - d * d)
}

/// 🧲 Pull-out `N_Rk,p = k_2 A_h f_ck` (EN 1992-4 7.2.1.5); `k2` comes from the annex row of the crack state.
pub fn pullout_n_rk_p(k2: f64, f_ck: f64, d: f64, d_h: f64) -> f64 {
    k2 * head_bearing_area(d, d_h) * f_ck
}

/// 🧲 Pull-out `N_Rk,p = N⁰_Rk,p √(f_ck/20)` from the characteristic resistance of an approval.
pub fn pullout_n_rk_p_scaled(n0_rk_p: f64, f_ck: f64) -> f64 {
    n0_rk_p * (f_ck / 20.0).sqrt()
}

/// 🪓 `ψ_h,sp = min((h/h_min)^(2/3), max((2 h_ef/h_min)^(2/3), 1))` of the splitting failure.
pub fn psi_h_sp(h: f64, h_min: f64, h_ef: f64) -> f64 {
    (h / h_min).powf(2.0 / 3.0).min((2.0 * h_ef / h_min).powf(2.0 / 3.0).max(1.0))
}

/// 🪓 `N_Rk,sp = min(N⁰_Rk,c, N_Rk,p) ψ` of the splitting failure (EN 1992-4 7.2.1.7) with the edge or member factor `psi`.
pub fn splitting_n_rk_sp(n0_rk_c: f64, n_rk_p: f64, psi: f64) -> f64 {
    n0_rk_c.min(n_rk_p) * psi
}

/// 🔩 Partial-factor row of the anchor steel: the validity limits and the factors of the tension and shear failure (EN 1992-4 Table 4.1); the caller reads every value from the registry rows `ndp.en1992.anchor-gamma-ms.*`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SteelPartialFactors {
    pub f_uk_max: f64,
    pub ratio_max: f64,
    pub slope_n: f64,
    pub min_n: f64,
    pub slope_v: f64,
    pub min_v: f64,
    pub other: f64,
}

impl SteelPartialFactors {
    /// 🔩 `γ_Ms` of the tension (N) or shear (V) failure.
    pub fn gamma_ms(&self, f_uk: f64, f_yk: f64, shear: bool) -> f64 {
        if f_uk > self.f_uk_max || f_yk / f_uk > self.ratio_max {
            return self.other;
        }
        if shear { (self.slope_v * f_uk / f_yk).max(self.min_v) } else { (self.slope_n * f_uk / f_yk).max(self.min_n) }
    }
}

/// 🔩 Steel tension `N_Rk,s = A_s f_uk` (EN 1992-4 7.2.1.2).
pub fn steel_n_rk_s(a_s: f64, f_uk: f64) -> f64 {
    a_s * f_uk
}

/// 🔩 Steel shear without lever arm `V_Rk,s = k_7 0.5 A_s f_uk` (EN 1992-4 7.2.2.3); `k7` is 1 without and 0.8 with grout.
pub fn steel_v_rk_s(a_s: f64, f_uk: f64, k7: f64) -> f64 {
    k7 * 0.5 * a_s * f_uk
}

/// 🔩 Steel shear with lever arm `V_Rk,s = α_M M⁰_Rk,s (1 − N_Ed/N_Rd,s)/l`, `M⁰_Rk,s = 1.2 W_el f_uk`; `n_utilisation` is `N_Ed/N_Rd,s`.
pub fn steel_v_rk_s_lever(alpha_m: f64, w_el: f64, f_uk: f64, lever: f64, n_utilisation: f64) -> f64 {
    alpha_m * 1.2 * w_el * f_uk * (1.0 - n_utilisation).max(0.0) / lever
}

/// 🧱 Inputs of the concrete edge breakout in shear; `k9` comes from the annex row of the crack state.
#[derive(Clone, Copy, Debug)]
pub struct EdgeInput {
    pub f_ck: f64,
    pub k9: f64,
    pub d_nom: f64,
    pub h_ef: f64,
    pub c1: f64,
    pub h: f64,
    pub c2_left: f64,
    pub c2_right: f64,
    pub s2: f64,
    pub alpha_v_deg: f64,
    pub e_v: f64,
    pub psi_re: f64,
}

impl EdgeInput {
    /// 🧱 Single anchor at `c1` from the edge with free sides, load towards the edge, no eccentricity, no edge reinforcement.
    pub fn single(f_ck: f64, k9: f64, d_nom: f64, h_ef: f64, c1: f64, h: f64) -> EdgeInput {
        EdgeInput { f_ck, k9, d_nom, h_ef, c1, h, c2_left: 1e9, c2_right: 1e9, s2: 0.0, alpha_v_deg: 0.0, e_v: 0.0, psi_re: 1.0 }
    }
}

/// 🧱 Concrete edge breakout `V_Rk,c = V⁰_Rk,c A_c,V/A⁰_c,V ψ_s,V ψ_h,V ψ_α,V ψ_ec,V ψ_re,V` (EN 1992-4 7.2.2.5).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Edge {
    pub v0_rk_c: f64,
    pub ratio: f64,
    pub psi_s: f64,
    pub psi_h: f64,
    pub psi_a: f64,
    pub psi_ec: f64,
    pub psi_re: f64,
    pub v_rk_c: f64,
}

/// 🧱 Concrete edge breakout in shear with `l_f = min(h_ef, 8 d_nom)`.
pub fn edge_breakout(input: &EdgeInput) -> Edge {
    let (c1, dn) = (input.c1, input.d_nom);
    let lf = input.h_ef.min(8.0 * dn);
    let alpha = 0.1 * (lf / c1).sqrt();
    let beta = 0.1 * (dn / c1).powf(0.2);
    let v0_rk_c = input.k9 * dn.powf(alpha) * lf.powf(beta) * input.f_ck.sqrt() * c1.powf(1.5);
    let width = input.c2_left.min(1.5 * c1) + input.s2 + input.c2_right.min(1.5 * c1);
    let ratio = width * input.h.min(1.5 * c1) / (4.5 * c1 * c1);
    let psi_s = (0.7 + 0.3 * input.c2_left.min(input.c2_right) / (1.5 * c1)).min(1.0);
    let psi_h = (1.5 * c1 / input.h).sqrt().max(1.0);
    let av = input.alpha_v_deg.to_radians();
    let psi_a = (1.0 / (av.cos().powi(2) + (av.sin() / 2.5).powi(2))).sqrt().max(1.0);
    let psi_ec = (1.0 / (1.0 + 2.0 * input.e_v / (3.0 * c1))).min(1.0);
    Edge { v0_rk_c, ratio, psi_s, psi_h, psi_a, psi_ec, psi_re: input.psi_re, v_rk_c: v0_rk_c * ratio * psi_s * psi_h * psi_a * psi_ec * input.psi_re }
}

/// 🪝 `k_8` of the concrete pry-out: one below `h_ef` 60 mm, else two.
pub fn pryout_k8(h_ef: f64) -> f64 {
    if h_ef < 60.0 { 1.0 } else { 2.0 }
}

/// 🪝 Concrete pry-out `V_Rk,cp = k_8 N_Rk,c` (EN 1992-4 7.2.2.4).
pub fn pryout_v_rk_cp(n_rk_c: f64, h_ef: f64) -> f64 {
    pryout_k8(h_ef) * n_rk_c
}

/// 🔗 Interaction `(N/N_Rd)^p + (V/V_Rd)^p` with negative actions and non-positive resistances counted as zero (EN 1992-4 7.2.3: p = 1.5 concrete, p = 2 steel).
pub fn interaction(n_ed: f64, n_rd: f64, v_ed: f64, v_rd: f64, exponent: f64) -> f64 {
    let term = |ed: f64, rd: f64| if rd > 0.0 { (ed / rd).max(0.0).powf(exponent) } else { 0.0 };
    term(n_ed, n_rd) + term(v_ed, v_rd)
}
