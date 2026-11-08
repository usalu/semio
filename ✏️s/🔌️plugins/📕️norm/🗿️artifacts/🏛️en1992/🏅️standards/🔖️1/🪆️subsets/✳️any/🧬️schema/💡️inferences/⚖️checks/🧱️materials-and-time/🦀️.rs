//! 🧱️ Materials, durability, creep and shrinkage of EN 1992-1-1 chapters 3 and 4 and Annex B as catalogue entries M-01 … M-14, D-01 … D-07, A-01 and A-02. See [`super`].

use super::*;

/// 🧱 Concrete property row of Table 3.1 computed from f_ck (MPa); strains in permille.
#[derive(Clone, Copy, Debug)]
pub struct Conc {
    pub fck: f64,
    pub fcm: f64,
    pub fctm: f64,
    pub fctk05: f64,
    pub fctk95: f64,
    pub ecm: f64,
    pub eps_c1: f64,
    pub eps_cu1: f64,
    pub eps_c2: f64,
    pub eps_cu2: f64,
    pub n: f64,
    pub eps_c3: f64,
    pub eps_cu3: f64,
}

/// 📚 Table 3.1 E_cm [MPa] of a standard grade from the shared registry rows `ndp.en1992.t3-1.ecm.c<fck>-<fcu>` (unverified, one source with the EN 1997 pile modulus).
fn ecm_registry_mpa(fck: f64) -> Option<f64> {
    semio_s_plugin_norm_compliance::ndp_number_opt(&format!("ndp.en1992.t3-1.ecm.c{}-{}", fck as u32, f_ck_cube(fck) as u32), AnnexChoice::En).map(|gpa| gpa * 1000.0)
}

/// 📈 Table 3.1 properties for any f_ck up to 90 MPa; standard grades use the tabulated f_ctm, f_ctk and E_cm, other grades the formulas.
pub fn conc(fck: f64) -> Conc {
    const T: [(f64, f64, f64, f64); 14] = [
        (12.0, 1.6, 1.1, 2.0), (16.0, 1.9, 1.3, 2.5), (20.0, 2.2, 1.5, 2.9), (25.0, 2.6, 1.8, 3.3), (30.0, 2.9, 2.0, 3.8),
        (35.0, 3.2, 2.2, 4.2), (40.0, 3.5, 2.5, 4.6), (45.0, 3.8, 2.7, 4.9), (50.0, 4.1, 2.9, 5.3), (55.0, 4.2, 3.0, 5.5),
        (60.0, 4.4, 3.1, 5.7), (70.0, 4.6, 3.2, 6.0), (80.0, 4.8, 3.4, 6.3), (90.0, 5.0, 3.5, 6.6),
    ];
    let fcm = fck + 8.0;
    let tab = T.iter().find(|r| (r.0 - fck).abs() < 1e-9);
    let fctm = if fck <= 50.0 { 0.30 * fck.powf(2.0 / 3.0) } else { 2.12 * (1.0 + fcm / 10.0).ln() };
    let hi = fck > 50.0;
    let q = ((90.0 - fck) / 100.0).max(0.0).powi(4);
    let eps_cu2 = if hi { 2.6 + 35.0 * q } else { 3.5 };
    Conc {
        fck,
        fcm,
        fctm: tab.map(|r| r.1).unwrap_or(fctm),
        fctk05: tab.map(|r| r.2).unwrap_or(0.7 * fctm),
        fctk95: tab.map(|r| r.3).unwrap_or(1.3 * fctm),
        ecm: tab.and_then(|r| ecm_registry_mpa(r.0)).unwrap_or(22000.0 * (fcm / 10.0).powf(0.3)),
        eps_c1: (0.7 * fcm.powf(0.31)).min(2.8),
        eps_cu1: if hi { 2.8 + 27.0 * ((98.0 - fcm) / 100.0).powi(4) } else { 3.5 },
        eps_c2: if hi { 2.0 + 0.085 * (fck - 50.0).powf(0.53) } else { 2.0 },
        eps_cu2,
        n: if hi { 1.4 + 23.4 * q } else { 2.0 },
        eps_c3: if hi { 1.75 + 0.55 * (fck - 50.0) / 40.0 } else { 1.75 },
        eps_cu3: eps_cu2,
    }
}

/// 💪 Design compressive strength f_cd = α_cc f_ck / γ_c of the persistent or accidental situation.
pub fn f_cd(annex: AnnexChoice, fck: f64, accidental: bool) -> f64 {
    let (a, g) = if accidental { ("ndp.en1992.m-01.alpha-cc-accidental", "ndp.en1992.m-01.gamma-c-accidental") } else { ("ndp.en1992.m-01.alpha-cc-persistent", "ndp.en1992.m-01.gamma-c-persistent") };
    ndp(annex, a) * fck / ndp(annex, g)
}

/// 🔩 Design tensile strength f_ctd = α_ct f_ctk,0.05 / γ_c.
pub fn f_ctd(annex: AnnexChoice, fck: f64) -> f64 {
    ndp(annex, "ndp.en1992.m-01.alpha-ct") * conc(fck).fctk05 / ndp(annex, "ndp.en1992.m-01.gamma-c-persistent")
}

/// 🔩 Design yield strength of reinforcement.
pub fn f_yd(annex: AnnexChoice, fyk: f64, accidental: bool) -> f64 {
    fyk / ndp(annex, if accidental { "ndp.en1992.gamma-s-acc" } else { "ndp.en1992.gamma-s" })
}

