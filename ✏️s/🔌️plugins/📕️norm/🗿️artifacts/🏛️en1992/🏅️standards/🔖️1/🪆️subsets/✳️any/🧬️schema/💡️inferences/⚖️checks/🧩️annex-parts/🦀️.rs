//! 🧩️ Annexes of EN 1992-1-1 (F in-plane stress reinforcement, J surface reinforcement, frame corners and corbels), warping torsion §6.3.3 and EN 1992-2 Annexes KK (time-dependent behaviour) and LL (shell elements, sandwich model) as catalogue entries AF-01, AJ-01 … AJ-03, T-05, AK-01 and AL-01. Tension is positive; N, mm, N/mm². See [`super`].

use super::*;

/// 🧭 Reinforcement stresses and strut stress of an in-plane stress state (Annex F): returns (ρ_x, ρ_y, σ_c) as tension-positive N/mm² on the layer thickness for the strut angle cot θ; a negative direction is dropped and the other one recomputed.
pub fn plane_stress_design(sx: f64, sy: f64, txy: f64, cot: f64) -> (f64, f64, f64) {
    let t = txy.abs();
    let (rx, ry, sc) = (sx + t * cot, sy + t / cot, t * (cot + 1.0 / cot));
    if rx < 0.0 {
        return (0.0, (sy + t * t / (-sx)).max(0.0), -sx * (1.0 + t * t / (sx * sx)));
    }
    if ry < 0.0 {
        return ((sx + t * t / (-sy)).max(0.0), 0.0, -sy * (1.0 + t * t / (sy * sy)));
    }
    (rx, ry, sc)
}

/// 📐 Annex F.1: reinforcement of a membrane element from σ_x, σ_y, τ_xy for a chosen strut inclination.
pub fn af01(c: &Case) -> R {
    let (sx, sy, txy, t, fck) = (c.n("sx")?, c.n("sy")?, c.n("txy")?, c.pos("t")?, c.pos("fck")?);
    let cot = c.o("cot_theta", 1.0).clamp(0.4, 2.5);
    let fyd = f_yd(c.annex, c.o("fyk", 500.0), false);
    let (rx, ry, sc) = plane_stress_design(sx, sy, txy, cot);
    let cap = nu1(c.annex, fck) * f_cd(c.annex, fck, false);
    Ok(vec![
        Outcome::num("as_x", "Required A_s,x per mm (mm²/mm)", "Erforderliches A_s,x je mm (mm²/mm)", rx * t / fyd, "f_tdx = t (σ_x + |τ| cot θ), A_s = f_td/f_yd".into()),
        Outcome::num("as_y", "Required A_s,y per mm (mm²/mm)", "Erforderliches A_s,y je mm (mm²/mm)", ry * t / fyd, "f_tdy = t (σ_y + |τ| tan θ)".into()),
        Outcome::mpa("sigma_c", "Strut stress σ_cd", "Druckstrebenspannung σ_cd", sc, format!("|τ| (cot θ + tan θ), cot θ = {}", f(cot))),
        Outcome::stress("util", "Strut utilisation σ_cd/(ν f_cd)", "Druckstrebenausnutzung σ_cd/(ν f_cd)", sc, cap, "Annex F.1: σ_cd ≤ ν f_cd".into()),
    ])
}

/// 🛡 Annex J.1: surface reinforcement of members with bars or tendons of large diameter, A_s,surf,min = k A_ct,ext in both directions.
pub fn aj01(c: &Case) -> R {
    let (hct, b) = (c.pos("h_ct")?, c.o("b", 1000.0));
    let k = ndp(c.annex, "ndp.en1992.surface-reinf");
    let req = k * hct * b;
    let mut v = vec![Outcome::mm2("as_surf_min", "Minimum surface reinforcement A_s,surf,min", "Mindest-Oberflächenbewehrung A_s,surf,min", req, format!("{} A_ct,ext, A_ct,ext = {} mm²", f(k), f(hct * b)))];
    if let Some(p) = c.opt("as_surf") { v.push(Outcome::area_min("util_as", "Surface reinforcement provided", "Vorhandene Oberflächenbewehrung", p, req, "A_s,surf ≥ A_s,surf,min".into())); }
    if let Some(s) = c.opt("s_surf") { v.push(Outcome::length("util_s", "Spacing of surface reinforcement", "Abstand der Oberflächenbewehrung", s, ndp(c.annex, "ndp.en1992.surface-smax"), "s ≤ 150 mm".into())); }
    Ok(v)
}

