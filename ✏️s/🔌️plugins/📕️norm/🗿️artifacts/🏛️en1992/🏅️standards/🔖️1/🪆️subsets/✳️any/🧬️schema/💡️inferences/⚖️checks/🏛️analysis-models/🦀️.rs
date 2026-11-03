//! 🏛️ Structural analysis models of EN 1992-1-1 chapter 5 as catalogue entries A-03 … A-15: imperfections, flange width, redistribution, plastic rotation, slenderness, nominal curvature, nominal stiffness, the general method and lateral stability. See [`super`].

use super::*;
use std::f64::consts::PI;

pub fn a03(c: &Case) -> R {
    let (l, m) = (c.pos("l_m")?, c.o("m", 1.0));
    let th0 = 1.0 / c.ndp("ndp.en1992.imperfection-theta0-divisor");
    let ah = (2.0 / l.sqrt()).clamp(2.0 / 3.0, 1.0);
    let am = (0.5 * (1.0 + 1.0 / m)).sqrt();
    let th = th0 * ah * am;
    let l0 = c.o("l0", l * 1000.0);
    let mut v = vec![Outcome::num("theta_i", "Inclination θ_i (rad)", "Schiefstellung θ_i (rad)", th, format!("α_h = {}, α_m = {}", f(ah), f(am))), Outcome::mm("e_i", "Eccentricity e_i", "Imperfektionsausmitte e_i", th * l0 / 2.0, "θ_i l_0/2".into())];
    if let Some(n) = c.opt("n_ed") { v.push(Outcome::n("h_i", "Equivalent horizontal force H_i", "Ersatzhorizontalkraft H_i", th * n, "θ_i N".into())); }
    Ok(v)
}

/// 📐 Effective flange width b_eff = Σ b_eff,i + b_w ≤ b for the flange lengths `b_i` (mm).
pub fn beff(b_i: &[f64], bw: f64, l0: f64) -> f64 {
    b_i.iter().map(|bi| (0.2 * bi + 0.1 * l0).min(0.2 * l0).min(*bi)).sum::<f64>() + bw
}

pub fn a04(c: &Case) -> R {
    let (bw, l0) = (c.pos("bw")?, c.pos("l0")?);
    let bi = c.l("b_i")?;
    let b = bi.iter().sum::<f64>() + bw;
    let be = beff(bi, bw, l0).min(b);
    Ok(vec![Outcome::mm("beff", "Effective flange width b_eff", "Mitwirkende Plattenbreite b_eff", be, format!("l_0 = {}", f(l0)))])
}

pub fn a05(c: &Case) -> R {
    let (b, h, d, a_s, fck) = (c.pos("b")?, c.pos("h")?, c.pos("d")?, c.pos("as")?, c.pos("fck")?);
    let g = conc(fck);
    let ae = c.o("es", 200000.0) / g.ecm;
    let i1 = b * h.powi(3) / 12.0;
    let (ei2, x) = match super::serviceability::cracked_state(b, h, d, a_s, 0.1 * h, 0.0, ae, c.pos("m")?, 0.0) { Some(s) => (g.ecm * s.1, s.0), None => (g.ecm * i1, h / 2.0) };
    Ok(vec![Outcome::info("ei_uncracked", "Uncracked stiffness EI_I (N·mm²)", "Ungerissene Steifigkeit EI_I (N·mm²)", QuantityKind::Dimensionless, g.ecm * i1, "E_cm I_c".into()), Outcome::info("ei_cracked", "Cracked stiffness EI_II (N·mm²)", "Gerissene Steifigkeit EI_II (N·mm²)", QuantityKind::Dimensionless, ei2, format!("x = {}", f(x))), Outcome::num("ratio", "EI_II/EI_I", "EI_II/EI_I", ei2 / (g.ecm * i1), "stiffness reduction".into())])
}

