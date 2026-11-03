//! 🪟️ Serviceability limit states of EN 1992-1-1 chapter 7 as catalogue entries SL-01 … SL-09: stress limitation, cracked-section stresses, crack control, crack width, span-depth ratio and deflection by curvature integration. See [`super`].

use super::*;

/// 📐 Linear-elastic cracked rectangular section with tension steel `a_s` at `d` and compression steel `a_s2` at `d2`, loaded by M and N (N compression positive, about the section mid-height); returns (x, I_II, σ_c, σ_s, σ_s2) in mm, mm⁴, MPa, `None` if the section is uncracked (x ≥ h). The rectangle is only the input form of the catalogue case: the state is [`Sec::elastic`], the same solver as the member checks.
pub fn cracked_state(b: f64, h: f64, d: f64, a_s: f64, d2: f64, a_s2: f64, alpha_e: f64, m: f64, n: f64) -> Option<(f64, f64, f64, f64, f64)> {
    let mut s = Sec::rect(b, h, conc(30.0), 0.0, 0.0);
    s.c.ecm = 30000.0;
    s.es = alpha_e * s.c.ecm;
    s.bars = vec![Bar { y: 0.0, z: h / 2.0 - d, a: a_s }, Bar { y: 0.0, z: h / 2.0 - d2, a: a_s2 }];
    let e = s.elastic(m, n, false).filter(|e| e.x < h - 1e-6)?;
    Some((e.x, e.i_ii, e.sigma_c, e.sigma_s, e.sigma_s2))
}

pub fn sec_in(c: &Case) -> Result<(f64, f64, f64, f64, f64, f64, f64), String> {
    let (b, h, d, a_s) = (c.pos("b")?, c.pos("h")?, c.pos("d")?, c.pos("as")?);
    let g = conc(c.pos("fck")?);
    let ec = match c.opt("phi_inf") { Some(p) => g.ecm / (1.0 + p), None => g.ecm };
    Ok((b, h, d, a_s, c.o("es", 200000.0) / ec, c.o("d2", 0.1 * h), c.o("as2", 0.0)))
}

pub fn sl01(c: &Case) -> R {
    let (b, h, d, a_s, ae, d2, a2) = sec_in(c)?;
    let fck = c.pos("fck")?;
    let (m, n) = (c.n("m_char")?, c.o("n_ed", 0.0));
    let (sc, ss) = match cracked_state(b, h, d, a_s, d2, a2, ae, m, n) {
        Some(s) => (s.2, s.3),
        None => { let w = b * h * h / 6.0; (n / (b * h) + m.abs() / w, ae * (n / (b * h) + m.abs() * (d - h / 2.0) / (b * h.powi(3) / 12.0))) }
    };
    let mut v = vec![Outcome::stress("util_char", "Concrete stress, characteristic", "Betondruckspannung, charakteristisch", sc, c.ndp("ndp.en1992.sigma-c-char") * fck, format!("σ_c = {}", f(sc)))];
    if let Some(mq) = c.opt("m_qp") {
        let s = cracked_state(b, h, d, a_s, d2, a2, ae, mq, n).map(|s| s.2).unwrap_or(n / (b * h) + mq.abs() / (b * h * h / 6.0));
        v.push(Outcome::stress("util_qp", "Concrete stress, quasi-permanent (linear creep)", "Betondruckspannung, quasi-ständig (lineares Kriechen)", s, c.ndp("ndp.en1992.sigma-c-qp") * fck, format!("σ_c,qp = {}", f(s))));
    }
    v.push(Outcome::mpa("sigma_s", "Steel stress", "Stahlspannung", ss, "cracked/uncracked elastic".into()).steps_de("gerissen/ungerissen elastisch"));
    Ok(v)
}

