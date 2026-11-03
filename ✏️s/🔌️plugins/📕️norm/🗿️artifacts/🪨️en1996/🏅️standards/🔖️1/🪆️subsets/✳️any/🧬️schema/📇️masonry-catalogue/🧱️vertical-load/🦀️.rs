//! 🧱️ Vertical load entries of the EN 1996 catalogue: M-10 … M-16 effective height, slenderness, eccentricities, Φ at top, mid-height and bottom, concentrated loads and openings. See [`super`].

use super::*;

pub(super) const ENTRIES: &[Entry] = &[
    Entry { id: "M-10", part: "EN 1996-1-1", clause: "5.5.1.2", en: "Effective height and restraint factor", de: "Knicklänge und Randeinspannung", inputs: &["h_m", "t_m", "sides", "l_m", "bearing_m", "stiff_len_m", "stiff_t_m"], eval: m10 },
    Entry { id: "M-11", part: "EN 1996-1-1", clause: "5.5.1.4", en: "Slenderness", de: "Schlankheit", inputs: &["hef_m", "t_m", "t1_m", "t2_m", "simplified"], eval: m11 },
    Entry { id: "M-12", part: "EN 1996-1-1", clause: "6.1.2.2", en: "Eccentricity and Φ at top and bottom", de: "Ausmitte und Φ oben und unten", inputs: &["t_m", "hef_m", "fd_pa", "n_top_n", "n_bot_n", "m_top_nm", "m_bot_nm", "ehe_m"], eval: m12 },
    Entry { id: "M-13", part: "EN 1996-1-1", clause: "6.1.2.2", en: "Mid-height Φm with creep (Annex G)", de: "Φm in Wandmitte mit Kriechen (Anhang G)", inputs: &["t_m", "hef_m", "fd_pa", "n_mid_n", "m_mid_nm", "m_hm_nm", "phi_inf", "k_e", "n_top_n", "n_bot_n", "m_top_nm", "m_bot_nm"], eval: m13 },
    Entry { id: "M-14", part: "EN 1996-1-1", clause: "6.1.3", en: "Concentrated loads", de: "Teilflächenpressung", inputs: &["ab_m2", "a1_m", "hc_m", "t_m", "lefm_m", "fd_pa", "ned_c_n"], eval: m14 },
    Entry { id: "M-15", part: "EN 1996-1-1", clause: "5.5, 6.1", en: "Piers and lintels", de: "Pfeiler und Stürze", inputs: &["pier_l_m", "t_m", "n_pier_n", "fd_pa", "opening_w_m", "density"], eval: m15 },
    Entry { id: "M-16", part: "EN 1996-1-1", clause: "6.1.2", en: "Vertical load with simultaneous wind", de: "Vertikallast mit gleichzeitigem Wind", inputs: &["t_m", "hef_m", "fd_pa", "n_mid_n", "w_ed_pa", "h_m", "phi_inf"], eval: m16 },
];



fn m10(c: &Case) -> R {
    let (h, t) = (c.n("h_m")?, c.n("t_m")?);
    let sides = c.n("sides")? as u8;
    let l = c.o("l_m", f64::INFINITY);
    let rho2 = c.opt("rho2").unwrap_or(if c.o("bearing_m", 0.0) >= 2.0 * t / 3.0 { 0.75 } else { 1.0 });
    let rho = rho_n(sides, rho2, h, if l.is_finite() { l } else { 1e9 });
    let hef = rho * h;
    let mut out = vec![
        Outcome::info(QuantityKind::Dimensionless, "rho-n", "restraint factor ρ_n", "Randeinspannungsfaktor ρ_n", rho, format!("sides={sides}, ρ₂={rho2:.2} ⇒ ρ_n={rho:.4}")).basis(&["ndp.en1996.m10.rho2", "ndp.en1996.m11.beta-floor"]),
        Outcome::info(QuantityKind::Length, "hef", "effective height h_ef = ρ_n h", "Knicklänge h_ef = ρ_n h", hef, format!("h_ef={hef:.4} m")).basis(&["ndp.en1996.m10.rho2"]),
    ];
    if let Some(len) = c.opt("stiff_len_m") {
        out.push(Outcome::limit(QuantityKind::Length, "stiffener-length", "stiffening wall length ≥ h/5", "Länge aussteifende Wand ≥ h/5", h / 5.0, len, format!("{len:.3} m against h/5={:.3} m", h / 5.0)).basis(&["ndp.en1996.m10.stiffener"]).fix("stiff_len_m", QuantityKind::Length, len, ceil_to(h / 5.0, 1e-3), true));
    }
    if let Some(st) = c.opt("stiff_t_m") {
        let need = (0.3 * t).max(0.085);
        out.push(Outcome::limit(QuantityKind::Length, "stiffener-thickness", "stiffening wall thickness ≥ max(0.3 t, 85 mm)", "Dicke aussteifende Wand ≥ max(0,3 t; 85 mm)", need, st, format!("{st:.3} m against {need:.3} m")).basis(&["ndp.en1996.m10.stiffener"]).fix("stiff_t_m", QuantityKind::Length, st, ceil_to(need, 1e-3), true));
    }
    Ok(out)
}

