//! 🔥️ EN 1991-1-2 catalogue entries E12.01 … E12.10: nominal and parametric fire curves, fire load, equivalent duration, localised fire, heat flux, fire combination, external flames, configuration factor and imported curves.

use super::*;
use semio_s_artifact_norm_en1991_model::part_1_2::FireCurve;

fn kelvin(c: f64) -> f64 { c + 273.15 }

fn e12_01(c: &Case) -> R {
    let t = c.n("t_min")?;
    if t < 0.0 { return Err("input t_min must not be negative".into()); }
    let curve = match c.s("curve")? { "standard" => FireCurve::Standard, "external" => FireCurve::External, "hydrocarbon" => FireCurve::Hydrocarbon, other => return Err(format!("unknown curve {other}; admissible: standard, external, hydrocarbon")) };
    let theta = part_1_2::nominal_gas_temp_k(curve, t * 60.0).ok_or("curve needs compartment data")?;
    Ok(vec![
        Outcome::info("theta_g", "Gas temperature θ_g", "Gastemperatur θ_g", QuantityKind::Temperature, theta).steps(format!("θ_g({t:.1} min) = {:.1} °C.", theta - 273.15)).basis(&["ndp.en1991.e12.firecurves"]),
        Outcome::info("alpha_c", "Convection coefficient α_c", "Wärmeübergangskoeffizient α_c", QuantityKind::HeatTransferCoefficient, part_1_2::alpha_c_for_curve(curve)).steps("α_c = 25 W/m²K (50 for the hydrocarbon curve).").steps_de("α_c = 25 W/m²K (50 bei der Kohlenwasserstoffkurve)."),
    ])
}

fn e12_02(c: &Case) -> R {
    if c.de() { return Err("The German National Annex uses Annex AA for a fully developed compartment fire".into()); }
    let (af, at, av, heq) = (c.pos("a_f")?, c.pos("a_t")?, c.pos("a_v")?, c.pos("h_eq")?);
    let b = c.n("b")?;
    if !(100.0..=2200.0).contains(&b) { return Err("input b must lie within 100 to 2200 J/m²s^½K".into()); }
    let qfd = c.pos("q_fd_mj")?;
    let t_lim = c.o("t_lim_min", 20.0) / 60.0;
    let p = part_1_2::parametric_fire(af, at, av, heq, b, qfd, t_lim);
    let mut out = vec![
        Outcome::info("O", "Opening factor O", "Öffnungsfaktor O", QuantityKind::Dimensionless, p.o).steps(format!("O = A_v√h_eq/A_t = {:.5} m^½.", p.o)).basis(&["ndp.en1991.e12.annex-a"]),
        Outcome::info("gamma", "Γ of the heating phase", "Γ der Erwärmungsphase", QuantityKind::Dimensionless, p.gamma_heating).steps(format!("Γ = {:.5}.", p.gamma_heating)).basis(&["ndp.en1991.e12.annex-a"]),
        Outcome::info("q_td", "Design fire load q_t,d", "Bemessungsbrandlast q_t,d", QuantityKind::Dimensionless, p.q_td_mj).steps(format!("q_t,d = q_f,d·A_f/A_t = {:.3} MJ/m².", p.q_td_mj)).basis(&["ndp.en1991.e12.annex-a"]),
        Outcome::info("t_max", "Time of maximum temperature t_max", "Zeitpunkt der Maximaltemperatur t_max", QuantityKind::Time, p.t_max_h * 3600.0).steps(format!("t_max = {:.4} h ({}).", p.t_max_h, if p.fuel_controlled { "fuel controlled" } else { "ventilation controlled" })).basis(&["ndp.en1991.e12.annex-a"]),
        Outcome::info("theta_max", "Maximum gas temperature θ_max", "Maximale Gastemperatur θ_max", QuantityKind::Temperature, kelvin(p.theta_max_c)).steps(format!("θ_max = {:.2} °C.", p.theta_max_c)).basis(&["ndp.en1991.e12.annex-a"]),
    ];
    if let Some(t) = c.opt("t_min") {
        let th = part_1_2::parametric_theta_c(&p, t / 60.0);
        out.push(Outcome::info("theta_g", "Gas temperature θ_g(t)", "Gastemperatur θ_g(t)", QuantityKind::Temperature, kelvin(th)).steps(format!("θ_g({t:.1} min) = {th:.2} °C.")).basis(&["ndp.en1991.e12.annex-a"]));
    }
    out.push(Outcome::warn("validity_q", "q_t,d within 50 to 1000 MJ/m²", "q_t,d innerhalb 50 bis 1000 MJ/m²", QuantityKind::Dimensionless, if p.q_td_mj < 50.0 { 50.0 / p.q_td_mj } else { p.q_td_mj / 1000.0 }, 1.0).steps(format!("q_t,d = {:.2} MJ/m².", p.q_td_mj)));
    out.push(Outcome::warn("validity_o", "O within 0.02 to 0.20", "O innerhalb 0,02 bis 0,20", QuantityKind::Dimensionless, if p.o < 0.02 { 0.02 / p.o } else { p.o / 0.20 }, 1.0).steps(format!("O = {:.4}.", p.o)));
    out.push(Outcome::warn("validity_geometry", "A_f ≤ 500 m², h ≤ 4 m", "A_f ≤ 500 m², h ≤ 4 m", QuantityKind::Dimensionless, (af / 500.0).max(heq / 4.0), 1.0).steps(format!("A_f = {af:.1} m², h_eq = {heq:.2} m.")));
    Ok(out)
}