pub fn sl02(c: &Case) -> R {
    let mut v = Vec::new();
    if let (Some(ss), Some(fyk)) = (c.opt("sigma_s"), c.opt("fyk")) {
        let k = if c.o("imposed_only", 0.0) > 0.0 { c.ndp("ndp.en1992.sigma-s-imposed") } else { c.ndp("ndp.en1992.sigma-s-char") };
        v.push(Outcome::stress("util_s", "Reinforcement stress limitation", "Begrenzung der Betonstahlspannung", ss, k * fyk, format!("k = {}", f(k))));
    }
    if let (Some(sp), Some(fpk), Some(fp01k)) = (c.opt("sigma_p"), c.opt("fpk"), c.opt("fp01k")) {
        let stage = c.so("stage", "after_losses");
        let lim = match stage {
            "jacking" => (c.ndp("ndp.en1992.p-k1") * fpk).min(c.ndp("ndp.en1992.p-k2") * fp01k),
            "transfer" => (c.ndp("ndp.en1992.p-k7") * fpk).min(c.ndp("ndp.en1992.p-k8") * fp01k),
            _ => c.ndp("ndp.en1992.sigma-p-qp") * fpk,
        };
        v.push(Outcome::stress("util_p", "Prestressing steel stress limit", "Begrenzung der Spannstahlspannung", sp, lim, format!("stage {stage}")));
    }
    if v.is_empty() { return Err("missing input sigma_s with fyk or sigma_p with fpk and fp01k".into()); }
    Ok(v)
}

pub fn sl03(c: &Case) -> R {
    let (b, h, d, a_s, ae, d2, a2) = sec_in(c)?;
    let (m, n) = (c.n("m_ed")?, c.o("n_ed", 0.0));
    let g = conc(c.pos("fck")?);
    let w = b * h * h / 6.0;
    let sct = m.abs() / w - n / (b * h);
    match cracked_state(b, h, d, a_s, d2, a2, ae, m, n) {
        Some((x, i2, sc, ss, ss2)) if sct >= g.fctm => Ok(vec![
            Outcome::mm("x", "Neutral axis depth x", "Nulllinientiefe x", x, "cracked state II".into()).steps_de("gerissener Zustand II"),
            Outcome::info("i_ii", "Second moment I_II (mm⁴)", "Flächenmoment I_II (mm⁴)", QuantityKind::Dimensionless, i2, "b x³/3 + α_e A_s (d−x)²".into()),
            Outcome::mpa("sigma_c", "Concrete stress", "Betonspannung", sc, "M x/I_II".into()),
            Outcome::mpa("sigma_s", "Tension steel stress", "Zugstahlspannung", ss, "α_e M (d−x)/I_II".into()),
            Outcome::mpa("sigma_s2", "Compression steel stress", "Druckstahlspannung", ss2, "α_e M (x−d')/I_II".into()),
        ]),
        _ => {
            let a_t = b * h + ae * (a_s + a2);
            let zc = (b * h * h / 2.0 + ae * (a_s * d + a2 * d2)) / a_t;
            let i1 = b * h.powi(3) / 12.0 + b * h * (zc - h / 2.0).powi(2) + ae * (a_s * (d - zc).powi(2) + a2 * (zc - d2).powi(2));
            Ok(vec![
                Outcome::mm("x", "Neutral axis depth x (state I centroid)", "Schwerpunktabstand x (Zustand I)", zc, "uncracked".into()),
                Outcome::mpa("sigma_c", "Concrete stress (uncracked)", "Betonspannung (ungerissen)", n / a_t + m * zc / i1, "state I".into()),
                Outcome::mpa("sigma_s", "Tension steel stress (uncracked)", "Zugstahlspannung (ungerissen)", ae * (n / a_t + m * (d - zc) / i1).abs(), "state I".into()),
            ])
        }
    }
}

