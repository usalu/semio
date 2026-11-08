//! 🔗️ Anchorage, laps and detailing rules of EN 1992-1-1 chapters 8 and 9 as catalogue entries B-01 … B-09 and DT-01 … DT-14. See [`super`].

use super::*;

/// 🪢 Ultimate bond stress f_bd (MPa): 2.25 η1 η2 f_ctd with f_ctd capped at the C60/75 value.
pub fn f_bd(annex: AnnexChoice, fck: f64, phi: f64, good_bond: bool) -> f64 {
    let eta1 = if good_bond { 1.0 } else { 0.7 };
    let eta2 = if phi <= 32.0 { 1.0 } else { (132.0 - phi) / 100.0 };
    f_bd_from(annex, eta1, eta2, f_ctd(annex, fck.min(60.0)))
}

/// 🪢 Ultimate bond stress f_bd = 2.25 η1 η2 f_ctd (Eq. 8.2) from the bond factors and the design tensile strength; any consistent stress unit.
pub fn f_bd_from(annex: AnnexChoice, eta1: f64, eta2: f64, f_ctd: f64) -> f64 {
    ndp(annex, "ndp.en1992.fbd-eta") * eta1 * eta2 * f_ctd
}

/// 📏 Basic required anchorage length l_b,rqd (mm).
pub fn l_b_rqd(phi: f64, sigma_sd: f64, fbd: f64) -> f64 {
    phi / 4.0 * sigma_sd / fbd
}

pub fn bond(c: &Case) -> Result<(f64, f64, f64), String> {
    let (fck, phi) = (c.pos("fck")?, c.pos("phi")?);
    let fbd = c.opt("fbd").unwrap_or_else(|| f_bd(c.annex, fck, phi, c.so("bond", "good") == "good"));
    let sigma = c.opt("sigma_sd").unwrap_or_else(|| f_yd(c.annex, c.o("fyk", 500.0), false));
    Ok((phi, fbd, sigma))
}

pub fn b01(c: &Case) -> R {
    let (fck, phi) = (c.pos("fck")?, c.pos("phi")?);
    let good = c.so("bond", "good") == "good";
    Ok(vec![Outcome::mpa("fbd", "Ultimate bond stress f_bd", "Verbundspannung f_bd", f_bd(c.annex, fck, phi, good), format!("η1 = {}, f_ctd = {}", if good { 1.0 } else { 0.7 }, f(f_ctd(c.annex, fck.min(60.0)))))])
}

pub fn b02(c: &Case) -> R {
    let (phi, fbd, sigma) = bond(c)?;
    Ok(vec![Outcome::mm("lb_rqd", "Basic anchorage length l_b,rqd", "Grundmaß der Verankerungslänge l_b,rqd", l_b_rqd(phi, sigma, fbd), format!("σ_sd = {}, f_bd = {}", f(sigma), f(fbd)))])
}

pub fn b03(c: &Case) -> R {
    let (phi, fbd, sigma) = bond(c)?;
    let lb = l_b_rqd(phi, sigma, fbd);
    let hook = c.so("shape", "straight") != "straight";
    let compression = c.so("loading", "tension") == "compression";
    let cd = c.o("cd", 3.0 * phi);
    let a1 = if hook && cd > 3.0 * phi { 0.7 } else { 1.0 };
    let a2 = if compression { 1.0 } else if hook { (1.0 - 0.15 * (cd - 3.0 * phi) / phi).clamp(0.7, 1.0) } else { (1.0 - 0.15 * (cd - phi) / phi).clamp(0.7, 1.0) };
    let lam = (c.o("sum_ast", 0.0) - c.o("sum_ast_min", 0.0)) / c.o("as", 1.0);
    let a3 = if compression { 1.0 } else { (1.0 - c.o("k_conf", 0.0) * lam).clamp(0.7, 1.0) };
    let a4 = if c.o("welded", 0.0) > 0.0 { 0.7 } else { 1.0 };
    let a5 = (1.0 - 0.04 * c.o("p_transverse", 0.0)).clamp(0.7, 1.0);
    let a235 = (a2 * a3 * a5).max(0.7);
    let lbd_raw = a1 * a235 * a4 * lb;
    let lmin = if compression { 0.6 * lb } else { 0.3 * lb }.max(10.0 * phi).max(100.0);
    let lbd = lbd_raw.max(lmin);
    let mut out = vec![
        Outcome::mm("lbd", "Design anchorage length l_bd", "Bemessungswert der Verankerungslänge l_bd", lbd, format!("α1 {} α2 {} α3 {} α4 {} α5 {}", f(a1), f(a2), f(a3), f(a4), f(a5))),
        Outcome::mm("lb_min", "Minimum anchorage length l_b,min", "Mindestverankerungslänge l_b,min", lmin, "max(0.3/0.6 l_b,rqd, 10φ, 100 mm)".into()),
        Outcome::mm("lb_rqd", "Basic anchorage length l_b,rqd", "Grundmaß l_b,rqd", lb, "φ/4 σ_sd/f_bd".into()),
        Outcome::mm("mandrel", "Minimum mandrel diameter", "Mindestbiegerollendurchmesser", phi * if phi <= 16.0 { c.ndp("ndp.en1992.k-mandrel-small") } else { c.ndp("ndp.en1992.k-mandrel-large") }, "8.3(2)".into()),
    ];
    if let Some(av) = c.opt("l_available") { out.push(Outcome::length("util", "Anchorage utilisation", "Verankerungsausnutzung", lbd, av, "l_bd ≤ l_available".into())); }
    Ok(out)
}

