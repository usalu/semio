//! 🧵️ Prestress (§5.10, §3.3) and fatigue (§6.8) as catalogue entries PR-01 … PR-10 and F-01 … F-07. See [`super`].

use super::*;

/// 🎻 Friction loss ratio P(x)/P_max = exp(−μ(θ + k x)), x in m.
pub fn friction_ratio(mu: f64, k: f64, theta: f64, x_m: f64) -> f64 {
    (-mu * (theta + k * x_m)).exp()
}

/// ⏳ Relaxation loss Δσ_pr/σ_pi (5.10.6, Eq. 3.28–3.30) for class 1, 2 or 3, μ = σ_pi/f_pk, t in hours, ρ_1000 in percent.
pub fn relaxation_ratio(class: u32, mu: f64, t_h: f64, rho1000: f64) -> f64 {
    let (a, b) = match class { 1 => (5.39, 6.7), 3 => (1.98, 8.0), _ => (0.66, 9.1) };
    a * rho1000 * (b * mu).exp() * (t_h / 1000.0).powf(0.75 * (1.0 - mu)) * 1e-5
}

/// 🎻 Default ρ_1000 in percent of the relaxation class.
pub fn rho1000_default(class: u32) -> f64 {
    match class { 1 => 8.0, 3 => 4.0, _ => 2.5 }
}

pub fn pr01(c: &Case) -> R {
    let (ap, fpk, fp01k) = (c.pos("ap")?, c.pos("fpk")?, c.pos("fp01k")?);
    let smax = (c.ndp("ndp.en1992.p-k1") * fpk).min(c.ndp("ndp.en1992.p-k2") * fp01k);
    let mut v = vec![Outcome::mpa("sigma_p_max", "Maximum jacking stress", "Höchste Spannstahlspannung beim Spannen", smax, "min(k1 f_pk, k2 f_p0.1k)".into()), Outcome::n("p_max", "Maximum jacking force P_max", "Größte Spannkraft P_max", ap * smax, "A_p σ_p,max".into())];
    if let Some(s) = c.opt("sigma_jack") { v.push(Outcome::stress("util", "Jacking stress", "Spannstahlspannung beim Spannen", s, smax, "σ ≤ σ_p,max".into())); }
    Ok(v)
}

pub fn pr02(c: &Case) -> R {
    let (ap, fpk, fp01k) = (c.pos("ap")?, c.pos("fpk")?, c.pos("fp01k")?);
    let lim = (c.ndp("ndp.en1992.p-k7") * fpk).min(c.ndp("ndp.en1992.p-k8") * fp01k);
    let mut v = vec![Outcome::mpa("sigma_pm0_max", "Limit of σ_pm0", "Grenze von σ_pm0", lim, "min(k7 f_pk, k8 f_p0.1k)".into()), Outcome::n("pm0_max", "Limit of P_m0", "Grenze von P_m0", ap * lim, "A_p σ_pm0,max".into())];
    if let Some(p) = c.opt("pm0") { v.push(Outcome::stress("util", "Initial prestress", "Anfangsspannung", p / ap, lim, "σ_pm0 ≤ limit".into())); }
    Ok(v)
}

pub fn pr03(c: &Case) -> R {
    let (ap, ep, ecm) = (c.pos("ap")?, c.o("ep", 195000.0), c.pos("ecm_t")?);
    let dsc = c.n("delta_sigma_c")?;
    if c.so("system", "pre") == "pre" {
        let d = ep / ecm * dsc;
        Ok(vec![Outcome::mpa("delta_sigma_el", "Elastic shortening loss Δσ_el", "Verlust aus elastischer Verkürzung Δσ_el", d, "α_e σ_c,p".into()), Outcome::n("delta_p_el", "ΔP_el", "ΔP_el", d * ap, "A_p Δσ_el".into())])
    } else {
        let n = c.pos("n_tendons")?;
        let j = (n - 1.0) / (2.0 * n);
        let dp = ap * ep * j * dsc / ecm;
        Ok(vec![Outcome::n("delta_p_el", "ΔP_el (post-tensioning)", "ΔP_el (nachträglicher Verbund)", dp, format!("j = {}", f(j)))])
    }
}

