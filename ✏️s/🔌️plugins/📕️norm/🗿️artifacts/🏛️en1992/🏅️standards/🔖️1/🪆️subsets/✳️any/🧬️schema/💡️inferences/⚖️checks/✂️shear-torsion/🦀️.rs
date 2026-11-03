//! ✂️ Shear and torsion of EN 1992-1-1 §6.2 and §6.3 as catalogue entries V-01 … V-08 and T-01 … T-04, including the NA-DE strut inclination and the variable strut optimisation. See [`super`].

use super::*;
/// 📐 Shear resistance without shear reinforcement (Eq. 6.2a/b); returns (V_Rd,c in N, v_Rd,c, v_min in MPa, k, ρ_l).
pub fn v_rd_c(annex: AnnexChoice, fck: f64, bw: f64, d: f64, asl: f64, sigma_cp: f64) -> (f64, f64, f64, f64, f64) {
    let gc = ndp(annex, "ndp.en1992.m-01.gamma-c-persistent");
    let k = (1.0 + (200.0 / d).sqrt()).min(2.0);
    let rho = (asl / (bw * d)).min(0.02);
    let fcd = f_cd(annex, fck, false);
    let scp = sigma_cp.clamp(f64::MIN, 0.2 * fcd);
    let crd = ndp(annex, "ndp.en1992.crd-c-shear") / gc;
    let vmin_c = vmin_coeff(annex, d, gc);
    let vmin = vmin_c * k.powf(1.5) * fck.sqrt();
    let k1 = ndp(annex, "ndp.en1992.k1-shear");
    let v1 = crd * k * (100.0 * rho * fck).powf(1.0 / 3.0) + k1 * scp;
    let v2 = vmin + k1 * scp;
    let v = v1.max(v2);
    (v * bw * d, v, vmin, k, rho)
}

/// 🔢 Coefficient of v_min: EC 0.035; DE 0.0525/γ_c for d ≤ 600 mm, 0.0375/γ_c for d > 800 mm, linear between.
pub fn vmin_coeff(annex: AnnexChoice, d: f64, gc: f64) -> f64 {
    match annex {
        AnnexChoice::En => ndp(annex, "ndp.en1992.vmin-shear"),
        AnnexChoice::De => interp(&[(600.0, ndp(annex, "ndp.en1992.vmin-shear") / gc), (800.0, ndp(annex, "ndp.en1992.vmin-shear-d800") / gc)], d),
    }
}

/// 💢 Strut strength reduction factor ν1.
pub fn nu1(annex: AnnexChoice, fck: f64) -> f64 {
    match annex {
        AnnexChoice::En => 0.6 * (1.0 - fck / 250.0),
        AnnexChoice::De => ndp(annex, "ndp.en1992.nu1-factor") * (ndp(annex, "ndp.en1992.nu2-offset") - fck / ndp(annex, "ndp.en1992.nu2-fck-divisor")).min(1.0),
    }
}

/// ✂️ Upper limit of the shear force without links V_Ed ≤ 0.5 b_w d ν f_cd (Eq. 6.5); any consistent unit system.
pub fn v_ed_max_unreinforced(bw: f64, d: f64, nu: f64, fcd: f64) -> f64 {
    0.5 * bw * d * nu * fcd
}

/// 💢 Strut strength reduction factor ν_1 = 0.6 for f_ck ≤ 60 MPa, otherwise max(0.9 − f_ck/200, 0.5) (Eq. 6.10aN/bN), f_ck in MPa.
pub fn nu1_strut(fck: f64) -> f64 {
    if fck <= 60.0 { 0.6 } else { (0.9 - fck / 200.0).max(0.5) }
}

/// 🧱 Upper limit of the shear link area A_sw,max = 0.5 α_cw ν_1 f_cd b_w s/f_ywd (Eq. 9.6N) with ν_1 of Eq. 6.6N; any consistent unit system.
pub fn a_sw_max(annex: AnnexChoice, fck: f64, fcd: f64, sigma_cp: f64, bw: f64, s: f64, fywd: f64) -> f64 {
    0.5 * alpha_cw(annex, sigma_cp, fcd) * nu1(annex, fck) * fcd * bw * s / fywd
}

/// 🌀 Shear flow of a torsion wall τ_t,i t_ef,i = T_Ed/(2 A_k) (Eq. 6.26); N/m for N·m and m².
pub fn torsion_shear_flow(t_ed: f64, a_k: f64) -> f64 {
    t_ed / (2.0 * a_k)
}

/// 🌀 Longitudinal torsion reinforcement ΣA_sl = T_Ed u_k cot θ/(2 A_k f_yd) (Eq. 6.28).
pub fn torsion_longitudinal(t_ed: f64, u_k: f64, a_k: f64, f_yd: f64, cot: f64) -> f64 {
    t_ed * u_k * cot / (2.0 * a_k * f_yd)
}

/// 🌀 Torsion resistance T_Rd,max = 2 ν α_cw f_cd A_k t_ef sin θ cos θ (Eq. 6.30) with θ in radians.
pub fn t_rd_max(nu: f64, acw: f64, fcd: f64, a_k: f64, t_ef: f64, theta: f64) -> f64 {
    2.0 * nu * acw * fcd * a_k * t_ef * theta.sin() * theta.cos()
}