pub fn b04(c: &Case) -> R {
    let (phi, fbd, sigma) = bond(c)?;
    let lb = l_b_rqd(phi, sigma, fbd);
    let rho1 = c.o("rho1", 50.0);
    let compression = c.so("loading", "tension") == "compression";
    let a6 = if compression { 1.0 } else { (rho1 / 25.0).sqrt().clamp(1.0, 1.5) };
    let cd = c.o("cd", 3.0 * phi);
    let a2 = if compression { 1.0 } else { (1.0 - 0.15 * (cd - phi) / phi).clamp(0.7, 1.0) };
    let a3 = if compression { 1.0 } else { c.o("alpha3", 1.0).clamp(0.7, 1.0) };
    let a5 = (1.0 - 0.04 * c.o("p_transverse", 0.0)).clamp(0.7, 1.0);
    let a1 = c.o("alpha1", 1.0);
    let l0 = (a1 * a2 * a3 * a5 * a6 * lb).max((0.3 * a6 * lb).max(15.0 * phi).max(200.0));
    let mut out = vec![Outcome::mm("l0", "Lap length l_0", "Übergreifungslänge l_0", l0, format!("α2 {} α6 {}", f(a2), f(a6))), Outcome::mm("l0_min", "Minimum lap length", "Mindestübergreifungslänge", (0.3 * a6 * lb).max(15.0 * phi).max(200.0), "max(0.3 α6 l_b,rqd, 15φ, 200)".into())];
    if let Some(av) = c.opt("l_available") { out.push(Outcome::length("util", "Lap utilisation", "Stoßausnutzung", l0, av, "l_0 ≤ l_provided".into())); }
    if let Some(a) = c.opt("clear_distance") { out.push(Outcome::length_min("clear", "Clear distance of lapped bars", "Lichter Abstand der Stoßstäbe", a, (2.0 * phi).max(20.0), "a ≥ max(2φ, 20 mm)".into())); }
    if let (Some(ast), Some(as_)) = (c.opt("sum_ast"), c.opt("as")) { out.push(Outcome::area_min("transverse", "Transverse reinforcement in the lap", "Querbewehrung im Stoß", ast, as_, "ΣA_st ≥ A_s".into())); }
    Ok(out)
}

pub fn b05(c: &Case) -> R {
    let (phi, nb) = (c.pos("phi")?, c.pos("nb")?);
    let limit = if c.so("situation", "other") == "compression" { 4.0 } else { 3.0 };
    Ok(vec![Outcome::mm("phi_n", "Equivalent diameter φ_n", "Vergleichsdurchmesser φ_n", (phi * nb.sqrt()).min(55.0), "φ √n_b ≤ 55 mm".into()), Outcome::ratio("nb", "Number of bars in the bundle", "Anzahl Stäbe im Bündel", nb, limit, "n_b ≤ 3 (4 in compression and laps)".into()).steps_de("n_b ≤ 3 (4 bei Druck und Stößen)")])
}

pub fn b06(c: &Case) -> R {
    let (phi, act) = (c.pos("phi")?, c.pos("act_ext")?);
    if phi <= 32.0 { return Ok(vec![Outcome::num("applicable", "Large-diameter rules not applicable (φ ≤ 32 mm)", "Regeln für große Durchmesser nicht anwendbar (φ ≤ 32 mm)", 0.0, "8.8".into())]); }
    let mut v = vec![Outcome::mm2("as_surf", "Surface reinforcement A_s,surf", "Oberflächenbewehrung A_s,surf", 0.01 * act, "0.01 A_ct,ext".into()), Outcome::num("eta2", "η2", "η2", (132.0 - phi) / 100.0, "bond factor".into()).steps_de("Verbundfaktor")];
    if let Some(p) = c.opt("as_surf_provided") { v.push(Outcome::area_min("util", "Surface reinforcement provided", "Vorhandene Oberflächenbewehrung", p, 0.01 * act, "≥ 0.01 A_ct,ext".into())); }
    Ok(v)
}

