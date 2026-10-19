//! 🏗️ Structure and member assessments of the EN 1995 timber checks: the integrity checks, the structure entry point and the member, floor and fire assessments. See [`super`].

use super::*;

/// 🏗 Evaluate all members and connections for the chosen annex.
fn push_duplicate_ids(
    report: &mut CheckReport,
    annex: AnnexChoice,
    table: &str,
    ids: &[String],
    path_for: &dyn Fn(&str) -> String,
) {
    let mut counts = std::collections::BTreeMap::<String, usize>::new();
    for id in ids {
        *counts.entry(id.clone()).or_insert(0) += 1;
    }
    for (id, count) in counts {
        if count < 2 {
            continue;
        }
        let path = path_for(&id);
        let subject = SubjectRef::new(id.clone(), &path, loc(&format!("Duplicate {table} id"), &format!("Doppelte {table}-Id")));
        let options: Vec<String> = ids.iter().filter(|x| x.as_str() != id).cloned().collect();
        let free = if options.is_empty() { vec![format!("{id}-unique")] } else { options };
        report.push(
            CheckResult::assess(
                format!("en1995.integrity.duplicate.{table}.{id}"),
                "EN 1995 integrity",
                ClauseId::new("EN 1995-1-1", "§1", "id"),
                subject.clone(),
                loc(&format!("Unique {table} id"), &format!("Eindeutige {table}-Id")),
            )
            .annex(annex)
            .explanation(loc(
                &format!("Duplicate {table} id '{id}' appears {count} times; each entity id must be unique."),
                &format!("Doppelte {table}-Id '{id}' kommt {count}-mal vor; jede Entitäts-Id muss eindeutig sein."),
            ))
            .status(semio_s_plugin_norm_compliance::CheckStatus::Fail)
            .remedy(Remedy::one_of(
                subject,
                free,
                loc(
                    &format!("Rename the duplicated '{id}' entry to a free id."),
                    &format!("Den doppelten '{id}'-Eintrag auf eine freie Id umbenennen."),
                ),
            ))
            .build(),
        );
    }
}

fn push_referential_integrity(report: &mut CheckReport, annex: AnnexChoice, members: &[TimberMember], connections: &[TimberConnection]) {
    push_duplicate_ids(
        report,
        annex,
        "members",
        &members.iter().map(|m| m.id.clone()).collect::<Vec<_>>(),
        &|id| format!("members[id={id}].id"),
    );
    push_duplicate_ids(
        report,
        annex,
        "connections",
        &connections.iter().map(|c| c.id.clone()).collect::<Vec<_>>(),
        &|id| format!("connections[id={id}].id"),
    );
    for m in members {
        let mid = &m.id;
        push_duplicate_ids(
            report,
            annex,
            "memberActions",
            &m.actions.iter().map(|a| a.id.clone()).collect::<Vec<_>>(),
            &|id| format!("members[id={mid}].actions[id={id}].id"),
        );
    }
    for c in connections {
        let cid = &c.id;
        push_duplicate_ids(
            report,
            annex,
            "connectionActions",
            &c.actions.iter().map(|a| a.id.clone()).collect::<Vec<_>>(),
            &|id| format!("connections[id={cid}].actions[id={id}].id"),
        );
    }
}

pub fn evaluate_structure(annex: AnnexChoice, members: &[TimberMember], connections: &[TimberConnection]) -> CheckReport {
    let mut report = CheckReport::default();
    let params = AnnexParams::for_annex(annex);
    push_referential_integrity(&mut report, annex, members, connections);
    if members.is_empty() {
        report.push(
            CheckResult::assess(
                "en1995.subject.empty",
                "EN 1995-1-1",
                ClauseId::new("EN 1995-1-1", "§1", "1"),
                SubjectRef::whole(loc("Timber structure", "Holztragwerk")),
                loc("Subject completeness", "Vollständigkeit des Gegenstands"),
            )
            .not_applicable(loc("No timber members defined.", "Keine Holzbauteile definiert."))
            .build(),
        );
    }
    for m in members {
        report.extend(evaluate_member(annex, &params, m));
    }
    for c in connections {
        report.extend(evaluate_connection(annex, &params, c));
    }
    report
}