pub fn sl04(c: &Case) -> R {
    let (fck, b, h) = (c.pos("fck")?, c.pos("b")?, c.pos("h")?);
    let g = conc(fck);
    let mut fct = c.o("fct_eff", g.fctm);
    if c.de() && c.o("thin_early_age", 0.0) > 0.0 { fct *= c.ndp("ndp.en1992.fct-eff-factor-slab"); }
    let k = interp(&[(300.0, 1.0), (800.0, 0.65)], c.o("h_member", h));
    let hstar = h.min(1000.0);
    let n = c.o("n_ed", 0.0);
    let sigma_c = n / (b * h);
    let k1 = if n >= 0.0 { 1.5 } else { 2.0 * hstar / (3.0 * h) };
    let kc = if c.so("part", "web") == "flange" { (0.9 * c.pos("f_cr")? / (c.pos("act")? * fct)).max(0.5) } else if c.o("pure_tension", 0.0) > 0.0 { 1.0 } else { (0.4 * (1.0 - sigma_c / (k1 * (h / hstar) * fct))).min(1.0) };
    let act = c.opt("act").unwrap_or(if c.o("pure_tension", 0.0) > 0.0 { b * h } else { b * h / 2.0 });
    let ss = c.o("sigma_s_allowed", c.o("fyk", 500.0));
    let asmin = kc * k * fct * act / ss;
    let mut v = vec![
        Outcome::mm2("as_min", "Minimum crack-control reinforcement A_s,min", "Mindestbewehrung zur Rissbreitenbegrenzung A_s,min", asmin, format!("k_c = {}, k = {}, f_ct,eff = {}", f(kc), f(k), f(fct))),
        Outcome::num("k_c", "Stress distribution coefficient k_c", "Beiwert der Spannungsverteilung k_c", kc, "Eq. 7.2".into()),
        Outcome::num("k", "Non-uniform stress coefficient k", "Beiwert nichtlinearer Spannungen k", k, "Figure 7.1".into()),
    ];
    if let Some(p) = c.opt("as_provided") { v.push(Outcome::area_min("util", "Provided crack-control reinforcement", "Vorhandene Bewehrung Rissbreitenbegrenzung", p, asmin, "A_s ≥ A_s,min".into())); }
    Ok(v)
}

/// 📚 Table 7.2N maximum bar diameter φ* (mm) for the stress and w_k class (0.4, 0.3, 0.2); `None` outside the table.
pub fn table_72n(sigma_s: f64, wk: f64) -> Option<f64> {
    const T: [(f64, [f64; 3]); 8] = [(160.0, [40.0, 32.0, 25.0]), (200.0, [32.0, 25.0, 16.0]), (240.0, [20.0, 16.0, 12.0]), (280.0, [16.0, 12.0, 8.0]), (320.0, [12.0, 10.0, 6.0]), (360.0, [10.0, 8.0, 5.0]), (400.0, [8.0, 6.0, 4.0]), (450.0, [6.0, 5.0, 0.0])];
    let col = if wk >= 0.35 { 0 } else if wk >= 0.25 { 1 } else { 2 };
    let row = T.iter().find(|r| sigma_s <= r.0)?;
    if row.1[col] > 0.0 { Some(row.1[col]) } else { None }
}

/// 📚 Table 7.3N maximum bar spacing (mm).
pub fn table_73n(sigma_s: f64, wk: f64) -> Option<f64> {
    const T: [(f64, [f64; 3]); 6] = [(160.0, [300.0, 300.0, 200.0]), (200.0, [300.0, 250.0, 150.0]), (240.0, [250.0, 200.0, 100.0]), (280.0, [200.0, 150.0, 50.0]), (320.0, [150.0, 100.0, 0.0]), (360.0, [100.0, 50.0, 0.0])];
    let col = if wk >= 0.35 { 0 } else if wk >= 0.25 { 1 } else { 2 };
    let row = T.iter().find(|r| sigma_s <= r.0)?;
    if row.1[col] > 0.0 { Some(row.1[col]) } else { None }
}

pub fn sl05(c: &Case) -> R {
    let (ss, wk) = (c.pos("sigma_s")?, c.pos("wk_lim")?);
    let mut v = Vec::new();
    let Some(mut phi_max) = table_72n(ss, wk) else { return Err("σ_s outside Tables 7.2N and 7.3N".into()) };
    if c.o("restraint", 0.0) > 0.0 {
        let (fct, h, d, hcr) = (c.o("fct_eff", 2.9), c.pos("h")?, c.pos("d")?, c.o("hcr", c.pos("h")?));
        let kc = c.o("kc", 0.4);
        phi_max *= fct / 2.9 * if c.so("action", "bending") == "bending" { kc * hcr / (2.0 * (h - d)) } else { hcr / (8.0 * (h - d)) };
    }
    let smax = table_73n(ss, wk).unwrap_or(0.0);
    v.push(Outcome::mm("phi_max", "Maximum bar diameter φ", "Größter Stabdurchmesser φ", phi_max, "Table 7.2N".into()));
    v.push(Outcome::mm("s_max", "Maximum bar spacing", "Größter Stababstand", smax, "Table 7.3N".into()));
    if let Some(phi) = c.opt("phi") { v.push(Outcome::length("util_phi", "Bar diameter", "Stabdurchmesser", phi, phi_max, "φ ≤ φ_max".into())); }
    if let (Some(s), true) = (c.opt("s"), smax > 0.0) { v.push(Outcome::length("util_s", "Bar spacing", "Stababstand", s, smax, "s ≤ s_max".into())); }
    Ok(v)
}

