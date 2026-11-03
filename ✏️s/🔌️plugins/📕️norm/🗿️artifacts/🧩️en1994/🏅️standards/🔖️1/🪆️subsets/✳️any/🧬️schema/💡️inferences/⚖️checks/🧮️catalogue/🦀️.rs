//! 🧮️ EN 1994 catalogue formulas: pure SI functions for C-01..C-54 with no model coupling, validated against the spec worked examples E1-01..E1-12.

use std::f64::consts::PI;

//#region 🔖️Constants
/// 📚️ Named material and model constants with their EN provenance (SI).
pub mod constants {
    pub const E_A: f64 = 210e9;
    pub const E_S: f64 = 210e9;
    pub const G_A: f64 = 81e9;
    pub const NU_A: f64 = 0.3;
    pub const REBAR_FSK: f64 = 500e6;
    pub const SHEETING_FYP: f64 = 280e6;
    /// ⚖️ γ_ap of the profiled sheeting, the registry row `ndp.en1994.gamma-ap` (equal in both annexes).
    pub fn gamma_ap() -> f64 {
        semio_s_plugin_norm_compliance::ndp_common("ndp.en1994.gamma-ap")
    }
    /// ⚖️ γ_VS of the longitudinal shear of a composite slab, the registry row `ndp.en1994.gamma-vs` (equal in both annexes).
    pub fn gamma_vs() -> f64 {
        semio_s_plugin_norm_compliance::ndp_common("ndp.en1994.gamma-vs")
    }
    pub const CONCRETE_BEAM_FACTOR: f64 = 0.85;
    pub const CONCRETE_DENSITY_N_M3: f64 = 25e3;
    pub const CREEP_PHI: f64 = 2.5;
    pub const PSI_L_PERMANENT: f64 = 1.1;
    pub const PSI_L_SHRINKAGE: f64 = 0.55;
    pub const SHRINKAGE_STRAIN: f64 = 0.3e-3;
    pub const SHEETING_DEVELOPED_LENGTH_FACTOR: f64 = 1.2;
    pub const RIB_PITCH_M: f64 = 0.25;
    pub const HOGGING_REBAR_DEPTH_M: f64 = 0.04;
    pub const COLUMN_REBAR_EDGE_M: f64 = 0.05;
    pub const PUNCHING_FOOTPRINT_M: f64 = 0.2;
    pub const STEEL_DENSITY: f64 = 7850.0;
    pub const BOARD_LAMBDA: f64 = 0.12;
    pub const BOARD_C: f64 = 1000.0;
    pub const BOARD_RHO: f64 = 800.0;
    pub const FIRE_STEP_S: f64 = 1.0;
    pub const REBAR_FATIGUE_DELTA_SIGMA_RSK: f64 = 162.5e6;
    pub const REBAR_FATIGUE_N_STAR: f64 = 1.0e6;
    pub const REBAR_FATIGUE_K2: f64 = 9.0;
    pub const PATCH_BEARING_LENGTH_M: f64 = 0.1;
    /// ⚖️ γ_s,fat of the reinforcing steel, the registry row `ndp.en1992.gamma-s-fat` (equal in both annexes).
    pub fn gamma_s_fat() -> f64 {
        semio_s_plugin_norm_compliance::ndp_common("ndp.en1992.gamma-s-fat")
    }
}
use constants::*;
//#endregion 🔖️Constants

//#region 🔖️Materials
/// 🧱 Mean axial tensile strength f_ctm [Pa] (EN 1992-1-1 Table 3.1).
pub fn fctm_pa(fck_pa: f64) -> f64 {
    let c = fck_pa / 1e6;
    if c <= 50.0 { 0.30 * c.powf(2.0 / 3.0) * 1e6 } else { 2.12 * (1.0 + (c + 8.0) / 10.0).ln() * 1e6 }
}

/// 🧱 Characteristic 5 % tensile strength f_ctk,0.05 = 0.7 f_ctm [Pa].
pub fn fctk005_pa(fck_pa: f64) -> f64 {
    0.7 * fctm_pa(fck_pa)
}

/// 🧱 Secant modulus E_cm [Pa] from f_cm = f_ck + 8 MPa (EN 1992-1-1 Table 3.1).
pub fn ecm_pa(fck_pa: f64) -> f64 {
    22e9 * ((fck_pa / 1e6 + 8.0) / 10.0).powf(0.3)
}

/// 🧱 Composite-beam concrete design strength 0.85·f_ck/γ_C [Pa] (cl. 6.2.1.2).
pub fn fcc_beam_pa(fck_pa: f64, gamma_c: f64) -> f64 {
    CONCRETE_BEAM_FACTOR * fck_pa / gamma_c
}

/// 🌡️ Characteristic material-scope result of C-01.
#[derive(Clone, Debug, PartialEq)]
pub struct MaterialScope {
    pub in_scope: bool,
    pub violation: String,
}

/// 🔍 C-01 validity ranges: C20/25..C60/75, S235..S460, f_u,stud ≤ 500 MPa.
pub fn material_scope(fck_pa: f64, fy_pa: f64, stud_fu_pa: f64) -> MaterialScope {
    let c = fck_pa / 1e6;
    let y = fy_pa / 1e6;
    let u = stud_fu_pa / 1e6;
    let violation = if !(20.0 - 1e-9..=60.0 + 1e-9).contains(&c) {
        format!("f_ck = {c:.0} MPa outside C20/25..C60/75")
    } else if !(235.0 - 1e-9..=460.0 + 1e-9).contains(&y) {
        format!("f_y = {y:.0} MPa outside S235..S460")
    } else if u > 500.0 + 1e-9 {
        format!("f_u,stud = {u:.0} MPa above 500 MPa")
    } else {
        String::new()
    };
    MaterialScope { in_scope: violation.is_empty(), violation }
}
//#endregion 🔖️Materials

//#region 🔖️EffectiveWidth
/// 📏 C-03 effective width b_eff = b_0 + Σ β_i·b_ei, b_ei = min(L_e/8, b_i), β_i = 0.55 + 0.025·L_e/b_ei ≤ 1 at end supports (cl. 5.4.1.2).
pub fn effective_width_m(le_m: f64, b0_m: f64, b1_m: f64, b2_m: f64, end_support: bool) -> f64 {
    let part = |bi: f64| {
        let be = (le_m / 8.0).min(bi);
        if end_support && be > 0.0 { ((0.55 + 0.025 * le_m / be).min(1.0)) * be } else { be }
    };
    b0_m + part(b1_m) + part(b2_m)
}

/// 📏 Equivalent span L_e between zero-moment points (Fig. 5.1): sagging simple L, end span 0.85·L, interior span 0.7·L, cantilever 2·L; hogging over a support 0.25·(L_left + L_right) with equal neighbouring spans.
pub fn equivalent_span_m(support: semio_s_artifact_norm_en1994_model::SupportCondition, span_m: f64, hogging: bool) -> f64 {
    use semio_s_artifact_norm_en1994_model::SupportCondition as S;
    match (support, hogging) {
        (S::Continuous2Span | S::EndSpan | S::InteriorSpan | S::Cantilever, true) => le_hogging_m(span_m, span_m),
        (S::Continuous2Span | S::EndSpan, false) => le_sagging_m(SpanPosition::End, span_m),
        (S::InteriorSpan, false) => le_sagging_m(SpanPosition::Interior, span_m),
        (S::Cantilever, false) => le_sagging_m(SpanPosition::Cantilever, span_m),
        (S::SimplySupported, _) => span_m,
    }
}

/// 📏 The Fig. 5.1 span position a `support` value denotes.
pub fn span_position(support: semio_s_artifact_norm_en1994_model::SupportCondition) -> SpanPosition {
    use semio_s_artifact_norm_en1994_model::SupportCondition as S;
    match support {
        S::Continuous2Span | S::EndSpan => SpanPosition::End,
        S::InteriorSpan => SpanPosition::Interior,
        S::Cantilever => SpanPosition::Cantilever,
        S::SimplySupported => SpanPosition::Simple,
    }
}
//#endregion 🔖️EffectiveWidth

//#region 🔖️Section
/// 🧱 Doubly symmetric I-section; `a` is the catalogue area so fillets enter through the effective web thickness.
#[derive(Clone, Copy, Debug)]
pub struct ISection {
    pub h: f64,
    pub b: f64,
    pub tf: f64,
    pub tw: f64,
    pub a: f64,
    pub i_y: f64,
}

