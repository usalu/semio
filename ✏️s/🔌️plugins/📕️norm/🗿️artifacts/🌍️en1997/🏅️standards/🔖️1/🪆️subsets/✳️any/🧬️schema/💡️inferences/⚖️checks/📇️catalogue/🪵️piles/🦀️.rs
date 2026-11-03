//! 🪵️ Piles, catalogue entries PILE-01 … PILE-11 (EN 1997-1 §7, Tables A.9-A.11, DIN 1054 §7, EA-Pfähle).

use super::*;

fn registry_pair(table: &str, row: u32, first: u8, annex: AnnexChoice) -> (f64, f64) {
    let id = |k: u8| format!("ndp.en1997.xi.{table}.n{row}.{k}");
    (semio_s_plugin_norm_compliance::ndp_number(&id(first), annex), semio_s_plugin_norm_compliance::ndp_number(&id(first + 1), annex))
}

fn tabulated_row(rows: &[u32], n: u32) -> u32 {
    rows.iter().rev().copied().find(|r| *r <= n).unwrap_or(rows[0])
}

/// 📐️ Correlation factors (ξ1, ξ2) from static load tests, read from the shared NDP registry (`ndp.en1997.xi.static.*`: EN 1997-1 Table A.9, DIN 1054 Table A 7.1 for the German annex, DE values unverified); `stiff` divides by 1.1 (ξ ≥ 1.0).
pub fn xi_static_tests(n: u32, annex: AnnexChoice, stiff: bool) -> (f64, f64) {
    let (x1, x2) = registry_pair("static", n.clamp(1, 5), 1, annex);
    if stiff { ((x1 / 1.1).max(1.0), (x2 / 1.1).max(1.0)) } else { (x1, x2) }
}

/// 📐️ Correlation factors (ξ3, ξ4) from ground-test profiles: `ndp.en1997.xi.ground.*` (EN 1997-1 Table A.10), the row of the largest tabulated n not above the number of profiles.
pub fn xi_ground_tests(n: u32) -> (f64, f64) {
    registry_pair("ground", tabulated_row(&[1, 2, 3, 4, 5, 7, 10], n.max(1)), 3, AnnexChoice::En)
}

/// 📐️ Correlation factors (ξ5, ξ6) from dynamic tests: `ndp.en1997.xi.dynamic.*` (EN 1997-1 Table A.11).
pub fn xi_dynamic_tests(n: u32) -> (f64, f64) {
    registry_pair("dynamic", tabulated_row(&[2, 5, 10, 15, 20], n.max(2)), 5, AnnexChoice::En)
}

/// 🧱️ Concrete strength classes of a pile shaft (EN 1992-1-1 Table 3.1, C12/15 to C50/60), the choices of the pile material.
pub const PILE_CONCRETE_CLASSES: [&str; 9] = ["C12/15", "C16/20", "C20/25", "C25/30", "C30/37", "C35/45", "C40/50", "C45/55", "C50/60"];

/// 🧱️ Secant modulus E_cm [Pa] of a concrete strength class from the shared registry rows `ndp.en1992.t3-1.ecm.*` (EN 1992-1-1 Table 3.1, one source for EN 1992 and the pile elastic shortening); `None` for a class outside [`PILE_CONCRETE_CLASSES`].
pub fn concrete_modulus_pa(class: &str) -> Option<f64> {
    let class = PILE_CONCRETE_CLASSES.iter().find(|known| known.eq_ignore_ascii_case(class.trim()))?;
    semio_s_plugin_norm_compliance::ndp_number_opt(&format!("ndp.en1992.t3-1.ecm.{}", class.to_ascii_lowercase().replace('/', "-")), AnnexChoice::En).map(|gpa| gpa * 1.0e9)
}

