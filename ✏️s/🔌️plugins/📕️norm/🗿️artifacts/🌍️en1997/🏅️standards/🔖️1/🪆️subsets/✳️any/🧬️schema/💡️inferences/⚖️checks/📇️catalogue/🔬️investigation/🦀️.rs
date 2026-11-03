//! 🔬️ Ground model, characteristic values and ground investigation, catalogue entries G-01, G-02, GI-01 … GI-17 (EN 1997-1 §2.4.3-2.4.5, EN 1997-2, DIN 4020, DIN 4094).

use super::*;

/// 📚️ Student t quantiles t(0.95; ν) of EN 1997 statistical derivation, ν = 1 … 10, 20 and ∞.
const T95: [(f64, f64); 12] = [(1.0, 6.314), (2.0, 2.920), (3.0, 2.353), (4.0, 2.132), (5.0, 2.015), (6.0, 1.943), (7.0, 1.895), (8.0, 1.860), (9.0, 1.833), (10.0, 1.812), (20.0, 1.725), (f64::INFINITY, 1.645)];

/// 📐️ Student t quantile t(0.95; ν): the tabulated values, linear in 1/ν between ν = 10, 20 and ∞.
pub fn t_95(nu: f64) -> f64 {
    let nu = nu.max(1.0);
    for w in T95.windows(2) {
        if nu <= w[1].0 {
            if w[0].0 >= 10.0 || w[1].0.is_infinite() {
                let (a, b) = (1.0 / w[0].0, if w[1].0.is_infinite() { 0.0 } else { 1.0 / w[1].0 });
                let t = (1.0 / nu - b) / (a - b);
                return w[1].1 + t * (w[0].1 - w[1].1);
            }
            let t = (nu - w[0].0) / (w[1].0 - w[0].0);
            return w[0].1 + t * (w[1].1 - w[0].1);
        }
    }
    1.645
}

/// 📐️ Statistical characteristic value X_k = X_m·(1 − k_n·V_x) (EN 1997-1 §2.4.5.2, Annex C of EN 1997-2).
pub fn characteristic_value(samples: &[f64], local: bool, v_known: Option<f64>) -> (f64, f64, f64, f64, f64) {
    let n = samples.len() as f64;
    let mean = samples.iter().sum::<f64>() / n;
    let s = if n > 1.0 { (samples.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / (n - 1.0)).sqrt() } else { 0.0 };
    let v = v_known.unwrap_or(s / mean);
    let t = if v_known.is_some() { 1.645 } else { t_95(n - 1.0) };
    let kn = t * if local { (1.0 + 1.0 / n).sqrt() } else { (1.0 / n).sqrt() };
    (mean, s, v, kn, (mean * (1.0 - kn * v)).max(0.0))
}

fn g_01(c: &Case) -> R {
    let h = c.series("h");
    let gamma = c.series("gamma");
    gate(!h.is_empty() && h.len() == gamma.len(), "series h1…hn and gamma1…gamma_n required")?;
    let (z, z_w, gw) = (c.positive("z")?, c.o("z_w", f64::INFINITY), c.gamma_w());
    let sat = c.series("gamma_sat");
    let (mut top, mut sigma, mut tan_phi_sum, mut thickness) = (0.0, 0.0, 0.0, 0.0);
    for (i, (hi, gi)) in h.iter().zip(&gamma).enumerate() {
        let gs = sat.get(i).copied().unwrap_or(*gi);
        let bottom = (top + hi).min(z);
        if bottom > top {
            let dry = (z_w.min(bottom) - top).max(0.0).min(bottom - top);
            sigma += gi * dry + gs * (bottom - top - dry);
            if let Some(phi) = c.opt(&format!("phi{}", i + 1)) {
                tan_phi_sum += rad(phi).tan() * (bottom - top);
                thickness += bottom - top;
            }
        }
        top += hi;
        if top >= z {
            break;
        }
    }
    gate(top >= z - 1e-9, "evaluation depth exceeds the layer series")?;
    let u = gw * (z - z_w).max(0.0);
    let mut out = vec![Outcome::pascal("sigma_v", "Total vertical stress", "Totale Vertikalspannung", sigma), Outcome::pascal("u", "Pore water pressure", "Porenwasserdruck", u), Outcome::pascal("sigma_v_eff", "Effective vertical stress", "Effektive Vertikalspannung", sigma - u)];
    if thickness > 0.0 {
        out.push(Outcome::number("phi_equivalent_deg", "Equivalent φ′ (thickness-weighted tan φ′) in degrees", "Ersatz-φ′ (dickengewichtetes tan φ′) in Grad", (tan_phi_sum / thickness).atan().to_degrees()));
    }
    Ok(out)
}

