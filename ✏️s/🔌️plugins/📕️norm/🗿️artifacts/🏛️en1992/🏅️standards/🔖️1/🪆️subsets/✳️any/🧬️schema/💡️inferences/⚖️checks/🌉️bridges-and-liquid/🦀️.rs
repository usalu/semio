//! 🌉️ Bridges (EN 1992-2) and liquid-retaining structures (EN 1992-3) as catalogue entries BR-01 … BR-08 and LQ-01 … LQ-07 on top of the part 1-1 functions. See [`super`].

use super::*;

fn br01(c: &Case) -> R {
    let fck = c.pos("fck")?;
    let lt = if c.o("prestressed", 0.0) > 0.0 { c.ndp("ndp.en1992.bridge-alpha-cc-lt") } else { 1.0 };
    let fcd = f_cd(c.annex, fck, false) * lt;
    Ok(vec![Outcome::mpa("fcd", "Design strength of bridge concrete", "Bemessungsfestigkeit Brückenbeton", fcd, format!("long-term factor {}", f(lt))), Outcome::ratio("fck_limit", "Concrete class against C70/85", "Betonklasse gegen C70/85", fck, 70.0, "f_ck ≤ 70 MPa".into())])
}

fn br02(c: &Case) -> R {
    let mut case = c.clone();
    let precast = c.o("precast", 0.0) > 0.0;
    case.num.insert("dc_dev".into(), if precast { 5.0 } else { 10.0 });
    case.num.entry("design_life".into()).or_insert(100.0);
    let mut out = super::materials_and_time::d04(&case)?;
    let chloride = matches!(c.s("exposure")?, "XD3" | "XS3");
    if chloride {
        let add = c.ndp("ndp.en1992.bridge-cover-chloride");
        if let Some(o) = out.iter_mut().find(|o| o.key == "cnom") { o.capacity += add / 1e3; }
        if let Some(o) = out.iter_mut().find(|o| o.key == "c_provided") { o.capacity += add / 1e3; }
    }
    Ok(out)
}

fn br03(c: &Case) -> R {
    let mut out = super::shear_torsion::v03(c)?;
    if c.opt("bi").is_some() {
        let mut case = c.clone();
        case.num.insert("fatigue".into(), 1.0);
        out.extend(super::shear_torsion::v05(&case)?.into_iter().map(|mut o| { o.key = format!("fatigue_joint_{}", o.key); o }));
    }
    Ok(out)
}

fn br04(c: &Case) -> R {
    let mut out = super::prestress_and_fatigue::f07(c)?;
    if let Some(eq) = c.opt("delta_sigma_tendon") {
        let kind = c.so("tendon_kind", "post_curved_steel");
        let (_, _, _, ds) = super::prestress_and_fatigue::sn_curve(c.annex, kind).ok_or("unknown tendon kind")?;
        out.push(Outcome::stress("util_tendon", "Tendon stress range", "Spannstahl-Schwingbreite", eq, ds / c.ndp("ndp.en1992.gamma-s-fat"), format!("Δσ_Rsk = {}", f(ds))));
    }
    Ok(out)
}

fn br05(c: &Case) -> R {
    let mut case = c.clone();
    let fctm = conc(c.pos("fck")?).fctm;
    let fct = c.o("fct_eff", fctm).max(c.ndp("ndp.en1992.bridge-fct-min"));
    case.num.insert("fct_eff".into(), fct);
    let mut out = super::serviceability::sl04(&case)?;
    let lim = super::serviceability::wk_limit(c.annex, c.so("exposure", "XC2"), c.o("prestressed_bonded", 0.0) > 0.0);
    out.push(Outcome::mm("wk_lim", "Crack width limit w_k,lim", "Rissbreitengrenze w_k,lim", lim, "Table 7.101".into()));
    if let Some(w) = c.opt("wk") { out.push(Outcome::length("util_wk", "Crack width", "Rissbreite", w, lim, "w_k ≤ w_k,lim".into())); }
    Ok(out)
}

