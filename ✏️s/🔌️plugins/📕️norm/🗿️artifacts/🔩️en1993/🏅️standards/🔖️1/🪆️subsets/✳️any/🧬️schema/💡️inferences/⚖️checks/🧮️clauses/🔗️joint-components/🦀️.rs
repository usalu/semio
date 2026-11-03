//! 🔗️ EN 1993-1-8 §6 component method: effective lengths, T-stub modes 1 to 3 with and without prying, column web components, row reduction, M_j,Rd, S_j and base plates (mm, N, MPa). See [EN 1993-1-8 §6](https://eurocodes.jrc.ec.europa.eu/).

use std::f64::consts::PI;

pub struct LEff {
    pub cp: f64,
    pub nc: f64,
}

impl LEff {
    pub fn l1(&self) -> f64 {
        self.cp.min(self.nc)
    }
    pub fn l2(&self) -> f64 {
        self.nc
    }
}

/// 🔗️ Unstiffened column flange, row considered individually (Table 6.4).
pub fn l_eff_column_flange_single(m: f64, e: f64) -> LEff {
    LEff { cp: 2.0 * PI * m, nc: 4.0 * m + 1.25 * e }
}

/// 🔗️ Unstiffened column flange, inner row as part of a group.
pub fn l_eff_column_flange_inner_group(p: f64) -> LEff {
    LEff { cp: 2.0 * p, nc: p }
}

/// 🔗️ Unstiffened column flange, end row as part of a group.
pub fn l_eff_column_flange_end_group(m: f64, e: f64, p: f64) -> LEff {
    LEff { cp: PI * m + p, nc: 2.0 * m + 0.625 * e + 0.5 * p }
}

/// 🔗️ End-plate row outside the tension flange, individually (Table 6.6) with the extension geometry m_x, e_x, width b_p and bolt gauge w.
pub fn l_eff_end_plate_extended(mx: f64, ex: f64, e: f64, w: f64, bp: f64) -> LEff {
    LEff { cp: (2.0 * PI * mx).min(PI * mx + w).min(PI * mx + 2.0 * e), nc: (4.0 * mx + 1.25 * ex).min(e + 2.0 * mx + 0.625 * ex).min(0.5 * bp).min(0.5 * w + 2.0 * mx + 0.625 * ex) }
}

/// 🔗️ End-plate first row below the tension flange, individually, with α from Figure 6.11.
pub fn l_eff_end_plate_first_below(m: f64, alpha: f64) -> LEff {
    LEff { cp: 2.0 * PI * m, nc: alpha * m }
}

/// 🔗️ End-plate first row below the tension flange as part of a group.
pub fn l_eff_end_plate_first_below_group(m: f64, e: f64, p: f64, alpha: f64) -> LEff {
    LEff { cp: PI * m + p, nc: 0.5 * p + alpha * m - (2.0 * m + 0.625 * e) }
}

/// 🔗️ End-plate other inner row, individually and in a group.
pub fn l_eff_end_plate_inner(m: f64, alpha: f64) -> LEff {
    LEff { cp: 2.0 * PI * m, nc: alpha * m }
}

/// 🔗️ Mean m of a welded column flange, m = (w − t_w − 2·0.8√2 a)/2, or of a rolled section, m = (w − t_w)/2 − 0.8 r.
pub fn m_welded(w: f64, tw: f64, a: f64) -> f64 {
    (w - tw - 2.0 * 0.8 * 2.0_f64.sqrt() * a) / 2.0
}

pub fn m_rolled(w: f64, tw: f64, r: f64) -> f64 {
    (w - tw) / 2.0 - 0.8 * r
}

