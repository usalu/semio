//! 🌡️ EN 1991-1-5, 1-6 and 1-7 catalogue entries E15.01 … E15.08, E16.01 … E16.06 and E17.01 … E17.09: thermal actions, actions during execution and accidental actions.

use super::*;

fn kelvin(c: f64) -> f64 { c + 273.15 }

fn e15_01(c: &Case) -> R {
    let ty = c.n("deck_type")? as u8;
    let (t_min, t_max, t_0) = (c.n("t_min_c")?, c.n("t_max_c")?, c.o("t_0_c", 10.0));
    let (dn, dx) = part_1_5::bridge_shift(ty).ok_or_else(|| format!("unknown deck type {ty}; admissible: 1 steel, 2 composite, 3 concrete"))?;
    let (con, exp) = part_1_5::bridge_delta_t_n(ty, t_min, t_max, t_0).ok_or("unknown deck type")?;
    let alpha = if ty == 3 { 10e-6 } else { 12e-6 };
    let l = c.o("length", 100.0);
    Ok(vec![
        Outcome::info("T_e_min", "Minimum uniform temperature T_e,min", "Minimale Brückentemperatur T_e,min", QuantityKind::Temperature, kelvin(t_min + dn)).steps(format!("T_e,min = T_min + ΔT = {:.1} °C.", t_min + dn)).basis(&["ndp.en1991.e15.shifts"]),
        Outcome::info("T_e_max", "Maximum uniform temperature T_e,max", "Maximale Brückentemperatur T_e,max", QuantityKind::Temperature, kelvin(t_max + dx)).steps(format!("T_e,max = T_max + ΔT = {:.1} °C.", t_max + dx)).basis(&["ndp.en1991.e15.shifts"]),
        Outcome::info("dT_con", "Contraction range ΔT_N,con", "Schrumpfbereich ΔT_N,con", QuantityKind::Temperature, con).steps(format!("ΔT_N,con = T_0 − T_e,min = {con:.1} K.")).basis(&["ndp.en1991.e15.shifts"]),
        Outcome::info("dT_exp", "Expansion range ΔT_N,exp", "Dehnbereich ΔT_N,exp", QuantityKind::Temperature, exp).steps(format!("ΔT_N,exp = T_e,max − T_0 = {exp:.1} K.")).basis(&["ndp.en1991.e15.shifts"]),
        Outcome::info("dL_con", "Contraction of the length", "Verkürzung der Länge", QuantityKind::Length, part_1_5::expansion_m(alpha, con, l)).steps(format!("ΔL = α ΔT L over {l} m.")).steps_de(format!("ΔL = α ΔT L über {l} m.")),
        Outcome::info("dL_exp", "Expansion of the length", "Verlängerung der Länge", QuantityKind::Length, part_1_5::expansion_m(alpha, exp, l)).steps(format!("ΔL = α ΔT L over {l} m.")).steps_de(format!("ΔL = α ΔT L über {l} m.")),
    ])
}

fn e15_02(c: &Case) -> R {
    let (heat, cool) = match c.s("deck")? { "steel" => (18.0, 13.0), "composite" => (15.0, 18.0), "slab" => (15.0, 8.0), "box" => (10.0, 5.0), "beam" => (15.0, 8.0), other => return Err(format!("unknown deck {other}; admissible: steel, composite, slab, box, beam")) };
    let (kh, kc) = match (c.opt("k_sur_heat"), c.opt("k_sur_cool")) {
        (Some(h), Some(k)) => (h, k),
        _ => match c.s("surfacing")? { "unsurfaced" => (0.7, 0.9), "waterproofed" => (1.6, 1.1), "50mm" => (1.0, 1.0), "100mm" | "150mm" => (0.7, 1.2), "ballast" => (0.6, 1.4), other => return Err(format!("unknown surfacing {other}; admissible: unsurfaced, waterproofed, 50mm, 100mm, 150mm, ballast")) },
    };
    Ok(vec![
        Outcome::info("dT_M_heat", "Heating gradient ΔT_M,heat", "Erwärmungsgradient ΔT_M,heat", QuantityKind::Temperature, heat * kh).steps(format!("ΔT_M,heat = {heat}·{kh} = {:.2} K.", heat * kh)).basis(&["ndp.en1991.e15.shifts"]),
        Outcome::info("dT_M_cool", "Cooling gradient ΔT_M,cool", "Abkühlgradient ΔT_M,cool", QuantityKind::Temperature, cool * kc).steps(format!("ΔT_M,cool = {cool}·{kc} = {:.2} K.", cool * kc)).basis(&["ndp.en1991.e15.shifts"]),
    ])
}