fn m11(c: &Case) -> R {
    let hef = c.n("hef_m")?;
    let t_ef = match (c.opt("t1_m"), c.opt("t2_m")) {
        (Some(t1), Some(t2)) => (t1.powi(3) + t2.powi(3)).cbrt(),
        _ => c.n("t_m")?,
    };
    let limit = if c.o("simplified", 0.0) > 0.0 { 21.0 } else { 27.0 };
    let lambda = hef / t_ef;
    Ok(vec![Outcome::limit(QuantityKind::Dimensionless, "slenderness", &format!("λ = h_ef/t_ef ≤ {limit:.0}"), &format!("λ = h_ef/t_ef ≤ {limit:.0}"), lambda, limit, format!("t_ef={t_ef:.4} m ⇒ λ={lambda:.3}")).basis(&["ndp.en1996.m11.limit"]).fix("t_m", QuantityKind::Length, t_ef, ceil_to(hef / limit, 1e-3), true)])
}

/// 📉 Eccentricity-based capacity per metre at one section: e, Φ_i and N_Rd.
fn station(t: f64, hef: f64, fd: f64, n: f64, m: f64, ehe: f64) -> (f64, f64, f64) {
    let e = (m / n.max(1e-9) + ehe + hef / 450.0).abs().max(0.05 * t);
    let phi = (1.0 - 2.0 * e / t).clamp(0.0, 1.0);
    (e, phi, phi * t * fd)
}

fn m12(c: &Case) -> R {
    let (t, hef, fd) = (c.n("t_m")?, c.n("hef_m")?, c.f_d_pa()?);
    let ehe = c.o("ehe_m", 0.0);
    let mut out = Vec::new();
    for (name, key_n, key_m, en, de) in [("top", "n_top_n", "m_top_nm", "top", "oben"), ("bottom", "n_bot_n", "m_bot_nm", "bottom", "unten")] {
        let n = c.n(key_n)?;
        let (e, phi, n_rd) = station(t, hef, fd, n, c.o(key_m, 0.0), ehe);
        let key: &'static str = if name == "top" { "n-top" } else { "n-bottom" };
        out.push(Outcome::limit(QuantityKind::Force, key, &format!("N_Ed,i ≤ Φ_i t f_d at the {en}"), &format!("N_Ed,i ≤ Φ_i t f_d {de}"), n, n_rd.max(1e-9), format!("e_i={e:.5} m, Φ_i={phi:.4}, N_Rd={n_rd:.0} N/m")).basis(&["ndp.en1996.m12.einit"]).fix("t_m", QuantityKind::Length, t, solve_threshold(t, 3.0, 1e-3, |x| n <= station(x, hef, fd, n, c.o(key_m, 0.0), ehe).2).unwrap_or(3.0), true));
    }
    Ok(out)
}

/// 📉 Mid-height state per metre: e_mk, creep eccentricity e_k and N_Rd,m.
fn mid_state(c: &Case, t: f64) -> Result<(f64, f64, f64, f64, f64), String> {
    let (hef, fd) = (c.n("hef_m")?, c.f_d_pa()?);
    let n = c.n("n_mid_n")?;
    let phi_inf = c.o("phi_inf", 1.5);
    let e_top = c.o("m_top_nm", 0.0) / c.o("n_top_n", n).max(1e-9);
    let e_bot = c.o("m_bot_nm", 0.0) / c.o("n_bot_n", n).max(1e-9);
    let em = (0.5 * (e_top.abs() + e_bot.abs()) + c.o("m_mid_nm", 0.0) / n.max(1e-9) + c.o("m_hm_nm", 0.0) / n.max(1e-9) + hef / 450.0).abs();
    let ek = 0.002 * phi_inf * (hef / t) * (t * em).sqrt();
    let emk = (em + ek).max(0.05 * t);
    let phi = phi_m(hef / t, emk, t, c.o("k_e", k_e(c.annex)));
    Ok((em, ek, emk, phi, phi * t * fd))
}

