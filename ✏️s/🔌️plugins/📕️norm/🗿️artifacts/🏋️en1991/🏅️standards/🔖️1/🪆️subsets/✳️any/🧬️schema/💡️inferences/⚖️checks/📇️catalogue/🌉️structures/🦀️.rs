//! 🌉️ EN 1991-2, 1991-3 and 1991-4 catalogue entries E2.01 … E2.17, E3.01 … E3.07 and E4.01 … E4.08: traffic loads on bridges, crane actions and silo and tank loads.

use super::*;

fn e2_01(c: &Case) -> R {
    let w = c.pos("width")?;
    let (n, wl, rem) = part_2::notional_lanes(w);
    Ok(vec![
        Outcome::info("n_lanes", "Number of notional lanes n₁", "Anzahl ideeller Fahrstreifen n₁", QuantityKind::Dimensionless, f64::from(n)).steps(format!("w = {w} m → n₁ = {n}.")),
        Outcome::info("w_lane", "Lane width w_l", "Fahrstreifenbreite w_l", QuantityKind::Length, wl).steps(format!("w_l = {wl:.3} m.")),
        Outcome::info("w_rem", "Remaining area width", "Breite der Restfläche", QuantityKind::Length, rem).steps(format!("Remaining = {rem:.3} m.")),
    ])
}

fn e2_02(c: &Case) -> R {
    let w = c.pos("width")?;
    let (n, wl, rem) = part_2::notional_lanes(w);
    let tandems: f64 = (1..=n).map(|l| part_2::lm1_tandem_n(c.annex, l)).sum();
    let mut out = vec![Outcome::info("tandems", "Sum of tandem systems", "Summe der Tandemsysteme", QuantityKind::Force, 2.0 * tandems).steps(format!("2·ΣQ_ik = {:.1} kN over {n} lanes.", 2.0 * tandems / 1000.0)).steps_de(format!("2·ΣQ_ik = {:.1} kN über {n} Fahrstreifen.", 2.0 * tandems / 1000.0)).basis(&["ndp.en1991.e2.na-factors"])];
    for lane in 1..=n { out.push(Outcome::info(&format!("udl_{lane}"), &format!("UDL of lane {lane}"), &format!("Gleichlast Fahrstreifen {lane}"), QuantityKind::Force, part_2::lm1_udl_pa(c.annex, lane) * wl).steps(format!("q_ik·w_l = {:.3} kN/m.", part_2::lm1_udl_pa(c.annex, lane) * wl / 1000.0)).basis(&["ndp.en1991.e2.na-factors"])); }
    out.push(Outcome::info("udl_rest", "UDL of the remaining area", "Gleichlast der Restfläche", QuantityKind::Force, part_2::remaining_area_udl_pa(c.annex) * rem).steps(format!("q_r·w_r = {:.3} kN/m.", part_2::remaining_area_udl_pa(c.annex) * rem / 1000.0)).basis(&["ndp.en1991.e2.na-factors"]));
    Ok(out)
}

fn e2_03(c: &Case) -> R {
    let q = c.o("axle", part_2::lm2_axle_n(c.annex));
    let (a, b) = (c.o("a", 0.35), c.o("b", 0.60));
    let wheel = q / 2.0;
    Ok(vec![
        Outcome::info("axle", "LM2 axle load", "LM2-Achslast", QuantityKind::Force, q).steps(format!("β_Q·400 kN = {:.1} kN.", q / 1000.0)).basis(&["ndp.en1991.e2.na-factors"]),
        Outcome::info("wheel_pressure", "LM2 wheel contact pressure", "LM2 Radaufstandspressung", QuantityKind::Pressure, part_2::wheel_pressure_pa(wheel, a, b)).steps(format!("{:.1} kN on {a}×{b} m.", wheel / 1000.0)),
    ])
}

fn e2_04(c: &Case) -> R {
    let (n, axle) = (c.pos("axles")?, c.pos("axle_load")?);
    Ok(vec![Outcome::info("total", "Total weight of the special vehicle", "Gesamtgewicht des Sonderfahrzeugs", QuantityKind::Force, n * axle).steps(format!("{n:.0} axle lines · {:.0} kN = {:.0} kN, spacing 1.5 m.", axle / 1000.0, n * axle / 1000.0)).steps_de(format!("{n:.0} Achslinien · {:.0} kN = {:.0} kN, Abstand 1,5 m.", axle / 1000.0, n * axle / 1000.0)).basis(&["ndp.en1991.e2.na-factors"])])
}

fn e2_05(_c: &Case) -> R { Ok(vec![Outcome::info("q_fk", "Crowd loading q_fk", "Menschenmenge q_fk", QuantityKind::Pressure, part_2::lm4_crowd_pa(AnnexChoice::En)).steps("q_fk = 5 kN/m².")]) }

fn e2_06(c: &Case) -> R {
    let (a, b, load) = (c.pos("a")?, c.pos("b")?, c.pos("load")?);
    let spread = c.o("pavement", 0.0) + c.o("slab_thickness", 0.0) / 2.0;
    let (sa, sb) = (a + 2.0 * spread, b + 2.0 * spread);
    Ok(vec![
        Outcome::info("footprint", "Dispersed footprint at mid-plane", "Ausgebreitete Aufstandsfläche in Plattenmitte", QuantityKind::Area, sa * sb).steps(format!("{sa:.3}×{sb:.3} m after 45° dispersal through {spread:.3} m.")).steps_de(format!("{sa:.3}×{sb:.3} m nach 45°-Ausbreitung über {spread:.3} m.")),
        Outcome::info("pressure", "Dispersed pressure", "Ausgebreitete Pressung", QuantityKind::Pressure, part_2::wheel_pressure_pa(load, sa, sb)).steps(format!("p = {:.2} kN/m².", part_2::wheel_pressure_pa(load, sa, sb) / 1000.0)),
    ])
}