/// 🧊 Cube strength of the standard grade (Table 3.1), interpolated for non-tabulated f_ck.
pub fn f_ck_cube(fck: f64) -> f64 {
    const T: [(f64, f64); 14] = [(12.0, 15.0), (16.0, 20.0), (20.0, 25.0), (25.0, 30.0), (30.0, 37.0), (35.0, 45.0), (40.0, 50.0), (45.0, 55.0), (50.0, 60.0), (55.0, 67.0), (60.0, 75.0), (70.0, 85.0), (80.0, 95.0), (90.0, 105.0)];
    interp(&T, fck)
}

/// 📏 Piecewise linear interpolation with end clamping.
pub fn interp(t: &[(f64, f64)], x: f64) -> f64 {
    if x <= t[0].0 {
        return t[0].1;
    }
    for w in t.windows(2) {
        if x <= w[1].0 {
            return w[0].1 + (w[1].1 - w[0].1) * (x - w[0].0) / (w[1].0 - w[0].0);
        }
    }
    t[t.len() - 1].1
}

/// 📈 Parabola-rectangle stress σ_c(ε) for ε in permille (Fig. 3.3), compression positive.
pub fn sigma_c_pr(c: &Conc, fcd: f64, eps: f64) -> f64 {
    if eps <= 0.0 || eps > c.eps_cu2 + 1e-12 {
        0.0
    } else {
        semio_s_plugin_norm_mechanics::section::fibre::parabola_rectangle(fcd, c.eps_c2, c.n, eps)
    }
}

/// 📐 Mean stress factor α_R = F_c/(f_cd x) and relative resultant depth δ from the compressed edge of the parabola-rectangle diagram for the edge strain ε in permille.
pub fn pr_block(c: &Conc, eps: f64) -> (f64, f64) {
    let (e2, n) = (c.eps_c2, c.n);
    let force = if eps <= e2 {
        let u = eps / e2;
        e2 * (u - (1.0 - (1.0 - u).powf(n + 1.0)) / (n + 1.0))
    } else {
        e2 - e2 / (n + 1.0) + (eps - e2)
    };
    let m = integrate_moment(c, eps);
    (force / eps, 1.0 - m / (force * eps))
}

fn integrate_moment(c: &Conc, eps: f64) -> f64 {
    let n = 2000;
    let h = eps / n as f64;
    (0..n).map(|i| {
        let e = (i as f64 + 0.5) * h;
        let s = if e < c.eps_c2 { 1.0 - (1.0 - e / c.eps_c2).powf(c.n) } else { 1.0 };
        s * e * h
    }).sum()
}

/// 📈 Bilinear stress σ_c(ε) in permille (Fig. 3.4).
pub fn sigma_c_bilinear(c: &Conc, fcd: f64, eps: f64) -> f64 {
    if eps <= 0.0 || eps > c.eps_cu3 + 1e-12 { 0.0 } else if eps < c.eps_c3 { fcd * eps / c.eps_c3 } else { fcd }
}

/// 🔩 Reinforcing steel stress (Fig. 3.8): inclined top branch with k = f_t/f_y up to ε_ud = 0.9 ε_uk, or horizontal; strain in permille, tension positive.
pub fn sigma_s(fyd: f64, es: f64, k: f64, eps_uk_permille: f64, inclined: bool, eps: f64) -> f64 {
    let e = eps / 1000.0;
    let ey = fyd / es;
    let a = e.abs();
    let s = if a <= ey {
        es * a
    } else if inclined {
        let eud = 0.9 * eps_uk_permille / 1000.0;
        fyd + (k * fyd - fyd) / (eud - ey) * (a.min(eud) - ey)
    } else {
        fyd
    };
    s * e.signum()
}

pub fn grade(c: &Case) -> Result<Conc, String> {
    Ok(conc(c.pos("fck")?))
}

pub fn m01(c: &Case) -> R {
    let fck = c.pos("fck")?;
    let acc = c.so("situation", "persistent") == "accidental";
    let fcd = f_cd(c.annex, fck, acc);
    let fctd = f_ctd(c.annex, fck);
    let mut out = vec![
        Outcome::mpa("fcd", "Design compressive strength f_cd", "Bemessungswert Betondruckfestigkeit f_cd", fcd, format!("f_cd = α_cc f_ck/γ_c = {}", f(fcd))),
        Outcome::mpa("fctd", "Design tensile strength f_ctd", "Bemessungswert Betonzugfestigkeit f_ctd", fctd, format!("f_ctd = α_ct f_ctk,0.05/γ_c = {}", f(fctd))),
    ];
    if fck > 90.0 {
        return Err("f_ck above 90 MPa is outside the scope of EN 1992-1-1".into());
    }
    out.push(Outcome::mpa("fctk05", "Characteristic tensile strength f_ctk,0.05", "Charakteristische Zugfestigkeit f_ctk,0,05", conc(fck).fctk05, "f_ctk,0.05 of Table 3.1".into()));
    Ok(out)
}