/// 🧱️ Elastic modulus E_c [Pa] of the pile shaft for the elastic shortening: the explicit modulus of the pile when it is above 0 (the override), else E_cm of its concrete class from the registry, else 0 (not given).
pub fn pile_elastic_modulus_pa(pile: &semio_s_artifact_norm_en1997_model::Pile) -> f64 {
    if pile.elastic_modulus > 0.0 {
        return pile.elastic_modulus;
    }
    pile.concrete_class.designation().and_then(concrete_modulus_pa).unwrap_or(0.0)
}

/// 📐️ Characteristic resistance min{mean/ξa; min/ξb} of a series of measured or calculated values.
pub fn characteristic_from_series(values: &[f64], xi_a: f64, xi_b: f64) -> (f64, f64, f64) {
    let mean = values.iter().sum::<f64>() / values.len() as f64;
    let min = values.iter().copied().fold(f64::INFINITY, f64::min);
    (mean, min, (mean / xi_a).min(min / xi_b))
}

fn pile_01(c: &Case) -> R {
    let r = c.series("rm");
    gate(!r.is_empty(), "series rm1…rmn of measured resistances required")?;
    let p = c.factors();
    let (x1, x2) = xi_static_tests(r.len() as u32, c.annex, c.o("stiff", 0.0) > 0.5);
    let (mean, min, rk) = characteristic_from_series(&r, x1, x2);
    let rd = rk / p.gamma_t;
    let mut out = vec![
        Outcome::newton("mean", "Mean measured resistance", "Mittelwert Probebelastung", mean),
        Outcome::newton("min", "Minimum measured resistance", "Minimalwert Probebelastung", min),
        Outcome::number("xi1", "ξ1", "ξ1", x1),
        Outcome::number("xi2", "ξ2", "ξ2", x2),
        Outcome::newton("r_c_k", "R_c,k", "R_c,k", rk),
        Outcome::newton("r_c_d", "R_c,d = R_c,k/γt", "R_c,d = R_c,k/γt", rd),
    ];
    if let Some(f) = c.opt("f_cd") {
        out.insert(0, Outcome::force("compression", "Compression F_c,d vs R_c,d", "Drucknachweis F_c,d gegen R_c,d", f, rd));
    }
    Ok(out)
}

fn pile_02(c: &Case) -> R {
    let rb = c.series("rb");
    let rs = c.series("rs");
    gate(!rb.is_empty() && rb.len() == rs.len(), "series rb1…rbn and rs1…rsn of equal length required")?;
    let p = c.factors();
    let n = rb.len() as u32;
    let (x3, x4) = if c.o("empirical", 0.0) > 0.5 { (1.0, 1.0) } else { xi_ground_tests(n) };
    let (_, _, rbk) = characteristic_from_series(&rb, x3, x4);
    let (_, _, rsk) = characteristic_from_series(&rs, x3, x4);
    let rd = rbk / p.gamma_b + rsk / p.gamma_s;
    let mut out = vec![
        Outcome::number("xi3", "ξ3", "ξ3", x3),
        Outcome::number("xi4", "ξ4", "ξ4", x4),
        Outcome::newton("r_b_k", "R_b,k", "R_b,k", rbk),
        Outcome::newton("r_s_k", "R_s,k", "R_s,k", rsk),
        Outcome::newton("r_c_d", "R_c,d = R_b,k/γb + R_s,k/γs", "R_c,d = R_b,k/γb + R_s,k/γs", rd),
    ];
    if let Some(f) = c.opt("f_cd") {
        out.insert(0, Outcome::force("compression", "Compression F_c,d vs R_c,d", "Drucknachweis F_c,d gegen R_c,d", f, rd));
    }
    Ok(out)
}

