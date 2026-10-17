//! 🌉️ Footbridge member and connection assessments of the EN 1995 timber checks. See [`super`].

use super::*;

fn crowd_line_n_per_m(m: &TimberMember) -> f64 {
    m.bridge_crowd_per_m2.max(0.0) * 800.0 * m.b_m.max(0.05)
}

pub(super) fn assess_bridge_member(annex: AnnexChoice, params: &AnnexParams, m: &TimberMember, props: &TimberProperties) -> Vec<Assessed<TimberMember>> {
    let head = |id: String, clause: ClauseId, leaf: &str, en: &str, de: &str| Head { id, part: "EN 1995-2", clause, subject: member_subject(m, leaf), title: loc(en, de), annex };
    let km = k_mod(parse_service_class(m.service_class), LoadDuration::Medium);
    let gamma = params.gamma_m(props.product);
    let kh = k_h(m.h_m, props.product);
    let wsec = w_m3(m);
    let q_crowd = crowd_line_n_per_m(m);
    let moment_coefficient = match m.support {
        SupportType::SimplySupported => 1.0 / 8.0,
        SupportType::Cantilever => 1.0 / 2.0,
        SupportType::ContinuousTwoSpan => 1.0 / 14.0,
    };
    let m_crowd = q_crowd * m.span_m.powi(2) * moment_coefficient;
    let m_ed = 1.5 * m_crowd;
    let f_m_d = km * kh * k_crit(lambda_rel_m(wsec, props.f_m_k, effective_m_crit_nm(m))) * props.f_m_k / gamma;
    let sigma_m = if wsec > 0.0 { m_ed / wsec } else { 0.0 };

    let m_fat = 0.4 * m_crowd;
    let delta_sigma = if wsec > 0.0 { m_fat / wsec } else { 0.0 };
    let delta_mpa = (delta_sigma / 1e6).max(1e-6);
    let n = (m.bridge_n_obs * m.bridge_t_l_years).max(1.0);
    let a_w = if m.bridge_a > 0.0 { m.bridge_a } else { 15.0 };
    let b_w = if m.bridge_b > 0.0 { m.bridge_b } else { 4.0 };
    let beta = if m.bridge_beta > 0.0 { m.bridge_beta } else { 5.0 };
    let n_r = 10.0_f64.powf(a_w - b_w * delta_mpa.log10()).max(1.0);
    let k_fat = (n_r / n).powf(1.0 / beta).min(1.0);
    let f_fat_d = km * k_fat * props.f_m_k / gamma;
    let mut out = vec![assessed(
        head(format!("en1995.2.a.fatigue.{}", m.id), ClauseId::new("EN 1995-2", "Annex A", "A"), "bridgeNObs", "Bridge fatigue (Annex A)", "Brückenermüdung (Anhang A)"),
        Quantity::new(QuantityKind::Stress, delta_sigma),
        Quantity::new(QuantityKind::Stress, f_fat_d.max(1e-9)),
        loc(
            &format!("Δσ={:.3} MPa, N={:.0}, N_R={:.0}, k_fat={:.3}; σ vs f_fat,d={:.3} MPa.", delta_mpa, n, n_r, k_fat, f_fat_d / 1e6),
            &format!("Δσ={:.3} MPa, N={:.0}, N_R={:.0}, k_fat={:.3}; σ gegen f_fat,d={:.3} MPa.", delta_mpa, n, n_r, k_fat, f_fat_d / 1e6),
        ),
        DEPTH_LEVERS,
    )];

    let ei = props.e_0_mean * i_m4(m);
    let mu = if m.mass_kg_per_m > 0.0 { m.mass_kg_per_m } else { props.rho_k * area_m2(m) };
    let f1 = if mu > 0.0 && ei > 0.0 && m.span_m > 0.0 { (std::f64::consts::PI / (2.0 * m.span_m.powi(2))) * (ei / mu).sqrt() } else { 1.0 };
    let xi = if m.damping_xi > 0.0 { m.damping_xi } else { 0.01 };
    let a_vert = (m.bridge_crowd_per_m2.max(0.0) / 1.5) * (5.0 / f1.max(0.5)) * (0.01 / xi).sqrt() * 0.35;
    let a_hor = 0.3 * a_vert;
    let (lim_v, lim_h) = (params.bridge_a_vert_limit(), params.bridge_a_hor_limit());
    out.push(assessed(
        head(format!("en1995.2.b.avert.{}", m.id), ClauseId::new("EN 1995-2", "Annex B", "B"), "bridgeCrowdPerM2", "Bridge pedestrian a_vert", "Brücke Fußgänger a_vert"),
        Quantity::acceleration_m_s2(a_vert),
        Quantity::acceleration_m_s2(lim_v),
        loc(&format!("a_vert={a_vert:.3} m/s² vs ≤ {lim_v:.3} m/s² (f1={f1:.2} Hz)."), &format!("a_vert={a_vert:.3} m/s² gegen ≤ {lim_v:.3} m/s² (f1={f1:.2} Hz).")),
        DEPTH_LEVERS,
    ));
    out.push(assessed(
        head(format!("en1995.2.b.ahor.{}", m.id), ClauseId::new("EN 1995-2", "Annex B", "B"), "bridgeCrowdPerM2", "Bridge pedestrian a_hor", "Brücke Fußgänger a_hor"),
        Quantity::acceleration_m_s2(a_hor),
        Quantity::acceleration_m_s2(lim_h),
        loc(&format!("a_hor={a_hor:.3} m/s² vs ≤ {lim_h:.3} m/s²."), &format!("a_hor={a_hor:.3} m/s² gegen ≤ {lim_h:.3} m/s².")),
        DEPTH_LEVERS,
    ));
    out.push(assessed(
        head(format!("en1995.2.uls.bending.{}", m.id), ClauseId::new("EN 1995-2", "§5", "5"), "bridgeCrowdPerM2", "Bridge ULS bending", "Brücke GZT Biegung"),
        Quantity::new(QuantityKind::Stress, sigma_m),
        Quantity::new(QuantityKind::Stress, f_m_d.max(1e-9)),
        loc(
            &format!("Crowd LM ULS σ_m={:.3} MPa vs f_m,d={:.3} MPa.", sigma_m / 1e6, f_m_d / 1e6),
            &format!("Personen-LM GZT σ_m={:.3} MPa gegen f_m,d={:.3} MPa.", sigma_m / 1e6, f_m_d / 1e6),
        ),
        DEPTH_LEVERS,
    ));
    if m.span_m > 0.0 {
        let w = deflection_m(m, props, q_crowd, 0.0);
        let lim = m.span_m / 400.0;
        out.push(assessed(
            head(format!("en1995.2.sls.deflection.{}", m.id), ClauseId::new("EN 1995-2", "§7", "7"), "bridgeCrowdPerM2", "Bridge SLS deflection", "Brücke GZG Durchbiegung"),
            Quantity::length_m(w),
            Quantity::length_m(lim),
            loc(&format!("w={:.1} mm vs L/400={:.1} mm.", w * 1000.0, lim * 1000.0), &format!("w={:.1} mm gegen L/400={:.1} mm.", w * 1000.0, lim * 1000.0)),
            DEPTH_LEVERS,
        ));
    }
    out
}