pub struct TStubModes {
    pub m_pl_1: f64,
    pub m_pl_2: f64,
    pub ft_1: f64,
    pub ft_2: f64,
    pub ft_3: f64,
    pub ft_rd: f64,
    pub mode: u8,
    pub prying: bool,
    pub l_b_star: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode1Method {
    One,
    Two,
}

/// 🔗️ T-stub flange in tension (Table 6.2): L_b* = 8.8 m³ A_s n_b/(Σ l_eff,1 t_f³), prying forces develop when L_b ≤ L_b*, absent for L_b > L_b*, then F_T,1-2 = 2 M_pl,1/m and mode 3.
pub fn t_stub_modes(l1: f64, l2: f64, tf: f64, fy: f64, m: f64, n: f64, ew: f64, sum_ft_rd: f64, a_s: f64, n_b: f64, lb: f64, method: Mode1Method, gm0: f64) -> TStubModes {
    let mp1 = 0.25 * l1 * tf * tf * fy / gm0;
    let mp2 = 0.25 * l2 * tf * tf * fy / gm0;
    let l_b_star = 8.8 * m.powi(3) * a_s * n_b / (l1 * tf.powi(3));
    let prying = lb <= l_b_star;
    let ft_3 = sum_ft_rd;
    let (ft_1, ft_2) = if prying {
        let f1 = match method {
            Mode1Method::One => (8.0 * n - 2.0 * ew) * mp1 / (2.0 * m * n - ew * (m + n)),
            Mode1Method::Two => 4.0 * mp1 / m,
        };
        (f1, (2.0 * mp2 + n * sum_ft_rd) / (m + n))
    } else {
        (2.0 * mp1 / m, f64::INFINITY)
    };
    let (ft_rd, mode) = if ft_1 <= ft_2 && ft_1 <= ft_3 { (ft_1, 1) } else if ft_2 <= ft_3 { (ft_2, 2) } else { (ft_3, 3) };
    TStubModes { m_pl_1: mp1, m_pl_2: mp2, ft_1, ft_2, ft_3, ft_rd, mode, prying, l_b_star }
}

/// 🔗️ n = min(e_min, 1.25 m).
pub fn n_prying(e_min: f64, m: f64) -> f64 {
    e_min.min(1.25 * m)
}

/// 🔗️ Reduction factor k_wc for longitudinal stress in the column web, 1 for σ ≤ 0.7 f_y,wc else 1.7 − σ/f_y,wc.
pub fn k_wc(sigma_com: f64, fy: f64) -> f64 {
    if sigma_com <= 0.7 * fy { 1.0 } else { 1.7 - sigma_com / fy }
}

/// 🔗️ b_eff,c,wc = t_fb + 2√2 a_b + 5 (t_fc + s) + s_p.
pub fn b_eff_c_wc(tfb: f64, ab: f64, tfc: f64, s: f64, sp: f64) -> f64 {
    tfb + 2.0 * 2.0_f64.sqrt() * ab + 5.0 * (tfc + s) + sp
}

/// 🔗️ Plate buckling reduction ρ of the column web, λ̄_p = 0.932 √(b_eff d_wc f_y/(E t_w²)).
pub fn rho_web(b_eff: f64, dwc: f64, fy: f64, e: f64, tw: f64) -> f64 {
    let lp = 0.932 * (b_eff * dwc * fy / (e * tw * tw)).sqrt();
    if lp <= 0.72 { 1.0 } else { (lp - 0.2) / (lp * lp) }
}

/// 🔗️ Transformation parameter ω (Table 6.3): ω1 = 1/√(1 + 1.3 (b_eff t_wc/A_vc)²), ω2 = 1/√(1 + 5.2 (…)²), interpolated in β.
pub fn omega_table_6_3(beta: f64, b_eff: f64, twc: f64, avc: f64) -> f64 {
    let x = (b_eff * twc / avc).powi(2);
    let (w1, w2) = (1.0 / (1.0 + 1.3 * x).sqrt(), 1.0 / (1.0 + 5.2 * x).sqrt());
    if beta <= 0.5 { 1.0 } else if beta < 1.0 { w1 + 2.0 * (1.0 - beta) * (1.0 - w1) } else if beta <= 2.0 { w1 + (beta - 1.0) * (w2 - w1) } else { w2 }
}

/// 🔗️ F_c,wc,Rd = min(ω k_wc ρ b_eff t_wc f_y/γM1, ω k_wc b_eff t_wc f_y/γM0) (§6.2.6.2).
pub fn fc_wc_rd_full(omega: f64, kwc: f64, rho: f64, b_eff: f64, twc: f64, fy: f64, gm0: f64, gm1: f64) -> f64 {
    (omega * kwc * rho * b_eff * twc * fy / gm1).min(omega * kwc * b_eff * twc * fy / gm0)
}

/// 🔗️ F_t,wc,Rd = ω b_eff,t,wc t_wc f_y/γM0.
pub fn ft_wc_rd(omega: f64, b_eff_t: f64, twc: f64, fy: f64, gm0: f64) -> f64 {
    omega * b_eff_t * twc * fy / gm0
}

/// 🔗️ F_c,fb,Rd = M_c,Rd/(h − t_fb) of the beam flange and web in compression (EN 1993-1-8 6.2.6.7; V-426 numbers it 6.2.6.6), `lever` = h − t_fb.
pub fn fc_fb_rd(m_c_rd: f64, lever: f64) -> f64 {
    m_c_rd / lever.max(1.0)
}

/// 🔗️ F_t,wb,Rd = b_eff,t,wb t_wb f_y,wb/γM0 of the beam web in tension (EN 1993-1-8 6.2.6.8), b_eff,t,wb = the effective length of the equivalent end plate T-stub.
pub fn ft_wb_rd(b_eff_t_wb: f64, twb: f64, fy_wb: f64, gm0: f64) -> f64 {
    b_eff_t_wb * twb * fy_wb / gm0
}

/// 🔗️ Column web panel shear including the stiffened contribution V_wp,Rd = 0.9 f_y A_vc/(√3 γM0) + 4 M_pl,fc,Rd/d_s (§6.2.6.1).
pub fn v_wp_rd_stiffened(fy: f64, avc: f64, m_pl_fc_rd: f64, m_pl_stf_rd: f64, ds: f64, gm0: f64, stiffened: bool) -> f64 {
    let base = 0.9 * fy * avc / (3.0_f64.sqrt() * gm0);
    if stiffened { (base + 4.0 * m_pl_fc_rd / ds).min(base + 2.0 * (m_pl_fc_rd + m_pl_stf_rd) / ds) } else { base }
}

/// 🔗️ Distributes bolt row resistances top-down: F_tx = min(F_x,iso, cap − ΣF_prev, min over r < x of group(r, x) − Σ_{k=r..x−1} F_k) (§6.2.7.2(7)).
pub fn distribute_rows(iso: &[f64], group: &dyn Fn(usize, usize) -> f64, cap: f64) -> Vec<f64> {
    let mut f: Vec<f64> = Vec::with_capacity(iso.len());
    for (x, &isox) in iso.iter().enumerate() {
        let mut fx = isox.min(cap - f.iter().sum::<f64>());
        for r in 0..x {
            fx = fx.min(group(r, x) - f[r..x].iter().sum::<f64>());
        }
        f.push(fx.max(0.0));
    }
    f
}

/// 🔗️ M_j,Rd = Σ h_r F_tr,Rd.
pub fn m_j_rd_rows(levers: &[f64], f: &[f64]) -> f64 {
    levers.iter().zip(f).map(|(h, f)| h * f).sum()
}

/// 🔗️ Equivalent lever arm z = Σ k_eff,r h_r²/Σ k_eff,r h_r and k_eq = Σ k_eff,r h_r/z.
pub fn z_eq_and_k_eq(k_eff: &[f64], h: &[f64]) -> (f64, f64) {
    let (num, den): (f64, f64) = k_eff.iter().zip(h).fold((0.0, 0.0), |a, (k, h)| (a.0 + k * h * h, a.1 + k * h));
    let z = num / den;
    (z, den / z)
}

/// 🔗️ Stiffness coefficients of Table 6.11: k1 web shear, k2 web compression, k3 web tension, k4 column flange, k5 end plate, k10 bolts.
pub fn k1_web_shear(avc: f64, beta: f64, z: f64) -> f64 {
    0.38 * avc / (beta * z)
}

pub fn k2_web_compression(b_eff: f64, twc: f64, dc: f64) -> f64 {
    0.7 * b_eff * twc / dc
}

pub fn k3_web_tension(b_eff: f64, twc: f64, dc: f64) -> f64 {
    0.7 * b_eff * twc / dc
}

pub fn k4_column_flange(l_eff: f64, tfc: f64, m: f64) -> f64 {
    0.9 * l_eff * tfc.powi(3) / m.powi(3)
}

pub fn k5_end_plate_bending(l_eff: f64, tp: f64, m: f64) -> f64 {
    0.9 * l_eff * tp.powi(3) / m.powi(3)
}

pub fn k10_bolt_row(a_s: f64, lb: f64, bolts_in_row: f64) -> f64 {
    1.6 * a_s * bolts_in_row / lb
}

/// 🔗️ Effective stiffness of one row k_eff,r = 1/Σ(1/k_i,r).
pub fn k_eff_row(ks: &[f64]) -> f64 {
    1.0 / ks.iter().map(|k| 1.0 / k).sum::<f64>()
}

/// 🔗️ S_j,ini = E z²/Σ(1/k_i) with the shear k1, compression k2 and equivalent tension k_eq.
pub fn s_j_ini_joint(e: f64, z: f64, k1: f64, k2: f64, k_eq: f64) -> f64 {
    e * z * z / (1.0 / k1 + 1.0 / k2 + 1.0 / k_eq)
}

/// 🔗️ Stiffness modification μ = 1 for M ≤ 2/3 M_j,Rd else (1.5 M/M_j,Rd)^ψ.
pub fn mu_stiffness(m_ed: f64, m_j_rd: f64, psi: f64) -> f64 {
    if m_ed <= 2.0 / 3.0 * m_j_rd { 1.0 } else { (1.5 * m_ed / m_j_rd).powf(psi) }
}

/// 🔗️ Secant stiffness S_j = S_j,ini/μ.
pub fn s_j(s_ini: f64, mu: f64) -> f64 {
    s_ini / mu
}

/// 🔗️ Moment-rotation characteristic φ(M) = M/S_j,ini up to 2/3 M_j,Rd and M/S_j above, n samples up to M_j,Rd.
pub fn m_phi_curve(s_ini: f64, m_j_rd: f64, psi: f64, n: usize) -> Vec<(f64, f64)> {
    (0..=n).map(|i| m_j_rd * i as f64 / n as f64).map(|m| (m / s_j(s_ini, mu_stiffness(m, m_j_rd, psi)), m)).collect()
}

/// 🔗️ Stiffness classification with frame type: rigid S ≥ k_b E I_b/L_b (k_b = 8 braced, 25 unbraced if K_b/K_c ≥ 0.1, else semi-rigid), pinned S ≤ 0.5 E I_b/L_b.
pub fn classify_stiffness_frame(s_ini: f64, e: f64, ib: f64, lb: f64, braced: bool, kb_over_kc: f64) -> &'static str {
    let kb = if braced { 8.0 } else { 25.0 };
    if s_ini <= 0.5 * e * ib / lb { "nominally-pinned" } else if s_ini >= kb * e * ib / lb && (braced || kb_over_kc >= 0.1) { "rigid" } else { "semi-rigid" }
}