fn pile_03(c: &Case) -> R {
    let p = c.factors();
    let rt_k = match c.opt("rt_k") {
        Some(v) => v,
        None => {
            let r = c.series("rt");
            gate(!r.is_empty(), "rt_k or series rt1…rtn required")?;
            let (x1, x2) = xi_static_tests(r.len() as u32, c.annex, false);
            characteristic_from_series(&r, x1, x2).2
        }
    };
    let rd = rt_k / p.gamma_st;
    let g_stb = c.o("g_pile", 0.0);
    let f_td = c.positive("f_td")?;
    Ok(vec![Outcome::force("tension", "Tension F_t,d vs R_t,d + G_stb,d", "Zug F_t,d gegen R_t,d + G_stb,d", f_td, rd + g_stb), Outcome::newton("r_t_d", "R_t,d", "R_t,d", rd)])
}

/// 🪵️ Characteristic length parameter λ = (k_h·D / 4EI)^¼ [1/m] of the Winkler pile (Hetényi).
pub fn lateral_lambda(k_h: f64, d: f64, ei: f64) -> f64 {
    (k_h * d / (4.0 * ei)).powf(0.25)
}

/// 🪵️ Lateral deflection line y(x) [m] of a long free-head pile on a Winkler foundation under the head load `h` [N]: y = 2Hλ/(k_h·D)·e^(−λx)·cos(λx), `x` the depth below the head.
pub fn lateral_deflection(h: f64, lambda: f64, k_h: f64, d: f64, x: f64) -> f64 {
    2.0 * h * lambda / (k_h * d) * (-lambda * x).exp() * (lambda * x).cos()
}

/// 🪵️ Bending moment line M(x) [N·m] of the same pile: M = −(H/λ)·e^(−λx)·sin(λx), extremum 0.3224·H/λ at λx = π/4.
pub fn lateral_moment(h: f64, lambda: f64, x: f64) -> f64 {
    -(h / lambda) * (-lambda * x).exp() * (lambda * x).sin()
}

/// 🪵️ Elastic critical load [N] of a pinned pile of length `l` on a Winkler foundation of line stiffness k_h·D: the minimum over the mode numbers m of m²π²EI/L² + k_h·D·L²/(m²π²); without soil stiffness the Euler load.
pub fn winkler_buckling_load(ei: f64, l: f64, k_h: f64, d: f64) -> f64 {
    let pi2 = std::f64::consts::PI.powi(2);
    (1..=400).map(|m| f64::from(m).powi(2) * pi2 * ei / (l * l) + k_h * d * l * l / (f64::from(m).powi(2) * pi2)).fold(f64::INFINITY, f64::min)
}

fn pile_04(c: &Case) -> R {
    let (d, h, e) = (c.positive("d")?, c.positive("h_d")?, c.o("e", 0.0));
    let gr = c.factors().gamma_r_e;
    let mut out = Vec::new();
    if let (Some(k), Some(ei)) = (c.opt("k_s"), c.opt("ei")) {
        let lambda = (k * d / (4.0 * ei)).powf(0.25);
        let y0 = 2.0 * h * lambda / (k * d);
        let m_max = 0.322 * h / lambda;
        out.push(Outcome::metre("y0", "Head deflection y0 (long pile)", "Kopfverschiebung y0 (lang)", y0));
        out.push(Outcome::newton_metre("m_max", "Maximum moment 0.322·H/λ", "Maximales Moment 0,322·H/λ", m_max));
        out.push(Outcome::number("lambda", "Characteristic length parameter λ", "Charakteristischer Längenparameter λ", lambda));
        if let Some(lim) = c.opt("y_lim") {
            out.insert(0, Outcome::length("deflection", "Deflection y0 vs limit", "Verschiebung y0 gegen Grenzwert", y0, lim));
        }
    }
    if let (Some(gamma), Some(l), Some(phi)) = (c.opt("gamma"), c.opt("l"), c.opt("phi_deg")) {
        let kp = earth_pressure::kp_rankine(phi);
        let hu = 0.5 * gamma * d * l.powi(3) * kp / (e + l);
        out.insert(0, Outcome::force("transverse", "Transverse H_d vs H_u/γR,e (Broms)", "Querkraft H_d gegen H_u/γR,e (Broms)", h, hu / gr));
        out.push(Outcome::newton("h_u", "Ultimate transverse force H_u", "Grenzquerkraft H_u", hu));
    }
    gate(!out.is_empty(), "k_s with ei, or gamma with l and phi_deg required")?;
    if let Some(row) = c.opt("row") {
        let mult = if row <= 1.0 { 0.8 } else if row <= 2.0 { 0.4 } else { 0.3 };
        out.push(Outcome::number("p_multiplier", "Group p-multiplier", "Gruppen-p-Multiplikator", mult));
    }
    Ok(out)
}