fn delta_n(c: &Case) -> Result<f64, String> {
    let flag = |k: &str| c.o(k, 0.0) > 0.5;
    let water = match c.o("water_supplies", 0.0) as u8 { 0 => 1.0, 1 => 0.87, 2 => 0.7, _ => return Err("input water_supplies must be 0, 1 or 2".into()) };
    Ok([(flag("sprinkler"), 0.61), (flag("heat_detection"), 0.87), (flag("smoke_detection"), 0.73), (flag("alarm"), 0.87), (flag("onsite_brigade"), 0.61), (flag("offsite_brigade"), 0.78), (flag("safe_access"), 0.9)].iter().fold(water, |acc, (on, f)| if *on { acc * f } else { acc }))
}

fn e12_03(c: &Case) -> R {
    if c.de() { return german_fire_load(c); }
    let area = c.pos("area")?;
    let (qfk, source) = match c.opt("q_fk_mj") {
        Some(v) if v > 0.0 => (v * 1e6, "given".to_string()),
        Some(_) => return Err("input q_fk_mj must be positive".into()),
        None => {
            let occ = c.s("occupancy")?;
            (part_1_2::characteristic_fire_load_j_m2(c.annex, occ).ok_or_else(|| format!("unknown occupancy {occ}; admissible: {}", part_1_2::OCCUPANCIES.join(", ")))?, occ.to_string())
        }
    };
    let dq2 = match c.opt("delta_q2") { Some(v) => v, None => part_1_2::occupancy_row(c.annex, c.s("occupancy")?).ok_or("unknown occupancy")?.1 };
    let dn = delta_n(c)?;
    let dq1 = part_1_2::delta_q1(area);
    let qfd = qfk * part_1_2::combustion_factor_m() * dq1 * dq2 * dn;
    Ok(vec![
        Outcome::info("delta_q1", "Compartment size factor δ_q1", "Abschnittsgrößenfaktor δ_q1", QuantityKind::Dimensionless, dq1).steps(format!("δ_q1({area:.0} m²) = {dq1:.4}.")).basis(&["ndp.en1991.e12.annex-e"]),
        Outcome::info("delta_n", "Active measures factor δ_n", "Faktor aktive Maßnahmen δ_n", QuantityKind::Dimensionless, dn).steps(format!("δ_n = {dn:.4}.")).basis(&["ndp.en1991.e12.annex-e"]),
        Outcome::info("q_fd", "Design fire load q_f,d", "Bemessungsbrandlast q_f,d", QuantityKind::Energy, qfd).steps(format!("q_f,d = q_f,k({source})·m·δ_q1·δ_q2·δ_n = {:.3} MJ/m².", qfd / 1e6)).basis(&["ndp.en1991.e12.annex-e"]),
    ])
}