pub fn b07(c: &Case) -> R {
    let (fck, phi, sigma) = (c.pos("fck")?, c.pos("phi")?, c.pos("sigma_pm0")?);
    let strand = c.so("tendon", "strand") == "strand";
    let eta_p1 = if strand { 3.2 } else { 2.7 };
    let a1 = if c.so("release", "gradual") == "gradual" { 1.0 } else { 1.25 };
    let a2 = if strand { 0.19 } else { 0.25 };
    let age = c.o("t", 28.0);
    let fctd = c.opt("fctd_t").unwrap_or_else(|| f_ctd(c.annex, fck) * beta_cc(age, cement_s(c.so("cement", "N"))).powf(if age < 28.0 { 1.0 } else { 2.0 / 3.0 }));
    let eta1 = if c.so("bond", "good") == "good" { 1.0 } else { 0.7 };
    let fbpt = eta_p1 * eta1 * fctd;
    let lpt = a1 * a2 * phi * sigma / fbpt;
    let d = c.o("d", 0.0);
    Ok(vec![
        Outcome::mm("lpt", "Transmission length l_pt", "Übertragungslänge l_pt", lpt, format!("f_bpt = {}", f(fbpt))),
        Outcome::mm("lpt1", "Design length l_pt1", "Bemessungslänge l_pt1", 0.8 * lpt, "0.8 l_pt".into()),
        Outcome::mm("lpt2", "Design length l_pt2", "Bemessungslänge l_pt2", 1.2 * lpt, "1.2 l_pt".into()),
        Outcome::mm("ldisp", "Dispersion length l_disp", "Ausbreitungslänge l_disp", (lpt * lpt + d * d).sqrt(), "√(l_pt² + d²)".into()),
    ])
}

pub fn b08(c: &Case) -> R {
    let phi = c.pos("phi")?;
    let hook = c.so("shape", "hook_135") != "welded";
    let straight_ext = if hook { (10.0 * phi).max(70.0) } else { 0.0 };
    let bend = if c.so("angle", "135") == "90" { (10.0 * phi).max(70.0) } else { (5.0 * phi).max(50.0) };
    Ok(vec![Outcome::mm("hook_length", "Hook extension length", "Hakenlänge", bend.max(straight_ext.min(bend)), "5φ (135°) / 10φ (90°), at least 50/70 mm".into()).steps_de("5φ (135°) / 10φ (90°), mindestens 50/70 mm"), Outcome::mm("mandrel", "Mandrel diameter", "Biegerollendurchmesser", if phi <= 16.0 { 4.0 } else { 7.0 } * phi, "4φ (φ ≤ 16 mm) / 7φ (φ > 16 mm), Table 8.1N".into()).steps_de("4φ (φ ≤ 16 mm) / 7φ (φ > 16 mm), Tabelle 8.1N")])
}

pub fn b09(c: &Case) -> R {
    let (phi, dg, a) = (c.pos("phi")?, c.o("dg", 16.0), c.pos("clear")?);
    let req = phi.max(dg + 5.0).max(20.0);
    Ok(vec![Outcome::length_min("clear", "Clear bar distance", "Lichter Stababstand", a, req, "max(φ, d_g + 5 mm, 20 mm)".into())])
}

pub fn dt01(c: &Case) -> R {
    let (fck, bt, d, h) = (c.pos("fck")?, c.pos("bt")?, c.pos("d")?, c.pos("h")?);
    let fyk = c.o("fyk", 500.0);
    let amin = (0.26 * conc(fck).fctm / fyk * bt * d).max(0.0013 * bt * d);
    let amax = 0.04 * c.o("ac", bt * h);
    let as_ = c.pos("as")?;
    Ok(vec![Outcome::area_min("as_min", "Minimum tension reinforcement", "Mindestzugbewehrung", as_, amin, format!("A_s,min = {}", f(amin))), Outcome::area("as_max", "Maximum reinforcement", "Höchstbewehrung", as_, amax, "A_s ≤ 0.04 A_c".into())])
}

pub fn dt02(c: &Case) -> R {
    let (ved, z, as_span) = (c.pos("v_ed")?, c.pos("z")?, c.pos("as_span")?);
    let cot = c.o("cot_theta", 2.5);
    let cot_a = c.o("cot_alpha", 0.0);
    let al = z * (cot - cot_a) / 2.0;
    let fed = ved * al / z + c.o("n_ed", 0.0);
    let mut v = vec![Outcome::mm("a_l", "Shift a_l", "Versatzmaß a_l", al, "z (cot θ − cot α)/2".into()), Outcome::n("f_ed", "Support tension F_Ed", "Auflagerzugkraft F_Ed", fed, "V_Ed a_l/z + N_Ed".into())];
    if let Some(p) = c.opt("as_support") { v.push(Outcome::area_min("as_support", "Support reinforcement against 25 % of span steel", "Auflagerbewehrung gegen 25 % der Feldbewehrung", p, 0.25 * as_span, "A_s,sup ≥ 0.25 A_s,span".into())); }
    Ok(v)
}