fn e15_03(c: &Case) -> R {
    let (e, a, dt) = (c.pos("e")?, c.pos("alpha")?, c.n("dt")?);
    let s = part_1_5::restraint_stress_pa(e, a, dt);
    Ok(vec![Outcome::info("sigma", "Restraint stress σ = E α ΔT", "Zwangsspannung σ = E α ΔT", QuantityKind::Stress, s).steps(format!("σ = {:.3} MPa for full restraint.", s / 1e6)).steps_de(format!("σ = {:.3} MPa bei voller Behinderung.", s / 1e6))])
}

fn e15_04(c: &Case) -> R {
    let (m, n) = (c.n("dt_m")?, c.n("dt_n")?);
    Ok(vec![
        Outcome::info("m_leading", "ΔT_M + ω_N ΔT_N", "ΔT_M + ω_N ΔT_N", QuantityKind::Temperature, m + 0.35 * n).steps(format!("{m} + 0.35·{n} = {:.3} K.", m + 0.35 * n)),
        Outcome::info("n_leading", "ΔT_N + ω_M ΔT_M", "ΔT_N + ω_M ΔT_M", QuantityKind::Temperature, n + 0.75 * m).steps(format!("{n} + 0.75·{m} = {:.3} K.", n + 0.75 * m)),
    ])
}

fn e15_05(c: &Case) -> R {
    let (e, i, a, dt, h) = (c.pos("e")?, c.pos("i")?, c.pos("alpha")?, c.n("dt_m")?, c.pos("h")?);
    let kappa = a * dt / h;
    let mut out = vec![
        Outcome::info("kappa", "Curvature κ = α ΔT_M/h", "Krümmung κ = α ΔT_M/h", QuantityKind::Dimensionless, kappa).steps(format!("κ = {kappa:.4e} 1/m.")),
        Outcome::info("M", "Restrained moment M = E I κ", "Zwangsmoment M = E I κ", QuantityKind::Moment, e * i * kappa).steps(format!("M = {:.3} kNm.", e * i * kappa / 1000.0)),
    ];
    if let Some(l) = c.opt("length") { out.push(Outcome::info("theta", "End rotation θ = κ L/2", "Endverdrehung θ = κ L/2", QuantityKind::Dimensionless, kappa * l / 2.0).steps(format!("θ = {:.4e} rad.", kappa * l / 2.0))); }
    Ok(out)
}

fn e15_06(c: &Case) -> R {
    let (a, dt, l) = (c.pos("alpha")?, c.n("dt")?, c.pos("length")?);
    let margin = c.o("margin_k", 0.0);
    let d = part_1_5::expansion_m(a, dt + margin, l);
    Ok(vec![Outcome::info("movement", "Joint movement", "Fugenbewegung", QuantityKind::Length, d).steps(format!("ΔL = α (ΔT + margin) L = {:.2} mm.", d * 1000.0))])
}

fn e15_07(c: &Case) -> R {
    let (t_min, t_max, alt) = (c.n("t_min_c")?, c.n("t_max_c")?, c.n("altitude")?);
    let corr = 0.5 * alt / 100.0;
    Ok(vec![
        Outcome::info("t_min", "Corrected T_min", "Korrigierte T_min", QuantityKind::Temperature, kelvin(t_min - corr)).steps(format!("T_min − 0.5 K/100 m · {alt} m = {:.2} °C.", t_min - corr)).basis(&["ndp.en1991.e15.altitude"]),
        Outcome::info("t_max", "Corrected T_max", "Korrigierte T_max", QuantityKind::Temperature, kelvin(t_max - corr)).steps(format!("T_max − 0.5 K/100 m · {alt} m = {:.2} °C.", t_max - corr)).basis(&["ndp.en1991.e15.altitude"]),
    ])
}