/// 🧱 Coefficient α_cw (6.2.3(3)); the NA-DE value is 1.0.
pub fn alpha_cw(annex: AnnexChoice, sigma_cp: f64, fcd: f64) -> f64 {
    if annex == AnnexChoice::De { return ndp(annex, "ndp.en1992.alpha-cw"); }
    let r = sigma_cp / fcd;
    if r <= 0.0 { 1.0 } else if r <= 0.25 { 1.0 + r } else if r <= 0.5 { 1.25 } else { 2.5 * (1.0 - r) }
}

/// 📏 Largest admissible cot θ: EC 2.5; DE min(3.0, (1.2 + 1.4 σ_cp/f_cd)/(1 − V_Rd,cc/V_Ed)).
pub fn cot_theta_max(annex: AnnexChoice, fck: f64, bw: f64, z: f64, sigma_cp: f64, fcd: f64, v_ed: f64) -> f64 {
    match annex {
        AnnexChoice::En => ndp(annex, "ndp.en1992.cot-theta-max"),
        AnnexChoice::De => {
            let vcc = ndp(annex, "ndp.en1992.vrdcc-c") * ndp(annex, "ndp.en1992.vrdcc-k") * fck.powf(1.0 / 3.0) * (1.0 - 1.2 * sigma_cp / fcd) * bw * z;
            let num = ndp(annex, "ndp.en1992.cot-theta-a") + ndp(annex, "ndp.en1992.cot-theta-b") * sigma_cp / fcd;
            let den = 1.0 - vcc / v_ed;
            if den <= 0.0 { ndp(annex, "ndp.en1992.cot-theta-max") } else { (num / den).min(ndp(annex, "ndp.en1992.cot-theta-max")) }
        }
    }
}

/// ⚖️ V_Rd,max for vertical (cotα = 0) or inclined links.
pub fn v_rd_max(acw: f64, bw: f64, z: f64, nu: f64, fcd: f64, cot: f64, cot_alpha: f64) -> f64 {
    acw * bw * z * nu * fcd * (cot + cot_alpha) / (1.0 + cot * cot)
}

/// ➰ V_Rd,s for links with inclination α (Asw/s in mm²/mm).
pub fn v_rd_s(asw_s: f64, z: f64, fywd: f64, cot: f64, alpha: f64) -> f64 {
    asw_s * z * fywd * (cot + 1.0 / alpha.tan()) * alpha.sin()
}

/// 🎯 Largest cot θ in [1, cot_max] that keeps V_Rd,max ≥ V_Ed, `None` when even cot θ = 1 fails.
pub fn cot_theta_optimum(v_ed: f64, vmax_at_one: f64, cot_max: f64, acw_bw_z_nu_fcd: f64, cot_alpha: f64) -> Option<f64> {
    if v_ed > vmax_at_one { return None; }
    let (mut lo, mut hi) = (1.0f64, cot_max.max(1.0));
    if v_rd_max(acw_bw_z_nu_fcd, 1.0, 1.0, 1.0, 1.0, hi, cot_alpha) >= v_ed { return Some(hi); }
    for _ in 0..80 {
        let mid = 0.5 * (lo + hi);
        if v_rd_max(acw_bw_z_nu_fcd, 1.0, 1.0, 1.0, 1.0, mid, cot_alpha) >= v_ed { lo = mid } else { hi = mid }
    }
    Some(lo)
}

pub struct Shear {
    fcd: f64,
    fywd: f64,
    nu: f64,
    acw: f64,
    z: f64,
    bw: f64,
    sigma_cp: f64,
    alpha: f64,
    fck: f64,
}

pub fn shear_basis(c: &Case) -> Result<Shear, String> {
    let (fck, bw) = (c.pos("fck")?, c.pos("bw")?);
    let d = c.opt("d");
    let z = match c.opt("z") { Some(z) => z, None => 0.9 * d.ok_or("missing input z or d")? };
    let acc = c.so("situation", "persistent") == "accidental";
    let fcd = f_cd(c.annex, fck, acc);
    let sigma_cp = c.o("sigma_cp", 0.0);
    Ok(Shear { fcd, fywd: f_yd(c.annex, c.o("fyk", 500.0), acc), nu: nu1(c.annex, fck), acw: alpha_cw(c.annex, sigma_cp, fcd), z, bw, sigma_cp, alpha: c.o("alpha_deg", 90.0).to_radians(), fck })
}

pub fn v01(c: &Case) -> R {
    let (av, d, ved) = (c.pos("av")?, c.pos("d")?, c.pos("v_ed")?);
    let r = av / (2.0 * d);
    let direct = c.o("direct_support", 1.0) > 0.0;
    let min_ratio = if c.de() { 0.5 } else { 0.0 };
    let beta = if direct && av <= 2.0 * d && av / d >= min_ratio { r.max(0.25) } else { 1.0 };
    Ok(vec![Outcome::num("beta", "Reduction factor β", "Abminderungsfaktor β", beta, format!("a_v/(2d) = {}", f(r))), Outcome::n("v_ed_red", "Reduced shear V_Ed,red", "Abgeminderte Querkraft V_Ed,red", beta * ved, "β V_Ed".into())])
}