fn g_02(c: &Case) -> R {
    let x = c.series("x");
    gate(!x.is_empty(), "series x1…xn of samples required")?;
    let (mean, s, v, kn, xk) = characteristic_value(&x, c.so("mode", "mean") == "local", c.opt("v_known"));
    let mut out = vec![
        Outcome::number("mean", "Sample mean X_m", "Stichprobenmittel X_m", mean),
        Outcome::number("s", "Standard deviation s", "Standardabweichung s", s),
        Outcome::number("v", "Coefficient of variation V_x", "Variationskoeffizient V_x", v),
        Outcome::number("kn", "Statistical coefficient k_n", "Statistischer Beiwert k_n", kn),
        Outcome::number("xk", "Characteristic value X_k", "Charakteristischer Wert X_k", xk),
    ];
    if x.len() < 3 || v > 0.3 {
        out.push(Outcome::number("warning", "Warning: n < 3 or V_x > 0.3", "Warnung: n < 3 oder V_x > 0,3", 1.0));
    }
    Ok(out)
}

/// 🔖️ Registry rows of the recommended investigation point spacing (EN 1997-2 Annex B.3, DIN 4020; unverified): GC1 and GC2 first, GC3 second.
pub const SPACING_MIN_IDS: [&str; 2] = ["ndp.en1997.2.4.2.spacing.gc12.min", "ndp.en1997.2.4.2.spacing.gc3.min"];
pub const SPACING_MAX_IDS: [&str; 2] = ["ndp.en1997.2.4.2.spacing.gc12.max", "ndp.en1997.2.4.2.spacing.gc3.max"];

/// 📏️ Recommended spacing `(min, max)` [m] of the investigation points of the geotechnical category `gk` from the registry.
pub fn spacing_limits_m(gk: u8, annex: semio_s_plugin_norm_compliance::AnnexChoice) -> (f64, f64) {
    let row = usize::from(gk >= 3);
    (semio_s_plugin_norm_compliance::ndp_number(SPACING_MIN_IDS[row], annex), semio_s_plugin_norm_compliance::ndp_number(SPACING_MAX_IDS[row], annex))
}

fn gi_01(c: &Case) -> R {
    let gk = c.o("gk", 2.0);
    let (depth, spacing_min, spacing_max) = if let Some(db) = c.opt("d_b") {
        ((5.0f64).max(3.0 * db), 0.0, 0.0)
    } else {
        let b = c.positive("b")?;
        let (lo, hi) = spacing_limits_m(if gk >= 3.0 { 3 } else { 2 }, c.annex);
        ((6.0f64).max(2.0 * b), lo, hi)
    };
    let mut out = vec![Outcome::metre("z_a", "Required investigation depth", "Erforderliche Aufschlusstiefe", depth), Outcome::metre("spacing_min", "Minimum point spacing", "Minimaler Aufschlussabstand", spacing_min), Outcome::metre("spacing_max", "Maximum point spacing", "Maximaler Aufschlussabstand", spacing_max)];
    if let Some(have) = c.opt("z_have") {
        out.insert(0, Outcome::length("coverage", "Required vs available investigation depth", "Erforderliche gegen vorhandene Aufschlusstiefe", depth, have));
    }
    Ok(out)
}