fn br06(c: &Case) -> R {
    let (mt, p, e) = (c.n("m_total")?, c.pos("p")?, c.n("e_p")?);
    let mut v = vec![Outcome::nmm("m_sec", "Secondary (hyperstatic) moment", "Sekundäres Moment", mt - p * e, "M_total − P e".into())];
    if let (Some(fs), Some(l)) = (c.opt("sag"), c.opt("l")) { v.push(Outcome::num("q_dev", "Deviation force (N/mm)", "Umlenkkraft (N/mm)", 8.0 * p * fs / (l * l), "8 P f/l²".into())); }
    Ok(v)
}

fn br07(c: &Case) -> R {
    let mut case = c.clone();
    let h = c.pos("h")?;
    let wheel = c.o("wheel_print", 400.0);
    case.num.insert("c1".into(), wheel + h);
    case.num.insert("c2".into(), wheel + h);
    case.num.insert("v_ed".into(), c.pos("q_wheel")?);
    case.num.insert("beta".into(), 1.0);
    case.text.insert("position".into(), "interior".into());
    case.num.remove("beta_computed");
    super::punching_and_struts::p03(&case)
}

fn br08(c: &Case) -> R {
    super::analysis_models::a09(c)
}

fn lq01(c: &Case) -> R {
    let (class, hd_h) = (c.n("class")? as u32, c.n("hd_over_h")?);
    let exposure = c.so("exposure", "XC2");
    let base = super::serviceability::wk_limit(c.annex, exposure, false);
    let lim = match class {
        0 => base,
        _ => interp(&[(5.0, c.ndp("ndp.en1992.tightness-wk-high")), (35.0, c.ndp("ndp.en1992.tightness-wk-low"))], hd_h),
    };
    let mut v = vec![Outcome::mm("wk_lim", "Crack width limit w_k,lim", "Rissbreitengrenze w_k,lim", lim, format!("tightness class {class}, h_D/h = {}", f(hd_h)))];
    if class >= 2 {
        let h = c.pos("h")?;
        let need = (50.0f64).min(0.2 * h);
        v.push(Outcome::length_min("compression_zone", "Compression zone height", "Höhe der Druckzone", c.o("x_compression", 0.0), need, "x ≥ min(50 mm, 0.2 h)".into()));
    }
    if let Some(w) = c.opt("wk") { v.push(Outcome::length("util", "Crack width", "Rissbreite", w, lim, "w_k ≤ w_k,lim".into())); }
    Ok(v)
}

fn lq02(c: &Case) -> R {
    let rax = c.pos("rax")?;
    let free = c.opt("eps_free").unwrap_or_else(|| c.o("eps_ca", 0.0) + c.o("eps_cd", 0.0) + c.o("alpha_c", 1.0e-5) * c.o("delta_t", 0.0));
    let eps = rax * free;
    let sr = c.pos("sr_max")?;
    let mut v = vec![Outcome::num("eps_sm_cm", "Restrained strain ε_sm − ε_cm", "Behinderte Dehnung ε_sm − ε_cm", eps, format!("R_ax = {}, ε_free = {}", f(rax), f(free))), Outcome::mm("wk", "Early-age crack width w_k", "Rissbreite im jungen Alter w_k", sr * eps, "s_r,max (ε_sm − ε_cm)".into())];
    if let Some(l) = c.opt("wk_lim") { v.push(Outcome::length("util", "Early-age crack width", "Rissbreite im jungen Alter", sr * eps, l, "w_k ≤ w_k,lim".into())); }
    Ok(v)
}

fn lq03(c: &Case) -> R {
    let mut case = c.clone();
    case.text.insert("duration".into(), "long".into());
    if c.opt("wk_lim").is_none() && c.opt("class").is_some() {
        let lim = lq01(c)?.into_iter().find(|x| x.key == "wk_lim").map(|x| x.value() * 1e3).ok_or("no tightness limit")?;
        case.num.insert("wk_lim".into(), lim);
    }
    super::serviceability::sl06(&case)
}