pub fn m02(c: &Case) -> R {
    let fck = c.pos("fck")?;
    if fck > 90.0 {
        return Err("f_ck above 90 MPa is outside the scope of EN 1992-1-1".into());
    }
    let g = conc(fck);
    Ok(vec![
        Outcome::mpa("fcm", "Mean strength f_cm", "Mittelwert f_cm", g.fcm, "f_cm = f_ck + 8".into()),
        Outcome::mpa("fctm", "Mean tensile strength f_ctm", "Mittlere Zugfestigkeit f_ctm", g.fctm, "Table 3.1".into()),
        Outcome::mpa("fctk05", "f_ctk,0.05", "f_ctk,0,05", g.fctk05, "Table 3.1".into()),
        Outcome::mpa("fctk95", "f_ctk,0.95", "f_ctk,0,95", g.fctk95, "Table 3.1".into()),
        Outcome::mpa("ecm", "Secant modulus E_cm", "Sekantenmodul E_cm", g.ecm, "Table 3.1, 22 (f_cm/10)^0.3 GPa between grades".into()),
        Outcome::num("eps_c1", "ε_c1 (permille)", "ε_c1 (Promille)", g.eps_c1, "min(0.7 f_cm^0.31, 2.8)".into()),
        Outcome::num("eps_c2", "ε_c2 (permille)", "ε_c2 (Promille)", g.eps_c2, "Table 3.1".into()),
        Outcome::num("eps_cu2", "ε_cu2 (permille)", "ε_cu2 (Promille)", g.eps_cu2, "Table 3.1".into()),
        Outcome::num("n", "Exponent n", "Exponent n", g.n, "Table 3.1".into()),
        Outcome::num("eps_c3", "ε_c3 (permille)", "ε_c3 (Promille)", g.eps_c3, "Table 3.1".into()),
        Outcome::num("eps_cu3", "ε_cu3 (permille)", "ε_cu3 (Promille)", g.eps_cu3, "= ε_cu2".into()),
        Outcome::info("nonstandard", "Non-standard grade flag (1 = yes)", "Kennzeichen Nicht-Normfestigkeitsklasse (1 = ja)", QuantityKind::Dimensionless, if [12.0, 16.0, 20.0, 25.0, 30.0, 35.0, 40.0, 45.0, 50.0, 55.0, 60.0, 70.0, 80.0, 90.0].contains(&fck) { 0.0 } else { 1.0 }, "Computed from formulas".into()),
    ])
}

pub fn m03(c: &Case) -> R {
    let fck = c.pos("fck")?;
    Ok(vec![Outcome::mpa("fck_cube", "Cube strength f_ck,cube", "Würfeldruckfestigkeit f_ck,cube", f_ck_cube(fck), "Table 3.1 mapping, linear between grades".into())])
}

/// ⏳ β_cc(t) = exp(s (1 − √(28/t))) of Eq. 3.2 for the age `t` in days and the cement coefficient `s`.
pub fn beta_cc(t: f64, s: f64) -> f64 {
    (s * (1.0 - (28.0 / t).sqrt())).exp()
}

/// 🧁 Cement class coefficient s of Eq. 3.2: R 0.20, N 0.25, S 0.38.
pub fn cement_s(class: &str) -> f64 {
    match class { "R" => 0.20, "S" => 0.38, _ => 0.25 }
}

pub fn m04(c: &Case) -> R {
    let (fck, t) = (c.pos("fck")?, c.pos("t")?);
    let g = conc(fck);
    let s = cement_s(c.so("cement", "N"));
    let b = beta_cc(t, s);
    let alpha = if t < 28.0 { 1.0 } else { 2.0 / 3.0 };
    Ok(vec![
        Outcome::mpa("fcm_t", "Mean strength f_cm(t)", "Mittlere Festigkeit f_cm(t)", b * g.fcm, format!("β_cc = {}", f(b))),
        Outcome::mpa("fctm_t", "Mean tensile strength f_ctm(t)", "Mittlere Zugfestigkeit f_ctm(t)", b.powf(alpha) * g.fctm, format!("α = {}", f(alpha))),
        Outcome::mpa("fck_t", "Strength f_ck(t)", "Festigkeit f_ck(t)", if t < 28.0 { b * g.fcm - 8.0 } else { fck }, "f_ck(t) = f_cm(t) − 8 for t < 28 d".into()),
    ])
}

pub fn m05(c: &Case) -> R {
    let g = grade(c)?;
    let fcd = f_cd(c.annex, g.fck, false);
    let eps = c.n("eps")?;
    let b = pr_block(&g, eps.clamp(1e-9, g.eps_cu2));
    let mut v = vec![Outcome::mpa("sigma_c", "Stress σ_c(ε)", "Spannung σ_c(ε)", sigma_c_pr(&g, fcd, eps), format!("Parabola-rectangle, f_cd = {}", f(fcd)))];
    v.push(Outcome::num("alpha_r", "Mean stress factor α_R", "Mittlerer Spannungsbeiwert α_R", b.0, "F_c = α_R f_cd b x".into()));
    v.push(Outcome::num("delta_r", "Resultant depth δ_R from compressed edge", "Lage der Resultierenden δ_R vom Druckrand", b.1, "δ_R x".into()));
    if eps > g.eps_cu2 {
        v.push(Outcome::num("out_of_range", "Strain out of range (1 = yes)", "Dehnung außerhalb des Bereichs (1 = ja)", 1.0, "ε > ε_cu2 gives σ_c = 0".into()));
    }
    Ok(v)
}