impl ISection {
    pub fn hw(&self) -> f64 {
        (self.h - 2.0 * self.tf).max(1e-9)
    }
    pub fn tw_eff(&self) -> f64 {
        self.tw.max((self.a - 2.0 * self.b * self.tf) / self.hw())
    }
    pub fn area(&self) -> f64 {
        2.0 * self.b * self.tf + self.hw() * self.tw_eff()
    }
    pub fn iz(&self) -> f64 {
        2.0 * self.tf * self.b.powi(3) / 12.0 + self.hw() * self.tw.powi(3) / 12.0
    }
    /// 🌀 Warping constant I_w = I_z·(h − t_f)²/4 of the doubly symmetric I-section.
    pub fn iw(&self) -> f64 {
        self.iz() * (self.h - self.tf).powi(2) / 4.0
    }
    pub fn it(&self) -> f64 {
        (2.0 * self.b * self.tf.powi(3) + self.hw() * self.tw.powi(3)) / 3.0
    }
    pub fn w_el(&self) -> f64 {
        self.i_y / (self.h / 2.0)
    }
    pub fn w_pl(&self) -> f64 {
        self.b * self.tf * (self.h - self.tf) + self.tw_eff() * self.hw().powi(2) / 4.0
    }
    fn rects(&self, top: f64) -> [(f64, f64, f64); 3] {
        self.rects_k(top, 1.0)
    }
    fn rects_k(&self, top: f64, web_k: f64) -> [(f64, f64, f64); 3] {
        [
            (top, top + self.tf, self.b),
            (top + self.tf, top + self.h - self.tf, self.tw_eff() * web_k),
            (top + self.h - self.tf, top + self.h, self.b),
        ]
    }
}

fn split_force(rects: &[(f64, f64, f64)], y: f64, f: f64) -> (f64, f64, f64, f64) {
    let (mut above, mut below, mut ma, mut mb) = (0.0, 0.0, 0.0, 0.0);
    for &(t, b, w) in rects {
        let yc = y.clamp(t, b);
        let a1 = w * (yc - t) * f;
        let a2 = w * (b - yc) * f;
        above += a1;
        ma += a1 * (t + yc) / 2.0;
        below += a2;
        mb += a2 * (yc + b) / 2.0;
    }
    (above, below, ma, mb)
}
//#endregion 🔖️Section

//#region 🔖️PlasticMoment
/// 📐 Result of a plastic sagging stress-block evaluation (depths measured down from the slab top).
#[derive(Clone, Copy, Debug)]
pub struct Plastic {
    pub m_nm: f64,
    pub nc_n: f64,
    pub x_m: f64,
    pub y_na_m: f64,
    pub zone: &'static str,
}

/// 📐 C-07/C-11/C-12 sagging plastic moment for a concrete force `nc_n` (capped at N_cf and N_pl,a): equilibrium of concrete block, steel compression and steel tension about the slab top.
pub fn plastic_sagging(sec: &ISection, fyd: f64, ys: f64, beff: f64, hc: f64, fcc: f64, nc_n: f64) -> Plastic {
    plastic_sagging_k(sec, 1.0, fyd, ys, beff, hc, fcc, nc_n)
}

/// 📐 Sagging plastic moment with the web strength scaled by `web_k` (cl. 6.2.2.4(2): (1 − ρ)·f_yd in the shear area); a web of unchanged thickness at `web_k`·f_yd carries the same force and moment as a web of `web_k`·t_w at f_yd.
pub fn plastic_sagging_k(sec: &ISection, web_k: f64, fyd: f64, ys: f64, beff: f64, hc: f64, fcc: f64, nc_n: f64) -> Plastic {
    let rects = sec.rects_k(ys, web_k);
    let n_pl = (2.0 * sec.b * sec.tf + sec.hw() * sec.tw_eff() * web_k) * fyd;
    let n_cf = fcc * beff * hc;
    let nc = nc_n.clamp(0.0, n_pl.min(n_cf));
    let (mut lo, mut hi) = (ys, ys + sec.h);
    for _ in 0..200 {
        let mid = 0.5 * (lo + hi);
        let (c, t, _, _) = split_force(&rects, mid, fyd);
        if c - t < -nc { lo = mid } else { hi = mid }
    }
    let y = 0.5 * (lo + hi);
    let (c, t, mc, mt) = split_force(&rects, y, fyd);
    let x = if fcc * beff > 0.0 { nc / (fcc * beff) } else { 0.0 };
    let zc = if c > 0.0 { mc / c } else { 0.0 };
    let zt = if t > 0.0 { mt / t } else { 0.0 };
    let m = t * zt - c * zc - nc * x / 2.0;
    let zone = if nc >= n_pl - 1e-6 || y <= ys + 1e-9 { "slab" } else if y <= ys + sec.tf { "flange" } else { "web" };
    Plastic { m_nm: m, nc_n: nc, x_m: x, y_na_m: y, zone }
}

/// 📐 C-07 full-connection sagging plastic moment M_pl,Rd.
pub fn plastic_full(sec: &ISection, fyd: f64, ys: f64, beff: f64, hc: f64, fcc: f64) -> Plastic {
    plastic_sagging(sec, fyd, ys, beff, hc, fcc, f64::INFINITY)
}

/// 📐 Full-connection sagging plastic moment with the web strength scaled by `web_k`.
pub fn plastic_full_k(sec: &ISection, web_k: f64, fyd: f64, ys: f64, beff: f64, hc: f64, fcc: f64) -> Plastic {
    plastic_sagging_k(sec, web_k, fyd, ys, beff, hc, fcc, f64::INFINITY)
}

/// 📐 C-13 linear interpolation M_Rd = M_a,pl,Rd + (M_pl,Rd − M_a,pl,Rd)·η.
pub fn plastic_linear(m_a_nm: f64, m_pl_nm: f64, eta: f64) -> f64 {
    m_a_nm + (m_pl_nm - m_a_nm) * eta.clamp(0.0, 1.0)
}

/// 📐 C-12 smallest concrete force N_c whose stress-block moment reaches `m_ed`; returns N_cf when even full connection falls short.
pub fn required_concrete_force(sec: &ISection, fyd: f64, ys: f64, beff: f64, hc: f64, fcc: f64, m_ed: f64) -> f64 {
    let n_full = plastic_full(sec, fyd, ys, beff, hc, fcc).nc_n;
    if plastic_sagging(sec, fyd, ys, beff, hc, fcc, 0.0).m_nm >= m_ed { return 0.0; }
    if plastic_sagging(sec, fyd, ys, beff, hc, fcc, n_full).m_nm < m_ed { return n_full; }
    let (mut lo, mut hi) = (0.0, n_full);
    for _ in 0..100 {
        let mid = 0.5 * (lo + hi);
        if plastic_sagging(sec, fyd, ys, beff, hc, fcc, mid).m_nm < m_ed { lo = mid } else { hi = mid }
    }
    hi
}

/// 📐 C-08 hogging plastic moment: concrete neglected, reinforcement force `as_m2·fsd` at depth `y_r` from the slab top, steel tension above and compression below the plastic axis.
pub fn plastic_hogging(sec: &ISection, fyd: f64, ys: f64, as_m2: f64, fsd: f64, y_r: f64) -> (f64, f64) {
    plastic_hogging_k(sec, 1.0, fyd, ys, as_m2, fsd, y_r)
}

/// 📐 Hogging plastic moment with the web strength scaled by `web_k`.
pub fn plastic_hogging_k(sec: &ISection, web_k: f64, fyd: f64, ys: f64, as_m2: f64, fsd: f64, y_r: f64) -> (f64, f64) {
    let rects = sec.rects_k(ys, web_k);
    let fs = (as_m2 * fsd).min((2.0 * sec.b * sec.tf + sec.hw() * sec.tw_eff() * web_k) * fyd);
    let (mut lo, mut hi) = (ys, ys + sec.h);
    for _ in 0..200 {
        let mid = 0.5 * (lo + hi);
        let (ta, cb, _, _) = split_force(&rects, mid, fyd);
        if cb - ta - fs > 0.0 { lo = mid } else { hi = mid }
    }
    let y = 0.5 * (lo + hi);
    let (ta, cb, mta, mcb) = split_force(&rects, y, fyd);
    let zt = if ta > 0.0 { mta / ta } else { 0.0 };
    let zc = if cb > 0.0 { mcb / cb } else { 0.0 };
    (cb * zc - ta * zt - fs * y_r, y)
}

/// 📐 Shear reduction factor of cl. 6.2.2.4(2): ρ = (2·V_Ed/V_pl,Rd − 1)² for V_Ed > ½·V_pl,Rd, capped at 1 (the web strength (1 − ρ)·f_yd never goes negative).
pub fn shear_rho(v_ed: f64, v_pl: f64) -> f64 {
    if v_ed <= 0.5 * v_pl { 0.0 } else { (2.0 * v_ed / v_pl - 1.0).powi(2).min(1.0) }
}