pub fn pr04(c: &Case) -> R {
    let (pmax, mu, k) = (c.pos("pmax")?, c.o("mu", c.ndp("ndp.en1992.friction-mu")), c.o("k", c.ndp("ndp.en1992.friction-k")));
    let (xs, th) = (c.l("x")?, c.l("theta")?);
    let mut v = Vec::new();
    for (i, (x, t)) in xs.iter().zip(th.iter()).enumerate() {
        let r = friction_ratio(mu, k, *t, *x);
        v.push(Outcome::n(&format!("p_{i}"), &format!("P_m0 at x = {} m", f(*x)), &format!("P_m0 bei x = {} m", f(*x)), pmax * r, format!("P/P_max = {}", f(r))));
    }
    if let (Some(slip), Some(ap), Some(rad)) = (c.opt("slip"), c.opt("ap"), c.opt("radius_m")) {
        let ep = c.o("ep", 195000.0);
        let p_grad = pmax * (mu / rad + k) / 1000.0;
        let x_sl = (ep * ap * slip / p_grad).sqrt();
        v.push(Outcome::mm("x_slip", "Slip influence length", "Einflusslänge des Schlupfs", x_sl, "√(E_p A_p Δ_sl/p)".into()));
        v.push(Outcome::n("delta_p_slip", "Loss at the anchorage", "Verlust am Anker", 2.0 * p_grad * x_sl, "2 p x_sl".into()));
    }
    Ok(v)
}

pub fn pr05(c: &Case) -> R {
    let (ep, ecm, phi) = (c.o("ep", 195000.0), c.pos("ecm")?, c.pos("phi")?);
    let (ap, ac, ic, zcp) = (c.pos("ap")?, c.pos("ac")?, c.pos("ic")?, c.n("zcp")?);
    let (eps_cs, scqp, sigma_pi, fpk) = (c.n("eps_cs")?, c.n("sigma_c_qp")?, c.pos("sigma_pi")?, c.pos("fpk")?);
    let class = c.o("relax_class", 2.0) as u32;
    let rho = c.o("rho1000", rho1000_default(class));
    let rel = relaxation_ratio(class, sigma_pi / fpk, c.o("t_hours", 500000.0), rho) * sigma_pi;
    let num = eps_cs.abs() * ep + 0.8 * rel + ep / ecm * phi * scqp;
    let den = 1.0 + ep / ecm * ap / ac * (1.0 + ac * zcp * zcp / ic) * (1.0 + 0.8 * phi);
    let loss = num / den;
    let mut v = vec![Outcome::mpa("relax", "Relaxation loss Δσ_pr", "Relaxationsverlust Δσ_pr", rel, format!("class {class}, ρ_1000 = {}", f(rho))), Outcome::mpa("loss", "Time-dependent loss Δσ_p,c+s+r", "Zeitabhängiger Verlust Δσ_p,c+s+r", loss, "Eq. 5.46 with magnitudes: ε_cs ≥ 0, σ_c,QP compression positive (tension reduces the loss)".into()).steps_de("Gl. 5.46 mit Beträgen: ε_cs ≥ 0, σ_c,QP Druck positiv (Zug verringert den Verlust)"), Outcome::n("pm_t", "Long-term force P_m,t", "Spannkraft P_m,t", (sigma_pi - loss) * ap, "(σ_pi − Δσ) A_p".into())];
    v.push(Outcome::stress("util", "Stress after losses", "Spannung nach Verlusten", sigma_pi - loss, c.ndp("ndp.en1992.sigma-p-qp") * fpk, "σ_pm,t ≤ k5 f_pk".into()));
    Ok(v)
}

pub fn pr06(c: &Case) -> R {
    let (sigma, fpk, fp01k) = (c.pos("sigma_pm_inf")?, c.pos("fpk")?, c.pos("fp01k")?);
    let dsig = c.o("delta_sigma", c.ndp("ndp.en1992.unbonded-dsigma"));
    let fpd = fp01k / c.ndp("ndp.en1992.gamma-s");
    Ok(vec![Outcome::mpa("sigma_p_uls", "σ_p at ULS (unbonded)", "σ_p im GZT (ohne Verbund)", sigma + dsig, format!("Δσ_p,ULS = {}", f(dsig))), Outcome::stress("util", "Unbonded tendon stress at ULS", "Spannung ohne Verbund im GZT", sigma + dsig, fpd.min(fpk), "σ_p,ULS ≤ f_pd".into())])
}