fn e2_07(c: &Case) -> R {
    let l = c.pos("loaded_length")?;
    let q = part_2::braking_force_kn(c.annex, l, 3.0) * 1000.0;
    Ok(vec![Outcome::info("Q_lk", "Braking force Q_lk", "Bremskraft Q_lk", QuantityKind::Force, q).steps(format!("Q_lk = 0.6·2·Q_1k + 0.1·q_1k·w_1·L within 180 to 900 kN = {:.1} kN.", q / 1000.0)).steps_de(format!("Q_lk = 0,6·2·Q_1k + 0,1·q_1k·w_1·L innerhalb 180 bis 900 kN = {:.1} kN.", q / 1000.0)).basis(&["ndp.en1991.e2.na-factors"])])
}

fn e2_08(c: &Case) -> R {
    let (qv, r) = (c.pos("q_v")?, c.pos("radius")?);
    let f = part_2::centrifugal_force_kn(c.annex, qv / 1000.0, r) * 1000.0;
    Ok(vec![Outcome::info("Q_tk", "Centrifugal force Q_tk", "Fliehkraft Q_tk", QuantityKind::Force, f).steps(format!("r = {r} m: Q_tk = {:.1} kN.", f / 1000.0))])
}

fn e2_09(c: &Case) -> R {
    let g = c.s("group")?;
    let m = part_2::group_members(g).ok_or_else(|| format!("unknown load group {g}; admissible: {}", part_2::GROUPS.join(", ")))?;
    let flag = |b: bool| if b { 1.0 } else { 0.0 };
    let mut out = vec![
        Outcome::info("lm1", "LM1 included", "LM1 enthalten", QuantityKind::Dimensionless, flag(m.lm1)).steps(format!("Group {g}.")),
        Outcome::info("lm2", "LM2 included", "LM2 enthalten", QuantityKind::Dimensionless, flag(m.lm2)).steps(format!("Group {g}.")),
        Outcome::info("lm3", "LM3 included", "LM3 enthalten", QuantityKind::Dimensionless, flag(m.lm3)).steps(format!("Group {g}.")),
        Outcome::info("lm4", "LM4 included", "LM4 enthalten", QuantityKind::Dimensionless, flag(m.lm4)).steps(format!("Group {g}.")),
    ];
    if m.footway { out.push(Outcome::info("footway", "Footway load of the group", "Gehweglast der Gruppe", QuantityKind::Pressure, if g == "gr1a" { part_2::footway_reduced_pa() } else { part_2::footway_pa(c.annex) }).steps("Reduced to 3 kN/m² in gr1a, 5 kN/m² otherwise.").steps_de("Auf 3 kN/m² in gr1a reduziert, sonst 5 kN/m².").basis(&["ndp.en1991.e2.na-factors"])); }
    Ok(out)
}

fn e2_10(c: &Case) -> R {
    let lane = c.n("lane")? as u8;
    if !(1..=3).contains(&lane) { return Err("input lane must be 1, 2 or 3".into()); }
    Ok(vec![
        Outcome::info("flm1_tandem", "FLM1 tandem 0.7·Q_ik", "FLM1 Tandem 0,7·Q_ik", QuantityKind::Force, 0.7 * part_2::lm1_tandem_n(c.annex, lane)).steps("0.7 Q_ik per axle group.").steps_de("0,7 Q_ik je Achsgruppe.").basis(&["ndp.en1991.e2.na-factors"]),
        Outcome::info("flm1_udl", "FLM1 UDL 0.3·q_ik", "FLM1 Gleichlast 0,3·q_ik", QuantityKind::Pressure, 0.3 * part_2::lm1_udl_pa(c.annex, lane)).steps("0.3 q_ik.").basis(&["ndp.en1991.e2.na-factors"]),
        Outcome::info("flm3_total", "FLM3 vehicle weight 4 × 120 kN", "FLM3 Fahrzeuggewicht 4 × 120 kN", QuantityKind::Force, 480_000.0).steps("Axle spacing 1.2, 6.0, 1.2 m, track 2.0 m.").steps_de("Achsabstand 1,2, 6,0, 1,2 m, Spurweite 2,0 m.").basis(&["ndp.en1991.e2.na-factors"]),
    ])
}

fn e2_11(c: &Case) -> R {
    let (fx, fy) = match c.s("member")? { "deck" => (500.0, 250.0), "pier" => (1000.0, 500.0), other => return Err(format!("unknown member {other}; admissible: deck, pier")) };
    Ok(vec![
        Outcome::info("F_dx", "Collision force F_d,x", "Anprallkraft F_d,x", QuantityKind::Force, fx * 1000.0).steps(format!("{fx} kN.")).basis(&["ndp.en1991.e17.impact"]),
        Outcome::info("F_dy", "Collision force F_d,y", "Anprallkraft F_d,y", QuantityKind::Force, fy * 1000.0).steps(format!("{fy} kN.")).basis(&["ndp.en1991.e17.impact"]),
    ])
}