/// 📐 Reduced hogging resistance under high shear (cl. 6.2.2.4(2)): the plastic neutral axis is re-solved with the web strength (1 − ρ)·f_yd.
pub fn shear_reduced_hogging_moment(m_rd_nm: f64, sec: &ISection, ys: f64, fyd: f64, as_m2: f64, fsd: f64, y_r: f64, v_ed: f64, v_pl: f64) -> (f64, f64) {
    let rho = shear_rho(v_ed, v_pl);
    if rho == 0.0 { return (m_rd_nm, 0.0); }
    (plastic_hogging_k(sec, 1.0 - rho, fyd, ys, as_m2, fsd, y_r).0, rho)
}

/// 📐 Reduced sagging resistance under high shear (cl. 6.2.2.4(2)): the plastic neutral axis of the full-connection stress block is re-solved with the web strength (1 − ρ)·f_yd.
pub fn shear_reduced_sagging_moment(p: &Plastic, sec: &ISection, ys: f64, beff: f64, hc: f64, fcc: f64, fyd: f64, v_ed: f64, v_pl: f64) -> (f64, f64) {
    let rho = shear_rho(v_ed, v_pl);
    if rho == 0.0 { return (p.m_nm, 0.0); }
    (plastic_full_k(sec, 1.0 - rho, fyd, ys, beff, hc, fcc).m_nm, rho)
}
//#endregion 🔖️PlasticMoment

//#region 🔖️Elastic
/// 📐 Transformed section properties in steel units (area, neutral-axis depth from slab top, second moment).
#[derive(Clone, Copy, Debug)]
pub struct Elastic {
    pub area: f64,
    pub y_na: f64,
    pub i: f64,
}

/// 📐 C-06 uncracked transformed section with concrete flange `beff × hc` divided by the modular ratio `n`.
pub fn transformed(sec: &ISection, ys: f64, beff: f64, hc: f64, n: f64) -> Elastic {
    let ac = beff * hc / n;
    let ic = beff * hc.powi(3) / 12.0 / n;
    let ya = ys + sec.h / 2.0;
    let a = sec.area();
    let area = a + ac;
    let y_na = (a * ya + ac * hc / 2.0) / area;
    let i = sec.i_y + a * (ya - y_na).powi(2) + ic + ac * (hc / 2.0 - y_na).powi(2);
    Elastic { area, y_na, i }
}

/// 📐 Cracked section (C-06 / C-43): steel plus reinforcement `as_m2` at depth `y_r`, concrete neglected.
pub fn cracked(sec: &ISection, ys: f64, as_m2: f64, y_r: f64) -> Elastic {
    let ar = as_m2 * E_S / E_A;
    let ya = ys + sec.h / 2.0;
    let a = sec.area();
    let area = a + ar;
    let y_na = (a * ya + ar * y_r) / area;
    let i = sec.i_y + a * (ya - y_na).powi(2) + ar * (y_r - y_na).powi(2);
    Elastic { area, y_na, i }
}

/// 📐 Modular ratio n = E_a/E_cm·(1 + ψ_L·φ) (cl. 5.4.2.2).
pub fn modular_ratio(ecm_pa: f64, psi_l: f64, phi: f64) -> f64 {
    E_A / ecm_pa * (1.0 + psi_l * phi)
}

/// 📐 Stresses [Pa] (concrete top, steel top, steel bottom) from a sagging moment on a transformed section; compression positive.
pub fn composite_stresses(el: &Elastic, sec: &ISection, ys: f64, n: f64, m_nm: f64) -> (f64, f64, f64) {
    let s = |y: f64| m_nm * (el.y_na - y) / el.i;
    (s(0.0) / n, s(ys), s(ys + sec.h))
}

/// 📐 Stresses [Pa] in the bare steel section (steel top, steel bottom), compression positive.
pub fn steel_stresses(sec: &ISection, m_nm: f64) -> (f64, f64) {
    let s = m_nm * (sec.h / 2.0) / sec.i_y;
    (s, -s)
}

/// 📐 C-09 elastic resistance factor k: largest k with σ_a(M_a) + k·σ(M_c) within f_yd (steel) and 0.85·f_cd (concrete); returns (k, M_El,Rd).
pub fn elastic_resistance(m_a: f64, sec: &ISection, el_c: &Elastic, ys: f64, n: f64, fyd: f64, fcc: f64, m_c_unit: f64) -> (f64, f64) {
    let (sa_t, sa_b) = steel_stresses(sec, m_a);
    let (uc, ut, ub) = composite_stresses(el_c, sec, ys, n, m_c_unit);
    let mut k = f64::INFINITY;
    let mut lim = |sa: f64, u: f64, limit_c: f64, limit_t: f64| {
        if u > 1e-12 { k = k.min((limit_c - sa) / u) } else if u < -1e-12 { k = k.min((-limit_t - sa) / u) }
    };
    lim(sa_t, ut, fyd, fyd);
    lim(sa_b, ub, fyd, fyd);
    if uc > 1e-12 { k = k.min((fcc - 0.0) / uc) }
    let k = k.max(0.0);
    (k, m_a + k * m_c_unit)
}
//#endregion 🔖️Elastic

//#region 🔖️ShearConnection
/// 🔩 C-16 stud resistance (P_Rd,1, P_Rd,2, P_Rd) [N] per Eq. 6.18/6.19 with α = 0.2(h_sc/d + 1) for 3 ≤ h_sc/d ≤ 4.
pub fn stud_resistance_n(d_m: f64, hsc_m: f64, fu_pa: f64, fck_pa: f64, ecm_pa: f64, gamma_v: f64) -> (f64, f64, f64) {
    let r = hsc_m / d_m;
    let alpha = if r > 4.0 { 1.0 } else { 0.2 * (r + 1.0) };
    let p1 = 0.8 * fu_pa * PI * d_m * d_m / 4.0 / gamma_v;
    let p2 = 0.29 * alpha * d_m * d_m * (fck_pa * ecm_pa).sqrt() / gamma_v;
    (p1, p2, p1.min(p2))
}

/// 🔩 C-17 Table 6.2 upper bound k_t,max for sheeting thickness `t_m`, `nr` studs per rib and through-welded studs (d ≤ 20 mm).
pub fn kt_max(t_m: f64, nr: u32, through_welded: bool) -> f64 {
    let thin = t_m <= 1.0e-3 + 1e-12;
    match (thin, nr, through_welded) {
        (true, 1, true) => 0.85,
        (true, 1, false) => 0.75,
        (true, _, true) => 0.70,
        (true, _, false) => 0.60,
        (false, 1, true) => 1.00,
        (false, 1, false) => 0.75,
        (false, _, true) => 0.80,
        (false, _, false) => 0.60,
    }
}

/// 🔩 C-17 sheeting reduction: k_t = 0.7/√n_r·(b_0/h_p)·(h_sc/h_p − 1) ≤ k_t,max (ribs transverse) or k_l = 0.6·(b_0/h_p)·(h_sc/h_p − 1) ≤ 1 (ribs parallel); 1 for solid slabs. `None` when n_r > 2 (out of scope).
pub fn sheeting_reduction(hp_m: f64, b0_m: f64, hsc_m: f64, d_m: f64, t_m: f64, nr: u32, parallel: bool) -> Option<f64> {
    if hp_m <= 1e-9 { return Some(1.0); }
    let hsc = hsc_m.min(hp_m + 0.075);
    let g = (b0_m / hp_m) * (hsc / hp_m - 1.0);
    if parallel { return Some((0.6 * g).clamp(0.0, 1.0)); }
    if nr > 2 { return None; }
    let kt = 0.7 / (nr.max(1) as f64).sqrt() * g;
    Some(kt.clamp(0.0, kt_max(t_m, nr, d_m <= 0.020 + 1e-9)))
}

/// 🔩 C-14 minimum degree of shear connection for equal flanges (cl. 6.6.1.2): L_e ≤ 25 m → 1 − (355/f_y)(0.75 − 0.03·L_e), at least 0.4; L_e > 25 m → 1. Non-ductile connectors need 1.
pub fn eta_min(le_m: f64, fy_pa: f64, ductile: bool) -> f64 {
    if !ductile || le_m > 25.0 { return 1.0; }
    (1.0 - (355e6 / fy_pa) * (0.75 - 0.03 * le_m)).max(0.4)
}