/// 🪟 Crack width quantities (x, h_c,eff, ρ_p,eff, s_r,max, ε_sm−ε_cm, w_k).
#[derive(Clone, Copy, Debug)]
pub struct Crack {
    pub x: f64,
    pub hc_eff: f64,
    pub rho_p: f64,
    pub sr_max: f64,
    pub eps: f64,
    pub wk: f64,
    pub sigma_s: f64,
}

/// 🧱 Section of a crack-width case: the polygon and bar lists when given, else the rectangle b × h with the tension steel `as` at `d` and the compression steel `as2` at `d2`; the concrete modulus `ec` and the ratio `ae` fix the elastic state.
fn crack_section(c: &Case, ec: f64, ae: f64) -> Result<Sec, String> {
    let mut s = if c.l("poly_y").is_ok() || c.l("bar_y").is_ok() {
        section_of(c)?
    } else {
        let (b, h, d) = (c.pos("b")?, c.pos("h")?, c.pos("d")?);
        let mut s = Sec::rect(b, h, conc(c.pos("fck")?), 0.0, 0.0);
        s.bars = vec![Bar { y: 0.0, z: h / 2.0 - d, a: c.pos("as")? }];
        if c.o("as2", 0.0) > 0.0 {
            s.bars.push(Bar { y: 0.0, z: h / 2.0 - c.o("d2", 0.1 * h), a: c.o("as2", 0.0) });
        }
        s
    };
    s.c.ecm = ec;
    s.es = ae * ec;
    Ok(s)
}

/// 🧮 Strain difference ε_sm − ε_cm of Eq. 7.9 with the modular ratio α_e given: max((σ_s − k_t f_ct,eff/ρ_p,eff (1 + α_e ρ_p,eff))/E_s, 0.6 σ_s/E_s); any consistent unit system.
pub fn strain_difference_alpha(sigma_s: f64, e_s: f64, alpha_e: f64, rho_p_eff: f64, k_t: f64, f_ct_eff: f64) -> f64 {
    ((sigma_s - k_t * f_ct_eff / rho_p_eff * (1.0 + alpha_e * rho_p_eff)) / e_s).max(0.6 * sigma_s / e_s)
}

/// 🧮 Strain difference ε_sm − ε_cm of Eq. 7.9 with α_e = E_s/E_cm; returns (ε_sm − ε_cm, α_e). See [`strain_difference_alpha`].
pub fn strain_difference(sigma_s: f64, e_s: f64, e_cm: f64, rho_p_eff: f64, k_t: f64, f_ct_eff: f64) -> (f64, f64) {
    let alpha_e = e_s / e_cm;
    (strain_difference_alpha(sigma_s, e_s, alpha_e, rho_p_eff, k_t, f_ct_eff), alpha_e)
}

/// 📏 Maximum crack spacing s_r,max = k_3 c + k_1 k_2 k_4 φ/ρ_p,eff (Eq. 7.11); any consistent length unit.
pub fn sr_max_closed(c: f64, phi: f64, rho_p_eff: f64, k1: f64, k2: f64, k3: f64, k4: f64) -> f64 {
    k3 * c + k1 * k2 * k4 * phi / rho_p_eff
}

/// 📏 Crack width w_k = s_r,max (ε_sm − ε_cm) (Eq. 7.8).
pub fn crack_width_value(sr_max: f64, strain_difference: f64) -> f64 {
    sr_max * strain_difference
}