pub fn v02(c: &Case) -> R {
    let (fck, bw, d, asl) = (c.pos("fck")?, c.pos("bw")?, c.pos("d")?, c.pos("asl")?);
    let acp = c.opt("ac").unwrap_or(bw * c.o("h", d / 0.9));
    let scp = c.opt("sigma_cp").unwrap_or_else(|| c.o("n_ed", 0.0) / acp);
    let (vrdc, v, vmin, k, rho) = v_rd_c(c.annex, fck, bw, d, asl, scp);
    let fcd = f_cd(c.annex, fck, false);
    let cap = v_ed_max_unreinforced(bw, d, nu1(c.annex, fck).max(0.6 * (1.0 - fck / 250.0)), fcd);
    let mut out = vec![
        Outcome::n("v_rd_c", "V_Rd,c", "V_Rd,c", vrdc.min(cap), format!("v_Rd,c = {} MPa, v_min = {}, k = {}, ρ_l = {}", f(v), f(vmin), f(k), f(rho))),
        Outcome::n("v_rd_c_uncapped", "V_Rd,c before the upper limit", "V_Rd,c vor der oberen Grenze", vrdc, "Eq. 6.2".into()),
    ];
    if let Some(ved) = c.opt("v_ed") { out.push(Outcome::force("util", "Shear utilisation without links", "Querkraftausnutzung ohne Bügel", ved, vrdc.min(cap), "V_Ed ≤ V_Rd,c".into())); }
    if let (Some(ib), Some(s), Some(fctd)) = (c.opt("i_b"), c.opt("s_first"), c.opt("fctd")) {
        let al = c.o("alpha_l", 1.0).min(1.0);
        let v = ib * bw / s * (fctd * fctd + al * (-scp).max(0.0) * fctd).sqrt();
        out.push(Outcome::n("v_rd_c_uncracked", "V_Rd,c uncracked prestressed (Eq. 6.4)", "V_Rd,c ungerissen vorgespannt (Gl. 6.4)", v, "I b_w/S sqrt(f_ctd² + α_l σ_cp f_ctd)".into()));
    }
    Ok(out)
}

pub fn v03(c: &Case) -> R {
    let s = shear_basis(c)?;
    let ved = c.pos("v_ed")?;
    let cot_a = 1.0 / s.alpha.tan();
    let cmax = cot_theta_max(c.annex, s.fck, s.bw, s.z, s.sigma_cp, s.fcd, ved);
    let k = s.acw * s.bw * s.z * s.nu * s.fcd;
    let vmax1 = v_rd_max(k, 1.0, 1.0, 1.0, 1.0, 1.0, cot_a);
    let opt = cot_theta_optimum(ved, vmax1, cmax, k, cot_a);
    let cot = c.opt("cot_theta").unwrap_or_else(|| opt.unwrap_or(1.0)).max(1.0).min(cmax.max(1.0));
    let asw_req = ved / (s.z * s.fywd * (cot + cot_a) * s.alpha.sin());
    let vmax = v_rd_max(k, 1.0, 1.0, 1.0, 1.0, cot, cot_a);
    let mut out = vec![
        Outcome::num("cot_theta", "Strut inclination cot θ", "Druckstrebenneigung cot θ", cot, format!("limit {} (annex {:?})", f(cmax), c.annex)),
        Outcome::n("v_rd_max", "V_Rd,max", "V_Rd,max", vmax, format!("ν1 = {}, α_cw = {}", f(s.nu), f(s.acw))),
        Outcome::num("asw_req", "Required A_sw/s (mm²/mm)", "Erforderliches A_sw/s (mm²/mm)", asw_req, "V_Ed/(z f_ywd (cot θ + cot α) sin α)".into()),
        Outcome::force("util_max", "Strut crushing utilisation", "Druckstrebenausnutzung", ved, vmax, "V_Ed ≤ V_Rd,max".into()),
        Outcome::n("dftd", "Additional tension force ΔF_td", "Zusätzliche Zugkraft ΔF_td", 0.5 * ved * (cot - cot_a), "0.5 V_Ed (cot θ − cot α)".into()),
        Outcome::mm("shift_al", "Shift a_l", "Versatzmaß a_l", s.z * (cot - cot_a) / 2.0, "z (cot θ − cot α)/2".into()),
    ];
    if opt.is_none() { out.push(Outcome::num("crushing_at_limit", "V_Ed exceeds V_Rd,max even at cot θ = 1 (1 = yes)", "V_Ed überschreitet V_Rd,max selbst bei cot θ = 1 (1 = ja)", 1.0, "increase the section or the concrete grade".into())); }
    if let Some(asw) = c.opt("asw_s") {
        let vs = v_rd_s(asw, s.z, s.fywd, cot, s.alpha);
        out.push(Outcome::force("util_s", "Link utilisation", "Bügelausnutzung", ved, vs, format!("V_Rd,s = {}", f(vs))));
        let fctm = conc(s.fck).fctm;
        let rho_min = if c.de() { 0.16 * fctm / c.o("fyk", 500.0) } else { 0.08 * s.fck.sqrt() / c.o("fyk", 500.0) };
        out.push(Outcome::ratio_min("rho_w", "Shear reinforcement ratio ρ_w", "Schubbewehrungsgrad ρ_w", asw / (s.bw * s.alpha.sin()), rho_min, "ρ_w ≥ 0.16 f_ctm/f_yk".into()));
        if let Some(d) = c.opt("d") { out.push(Outcome::length("s_l_max", "Maximum longitudinal link spacing", "Größter Bügelabstand", c.o("s", 0.0), 0.75 * d * (1.0 + cot_a), "s_l,max = 0.75 d (1 + cot α)".into())); }
    }
    Ok(out)
}

