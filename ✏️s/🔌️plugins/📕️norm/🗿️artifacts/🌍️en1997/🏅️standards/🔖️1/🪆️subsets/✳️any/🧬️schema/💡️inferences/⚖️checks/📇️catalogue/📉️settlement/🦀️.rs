//! 📉️ Settlement and consolidation, catalogue entries SLS-01 … SLS-04 (EN 1997-1 §6.6 and Annex F, DIN 4019).

use super::*;

/// 🧮️ Time factor T90 of the Terzaghi theory (U = 90 %), the root of [`degree_exact`], so `cv = T90 h² / t90`; theory, not a registry parameter.
pub fn t90_factor() -> f64 {
    time_factor_exact(0.9)
}

/// 📐️ Vertical stress under the centre of a uniformly loaded circle of radius a at depth z.
pub fn sigma_z_circle(sigma0: f64, a: f64, z: f64) -> f64 {
    sigma0 * (1.0 - (1.0 / (1.0 + (a / z).powi(2))).powf(1.5))
}

/// 📐️ Newmark influence factor of the corner of a rectangle m = a/z, n = b/z.
pub fn newmark_corner(m: f64, n: f64) -> f64 {
    let s = m * m + n * n + 1.0;
    let p = m * m * n * n;
    let first = 2.0 * m * n * s.sqrt() / (s + p) * (s + 1.0) / s;
    let den = s - p;
    let angle = (2.0 * m * n * s.sqrt()).atan2(den);
    (first + angle) / (4.0 * std::f64::consts::PI)
}

/// 📐️ Vertical stress under the centre of a rectangle B × L (corner method with four sub-rectangles).
pub fn sigma_z_rect_centre(sigma0: f64, b: f64, l: f64, z: f64) -> f64 {
    sigma0 * 4.0 * newmark_corner(0.5 * b / z, 0.5 * l / z)
}

/// 📐️ Vertical stress of a uniform `b × l` rectangle at offset `(px, py)` from its centre and depth `z` (corner method with signed corners, so the point may lie outside the loaded area).
pub fn sigma_z_rect_at(sigma0: f64, b: f64, l: f64, px: f64, py: f64, z: f64) -> f64 {
    let corner = |u: f64, v: f64| u.signum() * v.signum() * newmark_corner(u.abs() / z, v.abs() / z);
    let (u1, u2, v1, v2) = (0.5 * b - px, -0.5 * b - px, 0.5 * l - py, -0.5 * l - py);
    sigma0 * (corner(u1, v1) - corner(u2, v1) - corner(u1, v2) + corner(u2, v2))
}

/// 📐️ Vertical surface displacement of the elastic half-space at offset `(px, py)` from the centre of a flexible uniform `b × l` rectangle (Love's corner solution with signed corners); `e_pa` is Young's modulus.
pub fn displacement_rect_at(q_pa: f64, b: f64, l: f64, px: f64, py: f64, nu: f64, e_pa: f64) -> f64 {
    let corner = |u: f64, v: f64| {
        let (a, c) = (u.abs(), v.abs());
        if a < 1e-12 || c < 1e-12 {
            return 0.0;
        }
        let r = a.hypot(c);
        u.signum() * v.signum() * (a * ((c + r) / a).ln() + c * ((a + r) / c).ln())
    };
    let (u1, u2, v1, v2) = (0.5 * b - px, -0.5 * b - px, 0.5 * l - py, -0.5 * l - py);
    q_pa * (1.0 - nu * nu) / (std::f64::consts::PI * e_pa.max(1.0)) * (corner(u1, v1) - corner(u2, v1) - corner(u1, v2) + corner(u2, v2))
}