pub fn dt03(c: &Case) -> R {
    let (fck, bw, d) = (c.pos("fck")?, c.pos("bw")?, c.pos("d")?);
    let alpha = c.o("alpha_deg", 90.0).to_radians();
    let rho_min = if c.de() { 0.16 * conc(fck).fctm / c.o("fyk", 500.0) } else { 0.08 * fck.sqrt() / c.o("fyk", 500.0) };
    let mut v = vec![Outcome::num("rho_w_min", "Minimum shear reinforcement ratio", "Mindestschubbewehrungsgrad", rho_min, "0.16 f_ctm/f_yk".into()), Outcome::mm("sl_max", "Maximum longitudinal spacing", "Größter Längsabstand", 0.75 * d * (1.0 + 1.0 / alpha.tan()), "0.75 d (1 + cot α)".into()), Outcome::mm("st_max", "Maximum transverse spacing", "Größter Querabstand", (0.75 * d).min(600.0), "0.75 d ≤ 600 mm".into())];
    if let (Some(asw), Some(s)) = (c.opt("asw"), c.opt("s")) {
        v.push(Outcome::ratio_min("rho_w", "Shear reinforcement ratio", "Schubbewehrungsgrad", asw / (s * bw * alpha.sin()), rho_min, "ρ_w ≥ ρ_w,min".into()));
        v.push(Outcome::length("s_util", "Link spacing", "Bügelabstand", s, 0.75 * d * (1.0 + 1.0 / alpha.tan()), "s ≤ s_l,max".into()));
    }
    Ok(v)
}

pub fn dt04(c: &Case) -> R {
    let (uk, b, d) = (c.pos("u")?, c.pos("b")?, c.pos("d")?);
    let lim = (uk / 8.0).min(0.75 * d).min(b);
    let mut v = vec![Outcome::mm("s_max", "Largest torsion link spacing", "Größter Torsionsbügelabstand", lim, "min(u/8, s_l,max, b)".into())];
    if let Some(s) = c.opt("s") { v.push(Outcome::length("util", "Torsion link spacing", "Torsionsbügelabstand", s, lim, "s ≤ s_max".into())); }
    Ok(v)
}

pub fn dt05(c: &Case) -> R {
    let act = c.pos("act_ext")?;
    let req = 0.01 * act;
    let mut v = vec![Outcome::mm2("as_surf", "Surface reinforcement A_s,surf,min", "Oberflächenbewehrung A_s,surf,min", req, "0.01 A_ct,ext".into())];
    if let Some(p) = c.opt("as_provided") { v.push(Outcome::area_min("util", "Provided surface reinforcement", "Vorhandene Oberflächenbewehrung", p, req, "A_s ≥ A_s,surf,min".into())); }
    if let Some(s) = c.opt("s") { v.push(Outcome::length("spacing", "Surface bar spacing", "Abstand Oberflächenbewehrung", s, 150.0, "s ≤ 150 mm".into())); }
    Ok(v)
}

pub fn dt06(c: &Case) -> R {
    let (fck, h, d) = (c.pos("fck")?, c.pos("h")?, c.pos("d")?);
    let fyk = c.o("fyk", 500.0);
    let amin = (0.26 * conc(fck).fctm / fyk * 1000.0 * d).max(0.0013 * 1000.0 * d);
    let smax = (3.0 * h).min(400.0);
    let sec = (3.5 * h).min(450.0);
    let conc_load = (2.0 * h).min(250.0);
    let de_cap = if c.de() && h >= 250.0 { Some(c.ndp("ndp.en1992.slab-smax-cracked")) } else { None };
    let mut v = vec![
        Outcome::mm2("as_min", "Minimum reinforcement per metre", "Mindestbewehrung je Meter", amin, "9.2.1.1".into()),
        Outcome::mm("s_max", "Largest principal spacing", "Größter Hauptbewehrungsabstand", de_cap.map(|c| c.min(smax)).unwrap_or(smax), "min(3h, 400 mm); DE 250 mm in cracked zones".into()).steps_de("min(3h, 400 mm); DE 250 mm in gerissenen Bereichen"),
        Outcome::mm("s_max_secondary", "Largest secondary spacing", "Größter Verteilerabstand", sec, "min(3.5h, 450 mm)".into()),
        Outcome::mm("s_max_concentrated", "Largest spacing at concentrated loads", "Größter Abstand bei Einzellasten", conc_load, "min(2h, 250 mm)".into()),
    ];
    if let (Some(a), Some(s)) = (c.opt("as_main"), c.opt("s")) {
        v.push(Outcome::area_min("as_main", "Main reinforcement", "Hauptbewehrung", a, amin, "A_s ≥ A_s,min".into()));
        v.push(Outcome::length("s_util", "Main bar spacing", "Hauptbewehrungsabstand", s, de_cap.map(|c| c.min(smax)).unwrap_or(smax), "s ≤ s_max".into()));
        if let Some(sec_as) = c.opt("as_secondary") { v.push(Outcome::area_min("as_secondary", "Secondary reinforcement", "Querbewehrung", sec_as, 0.2 * a, "A_s,sec ≥ 0.2 A_s".into())); }
    }
    Ok(v)
}