fn german_fire_load(c: &Case) -> R {
    let gamma = c.pos("bb_gamma")?;
    let (qfd, steps) = match c.s("bb_source")? {
        "typical_use" => {
            let occupancy = c.s("occupancy")?;
            let q_occ = part_1_2::characteristic_fire_load_j_m2(AnnexChoice::De, occupancy)
                .ok_or_else(|| format!("No typical-use row in Table BB.1 for {occupancy}"))? / 1.0e6;
            let chi = c.pos("bb_use_chi")?;
            let construction = c.n("bb_construction_mj")?;
            if chi > 1.0 || construction < 0.0 {
                return Err("BB.3.2 requires combustion efficiency at most 1 and nonnegative construction fire load".into());
            }
            let load = gamma * (chi * q_occ + construction);
            (load, format!("BB.1 · {occupancy} · q_f,d = {gamma:.3} ({chi:.2} × {q_occ:.0} + {construction:.1}) = {load:.1} MJ/m²."))
        }
        "survey" => {
            let survey = c.n("bb_survey_mj")?;
            if survey < 0.0 {
                return Err("BB.2 requires a nonnegative effective surveyed fire load".into());
            }
            let load = (gamma * survey).max(50.0);
            (load, format!("BB.2 · q_f,d = max(50, {gamma:.3} × {survey:.1}) = {load:.1} MJ/m²."))
        }
        source => return Err(format!("unknown fire load basis {source}")),
    };
    Ok(vec![Outcome::info("q_fd", "Design fire load q_f,d", "Bemessungsbrandlast q_f,d", QuantityKind::Energy, qfd * 1.0e6)
        .steps(steps)])
}

/// 🔢 Annual and conditional structural fire reliability under German Annex BB.5.
fn e12_03_1(c: &Case) -> R {
    use crate::annex_bb_reliability::{annual_failure_target, brigade_failure, ignition_model, ignition_probability, suppression_failure, CompartmentReliability};

    if !c.de() { return Err("Use the German annex for the Annex BB.5 reliability method".into()); }
    let area = c.pos("area")?;
    let occupancy = c.s("bb_ignition_use")?;
    let (coefficient, exponent, average) = ignition_model(occupancy).ok_or("Select an occupancy covered by Table BB.3")?;
    let area_probability = ignition_probability(area, coefficient, exponent);
    let p1 = match c.s("bb_probability_method")? {
        "area" => area_probability,
        "average" => average,
        "conservative" => area_probability.max(average),
        _ => return Err("Select a supported Table BB.3 probability method".into()),
    };

    let occupants = c.n("bb_occupant_failure")?;
    if !(occupants > 0.0 && occupants <= 1.0) { return Err("The occupant response failure probability must lie within (0, 1]".into()); }
    let brigade = brigade_failure(c.s("bb_brigade")?, c.n("bb_response_min")?, c.n("bb_brigade_project")?)
        .ok_or("Supply an applicable brigade response time or a substantiated project failure probability")?;
    let automatic = suppression_failure(c.s("bb_suppression")?).ok_or("Select a supported suppression system")?;
    let target = annual_failure_target(c.s("bb_failure_group")?, c.s("bb_consequence")?, c.n("bb_project_failure")?)
        .ok_or("Select the applicable Table BB.5 class or a substantiated project target probability")?;
    let manual = occupants * brigade;
    let reliability = CompartmentReliability::evaluate(p1, manual, automatic, target)
        .ok_or("The ratio of target to fire probability must permit a finite positive fire design factor")?;

    Ok(vec![
        Outcome::info("p1", "Annual probability of an initial fire", "Jährliche Wahrscheinlichkeit eines Entstehungsbrandes", QuantityKind::Dimensionless, p1)
            .steps(format!("BB.10: p₁(A_f) = {area_probability:.5e}, Table BB.3 average = {average:.5e}.")),
        Outcome::info("p2", "Manual response failure probability", "Ausfallwahrscheinlichkeit der manuellen Brandbekämpfung", QuantityKind::Dimensionless, manual)
            .steps(format!("BB.11: p₂ = {occupants:.5e} × {brigade:.5e}.")),
        Outcome::info("pfi", "Annual probability of a damaging fire", "Jährliche Wahrscheinlichkeit eines Schadenfeuers", QuantityKind::Dimensionless, reliability.damaging_fire)
            .steps(format!("BB.9: p_fi = p₁·p₂·p₃, p₃ = {automatic:.5e}.")),
        Outcome::info("conditional_failure", "Conditional target failure probability", "Bedingte Ziel-Versagenswahrscheinlichkeit", QuantityKind::Dimensionless, reliability.conditional_failure)
            .steps(format!("BB.13: p_f,fi = p_f / p_fi, p_f = {target:.5e}.")),
        Outcome::info("beta", "Conditional reliability index", "Bedingter Zuverlässigkeitsindex", QuantityKind::Dimensionless, reliability.beta)
            .steps("BB.14: β_fi = −Φ⁻¹(p_f,fi)."),
        Outcome::info("gamma_classified", "Partial factor for classified fire load", "Teilsicherheitsbeiwert für klassifizierte Brandlast", QuantityKind::Dimensionless, reliability.classified_fire_load_factor)
            .steps("BB.15: V = 0.3, α = 0.6."),
        Outcome::info("gamma_surveyed", "Partial factor for surveyed fire load or heat release", "Teilsicherheitsbeiwert für erhobene Brandlast oder Wärmefreisetzung", QuantityKind::Dimensionless, reliability.surveyed_fire_load_factor)
            .steps("BB.15: V = 0.2, α = 0.6."),
    ])
}