pub fn v04(c: &Case) -> R {
    let (fck, scp) = (c.pos("fck")?, c.n("sigma_cp")?);
    let fcd = f_cd(c.annex, fck, false);
    let mut out = vec![Outcome::num("alpha_cw", "α_cw", "α_cw", alpha_cw(c.annex, scp, fcd), format!("σ_cp/f_cd = {}", f(scp / fcd)))];
    if let (Some(v0), Some(vccd), Some(vtd)) = (c.opt("v_ed0"), c.opt("v_ccd"), c.opt("v_td")) {
        out.push(Outcome::n("v_ed", "Net shear V_Ed", "Netto-Querkraft V_Ed", v0 - vccd - vtd, "V_Ed = V_Ed0 − V_ccd − V_td".into()));
    }
    Ok(out)
}

/// 🪢 Roughness coefficients (c, μ) of 6.2.5(2).
pub fn interface_cm(roughness: &str) -> (f64, f64) {
    match roughness { "very_smooth" => (0.025, 0.5), "smooth" => (0.20, 0.6), "indented" => (0.50, 0.9), _ => (0.40, 0.7) }
}

pub fn v05(c: &Case) -> R {
    let (fck, bi, z, ved) = (c.pos("fck")?, c.pos("bi")?, c.pos("z")?, c.pos("v_ed")?);
    let beta = c.o("beta", 1.0);
    let (mut cc, mu) = interface_cm(c.so("roughness", "rough"));
    if c.o("fatigue", 0.0) > 0.0 { cc = 0.0; }
    let sn = c.o("sigma_n", 0.0);
    let fcd = f_cd(c.annex, fck, false);
    let nu = 0.6 * (1.0 - fck / 250.0);
    let fctd = f_ctd(c.annex, fck);
    let alpha = c.o("alpha_deg", 90.0).to_radians();
    let rho = c.o("rho", 0.0);
    let fyd = f_yd(c.annex, c.o("fyk", 500.0), false);
    let cterm = if sn < 0.0 { 0.0 } else { cc * fctd };
    let vrdi = (cterm + mu * sn.clamp(f64::MIN, 0.6 * fcd) + rho * fyd * (mu * alpha.sin() + alpha.cos())).min(0.5 * nu * fcd);
    let vedi = beta * ved / (z * bi);
    Ok(vec![Outcome::stress("util", "Interface shear utilisation", "Schubfugenausnutzung", vedi, vrdi, format!("c = {}, μ = {}", f(cc), f(mu))), Outcome::mpa("v_rdi", "v_Rdi", "v_Rdi", vrdi, "Eq. 6.25".into())])
}

pub fn v06(c: &Case) -> R {
    let (hf, dfd, dx) = (c.pos("hf")?, c.pos("delta_fd")?, c.pos("delta_x")?);
    let fck = c.pos("fck")?;
    let cot = c.o("cot_theta_f", 1.2).clamp(1.0, if c.o("tension_flange", 0.0) > 0.0 { 1.25 } else { 2.0 });
    let fyd = f_yd(c.annex, c.o("fyk", 500.0), false);
    let ved = dfd / (hf * dx);
    let fcd = f_cd(c.annex, fck, false);
    let nu = 0.6 * (1.0 - fck / 250.0);
    let th = (1.0 / cot).atan();
    Ok(vec![
        Outcome::mpa("v_ed", "Longitudinal shear stress v_Ed", "Längsschubspannung v_Ed", ved, "ΔF_d/(h_f Δx)".into()),
        Outcome::num("asf_sf", "Required A_sf/s_f (mm²/mm)", "Erforderliches A_sf/s_f (mm²/mm)", ved * hf / (fyd * cot), "v_Ed h_f/(f_yd cot θ_f)".into()),
        Outcome::stress("util_crush", "Flange crushing utilisation", "Gurtdruckausnutzung", ved, nu * fcd * th.sin() * th.cos(), "v_Ed ≤ ν f_cd sin θ_f cos θ_f".into()),
    ])
}