fn pile_05(c: &Case) -> R {
    let (d, beta) = (c.positive("d")?, c.o("beta", 0.25));
    let h = c.series("h");
    let sv = c.series("sv");
    gate(!h.is_empty() && h.len() == sv.len(), "series h1…hn and sv1…svn required")?;
    let neg: f64 = h.iter().zip(&sv).map(|(hi, si)| std::f64::consts::PI * d * beta * si * hi).sum();
    let p = c.factors();
    let f_neg_d = p.gamma_g * neg;
    let mut out = vec![Outcome::newton("f_neg_k", "Drag load F_neg,k", "Negative Mantelreibung F_neg,k", neg), Outcome::newton("f_neg_d", "Drag load F_neg,d", "Negative Mantelreibung F_neg,d", f_neg_d)];
    if let (Some(fc), Some(rcd)) = (c.opt("f_cd"), c.opt("r_c_d")) {
        out.insert(0, Outcome::force("downdrag", "F_c,d + F_neg,d vs R_c,d", "F_c,d + F_neg,d gegen R_c,d", fc + f_neg_d, rcd));
    }
    Ok(out)
}

fn pile_06(c: &Case) -> R {
    let (m, n, s, d) = (c.positive("m")?, c.positive("n")?, c.positive("s")?, c.positive("d")?);
    let theta = (d / s).atan().to_degrees();
    let eta = 1.0 - theta * ((m - 1.0) * n + (n - 1.0) * m) / (90.0 * m * n);
    let count = m * n;
    let (sx, sy) = (c.o("sx", s), c.o("sy", s));
    let (v, mx, my) = (c.o("v", 0.0), c.o("mx", 0.0), c.o("my", 0.0));
    let xs: Vec<f64> = (0..n as usize).map(|i| (i as f64 - (n - 1.0) / 2.0) * sx).collect();
    let ys: Vec<f64> = (0..m as usize).map(|i| (i as f64 - (m - 1.0) / 2.0) * sy).collect();
    let sum_x2 = m * xs.iter().map(|x| x * x).sum::<f64>();
    let sum_y2 = n * ys.iter().map(|y| y * y).sum::<f64>();
    let x_max = xs.iter().copied().fold(0.0, f64::max);
    let y_max = ys.iter().copied().fold(0.0, f64::max);
    let term = |sum: f64, arm: f64, moment: f64| if sum > 0.0 { moment * arm / sum } else { 0.0 };
    let n_max = v / count + term(sum_y2, y_max, mx) + term(sum_x2, x_max, my);
    let n_min = v / count - term(sum_y2, y_max, mx) - term(sum_x2, x_max, my);
    let mut out = vec![
        Outcome::number("eta", "Converse-Labarre efficiency η", "Converse-Labarre-Wirkungsgrad η", eta),
        Outcome::newton("n_max", "Maximum pile load", "Maximale Pfahlkraft", n_max),
        Outcome::newton("n_min", "Minimum pile load", "Minimale Pfahlkraft", n_min),
    ];
    if let (Some(bl), Some(ll), Some(cu_h), Some(cu_b)) = (c.opt("block_b"), c.opt("block_l"), c.opt("cu_h_sum"), c.opt("cu_b")) {
        let r_block = 2.0 * (bl + ll) * cu_h + bl * ll * c.o("nc", 9.0) * cu_b;
        let rd = r_block / c.factors().gamma_b;
        out.push(Outcome::newton("r_block_d", "Block resistance R_block,d", "Blockwiderstand R_block,d", rd));
        if v > 0.0 {
            out.insert(0, Outcome::force("block", "Block failure V_d vs R_block,d", "Blockversagen V_d gegen R_block,d", v, rd));
        }
    }
    if let Some(rc) = c.opt("r_c_d") {
        out.insert(0, Outcome::force("max_pile", "Maximum pile load vs R_c,d", "Maximale Pfahlkraft gegen R_c,d", n_max, rc));
    }
    Ok(out)
}