pub fn a06(c: &Case) -> R {
    let (fck, xd, delta) = (c.pos("fck")?, c.pos("xu_d")?, c.pos("delta")?);
    let ecu = conc(fck).eps_cu2 / 1000.0;
    let k2 = 1.25 * (0.6 + 0.0014 / ecu);
    let (k1, k2n) = if fck <= 50.0 { (c.ndp("ndp.en1992.redistribution-k1"), if c.de() { c.ndp("ndp.en1992.redistribution-k2") } else { k2 }) } else { (0.54, 1.25 * (0.6 + 0.0014 / ecu)) };
    let dmin = (k1 + k2n * xd).max(if c.so("steel_class", "B") == "A" { 0.8 } else { 0.7 });
    let xlim = if fck <= 50.0 { 0.45 } else { 0.35 };
    Ok(vec![Outcome::ratio_min("delta", "Redistribution ratio δ", "Umlagerungsverhältnis δ", delta, dmin, format!("δ_min = {}", f(dmin))), Outcome::ratio("xu_d", "Neutral axis depth ratio x_u/d", "Druckzonenhöhenverhältnis x_u/d", xd, xlim, "x_u/d ≤ limit".into())])
}

/// 📉 Fig. 5.6N allowable plastic rotation (x_u/d, mrad) at λ = 3.0 for reinforcement class B, read from the figure and therefore listed as unverified.
const FIG_56_CLASS_B: [(f64, f64); 5] = [(0.05, 22.0), (0.1, 17.0), (0.2, 11.0), (0.3, 7.5), (0.45, 4.0)];
/// 📉 Fig. 5.6N allowable plastic rotation (x_u/d, mrad) at λ = 3.0 for reinforcement class C, read from the figure and therefore listed as unverified.
const FIG_56_CLASS_C: [(f64, f64); 5] = [(0.05, 55.0), (0.1, 44.0), (0.2, 26.0), (0.3, 17.0), (0.45, 9.0)];

pub fn a07(c: &Case) -> R {
    let (fck, xd, theta) = (c.pos("fck")?, c.pos("xu_d")?, c.pos("theta_pl")?);
    let (b, h, d) = (c.pos("b")?, c.pos("h")?, c.pos("d")?);
    let g = conc(fck);
    let lambda = c.o("lambda_shear", 3.0);
    let klam = (lambda / 3.0).sqrt();
    let fyd = f_yd(c.annex, c.o("fyk", 500.0), false);
    let ey = fyd / c.o("es", 200000.0);
    let kappa_u = g.eps_cu2 / 1000.0 / (xd * d);
    let kappa_y = ey / ((1.0 - c.o("xy_d", 0.3)) * d);
    let l_pl = 1.2 * h;
    let _ = b;
    let cap = ((kappa_u - kappa_y).max(0.0) * l_pl) * klam;
    let table: &[(f64, f64)] = if c.so("steel_class", "B") == "C" { &FIG_56_CLASS_C } else { &FIG_56_CLASS_B };
    let fig = interp(table, xd) * 1e-3 * klam;
    let lim = if fck <= 50.0 { 0.25 } else { 0.15 };
    Ok(vec![Outcome::num("theta_pl_d", "Plastic rotation capacity θ_pl,d (rad)", "Plastische Rotationsfähigkeit θ_pl,d (rad)", cap, format!("k_λ = {}", f(klam))), Outcome::ratio("util", "Rotation utilisation", "Rotationsausnutzung", theta, cap.max(1e-12), "θ_pl ≤ θ_pl,d".into()), Outcome::ratio("xu_d", "x_u/d limit", "Grenze x_u/d", xd, lim, "plastic analysis".into()), Outcome::num("theta_fig56", "Allowable rotation of Fig. 5.6N θ_pl,d (rad)", "Zulässige Rotation nach Bild 5.6N θ_pl,d (rad)", fig, format!("x_u/d = {}, k_λ = {}, curve read from the figure [unverified]", f(xd), f(klam))), Outcome::ratio("util_fig56", "Rotation utilisation against Fig. 5.6N", "Rotationsausnutzung nach Bild 5.6N", theta, fig.max(1e-12), "θ_pl ≤ θ_pl,d (Fig. 5.6N)".into())])
}

pub fn a08(c: &Case) -> R {
    let (ed, rm) = (c.pos("e_d")?, c.pos("r_m")?);
    let g = c.ndp("ndp.en1992.gamma-o-nl");
    Ok(vec![Outcome::force("util", "Non-linear analysis safety format", "Sicherheitsformat nichtlineare Berechnung", ed, rm / g, format!("R_d = R_m/γ_O, γ_O = {}", f(g)))])
}

fn eff_length(k1: f64, k2: f64, l: f64, braced: bool) -> f64 {
    if braced {
        0.5 * l * ((1.0 + k1 / (0.45 + k1)) * (1.0 + k2 / (0.45 + k2))).sqrt()
    } else {
        l * (1.0 + 10.0 * k1 * k2 / (k1 + k2)).sqrt().max((1.0 + k1 / (1.0 + k1)) * (1.0 + k2 / (1.0 + k2)))
    }
}