fn e2_12(c: &Case) -> R {
    let (l, w) = (c.pos("span")?, c.pos("width")?);
    let q = part_2::footbridge_load_kn_m2(l) * 1000.0;
    let total = q * l * w;
    Ok(vec![
        Outcome::info("q_fk", "Uniform load q_fk", "Gleichmäßige Last q_fk", QuantityKind::Pressure, q).steps(format!("q_fk = 2 + 120/(L+30) within 2.5 to 5.0 = {:.4} kN/m².", q / 1000.0)).steps_de(format!("q_fk = 2 + 120/(L+30) innerhalb 2,5 bis 5,0 = {:.4} kN/m².", q / 1000.0)),
        Outcome::info("Q_flk", "Horizontal force Q_flk = 10 % of the vertical load", "Horizontalkraft Q_flk = 10 % der Vertikallast", QuantityKind::Force, 0.1 * total).steps(format!("{:.2} kN.", 0.1 * total / 1000.0)).basis(&["ndp.en1991.e2.na-factors"]),
    ])
}

fn e2_13(c: &Case) -> R {
    let (f, dir) = (c.pos("frequency")?, c.s("direction")?);
    let read = semio_s_plugin_norm_compliance::ndp_common;
    let (ids, lo, hi) = match dir {
        "vertical" => (&["ndp.en1991.2.footbridge-band-vertical-max"][..], 0.0, read("ndp.en1991.2.footbridge-band-vertical-max")),
        "horizontal" => (&["ndp.en1991.2.footbridge-band-horizontal-max"][..], 0.0, read("ndp.en1991.2.footbridge-band-horizontal-max")),
        "jogging" => (&["ndp.en1991.2.footbridge-band-jogging-min", "ndp.en1991.2.footbridge-band-jogging-max"][..], read("ndp.en1991.2.footbridge-band-jogging-min"), read("ndp.en1991.2.footbridge-band-jogging-max")),
        other => return Err(format!("unknown direction {other}; admissible: vertical, horizontal, jogging")),
    };
    Ok(vec![Outcome::warn("resonance", "Natural frequency inside the band needing the comfort check (EN 1990 A2.4.3.2: vertical < 5 Hz, horizontal < 2.5 Hz)", "Eigenfrequenz im Band der Komfortnachweispflicht (EN 1990 A2.4.3.2)", QuantityKind::Dimensionless, if (lo..=hi).contains(&f) { 2.0 } else { 0.0 }, 1.0).steps(format!("f = {f} Hz against the band {lo} to {hi} Hz.")).steps_de(format!("f = {f} Hz gegenüber dem Band {lo} bis {hi} Hz.")).basis(ids)])
}

fn e2_14(c: &Case) -> R {
    let alpha = c.o("alpha", 1.0);
    if ![0.75, 0.83, 0.91, 1.0, 1.1, 1.21, 1.33, 1.46].iter().any(|a| (a - alpha).abs() < 1e-9) { return Err("classification factor alpha must be one of 0.75, 0.83, 0.91, 1.00, 1.10, 1.21, 1.33, 1.46".into()); }
    Ok(vec![
        Outcome::info("axle", "LM71 axle load α·250 kN", "LM71 Achslast α·250 kN", QuantityKind::Force, alpha * 250_000.0).steps(format!("4 axles at 1.6 m, α = {alpha}.")).steps_de(format!("4 Achsen im Abstand 1,6 m, α = {alpha}.")).basis(&["ndp.en1991.e2.rail"]),
        Outcome::info("udl", "LM71 UDL α·80 kN/m", "LM71 Gleichlast α·80 kN/m", QuantityKind::Force, alpha * 80_000.0).steps("Starting 0.8 m beyond the end axles.").steps_de("Beginn 0,8 m hinter den Endachsen.").basis(&["ndp.en1991.e2.rail"]),
    ])
}

fn e2_15(c: &Case) -> R {
    let kind = c.s("model")?;
    let (q, l, gap) = match kind { "sw0" => (133.0, 15.0, 5.3), "sw2" => (150.0, 25.0, 7.0), other => return Err(format!("unknown model {other}; admissible: sw0, sw2")) };
    let alpha = c.o("alpha", 1.0);
    Ok(vec![
        Outcome::info("udl", "SW UDL", "SW Gleichlast", QuantityKind::Force, alpha * q * 1000.0).steps(format!("{} kN/m over {l} m, gap {gap} m.", alpha * q)).steps_de(format!("{} kN/m über {l} m, Lücke {gap} m.", alpha * q)).basis(&["ndp.en1991.e2.rail"]),
        Outcome::info("total", "Load of one loaded length", "Last einer Belastungslänge", QuantityKind::Force, alpha * q * l * 1000.0).steps(format!("{:.0} kN.", alpha * q * l)).basis(&["ndp.en1991.e2.rail"]),
    ])
}