/// 🔗️ Strength classification (§5.2.3): full if M_j,Rd ≥ M_b,pl,Rd (or 2 M_c,pl,Rd below the top storey), pinned if ≤ 0.25 of that.
pub fn classify_strength_full(m_j_rd: f64, m_b_pl: f64, m_c_pl: f64, top_of_column: bool) -> &'static str {
    let full = if top_of_column { m_b_pl } else { m_b_pl.min(2.0 * m_c_pl) };
    if m_j_rd >= full { "full" } else if m_j_rd <= 0.25 * full { "nominally-pinned" } else { "partial" }
}

/// 🔗️ Base plate moment resistance by plastic equilibrium of the tension side (left, lever `z_l`) and the compression side (right, lever `z_r`) with their tension and compression resistances (Table 6.7 in closed form), N_Ed tension positive, lever arms from the plate centroid, M = T z_l + C z_r with T − C = N_Ed; the same side convention as the anchor statics of the base plate checks; `None` if N_Ed is outside the resistances.
pub fn base_plate_m_rd(n_ed: f64, ft_l: f64, fc_l: f64, ft_r: f64, fc_r: f64, z_l: f64, z_r: f64) -> Option<f64> {
    let lo = (-fc_l).max(n_ed - ft_r);
    let hi = ft_l.min(n_ed + fc_r);
    if lo > hi { None } else { Some(hi * (z_l + z_r) - n_ed * z_r) }
}