pub fn a09(c: &Case) -> R {
    let (fck, b, h, ned) = (c.pos("fck")?, c.pos("b")?, c.pos("h")?, c.pos("n_ed")?);
    let ac = b * h;
    let fcd = f_cd(c.annex, fck, false);
    let fyd = f_yd(c.annex, c.o("fyk", 500.0), false);
    let l0 = match (c.opt("k1"), c.opt("k2")) { (Some(k1), Some(k2)) => eff_length(k1, k2, c.pos("l")?, c.so("bracing", "braced") == "braced"), _ => c.pos("l0")? };
    let i = c.opt("i").unwrap_or(h.min(b) / 12.0f64.sqrt());
    let lam = l0 / i;
    let n = ned / (ac * fcd);
    let phi_ef = c.opt("phi_ef");
    let a = phi_ef.map(|p| 1.0 / (1.0 + 0.2 * p)).unwrap_or(0.7);
    let omega = c.opt("as").map(|a_s| a_s * fyd / (ac * fcd));
    let bb = omega.map(|o| (1.0 + 2.0 * o).sqrt()).unwrap_or(1.1);
    let cc = c.opt("rm").map(|r| 1.7 - r).unwrap_or(0.7);
    let mut lim = 20.0 * a * bb * cc / n.max(1e-9).sqrt();
    if c.de() && n >= c.ndp("ndp.en1992.lambda-lim-de-n") && c.o("isolated", 1.0) > 0.0 { lim = lim.max(25.0); }
    let mut v = vec![Outcome::num("lambda", "Slenderness λ", "Schlankheit λ", lam, format!("l_0 = {}, i = {}", f(l0), f(i))), Outcome::num("lambda_lim", "Limit slenderness λ_lim", "Grenzschlankheit λ_lim", lim, format!("A = {}, B = {}, C = {}, n = {}", f(a), f(bb), f(cc), f(n))), Outcome::ratio("util", "Second order effects may be neglected", "Theorie II. Ordnung vernachlässigbar", lam, lim, "λ ≤ λ_lim".into())];
    if let (Some(ly), Some(lz)) = (c.opt("lambda_y"), c.opt("lambda_z")) {
        v.push(Outcome::ratio("lambda_ratio", "λ_y/λ_z ≤ 2 for separate checks", "λ_y/λ_z ≤ 2 für getrennte Nachweise", (ly / lz).max(lz / ly), 2.0, "5.8.9(3)".into()));
    }
    Ok(v)
}

/// 🧮 Curvature 1/r = K_r K_φ ε_yd/(0.45 d) (Eq. 5.34) with 1/r_0 = f_yd/(E_s 0.45 d); any consistent unit system.
pub fn curvature_534(k_r: f64, k_phi: f64, f_yd: f64, e_s: f64, d: f64) -> f64 {
    k_r * k_phi * f_yd / e_s / (0.45 * d)
}

/// 🧮 Nominal curvature method of 5.8.8 (mm, N, MPa): returns (e_2, K_r, K_φ, 1/r).
pub fn nominal_curvature(fck: f64, fcd: f64, fyd: f64, es: f64, b: f64, h: f64, a_s: f64, is: f64, ned: f64, l0: f64, phi_ef: f64, c: f64) -> (f64, f64, f64, f64) {
    let ac = b * h;
    let d = h / 2.0 + is;
    let omega = a_s * fyd / (ac * fcd);
    let n = ned / (ac * fcd);
    let nu = 1.0 + omega;
    let kr = ((nu - n) / (nu - 0.4)).clamp(0.0, 1.0);
    let lam = l0 / (h.min(b) / 12.0f64.sqrt());
    let beta = 0.35 + fck / 200.0 - lam / 150.0;
    let kphi = (1.0 + beta * phi_ef).max(1.0);
    let inv_r = curvature_534(kr, kphi, fyd, es, d);
    (inv_r * l0 * l0 / c, kr, kphi, inv_r)
}