fn evaluate_member(annex: AnnexChoice, params: &AnnexParams, m: &TimberMember) -> Vec<CheckResult> {
    let Some(props) = properties_for_class(&m.strength_class) else {
        return vec![CheckResult::assess(
            format!("en1995.material.unknown.{}", m.id),
            "EN 1995-1-1",
            ClauseId::new("EN 1995-1-1", "§3", "3.2"),
            member_subject(m, "strengthClass"),
            loc("Strength class", "Festigkeitsklasse"),
        )
        .status(semio_s_plugin_norm_compliance::CheckStatus::Fail)
        .explanation(loc(&format!("Unknown strength class '{}'.", m.strength_class), &format!("Unbekannte Festigkeitsklasse '{}'.", m.strength_class)))
        .remedy(Remedy::one_of(
            member_subject(m, "strengthClass"),
            strength_class_options().iter().map(|s| s.to_string()).collect(),
            loc("Select a tabulated strength class (EN 338 / EN 14080).", "Tabellierte Festigkeitsklasse wählen (EN 338 / EN 14080)."),
        ))
        .annex(annex)
        .build()];
    };
    let probe = |candidate: &TimberMember| assess_member(annex, params, candidate, &props).into_iter().map(|a| (a.id, a.utilization)).collect();
    finish(m, assess_member(annex, params, m, &props), member_subject, &probe)
}