pub fn pr07(c: &Case) -> R {
    let pmt = c.pos("pm_t")?;
    let unb = c.so("bond", "bonded") != "bonded";
    let (rs, ri) = if unb { (c.ndp("ndp.en1992.p-r-sup-unbonded"), c.ndp("ndp.en1992.p-r-inf-unbonded")) } else { (c.ndp("ndp.en1992.p-r-sup"), c.ndp("ndp.en1992.p-r-inf")) };
    Ok(vec![Outcome::n("pk_sup", "P_k,sup", "P_k,sup", rs * pmt, format!("r_sup = {}", f(rs))), Outcome::n("pk_inf", "P_k,inf", "P_k,inf", ri * pmt, format!("r_inf = {}", f(ri))), Outcome::num("gamma_p", "γ_P", "γ_P", 1.0, "favourable and unfavourable".into()).steps_de("günstig und ungünstig")])
}

pub fn pr08(c: &Case) -> R {
    let p = c.pos("p")?;
    let q = if let (Some(fs), Some(l)) = (c.opt("sag"), c.opt("l")) { 8.0 * p * fs / (l * l) } else { p / c.pos("radius")? };
    Ok(vec![Outcome::num("q", "Deviation force per length (N/mm)", "Umlenkkraft je Länge (N/mm)", q, "q = P/r = 8 P f/l²".into())])
}

pub fn pr09(c: &Case) -> R {
    let (p, a, h) = (c.pos("pmax")?, c.pos("a")?, c.pos("h")?);
    let t = 0.25 * (1.0 - a / h) * p;
    let mut v = vec![Outcome::n("t_burst", "Bursting force T", "Spaltzugkraft T", t, "0.25 (1 − a/h) P".into()), Outcome::mm2("as_burst", "Bursting reinforcement", "Spaltzugbewehrung", t / f_yd(c.annex, c.o("fyk", 500.0), false), "T/f_yd".into())];
    if let Some(prov) = c.opt("as_provided") { v.push(Outcome::area_min("util", "Provided bursting reinforcement", "Vorhandene Spaltzugbewehrung", prov, t / f_yd(c.annex, c.o("fyk", 500.0), false), "A_s ≥ T/f_yd".into())); }
    Ok(v)
}

pub fn pr10(c: &Case) -> R {
    let (sc, fck_t) = (c.pos("sigma_c")?, c.pos("fck_t")?);
    let k = if c.o("nonlinear_creep_proven", 0.0) > 0.0 { 0.7 } else { 0.6 };
    Ok(vec![Outcome::stress("util", "Concrete stress at release", "Betonspannung bei Vorspannung", sc, k * fck_t, format!("σ_c ≤ {} f_ck(t)", f(k)))])
}

/// 📚 S-N parameters (N*, k1, k2, Δσ_Rsk in MPa) of Table 6.3N and 6.4N.
pub fn sn_curve(annex: AnnexChoice, kind: &str) -> Option<(f64, f64, f64, f64)> {
    Some(match kind {
        "straight" => (1e6, 5.0, 9.0, ndp(annex, "ndp.en1992.fatigue-straight")),
        "welded" => (1e7, 3.0, 5.0, ndp(annex, "ndp.en1992.fatigue-welded")),
        "coupler" => (1e7, 3.0, 5.0, ndp(annex, "ndp.en1992.fatigue-coupler")),
        "pretensioned" | "post_strand_plastic" => (1e6, 5.0, 9.0, 185.0),
        "post_straight_plastic" => (1e6, 5.0, 9.0, 150.0),
        "post_curved_steel" => (1e6, 5.0, 7.0, 120.0),
        "prestress_coupler" => (1e6, 5.0, 5.0, 80.0),
        _ => return None,
    })
}

pub fn f01(c: &Case) -> R {
    let required = matches!(c.so("structure", "building"), "bridge" | "crane" | "railway");
    Ok(vec![Outcome::num("required", "Fatigue verification required (1 = yes)", "Ermüdungsnachweis erforderlich (1 = ja)", if required { 1.0 } else { 0.0 }, c.so("structure", "building").to_string())])
}