pub(super) fn evaluate_connection(annex: AnnexChoice, params: &AnnexParams, c: &TimberConnection) -> Vec<CheckResult> {
    let Some(props) = properties_for_class(&c.strength_class) else {
        return vec![CheckResult::assess(
            format!("en1995.conn.material.{}", c.id),
            "EN 1995-1-1",
            ClauseId::new("EN 1995-1-1", "§8", "8"),
            conn_subject(c, "strengthClass"),
            loc("Connection timber class", "Holzklasse der Verbindung"),
        )
        .status(semio_s_plugin_norm_compliance::CheckStatus::Fail)
        .explanation(loc(
            &format!("Unknown strength class '{}'.", c.strength_class),
            &format!("Unbekannte Festigkeitsklasse '{}'.", c.strength_class),
        ))
        .remedy(Remedy::one_of(
            conn_subject(c, "strengthClass"),
            strength_class_options().iter().map(|s| s.to_string()).collect(),
            loc("Select a tabulated strength class (EN 338 / EN 14080).", "Tabellierte Festigkeitsklasse wählen (EN 338 / EN 14080)."),
        ))
        .annex(annex)
        .build()];
    };
    let probe = |candidate: &TimberConnection| assess_connection(annex, params, candidate, &props).into_iter().map(|a| (a.id, a.utilization)).collect();
    finish(c, assess_connection(annex, params, c, &props), conn_subject, &probe)
}