pub fn m06(c: &Case) -> R {
    let g = grade(c)?;
    let fcd = f_cd(c.annex, g.fck, false);
    Ok(vec![Outcome::mpa("sigma_c", "Bilinear stress σ_c(ε)", "Bilineare Spannung σ_c(ε)", sigma_c_bilinear(&g, fcd, c.n("eps")?), "Fig. 3.4".into())])
}

/// 📐 Rectangular block parameters λ and η for f_ck (3.1.7(3)).
pub fn lambda_eta(fck: f64) -> (f64, f64) {
    if fck <= 50.0 { (0.8, 1.0) } else { (0.8 - (fck - 50.0) / 400.0, 1.0 - (fck - 50.0) / 200.0) }
}

pub fn m07(c: &Case) -> R {
    let (fck, b, x, d) = (c.pos("fck")?, c.pos("b")?, c.pos("x")?, c.pos("d")?);
    let (lam, eta) = lambda_eta(fck);
    let fcd = f_cd(c.annex, fck, false);
    let narrowing = if c.o("narrowing", 0.0) > 0.0 && c.de() { 0.9 } else { 1.0 };
    let fc = narrowing * eta * fcd * lam * x * b;
    Ok(vec![
        Outcome::num("lambda", "λ", "λ", lam, "3.1.7(3)".into()),
        Outcome::num("eta", "η", "η", eta, "3.1.7(3)".into()),
        Outcome::n("fc", "Compression force F_c", "Betondruckkraft F_c", fc, "F_c = η f_cd λ x b".into()),
        Outcome::mm("z", "Lever arm z", "Hebelarm z", d - lam * x / 2.0, "z = d − λ x/2".into()),
    ])
}

pub fn m08(c: &Case) -> R {
    let (fyk, es) = (c.pos("fyk")?, c.o("es", 200000.0));
    let acc = c.so("situation", "persistent") == "accidental";
    let fyd = f_yd(c.annex, fyk, acc);
    let k = c.o("k", 1.08);
    let euk = c.o("eps_uk", 50.0);
    let eps = c.n("eps")?;
    let inc = c.o("inclined", 1.0) > 0.0;
    let s = sigma_s(fyd, es, k, euk, inc, eps);
    Ok(vec![
        Outcome::mpa("sigma_s", "Steel stress σ_s(ε)", "Stahlspannung σ_s(ε)", s, format!("f_yd = {}", f(fyd))),
        Outcome::num("eps_yd", "Yield strain ε_yd (permille)", "Fließdehnung ε_yd (Promille)", fyd / es * 1000.0, "f_yd/E_s".into()),
        Outcome::num("eps_ud", "Design limit strain ε_ud (permille)", "Rechenwert Grenzdehnung ε_ud (Promille)", 0.9 * euk, "0.9 ε_uk".into()),
    ])
}

pub fn m09(c: &Case) -> R {
    let (fp01k, fpk) = (c.pos("fp01k")?, c.pos("fpk")?);
    let ep = c.o("ep", 195000.0);
    let fpd = fp01k / ndp(c.annex, "ndp.en1992.gamma-s");
    let eps_ud = c.o("eps_ud", 20.0);
    let eps = c.n("eps")?;
    let sp = if eps.abs() / 1000.0 * ep <= fpd { ep * eps / 1000.0 } else { (fpd + (fpk / ndp(c.annex, "ndp.en1992.gamma-s") - fpd) * ((eps.abs().min(eps_ud) / 1000.0 - fpd / ep) / (eps_ud / 1000.0 - fpd / ep))) * eps.signum() };
    Ok(vec![Outcome::mpa("sigma_p", "Prestressing steel stress σ_p(ε)", "Spannstahlspannung σ_p(ε)", sp, format!("f_pd = {}", f(fpd))), Outcome::mpa("fpd", "f_pd = f_p0.1k/γ_s", "f_pd = f_p0,1k/γ_s", fpd, "3.3.6".into())])
}

pub fn m12(c: &Case) -> R {
    let (fck, phi) = (c.pos("fck")?, c.n("phi_inf")?);
    let g = conc(fck);
    let ec = g.ecm / (1.0 + phi);
    Ok(vec![Outcome::mpa("ec_eff", "Effective modulus E_c,eff", "Effektiver Elastizitätsmodul E_c,eff", ec, "E_cm/(1+φ)".into()), Outcome::num("alpha_e_eff", "α_e,eff", "α_e,eff", c.o("es", 200000.0) / ec, "E_s/E_c,eff".into())])
}

pub fn m14(c: &Case) -> R {
    let (fck, b, h) = (c.pos("fck")?, c.pos("b")?, c.pos("h")?);
    let g = conc(fck);
    let fl = ((1.6 - h / 1000.0) * g.fctm).max(g.fctm);
    let w = b * h * h / 6.0;
    let mcr = fl * w;
    let mut v = vec![Outcome::nmm("mcr", "Cracking moment M_cr", "Rissmoment M_cr", mcr, format!("f_ctm,fl = {}", f(fl)))];
    if let (Some(p), Some(e)) = (c.opt("p"), c.opt("e_p")) {
        let ac = b * h;
        v.push(Outcome::nmm("mcr_p", "Cracking moment with prestress M_cr,P", "Rissmoment mit Vorspannung M_cr,P", mcr + p * (e + w / ac), "M_cr + P (e_p + W/A)".into()));
    }
    Ok(v)
}