fn m13(c: &Case) -> R {
    let t = c.n("t_m")?;
    let n = c.n("n_mid_n")?;
    let (em, ek, emk, phi, n_rd) = mid_state(c, t)?;
    let a1 = 1.0 - 2.0 * emk / t;
    let mut out = vec![
        Outcome::info(QuantityKind::Length, "emk", "e_mk = e_m + e_k ≥ 0.05 t", "e_mk = e_m + e_k ≥ 0,05 t", emk, format!("e_m={em:.5} m, e_k={ek:.5} m, A1={a1:.4}")).basis(&["ndp.en1996.m12.einit"]),
        Outcome::info(QuantityKind::Dimensionless, "phi-m", "Φ_m = A1 exp(−u²/2)", "Φ_m = A1 exp(−u²/2)", phi, format!("Φ_m={phi:.4}")),
        Outcome::limit(QuantityKind::Force, "n-mid", "N_Ed,m ≤ Φ_m t f_d", "N_Ed,m ≤ Φ_m t f_d", n, n_rd.max(1e-9), format!("N_Rd,m={n_rd:.0} N/m")).fix("t_m", QuantityKind::Length, t, solve_threshold(t, 3.0, 1e-3, |x| mid_state(c, x).map(|s| n <= s.4).unwrap_or(false)).unwrap_or(3.0), true),
    ];
    if let (Ok(nt), Ok(nb)) = (c.n("n_top_n"), c.n("n_bot_n")) {
        let fd = c.f_d_pa()?;
        let hef = c.n("hef_m")?;
        let top = station(t, hef, fd, nt, c.o("m_top_nm", 0.0), c.o("ehe_m", 0.0));
        let bot = station(t, hef, fd, nb, c.o("m_bot_nm", 0.0), c.o("ehe_m", 0.0));
        let candidates = [("top", nt, top.2), ("mid", n, n_rd), ("bottom", nb, bot.2)];
        let (name, nn, rd) = candidates.iter().copied().max_by(|a, b| (a.1 / a.2.max(1e-9)).total_cmp(&(b.1 / b.2.max(1e-9)))).unwrap_or(candidates[0]);
        out.push(Outcome::info(QuantityKind::Dimensionless, "phi-top", "Φ at the top", "Φ oben", top.1, format!("Φ_top={:.4}", top.1)));
        out.push(Outcome::info(QuantityKind::Dimensionless, "phi-bottom", "Φ at the bottom", "Φ unten", bot.1, format!("Φ_bottom={:.4}", bot.1)));
        out.push(Outcome::limit(QuantityKind::Force, "governing", &format!("governing section N_Ed ≤ N_Rd ({name})"), &format!("maßgebender Schnitt N_Ed ≤ N_Rd ({name})"), nn, rd.max(1e-9), format!("governing section: {name}")));
    }
    Ok(out)
}

fn m14(c: &Case) -> R {
    let (ab, a1, hc, t) = (c.n("ab_m2")?, c.o("a1_m", 0.0), c.n("hc_m")?, c.n("t_m")?);
    let a_ef = c.o("lefm_m", 1.0) * t;
    let fd = c.f_d_pa()?;
    let beta = concentrated_beta(a1, hc, ab, a_ef);
    let n_rdc = beta * ab * fd;
    let ned = c.n("ned_c_n")?;
    let mut out = vec![
        Outcome::info(QuantityKind::Dimensionless, "beta", "enhancement factor β", "Erhöhungsfaktor β", beta, format!("β={beta:.4}, A_ef={a_ef:.4} m²")).basis(&["ndp.en1996.m14.beta"]),
        Outcome::limit(QuantityKind::Force, "n-rdc", "N_Edc ≤ β A_b f_d", "N_Edc ≤ β A_b f_d", ned, n_rdc.max(1e-9), format!("N_Rdc={n_rdc:.0} N")).basis(&["ndp.en1996.m14.beta"]).fix("ab_m2", QuantityKind::Area, ab, solve_threshold(ab, 0.45 * a_ef, 1e-4, |x| ned <= concentrated_beta(a1, hc, x, a_ef) * x * fd).unwrap_or(0.45 * a_ef), true),
        Outcome::limit(QuantityKind::Area, "area-ratio", "A_b ≤ 0.45 A_ef", "A_b ≤ 0,45 A_ef", ab, 0.45 * a_ef, format!("A_b={ab:.4} m² against {:.4} m²", 0.45 * a_ef)).basis(&["ndp.en1996.m14.beta"]).fix("ab_m2", QuantityKind::Area, ab, 0.45 * a_ef, false),
    ];
    if c.de() {
        out.push(Outcome::limit(QuantityKind::Stress, "sigma-1", "σ₁ = F/A_b ≤ 1.5 f_d", "σ₁ = F/A_b ≤ 1,5 f_d", ned / ab, 1.5 * fd, "NA-DE bearing stress limit".into()).basis(&["ndp.en1996.m14.beta"]).fix("ab_m2", QuantityKind::Area, ab, ceil_to(ned / (1.5 * fd), 1e-4), true));
    }
    Ok(out)
}