pub fn a10(c: &Case) -> R {
    let (fck, b, h, ned, l0) = (c.pos("fck")?, c.pos("b")?, c.pos("h")?, c.pos("n_ed")?, c.pos("l0")?);
    let fcd = f_cd(c.annex, fck, false);
    let fyd = f_yd(c.annex, c.o("fyk", 500.0), false);
    let (a_s, is) = (c.pos("as")?, c.pos("is")?);
    let (e2, kr, kphi, inv_r) = nominal_curvature(fck, fcd, fyd, c.o("es", 200000.0), b, h, a_s, is, ned, l0, c.o("phi_ef", 0.0), c.o("c", PI * PI));
    let ei = c.o("ei", l0 / 400.0);
    let e0 = c.o("e0", (h / 30.0).max(20.0));
    let m_ed = ned * (e0 + ei + e2);
    let mut v = vec![Outcome::mm("e2", "Second-order eccentricity e_2", "Ausmitte II. Ordnung e_2", e2, format!("K_r = {}, K_φ = {}", f(kr), f(kphi))), Outcome::num("inv_r", "Curvature 1/r (1/mm)", "Krümmung 1/r (1/mm)", inv_r, "K_r K_φ 1/r_0".into()), Outcome::nmm("m_ed", "Design moment M_Ed", "Bemessungsmoment M_Ed", m_ed, format!("e_tot = {}", f(e0 + ei + e2))), Outcome::mm("d_eff", "Effective depth d = h/2 + i_s", "Wirksame Höhe d = h/2 + i_s", h / 2.0 + is, "5.8.8.2".into())];
    if c.l("bar_y").is_ok() {
        let s = section_of(c)?;
        let (mrd, _) = s.m_rd_y(ned).ok_or("N_Ed outside the section resistance")?;
        v.push(Outcome::moment("util", "Slender column utilisation", "Ausnutzung schlanke Stütze", m_ed, mrd, format!("M_Rd = {}", f(mrd))));
    }
    Ok(v)
}

pub fn a11(c: &Case) -> R {
    let (fck, b, h, ned, l0) = (c.pos("fck")?, c.pos("b")?, c.pos("h")?, c.pos("n_ed")?, c.pos("l0")?);
    let g = conc(fck);
    let ac = b * h;
    let fcd = f_cd(c.annex, fck, false);
    let phi_ef = c.o("phi_ef", 0.0);
    let ecd = g.ecm / c.ndp("ndp.en1992.gamma-ce");
    let ic = b * h.powi(3) / 12.0;
    let a_s = c.pos("as")?;
    let is = c.pos("is_moment")?;
    let n = ned / (ac * fcd);
    let lam = l0 / (h / 12.0f64.sqrt());
    let k1 = (fck / 20.0).sqrt();
    let k2 = (n * lam / 170.0).min(0.2);
    let rho = a_s / ac;
    let (kc, ks) = if rho >= 0.002 { (k1 * k2 / (1.0 + phi_ef), 1.0) } else { (0.3 / (1.0 + 0.5 * phi_ef), 0.0) };
    let ei = kc * ecd * ic + ks * c.o("es", 200000.0) * is;
    let nb = PI * PI * ei / (l0 * l0);
    let c0 = c.o("c0", 8.0);
    let beta = PI * PI / c0;
    let m0 = c.n("m_0ed")?;
    let m = if ned < nb { m0 * (1.0 + beta / (nb / ned - 1.0)) } else { f64::INFINITY };
    Ok(vec![Outcome::info("ei", "Nominal stiffness EI (N·mm²)", "Nominelle Steifigkeit EI (N·mm²)", QuantityKind::Dimensionless, ei, format!("K_c = {}, K_s = {}", f(kc), f(ks))), Outcome::n("n_b", "Buckling load N_B", "Knicklast N_B", nb, "π² EI/l_0²".into()), Outcome::nmm("m_ed", "Amplified moment M_Ed", "Vergrößertes Moment M_Ed", m, "M_0Ed (1 + β/(N_B/N_Ed − 1))".into()), Outcome::force("stability", "Stability N_Ed < N_B", "Stabilität N_Ed < N_B", ned, nb, "N_Ed ≤ N_B".into())])
}

pub fn a12(c: &Case) -> R {
    s05_like(c)
}