/// 📏 Crack width per 7.3.4 (mm, N, MPa) on the general section: the neutral axis and the steel stress from the elastic state II, A_c,eff of 7.3.2(3) from the outline.
pub fn crack_width(c: &Case) -> Result<Crack, String> {
    let (cov, phi) = (c.pos("c")?, c.pos("phi")?);
    let g = conc(c.pos("fck")?);
    let es = c.o("es", 200000.0);
    let long = c.so("duration", "long") == "long";
    let kt = if long { c.ndp("ndp.en1992.kt-long") } else { c.ndp("ndp.en1992.kt-short") };
    let ec = if long { c.opt("phi_inf").map(|p| g.ecm / (1.0 + p)).unwrap_or(g.ecm) } else { g.ecm };
    let ae = c.opt("alpha_e").unwrap_or(es / ec);
    let sec = crack_section(c, ec, ae)?;
    let gross = sec.gross_props();
    let h = gross.z_top - gross.z_bot;
    let (x, ss) = match (c.opt("m_qp"), c.opt("sigma_s")) {
        (_, Some(s)) => (c.opt("x").or_else(|| c.opt("m_qp").and_then(|m| sec.elastic(m, c.o("n_ed", 0.0), false)).map(|e| e.x)).unwrap_or(h / 2.0), s),
        (Some(m), None) => match sec.elastic(m, c.o("n_ed", 0.0), false).filter(|e| e.x < h - 1e-6) { Some(e) => (e.x, e.sigma_s), None => return Err("section is uncracked: w_k = 0".into()) },
        _ => return Err("missing input m_qp or sigma_s".into()),
    };
    let (hc, ac_eff) = sec.crack_zone(x);
    let a_s = sec.tension_area();
    let ap = c.o("ap", 0.0);
    let xi1 = if ap > 0.0 { (c.o("xi", 0.5) * phi / c.o("phi_p", phi)).sqrt() } else { 0.0 };
    let rho_p = (a_s + xi1 * xi1 * ap) / ac_eff.max(1e-9);
    let fct = c.o("fct_eff", g.fctm);
    let ae_eq = c.opt("alpha_e").unwrap_or(es / g.ecm);
    let eps = strain_difference_alpha(ss, es, ae_eq, rho_p, kt, fct);
    let k1 = c.o("k1", 0.8);
    let (e1, e2) = (c.o("eps1", 1.0), c.o("eps2", 0.0));
    let k2 = if c.so("action", "bending") == "tension" { 1.0 } else if c.o("eps1", 0.0) > 0.0 { (e1 + e2) / (2.0 * e1) } else { 0.5 };
    let mut sr = sr_max_closed(cov, phi, rho_p, k1, k2, c.ndp("ndp.en1992.k3-crack"), c.ndp("ndp.en1992.k4-crack"));
    if let Some(s) = c.opt("s") { if s > 5.0 * (cov + phi / 2.0) { sr = 1.3 * (h - x); } }
    if let (Some(sy), Some(sz), Some(th)) = (c.opt("sr_y"), c.opt("sr_z"), c.opt("theta")) { sr = 1.0 / (th.cos() / sy + th.sin() / sz); }
    Ok(Crack { x, hc_eff: hc, rho_p, sr_max: sr, eps, wk: crack_width_value(sr, eps), sigma_s: ss })
}

/// 📏 Crack width limit w_k,lim (mm) of Table 7.1N / NA-DE.
pub fn wk_limit(annex: AnnexChoice, exposure: &str, prestressed_bonded: bool) -> f64 {
    if prestressed_bonded { ndp(annex, "ndp.en1992.wk-lim-prestress") } else if matches!(exposure, "X0" | "XC1") { ndp(annex, "ndp.en1992.wk-lim-xc1") } else { ndp(annex, "ndp.en1992.wk-lim") }
}

pub fn sl06(c: &Case) -> R {
    let k = match crack_width(c) {
        Ok(k) => k,
        Err(e) if e.starts_with("section is uncracked") => return Ok(vec![Outcome::mm("wk", "Crack width w_k (uncracked)", "Rissbreite w_k (ungerissen)", 0.0, e.clone()).steps_de(semio_s_plugin_norm_compliance::explain_reason(&e).de)]),
        Err(e) => return Err(e),
    };
    let lim = c.opt("wk_lim").unwrap_or_else(|| wk_limit(c.annex, c.so("exposure", "XC2"), c.o("prestressed_bonded", 0.0) > 0.0));
    Ok(vec![
        Outcome::length("util", "Crack width", "Rissbreite", k.wk, lim, format!("w_k = s_r,max (ε_sm−ε_cm), limit {}", f(lim))),
        Outcome::mm("sr_max", "Maximum crack spacing s_r,max", "Maximaler Rissabstand s_r,max", k.sr_max, "k3 c + k1 k2 k4 φ/ρ_p,eff".into()),
        Outcome::num("eps_diff", "Strain difference ε_sm − ε_cm", "Dehnungsdifferenz ε_sm − ε_cm", k.eps, "Eq. 7.9".into()),
        Outcome::mm("x", "Neutral axis depth x", "Nulllinientiefe x", k.x, "cracked state".into()).steps_de("gerissener Zustand"),
        Outcome::mm("hc_eff", "Effective tension height h_c,eff", "Wirksame Zugzonenhöhe h_c,eff", k.hc_eff, "min(2.5(h−d), (h−x)/3, h/2)".into()),
        Outcome::num("rho_p_eff", "Effective reinforcement ratio ρ_p,eff", "Wirksamer Bewehrungsgrad ρ_p,eff", k.rho_p, "A_s/A_c,eff".into()),
        Outcome::mpa("sigma_s", "Steel stress σ_s", "Stahlspannung σ_s", k.sigma_s, "quasi-permanent".into()).steps_de("quasi-ständig"),
    ])
}