fn e15_08(c: &Case) -> R {
    let (t_min, t_max, t_0) = (c.n("t_min_c")?, c.n("t_max_c")?, c.o("t_0_c", 10.0));
    let (con, exp) = part_1_5::building_delta_t(t_min, t_max, t_0);
    Ok(vec![
        Outcome::info("t_0", "Initial temperature T_0", "Anfangstemperatur T_0", QuantityKind::Temperature, kelvin(t_0)).steps(format!("T_0 = {t_0} °C = {:.2} K.", kelvin(t_0))),
        Outcome::info("dT_con", "Cooling range T_0 − T_min", "Abkühlbereich T_0 − T_min", QuantityKind::Temperature, con).steps(format!("{con:.1} K.")),
        Outcome::info("dT_exp", "Heating range T_max − T_0", "Erwärmungsbereich T_max − T_0", QuantityKind::Temperature, exp).steps(format!("{exp:.1} K.")),
    ])
}

fn e16_01(c: &Case) -> R {
    let days = c.pos("duration_days")?;
    let years = if days <= 3.0 { 2.0 } else if days <= 90.0 { 5.0 } else if days <= 365.0 { 10.0 } else { 50.0 };
    let p = 1.0 / years;
    let cp = part_1_6::c_prob(p);
    let v = c.o("v_b", 0.0);
    let mut out = vec![
        Outcome::info("n_years", "Return period", "Wiederkehrperiode", QuantityKind::Time, years * 365.25 * 86400.0).steps(format!("Duration {days} d → {years} a.")).basis(&["ndp.en1991.e16.returns"]),
        Outcome::info("c_prob", "Wind probability factor c_prob", "Wind-Wahrscheinlichkeitsbeiwert c_prob", QuantityKind::Dimensionless, cp).steps(format!("c_prob = {cp:.6}.")).basis(&["ndp.en1991.e16.returns"]),
        Outcome::info("snow_factor", "Snow factor s_n/s_k (V = 0.4)", "Schneefaktor s_n/s_k (V = 0,4)", QuantityKind::Dimensionless, part_1_3::return_period_factor(0.4, p)).steps("Annex D with V = 0.4.").steps_de("Anhang D mit V = 0,4.").basis(&["ndp.en1991.e16.returns"]),
    ];
    if v > 0.0 { out.push(Outcome::info("v_b", "Scaled basic velocity", "Skalierte Basisgeschwindigkeit", QuantityKind::Dimensionless, v * cp).steps(format!("v_b = {:.4} m/s.", v * cp))); out.push(Outcome::info("q_b", "Scaled basic velocity pressure", "Skalierter Basisgeschwindigkeitsdruck", QuantityKind::Pressure, part_1_4::basic_velocity_pressure_pa(c.o("rho", 1.25), v * cp)).steps("q_b = ½ρ(c_prob v_b)².")); }
    Ok(out)
}

fn e16_02(c: &Case) -> R {
    let area = c.pos("area")?;
    let qca = c.o("q_ca", 1000.0);
    let mut out = vec![Outcome::info("Q_ca", "Personnel and hand tools Q_ca", "Personen und Handwerkzeug Q_ca", QuantityKind::Force, qca * area).steps(format!("Q_ca = {:.2} kN/m² · {area} m² = {:.2} kN.", qca / 1000.0, qca * area / 1000.0)).basis(&["ndp.en1991.e16.returns"])];
    if let (Some(g), Some(h)) = (c.opt("gamma"), c.opt("h")) {
        let p = part_1_6::fresh_concrete_pressure_pa(g, h);
        out.push(Outcome::info("p_fresh", "Fresh concrete pressure γh", "Frischbetondruck γh", QuantityKind::Pressure, p).steps(format!("p = {:.2} kN/m².", p / 1000.0)));
        out.push(Outcome::info("p_total", "Fresh concrete plus personnel", "Frischbeton plus Personal", QuantityKind::Pressure, p + c.o("q_ca_concreting", 750.0)).steps(format!("p + {:.2} kN/m².", c.o("q_ca_concreting", 750.0) / 1000.0)));
    }
    Ok(out)
}