fn e2_16(c: &Case) -> R {
    let (p2, p3) = part_2::rail_dynamic_factors(c.pos("l_phi")?);
    Ok(vec![
        Outcome::info("phi_2", "Dynamic factor Φ₂ (well maintained track)", "Dynamikfaktor Φ₂ (sorgfältig unterhaltenes Gleis)", QuantityKind::Dimensionless, p2).steps(format!("Φ₂ = {p2:.5}.")).basis(&["ndp.en1991.e2.rail"]),
        Outcome::info("phi_3", "Dynamic factor Φ₃ (normal maintenance)", "Dynamikfaktor Φ₃ (normale Unterhaltung)", QuantityKind::Dimensionless, p3).steps(format!("Φ₃ = {p3:.5}.")).basis(&["ndp.en1991.e2.rail"]),
    ])
}

fn e2_17(c: &Case) -> R {
    let (v, r, lf, qvk) = (c.pos("v_kmh")?, c.pos("radius")?, c.pos("l_f")?, c.pos("q_vk")?);
    let f = part_2::rail_centrifugal_f(v, lf);
    let q = part_2::rail_centrifugal_kn(v, r, f, qvk / 1000.0) * 1000.0;
    let mut out = vec![
        Outcome::info("f", "Reduction factor f", "Abminderungsfaktor f", QuantityKind::Dimensionless, f).steps(format!("f = {f:.6}.")).basis(&["ndp.en1991.e2.rail"]),
        Outcome::info("Q_tk", "Centrifugal force Q_tk", "Fliehkraft Q_tk", QuantityKind::Force, q).steps(format!("Q_tk = v²/(127r)·f·Q_vk = {:.3} kN.", q / 1000.0)).basis(&["ndp.en1991.e2.rail"]),
    ];
    if let Some(l) = c.opt("loaded_length") {
        if l <= 0.0 { return Err("loaded_length must be positive".into()); }
        let (traction, braking) = ((33_000.0 * l).min(1_000_000.0), (20_000.0 * l).min(6_000_000.0));
        out.push(Outcome::info("traction", "Traction force Q_lak = 33 kN/m·L ≤ 1000 kN", "Anfahrkraft Q_lak = 33 kN/m·L ≤ 1000 kN", QuantityKind::Force, traction).steps(format!("{:.0} kN.", traction / 1000.0)).basis(&["ndp.en1991.e2.rail"]));
        out.push(Outcome::info("braking", "Braking force Q_lbk = 20 kN/m·L ≤ 6000 kN", "Bremskraft Q_lbk = 20 kN/m·L ≤ 6000 kN", QuantityKind::Force, braking).steps(format!("{:.0} kN.", braking / 1000.0)).basis(&["ndp.en1991.e2.rail"]));
    }
    Ok(out)
}

fn e3_01(c: &Case) -> R {
    let class = c.s("hoist_class")?;
    let phi2 = part_3::phi_2(class, c.o("v_h", 0.0)).ok_or_else(|| format!("unknown hoist class {class}; admissible: HC1, HC2, HC3, HC4"))?;
    let mut out = vec![
        Outcome::info("phi_1_up", "φ₁ unfavourable", "φ₁ ungünstig", QuantityKind::Dimensionless, 1.1).steps("φ₁ = 1 ± 0.1.").basis(&["ndp.en1991.e3.cranes"]),
        Outcome::info("phi_1_down", "φ₁ favourable", "φ₁ günstig", QuantityKind::Dimensionless, 0.9).steps("φ₁ = 1 ± 0.1.").basis(&["ndp.en1991.e3.cranes"]),
        Outcome::info("phi_2", "φ₂ = φ₂,min + β₂ v_h", "φ₂ = φ₂,min + β₂ v_h", QuantityKind::Dimensionless, phi2).steps(format!("{class}: φ₂ = {phi2:.5}.")).basis(&["ndp.en1991.e3.cranes"]),
        Outcome::info("phi_5", "φ₅", "φ₅", QuantityKind::Dimensionless, part_3::phi_5()).steps("φ₅ = 1.5.").basis(&["ndp.en1991.e3.cranes"]),
    ];
    if let Some(dm) = c.opt("dm_over_m") { let b3 = c.n("beta_3")?; let p3 = part_3::phi_3(dm, b3); out.push(Outcome::info("phi_3", "φ₃ = 1 − Δm/m (1 + β₃)", "φ₃ = 1 − Δm/m (1 + β₃)", QuantityKind::Dimensionless, p3).steps(format!("φ₃ = {p3:.4}.")).basis(&["ndp.en1991.e3.cranes"])); }
    Ok(out)
}

fn e3_02(c: &Case) -> R {
    let (qc, qh, nw) = (c.pos("q_c")?, c.n("q_h")?, c.pos("wheels")?);
    let class = c.s("hoist_class")?;
    let phi2 = part_3::phi_2(class, c.o("v_h", 0.0)).ok_or_else(|| format!("unknown hoist class {class}"))?;
    let q = (c.o("phi_1", 1.1) * qc + phi2 * qh) / nw;
    Ok(vec![Outcome::info("Q_r_max", "Maximum wheel load of load group 1", "Maximale Radlast Lastgruppe 1", QuantityKind::Force, q).steps(format!("Q_r = (φ₁Q_c + φ₂Q_H)/n = {:.3} kN.", q / 1000.0)).basis(&["ndp.en1991.e3.cranes"])])
}