fn pile_07(c: &Case) -> R {
    let (ei, l_eff, n_ed) = (c.positive("ei")?, c.positive("l_eff")?, c.positive("n_ed")?);
    let n_cr = std::f64::consts::PI.powi(2) * ei / (l_eff * l_eff);
    Ok(vec![Outcome::force("buckling", "N_Ed vs N_cr", "N_Ed gegen N_cr", n_ed, n_cr), Outcome::newton("n_cr", "Critical load π²EI/L²", "Kritische Last π²EI/L²", n_cr)])
}

fn pile_08(c: &Case) -> R {
    let r = c.series("rm");
    gate(!r.is_empty(), "series rm1…rmn of dynamic test resistances required")?;
    let (x5, x6) = xi_dynamic_tests(r.len() as u32);
    let (_, _, rk) = characteristic_from_series(&r, x5, x6);
    let rd = rk / c.factors().gamma_t;
    let mut out = vec![Outcome::number("xi5", "ξ5", "ξ5", x5), Outcome::number("xi6", "ξ6", "ξ6", x6), Outcome::newton("r_c_k", "R_c,k", "R_c,k", rk), Outcome::newton("r_c_d", "R_c,d", "R_c,d", rd)];
    if let (Some(eta), Some(w), Some(hf), Some(s), Some(se)) = (c.opt("eta_h"), c.opt("w"), c.opt("h_drop"), c.opt("s_set"), c.opt("s_e")) {
        out.push(Outcome::newton("r_driving", "Driving formula R = η·W·h/(s + 0.5 s_e)", "Rammformel R = η·W·h/(s + 0,5 s_e)", eta * w * hf / (s + 0.5 * se)));
    }
    if let Some(f) = c.opt("f_cd") {
        out.insert(0, Outcome::force("compression", "Compression F_c,d vs R_c,d", "Drucknachweis F_c,d gegen R_c,d", f, rd));
    }
    Ok(out)
}

/// 📐️ Pile head settlement `(mobilisation, elastic shortening)` in m under the load `n`: shaft and base resistance mobilise linearly up to the reference settlements `s_ref_s` and `s_ref_b`, the elastic shortening is `(n − R_s,mob / 2) L / EA`; `None` when `n` reaches the sum of the capacities. The reference settlements default to 1 % (shaft) and 10 % (base) of the diameter, unverified conventions.
pub fn resistance_settlement(n: f64, rs_max: f64, rb_max: f64, ea: f64, l: f64, s_ref: (f64, f64)) -> Option<(f64, f64)> {
    if n >= rs_max + rb_max {
        return None;
    }
    let (s_ref_s, s_ref_b) = s_ref;
    let total = |s: f64| rs_max * (s / s_ref_s).min(1.0) + rb_max * (s / s_ref_b).min(1.0);
    let (mut lo, mut hi) = (0.0, s_ref_b.max(s_ref_s));
    for _ in 0..80 {
        let mid = 0.5 * (lo + hi);
        if total(mid) < n { lo = mid } else { hi = mid }
    }
    let s_mob = 0.5 * (lo + hi);
    let rs_mob = rs_max * (s_mob / s_ref_s).min(1.0);
    Some((s_mob, (n - 0.5 * rs_mob) * l / ea))
}