pub fn dt07(c: &Case) -> R {
    let (h, ved) = (c.pos("h")?, c.pos("v_ed")?);
    let mut v = vec![Outcome::length_min("h_links", "Slab thickness for shear links", "Plattendicke für Querkraftbewehrung", h, 200.0, "h ≥ 200 mm".into())];
    if let Some(vrdmax) = c.opt("v_rd_max") { v.push(Outcome::force("min_links", "Shear level requiring minimum links", "Querkraft mit Mindestbügeln", ved, vrdmax / 3.0, "V_Ed ≤ V_Rd,max/3".into())); }
    if let Some(vrdc) = c.opt("v_rd_c") { v.push(Outcome::force("no_links", "Shear without links permitted", "Querkraft ohne Bügel zulässig", ved, vrdc, "V_Ed ≤ V_Rd,c".into())); }
    Ok(v)
}

pub fn dt08(c: &Case) -> R {
    let (ved, at) = (c.pos("v_ed")?, c.pos("a_t")?);
    let fyd = f_yd(c.annex, c.o("fyk", 500.0), true);
    let integrity = ved / fyd;
    let mut v = vec![
        Outcome::mm2("as_integrity", "Integrity bars through the column", "Durchlaufende Bewehrung durch die Stütze", integrity, "A_s = V_Ed/f_yd (accidental)".into()),
        Outcome::mm2("as_strip_min", "Top steel in the column strip", "Obere Bewehrung im Gurtstreifen", 0.5 * at, "0.5 A_t within 0.125 b each side".into()).steps_de("0,5 A_t innerhalb von 0,125 b je Seite"),
    ];
    if let Some(p) = c.opt("as_through") { v.push(Outcome::area_min("integrity", "Provided integrity steel", "Vorhandene Robustheitsbewehrung", p, integrity, "A_s ≥ V_Ed/f_yd".into())); }
    if let (Some(me), Some(be), Some(d)) = (c.opt("m_edge"), c.opt("b_e"), c.opt("d")) { v.push(Outcome::moment("m_transfer", "Edge column moment transfer", "Momentenübertragung Randstütze", me, 0.17 * be * d * d * c.pos("fck")?, "M_Ed ≤ 0.17 b_e d² f_ck".into())); }
    Ok(v)
}

pub fn dt09(c: &Case) -> R {
    let (ac, ned) = (c.pos("ac")?, c.o("n_ed", 0.0));
    let fyd = f_yd(c.annex, c.o("fyk", 500.0), false);
    let phi_min = if c.de() { 12.0 } else { 8.0 };
    let asmin = (c.ndp("ndp.en1992.col-asmin-n") * ned / fyd).max(0.002 * ac);
    let phi = c.o("phi", phi_min);
    let factor = c.ndp("ndp.en1992.col-link-spacing-de");
    let cap = c.ndp("ndp.en1992.col-link-spacing-max");
    let near = c.ndp("ndp.en1992.col-link-near-factor");
    let smax = (factor * phi).min(c.o("b", f64::MAX)).min(cap);
    let mut v = vec![
        Outcome::mm("phi_min", "Minimum bar diameter", "Mindeststabdurchmesser", phi_min, "9.5.2(1)".into()),
        Outcome::mm2("as_min", "Minimum column reinforcement", "Mindestbewehrung Stütze", asmin, "max(k N_Ed/f_yd, 0.002 A_c)".into()),
        Outcome::mm2("as_max", "Maximum column reinforcement", "Höchstbewehrung Stütze", if c.o("at_laps", 0.0) > 0.0 { 0.08 * ac } else { 0.04 * ac }, "0.04 A_c (0.08 A_c at laps)".into()),
        Outcome::mm("phi_link", "Minimum link diameter", "Mindestbügeldurchmesser", 6.0f64.max(phi / 4.0), "max(6 mm, φ/4)".into()),
        Outcome::mm("scl_max", "Largest link spacing", "Größter Bügelabstand", smax, format!("min({factor} φ, b, {cap} mm)")),
        Outcome::mm("scl_max_near", "Largest link spacing near beams and laps", "Größter Bügelabstand an Balken und Stößen", near * smax, format!("{near} s_cl,tmax")),
    ];
    if let Some(a) = c.opt("as") { v.push(Outcome::area_min("as", "Provided column reinforcement", "Vorhandene Stützenbewehrung", a, asmin, "A_s ≥ A_s,min".into())); v.push(Outcome::area("as_upper", "Column reinforcement against maximum", "Stützenbewehrung gegen Höchstwert", a, if c.o("at_laps", 0.0) > 0.0 { 0.08 * ac } else { 0.04 * ac }, "A_s ≤ 0.04 A_c (0.08 A_c at laps)".into())); }
    if let Some(n) = c.opt("bars") { v.push(Outcome::ratio_min("bars", "Number of longitudinal bars", "Anzahl Längsstäbe", n, 4.0, "n ≥ 4".into())); }
    Ok(v)
}