/// 📉 Limiting span-to-depth ratio of Eq. 7.16 with the NA-DE cap.
pub fn ld_limit(annex: AnnexChoice, fck: f64, rho: f64, rho_prime: f64, k: f64) -> f64 {
    let s = fck.sqrt();
    let rho0 = s * 1e-3;
    let basic = if rho <= rho0 { 11.0 + 1.5 * s * rho0 / rho + 3.2 * s * (rho0 / rho - 1.0).powf(1.5) } else { 11.0 + 1.5 * s * rho0 / (rho - rho_prime) + s / 12.0 * (rho_prime / rho0).sqrt() };
    let lim = k * basic;
    if annex == AnnexChoice::De { lim.min(k * ndp(annex, "ndp.en1992.ld-de-cap")) } else { lim }
}

/// 🧮 Factor K of Table 7.4N.
pub fn k_system(system: &str) -> f64 {
    match system { "end_span" => 1.3, "interior_span" => 1.5, "flat_slab" => 1.2, "cantilever" => 0.4, _ => 1.0 }
}

pub fn sl07(c: &Case) -> R {
    let (fck, span, d, a_prov) = (c.pos("fck")?, c.pos("span")?, c.pos("d")?, c.pos("as")?);
    let b = c.pos("b")?;
    let a_req = c.o("as_req", a_prov);
    let rho = a_req / (b * d);
    let rho_p = c.o("as2", 0.0) / (b * d);
    let k = k_system(c.so("system", "simply"));
    let fyk = c.o("fyk", 500.0);
    let mut lim = ld_limit(c.annex, fck, rho, rho_p, k);
    if c.o("beff_over_bw", 1.0) > 3.0 { lim *= 0.8; }
    if span > 7000.0 && c.o("brittle_partitions", 0.0) > 0.0 && c.so("system", "simply") != "flat_slab" { lim *= 7000.0 / span; }
    if span > 8500.0 && c.so("system", "simply") == "flat_slab" { lim *= 8500.0 / span; }
    lim *= (500.0 * a_prov / (fyk * a_req)).min(1.5);
    let sl = span / d;
    Ok(vec![Outcome::ratio("util", "Span-to-depth ratio", "Biegeschlankheit", sl, lim, format!("(l/d)_lim = {}, K = {}, ρ = {}", f(lim), f(k), f(rho))), Outcome::num("ld_limit", "Limiting l/d", "Grenzschlankheit l/d", lim, "Eq. 7.16".into())])
}