pub fn s05_like(c: &Case) -> R {
    let (nrd, mrdy, mrdz) = (c.pos("n_rd")?, c.pos("m_rdy")?, c.pos("m_rdz")?);
    let (ned, my, mz) = (c.n("n_ed")?, c.n("m_edy")?, c.n("m_edz")?);
    let a = super::section_resistance::biaxial_exponent(ned / nrd);
    let amp_y = c.o("amp_y", 1.0);
    let amp_z = c.o("amp_z", 1.0);
    let lhs = (mz.abs() * amp_z / mrdz).powf(a) + (my.abs() * amp_y / mrdy).powf(a);
    Ok(vec![Outcome::ratio("util", "Biaxial utilisation with amplified moments", "Zweiachsige Ausnutzung mit vergrößerten Momenten", lhs, 1.0, format!("a = {}", f(a)))])
}

pub fn a13(c: &Case) -> R {
    let mut s = section_of(c)?;
    s.steps = c.o("steps", 120.0) as usize;
    let (ned, l0, e0) = (c.pos("n_ed")?, c.pos("l0")?, c.o("e0", 0.0));
    let (lo, hi) = s.extent((0.0, 1.0));
    let depth = hi - lo;
    let alpha = PI / 2.0;
    let kmax = 2.0 * s.c.eps_cu2 / 1000.0 / depth * 2.0;
    let nk = c.o("kappa_points", 28.0) as usize;
    let m_int = |nn: f64, kappa: f64| -> Option<f64> {
        let (mut a, mut bb) = (1e-4 * depth, 1e3 * depth);
        let n_of = |x: f64| s.state_plane(alpha, kappa * x * 1000.0, x).0;
        if n_of(a) > nn || n_of(bb) < nn { return None; }
        for _ in 0..50 {
            let m = (a * bb).sqrt();
            if n_of(m) < nn { a = m } else { bb = m }
        }
        let x = (a * bb).sqrt();
        Some(s.state_plane(alpha, kappa * x * 1000.0, x).1)
    };
    let stable = |gamma: f64| -> bool {
        let nn = gamma * ned;
        (1..=nk).any(|i| {
            let k = kmax * i as f64 / nk as f64;
            m_int(nn, k).map(|mi| mi >= nn * (e0 + k * l0 * l0 / (PI * PI))).unwrap_or(false)
        })
    };
    let (mut lo_g, mut hi_g) = (0.0, 8.0);
    if !stable(1e-6) { return Err("section cannot carry any load".into()); }
    for _ in 0..c.o("bisect", 14.0) as usize {
        let m = 0.5 * (lo_g + hi_g);
        if stable(m.max(1e-6)) { lo_g = m } else { hi_g = m }
    }
    Ok(vec![Outcome::ratio("util", "Inverse load factor 1/γ (general method)", "Inverser Laststeigerungsfaktor 1/γ (allgemeines Verfahren)", 1.0, lo_g.max(1e-9), format!("γ = {}", f(lo_g))), Outcome::num("gamma", "Load factor γ at failure", "Laststeigerungsfaktor γ im Versagen", lo_g, "M_int(κ) ≥ N (e_0 + κ l_0²/π²)".into())])
}

pub fn a14(c: &Case) -> R {
    let (fv, ecd_ic, l, ns) = (c.pos("f_v_ed")?, c.pos("sum_ecd_ic")?, c.pos("l")?, c.pos("n_s")?);
    let k1 = c.ndp("ndp.en1992.k1-global");
    let lim = k1 * ecd_ic / (l * l) * ns / (ns + 1.6);
    Ok(vec![Outcome::force("util", "Global second-order criterion", "Kriterium globale Theorie II. Ordnung", fv, lim, format!("k_1 = {}", f(k1)))])
}

pub fn a15(c: &Case) -> R {
    let (l0t, b, h) = (c.pos("l0t")?, c.pos("b")?, c.pos("h")?);
    let transient = c.o("transient", 0.0) > 0.5;
    let (k, hb_max) = if transient { (70.0, 3.5) } else { (50.0, 2.5) };
    Ok(vec![Outcome::length("util_l", "Lateral instability l_0t/b", "Kippen l_0t/b", l0t / b, k / (h / b).powf(1.0 / 3.0), format!("l_0t/b ≤ {k}/(h/b)^(1/3), {} situation", if transient { "transient" } else { "persistent" })), Outcome::length("util_h", "Aspect ratio h/b", "Seitenverhältnis h/b", h / b, hb_max, format!("h/b ≤ {hb_max}"))])
}