fn e12_04(c: &Case) -> R {
    if c.de() { return Err("The German National Annex excludes Annex F for equivalent fire duration".into()); }
    let (qfd, h) = (c.pos("q_fd_mj")?, c.pos("height")?);
    let av = c.n("alpha_v")?;
    if !(0.025..=0.25).contains(&av) { return Err("input alpha_v must lie within 0.025 to 0.25".into()); }
    let kb = match c.opt("k_b") { Some(v) => v, None => { let b = c.n("b")?; if b > 2500.0 { 0.04 } else if b > 720.0 { 0.055 } else { 0.07 } } };
    let t = part_1_2::equivalent_duration_min(qfd, kb, h, av, c.o("alpha_h", 0.0));
    let mut out = vec![Outcome::info("t_ed", "Equivalent fire duration t_e,d", "Äquivalente Branddauer t_e,d", QuantityKind::Time, t * 60.0).steps(format!("t_e,d = q_f,d·k_b·w_f = {t:.2} min.")).basis(&["ndp.en1991.e12.annex-f"])];
    if let Some(r) = c.opt("resistance_min") { out.push(Outcome::limit("resistance", "t_e,d ≤ required resistance R", "t_e,d ≤ geforderte Feuerwiderstandsdauer R", QuantityKind::Time, t * 60.0, r * 60.0).steps(format!("t_e,d = {t:.2} min against R = {r:.0} min.")).steps_de(format!("t_e,d = {t:.2} min gegenüber R = {r:.0} min.")).basis(&["ndp.en1991.e12.annex-f"])); }
    Ok(out)
}

fn e12_05(c: &Case) -> R {
    let d = match c.opt("fire_area") { Some(a) if a > 0.0 => (4.0 * a / std::f64::consts::PI).sqrt(), _ => c.pos("d")? };
    let (q, z) = (c.pos("q_w")?, c.pos("z")?);
    let (lf, z0, th) = part_1_2::local_fire(d, q, z);
    if z <= z0 { return Err("height z must lie above the virtual origin".into()); }
    let mut out = vec![
        Outcome::info("L_f", "Flame length L_f", "Flammenlänge L_f", QuantityKind::Length, lf).steps(format!("L_f = −1.02·D + 0.0148·Q^0.4 = {lf:.3} m (D = {d:.3} m).")).basis(&["ndp.en1991.e12.annex-c"]),
        Outcome::info("z_0", "Virtual origin z_0", "Virtueller Ursprung z_0", QuantityKind::Length, z0).steps(format!("z_0 = {z0:.4} m.")).basis(&["ndp.en1991.e12.annex-c"]),
        Outcome::info("theta", "Plume axis temperature θ(z)", "Plumetemperatur θ(z)", QuantityKind::Temperature, kelvin(th)).steps(format!("θ(z = {z:.2} m) = {th:.1} °C (≤ 900 °C).")).basis(&["ndp.en1991.e12.annex-c"]),
    ];
    if let (Some(h), Some(r)) = (c.opt("ceiling_height"), c.opt("r")) {
        if lf >= h {
            let (lh, zp) = (c.pos("l_h")?, c.o("z_prime", 0.0));
            let y = (r + h + zp) / (lh + h + zp);
            let flux = part_1_2::hasemi_flux_w_m2(y);
            out.push(Outcome::info("hasemi", "Hasemi heat flux ĥ", "Hasemi-Wärmestrom ĥ", QuantityKind::Power, flux).steps(format!("y = {y:.4}: ĥ = {flux:.0} W/m².")).basis(&["ndp.en1991.e12.annex-c"]));
        }
    }
    Ok(out)
}