/// 📐 Annex J.2: tie force, bar force and mandrel diameter of a frame corner; opening moments with ρ above the threshold additionally need links.
pub fn aj02(c: &Case) -> R {
    let (m, d, b, fck) = (c.pos("m_ed")?, c.pos("d")?, c.pos("b")?, c.pos("fck")?);
    let fyd = f_yd(c.annex, c.o("fyk", 500.0), false);
    let z = c.o("z", 0.9 * d);
    let ft = m / z;
    let asr = ft / fyd;
    let mut v = vec![
        Outcome::n("f_t", "Tie force F_td = M_Ed/z", "Zugkraft F_td = M_Ed/z", ft, format!("z = {}", f(z))),
        Outcome::mm2("as_req", "Required corner tie A_s", "Erforderliche Eckbewehrung A_s", asr, "F_td/f_yd".into()),
    ];
    if let (Some(phi), Some(ab)) = (c.opt("phi"), c.opt("a_b")) {
        let fbt = ft / c.o("n_bars", 1.0).max(1.0);
        let pm = fbt * (1.0 / ab + 1.0 / (2.0 * phi)) / f_cd(c.annex, fck, false);
        v.push(Outcome::mm("phi_m_min", "Minimum mandrel diameter φ_m,min", "Mindest-Biegerollendurchmesser φ_m,min", pm, "F_bt (1/a_b + 1/(2φ))/f_cd, 8.3(3)".into()));
        if let Some(p) = c.opt("phi_m") { v.push(Outcome::length_min("util_phi_m", "Mandrel diameter provided", "Vorhandener Biegerollendurchmesser", p, pm, "φ_m ≥ φ_m,min".into())); }
    }
    if c.so("mode", "closing") == "opening" {
        let prov = c.o("as_provided", asr);
        let rho = prov / (b * d);
        let need = if rho > ndp(c.annex, "ndp.en1992.corner-rho-open") { ndp(c.annex, "ndp.en1992.corner-links-share") * prov } else { 0.0 };
        v.push(Outcome::num("rho", "Reinforcement ratio ρ", "Bewehrungsgrad ρ", rho, "A_s/(b d)".into()));
        v.push(Outcome::mm2("a_links_req", "Required closed links A_sw at the opening corner", "Erforderliche Bügel A_sw an der Öffnungsecke", need, "share of A_s when ρ above the threshold".into()).steps_de("Anteil von A_s, wenn ρ über dem Schwellenwert liegt"));
        if let Some(a) = c.opt("a_links") { v.push(Outcome::area_min("util_links", "Corner links provided", "Vorhandene Eckbügel", a, need, "A_links ≥ share A_s".into()).steps_de("A_links ≥ Anteil A_s")); }
    }
    Ok(v)
}

/// 🧱 Annex J.3 and §6.5: corbel with the strut-and-tie tie force, links k1 A_s,main and strut stress.
pub fn aj03(c: &Case) -> R {
    let (fed, ac, z0, fck) = (c.pos("f_ed")?, c.pos("a_c")?, c.pos("z0")?, c.pos("fck")?);
    let (hed, zh) = (c.o("h_ed", 0.0), c.o("z_h", 0.0));
    let fyd = f_yd(c.annex, c.o("fyk", 500.0), false);
    let ft = fed * ac / z0 + hed * (1.0 + zh / z0);
    let asm = ft / fyd;
    let k1 = ndp(c.annex, "ndp.en1992.corbel-k1");
    let cstrut = fed * (ac * ac + z0 * z0).sqrt() / z0;
    let mut v = vec![
        Outcome::n("f_td", "Main tie force F_td", "Zugbandkraft F_td", ft, "F_Ed a_c/z_0 + H_Ed (1 + z_H/z_0)".into()),
        Outcome::mm2("as_main", "Required main reinforcement A_s,main", "Erforderliche Hauptbewehrung A_s,main", asm, "F_td/f_yd".into()),
        Outcome::mm2("a_links", "Required closed horizontal links A_sk", "Erforderliche geschlossene Horizontalbügel A_sk", k1 * asm, format!("k1 A_s,main, k1 = {}", f(k1))),
        Outcome::n("c_strut", "Strut force C", "Druckstrebenkraft C", cstrut, "F_Ed/sin θ, tan θ = z_0/a_c".into()),
        Outcome::num("theta", "Strut inclination θ (rad)", "Strebenneigung θ (rad)", (z0 / ac).atan(), "atan(z_0/a_c)".into()),
    ];
    if let (Some(b), Some(w)) = (c.opt("b"), c.opt("w_strut")) {
        let cap = ndp(c.annex, "ndp.en1992.strut-cracked") * (1.0 - fck / 250.0) * f_cd(c.annex, fck, false);
        v.push(Outcome::stress("util_strut", "Strut utilisation", "Druckstrebenausnutzung", cstrut / (b * w), cap, "C/(b w) ≤ k ν' f_cd".into()));
    }
    Ok(v)
}