fn assess_member(annex: AnnexChoice, params: &AnnexParams, m: &TimberMember, props: &TimberProperties) -> Vec<Assessed<TimberMember>> {
    if m.role == MemberRole::Bridge {
        let mut out = assess_bridge_member(annex, params, m, props);
        if m.fire_duration_s > 0.0 {
            out.push(assess_fire(annex, params, m, props, None));
        }
        return out;
    }
    let mut out = Vec::new();
    let combos = enumerate_combos(m);
    let Some(uls) = governing_uls(&combos) else {
        return out;
    };
    let head = |kind: &str, clause: &str, leaf: &str, en: &str, de: &str| Head {
        id: format!("en1995.{clause}.{kind}.{}", m.id),
        part: "EN 1995-1-1",
        clause: ClauseId::new("EN 1995-1-1", &format!("§{clause}"), clause),
        subject: member_subject(m, leaf),
        title: loc(en, de),
        annex,
    };
    let sc = parse_service_class(m.service_class);
    let km = k_mod(sc, uls.duration);
    let kh = k_h(m.h_m, props.product);
    let gamma = params.gamma_m(props.product);
    let a = area_m2(m);
    let wsec = w_m3(m);
    let m_ed = uls.m_ed_nm;
    let v_ed = uls.v_ed_n;
    let n_ed = uls.n_ed_n;
    let n_t_ed = uls.n_t_ed_n;
    let f_c90_ed = uls.f_c90_ed_n;

    let kcrit = k_crit(lambda_rel_m(wsec, props.f_m_k, effective_m_crit_nm(m)));
    let f_m_d = km * kh * kcrit * props.f_m_k / gamma;
    let sigma_m = if wsec > 0.0 { m_ed / wsec } else { 0.0 };
    out.push(assessed(
        head("bending", "6.1.6", "actions", "Bending stress", "Biegespannung"),
        Quantity::new(QuantityKind::Stress, sigma_m),
        Quantity::new(QuantityKind::Stress, f_m_d.max(1e-9)),
        loc(
            &format!("σ_m,d={:.3} MPa vs f_m,d={:.3} MPa (k_crit={:.3}, combo {}).", sigma_m / 1e6, f_m_d / 1e6, kcrit, uls.id),
            &format!("σ_m,d={:.3} MPa gegen f_m,d={:.3} MPa (k_crit={:.3}, Kombi {}).", sigma_m / 1e6, f_m_d / 1e6, kcrit, uls.id),
        ),
        BENDING_LEVERS,
    ));

    let kcr = params.k_cr(props.f_v_k);
    let kv = k_v(m.h_m, m.notch_depth_m, m.notch_distance_m);
    let b_ef = kcr * m.b_m;
    let f_v_d = km * props.f_v_k / gamma;
    let tau = if b_ef * m.h_m > 0.0 { 1.5 * v_ed / (b_ef * m.h_m) / kv } else { 0.0 };
    out.push(assessed(
        head("shear", "6.1.7", "actions", "Shear stress", "Schubspannung"),
        Quantity::new(QuantityKind::Stress, tau),
        Quantity::new(QuantityKind::Stress, f_v_d.max(1e-9)),
        loc(
            &format!("τ_d={:.3} MPa vs f_v,d={:.3} MPa (k_cr={:.3}, k_v={:.3}).", tau / 1e6, f_v_d / 1e6, kcr, kv),
            &format!("τ_d={:.3} MPa gegen f_v,d={:.3} MPa (k_cr={:.3}, k_v={:.3}).", tau / 1e6, f_v_d / 1e6, kcr, kv),
        ),
        SECTION_LEVERS,
    ));

    if n_t_ed > 0.0 {
        let f_t_d = km * props.f_t_0_k / gamma;
        let sigma_t = if a > 0.0 { n_t_ed / a } else { 0.0 };
        out.push(assessed(
            head("tension", "6.1.2", "actions", "Tension parallel", "Zug parallel"),
            Quantity::new(QuantityKind::Stress, sigma_t),
            Quantity::new(QuantityKind::Stress, f_t_d.max(1e-9)),
            loc(
                &format!("σ_t,0,d={:.3} MPa vs f_t,0,d={:.3} MPa.", sigma_t / 1e6, f_t_d / 1e6),
                &format!("σ_t,0,d={:.3} MPa gegen f_t,0,d={:.3} MPa.", sigma_t / 1e6, f_t_d / 1e6),
            ),
            WIDTH_LEVERS,
        ));
    }

    if n_ed > 0.0 {
        let iy = (i_m4(m) / a.max(1e-12)).sqrt();
        let iz = (m.h_m * m.b_m.powi(3) / 12.0 / a.max(1e-12)).sqrt();
        let lam_y = if iy > 0.0 { m.buckling_length_y_m / iy } else { 0.0 };
        let lam_z = if iz > 0.0 { m.buckling_length_z_m / iz } else { 0.0 };
        let kc_y = k_c_buckling(lambda_rel_compression(lam_y, props.f_c_0_k, props.e_0_05));
        let kc_z = k_c_buckling(lambda_rel_compression(lam_z, props.f_c_0_k, props.e_0_05));
        let kc = kc_y.min(kc_z);
        let f_c_d = km * props.f_c_0_k / gamma;
        let sigma_c = if a > 0.0 { n_ed / a } else { 0.0 };
        out.push(assessed(
            head("compression", "6.3.2", "actions", "Compression with buckling", "Druck mit Knicken"),
            Quantity::new(QuantityKind::Stress, sigma_c),
            Quantity::new(QuantityKind::Stress, (kc * f_c_d).max(1e-9)),
            loc(
                &format!("σ_c,0,d={:.3} MPa vs min(k_c,y,k_c,z)·f_c,0,d={:.3} MPa (k_c,y={:.3}, k_c,z={:.3}).", sigma_c / 1e6, kc * f_c_d / 1e6, kc_y, kc_z),
                &format!("σ_c,0,d={:.3} MPa gegen min(k_c,y,k_c,z)·f_c,0,d={:.3} MPa (k_c,y={:.3}, k_c,z={:.3}).", sigma_c / 1e6, kc * f_c_d / 1e6, kc_y, kc_z),
            ),
            SECTION_LEVERS,
        ));
        let util_c = if kc * f_c_d > 0.0 { sigma_c / (kc * f_c_d) } else { 0.0 };
        let util_m = if f_m_d > 0.0 { sigma_m / f_m_d } else { 0.0 };
        let combined = util_c.powi(2) + util_m;
        out.push(assessed(
            head("combined", "6.2.4", "actions", "Combined compression and bending", "Kombinierter Druck und Biegung"),
            Quantity::new(QuantityKind::Dimensionless, combined),
            Quantity::new(QuantityKind::Dimensionless, 1.0),
            loc(
                &format!("Combined compression and bending: (σ_c/(k_c·f_c,d))² + σ_m/f_m,d = {combined:.3}."),
                &format!("Kombination Druck und Biegung: (σ_c/(k_c·f_c,d))² + σ_m/f_m,d = {combined:.3}."),
            ),
            SECTION_LEVERS,
        ));
    }

    if f_c90_ed > 0.0 && m.bearing_length_m > 0.0 {
        let kc90 = k_c_90(m.bearing_length_m, m.support_length_m);
        let f_c90_d = km * kc90 * props.f_c_90_k / gamma;
        let sigma90 = f_c90_ed / (m.b_m * m.bearing_length_m).max(1e-12);
        out.push(assessed(
            head("c90", "6.1.5", "bearingLengthM", "Compression perpendicular", "Druck quer zur Faser"),
            Quantity::new(QuantityKind::Stress, sigma90),
            Quantity::new(QuantityKind::Stress, f_c90_d.max(1e-9)),
            loc(
                &format!("σ_c,90,d={:.3} MPa vs k_c,90·f_c,90,d={:.3} MPa.", sigma90 / 1e6, f_c90_d / 1e6),
                &format!("σ_c,90,d={:.3} MPa gegen k_c,90·f_c,90,d={:.3} MPa.", sigma90 / 1e6, f_c90_d / 1e6),
            ),
            BEARING_LEVERS,
        ));
    }

    let sls_char = combos
        .iter()
        .filter(|c| c.kind == ComboKind::SlsCharacteristic)
        .max_by(|a, b| a.q_ed_line.abs().partial_cmp(&b.q_ed_line.abs()).unwrap_or(std::cmp::Ordering::Equal));
    if let Some(sls) = sls_char {
        if m.span_m > 0.0 && (sls.q_ed_line.abs() > 0.0 || sls.f_ed_point.abs() > 0.0) {
            let w_inst = deflection_m(m, props, sls.q_ed_line, sls.f_ed_point);
            let w_fin = w_inst * (1.0 + k_def(sc) * sls.psi2);
            let lim_inst = m.span_m / params.w_inst_limit_divisor();
            let lim_fin = m.span_m / params.w_fin_limit_divisor();
            out.push(assessed(
                head("winst", "7.2", "actions", "Instantaneous deflection", "Sofortverformung"),
                Quantity::length_m(w_inst),
                Quantity::length_m(lim_inst),
                loc(
                    &format!("w_inst={:.1} mm vs L/{:.0}={:.1} mm.", w_inst * 1000.0, params.w_inst_limit_divisor(), lim_inst * 1000.0),
                    &format!("w_inst={:.1} mm gegen L/{:.0}={:.1} mm.", w_inst * 1000.0, params.w_inst_limit_divisor(), lim_inst * 1000.0),
                ),
                DEPTH_LEVERS,
            ));
            out.push(assessed(
                head("wfin", "7.2", "actions", "Final deflection", "Endverformung"),
                Quantity::length_m(w_fin),
                Quantity::length_m(lim_fin),
                loc(
                    &format!("w_fin=w_inst·(1+k_def·ψ₂)={:.1} mm vs L/{:.0}={:.1} mm.", w_fin * 1000.0, params.w_fin_limit_divisor(), lim_fin * 1000.0),
                    &format!("w_fin=w_inst·(1+k_def·ψ₂)={:.1} mm gegen L/{:.0}={:.1} mm.", w_fin * 1000.0, params.w_fin_limit_divisor(), lim_fin * 1000.0),
                ),
                DEPTH_LEVERS,
            ));
        }
    }

    let sls_freq = combos
        .iter()
        .filter(|c| c.kind == ComboKind::SlsFrequent)
        .max_by(|a, b| a.q_ed_line.abs().partial_cmp(&b.q_ed_line.abs()).unwrap_or(std::cmp::Ordering::Equal));
    if let Some(freq) = sls_freq {
        if m.span_m > 0.0 && (freq.q_ed_line.abs() > 0.0 || freq.f_ed_point.abs() > 0.0) {
            let w_freq = deflection_m(m, props, freq.q_ed_line, freq.f_ed_point);
            let lim_freq = m.span_m / params.w_inst_limit_divisor();
            out.push(assessed(
                head("wfreq", "7.2", "actions", "Frequent deflection", "Häufige Durchbiegung"),
                Quantity::length_m(w_freq),
                Quantity::length_m(lim_freq),
                loc(
                    &format!("SLS frequent (ψ₁): w_freq={:.1} mm vs L/{:.0}={:.1} mm (combo {}).", w_freq * 1000.0, params.w_inst_limit_divisor(), lim_freq * 1000.0, freq.id),
                    &format!("GZG häufig (ψ₁): w_freq={:.1} mm gegen L/{:.0}={:.1} mm (Kombi {}).", w_freq * 1000.0, params.w_inst_limit_divisor(), lim_freq * 1000.0, freq.id),
                ),
                DEPTH_LEVERS,
            ));
        }
    }

    if m.role == MemberRole::Floor {
        out.extend(assess_floor(annex, params, m, props));
    }
    if m.fire_duration_s > 0.0 {
        out.push(assess_fire(annex, params, m, props, Some(uls)));
    }
    out
}