fn e16_03(c: &Case) -> R {
    let (cp, vb) = (part_1_6::c_prob(c.n("p")?), c.pos("v_b")?);
    let q = part_1_4::basic_velocity_pressure_pa(c.o("rho", 1.25), vb * cp);
    Ok(vec![
        Outcome::info("c_prob", "Probability factor", "Wahrscheinlichkeitsbeiwert", QuantityKind::Dimensionless, cp).steps(format!("c_prob = {cp:.6}.")),
        Outcome::info("v_b", "Scaled basic velocity", "Skalierte Basisgeschwindigkeit", QuantityKind::Dimensionless, vb * cp).steps(format!("v_b = {:.4} m/s.", vb * cp)),
        Outcome::info("q_b", "Scaled basic velocity pressure", "Skalierter Basisgeschwindigkeitsdruck", QuantityKind::Pressure, q).steps(format!("q_b = {:.4} kN/m².", q / 1000.0)),
    ])
}

fn e16_06(c: &Case) -> R {
    let q = c.v("q_ci")?;
    let sum: f64 = q.iter().sum();
    Ok(vec![Outcome::info("Q_c_sum", "Combined construction load Q_c", "Kombinierte Bauzustandslast Q_c", QuantityKind::Force, sum).steps(format!("Q_c = Σ Q_ci (ψ₀ = ψ₂ = 1.0) = {:.3} kN.", sum / 1000.0)).basis(&["ndp.en1991.e16.returns"])])
}

fn e17_01(c: &Case) -> R {
    let (fx, fy) = match c.s("road")? { "motorway" => (1000.0, 500.0), "urban" => (750.0, 375.0), "courtyard_cars" => (50.0, 25.0), "courtyard_lorries" => (150.0, 75.0), other => return Err(format!("unknown road {other}; admissible: motorway, urban, courtyard_cars, courtyard_lorries")) };
    Ok(vec![
        Outcome::info("F_dx", "Impact force F_d,x in driving direction", "Anprallkraft F_d,x in Fahrtrichtung", QuantityKind::Force, fx * 1000.0).steps(format!("F_d,x = {fx} kN.")).basis(&["ndp.en1991.e17.impact"]),
        Outcome::info("F_dy", "Impact force F_d,y across", "Anprallkraft F_d,y quer", QuantityKind::Force, fy * 1000.0).steps(format!("F_d,y = {fy} kN.")).basis(&["ndp.en1991.e17.impact"]),
    ])
}

fn e17_02(c: &Case) -> R {
    let w = c.pos("weight")?;
    Ok(vec![Outcome::info("F_d", "Forklift impact F_d = 5 W", "Gabelstaplerstoß F_d = 5 W", QuantityKind::Force, 5.0 * w).steps(format!("F_d = 5·{:.2} kN at 0.75 m height.", w / 1000.0)).steps_de(format!("F_d = 5·{:.2} kN in 0,75 m Höhe.", w / 1000.0)).basis(&["ndp.en1991.e17.impact"])])
}

fn e17_05(c: &Case) -> R {
    let m = c.pos("mass")?;
    let f = 3000.0 * m.sqrt();
    Ok(vec![Outcome::info("F_d", "Helicopter hard landing F_d = C√m", "Hubschrauber-Hartlandung F_d = C√m", QuantityKind::Force, f).steps(format!("F_d = 3·√{m} = {:.1} kN.", f / 1000.0)).basis(&["ndp.en1991.e17.impact"])])
}