pub fn f02(c: &Case) -> R {
    let (b, h, d, a_s) = (c.pos("b")?, c.pos("h")?, c.pos("d")?, c.pos("as")?);
    let ae = c.pos("alpha_e")?;
    let (mmax, mmin) = (c.n("m_max")?, c.n("m_min")?);
    let (_, i2, _, s1, _) = cracked_state_pub(b, h, d, a_s, ae, mmax).ok_or("section uncracked at M_max")?;
    let (_, _, _, s0, _) = cracked_state_pub(b, h, d, a_s, ae, mmin.max(1.0)).unwrap_or((0.0, i2, 0.0, 0.0, 0.0));
    let eta = match (c.opt("ap"), c.opt("xi"), c.opt("phi_s"), c.opt("phi_p")) {
        (Some(ap), Some(xi), Some(ps), Some(pp)) => (a_s + ap) / (a_s + ap * (xi * ps / pp).sqrt()),
        _ => 1.0,
    };
    let range = eta * (s1 - if mmin > 0.0 { s0 } else { 0.0 });
    Ok(vec![Outcome::mpa("delta_sigma_s", "Steel stress range Δσ_s", "Spannungsschwingbreite Δσ_s", range, format!("η = {}", f(eta)))])
}

fn cracked_state_pub(b: f64, h: f64, d: f64, a_s: f64, ae: f64, m: f64) -> Option<(f64, f64, f64, f64, f64)> {
    super::serviceability::cracked_state(b, h, d, a_s, 0.1 * h, 0.0, ae, m, 0.0)
}

pub fn f03(c: &Case) -> R {
    let (n, k1, k2, ds) = sn_curve(c.annex, c.s("kind")?).ok_or("unknown S-N curve kind")?;
    Ok(vec![Outcome::num("n_star", "N*", "N*", n, "cycles".into()), Outcome::num("k1", "Exponent k1", "Exponent k1", k1, "".into()), Outcome::num("k2", "Exponent k2", "Exponent k2", k2, "".into()), Outcome::mpa("dsigma_rsk", "Δσ_Rsk(N*)", "Δσ_Rsk(N*)", ds, "Table 6.3N / 6.4N".into())])
}

pub fn f04(c: &Case) -> R {
    let (n, k1, k2, ds) = sn_curve(c.annex, c.so("kind", "straight")).ok_or("unknown S-N curve kind")?;
    let gs = c.ndp("ndp.en1992.gamma-s-fat");
    let rd = ds / gs;
    let mut v = Vec::new();
    if let Some(eq) = c.opt("delta_sigma_equ") {
        v.push(Outcome::stress("util_equ", "Damage-equivalent stress range", "Schadensäquivalente Schwingbreite", c.o("gamma_f_fat", 1.0) * eq, rd, format!("Δσ_Rsk/γ_s,fat = {}", f(rd))));
    }
    if let (Ok(ranges), Ok(cycles)) = (c.l("ranges"), c.l("cycles")) {
        let d: f64 = ranges.iter().zip(cycles.iter()).map(|(s, n_i)| {
            let big_n = if *s >= rd { n * (rd / s).powf(k1) } else { n * (rd / s).powf(k2) };
            n_i / big_n
        }).sum();
        v.push(Outcome::ratio("damage", "Palmgren-Miner damage D_Ed", "Palmgren-Miner-Schädigung D_Ed", d, 1.0, "Σ n_i/N_i ≤ 1".into()));
    }
    if v.is_empty() { return Err("missing delta_sigma_equ or ranges with cycles".into()); }
    Ok(v)
}