pub fn v07(c: &Case) -> R {
    let (fck, bw, d, asl, av, ved) = (c.pos("fck")?, c.pos("bw")?, c.pos("d")?, c.pos("asl")?, c.pos("av")?, c.pos("v_ed")?);
    let beta = if av <= 2.0 * d && (!c.de() || av / d >= 0.5) { (av / (2.0 * d)).max(0.25) } else { 1.0 };
    let (vrdc, ..) = v_rd_c(c.annex, fck, bw, d, asl, c.o("sigma_cp", 0.0));
    let nu = 0.6 * (1.0 - fck / 250.0);
    let cap = 0.5 * bw * d * nu * f_cd(c.annex, fck, false);
    Ok(vec![
        Outcome::num("beta", "β", "β", beta, "a_v/(2d) ≥ 0.25".into()),
        Outcome::force("util_red", "Reduced shear utilisation", "Ausnutzung abgeminderte Querkraft", beta * ved, vrdc, "β V_Ed ≤ V_Rd,c".into()),
        Outcome::force("util_cap", "Upper shear limit utilisation", "Ausnutzung obere Querkraftgrenze", ved, cap, "V_Ed ≤ 0.5 b_w d ν f_cd".into()),
    ])
}

/// 📦 Equivalent thin-walled section of a polygon (A, u, t_ef, A_k, u_k) with minimum wall `t_min` (2(c + φ_l/2)): t_ef = max(A/u, t_min).
pub fn thin_walled(pts: &[(f64, f64)], t_min: f64) -> (f64, f64, f64, f64, f64) {
    let n = pts.len();
    let (mut a, mut u) = (0.0, 0.0);
    for i in 0..n {
        let (p, q) = (pts[i], pts[(i + 1) % n]);
        a += p.0 * q.1 - q.0 * p.1;
        u += ((q.0 - p.0).powi(2) + (q.1 - p.1).powi(2)).sqrt();
    }
    thin_walled_t(pts, (a.abs() / 2.0 / u).max(t_min))
}

/// 📦 The thin-walled section of a simple polygon for the wall `t_ef` (A, u, t_ef, A_k, u_k): the centre line is the outline moved inwards by t_ef/2, so A_k = A − u d + d² Σ cot(θ_i/2) and u_k = u − 2 d Σ cot(θ_i/2) with d = t_ef/2 and θ_i the interior angle, which exceeds 180° at re-entrant corners (cot < 0).
pub fn thin_walled_t(pts: &[(f64, f64)], t_ef: f64) -> (f64, f64, f64, f64, f64) {
    let n = pts.len();
    let (mut a, mut u) = (0.0, 0.0);
    for i in 0..n {
        let (p, q) = (pts[i], pts[(i + 1) % n]);
        a += p.0 * q.1 - q.0 * p.1;
        u += ((q.0 - p.0).powi(2) + (q.1 - p.1).powi(2)).sqrt();
    }
    let orientation = if a < 0.0 { -1.0 } else { 1.0 };
    let a = a.abs() / 2.0;
    let mut cot_sum = 0.0;
    for i in 0..n {
        let (p, q, r) = (pts[(i + n - 1) % n], pts[i], pts[(i + 1) % n]);
        let (v1, v2) = ((p.0 - q.0, p.1 - q.1), (r.0 - q.0, r.1 - q.1));
        let turn = orientation * (v2.0 * v1.1 - v2.1 * v1.0).atan2(v1.0 * v2.0 + v1.1 * v2.1);
        let interior = if turn < 0.0 { turn + std::f64::consts::TAU } else { turn };
        cot_sum += 1.0 / (interior / 2.0).tan();
    }
    let d = t_ef / 2.0;
    (a, u, t_ef, a - u * d + d * d * cot_sum, u - 2.0 * d * cot_sum)
}

pub struct Tors {
    ak: f64,
    uk: f64,
    tef: f64,
}

/// 🕳️ Wall thickness of a hollow section estimated from the net area and the mean circumference of the outer and the hole loops, which limits t_ef (6.3.1(1)).
pub fn hollow_wall(outer: &[(f64, f64)], holes: &[&[(f64, f64)]]) -> f64 {
    let measure = |p: &[(f64, f64)]| -> (f64, f64) {
        let n = p.len();
        (0..n).fold((0.0, 0.0), |t, i| {
            let (a, b) = (p[i], p[(i + 1) % n]);
            (t.0 + 0.5 * (a.0 * b.1 - b.0 * a.1), t.1 + (b.0 - a.0).hypot(b.1 - a.1))
        })
    };
    let (ao, uo) = measure(outer);
    let (ah, uh) = holes.iter().map(|h| measure(*h)).fold((0.0, 0.0), |t, m| (t.0 + m.0.abs(), t.1 + m.1));
    if holes.is_empty() { f64::INFINITY } else { (ao.abs() - ah) / (0.5 * (uo + uh)) }
}