/// 🔗️ Base plate concrete stiffness k13 = E_c √(b_eff l_eff)/(1.275 E), plate k15 = 0.85 l_eff t³/m³ and anchor k16 = 1.6 A_b/L_b.
pub fn k13_concrete(ec: f64, b_eff: f64, l_eff: f64, e: f64) -> f64 {
    ec * (b_eff * l_eff).sqrt() / (1.275 * e)
}

pub fn k15_base_plate(l_eff: f64, tp: f64, m: f64) -> f64 {
    0.85 * l_eff * tp.powi(3) / m.powi(3)
}

pub fn k16_anchor(ab: f64, lb: f64) -> f64 {
    1.6 * ab / lb
}

/// 🔗️ Base plate S_j,ini = E z²/(μ Σ 1/k_i).
pub fn s_j_ini_base(e: f64, z: f64, mu: f64, ks: &[f64]) -> f64 {
    e * z * z / (mu * ks.iter().map(|k| 1.0 / k).sum::<f64>())
}

/// 🔗️ f_jd = β_j F_Rdu/(b_eff l_eff) with F_Rdu = A_c0 f_cd √(A_c1/A_c0) ≤ 3 f_cd A_c0 (EN 1992-1-1 §6.7).
pub fn f_jd_bearing(beta_j: f64, a_c0: f64, a_c1: f64, fcd: f64, b_eff: f64, l_eff: f64) -> f64 {
    let f_rdu = (a_c0 * fcd * (a_c1 / a_c0).sqrt()).min(3.0 * fcd * a_c0);
    beta_j * f_rdu / (b_eff * l_eff)
}