pub fn dt10(c: &Case) -> R {
    let (ac, tw) = (c.pos("ac")?, c.pos("tw")?);
    let asv = 0.002 * ac;
    let mut v = vec![
        Outcome::mm2("asv_min", "Minimum vertical reinforcement", "Mindestlängsbewehrung", asv, "0.002 A_c".into()),
        Outcome::mm2("asv_max", "Maximum vertical reinforcement", "Höchstlängsbewehrung", 0.04 * ac, "0.04 A_c".into()),
        Outcome::mm("sv_max", "Largest vertical bar spacing", "Größter Längsstababstand", (3.0 * tw).min(400.0), "min(3 t_w, 400 mm)".into()),
    ];
    if let Some(a) = c.opt("asv") {
        v.push(Outcome::area_min("asv", "Provided vertical reinforcement", "Vorhandene Längsbewehrung", a, asv, "≥ 0.002 A_c".into()));
        v.push(Outcome::mm2("ash_min", "Minimum horizontal reinforcement", "Mindestquerbewehrung", (0.25 * a).max(0.001 * ac), "max(0.25 A_sv, 0.001 A_c)".into()));
        v.push(Outcome::area("links", "Links needed above 2 %", "Bügel ab 2 %", a, 0.02 * ac, "A_sv ≤ 0.02 A_c without links".into()).steps_de("A_sv ≤ 0,02 A_c ohne Bügel"));
    }
    Ok(v)
}

pub fn dt11(c: &Case) -> R {
    let (ac_per_m, h) = (c.pos("ac_per_m")?, c.pos("h")?);
    let req = (0.001 * ac_per_m).max(150.0);
    let mut v = vec![Outcome::mm2("as_mesh", "Orthogonal mesh per face", "Orthogonales Netz je Seite", req, "max(0.001 A_c, 150 mm²/m)".into()), Outcome::mm("s_max", "Largest mesh spacing", "Größter Netzabstand", (2.0 * h).min(300.0), "min(2h, 300 mm)".into())];
    if let Some(a) = c.opt("as_provided") { v.push(Outcome::area_min("util", "Provided mesh", "Vorhandenes Netz", a, req, "≥ required".into())); }
    Ok(v)
}

pub fn dt12(c: &Case) -> R {
    let (n, d) = (c.pos("n_ed")?, c.pos("d")?);
    let fyd = f_yd(c.annex, c.o("fyk", 500.0), false);
    let (ft, why) = match c.so("kind", "strip") {
        "pile_cap_2" => { let (s, col) = (c.pos("s")?, c.pos("c")?); (n / 2.0 * (s / 2.0 - col / 4.0) / (0.9 * d), "F_td = (N/2)(s/2 − c/4)/z".to_string()) }
        _ => { let (b, a) = (c.pos("b")?, c.pos("a")?); (n * (b - a) / (8.0 * d), "F_td = N (b − a)/(8d)".to_string()) }
    };
    let mut v = vec![Outcome::n("f_td", "Tie force F_td", "Zugkraft F_td", ft, why), Outcome::mm2("as_tie", "Required tie reinforcement", "Erforderliche Zugbewehrung", ft / fyd, "F_td/f_yd".into())];
    if let Some(p) = c.opt("as_provided") { v.push(Outcome::area("util", "Tie utilisation", "Zugbandausnutzung", ft / fyd, p, "A_s,req ≤ A_s,prov".into())); }
    Ok(v)
}