/// 🔩 Stud ductility condition of cl. 6.6.1.2: 16 ≤ d ≤ 25 mm and h_sc ≥ 4·d.
pub fn studs_ductile(d_m: f64, hsc_m: f64) -> bool {
    (0.016 - 1e-9..=0.025 + 1e-9).contains(&d_m) && hsc_m >= 4.0 * d_m - 1e-9
}

/// 🔩 C-18 longitudinal shear flow v_l,Ed = N_c / L_crit [N/m] and the maximum stud spacing P_Rd / v_l,Ed [m].
pub fn connector_flow(nc_n: f64, l_crit_m: f64, p_rd_n: f64) -> (f64, f64) {
    let v = nc_n / l_crit_m.max(1e-9);
    (v, if v > 1e-9 { p_rd_n / v } else { f64::INFINITY })
}

/// 🔩 C-19 slab longitudinal shear: (v_Ed, v_Rd,max, A_sf/s_f required) for shear flow `vl` [N/m], plane length `l_surf`, flange thickness `hf`, θ_f = 45° (cot θ = 1).
pub fn slab_shear(vl_n_per_m: f64, l_surf_m: f64, hf_m: f64, fck_pa: f64, gamma_c: f64, gamma_s: f64, cot_theta: f64) -> (f64, f64, f64) {
    let v_ed = vl_n_per_m / l_surf_m.max(1e-9);
    let nu = 0.6 * (1.0 - fck_pa / 1e6 / 250.0);
    let fcd = fck_pa / gamma_c;
    let theta = (1.0 / cot_theta).atan();
    let v_rd = nu * fcd * theta.sin() * theta.cos();
    let fyd = REBAR_FSK / gamma_s;
    (v_ed, v_rd, v_ed * hf_m / (fyd * cot_theta))
}