/// 🟫 Equivalent compression T-stub under one column flange of a base plate (EN 1993-1-8 6.2.5(3), Figure 6.4; mm, MPa, N): additional bearing width `c`, footprint `b_eff = t_fc + 2c` across and `l_eff = b_fc + 2c` along the flange, both limited by the plate edges, `f_jd` and `F_c,Rd = f_jd b_eff l_eff`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CStrip {
    pub c: f64,
    pub b_eff: f64,
    pub l_eff: f64,
    pub beta_j: f64,
    pub k_j: f64,
    pub f_jd: f64,
    pub fc_rd: f64,
}

/// 🟫 Fixed point of `c = t_p √(f_y/(3 f_jd γ_M0))` with the footprint `min(t_fc + 2c, L_p) × min(b_fc + 2c, B_p)` of the flange T-stub; `spread(b_eff, l_eff)` returns the rigid plate area `A_c0` and the spread area `A_c1` of the foundation (EN 1992-1-1 6.7).
pub fn base_plate_c_strip(tfc: f64, bfc: f64, tp: f64, fy: f64, gm0: f64, lp: f64, bp: f64, beta_j: f64, fcd: f64, spread: &dyn Fn(f64, f64) -> (f64, f64)) -> CStrip {
    let mut strip = CStrip { c: tp, b_eff: 0.0, l_eff: 0.0, beta_j, k_j: 1.0, f_jd: 0.0, fc_rd: 0.0 };
    for _ in 0..8 {
        let (b, l) = ((tfc + 2.0 * strip.c).min(lp), (bfc + 2.0 * strip.c).min(bp));
        let (a0, a1) = spread(b, l);
        let f_jd = f_jd_bearing(beta_j, a0, a1, fcd, b, l);
        strip = CStrip { c: super::parts_2_6::base_plate_c(tp, fy, f_jd, gm0), b_eff: b, l_eff: l, beta_j, k_j: (a1 / a0).sqrt().min(3.0), f_jd, fc_rd: f_jd * b * l };
    }
    strip
}

/// 🟫 Smallest additional bearing width `c` (mm) whose flange T-stub footprint `(t_fc + 2c) min(b_fc + 2c, B_p)` reaches `area` (mm²); `0` where the bare flange already does.
pub fn base_plate_c_required(tfc: f64, bfc: f64, bp: f64, area: f64) -> f64 {
    let free = ((-(tfc + bfc) + ((tfc - bfc).powi(2) + 4.0 * area).sqrt()) / 4.0).max(0.0);
    if bfc + 2.0 * free <= bp { free } else { ((area / bp - tfc) / 2.0).max(0.0) }
}