/// 🌫 Exposure descriptor: rank into the cover rows 0..6 of Table 4.4N, 0 for X0 … 6 for XD3/XS3.
pub fn cover_row(class: &str) -> Option<usize> {
    Some(match class {
        "X0" => 0,
        "XC1" => 1,
        "XC2" | "XC3" => 2,
        "XC4" => 3,
        "XD1" | "XS1" => 4,
        "XD2" | "XS2" => 5,
        "XD3" | "XS3" => 6,
        _ => return None,
    })
}

const COVER_EC: [[f64; 6]; 7] = [
    [10.0, 10.0, 10.0, 10.0, 10.0, 15.0],
    [10.0, 10.0, 10.0, 15.0, 20.0, 25.0],
    [10.0, 15.0, 20.0, 25.0, 30.0, 35.0],
    [15.0, 20.0, 25.0, 30.0, 35.0, 40.0],
    [20.0, 25.0, 30.0, 35.0, 40.0, 45.0],
    [25.0, 30.0, 35.0, 40.0, 45.0, 50.0],
    [30.0, 35.0, 40.0, 45.0, 50.0, 55.0],
];
const COVER_DE: [f64; 7] = [10.0, 10.0, 20.0, 25.0, 40.0, 40.0, 40.0];

/// 📏 c_min,dur in mm for the exposure row and structural class S1..S6 (Table 4.4N; Table 4.4DE has a single column independent of the structural class).
pub fn c_min_dur_mm(annex: AnnexChoice, row: usize, class: u32) -> f64 {
    let s = class.clamp(1, 6) as usize;
    match annex {
        AnnexChoice::En => COVER_EC[row][s - 1],
        AnnexChoice::De => COVER_DE[row],
    }
}

/// 🏗 Structural class S1..S6 from Table 4.3N for a start class S4 (exposure row for the strength threshold).
pub fn structural_class(design_life: f64, fck: f64, row: usize, slab: bool, special: bool) -> u32 {
    let mut s: i32 = 4;
    if design_life >= 100.0 { s += 2; }
    let t = |id: &str| ndp(AnnexChoice::En, &format!("ndp.en1992.d-04.class-fck.{id}"));
    let threshold = [t("xc1"), t("xc1"), t("xc2-xc3"), t("xc4-xd1-xd2-xs1"), t("xc4-xd1-xd2-xs1"), t("xc4-xd1-xd2-xs1"), t("xd3-xs2-xs3")][row];
    if fck >= threshold { s -= 1; }
    if slab { s -= 1; }
    if special { s -= 1; }
    s.clamp(1, 6) as u32
}

pub fn d01(c: &Case) -> R {
    let list = c.s("classes")?;
    let mut worst = 0usize;
    let mut name = "X0";
    for cl in list.split(',').map(str::trim) {
        let Some(r) = cover_row(cl) else { continue };
        if r >= worst { worst = r; name = ["X0", "XC1", "XC2", "XC4", "XD1", "XD2", "XD3"][r]; }
    }
    Ok(vec![Outcome::num("row", &format!("Governing exposure row {name}"), &format!("Maßgebende Expositionszeile {name}"), worst as f64, "The maximum over all classes governs.".into())])
}

/// 📊 Minimum concrete strength class of Annex E (DE) in MPa f_ck.
pub fn min_fck(class: &str, air: bool) -> Option<f64> {
    Some(match class {
        "X0" => 12.0,
        "XC1" => 16.0,
        "XC2" => 16.0,
        "XC3" => 20.0,
        "XC4" => 25.0,
        "XD1" | "XS1" => 30.0,
        "XD2" | "XS2" | "XD3" | "XS3" => 35.0,
        "XF1" => 25.0,
        "XF2" | "XF3" => if air { 25.0 } else { 35.0 },
        "XF4" => 30.0,
        "XA1" => 25.0,
        "XA2" | "XA3" => 35.0,
        "XM1" => 30.0,
        "XM2" | "XM3" => 35.0,
        _ => return None,
    })
}

pub fn d02(c: &Case) -> R {
    let fck = c.pos("fck")?;
    let air = c.o("air_entrained", 0.0) > 0.0;
    let need = c.s("classes")?.split(',').filter_map(|s| min_fck(s.trim(), air)).fold(0.0, f64::max);
    Ok(vec![Outcome::ratio_min("fck_min", "Concrete strength f_ck against minimum class", "Betonfestigkeit f_ck gegen Mindestklasse", fck, need, format!("f_ck,min = {}", f(need)))])
}

pub fn d03(c: &Case) -> R {
    let phi = c.pos("phi")?;
    let nb = c.o("nb", 1.0).max(1.0);
    let dg = c.o("dg", 16.0);
    let row = cover_row(c.s("exposure")?).ok_or("unknown exposure class")?;
    let sc = c.o("structural_class", 4.0) as u32;
    let phi_n = (phi * nb.sqrt()).min(55.0);
    let mut cmin_b = phi_n;
    if c.de() && dg > 16.0 { cmin_b = cmin_b.max(dg + 5.0); }
    let cdur = c_min_dur_mm(c.annex, row, sc);
    let cmin = cmin_b.max(cdur + c.o("dc_gamma", 0.0) - c.o("dc_st", 0.0) - c.o("dc_add", 0.0)).max(10.0);
    Ok(vec![
        Outcome::mm("cmin_b", "c_min,b", "c_min,b", cmin_b, format!("φ_n = {}", f(phi_n))),
        Outcome::mm("cmin_dur", "c_min,dur", "c_min,dur", cdur, "Table 4.4N".into()),
        Outcome::mm("cmin", "c_min", "c_min", cmin, "max(c_min,b, c_min,dur + Δ, 10 mm)".into()),
    ])
}