fn e3_03(c: &Case) -> R {
    let mut out = Vec::new();
    if let (Some(sq), Some(nr)) = (c.opt("sum_q_r"), c.opt("runway_beams")) {
        let k = part_3::drive_friction_mu() * sq;
        out.push(Outcome::info("K", "Drive force K = μ ΣQ_r", "Antriebskraft K = μ ΣQ_r", QuantityKind::Force, k).steps(format!("K = {:.2} kN.", k / 1000.0)).basis(&["ndp.en1991.e3.cranes"]));
        out.push(Outcome::info("H_L", "Longitudinal force H_L = φ₅ K/n_r", "Längskraft H_L = φ₅ K/n_r", QuantityKind::Force, part_3::phi_5() * k / nr).steps(format!("H_L = {:.2} kN.", part_3::phi_5() * k / nr / 1000.0)).basis(&["ndp.en1991.e3.cranes"]));
    }
    if let (Some(xi), Some(m), Some(a)) = (c.opt("xi"), c.opt("m_drive"), c.opt("a")) { out.push(Outcome::info("H_T", "Transverse force H_T = ξ M/a", "Querkraft H_T = ξ M/a", QuantityKind::Force, xi * m / a).steps(format!("H_T = {:.2} kN.", xi * m / a / 1000.0)).basis(&["ndp.en1991.e3.cranes"])); }
    if let Some(alpha) = c.opt("skew_rad") { out.push(Outcome::info("f_skew", "Skewing factor f", "Schiefstellungsfaktor f", QuantityKind::Dimensionless, part_3::skew_factor(alpha)).steps(format!("f = 0.3(1 − e^(−250α)) = {:.6}.", part_3::skew_factor(alpha))).basis(&["ndp.en1991.e3.cranes"])); }
    if let (Some(p7), Some(v1), Some(mc), Some(sb)) = (c.opt("phi_7"), c.opt("v_1"), c.opt("m_c"), c.opt("s_b")) { let h = part_3::buffer_force_n(p7, v1, mc, sb); out.push(Outcome::info("H_B", "Buffer force H_B", "Pufferkraft H_B", QuantityKind::Force, h).steps(format!("H_B = φ₇ v₁ √(m_c S_B) = {:.1} kN.", h / 1000.0)).basis(&["ndp.en1991.e3.cranes"])); }
    if out.is_empty() { return Err("missing inputs: give sum_q_r and runway_beams, or xi, m_drive and a, or skew_rad, or phi_7, v_1, m_c and s_b".into()); }
    Ok(out)
}

fn e3_04(c: &Case) -> R {
    let qh = c.pos("q_h")?;
    Ok(vec![
        Outcome::info("static", "Static test load 1.25·Q_H", "Statische Prüflast 1,25·Q_H", QuantityKind::Force, 1.25 * qh).steps(format!("{:.2} kN.", 1.25 * qh / 1000.0)).basis(&["ndp.en1991.e3.cranes"]),
        Outcome::info("dynamic", "Dynamic test load 1.1·Q_H", "Dynamische Prüflast 1,1·Q_H", QuantityKind::Force, 1.1 * qh).steps(format!("{:.2} kN.", 1.1 * qh / 1000.0)).basis(&["ndp.en1991.e3.cranes"]),
    ])
}

fn e3_05(c: &Case) -> R {
    let (lambda, q) = (c.pos("lambda")?, c.pos("q_max")?);
    Ok(vec![Outcome::info("Q_e", "Damage-equivalent load Q_e = λ Q_max", "Schädigungsäquivalente Last Q_e = λ Q_max", QuantityKind::Force, lambda * q).steps(format!("Q_e = {:.2} kN.", lambda * q / 1000.0)).basis(&["ndp.en1991.e3.cranes"])])
}

fn e3_07(c: &Case) -> R {
    let (m, e, rpm) = (c.pos("mass")?, c.pos("eccentricity")?, c.pos("rpm")?);
    let omega = rpm * 2.0 * std::f64::consts::PI / 60.0;
    Ok(vec![Outcome::info("F_dyn", "Rotating machine force m e ω²", "Fliehkraft rotierender Maschinen m e ω²", QuantityKind::Force, m * e * omega * omega).steps(format!("ω = {omega:.3} rad/s: F = {:.3} kN.", m * e * omega * omega / 1000.0)).steps_de(format!("ω = {omega:.3} rad/s: F = {:.3} kN.", m * e * omega * omega / 1000.0)).basis(&["ndp.en1991.e3.cranes"])])
}

fn silo_inputs(c: &Case) -> Result<(f64, f64, f64, f64, f64), String> {
    let gamma = c.pos("gamma")?;
    let mu = match c.opt("mu") { Some(m) => m, None => (0.75 * c.pos("phi")?).to_radians().tan() };
    let k = match c.opt("k") { Some(k) => k, None => 1.1 * (1.0 - c.pos("phi")?.to_radians().sin()) };
    let a = match c.opt("a_over_u") { Some(a) => a, None => c.pos("dc")? / 4.0 };
    Ok((gamma, mu, k, a, c.n("z")?))
}