fn gi_02(c: &Case) -> R {
    let (qc, fs, sv, svp) = (c.positive("qc")?, c.positive("fs")?, c.positive("sigma_v0")?, c.positive("sigma_v0_eff")?);
    let qt = qc + c.o("u2", 0.0) * (1.0 - c.o("a", 0.8));
    let rf = fs / qc * 100.0;
    let q_norm = (qt - sv) / svp;
    let f_norm = fs / (qt - sv) * 100.0;
    gate(q_norm > 0.0 && f_norm > 0.0, "net cone resistance must be positive")?;
    let ic = ((3.47 - q_norm.log10()).powi(2) + (f_norm.log10() + 1.22).powi(2)).sqrt();
    Ok(vec![Outcome::pascal("qt", "Corrected cone resistance q_t", "Korrigierter Spitzendruck q_t", qt), Outcome::number("rf", "Friction ratio R_f (%)", "Reibungsverhältnis R_f (%)", rf), Outcome::number("ic", "Soil behaviour type index I_c", "Bodenverhaltensindex I_c", ic)])
}

/// 🔩 Registry row of the CPT cone factor N_kt (unverified; the spec gives 15…20), the default when a case or layer names none.
pub const CPT_NKT_ID: &str = "ndp.en1997.2.cpt-nkt";

/// 🔩 Registry rows of the plausible range of N_kt (unverified, from the spec text 15…20): the lower and the upper end.
pub const CPT_NKT_RANGE_IDS: [&str; 2] = ["ndp.en1997.2.cpt-nkt-range.min", "ndp.en1997.2.cpt-nkt-range.max"];

/// 🔩 The plausible range `(min, max)` of the cone factor N_kt from the registry.
pub fn cpt_nkt_range(annex: semio_s_plugin_norm_compliance::AnnexChoice) -> (f64, f64) {
    (semio_s_plugin_norm_compliance::ndp_number(CPT_NKT_RANGE_IDS[0], annex), semio_s_plugin_norm_compliance::ndp_number(CPT_NKT_RANGE_IDS[1], annex))
}

fn gi_03(c: &Case) -> R {
    let (qt, sv) = (c.positive("qt")?, c.positive("sigma_v0")?);
    let nkt = c.opt("nkt").unwrap_or_else(|| semio_s_plugin_norm_compliance::ndp_number(CPT_NKT_ID, c.annex));
    Ok(vec![Outcome::pascal("cu", "Undrained shear strength c_u", "Undränierte Scherfestigkeit c_u", (qt - sv) / nkt), Outcome::number("nkt", "Cone factor N_kt used", "Verwendeter Spitzendruckfaktor N_kt", nkt)])
}

fn gi_04(c: &Case) -> R {
    let (qc, svp) = (c.positive("qc")?, c.positive("sigma_v0_eff")?);
    let pa = 100_000.0;
    Ok(vec![Outcome::number("phi_deg", "φ′ Kulhawy-Mayne (°)", "φ′ Kulhawy-Mayne (°)", 17.6 + 11.0 * ((qc / pa) / (svp / pa).sqrt()).log10())])
}

fn gi_05(c: &Case) -> R {
    let (qc, alpha) = (c.positive("qc")?, c.o("alpha_e", 4.0));
    Ok(vec![Outcome::pascal("e_oed", "Oedometric modulus α_E·q_c", "Steifemodul α_E·q_c", alpha * qc)])
}

fn gi_06(c: &Case) -> R {
    let (n, svp) = (c.positive("n")?, c.positive("sigma_v0_eff")?);
    let n60 = n * c.o("er", 60.0) / 60.0 * c.o("cb", 1.0) * c.o("cs", 1.0) * c.o("cr", 1.0);
    let cn = (100_000.0 / svp).sqrt().min(1.7);
    Ok(vec![Outcome::number("n60", "N60", "N60", n60), Outcome::number("n1_60", "(N1)60", "(N1)60", n60 * cn)])
}