fn e12_06(c: &Case) -> R {
    let (tg, tm) = (c.n("theta_g_c")?, c.n("theta_m_c")?);
    let alpha_c = c.o("alpha_c", 25.0);
    let h = part_1_2::h_net_w_m2(kelvin(tg), kelvin(tm), alpha_c, part_1_2::configuration_factor(), c.o("eps_m", part_1_2::member_emissivity()), part_1_2::flame_emissivity());
    Ok(vec![Outcome::info("h_net", "Net heat flux ĥ_net", "Nettowärmestrom ĥ_net", QuantityKind::Power, h).steps(format!("ĥ_net = α_c(θ_g−θ_m) + Φ ε_m ε_f σ(θ_g⁴−θ_m⁴) = {h:.0} W/m².")).basis(&["ndp.en1991.e12.firecurves"])])
}

fn e12_07(c: &Case) -> R {
    let (g, q, psi) = (c.pos("g_k")?, c.n("q_k")?, c.n("psi")?);
    let eta = part_1_2::eta_fi(g, q, psi, c.o("gamma_g", semio_s_plugin_norm_compliance::ndp_number("ndp.en1990.gG_sup.str.A1", c.annex)), c.o("gamma_q", semio_s_plugin_norm_compliance::ndp_number("ndp.en1990.gQ.str.A1", c.annex)));
    Ok(vec![Outcome::info("eta_fi", "Reduction factor η_fi", "Abminderungsfaktor η_fi", QuantityKind::Dimensionless, eta).steps(format!("η_fi = (G + ψ Q)/(γ_G G + γ_Q Q) = {eta:.5}."))])
}

fn e12_08(c: &Case) -> R {
    let tf = c.pos("t_f_c")? + 273.15;
    let flux = part_1_2::flame_emissivity() * part_1_2::STEFAN_BOLTZMANN * tf.powi(4);
    Ok(vec![Outcome::info("flux", "Flame radiation intensity I_f", "Flammenstrahlungsintensität I_f", QuantityKind::Power, flux).steps(format!("I_f = ε_f σ T_f⁴ = {flux:.0} W/m² (T_f = {tf:.1} K)."))])
}

fn e12_09(c: &Case) -> R {
    let (w, h, dist) = (c.pos("width")?, c.pos("height")?, c.pos("distance")?);
    let phi = part_1_2::rectangle_configuration_factor(h / dist, w / dist);
    Ok(vec![Outcome::info("phi", "Configuration factor Φ", "Konfigurationsfaktor Φ", QuantityKind::Dimensionless, phi).steps(format!("Φ({:.3}, {:.3}) = {phi:.5} (corner receiver; four of them for a centred receiver).", h / dist, w / dist)).steps_de(format!("Φ({:.3}, {:.3}) = {phi:.5} (Eckempfänger; vier davon bei zentralem Empfänger).", h / dist, w / dist))])
}