pub fn d04(c: &Case) -> R {
    let row = cover_row(c.s("exposure")?).ok_or("unknown exposure class")?;
    let sc = structural_class(c.o("design_life", 50.0), c.pos("fck")?, row, c.o("slab", 0.0) > 0.0, c.o("special", 0.0) > 0.0);
    let phi = c.pos("phi")?;
    let cmin = phi.max(c_min_dur_mm(c.annex, row, sc)).max(10.0);
    let dev = c.o("dc_dev", c.ndp("ndp.en1992.delta-c-dev"));
    let cnom = cmin + dev;
    let provided = c.pos("c_provided")?;
    Ok(vec![
        Outcome::num("structural_class", "Structural class S", "Baustoffklasse S", sc as f64, "Table 4.3N".into()),
        Outcome::mm("cnom", "Nominal cover c_nom", "Nennmaß c_nom", cnom, format!("c_min {} + Δc_dev {}", f(cmin), f(dev))),
        Outcome::length_min("c_provided", "Provided cover", "Vorhandene Betondeckung", provided, cnom, "c_provided ≥ c_nom".into()),
    ])
}

pub fn d05(c: &Case) -> R {
    let ground = c.so("ground", "none");
    let v = match ground { "soil" => 75.0, "blinding" => 40.0, _ => 0.0 };
    Ok(vec![Outcome::length_min("c_nom_ground", "Cover against ground", "Betondeckung gegen Erdreich", c.pos("c_provided")?, v, format!("c_nom ≥ {}", f(v)))])
}

pub fn d07(c: &Case) -> R {
    let life = c.pos("design_life")?;
    let cat = if life <= 10.0 { 1.0 } else if life <= 30.0 { 2.0 } else if life <= 50.0 { 4.0 } else { 5.0 };
    Ok(vec![Outcome::num("category", "Design working life category (EN 1990 Table 2.1)", "Nutzungsdauerkategorie (EN 1990 Tabelle 2.1)", cat, format!("{} years", f(life)))])
}

/// 🧵 Creep strain ε_cc = φ σ_c/E_c of 3.1.4(4) under the constant stress σ_c; any consistent unit system.
pub fn creep_strain(phi: f64, sigma_c: f64, e_c: f64) -> f64 {
    phi * sigma_c / e_c
}

/// 🧵 Creep coefficient φ(t,t0) of Annex B.1; arguments fcm (MPa), RH (%), h0 and t0 and t in mm and days, cement class S/N/R.
pub fn creep_phi(fcm: f64, rh: f64, h0: f64, t0: f64, t: f64, cement: &str) -> f64 {
    creep_terms(fcm, rh, h0, t0, t, cement).2
}

/// 🧵 Notional size h_0 = 2 A_c/u of 3.1.4(6) in the unit of the quotient A_c/u.
pub fn notional_size_ac_u(a_c: f64, u: f64) -> f64 {
    2.0 * a_c / u
}

/// 🧵 Cement-class corrected loading age t_0,T of Eq. B.9 in days (α = −1 S, 0 N, 1 R), at least 0.5 d.
pub fn adjusted_loading_age(t0: f64, cement: &str) -> f64 {
    let alpha = match cement { "S" => -1.0, "R" => 1.0, _ => 0.0 };
    (t0 * (9.0 / (2.0 + t0.powf(1.2)) + 1.0).powf(alpha)).max(0.5)
}

/// 🧵 Humidity factor φ_RH of Eq. B.3a/b for f_cm in MPa, RH in percent and h_0 in mm.
pub fn creep_phi_rh(fcm: f64, rh: f64, h0: f64) -> f64 {
    let base = (1.0 - rh / 100.0) / (0.1 * h0.powf(1.0 / 3.0));
    if fcm > 35.0 { (1.0 + base * (35.0 / fcm).powf(0.7)) * (35.0 / fcm).powf(0.2) } else { 1.0 + base }
}

/// 🧵 Humidity coefficient β_H of Eq. B.8a/b for f_cm in MPa, RH in percent and h_0 in mm.
pub fn creep_beta_h(fcm: f64, rh: f64, h0: f64) -> f64 {
    let base = 1.5 * (1.0 + (0.012 * rh).powi(18)) * h0;
    if fcm > 35.0 { let a3 = (35.0 / fcm).powf(0.5); (base + 250.0 * a3).min(1500.0 * a3) } else { (base + 250.0).min(1500.0) }
}

/// 🧵 Creep coefficient φ(t,t0) = φ_0 β_c(t,t0) of Eq. B.1 from φ_RH, β_H and the already corrected loading age; returns (φ_0, φ).
pub fn creep_phi_from(fcm: f64, phi_rh: f64, beta_h: f64, t0: f64, t: f64) -> (f64, f64) {
    let phi0 = phi_rh * 16.8 / fcm.sqrt() / (0.1 + t0.powf(0.2));
    (phi0, phi0 * ((t - t0) / (beta_h + t - t0)).powf(0.3))
}