fn gi_07(c: &Case) -> R {
    Ok(vec![Outcome::number("phi_deg", "φ′ Hatanaka-Uchida (°)", "φ′ Hatanaka-Uchida (°)", (20.0 * c.positive("n1_60")?).sqrt() + 20.0)])
}

fn gi_08(c: &Case) -> R {
    Ok(vec![Outcome::number("dr", "Relative density D_r (Skempton)", "Lagerungsdichte D_r (Skempton)", (c.positive("n1_60")? / 60.0).sqrt())])
}

fn gi_09(c: &Case) -> R {
    const T: [(f64, f64); 5] = [(2.0, 27.5), (5.0, 30.0), (10.0, 32.5), (20.0, 35.0), (30.0, 37.5)];
    let n10 = c.positive("n10")?;
    let phi = if n10 <= T[0].0 { T[0].1 } else if n10 >= T[4].0 { T[4].1 } else {
        let w = T.windows(2).find(|w| n10 <= w[1].0).unwrap_or(&T[3..5]);
        w[0].1 + (n10 - w[0].0) / (w[1].0 - w[0].0) * (w[1].1 - w[0].1)
    };
    Ok(vec![Outcome::number("phi_deg", "φ′ range value from N10 (DIN 4094 style table)", "φ′ aus N10 (Tabelle nach DIN 4094)", phi)])
}

fn gi_10(c: &Case) -> R {
    let e = c.series("e");
    let s = c.series("sigma");
    gate(e.len() >= 3 && e.len() == s.len(), "series e1…en and sigma1…sigman (n ≥ 3) required")?;
    let n = e.len();
    let cc = (e[n - 2] - e[n - 1]) / (s[n - 1] / s[n - 2]).log10();
    let cs = (e[0] - e[1]) / (s[1] / s[0]).log10();
    let mut out = vec![Outcome::number("cc", "Compression index C_c", "Kompressionsbeiwert C_c", cc), Outcome::number("cs", "Swelling index C_s", "Schwellbeiwert C_s", cs)];
    if let (Some(h), Some(t90)) = (c.opt("h_dr"), c.opt("t90")) {
        out.push(Outcome::number("cv", "Coefficient of consolidation c_v = T90·H²/t90", "Konsolidierungsbeiwert c_v = T90·H²/t90", super::settlement::t90_factor() * h * h / t90));
    }
    Ok(out)
}

/// 📐️ Least-squares line y = a + b·x.
pub fn regression(x: &[f64], y: &[f64]) -> (f64, f64) {
    let n = x.len() as f64;
    let (mx, my) = (x.iter().sum::<f64>() / n, y.iter().sum::<f64>() / n);
    let sxy: f64 = x.iter().zip(y).map(|(a, b)| (a - mx) * (b - my)).sum();
    let sxx: f64 = x.iter().map(|a| (a - mx).powi(2)).sum();
    let b = sxy / sxx;
    (my - b * mx, b)
}

/// 🧪️ Effective friction angle [°] and cohesion [Pa] of a laboratory series by the regression of GI-11 (triaxial: stress path p′/q, sin φ′ = tan α′, c′ = a/cos φ′) or GI-12 (direct shear: σ/τ line); `None` for other kinds or too few points.
pub fn lab_strength(kind: &str, normal: &[f64], shear: &[f64]) -> Option<(f64, f64)> {
    if normal.len() != shear.len() {
        return None;
    }
    match kind {
        "triaxial" if normal.len() >= 2 => {
            let (a, b) = regression(normal, shear);
            let phi = b.clamp(-1.0, 1.0).asin();
            Some((phi.to_degrees(), a / phi.cos()))
        }
        "directShear" if normal.len() >= 3 => {
            let (a, b) = regression(normal, shear);
            Some((b.atan().to_degrees(), a))
        }
        _ => None,
    }
}