/// 🔗️ Position of a bolt row in the end plate (Table 6.6).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RowKind {
    Extended,
    FirstBelow,
    Inner,
}

/// 🔗️ Column side of the component method (mm, N, MPa): flange and web geometry, gauge `w`, edge distance `e`, longitudinal web stress and the transformation parameter β.
#[derive(Clone, Copy, Debug)]
pub struct ColumnSide {
    pub tfc: f64,
    pub twc: f64,
    pub dc: f64,
    pub avc: f64,
    pub rc: f64,
    pub fy: f64,
    pub w: f64,
    pub e: f64,
    pub sigma_com: f64,
    pub beta: f64,
    pub tfb: f64,
    pub ab: f64,
}

/// 🔗️ End plate of the component method (mm, MPa) with the beam web thickness `twb` and yield strength `fy_wb` of 6.2.6.8; 0 = not given, the beam web in tension is then not assessed.
#[derive(Clone, Copy, Debug)]
pub struct PlateSide {
    pub tp: f64,
    pub bp: f64,
    pub fy: f64,
    pub twb: f64,
    pub fy_wb: f64,
}

/// 🔗️ One bolt row: lever arm `lever` to the compression centre, T-stub geometry, pitch to the previous row, bolts with their area, grip length and design tension resistance each, and the group flag with the previous row.
#[derive(Clone, Copy, Debug)]
pub struct RowGeom {
    pub kind: RowKind,
    pub lever: f64,
    pub m: f64,
    pub e: f64,
    pub mx: f64,
    pub ex: f64,
    pub alpha: f64,
    pub pitch: f64,
    pub bolts: f64,
    pub d: f64,
    pub a_s: f64,
    pub lb: f64,
    pub ft_rd_bolt: f64,
    pub group: bool,
}

/// 🔗️ The component of a bolt row that governs its tension resistance, with its bilingual label.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RowComponent {
    EndPlateBending,
    ColumnFlangeBending,
    ColumnWebTension,
    BeamWebTension,
}

impl RowComponent {
    pub const fn en(self) -> &'static str {
        match self {
            Self::EndPlateBending => "end plate in bending",
            Self::ColumnFlangeBending => "column flange in bending",
            Self::ColumnWebTension => "column web in tension",
            Self::BeamWebTension => "beam web in tension",
        }
    }

    pub const fn de(self) -> &'static str {
        match self {
            Self::EndPlateBending => "Kopfplatte auf Biegung",
            Self::ColumnFlangeBending => "Stützenflansch auf Biegung",
            Self::ColumnWebTension => "Stützensteg auf Zug",
            Self::BeamWebTension => "Trägersteg auf Zug",
        }
    }
}

/// 🔗️ Tension resistance and stiffness of one row: the column flange, end plate and column web components, the isolated minimum, the governing component and the reduced force of the plastic distribution.
#[derive(Clone, Debug)]
pub struct RowResult {
    pub f_column_flange: f64,
    pub f_end_plate: f64,
    pub f_web_tension: f64,
    pub f_beam_web: f64,
    pub f_iso: f64,
    pub f_rd: f64,
    pub mode_column: u8,
    pub mode_plate: u8,
    pub governing: RowComponent,
    pub k_eff: f64,
}

/// 🔗️ Beam web in tension of a bolt row, F_t,wb,Rd = b_eff,t,wb t_wb f_y,wb/γM0 with b_eff,t,wb the effective length of the equivalent end plate T-stub (EN 1993-1-8 6.2.6.8); infinite while the beam web is not given.
pub fn beam_web_tension(p: &PlateSide, b_eff_t_wb: f64, gm0: f64) -> f64 {
    if p.twb > 0.0 && p.fy_wb > 0.0 { ft_wb_rd(b_eff_t_wb, p.twb, p.fy_wb, gm0) } else { f64::INFINITY }
}