/// 🧵 Creep terms of Annex B.1 as (φ_RH, φ_0, φ(t,t0)); arguments as [`creep_phi`].
pub fn creep_terms(fcm: f64, rh: f64, h0: f64, t0: f64, t: f64, cement: &str) -> (f64, f64, f64) {
    let phi_rh = creep_phi_rh(fcm, rh, h0);
    let (phi0, phi) = creep_phi_from(fcm, phi_rh, creep_beta_h(fcm, rh, h0), adjusted_loading_age(t0, cement), t);
    (phi_rh, phi0, phi)
}

/// 🧵 Age-adjusted effective modulus E_c,adj = E_c/(1 + χ φ) of Annex KK; any consistent stress unit.
pub fn age_adjusted_modulus(e_c: f64, chi: f64, phi: f64) -> f64 {
    e_c / (1.0 + chi * phi)
}

pub fn a01(c: &Case) -> R {
    let fck = c.pos("fck")?;
    let fcm = fck + 8.0;
    let (rh, h0, t0) = (c.pos("rh")?, c.pos("h0")?, c.pos("t0")?);
    let cement = c.so("cement", "N");
    let mut v = vec![Outcome::num("phi_inf", "Creep coefficient φ(∞,t0)", "Kriechzahl φ(∞,t0)", creep_phi(fcm, rh, h0, t0, 1.0e9, cement), format!("RH {} %, h0 {} mm, t0 {} d", f(rh), f(h0), f(t0)))];
    if let Some(t) = c.opt("t") {
        v.push(Outcome::num("phi_t", "Creep coefficient φ(t,t0)", "Kriechzahl φ(t,t0)", creep_phi(fcm, rh, h0, t0, t, cement), "β_c(t,t0)".into()));
    }
    if let Some(s) = c.opt("sigma_c") {
        let k = s / (fck - 0.0).max(1.0);
        if k > 0.45 {
            v.push(Outcome::num("phi_nl", "Non-linear creep coefficient φ_nl", "Nichtlineare Kriechzahl φ_nl", v[0].value() * (1.5 * (k - 0.45)).exp(), format!("k_σ = {}", f(k))));
        }
    }
    Ok(v)
}

/// 📉 Basic drying shrinkage strain ε_cd,0 of Eq. B.11 for f_ck in MPa, RH in percent and the cement class S/N/R.
pub fn eps_cd_0(fck: f64, rh: f64, cement: &str) -> f64 {
    let (ads1, ads2) = match cement { "S" => (3.0, 0.13), "R" => (6.0, 0.11), _ => (4.0, 0.12) };
    0.85 * (220.0 + 110.0 * ads1) * (-ads2 * (fck + 8.0) / 10.0).exp() * 1e-6 * 1.55 * (1.0 - (rh / 100.0).powi(3))
}

/// 📉 Shrinkage strain ε_cs of Annex B.2 and 3.1.4(6) in absolute strain; t and ts in days.
pub fn shrinkage(fck: f64, rh: f64, h0: f64, t: f64, ts: f64, cement: &str) -> (f64, f64, f64) {
    let ecd0 = eps_cd_0(fck, rh, cement);
    let kh = interp(&[(100.0, 1.0), (200.0, 0.85), (300.0, 0.75), (500.0, 0.70)], h0);
    let bds = (t - ts) / ((t - ts) + 0.04 * h0.powf(1.5));
    let eca_inf = 2.5 * (fck - 10.0) * 1e-6;
    let bas = 1.0 - (-0.2 * t.sqrt()).exp();
    let ecd = bds * kh * ecd0;
    let eca = bas * eca_inf;
    (ecd, eca, ecd + eca)
}

pub fn a02(c: &Case) -> R {
    let (fck, rh, h0) = (c.pos("fck")?, c.pos("rh")?, c.pos("h0")?);
    let ts = c.o("ts", 3.0);
    let cement = c.so("cement", "N");
    let (d, a, s) = shrinkage(fck, rh, h0, 1.0e9, ts, cement);
    let mut v = vec![
        Outcome::num("eps_cd_inf", "Drying shrinkage ε_cd,∞", "Trocknungsschwinden ε_cd,∞", d, "k_h ε_cd,0".into()),
        Outcome::num("eps_ca_inf", "Autogenous shrinkage ε_ca,∞", "Autogenes Schwinden ε_ca,∞", a, "2.5 (f_ck − 10) 1e-6".into()),
        Outcome::num("eps_cs_inf", "Total shrinkage ε_cs,∞", "Gesamtschwinden ε_cs,∞", s, "ε_cd + ε_ca".into()),
    ];
    if let Some(t) = c.opt("t") {
        v.push(Outcome::num("eps_cs_t", "Shrinkage ε_cs(t)", "Schwinden ε_cs(t)", shrinkage(fck, rh, h0, t, ts, cement).2, "β_ds, β_as".into()));
    }
    if rh >= 99.0 {
        v.push(Outcome::num("swelling", "Swelling not covered (1 = yes)", "Quellen nicht abgedeckt (1 = ja)", 1.0, "RH ≥ 99 %".into()));
    }
    Ok(v)
}