fn pile_09(c: &Case) -> R {
    let (n, rs_max, rb_max, d) = (c.positive("n")?, c.positive("rs_max")?, c.positive("rb_max")?, c.positive("d")?);
    let (ea, l) = (c.positive("ea")?, c.positive("l")?);
    let (s_ref_s, s_ref_b) = (c.o("s_ref_s", 0.01 * d), c.o("s_ref_b", 0.10 * d));
    gate(n < rs_max + rb_max, "load exceeds the sum of shaft and base capacity")?;
    let (s_mob, s_el) = resistance_settlement(n, rs_max, rb_max, ea, l, (s_ref_s, s_ref_b)).ok_or("load exceeds the sum of shaft and base capacity")?;
    let s_total = s_mob + s_el;
    let mut out = vec![Outcome::metre("s_mobilised", "Mobilisation settlement", "Mobilisierungssetzung", s_mob), Outcome::metre("s_elastic", "Elastic shortening", "Elastische Stauchung", s_el), Outcome::metre("s_total", "Pile head settlement", "Pfahlkopfsetzung", s_total)];
    if let Some(lim) = c.opt("s_lim") {
        out.insert(0, Outcome::length("serviceability", "Pile settlement vs limit", "Pfahlsetzung gegen Grenzwert", s_total, lim));
    }
    Ok(out)
}

/// 📐️ Load at the settlement `s_g` on the piecewise-linear load-settlement curve; `None` when the curve never reaches it.
pub fn load_at_settlement(settlement: &[f64], load: &[f64], s_g: f64) -> Option<f64> {
    settlement.windows(2).zip(load.windows(2)).find(|(s, _)| s[0] <= s_g && s_g <= s[1]).map(|(s, q)| if s[1] - s[0] <= f64::EPSILON { q[1] } else { q[0] + (q[1] - q[0]) * (s_g - s[0]) / (s[1] - s[0]) })
}

fn pile_10(c: &Case) -> R {
    let (ts, tq) = (c.series("ts"), c.series("tq"));
    gate(ts.len() >= 2 && ts.len() == tq.len() && ts.windows(2).all(|w| w[1] > w[0]), "series ts1…tsn (strictly increasing settlement) and tq1…tqn of equal length (n ≥ 2) required")?;
    let d = c.positive("d")?;
    let s_g = c.o("s_crit_factor", 0.1) * d;
    let s_max = ts[ts.len() - 1];
    let r_m = load_at_settlement(&ts, &tq, s_g).unwrap_or_else(|| tq.iter().copied().fold(0.0, f64::max));
    let mut all = vec![r_m];
    all.extend(c.series("rm"));
    let p = c.factors();
    let (x1, x2) = xi_static_tests(all.len() as u32, c.annex, c.o("stiff", 0.0) > 0.5);
    let (mean, min, rk) = characteristic_from_series(&all, x1, x2);
    let rd = rk / p.gamma_t;
    let mut out = vec![
        Outcome::length("criterion_reached", "Failure settlement s_g = 0.1·D vs largest test settlement", "Grenzsetzung s_g = 0,1·D gegen größte Versuchssetzung", s_g, s_max),
        Outcome::metre("s_g", "Failure settlement s_g", "Grenzsetzung s_g", s_g),
        Outcome::newton("r_c_m", "Measured resistance R_c,m at s_g", "Gemessener Widerstand R_c,m bei s_g", r_m),
        Outcome::newton("mean", "Mean over all tests", "Mittelwert aller Versuche", mean),
        Outcome::newton("min", "Minimum over all tests", "Minimum aller Versuche", min),
        Outcome::number("xi1", "ξ1", "ξ1", x1),
        Outcome::number("xi2", "ξ2", "ξ2", x2),
        Outcome::newton("r_c_k", "R_c,k", "R_c,k", rk),
        Outcome::newton("r_c_d", "R_c,d = R_c,k/γt", "R_c,d = R_c,k/γt", rd),
        Outcome::number("xi_basis_unverified", "1 = ξ table basis not yet verified against the licensed DIN 1054:2021 / NA-DE text", "1 = ξ-Tabellengrundlage noch nicht gegen den lizenzierten Text DIN 1054:2021 / NA-DE geprüft", if c.de() { 1.0 } else { 0.0 }),
    ];
    if let Some(f) = c.opt("f_cd") {
        out.insert(0, Outcome::force("compression", "Compression F_c,d vs R_c,d", "Drucknachweis F_c,d gegen R_c,d", f, rd));
    }
    Ok(out)
}