/// 🔩 C-20 detailing utilisation of the geometry rules of §6.6.5 of one stud: h_sc ≥ h_p + 2d, d ≤ 25 mm, t ≤ 1.25 mm. Returns (utilisation, rule).
pub fn stud_detailing_geometry(d_m: f64, hsc_m: f64, hp_m: f64, t_m: f64) -> (f64, &'static str) {
    let rules: [(f64, &'static str); 3] = [((hp_m + 2.0 * d_m) / hsc_m.max(1e-9), "hsc >= hp + 2d"), (d_m / 0.025, "d <= 25"), (t_m / 0.00125, "t <= 1.25 mm")];
    rules.iter().fold((0.0, "none"), |acc, r| if r.0 > acc.0 { *r } else { acc })
}

/// 🔩 C-20 longitudinal spacing utilisation of one gap `s_long_m` between studs of diameter `d_m` in a slab of concrete height `hc_m`: s ≥ 5d and s ≤ min(6h_c, 800 mm). Returns (utilisation, rule).
pub fn stud_spacing_rule(d_m: f64, hc_m: f64, s_long_m: f64) -> (f64, &'static str) {
    let rules: [(f64, &'static str); 2] = [(5.0 * d_m / s_long_m.max(1e-9), "s_long >= 5d"), (s_long_m / (6.0 * hc_m).min(0.8).max(1e-9), "s_long <= min(6hc, 800)")];
    rules.iter().fold((0.0, "none"), |acc, r| if r.0 > acc.0 { *r } else { acc })
}

/// 🔩 C-20 detailing utilisation: worst of the §6.6.5 ratios (longitudinal ≥ 5d, s ≤ min(6h_c, 800 mm), h_sc ≥ h_p + 2d, d ≤ 25 mm, t ≤ 1.25 mm). The transverse spacing rule (≥ 2.5d solid, ≥ 4d otherwise) needs a transverse stud spacing the model does not carry. Returns (utilisation, rule).
pub fn stud_detailing(d_m: f64, hsc_m: f64, hp_m: f64, hc_m: f64, s_long_m: f64, t_m: f64) -> (f64, &'static str) {
    let (spacing, spacing_rule) = stud_spacing_rule(d_m, hc_m, s_long_m);
    let (geometry, geometry_rule) = stud_detailing_geometry(d_m, hsc_m, hp_m, t_m);
    if geometry > spacing { (geometry, geometry_rule) } else { (spacing, spacing_rule) }
}
//#endregion 🔖️ShearConnection

//#region 🔖️Ltb
/// 🌀 C-23 inverted U-frame stiffness k_s = k_1·k_2/(k_1 + k_2), k_1 = α·E_a·I_2/a (α = 2 edge), k_2 = E_a·t_w³/(4(1 − ν²)·h_s).
pub fn uframe_ks(i2_m4_per_m: f64, spacing_m: f64, sec: &ISection, alpha: f64) -> f64 {
    let k1 = alpha * E_A * i2_m4_per_m / spacing_m.max(1e-9);
    let hs = (sec.h - sec.tf).max(1e-9);
    let k2 = E_A * sec.tw.powi(3) / (4.0 * (1.0 - NU_A * NU_A) * hs);
    k1 * k2 / (k1 + k2)
}

/// 🌀 C-23 elastic critical moment M_cr = (k_c·C_4/L)·√((G_a·I_t + k_s·L²/π²)·E_a·I_fz), taking k_c·C_4 = π (uniform moment, conservative).
pub fn mcr_uframe(sec: &ISection, l_m: f64, ks: f64) -> f64 {
    let ifz = sec.iz() / 2.0;
    PI / l_m.max(1e-9) * ((G_A * sec.it() + ks * l_m * l_m / (PI * PI)) * E_A * ifz).sqrt()
}

/// 🌀 Steel-only LTB (construction stage) M_cr with warping, uniform moment.
pub fn mcr_bare(sec: &ISection, l_m: f64) -> f64 {
    let iw = sec.iw();
    let l = l_m.max(1e-9);
    PI / l * (E_A * sec.iz() * G_A * sec.it() * (1.0 + PI * PI * E_A * iw / (G_A * sec.it() * l * l))).sqrt()
}

/// 🌀 C-23 χ_LT (EN 1993-1-1 6.3.2.2, Table 6.4): rolled I-sections curve a/b (α 0.21 for h/b ≤ 2, 0.34 above), welded sections c/d (0.49, 0.76). Returns (λ_LT, χ_LT).
pub fn chi_lt(m_pl_k: f64, mcr: f64, h_over_b: f64, welded: bool) -> (f64, f64) {
    let lam = (m_pl_k / mcr.max(1e-9)).sqrt();
    let alpha = match (welded, h_over_b <= 2.0) {
        (false, true) => 0.21,
        (false, false) => 0.34,
        (true, true) => 0.49,
        (true, false) => 0.76,
    };
    let phi = 0.5 * (1.0 + alpha * (lam - 0.2) + lam * lam);
    (lam, (1.0 / (phi + (phi * phi - lam * lam).max(0.0).sqrt())).min(1.0))
}
//#endregion 🔖️Ltb

//#region 🔖️Serviceability
/// 📉 C-40 simply supported mid-span deflection [m] for permanent `g` and variable `q` line loads [N/m] with stiffness I(n_L) and I(n_0).
pub fn deflection_simple(l: f64, g: f64, q: f64, i_long: f64, i_short: f64) -> f64 {
    5.0 * l.powi(4) / (384.0 * E_A) * (g / i_long + q / i_short)
}

/// 📉 Two-span continuous beam maximum deflection qL⁴/(185·EI) = 0.0054·qL⁴/EI.
pub fn deflection_two_span(l: f64, g: f64, q: f64, i_long: f64, i_short: f64) -> f64 {
    0.0054 * l.powi(4) / E_A * (g / i_long + q / i_short)
}

/// 📉 C-40 shrinkage deflection κ_sh·L²/8, κ_sh = ε_cs·(A_c/n_s)·z_c/I(n_s) for a simply supported beam.
pub fn deflection_shrinkage(l: f64, eps_cs: f64, beff: f64, hc: f64, n_s: f64, z_c: f64, i_s: f64) -> f64 {
    eps_cs * (beff * hc / n_s) * z_c / i_s * l * l / 8.0
}

/// 🧵 C-42 minimum reinforcement A_s,min = k_s·k_c·k·f_ct,eff·A_ct/σ_s with k_c = 1/(1 + h_c/(2·z_0)) + 0.3 ≤ 1.
pub fn as_min_m2(fct_eff: f64, act_m2: f64, hc: f64, z0: f64, sigma_s: f64) -> f64 {
    let kc = (1.0 / (1.0 + hc / (2.0 * z0.max(1e-9))) + 0.3).min(1.0);
    0.9 * kc * 0.8 * fct_eff * act_m2 / sigma_s
}

/// 🧵 C-43 tension-stiffening increment Δσ_s = k·f_ctm/(α_st·ρ_s), α_st = A·I/(A_a·I_a), with the factor `k` of the registry row `ndp_id`.
fn tension_stiffening_increment(ndp_id: &str, fctm: f64, a_c: f64, i_c: f64, a_a: f64, i_a: f64, rho_s: f64) -> f64 {
    semio_s_plugin_norm_compliance::ndp_number(ndp_id, semio_s_plugin_norm_compliance::AnnexChoice::En) * fctm / ((a_c * i_c / (a_a * i_a)) * rho_s.max(1e-12))
}

/// 🧵 C-43 tension-stiffening increment of EN 1994-1-1 7.4.3 Eq. (7.5), the factor from the registry row `ndp.en1994.1-1.tension-stiffening` (0.4).
pub fn delta_sigma_ts(fctm: f64, a_c: f64, i_c: f64, a_a: f64, i_a: f64, rho_s: f64) -> f64 {
    tension_stiffening_increment("ndp.en1994.1-1.tension-stiffening", fctm, a_c, i_c, a_a, i_a, rho_s)
}

/// 🔁️ C-43 tension-stiffening increment of the reinforcement fatigue stress of a bridge (EN 1994-2 6.8.5.4), the factor from the registry row `ndp.en1994.2.fatigue-tension-stiffening` (0.2, unverified).
pub fn delta_sigma_ts_fatigue(fctm: f64, a_c: f64, i_c: f64, a_a: f64, i_a: f64, rho_s: f64) -> f64 {
    tension_stiffening_increment("ndp.en1994.2.fatigue-tension-stiffening", fctm, a_c, i_c, a_a, i_a, rho_s)
}

/// 🧵 EN 1992-1-1 Table 7.3N maximum bar spacing [m] for steel stress σ_s and w_k (0.4/0.3/0.2 mm columns), linearly interpolated; `None` beyond the table.
pub fn max_bar_spacing_m(sigma_s: f64, wk_m: f64) -> Option<f64> {
    let rows: [(f64, [f64; 3]); 6] = [
        (160.0, [300.0, 300.0, 200.0]),
        (200.0, [300.0, 250.0, 150.0]),
        (240.0, [250.0, 200.0, 100.0]),
        (280.0, [200.0, 150.0, 50.0]),
        (320.0, [150.0, 100.0, 0.0]),
        (360.0, [100.0, 50.0, 0.0]),
    ];
    let col = if wk_m >= 0.4e-3 - 1e-12 { 0 } else if wk_m >= 0.3e-3 - 1e-12 { 1 } else { 2 };
    let s = sigma_s / 1e6;
    if s > 360.0 { return None; }
    if s <= 160.0 { return Some(rows[0].1[col] / 1000.0); }
    for w in rows.windows(2) {
        if s <= w[1].0 {
            let t = (s - w[0].0) / (w[1].0 - w[0].0);
            return Some((w[0].1[col] + t * (w[1].1[col] - w[0].1[col])) / 1000.0);
        }
    }
    None
}
//#endregion 🔖️Serviceability

//#region 🔖️Slab
/// 🧩 Composite slab geometry per metre width (SI): overall depth `h`, sheeting depth `hp`, sheeting area `ap`, plastic-axis distance `ep` and centroid distance `e` of the sheeting from the bottom.
#[derive(Clone, Copy, Debug)]
pub struct SlabSection {
    pub h: f64,
    pub hp: f64,
    pub ap: f64,
    pub ep: f64,
    pub e: f64,
    pub fyp_d: f64,
    pub fcd: f64,
}

impl SlabSection {
    pub fn hc(&self) -> f64 {
        self.h - self.hp
    }
    pub fn dp(&self) -> f64 {
        self.h - self.e
    }
}

/// 🧩 C-34 bending resistance per metre at concrete force `nc` [N/m]: full connection N_cf = A_p·f_yp,d, partial M_pl,Rd = N_c·z + M_pr with M_pr = min(1.25·M_pa·(1 − N_c/N_cp), M_pa). Returns (M_Rd, x, z).
pub fn slab_bending(s: &SlabSection, mpa_nm_per_m: f64, nc_n_per_m: f64) -> (f64, f64, f64) {
    let ncf = s.ap * s.fyp_d;
    let nc = nc_n_per_m.clamp(0.0, ncf);
    let x = nc / (0.85 * s.fcd);
    let z = s.h - 0.5 * x - s.ep + (s.ep - s.e) * nc / ncf.max(1e-9);
    let mpr = if nc >= ncf - 1e-9 { 0.0 } else { (1.25 * mpa_nm_per_m * (1.0 - nc / ncf.max(1e-9))).min(mpa_nm_per_m) };
    (nc * z + mpr, x, z)
}

/// 🧩 C-35 m–k longitudinal shear resistance V_l,Rd = (b·d_p/γ_Vs)(m·A_p/(b·L_s) + k) [N/m] with m, k in Pa.
pub fn slab_mk(dp: f64, ap: f64, ls: f64, m_pa: f64, k_pa: f64, gamma_vs: f64) -> f64 {
    dp / gamma_vs * (m_pa * ap / ls.max(1e-9) + k_pa)
}

/// 🧩 C-36 equivalent bond strength τ_u,Rd = V_l,Rd/(b·z) derived from the m–k test values [Pa].
pub fn slab_tau_u(vl_rd: f64, z: f64) -> f64 {
    vl_rd / z.max(1e-9)
}

/// 🧩 C-37 vertical shear V_v,Rd = max(C_Rd,c·k·(100ρ·f_ck)^(1/3), v_min)·b_o·d_p [N/m] (EN 1992-1-1 6.2.2 via EN 1994-1-1 9.7.5), C_Rd,c = 0.18/γ_C, k = min(2, 1 + √(200/d[mm])), v_min = 0.035·k^1.5·√f_ck, ρ = A_p/(b_o·d_p) ≤ 0.02.
pub fn slab_vertical_shear(bo: f64, dp: f64, ap: f64, fck_pa: f64, gamma_c: f64) -> f64 {
    let fck = fck_pa / 1e6;
    let k = (1.0 + (200.0 / (dp * 1e3)).sqrt()).min(2.0);
    let rho = (ap / (bo * dp)).min(0.02);
    let v_c = 0.18 / gamma_c * k * (100.0 * rho * fck).powf(1.0 / 3.0);
    let v_min = 0.035 * k.powf(1.5) * fck.sqrt();
    v_c.max(v_min) * 1e6 * bo * dp
}

/// 🧩 C-38 punching resistance of a composite slab V_p,Rd = C_crit·h_c·v_Rd,c with C_crit at 2·d_p around a square footprint `a` and v_Rd,c = max(0.035·k^1.5·√f_ck, (0.18/γ_C)·k·(100ρ·f_ck)^(1/3)), k = min(2, 1 + √(0.2/d_p)).
pub fn slab_punching(a_m: f64, hc: f64, dp: f64, ap: f64, bo: f64, fck_pa: f64, gamma_c: f64) -> f64 {
    let c_crit = 4.0 * a_m + 2.0 * PI * 2.0 * dp;
    let k = (1.0 + (0.2 / dp).sqrt()).min(2.0);
    let rho = (ap / (bo.max(1e-9) * dp)).min(0.02);
    let fck = fck_pa / 1e6;
    let v_min = 0.035 * k.powf(1.5) * fck.sqrt();
    let v_c = 0.18 / gamma_c * k * (100.0 * rho * fck).powf(1.0 / 3.0);
    c_crit * hc * v_c.max(v_min) * 1e6
}

/// 🧩 C-33 sheeting as formwork: (M_Rd [N·m/m], deflection [m]) for A_p, depth h_p, load w [N/m²], span L.
pub fn sheeting_formwork(ap: f64, hp: f64, fyp: f64, w: f64, l: f64) -> (f64, f64) {
    let i = ap * hp * hp / 4.0;
    (ap * hp / 2.0 * fyp / 2.0, 5.0 * w * l.powi(4) / (384.0 * E_A * i.max(1e-18)))
}

/// 🧩 C-39 slab stiffness: average of uncracked and cracked second moment per metre (steel units) for depth `h`, concrete `hc`, sheeting A_p at `e` from the bottom and modular ratio n.
pub fn slab_effective_i(h: f64, hc: f64, ap: f64, e: f64, n: f64) -> f64 {
    let d = h - e;
    let ac = hc / n;
    let a_tot = ac + ap;
    let yna = (ac * hc / 2.0 + ap * d) / a_tot;
    let i1 = hc.powi(3) / 12.0 / n + ac * (hc / 2.0 - yna).powi(2) + ap * (d - yna).powi(2);
    let (a, b, c) = (0.5 / n, ap, -ap * d);
    let x = (-b + (b * b - 4.0 * a * c).sqrt()) / (2.0 * a);
    let i2 = x.powi(3) / 3.0 / n + ap * (d - x).powi(2);
    0.5 * (i1 + i2)
}
//#endregion 🔖️Slab

//#region 🔖️Fire
/// 🔥 C-44 ISO 834 gas temperature θ_g(t) = 20 + 345·log10(8·t[min] + 1) [°C].
pub fn iso834_c(t_s: f64) -> f64 {
    20.0 + 345.0 * (8.0 * t_s / 60.0 + 1.0).log10()
}

/// 🔥 C-45 specific heat of steel c_a(θ) [J/kgK] (EN 1993-1-2 3.4.1.2).
pub fn steel_specific_heat(theta: f64) -> f64 {
    if theta < 600.0 { 425.0 + 0.773 * theta - 1.69e-3 * theta.powi(2) + 2.22e-6 * theta.powi(3) }
    else if theta < 735.0 { 666.0 + 13002.0 / (738.0 - theta) }
    else if theta < 900.0 { 545.0 + 17820.0 / (theta - 731.0) }
    else { 650.0 }
}

/// 🔥 C-45 strength reduction k_y,θ of structural steel (EN 1993-1-2 Table 3.1, linear between 100 K nodes).
pub fn ky_theta(theta: f64) -> f64 {
    const K: [f64; 13] = [1.0, 1.0, 1.0, 1.0, 1.0, 0.78, 0.47, 0.23, 0.11, 0.06, 0.04, 0.02, 0.0];
    let t = theta.clamp(20.0, 1200.0);
    if t <= 100.0 { return 1.0; }
    let i = ((t / 100.0).floor() as usize).min(11);
    let f = (t - 100.0 * i as f64) / 100.0;
    K[i] + f * (K[i + 1] - K[i])
}

/// 🔥 C-47 protected steel temperature θ_a [°C] after every step of Δt = 1 s up to `t_end_s` (entry 0 is 20 °C, the last entry the end time) by explicit time stepping of EN 1993-1-2 Eq. 4.27 against the ISO 834 curve; `am_v` is A_p/V [1/m].
pub fn protected_steel_history_c(t_end_s: f64, am_v: f64, d_p: f64, lambda_p: f64, c_p: f64, rho_p: f64) -> Vec<f64> {
    let phi = c_p * rho_p / (steel_specific_heat(20.0) * STEEL_DENSITY) * d_p * am_v;
    let mut theta_a = 20.0;
    let mut t = 0.0;
    let mut out = vec![theta_a];
    while t < t_end_s - 1e-9 {
        let dt = FIRE_STEP_S.min(t_end_s - t);
        let g0 = iso834_c(t);
        let g1 = iso834_c(t + dt);
        let ca = steel_specific_heat(theta_a);
        let d = (lambda_p / d_p) * am_v / (ca * STEEL_DENSITY) * (g0 - theta_a) / (1.0 + phi / 3.0) * dt - ((phi / 10.0).exp() - 1.0) * (g1 - g0);
        let d = if g0 - theta_a > 0.0 { d.max(0.0) } else { d };
        theta_a += d;
        t += dt;
        out.push(theta_a);
    }
    out
}

/// 🔥 C-47 protected steel temperature θ_a(t) [°C] at `t_end_s` (the last entry of [`protected_steel_history_c`]).
pub fn protected_steel_temperature_c(t_end_s: f64, am_v: f64, d_p: f64, lambda_p: f64, c_p: f64, rho_p: f64) -> f64 {
    protected_steel_history_c(t_end_s, am_v, d_p, lambda_p, c_p, rho_p).last().copied().unwrap_or(20.0)
}

/// 🔥 C-47 critical steel temperature θ_cr = 39.19·ln(1/(0.9674·μ_0^3.833) − 1) + 482 [°C] (EN 1993-1-2 4.2.4).
pub fn critical_temperature_c(mu0: f64) -> f64 {
    let m = mu0.clamp(0.013, 0.99);
    39.19 * (1.0 / (0.9674 * m.powf(3.833)) - 1.0).ln() + 482.0
}
//#endregion 🔖️Fire

//#region 🔖️Column
/// 🏛 Equivalent square composite column section: outer side `b`, steel ring thickness `t`, rebar area `as_m2` in two layers at ±`yr`.
#[derive(Clone, Copy, Debug)]
pub struct ColSection {
    pub b: f64,
    pub t: f64,
    pub as_m2: f64,
    pub yr: f64,
    pub fyd: f64,
    pub fcd: f64,
    pub fsd: f64,
}

#[derive(Clone, Copy)]
enum Mat {
    Steel,
    Conc,
    Rebar,
}

impl ColSection {
    fn parts(&self) -> Vec<(f64, f64, f64, Mat)> {
        let h = self.b / 2.0;
        let core = (h - self.t).max(0.0);
        let mut v = vec![
            (h - self.t, h, self.b, Mat::Steel),
            (-h, -h + self.t, self.b, Mat::Steel),
            (-core, core, 2.0 * self.t, Mat::Steel),
            (-core, core, (self.b - 2.0 * self.t).max(0.0), Mat::Conc),
        ];
        if self.as_m2 > 0.0 {
            let e = 1e-6;
            let w = self.as_m2 / 2.0 / (2.0 * e);
            v.push((self.yr - e, self.yr + e, w, Mat::Rebar));
            v.push((-self.yr - e, -self.yr + e, w, Mat::Rebar));
        }
        v
    }
    fn f(&self, m: Mat) -> f64 {
        match m { Mat::Steel => self.fyd, Mat::Conc => self.fcd, Mat::Rebar => self.fsd }
    }
    /// 🏛 Axial force and moment about the centre for a plastic axis at `xi` with compression above it.
    pub fn nm_at(&self, xi: f64) -> (f64, f64) {
        let (mut n, mut m) = (0.0, 0.0);
        for (lo, hi, w, mat) in self.parts() {
            let f = self.f(mat);
            let cl = xi.max(lo);
            if hi > cl {
                let c = w * (hi - cl) * f;
                n += c;
                m += c * (cl + hi) / 2.0;
            }
            if !matches!(mat, Mat::Conc) {
                let th = xi.min(hi);
                if th > lo {
                    let t = w * (th - lo) * f;
                    n -= t;
                    m -= t * (lo + th) / 2.0;
                }
            }
        }
        (n, m)
    }
    pub fn n_pl(&self) -> f64 {
        self.nm_at(-self.b).0
    }
    pub fn n_pm(&self) -> f64 {
        self.parts().iter().filter(|p| matches!(p.3, Mat::Conc)).map(|p| p.2 * (p.1 - p.0) * self.fcd).sum()
    }
    pub fn area_steel(&self) -> f64 {
        4.0 * self.t * (self.b - self.t)
    }
    pub fn area_conc(&self) -> f64 {
        (self.b - 2.0 * self.t).powi(2)
    }
}

/// 🏛 C-29 interaction polygon vertices A, C, D, B as (N, M) [N, N·m]: A (N_pl, 0), C (N_pm, M_pl), D (N_pm/2, M_max), B (0, M_pl), found by numeric plastic-axis integration.
pub fn interaction_polygon(s: &ColSection) -> [(f64, f64); 4] {
    let npm = s.n_pm();
    let (mut lo, mut hi) = (-s.b / 2.0, s.b / 2.0);
    for _ in 0..200 {
        let mid = 0.5 * (lo + hi);
        if s.nm_at(mid).0 > 0.0 { lo = mid } else { hi = mid }
    }
    let b_pt = s.nm_at(0.5 * (lo + hi));
    let d_pt = s.nm_at(0.0);
    [(s.n_pl(), 0.0), (npm, b_pt.1), (0.5 * npm, d_pt.1), (0.0, b_pt.1)]
}

/// 🏛 C-29 normalised moment resistance μ_d(N) along A–C–D–B.
pub fn mu_d(poly: &[(f64, f64); 4], n: f64) -> f64 {
    let m_pl = poly[3].1;
    let pts = [(poly[0].0, 0.0), (poly[1].0, poly[1].1 / m_pl), (poly[2].0, poly[2].1 / m_pl), (poly[3].0, 1.0)];
    if n >= pts[0].0 { return 0.0; }
    for w in pts.windows(2) {
        if n <= w[0].0 && n >= w[1].0 {
            let t = (n - w[0].0) / (w[1].0 - w[0].0);
            return w[0].1 + t * (w[1].1 - w[0].1);
        }
    }
    1.0
}

/// 🏛 C-29 α_M = 0.9 for S235..S355, 0.8 for S420/S460.
pub fn alpha_m(fy_pa: f64) -> f64 {
    if fy_pa <= 355.1e6 { 0.9 } else { 0.8 }
}

/// 🏛 C-26 confinement of filled circular sections: (η_a, η_c) for relative slenderness `lam` and eccentricity ratio e/d (0 for λ̄ > 0.5 or e/d > 0.1).
pub fn confinement(lam: f64, e_over_d: f64) -> (f64, f64) {
    if lam > 0.5 || e_over_d > 0.1 { return (1.0, 0.0); }
    let a0 = (0.25 * (3.0 + 2.0 * lam)).min(1.0);
    let c0 = (4.9 - 18.5 * lam + 17.0 * lam * lam).max(0.0);
    (a0 + (1.0 - a0) * 10.0 * e_over_d, c0 * (1.0 - 10.0 * e_over_d))
}

/// 🏛 C-27 effective flexural stiffness (EI)_eff = E_a·I_a + E_s·I_s + K_e·E_cm·I_c, K_e = 0.6.
pub fn ei_eff(ia: f64, is: f64, ic: f64, ecm: f64, ke: f64) -> f64 {
    E_A * ia + E_S * is + ke * ecm * ic
}

/// 🏛 C-27 χ from EN 1993-1-1 6.3.1.2 with imperfection α (a 0.21, b 0.34, c 0.49).
pub fn chi_column(lam: f64, alpha: f64) -> f64 {
    let phi = 0.5 * (1.0 + alpha * (lam - 0.2) + lam * lam);
    (1.0 / (phi + (phi * phi - lam * lam).max(0.0).sqrt())).min(1.0)
}

/// 🏛 C-28 reduced concrete modulus E_cm,eff = E_cm/(1 + (N_G,Ed/N_Ed)·φ_t).
pub fn ecm_eff(ecm: f64, ng: f64, ned: f64, phi_t: f64) -> f64 {
    ecm / (1.0 + ng / ned.max(1e-9) * phi_t)
}

/// 🏛 C-30 second-order amplification k = max(1, β/(1 − N_Ed/N_cr,eff)); k = 1 when N_Ed/N_cr,eff ≤ 0.1.
pub fn second_order_k(ned: f64, ncr_eff: f64, beta: f64) -> f64 {
    if ned / ncr_eff <= 0.1 { return 1.0; }
    (beta / (1.0 - ned / ncr_eff)).max(1.0)
}
//#endregion 🔖️Column

//#region 🔖️Fatigue
/// 🌉 C-52 reinforcement fatigue resistance Δσ_Rsk·(N*/N)^(1/k_2)/γ_s,fat [Pa] (EN 1992-1-1 Table 6.3N straight bars).
pub fn rebar_fatigue_resistance_pa(n_cycles: f64) -> f64 {
    fatigue::rebar_curve().resistance_pa(n_cycles)
}
//#endregion 🔖️Fatigue

//#region 🔖️SpanKinds
/// 📏 Position of a span in a continuous beam for the equivalent length L_e of Fig. 5.1.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SpanPosition {
    Simple,
    End,
    Interior,
    Cantilever,
}

/// 📏 C-03 sagging equivalent span L_e: simple L, end span 0.85·L_1, interior span 0.7·L_2, cantilever 2·L_3 (Fig. 5.1).
pub fn le_sagging_m(position: SpanPosition, span_m: f64) -> f64 {
    span_m * match position {
        SpanPosition::Simple => 1.0,
        SpanPosition::End => 0.85,
        SpanPosition::Interior => 0.7,
        SpanPosition::Cantilever => 2.0,
    }
}

/// 📏 C-03 hogging equivalent span over a support: 0.25·(L_left + L_right) of the two adjacent spans (Fig. 5.1).
pub fn le_hogging_m(left_m: f64, right_m: f64) -> f64 {
    0.25 * (left_m + right_m)
}

/// 📏 C-04 minimum-to-maximum adjacent span ratio of a continuous beam; the simplified analysis of cl. 5.4.2 needs at least 0.6.
pub fn adjacent_span_ratio(spans_m: &[f64]) -> f64 {
    let (lo, hi) = spans_m.iter().fold((f64::INFINITY, 0.0_f64), |(lo, hi), &s| (lo.min(s), hi.max(s)));
    if hi > 0.0 { lo / hi } else { 1.0 }
}
//#endregion 🔖️SpanKinds

//#region 🔖️NonlinearBending
/// 📈 Strain limits and design strengths of the non-linear bending resistance (cl. 6.2.1.4, EN 1992-1-1 3.1.7 parabola-rectangle, bilinear steel without hardening).
#[derive(Clone, Copy, Debug)]
pub struct NonlinearInput {
    pub fyd: f64,
    pub fcd: f64,
    pub as_m2: f64,
    pub y_rebar_m: f64,
    pub fsd: f64,
    pub eps_c2: f64,
    pub eps_cu: f64,
    pub eps_su: f64,
}

impl NonlinearInput {
    pub fn standard(fyd: f64, fcd: f64, as_m2: f64, y_rebar_m: f64) -> Self {
        NonlinearInput { fyd, fcd, as_m2, y_rebar_m, fsd: REBAR_FSK / 1.15, eps_c2: 2.0e-3, eps_cu: 3.5e-3, eps_su: 0.02 }
    }
}

/// 📈 One point of the moment-curvature curve with the top-fibre strain and the largest steel tension strain.
#[derive(Clone, Copy, Debug)]
pub struct NonlinearPoint {
    pub kappa: f64,
    pub m_nm: f64,
    pub eps_top: f64,
    pub eps_steel_max: f64,
}

fn concrete_stress(inp: &NonlinearInput, eps: f64) -> f64 {
    if eps <= 0.0 { 0.0 } else if eps >= inp.eps_c2 { inp.fcd } else { inp.fcd * (1.0 - (1.0 - eps / inp.eps_c2).powi(2)) }
}

fn steel_stress(f: f64, e: f64, eps: f64) -> f64 {
    (e * eps).clamp(-f, f)
}

fn fibre_state(sec: &ISection, ys: f64, beff: f64, hc: f64, inp: &NonlinearInput, kappa: f64, eps_top: f64) -> (f64, f64, f64) {
    const STRIPS: usize = 200;
    let (mut n, mut m, mut eps_max) = (0.0, 0.0, f64::NEG_INFINITY);
    let strain = |y: f64| eps_top - kappa * y;
    let mut add = |force: f64, y: f64| {
        n += force;
        m -= force * y;
    };
    for i in 0..STRIPS {
        let y = (i as f64 + 0.5) * hc / STRIPS as f64;
        add(concrete_stress(inp, strain(y)) * beff * hc / STRIPS as f64, y);
    }
    for (t, b, w) in sec.rects(ys) {
        for i in 0..STRIPS {
            let y = t + (i as f64 + 0.5) * (b - t) / STRIPS as f64;
            add(steel_stress(inp.fyd, E_A, strain(y)) * w * (b - t) / STRIPS as f64, y);
            eps_max = eps_max.max(-strain(y));
        }
    }
    if inp.as_m2 > 0.0 {
        add(steel_stress(inp.fsd, E_S, strain(inp.y_rebar_m)) * inp.as_m2, inp.y_rebar_m);
    }
    (n, m, eps_max)
}

/// 📈 C-10 moment M and strains at curvature `kappa` [1/m] with the neutral axis found from N = 0 by bisection on the top-fibre strain.
pub fn moment_at_curvature(sec: &ISection, ys: f64, beff: f64, hc: f64, inp: &NonlinearInput, kappa: f64) -> NonlinearPoint {
    let (mut lo, mut hi) = (-0.1, 0.1);
    for _ in 0..80 {
        let mid = 0.5 * (lo + hi);
        if fibre_state(sec, ys, beff, hc, inp, kappa, mid).0 < 0.0 { lo = mid } else { hi = mid }
    }
    let eps_top = 0.5 * (lo + hi);
    let (_, m, eps_steel_max) = fibre_state(sec, ys, beff, hc, inp, kappa, eps_top);
    NonlinearPoint { kappa, m_nm: m, eps_top, eps_steel_max }
}

fn within_limits(p: &NonlinearPoint, inp: &NonlinearInput) -> bool {
    p.eps_top <= inp.eps_cu && p.eps_steel_max <= inp.eps_su
}

/// 📈 C-10 curvature at which the first strain limit (concrete ε_cu or steel ε_su) is reached, by doubling and bisection.
pub fn ultimate_curvature(sec: &ISection, ys: f64, beff: f64, hc: f64, inp: &NonlinearInput) -> f64 {
    let mut hi = 1e-4;
    for _ in 0..40 {
        if !within_limits(&moment_at_curvature(sec, ys, beff, hc, inp, hi), inp) { break }
        hi *= 2.0;
    }
    let mut lo = 0.0;
    for _ in 0..60 {
        let mid = 0.5 * (lo + hi);
        if within_limits(&moment_at_curvature(sec, ys, beff, hc, inp, mid), inp) { lo = mid } else { hi = mid }
    }
    lo
}

/// 📈 C-10 moment-curvature curve with `count` points up to the ultimate curvature.
pub fn moment_curvature(sec: &ISection, ys: f64, beff: f64, hc: f64, inp: &NonlinearInput, count: usize) -> Vec<NonlinearPoint> {
    let ku = ultimate_curvature(sec, ys, beff, hc, inp);
    (0..=count).map(|i| moment_at_curvature(sec, ys, beff, hc, inp, ku * i as f64 / count.max(1) as f64)).collect()
}

/// 📈 C-10 non-linear bending resistance M_Rd,nl: the curve maximum within the strain limits.
pub fn nonlinear_mrd(sec: &ISection, ys: f64, beff: f64, hc: f64, inp: &NonlinearInput) -> f64 {
    moment_curvature(sec, ys, beff, hc, inp, 60).iter().map(|p| p.m_nm).fold(0.0, f64::max)
}
//#endregion 🔖️NonlinearBending

//#region 🔖️PatchLoading
/// 🧷 C-24 patch-loading resistance of an unstiffened web, type a, per EN 1993-1-5 §6: F_Rd = χ_F·ℓ_y·t_w·f_yw/γ_M1. Returns (F_Rd, χ_F, ℓ_y, λ_F).
pub fn patch_loading(hw_m: f64, tw_m: f64, bf_m: f64, tf_m: f64, fyw_pa: f64, fyf_pa: f64, ss_m: f64, a_m: f64, gamma_m1: f64) -> (f64, f64, f64, f64) {
    let kf = 6.0 + 2.0 * (hw_m / a_m).powi(2);
    let m1 = fyf_pa * bf_m / (fyw_pa * tw_m);
    let fcr = 0.9 * kf * E_A * tw_m.powi(3) / hw_m;
    let ly_of = |m2: f64| (ss_m + 2.0 * tf_m * (1.0 + (m1 + m2).sqrt())).min(a_m);
    let lambda_of = |ly: f64| (ly * tw_m * fyw_pa / fcr).sqrt();
    let mut m2 = 0.0;
    let mut ly = ly_of(m2);
    let mut lambda = lambda_of(ly);
    if lambda > 0.5 {
        m2 = 0.02 * (hw_m / tf_m).powi(2);
        ly = ly_of(m2);
        lambda = lambda_of(ly);
    }
    let chi = (0.5 / lambda.max(1e-9)).min(1.0);
    (chi * ly * tw_m * fyw_pa / gamma_m1, chi, ly, lambda)
}

/// 🧷 C-24 interaction of transverse force and bending: 0.8·F_Ed/F_Rd + M_Ed/M_Rd ≤ 1.4 (EN 1993-1-5 Eq. 7.2); returns the utilisation against 1.4.
pub fn patch_interaction(f_ed: f64, f_rd: f64, m_ed: f64, m_rd: f64) -> f64 {
    (0.8 * f_ed / f_rd.max(1e-9) + m_ed / m_rd.max(1e-9)) / 1.4
}
//#endregion 🔖️PatchLoading

//#region 🔖️LoadIntroduction
/// 🔗 Steel-concrete interface of a composite column for the introduction of loads (cl. 6.7.4.3, Table 6.6).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum InterfaceKind {
    ConcreteEncased,
    FilledCircular,
    FilledRectangular,
    PartiallyEncasedFlange,
    PartiallyEncasedWeb,
}

/// 🔗 C-31 design shear strength τ_Rd of the interface [Pa] (Table 6.6: 0.30, 0.55, 0.40, 0.20, 0.00 MPa).
pub fn interface_tau_rd_pa(kind: InterfaceKind) -> f64 {
    1e6 * match kind {
        InterfaceKind::ConcreteEncased => 0.30,
        InterfaceKind::FilledCircular => 0.55,
        InterfaceKind::FilledRectangular => 0.40,
        InterfaceKind::PartiallyEncasedFlange => 0.20,
        InterfaceKind::PartiallyEncasedWeb => 0.0,
    }
}

/// 🔗 C-31 introduction length l_v = min(2·d, L/3) (cl. 6.7.4.2(1)) with `d` the lesser cross-section dimension.
pub fn introduction_length_m(d_m: f64, column_length_m: f64) -> f64 {
    (2.0 * d_m).min(column_length_m / 3.0)
}

/// 🔗 C-31 interface shear τ_Ed = N_intro/(u·l_v) against τ_Rd; returns (τ_Ed, τ_Rd, utilisation). Perimeter `u` is the steel-concrete contact perimeter.
pub fn load_introduction(kind: InterfaceKind, n_intro_n: f64, perimeter_m: f64, length_m: f64) -> (f64, f64, f64) {
    let ed = n_intro_n / (perimeter_m * length_m).max(1e-9);
    let rd = interface_tau_rd_pa(kind);
    (ed, rd, if rd > 0.0 { ed / rd } else if ed > 0.0 { f64::INFINITY } else { 0.0 })
}

/// 🔗 C-31 number of studs needed to carry the introduced force N_intro with the design resistance P_Rd.
pub fn introduction_studs(n_intro_n: f64, p_rd_n: f64) -> u32 {
    (n_intro_n / p_rd_n.max(1e-9)).ceil().max(0.0) as u32
}
//#endregion 🔖️LoadIntroduction

//#region 🔖️BridgeActions
/// 🌉 C-48 EN 1991-2 Load Model 1 for lane `lane` (1-based, 3 lanes defined): (tandem axle load Q_ik [N], UDL q_ik [N/m²]); the German annex uses 240/160/80 kN, EN 1991-2 recommends 300/200/100 kN.
pub fn lm1_lane(german_annex: bool, lane: usize) -> (f64, f64) {
    let axle_kn = if german_annex { [240.0, 160.0, 80.0] } else { [300.0, 200.0, 100.0] };
    let udl_kpa = [9.0, 2.5, 2.5];
    let i = lane.clamp(1, 3) - 1;
    (axle_kn[i] * 1e3, udl_kpa[i] * 1e3)
}

/// 🌉 C-48 lane width w_l = 3 m (EN 1991-2 4.2.3 for carriageway width w ≥ 6 m).
pub const LANE_WIDTH_M: f64 = 3.0;

/// 🌉 C-48 tandem axle spacing 1.2 m and wheel track 2.0 m (EN 1991-2 Fig. 4.2b).
pub const TANDEM_AXLE_SPACING_M: f64 = 1.2;

/// 🌉 C-48 maximum bending moment [N·m] and support shear [N] of one LM1 lane on a simple span: tandem with `axle_n` per axle at 1.2 m plus UDL `udl_n_m2` over the lane width.
pub fn lm1_simple_span_effects(span_m: f64, axle_n: f64, udl_n_m2: f64) -> (f64, f64) {
    let d = TANDEM_AXLE_SPACING_M.min(span_m);
    let m_tandem = 2.0 * axle_n / span_m * (span_m / 2.0 - d / 4.0).powi(2);
    let v_tandem = axle_n * (2.0 - d / span_m);
    let q = udl_n_m2 * LANE_WIDTH_M;
    (m_tandem + q * span_m * span_m / 8.0, v_tandem + q * span_m / 2.0)
}
//#endregion 🔖️BridgeActions

//#region 🔖️Mounts
#[path = "📉️continuous/🦀️.rs"]
pub mod continuous;
#[path = "🏛️encased/🦀️.rs"]
pub mod encased;
#[path = "🌡️strain-curve/🦀️.rs"]
pub mod strain_curve;
#[path = "🔥️column-fire/🦀️.rs"]
pub mod column_fire;
#[path = "🔥️beam-fire/🦀️.rs"]
pub mod beam_fire;
#[path = "🔁️fatigue/🦀️.rs"]
pub mod fatigue;
#[path = "🌉️bridge/🦀️.rs"]
pub mod bridge;
#[path = "📏️crack-width/🦀️.rs"]
pub mod crack_width;
#[path = "🌊️vibration/🦀️.rs"]
pub mod vibration;
//#endregion 🔖️Mounts

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