pub fn sl08(c: &Case) -> R {
    let (b, h, d, a_s, ae, d2, a2) = sec_in(c)?;
    let (l, q) = (c.pos("span")?, c.pos("q")?);
    let g = conc(c.pos("fck")?);
    let ec = match c.opt("phi_inf") { Some(p) => g.ecm / (1.0 + p), None => g.ecm };
    let beta = c.o("beta", 0.5);
    let fct = c.o("fct_eff", g.fctm);
    let ecs = c.o("eps_cs", 0.0);
    let a_t = b * h + ae * (a_s + a2);
    let zc = (b * h * h / 2.0 + ae * (a_s * d + a2 * d2)) / a_t;
    let i1 = b * h.powi(3) / 12.0 + b * h * (zc - h / 2.0).powi(2) + ae * (a_s * (d - zc).powi(2) + a2 * (zc - d2).powi(2));
    let mcr = fct * i1 / (h - zc);
    let kappa_at = |m: f64| -> f64 {
        let k1 = m / (ec * i1);
        let (k2, i2, s2) = match cracked_state(b, h, d, a_s, d2, a2, ae, m, 0.0) { Some(s) => (m / (ec * s.1), s.1, ae * a_s * (d - s.0)), None => (k1, i1, ae * a_s * (d - zc)) };
        let zeta = if m.abs() > mcr { 1.0 - beta * (mcr / m.abs()).powi(2) } else { 0.0 };
        let cs = |s: f64, i: f64| if i > 0.0 { ecs * s / i } else { 0.0 };
        zeta * (k2 + cs(s2, i2)) + (1.0 - zeta) * (k1 + cs(ae * a_s * (d - zc), i1))
    };
    let n = 400;
    let dx = l / n as f64;
    let mut w = 0.0;
    let mut profile = Vec::new();
    for i in 0..n {
        let x = (i as f64 + 0.5) * dx;
        let m = q * x * (l - x) / 2.0;
        let mbar = if x < l / 2.0 { x / 2.0 } else { (l - x) / 2.0 };
        w += kappa_at(m) * mbar * dx;
    }
    for j in 0..=10 {
        let xs = l * j as f64 / 10.0;
        let mut wj = 0.0;
        for i in 0..n {
            let x = (i as f64 + 0.5) * dx;
            let m = q * x * (l - x) / 2.0;
            let mbar = if x < xs { x * (l - xs) / l } else { xs * (l - x) / l };
            wj += kappa_at(m) * mbar * dx;
        }
        profile.push(Outcome::mm(&format!("w_{j}"), &format!("Deflection at {}0 % of the span", j), &format!("Durchbiegung bei {}0 % der Spannweite", j), wj, "∫ κ m̄ dx".into()));
    }
    let (qp, after) = (c.ndp("ndp.en1992.defl-limit-qp"), c.ndp("ndp.en1992.defl-limit-after"));
    let mut v = vec![Outcome::length("util", &format!("Deflection against l/{qp:.0}"), &format!("Durchbiegung gegen l/{qp:.0}"), w, l / qp, format!("w = {} mm", f(w))), Outcome::mm("w_max", "Maximum deflection", "Größte Durchbiegung", w, "midspan".into())];
    if let Some(wb) = c.opt("w_before") { v.push(Outcome::length("util_after", &format!("Deflection after construction against l/{after:.0}"), &format!("Durchbiegung nach Herstellung gegen l/{after:.0}"), w - wb, l / after, "w − w_before".into())); }
    v.extend(profile);
    Ok(v)
}

pub fn sl09(c: &Case) -> R {
    let (b, h, p, e, m) = (c.pos("b")?, c.pos("h")?, c.pos("p")?, c.n("e_p")?, c.n("m_qp")?);
    let w = b * h * h / 6.0;
    let bot = -p / (b * h) - p * e / w + m / w;
    let fctm = conc(c.pos("fck")?).fctm;
    let fl = ((1.6 - h / 1000.0) * fctm).max(fctm);
    let mut v = vec![Outcome::mpa("sigma_bottom", "Bottom fibre stress (tension positive)", "Randspannung unten (Zug positiv)", bot, "−P/A − P e/W + M/W".into()), Outcome::stress("util_cracking", "Tension against flexural tensile strength", "Zugspannung gegen Biegezugfestigkeit", bot.max(0.0), fl, "σ ≤ f_ctm,fl".into())];
    if c.o("decompression", 0.0) > 0.0 { v.push(Outcome::stress("util_decompression", "Decompression (σ ≤ 0)", "Dekompression (σ ≤ 0)", bot.max(0.0), 1e-9, "XD/XS, bonded tendons".into()).steps_de("XD/XS, Spannglieder mit Verbund")); }
    Ok(v)
}