fn pile_11(c: &Case) -> R {
    let d = c.positive("d")?;
    let (perimeter, area) = (std::f64::consts::PI * d, std::f64::consts::FRAC_PI_4 * d * d);
    let mut derived = c.clone();
    let mut out = Vec::new();
    for j in 1..=16 {
        let (h, qs) = (c.series(&format!("p{j}_h")), c.series(&format!("p{j}_qs")));
        if h.is_empty() {
            break;
        }
        gate(h.len() == qs.len(), "each profile needs equally long series pJ_h1…, pJ_qs1…")?;
        let rs: f64 = h.iter().zip(&qs).map(|(h, q)| perimeter * h * q).sum();
        let rb = c.positive(&format!("p{j}_qb"))? * area;
        derived.num.insert(format!("rb{j}"), rb);
        derived.num.insert(format!("rs{j}"), rs);
        out.push(Outcome::newton(&format!("r_b_cal{j}"), &format!("Calculated base resistance, profile {j}"), &format!("Berechneter Spitzenwiderstand, Profil {j}"), rb));
        out.push(Outcome::newton(&format!("r_s_cal{j}"), &format!("Calculated shaft resistance, profile {j}"), &format!("Berechneter Mantelwiderstand, Profil {j}"), rs));
    }
    gate(!out.is_empty(), "at least one profile pointer p1_h1, p1_qs1, p1_qb required")?;
    let mut result = pile_02(&derived)?;
    result.extend(out);
    Ok(result)
}

fn pile_12(c: &Case) -> R {
    let (m, n, s, d, l, v) = (c.positive("m")?, c.positive("n")?, c.positive("s")?, c.positive("d")?, c.positive("l")?, c.positive("v")?);
    let (spread, factor) = (c.positive("spread")?, c.positive("raft_depth_factor")?);
    gate(factor < 1.0, "raft_depth_factor must be below 1")?;
    let (heights, moduli) = (c.series("h"), c.series("e"));
    gate(!heights.is_empty() && heights.len() == moduli.len(), "layer series h1…hn and e1…en below the equivalent raft required")?;
    let grow = 2.0 * spread * factor * l;
    let (b, length) = ((n - 1.0) * c.o("sx", s) + d + grow, (m - 1.0) * c.o("sy", s) + d + grow);
    let p = v / (b * length);
    let mut z_top = 0.0;
    let mut total = 0.0;
    for (h, e) in heights.iter().zip(&moduli) {
        total += super::settlement::sigma_z_rect_centre(p, b, length, z_top + 0.5 * h) * h / e;
        z_top += h;
    }
    let mut out = vec![
        Outcome::metre("b_raft", "Equivalent raft width", "Breite des Ersatzfundaments", b),
        Outcome::metre("l_raft", "Equivalent raft length", "Länge des Ersatzfundaments", length),
        Outcome::pascal("p_raft", "Pressure on the equivalent raft", "Pressung auf dem Ersatzfundament", p),
        Outcome::metre("s_group", "Pile group settlement", "Pfahlgruppensetzung", total),
    ];
    if let Some(limit) = c.opt("s_lim") {
        out.insert(0, Outcome::length("serviceability", "Group settlement vs limit", "Gruppensetzung gegen Grenzwert", total, limit));
    }
    Ok(out)
}