/// 🔗️ Joint result: rows, compression caps, M_j,Rd = Σ h_r F_tr,Rd, equivalent lever arm z and S_j,ini.
#[derive(Clone, Debug)]
pub struct RowsResult {
    pub rows: Vec<RowResult>,
    pub f_c_wc_rd: f64,
    pub v_wp_rd: f64,
    pub f_beam: f64,
    pub cap: f64,
    pub m_j_rd: f64,
    pub s_ini: f64,
    pub z: f64,
    pub k_eq: f64,
}

fn plate_leff(c: &ColumnSide, p: &PlateSide, r: &RowGeom) -> LEff {
    match r.kind {
        RowKind::Extended => l_eff_end_plate_extended(r.mx, r.ex, r.e, c.w, p.bp),
        RowKind::FirstBelow => l_eff_end_plate_first_below(r.m, r.alpha),
        RowKind::Inner => l_eff_end_plate_inner(r.m, r.alpha),
    }
}

fn prying_edge(r: &RowGeom) -> f64 {
    if r.ex > 0.0 { r.e.min(r.ex) } else { r.e }
}

fn lever_m(r: &RowGeom) -> f64 {
    if r.kind == RowKind::Extended { r.mx } else { r.m }
}

fn group_lengths(c: &ColumnSide, rows: &[RowGeom], first: usize, last: usize) -> (LEff, LEff) {
    let m_c = m_rolled(c.w, c.twc, c.rc);
    let (mut column, mut plate) = (LEff { cp: 0.0, nc: 0.0 }, LEff { cp: 0.0, nc: 0.0 });
    for index in first..=last {
        let r = &rows[index];
        let pitch = if index == first { rows[(index + 1).min(last)].pitch } else { r.pitch };
        let (pc, pp) = if index == first || index == last {
            let plate_row = if index == first && r.kind == RowKind::FirstBelow { l_eff_end_plate_first_below_group(r.m, r.e, pitch, r.alpha) } else { l_eff_column_flange_end_group(lever_m(r), r.e, pitch) };
            (l_eff_column_flange_end_group(m_c, c.e, pitch), plate_row)
        } else {
            (l_eff_column_flange_inner_group(r.pitch), l_eff_end_plate_inner(r.m, r.alpha))
        };
        column.cp += pc.cp;
        column.nc += pc.nc;
        plate.cp += pp.cp;
        plate.nc += pp.nc;
    }
    (column, plate)
}