/// 🧮️ Stress-strain (oedometric) settlement at offset `(px, py)` from the centre of a loaded `b × l` rectangle at `embedment`: the layers below its base to twice the longer side, 10 sub-layers each, stress by [`sigma_z_rect_at`] from the loaded base level.
pub fn stress_strain_settlement_at(layers: &[SoilLayer], b: f64, l: f64, embedment: f64, q_pa: f64, px: f64, py: f64) -> f64 {
    let limit = 2.0 * b.max(l);
    let mut total = 0.0;
    for layer in layers {
        let (top, bottom) = (layer.depth_top.max(embedment), layer.depth_bottom.min(embedment + limit));
        if bottom <= top || layer.oedometric_modulus <= 0.0 {
            continue;
        }
        let n = 10;
        let h = (bottom - top) / n as f64;
        total += (0..n).map(|i| sigma_z_rect_at(q_pa, b, l, px, py, (top + (i as f64 + 0.5) * h - embedment).max(1e-3)) * h / layer.oedometric_modulus).sum::<f64>();
    }
    total
}

/// 📐️ Vertical stress under the centre of an infinitely long strip of width b.
pub fn sigma_z_strip_centre(sigma0: f64, b: f64, z: f64) -> f64 {
    let alpha = 2.0 * (0.5 * b / z).atan();
    sigma0 / std::f64::consts::PI * (alpha + alpha.sin())
}

/// 📐️ Time factor T_v(U) of one-dimensional consolidation.
pub fn time_factor(u: f64) -> f64 {
    if u < 0.6 { std::f64::consts::FRAC_PI_4 * u * u } else { -0.9332 * (1.0 - u).log10() - 0.0851 }
}

fn sls_01(c: &Case) -> R {
    let sigma0 = c.positive("sigma0")?;
    let shape = c.so("shape", "rect");
    let heights = c.series("h");
    let moduli = c.series("e");
    gate(!heights.is_empty() && heights.len() == moduli.len(), "layer series h1…hn and e1…en required")?;
    let mut z_top = c.o("z_start", 0.0);
    let mut total = 0.0;
    let mut out = Vec::new();
    for (i, (h, e)) in heights.iter().zip(&moduli).enumerate() {
        let z = z_top + 0.5 * h;
        let sigma = match shape {
            "circle" => sigma_z_circle(sigma0, c.positive("a")?, z),
            "strip" => sigma_z_strip_centre(sigma0, c.positive("b")?, z),
            "two_to_one" => sigma0 * c.positive("b")? * c.positive("l")? / ((c.n("b")? + z) * (c.n("l")? + z)),
            _ => sigma_z_rect_centre(sigma0, c.positive("b")?, c.positive("l")?, z),
        };
        let s_i = sigma * h / e;
        total += s_i;
        out.push(Outcome::pascal(&format!("sigma_z{}", i + 1), "Vertical stress at mid-layer", "Vertikalspannung Schichtmitte", sigma));
        z_top += h;
    }
    total *= c.o("rigid_factor", 1.0);
    let mut res = vec![Outcome::metre("s", "Settlement Σσz·h/E", "Setzung Σσz·h/E", total)];
    if let Some(limit) = c.opt("s_lim") {
        res.insert(0, Outcome::length("serviceability", "Settlement s vs limit", "Setzung s gegen Grenzwert", total, limit));
    }
    res.extend(out);
    Ok(res)
}

fn sls_01b(c: &Case) -> R {
    let (p, e, nu) = (c.positive("p")?, c.positive("e_m")?, c.o("nu", 0.3));
    let s = match c.so("shape", "circle_flexible") {
        "circle_rigid" => std::f64::consts::PI * p * c.positive("a")? * (1.0 - nu * nu) / (2.0 * e),
        "factor" => p * c.positive("b")? * c.positive("f")? / e,
        _ => 2.0 * p * c.positive("a")? * (1.0 - nu * nu) / e,
    };
    Ok(vec![Outcome::metre("s", "Elastic settlement", "Elastische Setzung", s)])
}

