//! 🧱️ EN 1993-1-1 §6.3 member stability: buckling curves, LTB, Mcr, interaction Annex A/B, general method (mm, N, MPa, Nmm). See [EN 1993-1-1 §6.3](https://eurocodes.jrc.ec.europa.eu/).

use std::f64::consts::PI;

/// 🧱️ Modulus of elasticity E [MPa] from `ndp.en1993.1-1.e-modulus`.
pub fn e_steel() -> f64 {
    semio_s_plugin_norm_compliance::ndp_common("ndp.en1993.1-1.e-modulus")
}

/// 🧱️ Shear modulus G [MPa] from `ndp.en1993.1-1.g-modulus`.
pub fn g_steel() -> f64 {
    semio_s_plugin_norm_compliance::ndp_common("ndp.en1993.1-1.g-modulus")
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Curve {
    A0,
    A,
    B,
    C,
    D,
}

impl Curve {
    pub fn alpha(self) -> f64 {
        match self {
            Curve::A0 => 0.13,
            Curve::A => 0.21,
            Curve::B => 0.34,
            Curve::C => 0.49,
            Curve::D => 0.76,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BucklingSectionKind {
    RolledI,
    WeldedI,
    HotHollow,
    ColdHollow,
    WeldedBox { thick_welds: bool },
    Channel,
    Tee,
    Solid,
    Angle,
}

/// 📉 Buckling curve of Table 6.2 for the axis (`y` = true for y-y).
pub fn curve_table_6_2(kind: BucklingSectionKind, y_axis: bool, h: f64, b: f64, tf: f64, fy: f64) -> Curve {
    let s460 = fy >= 460.0;
    match kind {
        BucklingSectionKind::RolledI => {
            if h / b > 1.2 {
                if tf <= 40.0 {
                    if s460 { Curve::A0 } else if y_axis { Curve::A } else { Curve::B }
                } else if s460 {
                    Curve::A
                } else if y_axis {
                    Curve::B
                } else {
                    Curve::C
                }
            } else if tf <= 100.0 {
                if s460 { Curve::A } else if y_axis { Curve::B } else { Curve::C }
            } else if s460 {
                Curve::C
            } else {
                Curve::D
            }
        }
        BucklingSectionKind::WeldedI => {
            if tf <= 40.0 {
                if y_axis { Curve::B } else { Curve::C }
            } else if y_axis {
                Curve::C
            } else {
                Curve::D
            }
        }
        BucklingSectionKind::HotHollow => if s460 { Curve::A0 } else { Curve::A },
        BucklingSectionKind::ColdHollow => Curve::C,
        BucklingSectionKind::WeldedBox { thick_welds } => if thick_welds { Curve::C } else { Curve::B },
        BucklingSectionKind::Channel | BucklingSectionKind::Tee | BucklingSectionKind::Solid => Curve::C,
        BucklingSectionKind::Angle => Curve::B,
    }
}

pub fn n_cr(ei: f64, l_cr: f64) -> f64 {
    PI * PI * ei / (l_cr * l_cr)
}

pub fn lambda_bar_from_ncr(a_fy: f64, ncr: f64) -> f64 {
    (a_fy / ncr).sqrt()
}

pub fn lambda_1(fy: f64) -> f64 {
    PI * (e_steel() / fy).sqrt()
}

pub fn chi_general(lambda: f64, alpha: f64, lambda_0: f64, beta: f64) -> f64 {
    let phi = 0.5 * (1.0 + alpha * (lambda - lambda_0) + beta * lambda * lambda);
    (1.0 / (phi + (phi * phi - beta * lambda * lambda).max(0.0).sqrt())).min(1.0)
}

/// 📉 χ of Eq. 6.49 (no reduction for λ̄ ≤ 0.2 or N_Ed/N_cr ≤ 0.04).
pub fn chi_flexural(lambda: f64, curve: Curve, n_ed: f64, ncr: f64) -> f64 {
    if lambda <= 0.2 || n_ed / ncr <= 0.04 { 1.0 } else { chi_general(lambda, curve.alpha(), 0.2, 1.0) }
}

/// 🌀 N_cr,T for a section with polar radius i0 (Eq. 6.51 basis).
pub fn n_cr_torsional(g: f64, it: f64, e: f64, iw: f64, l_t: f64, i0_sq: f64) -> f64 {
    (g * it + PI * PI * e * iw / (l_t * l_t)) / i0_sq
}

/// 🌀 N_cr,TF for a section symmetric about the y-y axis with shear-centre offset y0.
pub fn n_cr_torsional_flexural(ncr_y: f64, ncr_t: f64, y0: f64, i0_sq: f64) -> f64 {
    let beta = 1.0 - y0 * y0 / i0_sq;
    let r = ncr_t / ncr_y;
    ncr_y / (2.0 * beta) * (1.0 + r - ((1.0 - r).powi(2) + 4.0 * (y0 * y0 / i0_sq) * r).sqrt())
}

/// 🔁 Effective-length factor of a frame column from end stiffness ratios (Jackson–Moreland nomogram equations).
pub fn k_nomogram(ga: f64, gb: f64, sway: bool) -> f64 {
    let f = |k: f64| -> f64 {
        let x = PI / k;
        if sway {
            (ga * gb * x * x - 36.0) / (6.0 * (ga + gb)) - x / x.tan()
        } else {
            (ga * gb / 4.0) * x * x + ((ga + gb) / 2.0) * (1.0 - x / x.tan()) + 2.0 * (x / 2.0).tan() / x - 1.0
        }
    };
    let (start, end) = if sway { (1.0001, 40.0) } else { (0.5001, 1.0) };
    let step = 0.0005;
    let mut k = start;
    let mut prev = f(k);
    while k < end {
        let next = k + step;
        let cur = f(next);
        if prev.is_finite() && cur.is_finite() && prev * cur <= 0.0 && prev.abs() < 50.0 && cur.abs() < 50.0 {
            let (mut lo, mut hi) = (k, next);
            for _ in 0..80 {
                let mid = 0.5 * (lo + hi);
                if (f(mid) > 0.0) == (f(lo) > 0.0) { lo = mid } else { hi = mid }
            }
            return 0.5 * (lo + hi);
        }
        k = next;
        prev = cur;
    }
    f64::NAN
}

pub fn ltb_alpha_table_6_4(rolled: bool, welded: bool, h_over_b: f64) -> f64 {
    let c = if rolled {
        if h_over_b <= 2.0 { Curve::A } else { Curve::B }
    } else if welded {
        if h_over_b <= 2.0 { Curve::C } else { Curve::D }
    } else {
        Curve::D
    };
    c.alpha()
}

pub fn ltb_curve_table_6_5(rolled: bool, h_over_b: f64) -> Curve {
    match (rolled, h_over_b <= 2.0) {
        (true, true) => Curve::B,
        (true, false) => Curve::C,
        (false, true) => Curve::C,
        (false, false) => Curve::D,
    }
}

/// 📉 χ_LT of Eq. 6.56 (§6.3.2.2) with the negligible-LTB shortcut.
pub fn chi_lt_general(lambda_lt: f64, alpha_lt: f64, m_ed: f64, mcr: f64) -> f64 {
    if lambda_lt <= 0.2 || m_ed / mcr <= 0.04 { 1.0 } else { chi_general(lambda_lt, alpha_lt, 0.2, 1.0) }
}

/// 📉 χ_LT of Eq. 6.57 (§6.3.2.3), capped by 1 and 1/λ̄².
pub fn chi_lt_rolled(lambda_lt: f64, alpha_lt: f64, lambda_0: f64, beta: f64, m_ed: f64, mcr: f64) -> f64 {
    if lambda_lt <= lambda_0 || m_ed / mcr <= lambda_0 * lambda_0 {
        return 1.0;
    }
    chi_general(lambda_lt, alpha_lt, lambda_0, beta).min(1.0 / (lambda_lt * lambda_lt))
}

/// 📉 k_c of Table 6.6 for end moments ψ.
pub fn kc_psi(psi: f64) -> f64 {
    1.0 / (1.33 - 0.33 * psi)
}

pub fn kc_udl_fork() -> f64 {
    0.94
}

pub fn kc_point_load_fork() -> f64 {
    0.86
}

/// 📉 Modification factor f of Eq. 6.58.
pub fn ltb_f(kc: f64, lambda_lt: f64) -> f64 {
    (1.0 - 0.5 * (1.0 - kc) * (1.0 - 2.0 * (lambda_lt - 0.8).powi(2))).min(1.0)
}

pub fn chi_lt_mod(chi_lt: f64, f: f64) -> f64 {
    (chi_lt / f).min(1.0)
}

/// 🪶️ C1 for end moments ψ, ENV 1993-1-1 Annex F table with linear interpolation.
pub fn c1_end_moments(psi: f64) -> f64 {
    const T: [(f64, f64); 9] = [(-1.0, 2.752), (-0.75, 2.927), (-0.5, 2.704), (-0.25, 2.281), (0.0, 1.879), (0.25, 1.563), (0.5, 1.323), (0.75, 1.141), (1.0, 1.0)];
    let p = psi.clamp(-1.0, 1.0);
    for w in T.windows(2) {
        if p >= w[0].0 && p <= w[1].0 {
            let t = (p - w[0].0) / (w[1].0 - w[0].0);
            return w[0].1 + t * (w[1].1 - w[0].1);
        }
    }
    1.0
}

/// 🪶️ Mcr of a doubly symmetric I (ECCS closed form); `zg` positive above the shear centre for gravity loads.
pub fn mcr_closed_form(e: f64, g: f64, iz: f64, it: f64, iw: f64, l: f64, kz: f64, kw: f64, c1: f64, c2: f64, zg: f64) -> f64 {
    let lz = kz * l;
    let a = PI * PI * e * iz / (lz * lz);
    let root = ((kz / kw).powi(2) * iw / iz + lz * lz * g * it / (PI * PI * e * iz) + (c2 * zg).powi(2)).sqrt();
    c1 * a * (root - c2 * zg)
}

pub fn lambda_lt_from_mcr(w_y: f64, fy: f64, mcr: f64) -> f64 {
    (w_y * fy / mcr).sqrt()
}

/// 🪶️ §6.3.2.4 simplified check: returns the utilisation k_c L_c/(i_fz λ1) over λ_c0 M_c,Rd/M_y,Ed.
pub fn simplified_compression_flange_utilisation(kc: f64, lc: f64, i_fz: f64, fy: f64, lambda_c0: f64, mc_rd: f64, my_ed: f64) -> f64 {
    let lambda_f = kc * lc / (i_fz * lambda_1(fy));
    lambda_f / (lambda_c0 * mc_rd / my_ed)
}

pub fn m_b_rd_compression_flange(kfl: f64, chi: f64, mc_rd: f64) -> f64 {
    (kfl * chi * mc_rd).min(mc_rd)
}

pub fn cm_linear(psi: f64) -> f64 {
    (0.6 + 0.4 * psi).max(0.4)
}

/// 🔗️ C_m of Table B.3 for a hogging-dominated diagram with transverse load, α_s = Ms/Mh.
pub fn cm_alpha_s(alpha_s: f64) -> f64 {
    (if alpha_s >= 0.0 { 0.2 + 0.8 * alpha_s } else { 0.1 - 0.8 * alpha_s }).max(0.4)
}

/// 🔗️ C_m of Table B.3 for a sagging-dominated diagram, α_h = Mh/Ms, load type `udl` or point load.
pub fn cm_alpha_h(alpha_h: f64, psi: f64, udl: bool) -> f64 {
    let (a, b) = if udl { (0.95, 0.05) } else { (0.90, 0.10) };
    if alpha_h >= 0.0 { a + b * alpha_h } else { a + b * alpha_h * (1.0 + 2.0 * psi) }
}

#[derive(Clone, Copy)]
pub struct InteractionInput {
    pub n_ed: f64,
    pub my_ed: f64,
    pub mz_ed: f64,
    pub n_rk: f64,
    pub my_rk: f64,
    pub mz_rk: f64,
    pub chi_y: f64,
    pub chi_z: f64,
    pub chi_lt: f64,
    pub lambda_y: f64,
    pub lambda_z: f64,
    pub cmy: f64,
    pub cmz: f64,
    pub cm_lt: f64,
    pub class: u8,
    pub susceptible_torsion: bool,
    pub rhs: bool,
    pub gm1: f64,
    pub delta_my: f64,
    pub delta_mz: f64,
}

pub struct Interaction {
    pub kyy: f64,
    pub kyz: f64,
    pub kzy: f64,
    pub kzz: f64,
    pub eta_61: f64,
    pub eta_62: f64,
}

fn finish(i: &InteractionInput, kyy: f64, kyz: f64, kzy: f64, kzz: f64) -> Interaction {
    let g = i.gm1;
    let n_y = i.n_ed / (i.chi_y * i.n_rk / g);
    let n_z = i.n_ed / (i.chi_z * i.n_rk / g);
    let my = (i.my_ed + i.delta_my) / (i.chi_lt * i.my_rk / g);
    let mz = (i.mz_ed + i.delta_mz) / (i.mz_rk / g);
    Interaction { kyy, kyz, kzy, kzz, eta_61: n_y + kyy * my + kyz * mz, eta_62: n_z + kzy * my + kzz * mz }
}

/// 🔗️ Annex B (method 2), Tables B.1/B.2.
pub fn interaction_annex_b(i: &InteractionInput) -> Interaction {
    let g = i.gm1;
    let ny = i.n_ed / (i.chi_y * i.n_rk / g);
    let nz = i.n_ed / (i.chi_z * i.n_rk / g);
    let c12 = i.class <= 2;
    let kyy = if c12 {
        (i.cmy * (1.0 + (i.lambda_y - 0.2) * ny)).min(i.cmy * (1.0 + 0.8 * ny))
    } else {
        (i.cmy * (1.0 + 0.6 * i.lambda_y * ny)).min(i.cmy * (1.0 + 0.6 * ny))
    };
    let kzz = if i.rhs {
        if c12 { (i.cmz * (1.0 + (i.lambda_z - 0.2) * nz)).min(i.cmz * (1.0 + 0.8 * nz)) } else { (i.cmz * (1.0 + 0.6 * i.lambda_z * nz)).min(i.cmz * (1.0 + 0.6 * nz)) }
    } else if c12 {
        (i.cmz * (1.0 + (2.0 * i.lambda_z - 0.6) * nz)).min(i.cmz * (1.0 + 1.4 * nz))
    } else {
        (i.cmz * (1.0 + 0.6 * i.lambda_z * nz)).min(i.cmz * (1.0 + 0.6 * nz))
    };
    let kyz = if c12 { 0.6 * kzz } else { kzz };
    let kzy = if !i.susceptible_torsion {
        if c12 { 0.6 * kyy } else { 0.8 * kyy }
    } else if c12 {
        let d = (i.cm_lt - 0.25).max(1e-6);
        if i.lambda_z >= 0.4 {
            (1.0 - 0.1 * i.lambda_z / d * nz).max(1.0 - 0.1 / d * nz)
        } else {
            (0.6 + i.lambda_z).min(1.0 - 0.1 * i.lambda_z / d * nz)
        }
    } else {
        let d = (i.cm_lt - 0.25).max(1e-6);
        (1.0 - 0.05 * i.lambda_z / d * nz).max(1.0 - 0.05 / d * nz)
    };
    finish(i, kyy, kyz, kzy, kzz)
}

#[derive(Clone, Copy)]
pub struct AnnexAInput {
    pub base: InteractionInput,
    pub ncr_y: f64,
    pub ncr_z: f64,
    pub ncr_t: f64,
    pub ncr_tf: f64,
    pub wel_y: f64,
    pub wpl_y: f64,
    pub wel_z: f64,
    pub wpl_z: f64,
    pub mpl_y_rd: f64,
    pub mpl_z_rd: f64,
    pub it: f64,
    pub iy: f64,
    pub a_area: f64,
    pub c1: f64,
    pub lambda_0: f64,
    pub cmy0: f64,
    pub cmz0: f64,
}

/// 🔗️ Annex A (method 1), Tables A.1/A.2 for class 1 and 2; class 3 and 4 use w = 1.
pub fn interaction_annex_a(a: &AnnexAInput) -> Interaction {
    let i = &a.base;
    let n = i.n_ed;
    let c12 = i.class <= 2;
    let (wy, wz) = if c12 { ((a.wpl_y / a.wel_y).min(1.5), (a.wpl_z / a.wel_z).min(1.5)) } else { (1.0, 1.0) };
    let g = i.gm1;
    let npl = n / (i.n_rk / g);
    let a_lt = (1.0 - a.it / a.iy).max(0.0);
    let eps_y = (i.my_ed / n) * (a.a_area / a.wel_y);
    let lmax = i.lambda_y.max(i.lambda_z);
    let mu_y = (1.0 - n / a.ncr_y) / (1.0 - i.chi_y * n / a.ncr_y);
    let mu_z = (1.0 - n / a.ncr_z) / (1.0 - i.chi_z * n / a.ncr_z);
    let limit = 0.2 * a.c1.sqrt() * ((1.0 - n / a.ncr_z) * (1.0 - n / a.ncr_tf)).powf(0.25);
    let simple = a.lambda_0 <= limit;
    let (cmy, cmz, cm_lt) = if simple {
        (a.cmy0, a.cmz0, 1.0)
    } else {
        let s = eps_y.abs().sqrt() * a_lt;
        let cmy = a.cmy0 + (1.0 - a.cmy0) * s / (1.0 + s);
        let cmlt = (cmy * cmy * a_lt / ((1.0 - n / a.ncr_z) * (1.0 - n / a.ncr_t)).sqrt()).max(1.0);
        (cmy, a.cmz0, cmlt)
    };
    let (blt, clt, dlt, elt) = if simple {
        (0.0, 0.0, 0.0, 0.0)
    } else {
        let m_y = i.my_ed / (i.chi_lt * a.mpl_y_rd);
        let m_z = i.mz_ed / a.mpl_z_rd;
        (
            0.5 * a_lt * a.lambda_0.powi(2) * m_y * m_z,
            10.0 * a_lt * a.lambda_0.powi(2) / (5.0 + i.lambda_z.powi(4)) * (i.my_ed / (cmy * i.chi_lt * a.mpl_y_rd)),
            2.0 * a_lt * a.lambda_0 / (0.1 + i.lambda_z.powi(4)) * (i.my_ed / (cmy * i.chi_lt * a.mpl_y_rd)) * (i.mz_ed / (cmz * a.mpl_z_rd)),
            1.7 * a_lt * a.lambda_0 / (0.1 + i.lambda_z.powi(4)) * (i.my_ed / (cmy * i.chi_lt * a.mpl_y_rd)),
        )
    };
    let (cyy, cyz, czy, czz) = if c12 {
        (
            (1.0 + (wy - 1.0) * ((2.0 - 1.6 / wy * cmy * cmy * lmax - 1.6 / wy * cmy * cmy * lmax * lmax) * npl - blt)).max(a.wel_y / a.wpl_y),
            (1.0 + (wz - 1.0) * ((2.0 - 14.0 * cmz * cmz * lmax * lmax / wz.powi(5)) * npl - clt)).max(0.6 * (wz / wy).sqrt() * a.wel_z / a.wpl_z),
            (1.0 + (wy - 1.0) * ((2.0 - 14.0 * cmy * cmy * lmax * lmax / wy.powi(5)) * npl - dlt)).max(0.6 * (wy / wz).sqrt() * a.wel_y / a.wpl_y),
            (1.0 + (wz - 1.0) * ((2.0 - 1.6 / wz * cmz * cmz * lmax - 1.6 / wz * cmz * cmz * lmax * lmax) * npl - elt)).max(a.wel_z / a.wpl_z),
        )
    } else {
        (1.0, 1.0, 1.0, 1.0)
    };
    let kyy = cmy * cm_lt * mu_y / (1.0 - n / a.ncr_y) / cyy;
    let kyz = cmz * mu_y / (1.0 - n / a.ncr_z) / cyz * 0.6 * (wz / wy).sqrt();
    let kzy = cmy * cm_lt * mu_z / (1.0 - n / a.ncr_y) / czy * 0.6 * (wy / wz).sqrt();
    let kzz = cmz * mu_z / (1.0 - n / a.ncr_z) / czz;
    finish(i, kyy, kyz, kzy, kzz)
}

/// 🧭️ C_mi,0 from the first-order deflection δ_x and moment M_Ed(x).
pub fn cm0(e: f64, ii: f64, delta_x: f64, l: f64, m_ed_x: f64, n_ed: f64, ncr_i: f64) -> f64 {
    1.0 + (PI * PI * e * ii * delta_x / (l * l * m_ed_x.abs()) - 1.0) * n_ed / ncr_i
}

pub struct GeneralMethod {
    pub lambda_op: f64,
    pub chi_op: f64,
    pub utilisation: f64,
}

/// 🧭️ §6.3.4 general method: chi_op from the lower of χ and χ_LT curves, utilisation α_ult,k χ_op/γM1 ≥ 1 inverted.
pub fn general_method_6_3_4(alpha_ult_k: f64, alpha_cr_op: f64, alpha: f64, gm1: f64) -> GeneralMethod {
    let lambda_op = (alpha_ult_k / alpha_cr_op).sqrt();
    let chi_op = if lambda_op <= 0.2 { 1.0 } else { chi_general(lambda_op, alpha, 0.2, 1.0) };
    GeneralMethod { lambda_op, chi_op, utilisation: gm1 / (chi_op * alpha_ult_k) }
}

/// 🧭️ §6.3.4 α_ult,k from the linear interaction of the minimum cross-section resistance.
pub fn alpha_ult_k_linear(n_ed: f64, n_rk: f64, my_ed: f64, my_rk: f64, mz_ed: f64, mz_rk: f64) -> f64 {
    1.0 / (n_ed.abs() / n_rk + my_ed.abs() / my_rk + mz_ed.abs() / mz_rk)
}

/// 🧭️ §6.3.5.3 stable length L_m between a plastic hinge and the next restraint (Eq. 6.67).
pub fn plastic_hinge_stable_length(iz: f64, n_ed: f64, a: f64, c1: f64, wpl_y: f64, it: f64, fy: f64) -> f64 {
    38.0 * iz / (((1.0 / 57.4) * (n_ed / a) / 1.0 + (1.0 / 756.0) / (c1 * c1) * wpl_y * wpl_y / (a * it) * (fy / 235.0).powi(2)).sqrt())
}