pub fn dt13(c: &Case) -> R {
    let (fbt, ab, phi, fck) = (c.pos("f_bt")?, c.pos("a_b")?, c.pos("phi")?, c.pos("fck")?);
    let fcd = f_cd(c.annex, fck, false);
    let phi_m = fbt * (1.0 / ab + 1.0 / (2.0 * phi)) / fcd;
    let mut v = vec![Outcome::mm("phi_m_min", "Minimum mandrel diameter φ_m,min", "Mindestbiegerollendurchmesser φ_m,min", phi_m, "F_bt (1/a_b + 1/(2φ))/f_cd".into())];
    if let Some(p) = c.opt("phi_m") { v.push(Outcome::length_min("util", "Provided mandrel diameter", "Vorhandener Biegerollendurchmesser", p, phi_m, "φ_m ≥ φ_m,min".into())); }
    if let (Some(m), Some(z)) = (c.opt("m_ed"), c.opt("z")) { v.push(Outcome::n("corner_tension", "Corner tension force", "Eckzugkraft", m / z, "M_Ed/z".into())); }
    Ok(v)
}

pub fn dt14(c: &Case) -> R {
    let (li, fyk) = (c.pos("li")?, c.o("fyk", 500.0));
    let fyd = f_yd(c.annex, fyk, true);
    let per = (li * c.ndp("ndp.en1992.tie-q1")).min(c.ndp("ndp.en1992.tie-q2")) * 1000.0;
    let int = c.ndp("ndp.en1992.tie-internal").max(15.0) * 1000.0 * c.o("width_m", 1.0);
    let col = c.ndp("ndp.en1992.tie-column") * 1000.0 * c.o("wall_length_m", 1.0);
    let (kind, f_req) = match c.so("tie", "peripheral") { "internal" => ("internal", int), "column" => ("column", col), _ => ("peripheral", per) };
    let mut v = vec![Outcome::n("f_tie", &format!("Tie force {kind}"), &format!("Zugkraft {kind}"), f_req, "9.10.2".into()), Outcome::mm2("as_req", "Required tie steel", "Erforderlicher Zuganker", f_req / fyd, "F/f_yd (γ_s = 1.0)".into())];
    if let Some(a) = c.opt("as_provided") { v.push(Outcome::area_min("util", "Provided tie steel", "Vorhandener Zuganker", a, f_req / fyd, "A_s ≥ F/f_yd".into())); }
    Ok(v)
}