fn e17_06(c: &Case) -> R {
    let (g, q, psi, s, l) = (c.pos("g_k")? / 1000.0, c.n("q_k")? / 1000.0, c.n("psi")?, c.pos("s")?, c.pos("l")?);
    let (tp, tp_cap) = part_1_7::tie_force_kn(0.4, g, q, psi, s, l);
    let (ti, ti_cap) = part_1_7::tie_force_kn(0.8, g, q, psi, s, l);
    let mut out = vec![
        Outcome::info("T_p", "Peripheral tie force T_p", "Randzugkraft T_p", QuantityKind::Force, tp * 1000.0).steps(format!("T_p = 0.4(g + ψq)sL = {tp:.3} kN.")).basis(&["ndp.en1991.e17.robust"]),
        Outcome::info("T_p_cap", "Peripheral tie force limited to 75 kN", "Randzugkraft begrenzt auf 75 kN", QuantityKind::Force, tp_cap * 1000.0).steps(format!("min(T_p, 75) = {tp_cap:.3} kN.")).basis(&["ndp.en1991.e17.robust"]),
        Outcome::info("T_i", "Internal tie force T_i", "Innenzugkraft T_i", QuantityKind::Force, ti * 1000.0).steps(format!("T_i = 0.8(g + ψq)sL = {ti:.3} kN.")).basis(&["ndp.en1991.e17.robust"]),
        Outcome::info("T_i_cap", "Internal tie force limited to 75 kN", "Innenzugkraft begrenzt auf 75 kN", QuantityKind::Force, ti_cap * 1000.0).steps(format!("min(T_i, 75) = {ti_cap:.3} kN.")).basis(&["ndp.en1991.e17.robust"]),
    ];
    if let Some(a) = c.opt("floor_area") { out.push(Outcome::info("removal_area", "Admissible damage area per storey", "Zulässige Schadensfläche je Geschoss", QuantityKind::Area, (0.15 * a).min(100.0)).steps(format!("min(15 % of {a} m², 100 m²) = {:.2} m².", (0.15 * a).min(100.0))).basis(&["ndp.en1991.e17.robust"])); }
    if let Some(a) = c.opt("key_area") { out.push(Outcome::info("key_element", "Key element load A_d = 34 kN/m²", "Schlüsselelement A_d = 34 kN/m²", QuantityKind::Force, 34_000.0 * a).steps(format!("A_d·{a} m² = {:.1} kN.", 34.0 * a)).basis(&["ndp.en1991.e17.robust"])); }
    if let Some(t) = c.opt("provided_tie") { out.push(Outcome::limit("tie_capacity", "Required tie force against the provided tie", "Erforderliche Zugkraft gegen vorhandene Zugkraft", QuantityKind::Force, ti_cap * 1000.0, t).steps(format!("{ti_cap:.2} kN against {:.2} kN provided.", t / 1000.0)).steps_de(format!("{ti_cap:.2} kN gegenüber vorhandenen {:.2} kN.", t / 1000.0)).basis(&["ndp.en1991.e17.robust"])); }
    Ok(out)
}

fn e17_07(c: &Case) -> R {
    let (ph, pd, cc) = (c.v("p_h")?, c.v("p_d_h")?, c.v("consequence")?);
    if ph.len() != pd.len() || ph.len() != cc.len() || ph.is_empty() { return Err("p_h, p_d_h and consequence must be non-empty series of equal length".into()); }
    let risk: f64 = ph.iter().zip(pd.iter()).zip(cc.iter()).map(|((a, b), k)| a * b * k).sum();
    let mut out = vec![Outcome::info("risk", "Risk R = Σ P(H) P(D|H) C(D)", "Risiko R = Σ P(H) P(D|H) C(D)", QuantityKind::Dimensionless, risk).steps(format!("R = {risk:.6e}."))];
    if let Some(acc) = c.opt("r_acc") { out.push(Outcome::limit("acceptance", "R against acceptable risk", "R gegen akzeptables Risiko", QuantityKind::Dimensionless, risk, acc).steps(format!("R = {risk:.3e} against {acc:.3e}.")).steps_de(format!("R = {risk:.3e} gegenüber {acc:.3e}."))); }
    Ok(out)
}