fn e4_01(c: &Case) -> R {
    let mut out = Vec::new();
    if let Some(cap) = c.opt("capacity_t") { out.push(Outcome::info("aac", "Action assessment class", "Auswirkungsklasse", QuantityKind::Dimensionless, f64::from(part_4::action_assessment_class(cap))).steps(format!("{cap} t → AAC {}.", part_4::action_assessment_class(cap))).basis(&["ndp.en1991.e4.silo", "ndp.en1991.4.aac-class1-below", "ndp.en1991.4.aac-class3-above"])); }
    if let (Some(hc), Some(dc)) = (c.opt("hc"), c.opt("dc")) {
        let r = hc / dc;
        let class = part_4::slenderness_class(r);
        out.push(Outcome::info("slenderness", "Slenderness h_c/d_c", "Schlankheit h_c/d_c", QuantityKind::Dimensionless, r).steps(format!("h_c/d_c = {r:.4}: {class}.")).basis(&["ndp.en1991.e4.silo"]));
        if let Some(g) = c.opt("gamma") { let mass = part_4::silo_mass_t(g, dc, hc); out.push(Outcome::info("mass", "Mass held", "Gelagerte Masse", QuantityKind::Mass, mass * 1000.0).steps(format!("m = π/4 d² h γ/g = {mass:.2} t."))); }
    }
    if out.is_empty() { return Err("missing input capacity_t or hc and dc".into()); }
    Ok(out)
}

fn e4_02(c: &Case) -> R {
    let phi = c.pos("phi")?;
    let mu = (0.75 * phi).to_radians().tan();
    Ok(vec![
        Outcome::info("K_m", "Lateral pressure ratio K = 1.1 (1 − sin φ)", "Horizontaldruckverhältnis K = 1,1 (1 − sin φ)", QuantityKind::Dimensionless, 1.1 * (1.0 - phi.to_radians().sin())).steps("K_m of the mean bulk solid values.").steps_de("K_m aus den mittleren Schüttgutwerten.").basis(&["ndp.en1991.e4.silo"]),
        Outcome::info("K_a", "Rankine active coefficient K_a", "Aktiver Erddruckbeiwert K_a (Rankine)", QuantityKind::Dimensionless, part_4::rankine_active(phi)).steps(format!("K_a = {:.6}.", part_4::rankine_active(phi))),
        Outcome::info("mu", "Wall friction μ = tan(0.75 φ)", "Wandreibung μ = tan(0,75 φ)", QuantityKind::Dimensionless, mu).steps(format!("μ = {mu:.6}.")).basis(&["ndp.en1991.e4.silo"]),
    ])
}

fn e4_03(c: &Case) -> R {
    let (gamma, mu, k, a, z) = silo_inputs(c)?;
    if z < 0.0 { return Err("input z must not be negative".into()); }
    let z_o = part_4::janssen_z_o_m(a, mu, k);
    let p_ho = part_4::janssen_p_ho_pa(gamma, a, mu);
    let y = 1.0 - (-z / z_o).exp();
    let p_hf = p_ho * y;
    let friction = mu * p_ho * (z - z_o * (1.0 - (-z / z_o).exp()));
    Ok(vec![
        Outcome::info("K", "Lateral pressure ratio K", "Horizontaldruckverhältnis K", QuantityKind::Dimensionless, k).steps(format!("K = {k:.5}.")),
        Outcome::info("z_o", "Characteristic depth z_o", "Charakteristische Tiefe z_o", QuantityKind::Length, z_o).steps(format!("z_o = A/(Kμ U) = {z_o:.5} m.")),
        Outcome::info("p_ho", "Asymptotic pressure p_ho", "Asymptotischer Druck p_ho", QuantityKind::Pressure, p_ho).steps(format!("p_ho = γ K z_o = {:.4} kN/m².", p_ho / 1000.0)),
        Outcome::info("Y_J", "Janssen function Y_J", "Janssen-Funktion Y_J", QuantityKind::Dimensionless, y).steps(format!("Y_J = {y:.6}.")),
        Outcome::info("p_hf", "Horizontal pressure p_hf", "Horizontaldruck p_hf", QuantityKind::Pressure, p_hf).steps(format!("p_hf = {:.4} kN/m².", p_hf / 1000.0)),
        Outcome::info("p_wf", "Wall friction p_wf = μ p_hf", "Wandreibung p_wf = μ p_hf", QuantityKind::Pressure, mu * p_hf).steps(format!("p_wf = {:.4} kN/m².", mu * p_hf / 1000.0)),
        Outcome::info("p_vf", "Vertical pressure p_vf = p_hf/K", "Vertikaldruck p_vf = p_hf/K", QuantityKind::Pressure, p_hf / k).steps(format!("p_vf = {:.4} kN/m².", p_hf / k / 1000.0)),
        Outcome::info("p_hf_max", "Maximum horizontal pressure (K_u = 1.2 K, μ_l = μ/1.15)", "Maximaler Horizontaldruck (K_u = 1,2 K, μ_l = μ/1,15)", QuantityKind::Pressure, part_4::horizontal_max_pa(gamma, a, mu, k, z)).steps("Upper characteristic values of the lateral pressure ratio and the wall friction.").steps_de("Obere charakteristische Werte des Horizontaldruckverhältnisses und der Wandreibung.").basis(&["ndp.en1991.e4.silo"]),
        Outcome::info("p_wf_max", "Maximum wall friction (K_l = K/1.2, μ_u = 1.15 μ)", "Maximale Wandreibung (K_l = K/1,2, μ_u = 1,15 μ)", QuantityKind::Pressure, part_4::wall_friction_max_pa(gamma, a, mu, k, z)).steps("Lower K with upper wall friction.").steps_de("Unteres K mit oberer Wandreibung.").basis(&["ndp.en1991.e4.silo"]),
        Outcome::info("N_wf", "Wall friction per perimeter length", "Wandreibungskraft je Umfangslänge", QuantityKind::Force, friction).steps(format!("N_wf = μ p_ho (z − z_o(1 − e^(−z/z_o))) = {:.4} kN/m.", friction / 1000.0)),
    ])
}

