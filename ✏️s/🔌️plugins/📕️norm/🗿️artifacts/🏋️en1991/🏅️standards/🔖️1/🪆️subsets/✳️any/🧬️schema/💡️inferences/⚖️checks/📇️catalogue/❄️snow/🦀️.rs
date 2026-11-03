//! ❄️ EN 1991-1-3 catalogue entries E13.01 … E13.12: roof snow load, ground snow load, shape coefficients, drifts, overhangs, snow guards, exceptional load, return periods and snow density.

use super::*;

fn ground(c: &Case) -> Result<f64, String> {
    match c.annex {
        AnnexChoice::De => {
            let zone = c.s("zone")?;
            part_1_3::ground_snow_pa(zone, c.n("altitude")?).ok_or_else(|| format!("unknown snow zone {zone}; admissible: {}", na_de::SnowZone::ADMISSIBLE.join(", ")))
        }
        AnnexChoice::En => c.pos("s_k"),
    }
}

fn e13_01(c: &Case) -> R {
    let (mu, ce, ct) = (c.n("mu")?, c.o("c_e", 1.0), c.o("c_t", 1.0));
    if mu < 0.0 || ce <= 0.0 || ct <= 0.0 { return Err("mu must not be negative and C_e, C_t must be positive".into()); }
    let sk = match c.opt("s_k") { Some(v) if v > 0.0 => v, _ => ground(c)? };
    let s = part_1_3::roof_snow_pa(sk, mu, ce, ct);
    Ok(vec![Outcome::info("s", "Roof snow load s", "Dachschneelast s", QuantityKind::Pressure, s).steps(format!("s = μ·C_e·C_t·s_k = {mu:.3}·{ce}·{ct}·{:.4} = {:.4} kN/m².", sk / 1000.0, s / 1000.0))])
}