fn gi_11(c: &Case) -> R {
    let p = c.series("p");
    let q = c.series("q");
    gate(p.len() >= 2 && p.len() == q.len(), "series p1…pn and q1…qn (n ≥ 2) required")?;
    let (a, b) = regression(&p, &q);
    let phi = b.asin();
    Ok(vec![Outcome::number("phi_deg", "φ′ from sin φ′ = tan α′ (°)", "φ′ aus sin φ′ = tan α′ (°)", phi.to_degrees()), Outcome::pascal("c", "c′ = a/cos φ′", "c′ = a/cos φ′", a / phi.cos())])
}

fn gi_12(c: &Case) -> R {
    let s = c.series("sigma");
    let t = c.series("tau");
    gate(s.len() >= 3 && s.len() == t.len(), "series sigma1…sigman and tau1…taun (n ≥ 3) required")?;
    let (a, b) = regression(&s, &t);
    Ok(vec![Outcome::number("phi_deg", "φ′ (°)", "φ′ (°)", b.atan().to_degrees()), Outcome::pascal("c", "c′", "c′", a)])
}

/// 📐️ Regression with the one-sided 95 % confidence limits of the line (EN 1997-1 §2.4.5.2(11) cautious estimate of the mean): `(a, b, se_a, se_b, t, a_k, b_k)` for y = a + b·x, `t = t(0.95; n − 2)`.
pub fn regression_lower_bound(x: &[f64], y: &[f64]) -> Option<(f64, f64, f64, f64, f64, f64, f64)> {
    let n = x.len() as f64;
    if x.len() < 3 || x.len() != y.len() {
        return None;
    }
    let (a, b) = regression(x, y);
    let mx = x.iter().sum::<f64>() / n;
    let sxx: f64 = x.iter().map(|v| (v - mx).powi(2)).sum();
    let sse: f64 = x.iter().zip(y).map(|(u, v)| (v - a - b * u).powi(2)).sum();
    let s_e = (sse / (n - 2.0)).sqrt();
    let (se_b, se_a) = (s_e / sxx.sqrt(), s_e * (1.0 / n + mx * mx / sxx).sqrt());
    let t = t_95(n - 2.0);
    Some((a, b, se_a, se_b, t, a - t * se_a, b - t * se_b))
}

fn gi_17(c: &Case) -> R {
    let triaxial = c.so("kind", "shear") == "triaxial";
    let (x, y) = if triaxial { (c.series("p"), c.series("q")) } else { (c.series("sigma"), c.series("tau")) };
    gate(x.len() >= 3 && x.len() == y.len(), "series p1…pn and q1…qn (triaxial) or sigma1…sigman and tau1…taun (shear), n ≥ 3, required")?;
    let (a, b, se_a, se_b, t, a_k, b_k) = regression_lower_bound(&x, &y).ok_or("regression needs three points")?;
    gate(b > 0.0 && b_k > 0.0 && (!triaxial || b_k < 1.0), "the lower confidence slope must stay positive (and below 1 for sin φ′)")?;
    let (phi_mean, phi_k) = if triaxial { (b.asin(), b_k.asin()) } else { (b.atan(), b_k.atan()) };
    let scale = if triaxial { 1.0 / phi_mean.cos() } else { 1.0 };
    let scale_k = if triaxial { 1.0 / phi_k.cos() } else { 1.0 };
    let mut out = vec![
        Outcome::number("phi_mean_deg", "Regression φ′ (°)", "Regressions-φ′ (°)", phi_mean.to_degrees()),
        Outcome::pascal("c_mean", "Regression c′", "Regressions-c′", a * scale),
        Outcome::number("t", "Student t(0.95; n − 2)", "Student-t(0,95; n − 2)", t),
        Outcome::number("se_slope", "Standard error of the slope", "Standardfehler der Steigung", se_b),
        Outcome::pascal("se_intercept", "Standard error of the intercept", "Standardfehler des Achsenabschnitts", se_a),
        Outcome::number("phi_k_deg", "Characteristic φ′k (°)", "Charakteristisches φ′k (°)", phi_k.to_degrees()),
        Outcome::pascal("c_k", "Characteristic c′k", "Charakteristisches c′k", (a_k * scale_k).max(0.0)),
    ];
    if let Some(assumed) = c.opt("phi_assumed_deg") {
        out.insert(0, Outcome::ratio("phi_assumed", "Assumed φ′ vs characteristic φ′k", "Angesetztes φ′ gegen charakteristisches φ′k", rad(assumed).tan(), phi_k.tan()));
    }
    if let Some(assumed) = c.opt("c_assumed") {
        out.insert(0, Outcome::pressure("c_assumed", "Assumed c′ vs characteristic c′k", "Angesetztes c′ gegen charakteristisches c′k", assumed, (a_k * scale_k).max(0.0)));
    }
    Ok(out)
}