fn e4_04(c: &Case) -> R {
    let (gamma, mu, k, a, z) = silo_inputs(c)?;
    let p_hf = part_4::janssen_horizontal_pa(gamma, a, mu, k, z);
    let (ch, cw) = (c.o("c_h", part_4::discharge_c_h()), c.o("c_w", part_4::discharge_c_w()));
    let mut out = vec![
        Outcome::info("p_he", "Discharge horizontal pressure p_he = C_h p_hf", "Entleerungs-Horizontaldruck p_he = C_h p_hf", QuantityKind::Pressure, ch * p_hf).steps(format!("C_h = {ch}: {:.4} kN/m².", ch * p_hf / 1000.0)).basis(&["ndp.en1991.e4.silo"]),
        Outcome::info("p_we", "Discharge wall friction p_we = C_w p_wf", "Entleerungs-Wandreibung p_we = C_w p_wf", QuantityKind::Pressure, cw * mu * p_hf).steps(format!("C_w = {cw}: {:.4} kN/m².", cw * mu * p_hf / 1000.0)).basis(&["ndp.en1991.e4.silo"]),
        Outcome::info("p_pf", "Patch pressure p_pf", "Patch-Druck p_pf", QuantityKind::Pressure, part_4::patch_factor() * p_hf).steps(format!("p_pf = 0.2 p_hf = {:.4} kN/m².", part_4::patch_factor() * p_hf / 1000.0)).basis(&["ndp.en1991.e4.silo"]),
    ];
    if let Some(dc) = c.opt("dc") { out.push(Outcome::info("patch_width", "Patch width π d_c/16", "Patch-Breite π d_c/16", QuantityKind::Length, std::f64::consts::PI * dc / 16.0).steps(format!("s = {:.3} m.", std::f64::consts::PI * dc / 16.0)).basis(&["ndp.en1991.e4.silo"])); }
    Ok(out)
}

fn e4_05(c: &Case) -> R {
    let (gamma, mu, k, a, z) = silo_inputs(c)?;
    let p_hf = part_4::horizontal_max_pa(gamma, a, mu, k, z);
    let mut out = vec![Outcome::info("p_pf", "Patch pressure p_pf = 0.2 p_hf,max", "Patch-Druck p_pf = 0,2 p_hf,max", QuantityKind::Pressure, part_4::patch_factor() * p_hf).steps(format!("p_pf = {:.4} kN/m².", part_4::patch_factor() * p_hf / 1000.0)).basis(&["ndp.en1991.e4.silo"])];
    if let Some(dc) = c.opt("dc") { out.push(Outcome::info("patch_width", "Patch width π d_c/16", "Patch-Breite π d_c/16", QuantityKind::Length, std::f64::consts::PI * dc / 16.0).steps(format!("s = {:.3} m.", std::f64::consts::PI * dc / 16.0)).basis(&["ndp.en1991.e4.silo"])); }
    Ok(out)
}

fn e4_08(c: &Case) -> R {
    let (gamma, z) = (c.pos("gamma")?, c.n("z")?);
    Ok(vec![Outcome::info("p", "Hydrostatic pressure γz", "Hydrostatischer Druck γz", QuantityKind::Pressure, part_4::tank_hydrostatic_pa(gamma, z)).steps(format!("p = {:.3} kN/m².", part_4::tank_hydrostatic_pa(gamma, z) / 1000.0))])
}