pub fn f05(c: &Case) -> R {
    let (fck, smax, smin) = (c.pos("fck")?, c.pos("sigma_c_max")?, c.n("sigma_c_min")?);
    let fcd = f_cd(c.annex, fck, false);
    let b = c.o("beta_cc_t0", 1.0);
    let fcd_fat = 0.85 * b * fcd * (1.0 - fck / 250.0);
    let emax = smax / fcd_fat;
    let req = smin / smax;
    let lhs = emax + 0.43 * (1.0 - req).max(0.0).sqrt();
    let cap = if fck <= 50.0 { 0.9 } else { 0.8 };
    Ok(vec![Outcome::ratio("util", "Concrete fatigue (Eq. 6.72)", "Betonermüdung (Gl. 6.72)", lhs, 1.0, format!("f_cd,fat = {}", f(fcd_fat))), Outcome::ratio("util_simple", "Concrete fatigue, simplified", "Betonermüdung, vereinfacht", smax / fcd_fat, (0.5 + 0.45 * smin / fcd_fat).min(cap), "σ_max/f_cd,fat ≤ 0.5 + 0.45 σ_min/f_cd,fat".into())])
}

pub fn f06(c: &Case) -> R {
    let (vmax, vmin, vrdc) = (c.pos("v_max")?, c.n("v_min")?, c.pos("v_rd_c")?);
    let cap = if c.o("fck", 30.0) <= 50.0 { 0.9 } else { 0.8 };
    let mut v = vec![Outcome::ratio("util", "Shear fatigue without links", "Querkraftermüdung ohne Bügel", vmax / vrdc, (0.5 + 0.45 * vmin / vrdc).min(cap), "V_max/V_Rd,c ≤ 0.5 + 0.45 V_min/V_Rd,c ≤ 0.9".into())];
    if let Some(cot) = c.opt("cot_theta") { v.push(Outcome::num("cot_fat", "Fatigue strut inclination cot θ_fat", "Druckstrebenneigung Ermüdung cot θ_fat", cot.sqrt().min(1.5), "√cot θ ≤ 1.5".into())); }
    Ok(v)
}

pub fn f07(c: &Case) -> R {
    let (l1, l2, l3, l4) = (c.o("lambda1", 1.0), c.o("lambda2", 1.0), c.o("lambda3", 1.0), c.o("lambda4", 1.0));
    let lam = (l1 * l2 * l3 * l4).min(c.o("lambda_max", 1.0e9));
    let ds71 = c.pos("delta_sigma_71")?;
    let equ = c.o("phi_fat", 1.0) * ds71 * lam;
    let (_, _, _, ds) = sn_curve(c.annex, c.so("kind", "straight")).ok_or("unknown S-N curve kind")?;
    Ok(vec![Outcome::num("lambda_s", "λ_s", "λ_s", lam, "λ_s,1 λ_s,2 λ_s,3 λ_s,4".into()), Outcome::stress("util", "Equivalent stress range", "Äquivalente Schwingbreite", equ, ds / c.ndp("ndp.en1992.gamma-s-fat"), "φ_fat Δσ_s,71 λ_s ≤ Δσ_Rsk/γ_s,fat".into())])
}