fn lq04(c: &Case) -> R {
    let (wk, hd_h) = (c.pos("wk")?, c.pos("hd_over_h")?);
    let ok_head = hd_h <= c.ndp("ndp.en1992.lq-04-head-ratio");
    let (lim_low, lim_high) = (c.ndp("ndp.en1992.lq-04-wk-selfhealing"), c.ndp("ndp.en1992.lq-04-wk-above"));
    Ok(vec![Outcome::length("util", "Self-healing crack width", "Selbstheilungs-Rissbreite", wk, if ok_head { lim_low } else { lim_high }, format!("w_k ≤ {lim_low} mm (h_D/h ≤ {}), else {lim_high} mm", c.ndp("ndp.en1992.lq-04-head-ratio"))), Outcome::num("self_healing", "Self-healing admissible (1 = yes)", "Selbstheilung zulässig (1 = ja)", if ok_head { 1.0 } else { 0.0 }, "7.3.1".into())])
}

fn lq05(c: &Case) -> R {
    let mut out = super::materials_and_time::d04(c)?;
    let add = c.ndp("ndp.en1992.liquid-cover-add");
    for o in out.iter_mut().filter(|o| o.key == "cnom" || o.key == "c_provided") { o.capacity += add / 1e3; }
    Ok(out)
}

fn lq06(c: &Case) -> R {
    let (p_liq, r, t, pm) = (c.pos("pressure")?, c.pos("radius")?, c.pos("t")?, c.pos("pm_t_per_m")?);
    let ring = p_liq * r;
    let sigma = (pm - ring) / t;
    Ok(vec![Outcome::n("ring_force", "Ring tension per metre", "Ringzugkraft je Meter", ring, "p R".into()), Outcome::stress("util_decompression", "Residual compression against minimum", "Restdruck gegen Mindestwert", c.ndp("ndp.en1992.tank-sigma-min"), sigma.max(1e-9), format!("σ_c = {}", f(sigma)))])
}

fn lq07(c: &Case) -> R {
    let items = c.l("checklist")?;
    let done = items.iter().filter(|v| **v > 0.5).count() as f64;
    Ok(vec![Outcome::ratio_min("checklist", "Detailing checklist items satisfied", "Konstruktive Punkte erfüllt", done, items.len() as f64, "joints, water bars, corners".into())])
}