fn assess_floor(annex: AnnexChoice, params: &AnnexParams, m: &TimberMember, props: &TimberProperties) -> Vec<Assessed<TimberMember>> {
    let l = m.span_m.max(1e-6);
    let ei = props.e_0_mean * i_m4(m);
    let mu = if m.mass_kg_per_m > 0.0 {
        m.mass_kg_per_m
    } else if m.mass_kg_per_m2 > 0.0 {
        m.mass_kg_per_m2 * m.b_m.max(0.05)
    } else {
        props.rho_k * area_m2(m)
    };
    let support_factor = match m.support {
        SupportType::SimplySupported => 1.0,
        SupportType::Cantilever => 0.56,
        SupportType::ContinuousTwoSpan => 1.5,
    };
    let f1 = if mu > 0.0 && ei > 0.0 { support_factor * (std::f64::consts::PI / (2.0 * l * l)) * (ei / mu).sqrt() } else { 0.0 };
    let w_1kn = deflection_m(m, props, 0.0, 1000.0);
    let xi = if m.damping_xi > 0.0 { m.damping_xi } else { 0.01 };
    let m_star = mu * l / 2.0;
    let a_vert = if m_star > 0.0 && f1 > 0.0 {
        (1.0 / (m_star * xi.sqrt()).max(1e-9)) * (f1 / 8.0).max(0.1) * 0.25
    } else {
        0.0
    };
    let m_area = if m.mass_kg_per_m2 > 0.0 { m.mass_kg_per_m2 } else { mu };
    let v_impulse = if m_area > 0.0 && xi > 0.0 {
        (std::f64::consts::PI / (0.8 * m_area * xi)).sqrt()
    } else {
        0.0
    };
    let b_vel = params.floor_velocity_b();
    let v_lim = if f1 > 0.0 && xi > 0.0 { b_vel.powf(f1 * xi - 1.0) } else { 0.0 };

    let head = |kind: &str, leaf: &str, en: &str, de: &str| Head {
        id: format!("en1995.7.3.{kind}.{}", m.id),
        part: "EN 1995-1-1",
        clause: ClauseId::new("EN 1995-1-1", "§7.3", "7.3"),
        subject: member_subject(m, leaf),
        title: loc(en, de),
        annex,
    };
    let mut out = Vec::new();
    let f1_lim = 8.0;
    out.push(assessed(
        head("f1", "hM", "Floor fundamental frequency", "Eigenfrequenz Decke"),
        Quantity::new(QuantityKind::Dimensionless, f1_lim),
        Quantity::new(QuantityKind::Dimensionless, f1.max(1e-9)),
        loc(
            &format!("f₁={:.2} Hz vs DE NA simplified-path threshold {f1_lim:.0} Hz (mass/stiffness/span).", f1),
            &format!("f₁={:.2} Hz gegen DE-NA-Schwelle {f1_lim:.0} Hz (Masse/Steifigkeit/Spannweite).", f1),
        ),
        DEPTH_LEVERS,
    ));
    let w_lim = params.floor_w_1kn_limit_m();
    out.push(assessed(
        head("stiffness", "hM", "Floor unit-load stiffness", "Deckensteifigkeit unter 1 kN"),
        Quantity::length_m(w_1kn),
        Quantity::length_m(w_lim),
        loc(
            &format!("w(1 kN)={:.2} mm vs limit {:.2} mm (§7.3.3).", w_1kn * 1000.0, w_lim * 1000.0),
            &format!("w(1 kN)={:.2} mm gegen Grenzwert {:.2} mm (§7.3.3).", w_1kn * 1000.0, w_lim * 1000.0),
        ),
        DEPTH_LEVERS,
    ));
    if f1 + 1e-9 >= f1_lim {
        out.push(assessed(
            head("velocity", "massKgPerM2", "Floor unit-impulse velocity", "Einheitsimpuls-Geschwindigkeit Decke"),
            Quantity::new(QuantityKind::Dimensionless, v_impulse),
            Quantity::new(QuantityKind::Dimensionless, v_lim.max(1e-12)),
            loc(
                &format!("v={:.4} m/(N·s²) vs b^(f₁ζ−1)={:.4} (b={b_vel:.0}, f₁={:.2} Hz, m={:.0} kg/m²).", v_impulse, v_lim, f1, m_area),
                &format!("v={:.4} m/(N·s²) gegen b^(f₁ζ−1)={:.4} (b={b_vel:.0}, f₁={:.2} Hz, m={:.0} kg/m²).", v_impulse, v_lim, f1, m_area),
            ),
            MASS_LEVERS,
        ));
    } else {
        let a_lim = params.floor_a_limit();
        out.push(assessed(
            head("acceleration", "dampingXi", "Floor acceleration", "Deckenbeschleunigung"),
            Quantity::acceleration_m_s2(a_vert),
            Quantity::acceleration_m_s2(a_lim),
            loc(
                &format!("a_vert={:.3} m/s² from mass/stiffness/damping (f₁={:.2} Hz < 8 Hz) vs {:.3} m/s².", a_vert, f1, a_lim),
                &format!("a_vert={:.3} m/s² aus Masse/Steifigkeit/Dämpfung (f₁={:.2} Hz < 8 Hz) gegen {:.3} m/s².", a_vert, f1, a_lim),
            ),
            DEPTH_LEVERS,
        ));
    }
    out
}