/// 🔗️ Component method of an end plate joint (EN 1993-1-8 6.2.6, 6.2.7, 6.3): per row the column flange and end plate T-stubs and the column web in tension, the group effect of consecutive grouped rows, the plastic distribution of the row forces below the compression caps F_c,wc,Rd, V_wp,Rd/β and the beam resistance, then M_j,Rd and S_j,ini.
pub fn component_rows(c: &ColumnSide, p: &PlateSide, rows: &[RowGeom], f_beam: f64, e_modulus: f64, gm0: f64, gm1: f64) -> RowsResult {
    let m_c = m_rolled(c.w, c.twc, c.rc);
    let lc = l_eff_column_flange_single(m_c, c.e);
    let b_eff_c = b_eff_c_wc(c.tfb, c.ab, c.tfc, c.rc, 0.0);
    let fc_wc = fc_wc_rd_full(omega_table_6_3(c.beta, b_eff_c, c.twc, c.avc), k_wc(c.sigma_com, c.fy), rho_web(b_eff_c, c.dc, c.fy, e_modulus, c.twc), b_eff_c, c.twc, c.fy, gm0, gm1);
    let v_wp = v_wp_rd_stiffened(c.fy, c.avc, 0.0, 0.0, 1.0, gm0, false) / c.beta;
    let mut out = Vec::with_capacity(rows.len());
    let mut levers = Vec::new();
    let mut k_rows = Vec::new();
    for r in rows {
        let ew = 0.25 * 1.6 * r.d;
        let sum_ft = r.bolts * r.ft_rd_bolt;
        let col = t_stub_modes(lc.l1(), lc.l2(), c.tfc, c.fy, m_c, n_prying(c.e, m_c), ew, sum_ft, r.a_s, r.bolts, r.lb, Mode1Method::Two, gm0);
        let b_eff_t = if col.mode == 1 { lc.l1() } else { lc.l2() };
        let f_web = ft_wc_rd(omega_table_6_3(c.beta, b_eff_t, c.twc, c.avc), b_eff_t, c.twc, c.fy, gm0);
        let leff = plate_leff(c, p, r);
        let m_use = lever_m(r);
        let stub = t_stub_modes(leff.l1(), leff.l2(), p.tp, p.fy, m_use, n_prying(prying_edge(r), m_use), ew, sum_ft, r.a_s, r.bolts, r.lb, Mode1Method::Two, gm0);
        let f_beam_web = beam_web_tension(p, if stub.mode == 1 { leff.l1() } else { leff.l2() }, gm0);
        let f_iso = col.ft_rd.min(stub.ft_rd).min(f_web).min(f_beam_web);
        let governing = if f_iso == stub.ft_rd { RowComponent::EndPlateBending } else if f_iso == col.ft_rd { RowComponent::ColumnFlangeBending } else if f_iso == f_beam_web { RowComponent::BeamWebTension } else { RowComponent::ColumnWebTension };
        let ks = [k3_web_tension(b_eff_t, c.twc, c.dc), k4_column_flange(lc.l1(), c.tfc, m_c), k5_end_plate_bending(leff.l1(), p.tp, m_use), k10_bolt_row(r.a_s, r.lb, r.bolts)];
        k_rows.push(k_eff_row(&ks));
        levers.push(r.lever);
        out.push(RowResult { f_column_flange: col.ft_rd, f_end_plate: stub.ft_rd, f_web_tension: f_web, f_beam_web, f_iso, f_rd: f_iso, mode_column: col.mode, mode_plate: stub.mode, governing, k_eff: *k_rows.last().unwrap() });
    }
    let iso: Vec<f64> = out.iter().map(|r| r.f_iso).collect();
    let group = |first: usize, last: usize| {
        if (first + 1..=last).any(|i| !rows[i].group) {
            return f64::INFINITY;
        }
        let (lcg, lpg) = group_lengths(c, rows, first, last);
        let (bolts, a_s, lb) = (rows[first..=last].iter().map(|r| r.bolts).sum::<f64>(), rows[first].a_s, rows[first].lb);
        let sum_ft: f64 = rows[first..=last].iter().map(|r| r.bolts * r.ft_rd_bolt).sum();
        let ew = 0.25 * 1.6 * rows[first].d;
        let col = t_stub_modes(lcg.l1(), lcg.l2(), c.tfc, c.fy, m_c, n_prying(c.e, m_c), ew, sum_ft, a_s, bolts, lb, Mode1Method::Two, gm0);
        let m_use = lever_m(&rows[first]);
        let plate = t_stub_modes(lpg.l1(), lpg.l2(), p.tp, p.fy, m_use, n_prying(prying_edge(&rows[first]), m_use), ew, sum_ft, a_s, bolts, lb, Mode1Method::Two, gm0);
        let b_eff_t = if col.mode == 1 { lcg.l1() } else { lcg.l2() };
        col.ft_rd.min(plate.ft_rd).min(ft_wc_rd(omega_table_6_3(c.beta, b_eff_t, c.twc, c.avc), b_eff_t, c.twc, c.fy, gm0)).min(beam_web_tension(p, if plate.mode == 1 { lpg.l1() } else { lpg.l2() }, gm0))
    };
    let cap = fc_wc.min(v_wp).min(f_beam);
    let forces = distribute_rows(&iso, &group, cap);
    for (row, force) in out.iter_mut().zip(&forces) {
        row.f_rd = *force;
    }
    let (z, k_eq) = z_eq_and_k_eq(&k_rows, &levers);
    let s_ini = s_j_ini_joint(e_modulus, z, k1_web_shear(c.avc, c.beta, z), k2_web_compression(b_eff_c, c.twc, c.dc), k_eq);
    RowsResult { rows: out, f_c_wc_rd: fc_wc, v_wp_rd: v_wp, f_beam, cap, m_j_rd: m_j_rd_rows(&levers, &forces), s_ini, z, k_eq }
}