/// 🗂 Entries of this domain.
pub const ENTRIES: &[Entry] = &[
    entry_of("BR-01", "DIN EN 1992-2", "3.1", "Bridge concrete", "Brückenbeton", &["fck", "prestressed"], &["ndp.en1992.m-01.alpha-cc-persistent", "ndp.en1992.m-01.gamma-c-persistent", "ndp.en1992.bridge-alpha-cc-lt"], br01),
    entry_of("BR-02", "DIN EN 1992-2", "4.4.1.2", "Bridge cover", "Betondeckung Brücken", &["exposure", "fck", "phi", "design_life", "precast", "c_provided"], &["ndp.en1992.cmin-dur-table", "ndp.en1992.bridge-cover-chloride"], br02),
    entry_of("BR-03", "DIN EN 1992-2", "6.2", "Bridge shear", "Querkraft Brücken", &["fck", "bw", "d", "z", "v_ed", "asw_s", "bi", "roughness"], &["ndp.en1992.cot-theta-max", "ndp.en1992.cot-theta-a", "ndp.en1992.cot-theta-b", "ndp.en1992.vrdcc-c", "ndp.en1992.vrdcc-k", "ndp.en1992.nu1-factor", "ndp.en1992.nu2-offset", "ndp.en1992.nu2-fck-divisor", "ndp.en1992.alpha-cw", "ndp.en1992.m-01.alpha-cc-persistent", "ndp.en1992.m-01.gamma-c-persistent", "ndp.en1992.gamma-s"], br03),
    entry_of("BR-04", "DIN EN 1992-2", "6.8", "Bridge fatigue", "Ermüdung Brücken", &["lambda1", "lambda2", "lambda3", "lambda4", "delta_sigma_71", "delta_sigma_tendon", "tendon_kind"], &["ndp.en1992.fatigue-straight", "ndp.en1992.gamma-s-fat"], br04),
    entry_of("BR-05", "DIN EN 1992-2", "7.3", "Bridge crack control", "Rissbreitenbegrenzung Brücken", &["fck", "b", "h", "fct_eff", "exposure", "prestressed_bonded", "wk"], &["ndp.en1992.bridge-fct-min", "ndp.en1992.wk-lim-prestress", "ndp.en1992.wk-lim"], br05),
    entry_of("BR-06", "DIN EN 1992-2", "5.10", "Bridge prestress", "Vorspannung Brücken", &["m_total", "p", "e_p", "sag", "l"], &[], br06),
    entry_of("BR-07", "DIN EN 1992-2", "6.4", "Wheel load punching", "Durchstanzen unter Radlast", &["fck", "d", "h", "q_wheel", "wheel_print", "rho_ly", "rho_lz"], &["ndp.en1992.crd-c-punch", "ndp.en1992.k1-punch", "ndp.en1992.m-01.gamma-c-persistent", "ndp.en1992.punch-vrdmax-factor", "ndp.en1992.vmin-shear", "ndp.en1992.vmin-shear-d800"], br07),
    entry_of("BR-08", "DIN EN 1992-2", "5.8", "Bridge piers", "Brückenpfeiler", &["fck", "b", "h", "n_ed", "l0", "as"], &["ndp.en1992.lambda-lim-de-n", "ndp.en1992.m-01.alpha-cc-persistent", "ndp.en1992.m-01.gamma-c-persistent"], br08),
    entry_of("LQ-01", "DIN EN 1992-3", "7.3.1", "Tightness classes", "Dichtheitsklassen", &["class", "hd_over_h", "exposure", "h", "x_compression", "wk"], &["ndp.en1992.tightness-wk-high", "ndp.en1992.tightness-wk-low", "ndp.en1992.wk-lim", "ndp.en1992.wk-lim-xc1"], lq01),
    entry_of("LQ-02", "DIN EN 1992-3", "7.3.2", "Early-age restraint cracking", "Zwangrissbildung im jungen Alter", &["rax", "eps_free", "eps_ca", "eps_cd", "alpha_c", "delta_t", "sr_max", "wk_lim"], &[], lq02),
    entry_of("LQ-03", "DIN EN 1992-3", "7.3.4", "Crack width of liquid structures", "Rissbreite Flüssigkeitsbehälter", &["fck", "b", "h", "d", "as", "c", "phi", "m_qp", "sigma_s", "class", "hd_over_h"], &["ndp.en1992.kt-long", "ndp.en1992.k3-crack", "ndp.en1992.k4-crack", "ndp.en1992.tightness-wk-high", "ndp.en1992.tightness-wk-low"], lq03),
    entry_of("LQ-04", "DIN EN 1992-3", "7.3.1(105)", "Self-healing", "Selbstheilung", &["wk", "hd_over_h"], &["ndp.en1992.tightness-wk-high", "ndp.en1992.lq-04-head-ratio", "ndp.en1992.lq-04-wk-selfhealing", "ndp.en1992.lq-04-wk-above"], lq04),
    entry_of("LQ-05", "DIN EN 1992-3", "4.4", "Cover of liquid structures", "Betondeckung Flüssigkeitsbehälter", &["exposure", "fck", "phi", "design_life", "c_provided"], &["ndp.en1992.liquid-cover-add", "ndp.en1992.cmin-dur-table", "ndp.en1992.delta-c-dev"], lq05),
    entry_of("LQ-06", "DIN EN 1992-3", "7.4", "Prestressed tanks", "Vorgespannte Behälter", &["pressure", "radius", "t", "pm_t_per_m"], &["ndp.en1992.tank-sigma-min"], lq06),
    entry_of("LQ-07", "DIN EN 1992-3", "8", "Joint and corner detailing checklist", "Checkliste Fugen und Ecken", &["checklist"], &[], lq07),
];