/// 🗂 Entries of this domain.
pub const ENTRIES: &[Entry] = &[
    entry_of("B-01", "DIN EN 1992-1-1", "8.4.2", "Ultimate bond stress", "Verbundspannung", &["fck", "phi", "bond"], &["ndp.en1992.m-01.alpha-ct", "ndp.en1992.m-01.gamma-c-persistent", "ndp.en1992.fbd-eta"], b01),
    entry_of("B-02", "DIN EN 1992-1-1", "8.4.3", "Basic required anchorage length", "Grundmaß der Verankerungslänge", &["fck", "phi", "bond", "fyk", "sigma_sd", "fbd"], &["ndp.en1992.m-01.alpha-ct", "ndp.en1992.m-01.gamma-c-persistent", "ndp.en1992.gamma-s", "ndp.en1992.fbd-eta"], b02),
    entry_of("B-03", "DIN EN 1992-1-1", "8.4.4", "Design anchorage length", "Bemessungswert der Verankerungslänge", &["fck", "phi", "bond", "shape", "loading", "cd", "sum_ast", "sum_ast_min", "as", "k_conf", "welded", "p_transverse", "l_available"], &["ndp.en1992.m-01.alpha-ct", "ndp.en1992.m-01.gamma-c-persistent", "ndp.en1992.gamma-s", "ndp.en1992.fbd-eta", "ndp.en1992.k-mandrel-small", "ndp.en1992.k-mandrel-large"], b03),
    entry_of("B-04", "DIN EN 1992-1-1", "8.7.3", "Lap length", "Übergreifungslänge", &["fck", "phi", "bond", "rho1", "cd", "loading", "alpha3", "p_transverse", "l_available", "clear_distance", "sum_ast", "as"], &["ndp.en1992.m-01.alpha-ct", "ndp.en1992.m-01.gamma-c-persistent", "ndp.en1992.gamma-s", "ndp.en1992.fbd-eta"], b04),
    entry_of("B-05", "DIN EN 1992-1-1", "8.9", "Bundled bars", "Stabbündel", &["phi", "nb", "situation"], &[], b05),
    entry_of("B-06", "DIN EN 1992-1-1", "8.8", "Large-diameter bars", "Große Stabdurchmesser", &["phi", "act_ext", "as_surf_provided"], &[], b06),
    entry_of("B-07", "DIN EN 1992-1-1", "8.10.2", "Transmission length of prestress", "Übertragungslänge der Vorspannung", &["fck", "phi", "sigma_pm0", "tendon", "release", "bond", "fctd_t", "t", "cement", "d"], &["ndp.en1992.m-01.alpha-ct", "ndp.en1992.m-01.gamma-c-persistent"], b07),
    entry_of("B-08", "DIN EN 1992-1-1", "8.5", "Anchorage of links", "Verankerung von Bügeln", &["phi", "shape", "angle"], &[], b08),
    entry_of("B-09", "DIN EN 1992-1-1", "8.2", "Spacing of bars", "Stababstand", &["phi", "dg", "clear"], &[], b09),
    entry_of("DT-01", "DIN EN 1992-1-1", "9.2.1.1", "Minimum and maximum tension reinforcement of beams", "Mindest- und Höchstbewehrung Balken", &["fck", "bt", "d", "h", "as", "ac", "fyk"], &[], dt01),
    entry_of("D-30", "DIN EN 1992-1-1", "9.2.1.1", "Minimum and maximum reinforcement summary", "Mindest- und Höchstbewehrung Zusammenfassung", &["fck", "bt", "d", "h", "as", "ac", "fyk"], &[], dt01),
    entry_of("DT-02", "DIN EN 1992-1-1", "9.2.1.3", "End supports and shift rule", "Endauflager und Versatzmaß", &["v_ed", "z", "as_span", "cot_theta", "cot_alpha", "n_ed", "as_support"], &[], dt02),
    entry_of("DT-03", "DIN EN 1992-1-1", "9.2.2", "Shear reinforcement of beams", "Schubbewehrung Balken", &["fck", "bw", "d", "alpha_deg", "fyk", "asw", "s"], &[], dt03),
    entry_of("DT-04", "DIN EN 1992-1-1", "9.2.3", "Torsion links", "Torsionsbügel", &["u", "b", "d", "s"], &[], dt04),
    entry_of("DT-05", "DIN EN 1992-1-1", "9.2.4", "Surface reinforcement", "Oberflächenbewehrung", &["act_ext", "as_provided", "s"], &[], dt05),
    entry_of("DT-06", "DIN EN 1992-1-1", "9.3.1", "Solid slab reinforcement", "Bewehrung Vollplatte", &["fck", "h", "d", "fyk", "as_main", "s", "as_secondary"], &["ndp.en1992.slab-smax-cracked"], dt06),
    entry_of("DT-07", "DIN EN 1992-1-1", "9.3.2", "Slab shear reinforcement", "Querkraftbewehrung Platte", &["h", "v_ed", "v_rd_max", "v_rd_c"], &[], dt07),
    entry_of("DT-08", "DIN EN 1992-1-1", "9.4", "Flat slab reinforcement", "Bewehrung Flachdecke", &["v_ed", "a_t", "fyk", "as_through", "m_edge", "b_e", "d", "fck"], &["ndp.en1992.gamma-s-acc"], dt08),
    entry_of("DT-09", "DIN EN 1992-1-1", "9.5", "Column detailing", "Konstruktive Regeln Stützen", &["ac", "n_ed", "fyk", "phi", "b", "at_laps", "as", "bars"], &["ndp.en1992.col-asmin-n", "ndp.en1992.gamma-s", "ndp.en1992.col-link-spacing-de", "ndp.en1992.col-link-spacing-max", "ndp.en1992.col-link-near-factor"], dt09),
    entry_of("DT-10", "DIN EN 1992-1-1", "9.6", "Wall detailing", "Konstruktive Regeln Wände", &["ac", "tw", "asv"], &[], dt10),
    entry_of("DT-11", "DIN EN 1992-1-1", "9.7", "Deep beams", "Wandartige Träger", &["ac_per_m", "h", "as_provided"], &[], dt11),
    entry_of("DT-12", "DIN EN 1992-1-1", "9.8", "Foundations", "Fundamente", &["kind", "n_ed", "d", "b", "a", "s", "c", "fyk", "as_provided"], &["ndp.en1992.gamma-s"], dt12),
    entry_of("DT-13", "DIN EN 1992-1-1", "9.9", "Regions with discontinuity", "Diskontinuitätsbereiche", &["f_bt", "a_b", "phi", "fck", "phi_m", "m_ed", "z"], &["ndp.en1992.m-01.alpha-cc-persistent", "ndp.en1992.m-01.gamma-c-persistent"], dt13),
    entry_of("DT-14", "DIN EN 1992-1-1", "9.10", "Tying systems", "Zugankersysteme", &["li", "tie", "width_m", "wall_length_m", "fyk", "as_provided"], &["ndp.en1992.tie-q1", "ndp.en1992.tie-q2", "ndp.en1992.tie-internal", "ndp.en1992.tie-column", "ndp.en1992.gamma-s-acc"], dt14),
];