pub fn tors_geom(c: &Case) -> Result<Tors, String> {
    let tmin = 2.0 * (c.o("c", 0.0) + c.o("phi_l", 0.0) / 2.0);
    let ring = |y: &str, z: &str| c.l(y).ok().zip(c.l(z).ok()).map(|(y, z)| y.iter().copied().zip(z.iter().copied()).collect::<Vec<(f64, f64)>>());
    let pts: Vec<(f64, f64)> = if let Some(outer) = ring("poly_y", "poly_z") {
        outer
    } else {
        let (b, h) = (c.pos("b")?, c.pos("h")?);
        vec![(0.0, 0.0), (b, 0.0), (b, h), (0.0, h)]
    };
    let hole = ring("hole_y", "hole_z");
    let wall = hole.as_ref().map_or(f64::INFINITY, |h| hollow_wall(&pts, &[h.as_slice()]));
    let (a, u, ..) = thin_walled(&pts, 0.0);
    let (_, _, tef, ak, uk) = thin_walled_t(&pts, (a / u).min(wall).max(tmin));
    Ok(Tors { ak, uk, tef })
}

pub fn t01(c: &Case) -> R {
    let g = tors_geom(c)?;
    Ok(vec![Outcome::mm("tef", "Effective wall thickness t_ef", "Wirksame Wanddicke t_ef", g.tef, "A/u ≥ 2(c + φ_l/2)".into()), Outcome::mm2("ak", "Enclosed area A_k", "Eingeschlossene Fläche A_k", g.ak, "centreline polygon".into()), Outcome::mm("uk", "Perimeter u_k", "Umfang u_k", g.uk, "centreline".into())])
}

pub fn t02(c: &Case) -> R {
    let g = tors_geom(c)?;
    let (fck, ted) = (c.pos("fck")?, c.pos("t_ed")?);
    let fcd = f_cd(c.annex, fck, false);
    let fywd = f_yd(c.annex, c.o("fyk", 500.0), false);
    let cot = c.o("cot_theta", 1.0).max(1.0).min(ndp(c.annex, "ndp.en1992.cot-theta-max"));
    let th = (1.0 / cot).atan();
    let trdmax = 2.0 * nu1(c.annex, fck) * alpha_cw(c.annex, c.o("sigma_cp", 0.0), fcd) * fcd * g.ak * g.tef * th.sin() * th.cos();
    let trdc = 2.0 * g.ak * g.tef * f_ctd(c.annex, fck);
    let mut v = vec![
        Outcome::moment("util", "Torsion crushing utilisation", "Torsionsdruckstrebenausnutzung", ted, trdmax, format!("T_Rd,max = {}", f(trdmax))),
        Outcome::nmm("t_rd_c", "T_Rd,c (no reinforcement needed below)", "T_Rd,c (darunter keine Bewehrung)", trdc, "2 A_k t_ef f_ctd".into()),
        Outcome::num("asw_s", "Required A_sw/s (mm²/mm)", "Erforderliches A_sw/s (mm²/mm)", ted / (2.0 * g.ak * fywd * cot), "T_Ed/(2 A_k f_ywd cot θ)".into()),
        Outcome::mm2("asl", "Required longitudinal steel ΣA_sl", "Erforderliche Längsbewehrung ΣA_sl", ted * g.uk * cot / (2.0 * g.ak * f_yd(c.annex, c.o("fyk", 500.0), false)), "T_Ed u_k cot θ/(2 A_k f_yd)".into()),
        Outcome::mm("s_max", "Largest link spacing u_k/8", "Größter Bügelabstand u_k/8", g.uk / 8.0, "9.2.3(3)".into()),
    ];
    if let Some(prov) = c.opt("asw_s_provided") { v.push(Outcome::area("util_links", "Torsion link utilisation (per mm)", "Torsionsbügelausnutzung (je mm)", ted / (2.0 * g.ak * fywd * cot), prov, "A_sw/s,req ≤ provided".into())); }
    let asl_req = ted * g.uk * cot / (2.0 * g.ak * f_yd(c.annex, c.o("fyk", 500.0), false));
    let share = c.o("tension_share", 1.0).clamp(0.0, 1.0);
    if let Some(prov) = c.opt("asl_provided") { v.push(Outcome::area("util_long", "Tension chord utilisation: bending plus torsion longitudinal steel", "Zuggurtausnutzung: Biege- plus Torsionslängsbewehrung", c.o("asl_bending", 0.0) + share * asl_req, prov, "A_s,bending + share·ΣA_sl,req ≤ provided; torsion steel in addition to bending steel (6.3.2(3))".into())); }
    if let Some(prov) = c.opt("asl_compression_provided") { v.push(Outcome::area("util_long_comp", "Compression chord utilisation: torsion longitudinal steel", "Druckgurtausnutzung: Torsionslängsbewehrung", (1.0 - share) * asl_req, prov, "(1 − share)·ΣA_sl,req ≤ provided; no reduction by the available compression (6.3.2(3))".into())); }
    v.push(Outcome::num("shear_flow", "Torsional shear flow q = T_Ed/(2 A_k) (N/mm)", "Schubfluss q = T_Ed/(2 A_k) (N/mm)", torsion_shear_flow(ted, g.ak), "6.3.1(1)".into()));
    Ok(v)
}