fn sls_02(c: &Case) -> R {
    let (h, cc, e0, s0, ds) = (c.positive("h")?, c.positive("cc")?, c.positive("e0")?, c.positive("sigma0")?, c.n("dsigma")?);
    let cs = c.o("cs", 0.1 * cc);
    let sigma_p = c.o("sigma_p", s0);
    let s = if s0 + ds <= sigma_p {
        cs * h / (1.0 + e0) * ((s0 + ds) / s0).log10()
    } else if sigma_p > s0 {
        cs * h / (1.0 + e0) * (sigma_p / s0).log10() + cc * h / (1.0 + e0) * ((s0 + ds) / sigma_p).log10()
    } else {
        cc * h / (1.0 + e0) * ((s0 + ds) / s0).log10()
    };
    let mut out = vec![Outcome::metre("s_c", "Primary consolidation settlement", "Konsolidationssetzung", s)];
    if let (Some(cv), Some(hdr)) = (c.opt("cv"), c.opt("h_dr")) {
        let t90 = t90_factor() * hdr * hdr / cv;
        out.push(Outcome::info(QuantityKind::Time, "t90", "Time to 90 % consolidation", "Zeit bis 90 % Konsolidation", t90));
        if let Some(t) = c.opt("t") {
            let tv = cv * t / (hdr * hdr);
            let u = if tv < std::f64::consts::FRAC_PI_4 * 0.36 { (4.0 * tv / std::f64::consts::PI).sqrt() } else { 1.0 - 10f64.powf(-(tv + 0.0851) / 0.9332) };
            out.push(Outcome::number("u_avg", "Average degree of consolidation", "Mittlerer Konsolidationsgrad", u));
            out.push(Outcome::metre("s_t", "Settlement at time t", "Setzung zur Zeit t", s * u));
        }
    }
    Ok(out)
}

fn sls_03(c: &Case) -> R {
    let (h, ca, e0, t1, t2) = (c.positive("h")?, c.positive("c_alpha")?, c.positive("e0")?, c.positive("t1")?, c.positive("t2")?);
    gate(t2 > t1, "t2 must exceed t1")?;
    Ok(vec![Outcome::metre("s_sec", "Secondary settlement", "Sekundärsetzung", ca * h / (1.0 + e0) * (t2 / t1).log10())])
}

fn sls_04(c: &Case) -> R {
    let (sa, sb, l) = (c.n("s_a")?, c.n("s_b")?, c.positive("l")?);
    let tilt = (sa - sb).abs() / l;
    let limit = 1.0 / c.o("limit_den", 500.0);
    Ok(vec![Outcome::ratio("tilt", "Angular distortion vs limit", "Winkelverdrehung gegen Grenzwert", tilt, limit), Outcome::number("tan_omega", "Tilt tan ω", "Schiefstellung tan ω", tilt)])
}

/// 📈️ Average degree of consolidation by the exact Terzaghi series, `U = 1 - Σ 2/M² e^(-M² Tv)` with `M = π(2m+1)/2`, for a layer drained on one side (or half of a layer drained on both); the single source of the catalogue and the scene curves.
pub fn degree_exact(tv: f64) -> f64 {
    if tv <= 0.0 {
        return 0.0;
    }
    (1.0 - (0..400).map(|m| { let k = std::f64::consts::FRAC_PI_2 * f64::from(2 * m + 1); 2.0 / (k * k) * (-k * k * tv).exp() }).sum::<f64>()).clamp(0.0, 1.0)
}

/// 📈️ Time factor `Tv` of the degree of consolidation `u` in (0, 1), the inverse of [`degree_exact`] by bisection.
pub fn time_factor_exact(u: f64) -> f64 {
    let (mut lo, mut hi) = (0.0, 20.0);
    for _ in 0..200 {
        let mid = 0.5 * (lo + hi);
        if degree_exact(mid) < u { lo = mid } else { hi = mid }
    }
    0.5 * (lo + hi)
}