/// 🗂 Entries of this domain.
pub const ENTRIES: &[Entry] = &[
    entry_of("M-01", "DIN EN 1992-1-1", "3.1.6", "Design strength of concrete", "Bemessungswerte der Betonfestigkeit", &["fck", "situation"], &["ndp.en1992.m-01.alpha-cc-persistent", "ndp.en1992.m-01.alpha-cc-accidental", "ndp.en1992.m-01.alpha-ct", "ndp.en1992.m-01.gamma-c-persistent", "ndp.en1992.m-01.gamma-c-accidental"], m01),
    entry_of("M-02", "DIN EN 1992-1-1", "3.1.3", "Concrete property table", "Betonkennwerte", &["fck"], &[], m02),
    entry_of("M-03", "DIN EN 1992-1-1", "3.1.2", "Cube and cylinder strength", "Zylinder-Würfel-Zuordnung", &["fck"], &[], m03),
    entry_of("M-04", "DIN EN 1992-1-1", "3.1.2", "Strength development in time", "Zeitliche Entwicklung der Festigkeit", &["fck", "t", "cement"], &[], m04),
    entry_of("M-05", "DIN EN 1992-1-1", "3.1.5", "Parabola-rectangle diagram", "Parabel-Rechteck-Diagramm", &["fck", "eps"], &["ndp.en1992.m-01.alpha-cc-persistent", "ndp.en1992.m-01.gamma-c-persistent"], m05),
    entry_of("M-06", "DIN EN 1992-1-1", "3.1.5", "Bilinear concrete diagram", "Bilineares Betondiagramm", &["fck", "eps"], &["ndp.en1992.m-01.alpha-cc-persistent", "ndp.en1992.m-01.gamma-c-persistent"], m06),
    entry_of("M-07", "DIN EN 1992-1-1", "3.1.7", "Rectangular stress block", "Rechteckspannungsblock", &["fck", "b", "x", "d", "narrowing"], &["ndp.en1992.m-01.alpha-cc-persistent", "ndp.en1992.m-01.gamma-c-persistent"], m07),
    entry_of("M-08", "DIN EN 1992-1-1", "3.2.7", "Reinforcing steel diagram", "Spannungs-Dehnungs-Linie Betonstahl", &["fyk", "es", "k", "eps_uk", "inclined", "eps"], &["ndp.en1992.gamma-s", "ndp.en1992.gamma-s-acc"], m08),
    entry_of("M-09", "DIN EN 1992-1-1", "3.3.6", "Prestressing steel diagram", "Spannstahl-Linie", &["fp01k", "fpk", "ep", "eps", "eps_ud"], &["ndp.en1992.gamma-s"], m09),
    entry_of("M-12", "DIN EN 1992-1-1", "3.1.4", "Effective modulus", "Effektiver Elastizitätsmodul", &["fck", "phi_inf", "es"], &[], m12),
    entry_of("S-12", "DIN EN 1992-1-1", "3.1.4(2)", "Effective modulus (section view)", "Effektiver Elastizitätsmodul (Querschnitt)", &["fck", "phi_inf", "es"], &[], m12),
    entry_of("M-14", "DIN EN 1992-1-1", "7.1", "Cracking moment", "Rissmoment", &["fck", "b", "h", "p", "e_p"], &[], m14),
    entry_of("M-10", "DIN EN 1992-1-1", "3.1.4", "Creep coefficient (materials view)", "Kriechzahl (Baustoffsicht)", &["fck", "rh", "h0", "t0", "t", "cement", "sigma_c"], &[], a01),
    entry_of("M-11", "DIN EN 1992-1-1", "3.1.4(6)", "Shrinkage (materials view)", "Schwinden (Baustoffsicht)", &["fck", "rh", "h0", "ts", "t", "cement"], &[], a02),
    entry_of("D-01", "DIN EN 1992-1-1", "4.2", "Exposure classification", "Expositionsklassen", &["classes"], &[], d01),
    entry_of("D-02", "DIN EN 1992-1-1", "E.1DE", "Minimum concrete strength class", "Mindestbetonfestigkeitsklasse", &["fck", "classes", "air_entrained"], &[], d02),
    entry_of("D-03", "DIN EN 1992-1-1", "4.4.1.2", "Minimum cover", "Mindestbetondeckung", &["phi", "nb", "dg", "exposure", "structural_class", "dc_gamma", "dc_st", "dc_add"], &["ndp.en1992.cmin-dur-table"], d03),
    entry_of("D-04", "DIN EN 1992-1-1", "4.4.1.1", "Nominal cover", "Nennmaß der Betondeckung", &["exposure", "fck", "phi", "design_life", "slab", "special", "dc_dev", "c_provided"], &["ndp.en1992.cmin-dur-table", "ndp.en1992.delta-c-dev"], d04),
    entry_of("D-05", "DIN EN 1992-1-1", "4.4.1.3", "Cover against ground", "Betondeckung gegen Erdreich", &["ground", "c_provided"], &[], d05),
    entry_of("D-07", "DIN EN 1992-1-1", "4.3", "Design working life", "Nutzungsdauer", &["design_life"], &[], d07),
    entry_of("A-01", "DIN EN 1992-1-1", "B.1", "Creep coefficient", "Kriechzahl", &["fck", "rh", "h0", "t0", "t", "cement", "sigma_c"], &[], a01),
    entry_of("A-02", "DIN EN 1992-1-1", "B.2", "Shrinkage strain", "Schwinddehnung", &["fck", "rh", "h0", "ts", "t", "cement"], &[], a02),
];