fn e17_08(c: &Case) -> R {
    let (m, v, k) = (c.pos("mass")?, c.pos("speed")?, c.o("k", part_1_7::vehicle_stiffness_n_m()));
    let f_el = part_1_7::vehicle_impact_force_n(m, v);
    let mut out = vec![
        Outcome::info("energy", "Kinetic energy ½mv²", "Kinetische Energie ½mv²", QuantityKind::Energy, part_1_7::impact_energy_j(m, v)).steps(format!("E = {:.3} kJ.", part_1_7::impact_energy_j(m, v) / 1000.0)),
        Outcome::info("F_el", "Elastic impact force F = v√(km)", "Elastische Anprallkraft F = v√(km)", QuantityKind::Force, v * (k * m).sqrt()).steps(format!("F = {:.2} kN.", v * (k * m).sqrt() / 1000.0)).basis(&["ndp.en1991.e17.impact"]),
    ];
    if let (Some(f_pl), Some(y_el)) = (c.opt("f_pl"), c.opt("y_el")) {
        let y_max = (part_1_7::impact_energy_j(m, v) - 0.5 * f_el * y_el) / f_pl + y_el;
        out.push(match c.opt("y_allowed") { Some(a) => Outcome::limit("y_max", "Plastic deformation y_max", "Plastische Verformung y_max", QuantityKind::Length, y_max, a), None => Outcome::info("y_max", "Plastic deformation y_max", "Plastische Verformung y_max", QuantityKind::Length, y_max) }.steps(format!("y_max = (E − ½F_el y_el)/F_pl + y_el = {y_max:.4} m.")));
    }
    Ok(out)
}

fn e17_09(c: &Case) -> R {
    let (pv, ratio) = (c.n("p_v")?, c.pos("av_over_v")?);
    Ok(vec![
        Outcome::info("p_d1", "Design pressure D.1: 3 + p_v", "Bemessungsdruck D.1: 3 + p_v", QuantityKind::Pressure, (3.0 + pv) * 1000.0).steps(format!("p_d = {:.3} kN/m².", 3.0 + pv)).basis(&["ndp.en1991.e17.blast"]),
        Outcome::info("p_d2", "Design pressure D.2: 3 + p_v/2 + 0.04/(A_v/V)²", "Bemessungsdruck D.2: 3 + p_v/2 + 0,04/(A_v/V)²", QuantityKind::Pressure, part_1_7::gas_explosion_pd_kn_m2(pv, ratio) * 1000.0).steps(format!("p_d = {:.3} kN/m².", part_1_7::gas_explosion_pd_kn_m2(pv, ratio))).basis(&["ndp.en1991.e17.blast"]),
    ])
}

fn e17_10(c: &Case) -> R {
    let (m, r) = (c.pos("charge_kg")?, c.pos("standoff")?);
    let p = part_1_7::explosion_pressure_pa(m, r);
    Ok(vec![Outcome::info("p_so", "Incident blast overpressure", "Einfallender Explosionsüberdruck", QuantityKind::Pressure, p).steps(format!("Z = R/W^⅓ = {:.3} m/kg^⅓ → p = {:.2} kN/m².", r / m.cbrt(), p / 1000.0)).basis(&["ndp.en1991.e17.blast"])])
}