fn m15(c: &Case) -> R {
    let (l, t, fd) = (c.n("pier_l_m")?, c.n("t_m")?, c.f_d_pa()?);
    let n = c.n("n_pier_n")?;
    let phi = c.o("phi", 1.0 - 2.0 * 0.05);
    let n_rd = phi * l * t * fd;
    let mut out = vec![
        Outcome::limit(QuantityKind::Length, "pier-length", "pier length ≥ 0.5 m", "Pfeilerlänge ≥ 0,5 m", 0.5, l, format!("{l:.3} m")).basis(&["ndp.en1996.m15.lintel"]).fix("pier_l_m", QuantityKind::Length, l, 0.5, true),
        Outcome::limit(QuantityKind::Force, "pier-compression", "pier N_Ed ≤ Φ l t f_d", "Pfeiler N_Ed ≤ Φ l t f_d", n, n_rd.max(1e-9), format!("N_Rd={n_rd:.0} N, σ={:.3} MPa", n / (l * t) / 1e6)).fix("pier_l_m", QuantityKind::Length, l, ceil_to(n / (phi * t * fd), 1e-3), true),
    ];
    if let Some(w) = c.opt("opening_w_m") {
        let g = c.o("density", 1800.0) * 9.81 * t;
        let load = g * w * w / 4.0;
        out.push(Outcome::info(QuantityKind::Force, "lintel-load", "lintel load from 45° triangle", "Sturzlast aus 45°-Dreieck", load, format!("W=ρ g t l²/4={load:.0} N, M=W l/6={:.0} N·m", load * w / 6.0)).basis(&["ndp.en1996.m15.lintel"]));
        out.push(Outcome::info(QuantityKind::Moment, "lintel-moment", "lintel moment M = W l/6", "Sturzmoment M = W l/6", load * w / 6.0, "simply supported triangular load".into()).basis(&["ndp.en1996.m15.lintel"]));
    }
    Ok(out)
}

fn m16(c: &Case) -> R {
    let (t, h) = (c.n("t_m")?, c.n("h_m")?);
    let n = c.n("n_mid_n")?;
    let m_wind = c.n("w_ed_pa")? * h * h / 8.0;
    let with = c.clone().with_n("m_hm_nm", m_wind).with_n("phi_inf", c.o("phi_inf", 1.5));
    let (_, _, emk, phi, n_rd) = mid_state(&with, t)?;
    let (_, _, _, phi0, _) = mid_state(&c.clone().with_n("m_hm_nm", 0.0), t)?;
    Ok(vec![
        Outcome::info(QuantityKind::Dimensionless, "phi-reduction", "Φ_m with / without wind", "Φ_m mit / ohne Wind", phi / phi0.max(1e-9), format!("Φ_m={phi:.4} vs {phi0:.4}, e_mk={emk:.5} m")),
        Outcome::limit(QuantityKind::Force, "n-mid-wind", "N_Ed,m ≤ Φ_m t f_d with wind eccentricity", "N_Ed,m ≤ Φ_m t f_d mit Windausmitte", n, n_rd.max(1e-9), format!("M_wind={m_wind:.0} N·m/m, N_Rd={n_rd:.0} N/m")).fix("t_m", QuantityKind::Length, t, solve_threshold(t, 3.0, 1e-3, |x| mid_state(&with, x).map(|s| n <= s.4).unwrap_or(false)).unwrap_or(3.0), true),
    ])
}