/// 🌀 §6.3.3: warping torsion of an open thin-walled section; the bimoment B becomes the lateral moment M_f = B/h_s of a flange designed as a rectangular beam.
pub fn t05(c: &Case) -> R {
    let (bim, hs, tf, df, fck) = (c.pos("bimoment")?, c.pos("h_s")?, c.pos("t_f")?, c.pos("d_f")?, c.pos("fck")?);
    let fyk = c.o("fyk", 500.0);
    let mf = bim / hs;
    let (lam, eta) = lambda_eta(fck);
    let fcd = f_cd(c.annex, fck, false);
    let mu = mf / (eta * fcd * tf * df * df);
    let mut v = vec![Outcome::nmm("m_f", "Flange lateral moment M_f = B/h_s", "Gurtquermoment M_f = B/h_s", mf, "I-section decomposition".into()).steps_de("Zerlegung des I-Querschnitts")];
    if let Some(vw) = c.opt("t_w") { v.push(Outcome::n("v_f", "Flange shear V_f = T_w/h_s", "Gurtquerkraft V_f = T_w/h_s", vw / hs, "T_w = dB/dx".into())); }
    if mu > 0.5 { return Err("flange section cannot carry the lateral moment".into()); }
    let u = 1.0 - (1.0 - 2.0 * mu).sqrt();
    let asr = eta * fcd * tf * df * u / f_yd(c.annex, fyk, false);
    let eps_yd = f_yd(c.annex, fyk, false) / 200_000.0;
    let ecu = conc(fck).eps_cu3 / 1000.0;
    v.push(Outcome::mm2("as_req", "Required flange reinforcement per face", "Erforderliche Gurtbewehrung", asr, format!("μ = {}, rectangular block", f(mu))));
    v.push(Outcome::ratio("util_x", "Neutral axis depth x/d", "Druckzonenhöhe x/d", u / lam, ecu / (ecu + eps_yd), "x/d ≤ ε_cu/(ε_cu + ε_yd)".into()));
    if let Some(p) = c.opt("as_provided") { v.push(Outcome::area_min("util_as", "Flange reinforcement provided", "Vorhandene Gurtbewehrung", p, asr, "A_s ≥ A_s,req".into())); }
    Ok(v)
}

/// ⏳ EN 1992-2 Annex KK: age-adjusted effective modulus E_c,adj = E_cm/(1 + χ φ), creep strain and Dischinger redistribution (1 − e^−φ) of a system change.
pub fn ak01(c: &Case) -> R {
    let (fck, phi) = (c.pos("fck")?, c.pos("phi")?);
    let chi = c.o("chi", ndp(c.annex, "ndp.en1992.kk-ageing-chi"));
    let ecm = conc(fck).ecm;
    let xi = 1.0 - (-phi).exp();
    let mut v = vec![
        Outcome::mpa("e_adj", "Age-adjusted modulus E_c,adj", "Altersangepasster Modul E_c,adj", ecm / (1.0 + chi * phi), format!("E_cm/(1 + χ φ), χ = {}", f(chi))),
        Outcome::num("xi", "Redistribution factor 1 − e^−φ", "Umlagerungsfaktor 1 − e^−φ", xi, "Dischinger".into()),
    ];
    if let Some(s) = c.opt("sigma_c") { v.push(Outcome::num("eps_cc", "Creep strain ε_cc = φ σ_c/E_cm", "Kriechdehnung ε_cc = φ σ_c/E_cm", phi * s / ecm, "EN 1992-1-1 3.1.4".into())); }
    if let (Some(m1), Some(m2)) = (c.opt("m1"), c.opt("m2")) { v.push(Outcome::nmm("m_final", "Final moment M1 + (M2 − M1)(1 − e^−φ)", "Endmoment M1 + (M2 − M1)(1 − e^−φ)", m1 + (m2 - m1) * xi, "static system change".into()).steps_de("Änderung des statischen Systems")); }
    Ok(v)
}