fn assess_connection(annex: AnnexChoice, params: &AnnexParams, c: &TimberConnection, props: &TimberProperties) -> Vec<Assessed<TimberConnection>> {
    let combos = enumerate_connection_combos(c);
    let Some((_, _, duration, f_ed)) = combos
        .iter()
        .filter(|(_, k, _, _)| *k == ComboKind::Uls)
        .max_by(|a, b| a.3.abs().partial_cmp(&b.3.abs()).unwrap_or(std::cmp::Ordering::Equal))
        .cloned()
    else {
        return Vec::new();
    };
    let head = |id: String, clause: &str, leaf: &str, en: &str, de: &str| Head {
        id,
        part: "EN 1995-1-1",
        clause: ClauseId::new("EN 1995-1-1", &format!("§{clause}"), clause),
        subject: conn_subject(c, leaf),
        title: loc(en, de),
        annex,
    };
    let km = k_mod(parse_service_class(c.service_class), duration);
    let gamma = params.gamma_m_connection();
    let fh = f_h_k(props.rho_k, c.diameter_m, &c.fastener_type);
    let my = m_y_k(c.f_u_k, c.diameter_m);
    let fax = if c.fastener_type.to_ascii_lowercase().contains("screw") { 0.2 * fh * c.diameter_m * c.t1_m } else { 0.0 };
    let f_v_rk_1 = match (c.steel_plate, c.shear_planes >= 2) {
        (true, true) => johansen_steel_central_f_v_rk(c.t1_m, c.diameter_m, fh, my, fax),
        (true, false) => johansen_steel_timber_f_v_rk(c.t1_m, c.steel_plate_thickness_m, c.diameter_m, fh, my, fax),
        (false, true) => johansen_double_shear_f_v_rk(c.t1_m, c.t2_m, c.diameter_m, fh, fh, my, fax),
        (false, false) => johansen_single_shear_f_v_rk(c.t1_m, c.t2_m, c.diameter_m, fh, fh, my, fax),
    };
    let rows = c.rows.max(1);
    let n_per_row = ((c.number as f64) / (rows as f64)).round().max(1.0) as u32;
    let nef = n_ef(n_per_row, c.spacing_m, c.diameter_m) * (rows as f64);
    let planes = c.shear_planes.max(1) as f64;
    let f_v_rd = km * nef * planes * f_v_rk_1 / gamma;
    let clause = if c.steel_plate { "8.2.3" } else { "8.2.2" };
    let mut out = vec![assessed(
        head(format!("en1995.{clause}.johansen.{}", c.id), clause, "actions", "Dowel-type connection (Johansen EYM)", "Stiftförmige Verbindung (Johansen)"),
        Quantity::force_kn(f_ed / 1000.0),
        Quantity::force_kn(f_v_rd / 1000.0),
        loc(
            &format!("F_Ed={:.2} kN vs F_v,Rd={:.2} kN (F_v,Rk={:.2} kN, n_ef={:.2}, planes={:.1}).", f_ed / 1000.0, f_v_rd / 1000.0, f_v_rk_1 / 1000.0, nef, planes),
            &format!("F_Ed={:.2} kN gegen F_v,Rd={:.2} kN (F_v,Rk={:.2} kN, n_ef={:.2}, Ebenen={:.1}).", f_ed / 1000.0, f_v_rd / 1000.0, f_v_rk_1 / 1000.0, nef, planes),
        ),
        JOHANSEN_LEVERS,
    )];
    let (a1_min, a3_min, a4_min) = spacing_minima(c);
    for (kind, leaf, minimum, provided, en, de, levers) in [
        ("a1", "spacingM", a1_min, c.spacing_m, "Fastener spacing a₁", "Verbindungsmittelabstand a₁", SPACING_LEVERS),
        ("a3t", "endDistanceM", a3_min, c.end_distance_m, "End distance a₃,t", "Hirnholzabstand a₃,t", END_LEVERS),
        ("a4t", "edgeDistanceM", a4_min, c.edge_distance_m, "Edge distance a₄,t", "Randabstand a₄,t", EDGE_LEVERS),
    ] {
        out.push(assessed(
            head(format!("en1995.8.spacing.{kind}.{}", c.id), "8.3", leaf, en, de),
            Quantity::length_m(minimum),
            Quantity::length_m(provided.max(1e-12)),
            loc(
                &format!("Required ≥ {:.1} mm, provided {:.1} mm (Tables 8.2–8.5, α = 0).", minimum * 1000.0, provided * 1000.0),
                &format!("Erforderlich ≥ {:.1} mm, vorhanden {:.1} mm (Tabellen 8.2–8.5, α = 0).", minimum * 1000.0, provided * 1000.0),
            ),
            levers,
        ));
    }
    out
}