pub(super) const ENTRIES: &[Entry] = &[
    Entry { id: "E15.01", standard: "EN 1991-1-5", clause: "6.1.3", en: "Uniform bridge temperature component", de: "Gleichmäßiger Temperaturanteil Brücken", inputs: &["deck_type", "t_min_c", "t_max_c", "t_0_c", "length"], eval: e15_01 },
    Entry { id: "E15.02", standard: "EN 1991-1-5", clause: "6.1.4", en: "Linear temperature difference of bridges", de: "Linearer Temperaturunterschied Brücken", inputs: &["deck", "surfacing", "k_sur_heat", "k_sur_cool"], eval: e15_02 },
    Entry { id: "E15.03", standard: "EN 1991-1-5", clause: "5", en: "Full restraint stress", de: "Spannung bei voller Behinderung", inputs: &["e", "alpha", "dt"], eval: e15_03 },
    Entry { id: "E15.04", standard: "EN 1991-1-5", clause: "6.1.5", en: "Combination of uniform and gradient components", de: "Kombination von Temperaturanteilen", inputs: &["dt_m", "dt_n"], eval: e15_04 },
    Entry { id: "E15.05", standard: "EN 1991-1-5", clause: "6.1.4", en: "Restrained bowing moment", de: "Zwangsmoment aus Verkrümmung", inputs: &["e", "i", "alpha", "dt_m", "h", "length"], eval: e15_05 },
    Entry { id: "E15.06", standard: "EN 1991-1-5", clause: "6.1.3", en: "Joint movement", de: "Fugenbewegung", inputs: &["alpha", "dt", "length", "margin_k"], eval: e15_06 },
    Entry { id: "E15.07", standard: "EN 1991-1-5", clause: "A", en: "Altitude correction of shade temperatures", de: "Höhenkorrektur der Schattentemperaturen", inputs: &["t_min_c", "t_max_c", "altitude"], eval: e15_07 },
    Entry { id: "E15.08", standard: "EN 1991-1-5", clause: "5", en: "Building temperature ranges", de: "Temperaturbereiche Gebäude", inputs: &["t_min_c", "t_max_c", "t_0_c"], eval: e15_08 },
    Entry { id: "E16.01", standard: "EN 1991-1-6", clause: "3", en: "Return period of transient situations", de: "Wiederkehrperiode vorübergehender Situationen", inputs: &["duration_days", "v_b", "rho"], eval: e16_01 },
    Entry { id: "E16.02", standard: "EN 1991-1-6", clause: "4.11", en: "Construction loads Q_ca and fresh concrete", de: "Bauzustandslasten Q_ca und Frischbeton", inputs: &["area", "q_ca", "gamma", "h", "q_ca_concreting"], eval: e16_02 },
    Entry { id: "E16.03", standard: "EN 1991-1-6", clause: "3.3", en: "Wind during execution", de: "Wind im Bauzustand", inputs: &["p", "v_b", "rho"], eval: e16_03 },
    Entry { id: "E16.06", standard: "EN 1991-1-6", clause: "A.2", en: "Combination of construction loads", de: "Kombination von Bauzustandslasten", inputs: &["q_ci"], eval: e16_06 },
    Entry { id: "E17.01", standard: "EN 1991-1-7", clause: "4.3", en: "Impact of road vehicles", de: "Anprall von Straßenfahrzeugen", inputs: &["road"], eval: e17_01 },
    Entry { id: "E17.02", standard: "EN 1991-1-7", clause: "4.5", en: "Impact of forklift trucks", de: "Anprall von Gabelstaplern", inputs: &["weight"], eval: e17_02 },
    Entry { id: "E17.05", standard: "EN 1991-1-7", clause: "4.7", en: "Helicopter hard landing", de: "Hubschrauber-Hartlandung", inputs: &["mass"], eval: e17_05 },
    Entry { id: "E17.06", standard: "EN 1991-1-7", clause: "A", en: "Robustness: ties, removal and key elements", de: "Robustheit: Zugglieder, Wegfall, Schlüsselelemente", inputs: &["g_k", "q_k", "psi", "s", "l", "floor_area", "key_area", "provided_tie"], eval: e17_06 },
    Entry { id: "E17.07", standard: "EN 1991-1-7", clause: "B", en: "Risk analysis", de: "Risikoanalyse", inputs: &["p_h", "p_d_h", "consequence", "r_acc"], eval: e17_07 },
    Entry { id: "E17.08", standard: "EN 1991-1-7", clause: "C", en: "Dynamic design for impact", de: "Dynamische Bemessung für Anprall", inputs: &["mass", "speed", "k", "f_pl", "y_el", "y_allowed"], eval: e17_08 },
    Entry { id: "E17.09", standard: "EN 1991-1-7", clause: "D", en: "Internal gas explosions", de: "Innere Gasexplosionen", inputs: &["p_v", "av_over_v"], eval: e17_09 },
    Entry { id: "E17.10", standard: "EN 1991-1-7", clause: "4.1", en: "External explosion overpressure", de: "Äußerer Explosionsüberdruck", inputs: &["charge_kg", "standoff"], eval: e17_10 },
];