/// 🐚 EN 1992-2 Annex LL: sandwich model of a shell element; the outer layers carry n/2 ± m/z and are designed as membranes (Annex F). Positive m puts the bottom layer in tension.
pub fn al01(c: &Case) -> R {
    let (nx, ny, nxy, mx, my, mxy) = (c.o("nx", 0.0), c.o("ny", 0.0), c.o("nxy", 0.0), c.o("mx", 0.0), c.o("my", 0.0), c.o("mxy", 0.0));
    let (h, a, fck) = (c.pos("h")?, c.pos("a")?, c.pos("fck")?);
    let z = h - 2.0 * a;
    if z <= 0.0 { return Err("lever arm h − 2a must be positive".into()); }
    let t = 2.0 * a;
    let fyd = f_yd(c.annex, c.o("fyk", 500.0), false);
    let cot = c.o("cot_theta", 1.0).clamp(0.4, 2.5);
    let cap = nu1(c.annex, fck) * f_cd(c.annex, fck, false);
    let mut v = vec![];
    for (name, en, de, sg) in [("bot", "bottom", "unten", 1.0), ("top", "top", "oben", -1.0)] {
        let (lx, ly, lxy) = (nx / 2.0 + sg * mx / z, ny / 2.0 + sg * my / z, nxy / 2.0 + sg * mxy / z);
        let (rx, ry, sc) = plane_stress_design(lx / t, ly / t, lxy / t, cot);
        v.push(Outcome::num(&format!("as_x_{name}"), &format!("A_s,x {en} layer (mm²/mm)"), &format!("A_s,x Lage {de} (mm²/mm)"), rx * t / fyd, format!("layer forces {} {} {} N/mm", f(lx), f(ly), f(lxy))));
        v.push(Outcome::num(&format!("as_y_{name}"), &format!("A_s,y {en} layer (mm²/mm)"), &format!("A_s,y Lage {de} (mm²/mm)"), ry * t / fyd, "Annex F per layer".into()).steps_de("Anhang F je Lage"));
        v.push(Outcome::stress(&format!("util_{name}"), &format!("Strut utilisation {en} layer"), &format!("Druckstrebenausnutzung Lage {de}"), sc, cap, "σ_cd ≤ ν f_cd".into()));
    }
    Ok(v)
}

/// 🗂 Catalogue rows of the annex parts.
pub const ENTRIES: &[Entry] = &[
    entry_of("AF-01", "DIN EN 1992-1-1", "Annex F", "Reinforcement for in-plane stress", "Bewehrung für ebenen Spannungszustand", &["sx", "sy", "txy", "t", "fck", "fyk", "cot_theta"], &[], af01),
    entry_of("AJ-01", "DIN EN 1992-1-1", "Annex J.1", "Surface reinforcement", "Oberflächenbewehrung", &["h_ct", "b", "as_surf", "s_surf"], &["ndp.en1992.surface-reinf", "ndp.en1992.surface-smax"], aj01),
    entry_of("AJ-02", "DIN EN 1992-1-1", "Annex J.2", "Frame corners", "Rahmenecken", &["m_ed", "d", "b", "fck", "fyk", "z", "phi", "a_b", "n_bars", "phi_m", "mode", "as_provided", "a_links"], &["ndp.en1992.corner-rho-open", "ndp.en1992.corner-links-share"], aj02),
    entry_of("AJ-03", "DIN EN 1992-1-1", "Annex J.3", "Corbels", "Konsolen", &["f_ed", "h_ed", "a_c", "z0", "z_h", "fck", "fyk", "b", "w_strut"], &["ndp.en1992.strut-cracked", "ndp.en1992.corbel-k1"], aj03),
    entry_of("T-05", "DIN EN 1992-1-1", "6.3.3", "Warping torsion", "Wölbkrafttorsion", &["bimoment", "t_w", "h_s", "t_f", "d_f", "fck", "fyk", "as_provided"], &[], t05),
    entry_of("AK-01", "DIN EN 1992-2", "Annex KK", "Time-dependent behaviour", "Zeitabhängiges Verhalten", &["fck", "phi", "chi", "sigma_c", "m1", "m2"], &["ndp.en1992.kk-ageing-chi"], ak01),
    entry_of("AL-01", "DIN EN 1992-2", "Annex LL", "Shell elements (sandwich model)", "Schalenelemente (Sandwichmodell)", &["nx", "ny", "nxy", "mx", "my", "mxy", "h", "a", "fck", "fyk", "cot_theta"], &[], al01),
];