fn gi_14(c: &Case) -> R {
    let (ple, em) = (c.positive("p_le")?, c.positive("e_m")?);
    Ok(vec![Outcome::pascal("q_b_k", "Base resistance q_b,k = k_p·p_le*", "Spitzendruck q_b,k = k_p·p_le*", c.o("k_p", 1.0) * ple), Outcome::pascal("e_oed", "E_oed = E_M/α_M", "E_oed = E_M/α_M", em / c.o("alpha_m", 0.5))])
}

fn gi_15(c: &Case) -> R {
    let (d10, d30, d60) = (c.positive("d10")?, c.positive("d30")?, c.positive("d60")?);
    Ok(vec![Outcome::number("cu", "Uniformity coefficient C_u", "Ungleichförmigkeitszahl C_u", d60 / d10), Outcome::number("cc", "Curvature coefficient C_c", "Krümmungszahl C_c", d30 * d30 / (d10 * d60)), Outcome::number("k", "Permeability k = 0.0116·d10² (m/s, d10 in mm)", "Durchlässigkeit k = 0,0116·d10² (m/s, d10 in mm)", 0.0116 * d10 * d10)])
}

fn gi_16(c: &Case) -> R {
    let (q, r1, r2, h1, h2) = (c.positive("q")?, c.positive("r1")?, c.positive("r2")?, c.positive("h1")?, c.positive("h2")?);
    gate(r2 > r1 && h2 > h1, "observation wells must satisfy r2 > r1 and h2 > h1")?;
    Ok(vec![Outcome::number("k", "Permeability k (Thiem, m/s)", "Durchlässigkeit k (Thiem, m/s)", q * (r2 / r1).ln() / (std::f64::consts::PI * (h2 * h2 - h1 * h1)))])
}