fn e12_10(c: &Case) -> R {
    let (times, temps) = (c.v("times_min")?, c.v("temps_c")?);
    if times.len() != temps.len() || times.len() < 2 || times.windows(2).any(|w| w[1] <= w[0]) { return Err("times_min and temps_c must be strictly increasing series of equal length ≥ 2".into()); }
    let t = c.n("t_min")?;
    let rows: Vec<(f64, f64)> = times.iter().copied().zip(temps.iter().copied()).collect();
    let th = numeric::table(&rows, t);
    Ok(vec![Outcome::info("theta_g", "Imported gas temperature θ_g", "Importierte Gastemperatur θ_g", QuantityKind::Temperature, kelvin(th)).steps(format!("θ_g({t:.1} min) = {th:.1} °C by linear interpolation.")).steps_de(format!("θ_g({t:.1} min) = {th:.1} °C durch lineare Interpolation."))])
}

pub(super) const ENTRIES: &[Entry] = &[
    Entry { id: "E12.01", standard: "EN 1991-1-2", clause: "3.2.1", en: "Nominal temperature-time curves", de: "Einheits-Temperaturzeitkurven", inputs: &["curve", "t_min"], eval: e12_01 },
    Entry { id: "E12.02", standard: "EN 1991-1-2", clause: "3.3 / A", en: "Parametric temperature-time curve", de: "Parametrische Temperaturzeitkurve", inputs: &["a_f", "a_t", "a_v", "h_eq", "b", "q_fd_mj", "t_lim_min", "t_min"], eval: e12_02 },
    Entry { id: "E12.03", standard: "EN 1991-1-2", clause: "3.3.2 / E / NA BB.3", en: "Design fire load density", de: "Brandlastdichte", inputs: &["area", "occupancy", "q_fk_mj", "delta_q2", "sprinkler", "water_supplies", "heat_detection", "smoke_detection", "alarm", "onsite_brigade", "offsite_brigade", "safe_access", "bb_source", "bb_gamma", "bb_use_chi", "bb_construction_mj", "bb_survey_mj"], eval: e12_03 },
    Entry { id: "E12.03.1", standard: "EN 1991-1-2", clause: "DIN NA BB.5", en: "Reliability and partial factors for a German compartment fire", de: "Zuverlässigkeit und Teilsicherheitsbeiwerte für einen Brandabschnitt nach deutschem NA", inputs: &["area", "bb_ignition_use", "bb_probability_method", "bb_occupant_failure", "bb_brigade", "bb_response_min", "bb_brigade_project", "bb_suppression", "bb_failure_group", "bb_consequence", "bb_project_failure"], eval: e12_03_1 },
    Entry { id: "E12.04", standard: "EN 1991-1-2", clause: "F", en: "Equivalent fire duration", de: "Äquivalente Branddauer", inputs: &["q_fd_mj", "height", "alpha_v", "alpha_h", "k_b", "b", "resistance_min"], eval: e12_04 },
    Entry { id: "E12.05", standard: "EN 1991-1-2", clause: "C", en: "Localised fires", de: "Lokale Brände", inputs: &["d", "fire_area", "q_w", "z", "ceiling_height", "r", "l_h", "z_prime"], eval: e12_05 },
    Entry { id: "E12.06", standard: "EN 1991-1-2", clause: "3.1 / 3.2", en: "Net heat flux", de: "Netto-Wärmestrom", inputs: &["theta_g_c", "theta_m_c", "alpha_c", "eps_m"], eval: e12_06 },
    Entry { id: "E12.07", standard: "EN 1991-1-2", clause: "4.3", en: "Actions in the fire design situation", de: "Einwirkungen im Brandfall", inputs: &["g_k", "q_k", "psi", "gamma_g", "gamma_q"], eval: e12_07 },
    Entry { id: "E12.08", standard: "EN 1991-1-2", clause: "B", en: "Thermal actions on external members", de: "Thermische Einwirkungen auf außenliegende Bauteile", inputs: &["t_f_c"], eval: e12_08 },
    Entry { id: "E12.09", standard: "EN 1991-1-2", clause: "G", en: "Configuration factor", de: "Konfigurationsfaktor", inputs: &["width", "height", "distance"], eval: e12_09 },
    Entry { id: "E12.10", standard: "EN 1991-1-2", clause: "3.3.3", en: "Imported temperature-time series", de: "Importierte Temperaturzeitreihe", inputs: &["times_min", "temps_c", "t_min"], eval: e12_10 },
];