pub const ENTRIES: &[Entry] = &[
    Entry { id: "PILE-01", part: "EN 1997-1", clause: "7.6.2.2", en: "Pile compression from static load tests", de: "Pfahldruck aus statischen Probebelastungen", inputs: &["rm1", "stiff", "f_cd", "approach", "situation"], eval: pile_01 },
    Entry { id: "PILE-02", part: "EN 1997-1", clause: "7.6.2.3", en: "Pile compression from ground tests", de: "Pfahldruck aus Baugrunderkundung", inputs: &["rb1", "rs1", "empirical", "f_cd", "approach", "situation"], eval: pile_02 },
    Entry { id: "PILE-03", part: "EN 1997-1", clause: "7.6.3", en: "Pile tension resistance", de: "Pfahlzugwiderstand", inputs: &["rt_k", "rt1", "g_pile", "f_td", "approach", "situation"], eval: pile_03 },
    Entry { id: "PILE-04", part: "EN 1997-1", clause: "7.7", en: "Transversely loaded piles", de: "Horizontal belastete Pfähle", inputs: &["d", "h_d", "e", "k_s", "ei", "y_lim", "gamma", "l", "phi_deg", "row"], eval: pile_04 },
    Entry { id: "PILE-05", part: "EN 1997-1", clause: "7.3.2.2", en: "Negative skin friction", de: "Negative Mantelreibung", inputs: &["d", "beta", "h1", "sv1", "f_cd", "r_c_d", "approach", "situation"], eval: pile_05 },
    Entry { id: "PILE-06", part: "EN 1997-1", clause: "7.6.2", en: "Pile group effects", de: "Pfahlgruppenwirkung", inputs: &["m", "n", "s", "d", "sx", "sy", "v", "mx", "my", "r_c_d", "block_b", "block_l", "cu_h_sum", "cu_b", "nc"], eval: pile_06 },
    Entry { id: "PILE-07", part: "EN 1997-1", clause: "7.8", en: "Structural pile interface, buckling", de: "Schnittstelle Pfahltragwerk, Knicken", inputs: &["ei", "l_eff", "n_ed"], eval: pile_07 },
    Entry { id: "PILE-08", part: "EN 1997-1", clause: "7.6.2.4", en: "Dynamic tests and driving formulae", de: "Dynamische Tests und Rammformeln", inputs: &["rm1", "f_cd", "eta_h", "w", "h_drop", "s_set", "s_e", "approach", "situation"], eval: pile_08 },
    Entry { id: "PILE-09", part: "EN 1997-1", clause: "7.6.4", en: "Pile settlement, resistance-settlement", de: "Pfahlsetzung, Widerstands-Setzungs-Linie", inputs: &["n", "rs_max", "rb_max", "d", "ea", "l", "s_ref_s", "s_ref_b", "s_lim"], eval: pile_09 },
    Entry { id: "PILE-10", part: "EN 1997-1", clause: "7.6.2.2", en: "Pile load-settlement test, failure settlement criterion", de: "Pfahlprobebelastung, Grenzsetzungskriterium", inputs: &["ts1", "tq1", "d", "s_crit_factor", "rm1", "stiff", "f_cd", "approach", "situation"], eval: pile_10 },
    Entry { id: "PILE-11", part: "EN 1997-1", clause: "7.6.2.3", en: "Pile compression from ground-test profiles (unit resistances)", de: "Pfahldruck aus Erkundungsprofilen (Einheitswiderstände)", inputs: &["d", "p1_h1", "p1_qs1", "p1_qb", "empirical", "f_cd", "approach", "situation"], eval: pile_11 },
    Entry { id: "PILE-12", part: "EN 1997-1", clause: "7.6.4", en: "Pile group settlement, equivalent raft", de: "Pfahlgruppensetzung, Ersatzfundament", inputs: &["m", "n", "s", "sx", "sy", "d", "l", "v", "spread", "raft_depth_factor", "h1", "e1", "s_lim"], eval: pile_12 },
];