pub const ENTRIES: &[Entry] = &[
    Entry { id: "G-01", part: "EN 1997-1", clause: "2.4.3", en: "Soil layering, stress profile", de: "Baugrundmodell, Spannungsverlauf", inputs: &["h1", "gamma1", "gamma_sat1", "phi1", "z", "z_w", "gamma_w"], eval: g_01 },
    Entry { id: "G-02", part: "EN 1997-1", clause: "2.4.5.2", en: "Characteristic values, statistical derivation", de: "Charakteristische Werte, statistische Auswertung", inputs: &["x1", "mode", "v_known"], eval: g_02 },
    Entry { id: "GI-01", part: "EN 1997-2", clause: "2.4", en: "Investigation extent", de: "Erkundungsumfang", inputs: &["b", "d_b", "gk", "z_have"], eval: gi_01 },
    Entry { id: "GI-02", part: "EN 1997-2", clause: "E", en: "CPT soil classification", de: "CPT-Klassifikation", inputs: &["qc", "fs", "u2", "a", "sigma_v0", "sigma_v0_eff"], eval: gi_02 },
    Entry { id: "GI-03", part: "EN 1997-2", clause: "D", en: "CPT undrained strength", de: "CPT undränierte Scherfestigkeit", inputs: &["qt", "sigma_v0", "nkt"], eval: gi_03 },
    Entry { id: "GI-04", part: "EN 1997-2", clause: "D", en: "CPT friction angle (Kulhawy-Mayne)", de: "CPT Reibungswinkel (Kulhawy-Mayne)", inputs: &["qc", "sigma_v0_eff"], eval: gi_04 },
    Entry { id: "GI-05", part: "EN 1997-2", clause: "D", en: "CPT oedometric modulus", de: "CPT Steifemodul", inputs: &["qc", "alpha_e"], eval: gi_05 },
    Entry { id: "GI-06", part: "EN 1997-2", clause: "F", en: "SPT correction", de: "SPT-Korrektur", inputs: &["n", "er", "cb", "cs", "cr", "sigma_v0_eff"], eval: gi_06 },
    Entry { id: "GI-07", part: "EN 1997-2", clause: "F", en: "SPT friction angle (Hatanaka-Uchida)", de: "SPT Reibungswinkel (Hatanaka-Uchida)", inputs: &["n1_60"], eval: gi_07 },
    Entry { id: "GI-08", part: "EN 1997-2", clause: "F", en: "SPT relative density (Skempton)", de: "SPT Lagerungsdichte (Skempton)", inputs: &["n1_60"], eval: gi_08 },
    Entry { id: "GI-09", part: "EN 1997-2", clause: "G", en: "Dynamic probing friction angle", de: "Rammsondierung Reibungswinkel", inputs: &["n10"], eval: gi_09 },
    Entry { id: "GI-10", part: "EN 1997-2", clause: "R", en: "Oedometer test evaluation", de: "Ödometerversuch Auswertung", inputs: &["e1", "sigma1", "h_dr", "t90"], eval: gi_10 },
    Entry { id: "GI-11", part: "EN 1997-2", clause: "S", en: "Triaxial test, Mohr-Coulomb envelope", de: "Triaxialversuch, Mohr-Coulomb-Einhüllende", inputs: &["p1", "q1"], eval: gi_11 },
    Entry { id: "GI-12", part: "EN 1997-2", clause: "S", en: "Direct shear test regression", de: "Rahmenscherversuch Regression", inputs: &["sigma1", "tau1"], eval: gi_12 },
    Entry { id: "GI-13", part: "EN 1997-2", clause: "2.4.5.2", en: "Characteristic value derivation", de: "Ableitung charakteristischer Werte", inputs: &["x1", "mode", "v_known"], eval: g_02 },
    Entry { id: "GI-17", part: "EN 1997-2", clause: "2.4.5.2", en: "Laboratory series, characteristic strength parameters", de: "Laborversuchsserie, charakteristische Festigkeitsparameter", inputs: &["sigma1", "tau1", "p1", "q1", "kind", "phi_assumed_deg", "c_assumed"], eval: gi_17 },
    Entry { id: "GI-14", part: "EN 1997-2", clause: "E", en: "Pressuremeter (Menard)", de: "Pressiometer (Menard)", inputs: &["p_le", "e_m", "k_p", "alpha_m"], eval: gi_14 },
    Entry { id: "GI-15", part: "EN 1997-2", clause: "5", en: "Grain-size classification and permeability", de: "Kornverteilung und Durchlässigkeit", inputs: &["d10", "d30", "d60"], eval: gi_15 },
    Entry { id: "GI-16", part: "EN 1997-2", clause: "6", en: "Pumping test permeability", de: "Pumpversuch Durchlässigkeit", inputs: &["q", "r1", "r2", "h1", "h2"], eval: gi_16 },
];