pub fn t03(c: &Case) -> R {
    let ted = c.pos("t_ed")?;
    let ti = c.l("stiffness")?;
    let sum: f64 = ti.iter().sum();
    if sum <= 0.0 { return Err("stiffness sum must be positive".into()); }
    Ok(ti.iter().enumerate().map(|(i, k)| Outcome::nmm(&format!("t_{i}"), &format!("Torsion share {i}"), &format!("Torsionsanteil {i}"), ted * k / sum, "T_Ed T_i/ΣT".into())).collect())
}

pub fn t04(c: &Case) -> R {
    let g = tors_geom(c)?;
    let (fck, ted, ved, bw, d, asl) = (c.pos("fck")?, c.pos("t_ed")?, c.pos("v_ed")?, c.pos("bw")?, c.pos("d")?, c.o("asl", 0.0));
    let fcd = f_cd(c.annex, fck, false);
    let trdc = 2.0 * g.ak * g.tef * f_ctd(c.annex, fck);
    let (vrdc, ..) = v_rd_c(c.annex, fck, bw, d, asl.max(1e-9), 0.0);
    let cot = c.o("cot_theta", 1.0);
    let z = c.o("z", 0.9 * d);
    let vmax = v_rd_max(alpha_cw(c.annex, 0.0, fcd), bw, z, nu1(c.annex, fck), fcd, cot, 0.0);
    let trdmax = 2.0 * nu1(c.annex, fck) * fcd * g.ak * g.tef * (1.0 / cot).atan().sin() * (1.0 / cot).atan().cos();
    let mut v = vec![
        Outcome::ratio("util_min", "Interaction without calculated reinforcement", "Interaktion ohne rechnerische Bewehrung", ted / trdc + ved / vrdc, 1.0, "T_Ed/T_Rd,c + V_Ed/V_Rd,c ≤ 1".into()),
        Outcome::ratio("util_max", "Interaction at the strut limit", "Interaktion an der Druckstrebengrenze", ted / trdmax + ved / vmax, 1.0, "T_Ed/T_Rd,max + V_Ed/V_Rd,max ≤ 1".into()),
    ];
    v.extend([
        Outcome::nmm("t_rd_c", "T_Rd,c", "T_Rd,c", trdc, "2 A_k t_ef f_ctd".into()),
        Outcome::nmm("t_rd_max", "T_Rd,max", "T_Rd,max", trdmax, "6.30".into()),
        Outcome::n("v_rd_c", "V_Rd,c", "V_Rd,c", vrdc, "6.2.2".into()),
        Outcome::n("v_rd_max", "V_Rd,max", "V_Rd,max", vmax, "6.9".into()),
    ]);
    if c.o("box", 0.0) > 0.0 {
        let zi = c.o("z_i", z);
        v.push(Outcome::n("v_ed_t", "Web shear from torsion V_Ed,T", "Stegquerkraft aus Torsion V_Ed,T", ted * zi / (2.0 * g.ak), "T_Ed z_i/(2 A_k)".into()));
    }
    Ok(v)
}

pub fn v09(c: &Case) -> R {
    let (fck, hf, dfd, dx) = (c.pos("fck")?, c.pos("hf")?, c.pos("delta_fd")?, c.pos("delta_x")?);
    let v = dfd / (hf * dx);
    let lim = 0.4 * f_ctd(c.annex, fck);
    Ok(vec![Outcome::stress("util", "Flange shear without links against k f_ctd", "Gurtschub ohne Bügel gegen k f_ctd", v, lim, "v_Ed ≤ k f_ctd, k = 0.4 (6.2.4(6))".into())])
}