fn e13_02(c: &Case) -> R {
    let sk = if c.de() {
        ground(c)?
    } else if let Ok(zone) = c.s("cen_zone") {
        let (z, a) = (c.n("z")?, c.n("altitude")?);
        let kn = match zone {
            "alpine" => (0.642 * z + 0.009) * (1.0 + (a / 728.0).powi(2)),
            "central_east" => (0.264 * z - 0.002) * (1.0 + (a / 256.0).powi(2)),
            "mediterranean" => (0.498 * z - 0.209) * (1.0 + (a / 452.0).powi(2)),
            other => return Err(format!("unknown CEN zone {other}; admissible: alpine, central_east, mediterranean")),
        };
        kn * 1000.0
    } else { ground(c)? };
    let basis: &[&'static str] = if c.de() { &["ndp.en1991.e13.na-zones"] } else { &["ndp.en1991.e13.en-annex-c"] };
    let mut out = vec![Outcome::info("s_k", "Ground snow load s_k", "Schneelast auf dem Boden s_k", QuantityKind::Pressure, sk).steps(format!("s_k = {:.5} kN/m².", sk / 1000.0)).basis(basis)];
    if c.de() && c.o("north_german_lowland", 0.0) > 0.5 { out.push(Outcome::info("s_Ad", "Exceptional snow load s_Ad", "Außergewöhnliche Schneelast s_Ad", QuantityKind::Pressure, part_1_3::exceptional_snow_pa(sk)).steps("s_Ad = 2.3·s_k (North German lowland).").steps_de("s_Ad = 2,3·s_k (Norddeutsches Tiefland).").basis(&["ndp.en1991.e13.na-zones"])); }
    Ok(out)
}

fn e13_03(c: &Case) -> R {
    let (a1, a2) = (c.n("alpha")?, c.o("alpha_2", c.n("alpha")?));
    let m1 = part_1_3::mu_1(a1);
    let mut out = vec![Outcome::info("mu1", "Shape coefficient μ₁", "Formbeiwert μ₁", QuantityKind::Dimensionless, m1).steps(format!("μ₁({a1:.1}°) = {m1:.4}."))];
    let m1b = part_1_3::mu_1(a2);
    out.push(Outcome::info("case_i", "Undrifted: both slopes", "Unverweht: beide Seiten", QuantityKind::Dimensionless, m1.max(m1b)).steps(format!("μ₁(α₁) = {m1:.4}, μ₁(α₂) = {m1b:.4}.")));
    out.push(Outcome::info("case_ii", "Drifted: 0.5 μ₁(α₁) and μ₁(α₂)", "Verweht: 0,5 μ₁(α₁) und μ₁(α₂)", QuantityKind::Dimensionless, (0.5 * m1).max(m1b)).steps("Duopitch load arrangement (ii).").steps_de("Lastanordnung (ii) des Satteldachs."));
    out.push(Outcome::info("case_iii", "Drifted: μ₁(α₁) and 0.5 μ₁(α₂)", "Verweht: μ₁(α₁) und 0,5 μ₁(α₂)", QuantityKind::Dimensionless, m1.max(0.5 * m1b)).steps("Duopitch load arrangement (iii).").steps_de("Lastanordnung (iii) des Satteldachs."));
    Ok(out)
}

fn e13_04(c: &Case) -> R {
    let (a1, a2) = (c.n("alpha_1")?, c.n("alpha_2")?);
    let mean = 0.5 * (a1 + a2);
    let mu2 = part_1_3::mu_2(mean).min(1.6);
    Ok(vec![
        Outcome::info("mu1", "Outer slopes μ₁", "Außenfelder μ₁", QuantityKind::Dimensionless, part_1_3::mu_1(a1).max(part_1_3::mu_1(a2))).steps("μ₁(α) per slope.").steps_de("μ₁(α) je Dachfläche."),
        Outcome::info("mu2", "Valley μ₂", "Kehle μ₂", QuantityKind::Dimensionless, mu2).steps(format!("α = {mean:.1}°: μ₂ = 0.8 + 0.8α/30 ≤ 1.6 = {mu2:.4}.")),
    ])
}

fn e13_05(c: &Case) -> R {
    let (b, h) = (c.pos("b")?, c.pos("h")?);
    let m3 = part_1_3::mu_3(h, b);
    Ok(vec![
        Outcome::info("mu3", "Cylindrical roof μ₃", "Tonnendach μ₃", QuantityKind::Dimensionless, m3).steps(format!("μ₃ = 0.2 + 10h/b ≤ 2.0 = {m3:.4}.")),
        Outcome::info("mu3_drifted", "Drifted: 0.5 μ₃", "Verweht: 0,5 μ₃", QuantityKind::Dimensionless, 0.5 * m3).steps("Load case drifted.").steps_de("Lastfall mit Schneeverwehung."),
    ])
}

fn e13_06(c: &Case) -> R {
    let (h, sk) = (c.pos("h")?, c.pos("s_k")?);
    let (raw, capped, ls) = part_1_3::obstruction_drift(h, sk, c.annex);
    let mut out = vec![
        Outcome::info("mu2_raw", "Drift coefficient γh/s_k", "Verwehungsbeiwert γh/s_k", QuantityKind::Dimensionless, raw).steps(format!("γh/s_k = 2·{h}/{:.3} = {raw:.5}.", sk / 1000.0)).basis(&["ndp.en1991.e13.drift"]),
        Outcome::info("mu2", "Drift coefficient μ₂ (0.8 ≤ μ₂ ≤ 2.0)", "Verwehungsbeiwert μ₂ (0,8 ≤ μ₂ ≤ 2,0)", QuantityKind::Dimensionless, capped).steps(format!("μ₂ = {capped:.4}.")).basis(&["ndp.en1991.e13.drift"]),
        Outcome::info("l_s", "Drift length l_s", "Verwehungslänge l_s", QuantityKind::Length, ls).steps(format!("l_s = 2h within 5 to 15 m = {ls:.2} m.")).steps_de(format!("l_s = 2h innerhalb 5 bis 15 m = {ls:.2} m.")).basis(&["ndp.en1991.e13.drift"]),
    ];
    if let (Some(b1), Some(b2)) = (c.opt("b1"), c.opt("b2")) {
        let mu_w = part_1_3::drift_mu_w(h, b1, b2, sk, c.annex);
        out.push(Outcome::info("mu_w", "Wind-induced μ_w", "Windinduziertes μ_w", QuantityKind::Dimensionless, mu_w).steps(format!("μ_w = (b₁+b₂)/2h limited by γh/s_k = {mu_w:.4}.")).steps_de(format!("μ_w = (b₁+b₂)/2h begrenzt durch γh/s_k = {mu_w:.4}.")).basis(&["ndp.en1991.e13.drift"]));
    }
    Ok(out)
}

fn e13_07(c: &Case) -> R {
    let s = c.pos("s")?;
    let d = s / 3000.0;
    let se = part_1_3::overhang_line_load_n_m(s);
    Ok(vec![
        Outcome::info("d", "Snow depth d", "Schneehöhe d", QuantityKind::Length, d).steps(format!("d = s/γ = {d:.4} m.")).basis(&["ndp.en1991.e13.overhang"]),
        Outcome::info("s_e", "Overhang line load s_e", "Linienlast Überstand s_e", QuantityKind::Force, se).steps(format!("s_e = k s²/γ = {:.5} kN/m.", se / 1000.0)).basis(&["ndp.en1991.e13.overhang"]),
    ])
}

fn e13_08(c: &Case) -> R {
    let (s, b, a) = (c.pos("s")?, c.pos("b")?, c.n("alpha")?);
    let f = part_1_3::snow_guard_force_n_m(s, b, a);
    Ok(vec![Outcome::info("F_s", "Snow guard force F_s", "Schneefangkraft F_s", QuantityKind::Force, f).steps(format!("F_s = s·b·sin α = {:.4} kN/m.", f / 1000.0))])
}

fn e13_13(c: &Case) -> R {
    let s = c.pos("s")?;
    let se = part_1_3::overhang_line_load_n_m(s);
    let mut out = vec![Outcome::info("s_e", "Overhang line load s_e", "Linienlast Überstand s_e", QuantityKind::Force, se).steps(format!("s_e = k s²/γ = {:.5} kN/m.", se / 1000.0)).basis(&["ndp.en1991.e13.overhang"])];
    if let (Some(b), Some(a)) = (c.opt("b"), c.opt("alpha")) {
        let f = part_1_3::snow_guard_force_n_m(s, b, a);
        out.push(Outcome::info("F_s", "Snow guard force F_s", "Schneefangkraft F_s", QuantityKind::Force, f).steps(format!("F_s = s·b·sin α = {:.4} kN/m.", f / 1000.0)));
    }
    Ok(out)
}

fn e13_09(c: &Case) -> R {
    let sk = ground(c)?;
    let ad = if c.de() {
        if c.o("north_german_lowland", 0.0) < 0.5 { return Err("exceptional snow load applies to the North German lowland only".into()); }
        part_1_3::exceptional_snow_pa(sk)
    } else { c.o("c_esl", 2.0) * sk };
    Ok(vec![Outcome::info("s_Ad", "Exceptional snow load s_Ad", "Außergewöhnliche Schneelast s_Ad", QuantityKind::Pressure, ad).steps(format!("s_Ad = {:.4} kN/m².", ad / 1000.0)).basis(&["ndp.en1991.e13.na-zones"])])
}

fn e13_10(c: &Case) -> R {
    let (v, n) = (c.pos("v")?, c.pos("n_years")?);
    if n <= 1.0 { return Err("input n_years must exceed 1".into()); }
    let f = part_1_3::return_period_factor(v, 1.0 / n);
    Ok(vec![Outcome::info("factor", "Ratio s_n/s_k", "Verhältnis s_n/s_k", QuantityKind::Dimensionless, f).steps(format!("V = {v}, n = {n} a: s_n/s_k = {f:.5}."))])
}

fn e13_11(c: &Case) -> R {
    let kn = match c.s("state")? { "fresh" => 1.0, "settled_days" => 2.0, "settled_months" => 3.0, "wet" => 4.0, other => return Err(format!("unknown snow state {other}; admissible: fresh, settled_days, settled_months, wet")) };
    Ok(vec![Outcome::info("gamma", "Snow density γ", "Schneewichte γ", QuantityKind::Dimensionless, kn * 1000.0).steps(format!("γ = {kn} kN/m³.")).basis(&["ndp.en1991.e13.weights"])])
}

fn e13_12(c: &Case) -> R {
    let (ce, ct, mu, sk) = (c.n("c_e")?, c.n("c_t")?, c.n("mu")?, c.pos("s_k")?);
    let s = mu * ce * ct * sk;
    Ok(vec![Outcome::info("s", "Roof snow load for exposure and thermal coefficients", "Dachschneelast mit Umgebungs- und Temperaturbeiwert", QuantityKind::Pressure, s).steps(format!("s = {:.4} kN/m².", s / 1000.0))])
}

pub(super) const ENTRIES: &[Entry] = &[
    Entry { id: "E13.01", standard: "EN 1991-1-3", clause: "5.2", en: "Snow load on roofs", de: "Schneelast auf Dächern", inputs: &["mu", "c_e", "c_t", "s_k", "zone", "altitude"], eval: e13_01 },
    Entry { id: "E13.02", standard: "EN 1991-1-3", clause: "4.1", en: "Ground snow load", de: "Schneelast auf dem Boden", inputs: &["zone", "altitude", "s_k", "cen_zone", "z", "north_german_lowland"], eval: e13_02 },
    Entry { id: "E13.03", standard: "EN 1991-1-3", clause: "5.3.2", en: "Mono-pitch and duo-pitch roofs", de: "Pult- und Satteldächer", inputs: &["alpha", "alpha_2"], eval: e13_03 },
    Entry { id: "E13.04", standard: "EN 1991-1-3", clause: "5.3.4", en: "Multi-span roofs", de: "Mehrfeldrige Dächer", inputs: &["alpha_1", "alpha_2"], eval: e13_04 },
    Entry { id: "E13.05", standard: "EN 1991-1-3", clause: "5.3.5", en: "Cylindrical roofs", de: "Zylindrische Dächer", inputs: &["b", "h"], eval: e13_05 },
    Entry { id: "E13.06", standard: "EN 1991-1-3", clause: "5.3.6", en: "Drifts at obstructions and parapets", de: "Schneeverwehung an Aufbauten und Attiken", inputs: &["h", "s_k", "b1", "b2"], eval: e13_06 },
    Entry { id: "E13.07", standard: "EN 1991-1-3", clause: "5.3.7", en: "Snow overhanging the eaves", de: "Schneeüberhang an der Traufe", inputs: &["s"], eval: e13_07 },
    Entry { id: "E13.08", standard: "EN 1991-1-3", clause: "5.3.8", en: "Snow guards", de: "Schneefanggitter", inputs: &["s", "b", "alpha"], eval: e13_08 },
    Entry { id: "E13.09", standard: "EN 1991-1-3", clause: "4.3 / B", en: "Exceptional snow loads", de: "Außergewöhnliche Schneelasten", inputs: &["zone", "altitude", "s_k", "north_german_lowland", "c_esl"], eval: e13_09 },
    Entry { id: "E13.10", standard: "EN 1991-1-3", clause: "D", en: "Adjustment to the return period", de: "Anpassung an die Wiederkehrperiode", inputs: &["v", "n_years"], eval: e13_10 },
    Entry { id: "E13.11", standard: "EN 1991-1-3", clause: "E", en: "Snow density", de: "Wichte des Schnees", inputs: &["state"], eval: e13_11 },
    Entry { id: "E13.12", standard: "EN 1991-1-3", clause: "5.2", en: "Exposure and thermal coefficients", de: "Umgebungs- und Temperaturbeiwert", inputs: &["c_e", "c_t", "mu", "s_k"], eval: e13_12 },
    Entry { id: "E13.13", standard: "EN 1991-1-3", clause: "6", en: "Local effects: overhangs and sliding snow", de: "Örtliche Einwirkungen: Überstand und abrutschender Schnee", inputs: &["s", "b", "alpha"], eval: e13_13 },
];