/// 🗂 Entries of this domain.
pub const ENTRIES: &[Entry] = &[
    entry_of("A-03", "DIN EN 1992-1-1", "5.2", "Imperfections", "Imperfektionen", &["l_m", "m", "l0", "n_ed"], &["ndp.en1992.imperfection-theta0-divisor"], a03),
    entry_of("A-04", "DIN EN 1992-1-1", "5.3.2.1", "Effective flange width", "Mitwirkende Plattenbreite", &["bw", "l0", "b_i"], &[], a04),
    entry_of("A-05", "DIN EN 1992-1-1", "5.4", "Linear elastic analysis stiffness", "Linear-elastische Berechnung, Steifigkeit", &["b", "h", "d", "as", "fck", "m", "es"], &[], a05),
    entry_of("A-06", "DIN EN 1992-1-1", "5.5", "Redistribution", "Umlagerung", &["fck", "xu_d", "delta", "steel_class"], &["ndp.en1992.redistribution-k1", "ndp.en1992.redistribution-k2"], a06),
    entry_of("A-07", "DIN EN 1992-1-1", "5.6", "Plastic rotation capacity", "Plastische Rotationsfähigkeit", &["fck", "xu_d", "theta_pl", "b", "h", "d", "lambda_shear", "fyk", "xy_d", "steel_class"], &["ndp.en1992.m-01.alpha-cc-persistent", "ndp.en1992.m-01.gamma-c-persistent", "ndp.en1992.gamma-s", "ndp.en1992.plastic-rotation-model"], a07),
    entry_of("A-08", "DIN EN 1992-1-1", "5.7", "Non-linear analysis safety format", "Nichtlineare Berechnung, Sicherheitsformat", &["e_d", "r_m"], &["ndp.en1992.gamma-o-nl"], a08),
    entry_of("A-09", "DIN EN 1992-1-1", "5.8.3", "Slenderness and limit slenderness", "Schlankheit und Grenzschlankheit", &["fck", "b", "h", "n_ed", "k1", "k2", "l", "bracing", "l0", "i", "phi_ef", "as", "rm", "isolated", "lambda_y", "lambda_z", "fyk"], &["ndp.en1992.lambda-lim-de-n", "ndp.en1992.m-01.alpha-cc-persistent", "ndp.en1992.m-01.gamma-c-persistent", "ndp.en1992.gamma-s"], a09),
    entry_of("A-10", "DIN EN 1992-1-1", "5.8.8", "Nominal curvature method", "Modellstützenverfahren", &["fck", "b", "h", "n_ed", "l0", "as", "is", "phi_ef", "c", "ei", "e0", "fyk", "es", "bar_y", "bar_z", "bar_a"], &["ndp.en1992.m-01.alpha-cc-persistent", "ndp.en1992.m-01.gamma-c-persistent", "ndp.en1992.gamma-s"], a10),
    entry_of("A-11", "DIN EN 1992-1-1", "5.8.7", "Nominal stiffness method", "Verfahren mit Nennsteifigkeit", &["fck", "b", "h", "n_ed", "l0", "as", "is_moment", "phi_ef", "c0", "m_0ed", "es"], &["ndp.en1992.gamma-ce", "ndp.en1992.m-01.alpha-cc-persistent", "ndp.en1992.m-01.gamma-c-persistent"], a11),
    entry_of("A-12", "DIN EN 1992-1-1", "5.8.9", "Biaxial bending of slender columns", "Zweiachsige Biegung schlanker Stützen", &["n_rd", "m_rdy", "m_rdz", "n_ed", "m_edy", "m_edz", "amp_y", "amp_z"], &[], a12),
    entry_of("A-13", "DIN EN 1992-1-1", "5.8.6", "General method", "Allgemeines Verfahren", &["fck", "b", "h", "bar_y", "bar_z", "bar_a", "n_ed", "l0", "e0", "kappa_points", "bisect", "steps"], &["ndp.en1992.m-01.alpha-cc-persistent", "ndp.en1992.m-01.gamma-c-persistent", "ndp.en1992.gamma-s"], a13),
    entry_of("A-14", "DIN EN 1992-1-1", "5.8.3.3", "Global second-order effects", "Globale Effekte II. Ordnung", &["f_v_ed", "sum_ecd_ic", "l", "n_s"], &["ndp.en1992.k1-global"], a14),
    entry_of("A-15", "DIN EN 1992-1-1", "5.9", "Lateral instability of slender beams", "Kippen schlanker Träger", &["l0t", "b", "h", "transient"], &[], a15),
];