fn assess_fire(annex: AnnexChoice, params: &AnnexParams, m: &TimberMember, props: &TimberProperties, uls: Option<&LoadCombo>) -> Assessed<TimberMember> {
    let m_ed = uls.map(|u| u.m_ed_nm).unwrap_or_else(|| enumerate_combos(m).into_iter().filter(|c| c.kind == ComboKind::Uls).map(|c| c.m_ed_nm.abs()).fold(0.0_f64, f64::max));
    let deff = d_ef(m.fire_duration_s, params.beta_n(props.product));
    let (b_fi, h_fi) = residual_bh(m.b_m, m.h_m, deff);
    let w_fi = b_fi * h_fi.powi(2) / 6.0;
    let k_fi = params.k_fi(props.product);
    let k_mod_fi = params.k_mod_fi();
    let gamma_m_fi = params.gamma_m_fi();
    let m_rd_fi = k_mod_fi * k_fi * props.f_m_k / gamma_m_fi * w_fi;
    assessed(
        Head {
            id: format!("en1995.1-2.4.fire.{}", m.id),
            part: "EN 1995-1-2",
            clause: ClauseId::new("EN 1995-1-2", "§4.2", "4.2"),
            subject: member_subject(m, "hM"),
            title: loc("Fire residual bending", "Brand Restbiegetragfähigkeit"),
            annex,
        },
        Quantity::new(QuantityKind::Moment, m_ed),
        Quantity::new(QuantityKind::Moment, m_rd_fi.max(1e-9)),
        loc(
            &format!("d_ef={:.1} mm (β_n), k_fi={:.2}, k_mod,fi={:.2}, γ_M,fi={:.2}; M_Ed={:.1} kNm vs M_Rd,fi={:.1} kNm.", deff * 1000.0, k_fi, k_mod_fi, gamma_m_fi, m_ed / 1000.0, m_rd_fi / 1000.0),
            &format!("d_ef={:.1} mm (β_n), k_fi={:.2}, k_mod,fi={:.2}, γ_M,fi={:.2}; M_Ed={:.1} kNm gegen M_Rd,fi={:.1} kNm.", deff * 1000.0, k_fi, k_mod_fi, gamma_m_fi, m_ed / 1000.0, m_rd_fi / 1000.0),
        ),
        SECTION_LEVERS,
    )
}