/// 🗂 Entries of this domain.
pub const ENTRIES: &[Entry] = &[
    entry_of("V-01", "DIN EN 1992-1-1", "6.2.1", "Shear design framework and support reduction", "Querkraftnachweis, Abminderung am Auflager", &["av", "d", "v_ed", "direct_support"], &[], v01),
    entry_of("V-02", "DIN EN 1992-1-1", "6.2.2", "Shear without shear reinforcement", "Querkraft ohne Querkraftbewehrung", &["fck", "bw", "d", "h", "asl", "n_ed", "sigma_cp", "v_ed"], &["ndp.en1992.crd-c-shear", "ndp.en1992.k1-shear", "ndp.en1992.vmin-shear", "ndp.en1992.vmin-shear-d800", "ndp.en1992.m-01.gamma-c-persistent", "ndp.en1992.m-01.alpha-cc-persistent"], v02),
    entry_of("V-03", "DIN EN 1992-1-1", "6.2.3", "Shear with links, variable strut inclination", "Querkraft mit Bügeln, veränderliche Druckstrebenneigung", &["fck", "bw", "d", "z", "v_ed", "fyk", "asw_s", "s", "alpha_deg", "sigma_cp", "cot_theta"], &["ndp.en1992.cot-theta-max", "ndp.en1992.cot-theta-a", "ndp.en1992.cot-theta-b", "ndp.en1992.vrdcc-c", "ndp.en1992.vrdcc-k", "ndp.en1992.nu1-factor", "ndp.en1992.nu2-offset", "ndp.en1992.nu2-fck-divisor", "ndp.en1992.alpha-cw", "ndp.en1992.m-01.alpha-cc-persistent", "ndp.en1992.m-01.gamma-c-persistent", "ndp.en1992.gamma-s"], v03),
    entry_of("V-04", "DIN EN 1992-1-1", "6.2.3(3)", "Shear with axial force and prestress", "Querkraft mit Normalkraft und Vorspannung", &["fck", "sigma_cp", "v_ed0", "v_ccd", "v_td"], &["ndp.en1992.alpha-cw"], v04),
    entry_of("V-05", "DIN EN 1992-1-1", "6.2.5", "Shear at interfaces", "Schubfuge", &["fck", "bi", "z", "v_ed", "beta", "roughness", "sigma_n", "rho", "alpha_deg", "fyk", "fatigue"], &["ndp.en1992.m-01.alpha-ct", "ndp.en1992.m-01.gamma-c-persistent", "ndp.en1992.gamma-s", "ndp.en1992.m-01.alpha-cc-persistent"], v05),
    entry_of("V-06", "DIN EN 1992-1-1", "6.2.4", "Shear between web and flanges", "Schub zwischen Gurt und Steg", &["fck", "hf", "delta_fd", "delta_x", "cot_theta_f", "tension_flange", "fyk"], &["ndp.en1992.gamma-s", "ndp.en1992.m-01.gamma-c-persistent", "ndp.en1992.m-01.alpha-cc-persistent"], v06),
    entry_of("V-07", "DIN EN 1992-1-1", "6.2.2(6)", "Loads close to supports", "Lasten in Auflagernähe", &["fck", "bw", "d", "asl", "av", "v_ed", "sigma_cp"], &["ndp.en1992.crd-c-shear", "ndp.en1992.k1-shear", "ndp.en1992.vmin-shear", "ndp.en1992.vmin-shear-d800", "ndp.en1992.m-01.gamma-c-persistent", "ndp.en1992.m-01.alpha-cc-persistent"], v07),
    entry_of("V-08", "DIN EN 1992-1-1", "6.3.2(4)", "Combined shear and torsion", "Querkraft und Torsion kombiniert", &["fck", "b", "h", "bw", "d", "asl", "t_ed", "v_ed", "cot_theta", "box"], &["ndp.en1992.m-01.alpha-ct", "ndp.en1992.m-01.gamma-c-persistent", "ndp.en1992.m-01.alpha-cc-persistent", "ndp.en1992.nu1-factor", "ndp.en1992.nu2-offset", "ndp.en1992.nu2-fck-divisor", "ndp.en1992.crd-c-shear", "ndp.en1992.k1-shear", "ndp.en1992.vmin-shear", "ndp.en1992.vmin-shear-d800"], t04),
    entry_of("V-09", "DIN EN 1992-1-1", "6.2.4(6)", "Flange shear without reinforcement", "Gurtschub ohne Bewehrung", &["fck", "hf", "delta_fd", "delta_x"], &["ndp.en1992.m-01.alpha-ct", "ndp.en1992.m-01.gamma-c-persistent"], v09),
    entry_of("T-01", "DIN EN 1992-1-1", "6.3.2(1)", "Equivalent thin-walled section", "Ersatz-Kasten", &["b", "h", "poly_y", "poly_z", "hole_y", "hole_z", "c", "phi_l"], &[], t01),
    entry_of("T-02", "DIN EN 1992-1-1", "6.3.2", "Torsion resistance", "Torsionsbemessung", &["fck", "b", "h", "c", "phi_l", "t_ed", "cot_theta", "fyk", "asw_s_provided", "asl_provided", "asl_bending", "tension_share", "asl_compression_provided"], &["ndp.en1992.m-01.alpha-ct", "ndp.en1992.m-01.gamma-c-persistent", "ndp.en1992.gamma-s", "ndp.en1992.m-01.alpha-cc-persistent", "ndp.en1992.nu1-factor", "ndp.en1992.nu2-offset", "ndp.en1992.nu2-fck-divisor", "ndp.en1992.alpha-cw"], t02),
    entry_of("T-03", "DIN EN 1992-1-1", "6.3.2(4)", "Torsion in composite sections", "Torsion in zusammengesetzten Querschnitten", &["t_ed", "stiffness"], &[], t03),
    entry_of("T-04", "DIN EN 1992-1-1", "6.3.2(103)", "Combined torsion and shear", "Torsion und Querkraft kombiniert", &["fck", "b", "h", "bw", "d", "asl", "t_ed", "v_ed", "cot_theta", "box", "z_i", "z", "poly_y", "poly_z", "hole_y", "hole_z", "c", "phi_l"], &["ndp.en1992.m-01.alpha-ct", "ndp.en1992.m-01.gamma-c-persistent", "ndp.en1992.m-01.alpha-cc-persistent", "ndp.en1992.nu1-factor", "ndp.en1992.nu2-offset", "ndp.en1992.nu2-fck-divisor", "ndp.en1992.crd-c-shear", "ndp.en1992.k1-shear", "ndp.en1992.vmin-shear", "ndp.en1992.vmin-shear-d800"], t04),
];