fn sls_05(c: &Case) -> R {
    let (s_final, h_dr) = (c.positive("s_final")?, c.positive("h_dr")?);
    let cv = match (c.opt("cv"), c.opt("t90_lab"), c.opt("h_dr_lab")) {
        (Some(cv), ..) if cv > 0.0 => cv,
        (_, Some(t90), Some(h)) if t90 > 0.0 && h > 0.0 => time_factor_exact(0.9) * h * h / t90,
        _ => return Err("c_v, or the laboratory pair t90_lab and h_dr_lab, required".into()),
    };
    let time_of = |u: f64| time_factor_exact(u) * h_dr * h_dr / cv;
    let mut out = vec![
        Outcome::number("cv", "Coefficient of consolidation c_v (m²/s)", "Konsolidierungsbeiwert c_v (m²/s)", cv),
        Outcome::info(QuantityKind::Time, "t50", "Time to 50 % consolidation", "Zeit bis 50 % Konsolidation", time_of(0.5)),
        Outcome::info(QuantityKind::Time, "t90", "Time to 90 % consolidation", "Zeit bis 90 % Konsolidation", time_of(0.9)),
    ];
    if let Some(t) = c.opt("t") {
        let u = degree_exact(cv * t / (h_dr * h_dr));
        out.push(Outcome::number("u_avg", "Average degree of consolidation U(t)", "Mittlerer Konsolidationsgrad U(t)", u));
        out.push(Outcome::metre("s_t", "Settlement at time t", "Setzung zur Zeit t", s_final * u));
        out.push(Outcome::metre("s_remaining", "Remaining settlement after t", "Restsetzung nach t", s_final * (1.0 - u)));
        if let Some(limit) = c.opt("s_res_lim") {
            out.insert(0, Outcome::length("residual", "Remaining settlement vs admissible residual", "Restsetzung gegen zulässige Restsetzung", s_final * (1.0 - u), limit));
        }
    }
    Ok(out)
}

pub const ENTRIES: &[Entry] = &[
    Entry { id: "SLS-01", part: "EN 1997-1", clause: "6.6.2", en: "Settlement, stress-strain method", de: "Setzung, Spannungs-Dehnungs-Verfahren", inputs: &["sigma0", "shape", "a", "b", "l", "h1", "e1", "z_start", "rigid_factor", "s_lim"], eval: sls_01 },
    Entry { id: "SLS-01B", part: "EN 1997-1", clause: "F.2", en: "Elastic settlement (Annex F.2)", de: "Elastische Setzung (Anhang F.2)", inputs: &["p", "e_m", "nu", "shape", "a", "b", "f"], eval: sls_01b },
    Entry { id: "SLS-02", part: "EN 1997-1", clause: "6.6.2", en: "Primary consolidation and time-settlement", de: "Primärkonsolidation und Zeit-Setzung", inputs: &["h", "cc", "cs", "e0", "sigma0", "dsigma", "sigma_p", "cv", "h_dr", "t"], eval: sls_02 },
    Entry { id: "SLS-03", part: "EN 1997-1", clause: "6.6.3", en: "Secondary settlement (creep)", de: "Sekundärsetzung (Kriechen)", inputs: &["h", "c_alpha", "e0", "t1", "t2"], eval: sls_03 },
    Entry { id: "SLS-04", part: "EN 1997-1", clause: "H", en: "Differential settlement and tilt", de: "Setzungsdifferenz und Schiefstellung", inputs: &["s_a", "s_b", "l", "limit_den"], eval: sls_04 },
    Entry { id: "SLS-05", part: "EN 1997-1", clause: "6.6.2", en: "Consolidation time curve, exact series and oedometer c_v", de: "Konsolidationszeitverlauf, exakte Reihe und Ödometer-c_v", inputs: &["s_final", "h_dr", "cv", "t90_lab", "h_dr_lab", "t", "s_res_lim"], eval: sls_05 },
];