/// 🗂 Entries of this domain.
pub const ENTRIES: &[Entry] = &[
    entry_of("PR-01", "DIN EN 1992-1-1", "5.10.2.1", "Maximum stressing force", "Höchstkraft beim Spannen", &["ap", "fpk", "fp01k", "sigma_jack"], &["ndp.en1992.p-k1", "ndp.en1992.p-k2"], pr01),
    entry_of("PR-02", "DIN EN 1992-1-1", "5.10.3", "Initial prestress force", "Anfangsspannkraft", &["ap", "fpk", "fp01k", "pm0"], &["ndp.en1992.p-k7", "ndp.en1992.p-k8"], pr02),
    entry_of("PR-03", "DIN EN 1992-1-1", "5.10.4", "Immediate losses, elastic shortening", "Sofortige Verluste, elastische Verkürzung", &["ap", "ep", "ecm_t", "delta_sigma_c", "system", "n_tendons"], &[], pr03),
    entry_of("PR-04", "DIN EN 1992-1-1", "5.10.5", "Friction and anchorage slip", "Reibung und Verankerungsschlupf", &["pmax", "mu", "k", "x", "theta", "slip", "ap", "radius_m", "ep"], &["ndp.en1992.friction-mu", "ndp.en1992.friction-k"], pr04),
    entry_of("PR-05", "DIN EN 1992-1-1", "5.10.6", "Time-dependent losses", "Zeitabhängige Verluste", &["ep", "ecm", "phi", "ap", "ac", "ic", "zcp", "eps_cs", "sigma_c_qp", "sigma_pi", "fpk", "relax_class", "rho1000", "t_hours"], &["ndp.en1992.sigma-p-qp"], pr05),
    entry_of("PR-06", "DIN EN 1992-1-1", "5.10.8", "Unbonded tendons at ULS", "Spannglieder ohne Verbund im GZT", &["sigma_pm_inf", "fpk", "fp01k", "delta_sigma"], &["ndp.en1992.unbonded-dsigma", "ndp.en1992.gamma-s"], pr06),
    entry_of("PR-07", "DIN EN 1992-1-1", "5.10.9", "Characteristic prestress values", "Charakteristische Vorspannwerte", &["pm_t", "bond"], &["ndp.en1992.p-r-sup", "ndp.en1992.p-r-inf", "ndp.en1992.p-r-sup-unbonded", "ndp.en1992.p-r-inf-unbonded"], pr07),
    entry_of("PR-08", "DIN EN 1992-1-1", "5.10.10", "Deviation forces", "Umlenkkräfte", &["p", "sag", "l", "radius"], &[], pr08),
    entry_of("PR-09", "DIN EN 1992-1-1", "8.10.3", "Anchorage zones of post-tensioned members", "Verankerungsbereiche", &["pmax", "a", "h", "fyk", "as_provided"], &["ndp.en1992.gamma-s"], pr09),
    entry_of("PR-10", "DIN EN 1992-1-1", "5.10.2.2", "Concrete stress at release", "Betonspannung bei Vorspannung", &["sigma_c", "fck_t", "nonlinear_creep_proven"], &[], pr10),
    entry_of("A-16", "DIN EN 1992-1-1", "5.10", "Prestressed members, analysis stress limits", "Vorgespannte Bauteile, Spannungsgrenzen", &["ap", "fpk", "fp01k", "pm0"], &["ndp.en1992.p-k7", "ndp.en1992.p-k8"], pr02),
    entry_of("F-01", "DIN EN 1992-1-1", "6.8.1", "Fatigue verification framework", "Ermüdungsnachweis Ablauf", &["structure"], &[], f01),
    entry_of("F-02", "DIN EN 1992-1-1", "6.8.3", "Stress ranges for fatigue", "Spannungsschwingbreiten", &["b", "h", "d", "as", "alpha_e", "m_max", "m_min", "ap", "xi", "phi_s", "phi_p"], &[], f02),
    entry_of("F-03", "DIN EN 1992-1-1", "6.8.4", "S-N curves of steel", "Wöhlerlinien Stahl", &["kind"], &["ndp.en1992.fatigue-straight", "ndp.en1992.fatigue-welded", "ndp.en1992.fatigue-coupler"], f03),
    entry_of("F-04", "DIN EN 1992-1-1", "6.8.5", "Fatigue verification of steel", "Ermüdungsnachweis Stahl", &["kind", "delta_sigma_equ", "gamma_f_fat", "ranges", "cycles"], &["ndp.en1992.fatigue-straight", "ndp.en1992.fatigue-welded", "ndp.en1992.fatigue-coupler", "ndp.en1992.gamma-s-fat"], f04),
    entry_of("F-05", "DIN EN 1992-1-1", "6.8.7", "Concrete compression fatigue", "Betonermüdung Druck", &["fck", "sigma_c_max", "sigma_c_min", "beta_cc_t0"], &["ndp.en1992.m-01.alpha-cc-persistent", "ndp.en1992.m-01.gamma-c-persistent"], f05),
    entry_of("F-06", "DIN EN 1992-1-1", "6.8.7(2)", "Shear fatigue", "Querkraftermüdung", &["fck", "v_max", "v_min", "v_rd_c", "cot_theta"], &[], f06),
    entry_of("F-07", "DIN EN 1992-2", "6.8.5", "Damage-equivalent lambda method", "Lambda-Verfahren", &["lambda1", "lambda2", "lambda3", "lambda4", "lambda_max", "delta_sigma_71", "phi_fat", "kind"], &["ndp.en1992.fatigue-straight", "ndp.en1992.gamma-s-fat"], f07),
];