pub(super) const ENTRIES: &[Entry] = &[
    Entry { id: "E2.01", standard: "EN 1991-2", clause: "4.2.3", en: "Notional lanes", de: "Ideelle Fahrstreifen", inputs: &["width"], eval: e2_01 },
    Entry { id: "E2.02", standard: "EN 1991-2", clause: "4.3.2", en: "Load model 1 (LM1)", de: "Lastmodell 1 (LM1)", inputs: &["width"], eval: e2_02 },
    Entry { id: "E2.03", standard: "EN 1991-2", clause: "4.3.3", en: "Load model 2 (LM2)", de: "Lastmodell 2 (LM2)", inputs: &["axle", "a", "b"], eval: e2_03 },
    Entry { id: "E2.04", standard: "EN 1991-2", clause: "4.3.4", en: "Load model 3 (LM3) special vehicles", de: "Lastmodell 3 (LM3) Sonderfahrzeuge", inputs: &["axles", "axle_load"], eval: e2_04 },
    Entry { id: "E2.05", standard: "EN 1991-2", clause: "4.3.5", en: "Load model 4 (LM4) crowd", de: "Lastmodell 4 (LM4) Menschenmenge", inputs: &[], eval: e2_05 },
    Entry { id: "E2.06", standard: "EN 1991-2", clause: "4.3.6", en: "Dispersal of concentrated loads", de: "Lastausbreitung von Einzellasten", inputs: &["a", "b", "load", "pavement", "slab_thickness"], eval: e2_06 },
    Entry { id: "E2.07", standard: "EN 1991-2", clause: "4.4.1", en: "Braking and acceleration forces", de: "Brems- und Anfahrkräfte", inputs: &["loaded_length"], eval: e2_07 },
    Entry { id: "E2.08", standard: "EN 1991-2", clause: "4.4.2", en: "Centrifugal forces", de: "Fliehkräfte", inputs: &["q_v", "radius"], eval: e2_08 },
    Entry { id: "E2.09", standard: "EN 1991-2", clause: "4.5", en: "Groups of traffic loads", de: "Lastgruppen", inputs: &["group"], eval: e2_09 },
    Entry { id: "E2.10", standard: "EN 1991-2", clause: "4.6", en: "Fatigue load models", de: "Ermüdungslastmodelle", inputs: &["lane"], eval: e2_10 },
    Entry { id: "E2.11", standard: "EN 1991-2", clause: "4.7", en: "Collision on bridge members", de: "Anprall an Brückenbauteilen", inputs: &["member"], eval: e2_11 },
    Entry { id: "E2.12", standard: "EN 1991-2", clause: "5", en: "Footbridge loads", de: "Lasten auf Fußgängerbrücken", inputs: &["span", "width"], eval: e2_12 },
    Entry { id: "E2.13", standard: "EN 1991-2", clause: "5.7", en: "Pedestrian dynamic excitation", de: "Dynamische Fußgängeranregung", inputs: &["frequency", "direction"], eval: e2_13 },
    Entry { id: "E2.14", standard: "EN 1991-2", clause: "6.3.2", en: "Rail load model 71", de: "Eisenbahn-Lastmodell 71", inputs: &["alpha"], eval: e2_14 },
    Entry { id: "E2.15", standard: "EN 1991-2", clause: "6.3.3", en: "Rail load models SW/0 and SW/2", de: "Eisenbahn-Lastmodelle SW/0 und SW/2", inputs: &["model", "alpha"], eval: e2_15 },
    Entry { id: "E2.16", standard: "EN 1991-2", clause: "6.4", en: "Rail dynamic factors", de: "Dynamische Beiwerte Eisenbahn", inputs: &["l_phi"], eval: e2_16 },
    Entry { id: "E2.17", standard: "EN 1991-2", clause: "6.5", en: "Rail horizontal forces: centrifugal, traction and braking", de: "Horizontale Eisenbahnlasten: Fliehkraft, Anfahren und Bremsen", inputs: &["v_kmh", "radius", "l_f", "q_vk", "loaded_length"], eval: e2_17 },
    Entry { id: "E3.01", standard: "EN 1991-3", clause: "2.5", en: "Dynamic factors of cranes", de: "Dynamische Beiwerte von Kranen", inputs: &["hoist_class", "v_h", "dm_over_m", "beta_3"], eval: e3_01 },
    Entry { id: "E3.02", standard: "EN 1991-3", clause: "2.7", en: "Vertical wheel load of load group 1", de: "Vertikale Radlast Lastgruppe 1", inputs: &["q_c", "q_h", "wheels", "phi_1", "hoist_class", "v_h"], eval: e3_02 },
    Entry { id: "E3.03", standard: "EN 1991-3", clause: "2.7", en: "Horizontal crane forces", de: "Horizontale Kranlasten", inputs: &["sum_q_r", "runway_beams", "xi", "m_drive", "a", "skew_rad", "phi_7", "v_1", "m_c", "s_b"], eval: e3_03 },
    Entry { id: "E3.04", standard: "EN 1991-3", clause: "2.10", en: "Test loads", de: "Prüflasten", inputs: &["q_h"], eval: e3_04 },
    Entry { id: "E3.05", standard: "EN 1991-3", clause: "2.12", en: "Fatigue damage-equivalent load", de: "Schädigungsäquivalente Last", inputs: &["lambda", "q_max"], eval: e3_05 },
    Entry { id: "E3.07", standard: "EN 1991-3", clause: "5", en: "Dynamic loads of machinery", de: "Dynamische Lasten von Maschinen", inputs: &["mass", "eccentricity", "rpm"], eval: e3_07 },
    Entry { id: "E4.01", standard: "EN 1991-4", clause: "2.2", en: "Action assessment class and slenderness", de: "Auswirkungsklasse und Schlankheit", inputs: &["capacity_t", "hc", "dc", "gamma"], eval: e4_01 },
    Entry { id: "E4.02", standard: "EN 1991-4", clause: "E", en: "Bulk solid parameters", de: "Schüttgutkennwerte", inputs: &["phi"], eval: e4_02 },
    Entry { id: "E4.03", standard: "EN 1991-4", clause: "5.2", en: "Janssen filling loads", de: "Befüllungslasten nach Janssen", inputs: &["gamma", "phi", "mu", "k", "dc", "a_over_u", "z"], eval: e4_03 },
    Entry { id: "E4.04", standard: "EN 1991-4", clause: "5.3", en: "Discharge and patch loads", de: "Entleerungs- und Patchlasten", inputs: &["gamma", "phi", "mu", "k", "dc", "a_over_u", "z", "c_h", "c_w"], eval: e4_04 },
    Entry { id: "E4.05", standard: "EN 1991-4", clause: "5.2.4", en: "Patch load", de: "Patchlast", inputs: &["gamma", "phi", "mu", "k", "dc", "a_over_u", "z"], eval: e4_05 },
    Entry { id: "E4.08", standard: "EN 1991-4", clause: "7", en: "Hydrostatic tank pressure", de: "Hydrostatischer Tankdruck", inputs: &["gamma", "z"], eval: e4_08 },
];