/// 🗂 Entries of this domain.
pub const ENTRIES: &[Entry] = &[
    entry_of("SL-01", "DIN EN 1992-1-1", "7.2", "Concrete stress limitation", "Begrenzung der Betondruckspannung", &["fck", "b", "h", "d", "as", "d2", "as2", "m_char", "m_qp", "n_ed", "phi_inf", "es"], &["ndp.en1992.sigma-c-char", "ndp.en1992.sigma-c-qp"], sl01),
    entry_of("SL-02", "DIN EN 1992-1-1", "7.2(5)", "Steel stress limitation", "Begrenzung der Stahlspannung", &["sigma_s", "fyk", "imposed_only", "sigma_p", "fpk", "fp01k", "stage"], &["ndp.en1992.sigma-s-char", "ndp.en1992.sigma-s-imposed", "ndp.en1992.sigma-p-qp", "ndp.en1992.p-k1", "ndp.en1992.p-k2", "ndp.en1992.p-k7", "ndp.en1992.p-k8"], sl02),
    entry_of("SL-03", "DIN EN 1992-1-1", "7.1", "Cracked section stresses", "Spannungen im Zustand II", &["fck", "b", "h", "d", "as", "d2", "as2", "m_ed", "n_ed", "phi_inf", "es"], &[], sl03),
    entry_of("S-24", "DIN EN 1992-1-1", "7.1", "State determination cracked or uncracked", "Zustandsbestimmung gerissen oder ungerissen", &["fck", "b", "h", "d", "as", "d2", "as2", "m_ed", "n_ed", "phi_inf", "es"], &[], sl03),
    entry_of("S-30", "DIN EN 1992-1-1", "7.2", "Stress state of the cracked section", "Spannungszustand Zustand II", &["fck", "b", "h", "d", "as", "d2", "as2", "m_ed", "n_ed", "phi_inf", "es"], &[], sl03),
    entry_of("S-31", "DIN EN 1992-1-1", "7.2", "Stress state, characteristic combination", "Spannungszustand, charakteristische Kombination", &["fck", "b", "h", "d", "as", "d2", "as2", "m_ed", "n_ed", "phi_inf", "es"], &[], sl03),
    entry_of("S-34", "DIN EN 1992-1-1", "7.2", "Stress state, quasi-permanent combination", "Spannungszustand, quasi-ständige Kombination", &["fck", "b", "h", "d", "as", "d2", "as2", "m_ed", "n_ed", "phi_inf", "es"], &[], sl03),
    entry_of("SL-04", "DIN EN 1992-1-1", "7.3.2", "Minimum reinforcement for crack control", "Mindestbewehrung Rissbreitenbegrenzung", &["fck", "b", "h", "h_member", "n_ed", "fct_eff", "act", "pure_tension", "part", "f_cr", "sigma_s_allowed", "fyk", "as_provided", "thin_early_age"], &["ndp.en1992.fct-eff-factor-slab"], sl04),
    entry_of("SL-05", "DIN EN 1992-1-1", "7.3.3", "Crack control without calculation", "Rissbreitenbegrenzung ohne direkten Nachweis", &["sigma_s", "wk_lim", "phi", "s", "restraint", "fct_eff", "h", "d", "hcr", "kc", "action"], &[], sl05),
    entry_of("SL-06", "DIN EN 1992-1-1", "7.3.4", "Crack width calculation", "Rissbreitenberechnung", &["fck", "b", "h", "d", "as", "c", "phi", "m_qp", "sigma_s", "n_ed", "duration", "alpha_e", "s", "exposure", "wk_lim", "k1", "action", "ap", "xi", "phi_p"], &["ndp.en1992.kt-long", "ndp.en1992.kt-short", "ndp.en1992.k3-crack", "ndp.en1992.k4-crack", "ndp.en1992.wk-lim", "ndp.en1992.wk-lim-xc1", "ndp.en1992.wk-lim-prestress"], sl06),
    entry_of("SL-07", "DIN EN 1992-1-1", "7.4.2", "Span-to-depth limit", "Verformungsbegrenzung durch Biegeschlankheit", &["fck", "b", "d", "as", "as2", "span", "system", "as_req", "fyk", "beff_over_bw", "brittle_partitions"], &["ndp.en1992.ld-de-cap"], sl07),
    entry_of("SL-08", "DIN EN 1992-1-1", "7.4.3", "Deflection by calculation", "Verformungsberechnung", &["fck", "b", "h", "d", "as", "span", "q", "phi_inf", "beta", "eps_cs", "w_before"], &["ndp.en1992.defl-limit-qp", "ndp.en1992.defl-limit-after"], sl08),
    entry_of("SL-09", "DIN EN 1992-1-1", "7.1", "Decompression and cracking of prestressed members", "Dekompression und Rissbildung vorgespannter Bauteile", &["fck", "b", "h", "p", "e_p", "m_qp", "decompression"], &[], sl09),
];
