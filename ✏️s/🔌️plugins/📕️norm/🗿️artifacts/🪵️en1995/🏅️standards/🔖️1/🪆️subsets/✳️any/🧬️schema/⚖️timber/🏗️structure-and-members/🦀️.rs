//! 🏗️ Structure and member assessments of the EN 1995 timber checks: the integrity checks, the structure entry point and the member, floor and fire assessments. See [`super`].

use super::*;

/// 🏗 Evaluate all members and connections for the chosen annex.
fn push_duplicate_ids(report: &mut CheckReport, annex: AnnexChoice, table: &str, ids: &[String], path_for: &dyn Fn(&str) -> String) {
    push_duplicate_ids_in(report, annex, table, ids, "", path_for);
}

/// 🆔️ Duplicate ids of the rows nested in the row `owner` (a case's numbers or texts): the verdict is filed under `owner` and its id carries `owner`, so two cases may repeat the same key.
fn push_duplicate_ids_in(report: &mut CheckReport, annex: AnnexChoice, table: &str, ids: &[String], owner: &str, path_for: &dyn Fn(&str) -> String) {
    let (entity, scope) = (|id: &str| if owner.is_empty() { id.to_string() } else { owner.to_string() }, if owner.is_empty() { String::new() } else { format!("{owner}.") });
    let mut counts = std::collections::BTreeMap::<String, usize>::new();
    for id in ids {
        *counts.entry(id.clone()).or_insert(0) += 1;
    }
    for (id, count) in counts {
        if count < 2 {
            continue;
        }
        let path = path_for(&id);
        let subject = SubjectRef::new(entity(&id), &path, loc(&format!("Duplicate {table} id"), &format!("Doppelte {table}-Id")));
        let options: Vec<String> = ids.iter().filter(|x| x.as_str() != id).cloned().collect();
        let free = if options.is_empty() { vec![format!("{id}-unique")] } else { options };
        report.push(
            CheckResult::assess(
                format!("en1995.integrity.duplicate.{table}.{scope}{id}"),
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

fn push_referential_integrity(report: &mut CheckReport, annex: AnnexChoice, members: &[TimberMember], connections: &[TimberConnection], placements: &[Placement], tapers: &[Taper], cutouts: &[Cutout], fasteners: &[Fastener], panels: &[Panel], floors: &[Floor], diaphragms: &[Diaphragm], trusses: &[Truss], bracings: &[Bracing], decks: &[Deck]) {
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
    push_duplicate_ids(report, annex, "placements", &placements.iter().map(|p| p.id.clone()).collect::<Vec<_>>(), &|id| format!("placements[id={id}].id"));
    push_duplicate_ids(report, annex, "tapers", &tapers.iter().map(|p| p.id.clone()).collect::<Vec<_>>(), &|id| format!("tapers[id={id}].id"));
    push_duplicate_ids(report, annex, "cutouts", &cutouts.iter().map(|p| p.id.clone()).collect::<Vec<_>>(), &|id| format!("cutouts[id={id}].id"));
    push_duplicate_ids(report, annex, "fasteners", &fasteners.iter().map(|p| p.id.clone()).collect::<Vec<_>>(), &|id| format!("fasteners[id={id}].id"));
    push_duplicate_ids(report, annex, "panels", &panels.iter().map(|p| p.id.clone()).collect::<Vec<_>>(), &|id| format!("panels[id={id}].id"));
    push_duplicate_ids(report, annex, "floors", &floors.iter().map(|p| p.id.clone()).collect::<Vec<_>>(), &|id| format!("floors[id={id}].id"));
    push_duplicate_ids(report, annex, "diaphragms", &diaphragms.iter().map(|p| p.id.clone()).collect::<Vec<_>>(), &|id| format!("diaphragms[id={id}].id"));
    push_duplicate_ids(report, annex, "trusses", &trusses.iter().map(|p| p.id.clone()).collect::<Vec<_>>(), &|id| format!("trusses[id={id}].id"));
    push_duplicate_ids(report, annex, "bracings", &bracings.iter().map(|p| p.id.clone()).collect::<Vec<_>>(), &|id| format!("bracings[id={id}].id"));
    push_duplicate_ids(report, annex, "decks", &decks.iter().map(|p| p.id.clone()).collect::<Vec<_>>(), &|id| format!("decks[id={id}].id"));
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

pub fn evaluate_structure(document: &En1995Snapshot) -> CheckReport {
    evaluate_scoped(document, Scope::All)
}

/// 🎯️ The evaluation restricted to `scope` (global integrity and verdicts, or one member, connection or catalogue-case row); the ordered concatenation of all scopes is [`evaluate_structure`].
pub fn evaluate_scoped(document: &En1995Snapshot, scope: Scope<'_>) -> CheckReport {
    let annex = document.annex;
    let (members, connections) = (&document.members, &document.connections);
    let mut report = CheckReport::default();
    let params = AnnexParams::new(annex, document.reliability_class).with_uls_equation(document.uls_equation);
    if scope.globals() {
        push_referential_integrity(&mut report, annex, members, connections, &document.placements, &document.tapers, &document.cutouts, &document.fasteners, &document.panels, &document.floors, &document.diaphragms, &document.trusses, &document.bracings, &document.decks);
        push_duplicate_ids(&mut report, annex, "cases", &document.cases.iter().map(|c| c.id.clone()).collect::<Vec<_>>(), &|id| format!("cases[id={id}].id"));
        for c in &document.cases {
            let cid = &c.id;
            push_duplicate_ids_in(&mut report, annex, "caseNumbers", &c.numbers.iter().map(|n| n.id.clone()).collect::<Vec<_>>(), cid, &|id| format!("cases[id={cid}].numbers[id={id}].id"));
            push_duplicate_ids_in(&mut report, annex, "caseTexts", &c.texts.iter().map(|t| t.id.clone()).collect::<Vec<_>>(), cid, &|id| format!("cases[id={cid}].texts[id={id}].id"));
        }
        if members.is_empty() && document.cases.is_empty() {
            report.push(
                CheckResult::assess(
                    "en1995.subject.empty",
                    "EN 1995-1-1",
                    ClauseId::new("EN 1995-1-1", "§1", "1"),
                    SubjectRef::whole(loc("Timber structure", "Holztragwerk")),
                    loc("Subject completeness", "Vollständigkeit des Gegenstands"),
                )
                .not_applicable(loc("No timber members or catalogue cases defined.", "Keine Holzbauteile oder Katalogfälle definiert."))
                .build(),
            );
        }
    }
    for (index, m) in members.iter().enumerate().filter(|(index, _)| scope.admits("members", *index)) {
        report.extend(evaluate_member(annex, &params, m));
    }
    for (index, c) in connections.iter().enumerate().filter(|(index, _)| scope.admits("connections", *index)) {
        report.extend(evaluate_connection(annex, &params, c));
    }
    for (index, c) in document.cases.iter().enumerate().filter(|(index, _)| scope.admits("cases", *index)) {
        report.extend(evaluate_case_row(annex, c));
    }
    for table in ENTITY_TABLES {
        for index in (0..entity_ids(document, table).len()).filter(|index| scope.admits(table, *index)) {
            report.extend(evaluate_entity(document, table, index));
        }
    }
    report
}

/// 📇️ The catalogue verification of one document case through the shared design-case bridge: input issues with remedies, then the `T-nn` checks on the numeric and text rows (N, mm, N/mm², degrees) with a sizing remedy on failure.
pub fn evaluate_case_row(annex: AnnexChoice, c: &CaseRow) -> Vec<CheckResult> {
    evaluate_design_case(&semio_s_plugin_norm_compliance::design_case::DesignCase::from(c), annex)
}

fn evaluate_member(annex: AnnexChoice, params: &AnnexParams, m: &TimberMember) -> Vec<CheckResult> {
    let Some(props) = properties_for_class(m.strength_class.id()) else {
        return vec![CheckResult::assess(
            format!("en1995.material.unknown.{}", m.id),
            "EN 1995-1-1",
            ClauseId::new("EN 1995-1-1", "§3", "3.2"),
            member_subject(m, "strengthClass"),
            loc("Strength class", "Festigkeitsklasse"),
        )
        .status(semio_s_plugin_norm_compliance::CheckStatus::Fail)
        .explanation(loc(&format!("Unknown strength class '{}'.", m.strength_class.id()), &format!("Unbekannte Festigkeitsklasse '{}'.", m.strength_class.id())))
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

struct Worst {
    demand: f64,
    capacity: f64,
    text: String,
    combo: LoadCombo,
}

fn scaled(c: &LoadCombo, profile: &Profile, s: f64) -> LoadCombo {
    LoadCombo { m_ed_nm: c.m_ed_nm * profile.moment(s), v_ed_n: c.v_ed_n * profile.shear(s), ..c.clone() }
}

fn along_member(m: &TimberMember, check: Assessed<TimberMember>, w: &Worst, kind: QuantityKind, f: &dyn Fn(&LoadCombo) -> (f64, f64, String)) -> Assessed<TimberMember> {
    match Profile::of(m, w.combo.q_ed_line, w.combo.f_ed_point) {
        Some(profile) => check.along(profile.peaks(), kind, |s| {
            let (demand, capacity, _) = f(&scaled(&w.combo, &profile, s));
            (demand, capacity)
        }),
        None => check,
    }
}

fn along_deflection(m: &TimberMember, check: Assessed<TimberMember>, limit: f64, f: impl Fn(f64) -> f64) -> Assessed<TimberMember> {
    match Profile::of(m, 1.0, 0.0) {
        Some(profile) => check.along(profile.peaks(), QuantityKind::Length, |s| (f(s), limit)),
        None => check,
    }
}

fn worst(uls: &[&LoadCombo], f: &dyn Fn(&LoadCombo) -> (f64, f64, String)) -> Option<Worst> {
    uls.iter()
        .map(|c| {
            let (demand, capacity, text) = f(*c);
            Worst { demand, capacity: capacity.max(1e-9), text: format!("{text} [{}]", c.id), combo: (*c).clone() }
        })
        .max_by(|a, b| (a.demand / a.capacity).partial_cmp(&(b.demand / b.capacity)).unwrap_or(std::cmp::Ordering::Equal))
}

fn perm_loads(m: &TimberMember) -> (f64, f64) {
    m.actions.iter().filter(|a| action_kind_family(a.kind) == "permanent").fold((0.0, 0.0), |s, a| (s.0 + a.q_line_n_per_m, s.1 + a.f_point_n))
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
    let combos = enumerate_combos(m, params);
    let uls: Vec<&LoadCombo> = combos.iter().filter(|c| c.kind == ComboKind::Uls).collect();
    if uls.is_empty() {
        return out;
    }
    let head = |kind: &str, clause: &str, leaf: &str, en: &str, de: &str| Head {
        id: format!("en1995.{clause}.{kind}.{}", m.id),
        part: "EN 1995-1-1",
        clause: ClauseId::new("EN 1995-1-1", &format!("§{clause}"), clause),
        subject: member_subject(m, leaf),
        title: loc(en, de),
        annex,
    };
    let row = strength_row(m.strength_class.id());
    let hardwood = row.map(|r| r.hardwood).unwrap_or(false);
    let sc = parse_service_class(m.service_class);
    let gamma = params.gamma_m(props.product);
    let a = area_m2(m);
    let wsec = w_m3(m);
    let (h_mm, b_mm) = (m.h_m * 1000.0, m.b_m * 1000.0);
    let kh_b = size_factor_kh(props.product, props.rho_k, h_mm, 0.12);
    let kh_t = size_factor_kh(props.product, props.rho_k, if props.product == TimberProduct::Solid { h_mm.max(b_mm) } else { b_mm }, 0.12);
    let m_crit = effective_m_crit_nm(m, props.e_0_05);
    let kcrit = kcrit_factor(lambda_rel_m(wsec, props.f_m_k, m_crit));
    let km = 0.7;
    let (iy, iz) = ((i_m4(m) / a.max(1e-12)).sqrt(), (m.h_m * m.b_m.powi(3) / 12.0 / a.max(1e-12)).sqrt());
    let beta_c = if props.product == TimberProduct::Solid { 0.2 } else { 0.1 };
    let rel_y = lambda_rel_compression(m.buckling_length_y_m / iy.max(1e-12), props.f_c_0_k, props.e_0_05);
    let rel_z = lambda_rel_compression(m.buckling_length_z_m / iz.max(1e-12), props.f_c_0_k, props.e_0_05);
    let (kc_y, kc_z) = (kc_factor(rel_y, beta_c), kc_factor(rel_z, beta_c));
    let mpa = |x: f64| x / 1e6;

    let bending = |c: &LoadCombo| {
        let fm = k_mod(sc, c.duration) * kh_b * kcrit * props.f_m_k / gamma;
        let s = c.m_ed_nm.abs() / wsec;
        (s, fm, format!("σ_m,d={:.3} MPa vs k_crit·f_m,d={:.3} MPa (k_h={kh_b:.3}, k_crit={kcrit:.3})", mpa(s), mpa(fm)))
    };
    if let Some(w) = worst(&uls, &bending) {
        let check = assessed(head("bending", "6.1.6", "actions", "Bending stress", "Biegespannung"), Quantity::new(QuantityKind::Stress, w.demand), Quantity::new(QuantityKind::Stress, w.capacity), loc(&w.text, &w.text), BENDING_LEVERS);
        out.push(along_member(m, check, &w, QuantityKind::Stress, &bending));
    }

    let kcr = params.k_cr(props.product, props.f_v_k);
    let h_ef = (m.h_m - m.notch_depth_m).max(1e-6);
    let alpha_n = h_ef / m.h_m;
    let kv = notch_kv(h_mm, alpha_n, m.notch_distance_m * 1000.0, 0.0, if props.product == TimberProduct::Glulam { 6.5 } else if props.product == TimberProduct::Lvl { 4.5 } else { 5.0 }, m.notch_depth_m > 0.0);
    let shear = |c: &LoadCombo| {
        let tau = 1.5 * c.v_ed_n.abs() / (kcr * m.b_m * h_ef);
        let fv = kv * k_mod(sc, c.duration) * props.f_v_k / gamma;
        (tau, fv, format!("τ_d={:.3} MPa vs k_v·f_v,d={:.3} MPa (k_cr={kcr:.3}, k_v={kv:.3}, h_ef={:.0} mm)", mpa(tau), mpa(fv), h_ef * 1000.0))
    };
    if let Some(w) = worst(&uls, &shear) {
        let check = assessed(head("shear", "6.1.7", "actions", "Shear stress", "Schubspannung"), Quantity::new(QuantityKind::Stress, w.demand), Quantity::new(QuantityKind::Stress, w.capacity), loc(&w.text, &w.text), SECTION_LEVERS);
        out.push(along_member(m, check, &w, QuantityKind::Stress, &shear));
    }

    if uls.iter().any(|c| c.n_t_ed_n > 0.0) {
        let tension = |c: &LoadCombo| {
            let ft = k_mod(sc, c.duration) * kh_t * props.f_t_0_k / gamma;
            let fm = k_mod(sc, c.duration) * kh_b * props.f_m_k / gamma;
            let (st, sm) = (c.n_t_ed_n / a, c.m_ed_nm.abs() / wsec);
            (st / ft + sm / fm, 1.0, format!("σ_t/f_t,0,d + σ_m/f_m,d = {:.3}/{:.3} + {:.3}/{:.3} (6.17)", mpa(st), mpa(ft), mpa(sm), mpa(fm)))
        };
        if let Some(w) = worst(&uls, &tension) {
            let check = assessed(head("tension", "6.2.3", "actions", "Tension and bending", "Zug und Biegung"), Quantity::new(QuantityKind::Dimensionless, w.demand), Quantity::new(QuantityKind::Dimensionless, 1.0), loc(&w.text, &w.text), WIDTH_LEVERS);
            out.push(along_member(m, check, &w, QuantityKind::Dimensionless, &tension));
        }
    }

    if uls.iter().any(|c| c.n_ed_n > 0.0) {
        if let Some(w) = worst(&uls, &|c| {
            let fc = k_mod(sc, c.duration) * props.f_c_0_k / gamma;
            let sc_d = c.n_ed_n / a;
            let kc = kc_y.min(kc_z);
            (sc_d, kc * fc, format!("σ_c,0,d={:.3} MPa vs k_c·f_c,0,d={:.3} MPa (k_c,y={kc_y:.3}, k_c,z={kc_z:.3})", mpa(sc_d), mpa(kc * fc)))
        }) {
            out.push(assessed(head("compression", "6.3.2", "actions", "Compression with buckling", "Druck mit Knicken"), Quantity::new(QuantityKind::Stress, w.demand), Quantity::new(QuantityKind::Stress, w.capacity), loc(&w.text, &w.text), SECTION_LEVERS));
        }
        let combined = |c: &LoadCombo| {
            let km_d = k_mod(sc, c.duration);
            let (fc, fm) = (km_d * props.f_c_0_k / gamma, km_d * kh_b * props.f_m_k / gamma);
            let (s_c, s_m) = (c.n_ed_n / a, c.m_ed_nm.abs() / wsec);
            let u = if rel_y <= 0.3 && rel_z <= 0.3 { (s_c / fc).powi(2) + s_m / fm } else { (s_c / (kc_y * fc) + s_m / fm).max(s_c / (kc_z * fc) + km * s_m / fm) };
            let u_ltb = (s_m / (kcrit * fm)).powi(2) + s_c / (kc_z * fc);
            let u = u.max(u_ltb);
            (u, 1.0, format!("combined compression and bending (6.23/6.24/6.35, k_c,y={kc_y:.3}, k_c,z={kc_z:.3}, k_crit={kcrit:.3}) = {u:.3}"))
        };
        if let Some(w) = worst(&uls, &combined) {
            let check = assessed(head("combined", "6.3.2.c", "actions", "Combined compression and bending", "Kombinierter Druck und Biegung"), Quantity::new(QuantityKind::Dimensionless, w.demand), Quantity::new(QuantityKind::Dimensionless, 1.0), loc(&w.text, &w.text), SECTION_LEVERS);
            out.push(along_member(m, check, &w, QuantityKind::Dimensionless, &combined));
        }
    }

    if uls.iter().any(|c| c.f_c90_ed_n > 0.0) && m.bearing_length_m > 0.0 {
        let l = m.bearing_length_m * 1000.0;
        let overhang = ((m.support_length_m - m.bearing_length_m).max(0.0)) * 1000.0;
        let lef = bearing_length_ef(l, overhang, 1e9, 1e9);
        let kc90 = k_c90_factor(props.product, hardwood, false, l, 1e9, h_mm);
        if let Some(w) = worst(&uls, &|c| {
            let f90 = kc90 * k_mod(sc, c.duration) * props.f_c_90_k / gamma;
            let s90 = c.f_c90_ed_n / (m.b_m * lef / 1000.0);
            (s90, f90, format!("σ_c,90,d={:.3} MPa vs k_c,90·f_c,90,d={:.3} MPa (l_ef={lef:.0} mm, k_c,90={kc90:.2})", mpa(s90), mpa(f90)))
        }) {
            out.push(assessed(head("c90", "6.1.5", "bearingLengthM", "Compression perpendicular", "Druck quer zur Faser"), Quantity::new(QuantityKind::Stress, w.demand), Quantity::new(QuantityKind::Stress, w.capacity), loc(&w.text, &w.text), BEARING_LEVERS));
        }
    }

    if annex == AnnexChoice::De && m.notch_depth_m > 0.0 {
        out.push(assessed(
            head("notch-na", "6.5.2", "notchDepthM", "Notch depth (NA-DE)", "Ausklinkungstiefe (NA-DE)"),
            Quantity::new(QuantityKind::Dimensionless, 0.5),
            Quantity::new(QuantityKind::Dimensionless, alpha_n),
            loc(&format!("h_ef/h={alpha_n:.2} ≥ 0.5 required; x/h={:.2} ≤ 0.4 recommended.", m.notch_distance_m / m.h_m), &format!("h_ef/h={alpha_n:.2} ≥ 0,5 erforderlich; x/h={:.2} ≤ 0,4 empfohlen.", m.notch_distance_m / m.h_m)),
            DEPTH_LEVERS,
        ));
    }

    let sls_char = combos
        .iter()
        .filter(|c| c.kind == ComboKind::SlsCharacteristic)
        .max_by(|a, b| a.q_ed_line.abs().partial_cmp(&b.q_ed_line.abs()).unwrap_or(std::cmp::Ordering::Equal));
    if let Some(sls) = sls_char {
        if m.span_m > 0.0 && (sls.q_ed_line.abs() > 0.0 || sls.f_ed_point.abs() > 0.0) {
            let (qg, fg) = perm_loads(m);
            let w_g = deflection_m(m, props, qg, fg);
            let w_tot = deflection_m(m, props, sls.q_ed_line, sls.f_ed_point);
            let w_q = (w_tot - w_g).max(0.0);
            let kdef = k_def(sc, props.product);
            let w_fin = w_g * (1.0 + kdef) + w_q * (1.0 + sls.psi2 * kdef);
            let leff = if m.support == SupportType::Cantilever { 2.0 * m.span_m } else { m.span_m };
            let (lim_inst, lim_fin) = (leff / params.w_inst_limit_divisor(), leff / params.w_fin_limit_divisor());
            let w_g_at = |s: f64| deflection_at(m, props, qg, fg, s);
            let w_q_at = |s: f64| (deflection_at(m, props, sls.q_ed_line, sls.f_ed_point, s) - w_g_at(s)).max(0.0);
            let w_inst = assessed(
                head("winst", "7.2", "actions", "Instantaneous deflection (variable part)", "Sofortverformung (veränderlicher Anteil)"),
                Quantity::length_m(w_q),
                Quantity::length_m(lim_inst),
                loc(&format!("w_inst,Q={:.1} mm vs L/{:.0}={:.1} mm.", w_q * 1000.0, params.w_inst_limit_divisor(), lim_inst * 1000.0), &format!("w_inst,Q={:.1} mm gegen L/{:.0}={:.1} mm.", w_q * 1000.0, params.w_inst_limit_divisor(), lim_inst * 1000.0)),
                DEPTH_LEVERS,
            );
            out.push(along_deflection(m, w_inst, lim_inst, w_q_at));
            let text = format!("w_fin=w_G(1+k_def)+w_Q(1+ψ₂k_def)={:.1} mm (k_def={kdef:.2}, ψ₂={:.2}) vs L/{:.0}={:.1} mm.", w_fin * 1000.0, sls.psi2, params.w_fin_limit_divisor(), lim_fin * 1000.0);
            let w_fin_check = assessed(head("wfin", "7.2", "actions", "Final deflection", "Endverformung"), Quantity::length_m(w_fin), Quantity::length_m(lim_fin), loc(&text, &text), DEPTH_LEVERS);
            out.push(along_deflection(m, w_fin_check, lim_fin, |s| w_g_at(s) * (1.0 + kdef) + w_q_at(s) * (1.0 + sls.psi2 * kdef)));
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
            let text = format!("SLS frequent (ψ₁): w_freq={:.1} mm vs L/{:.0}={:.1} mm (combo {}).", w_freq * 1000.0, params.w_inst_limit_divisor(), lim_freq * 1000.0, freq.id);
            let w_freq_check = assessed(head("wfreq", "7.2", "actions", "Frequent deflection", "Häufige Durchbiegung"), Quantity::length_m(w_freq), Quantity::length_m(lim_freq), loc(&text, &text), DEPTH_LEVERS);
            out.push(along_deflection(m, w_freq_check, lim_freq, |s| deflection_at(m, props, freq.q_ed_line, freq.f_ed_point, s)));
        }
    }

    if m.role == MemberRole::Floor {
        out.extend(assess_floor(annex, params, m, props));
    }
    if m.fire_duration_s > 0.0 {
        let governing = uls.iter().copied().max_by(|a, b| a.m_ed_nm.abs().partial_cmp(&b.m_ed_nm.abs()).unwrap_or(std::cmp::Ordering::Equal));
        out.push(assess_fire(annex, params, m, props, governing));
    }
    out
}

fn assess_floor(annex: AnnexChoice, _params: &AnnexParams, m: &TimberMember, props: &TimberProperties) -> Vec<Assessed<TimberMember>> {
    let l = m.span_m.max(1e-6);
    let strip = if m.mass_kg_per_m > 0.0 && m.mass_kg_per_m2 > 0.0 { m.mass_kg_per_m / m.mass_kg_per_m2 } else { m.b_m.max(0.05) };
    let ei_l = props.e_0_mean * i_m4(m) / strip;
    let ei_b = props.e_0_mean * 0.022_f64.powi(3) / 12.0;
    let mass = if m.mass_kg_per_m2 > 0.0 { m.mass_kg_per_m2 } else { (if m.mass_kg_per_m > 0.0 { m.mass_kg_per_m } else { props.rho_k * area_m2(m) }) / strip };
    let f1 = floor_f1(l, ei_l, mass);
    let w_unit = unit_point_deflection(1000.0, l, l, ei_l, ei_b);
    let de = annex == AnnexChoice::De;
    let (f1_lim, a_lim) = (if de { 6.0 } else { 8.0 }, if de { 0.5 } else { 1.5 });
    let zeta = if m.damping_xi > 0.0 { m.damping_xi } else { 0.01 };
    let head = |kind: &str, leaf: &str, en: &str, de_t: &str| Head {
        id: format!("en1995.7.3.{kind}.{}", m.id),
        part: "EN 1995-1-1",
        clause: ClauseId::new("EN 1995-1-1", "§7.3", "7.3"),
        subject: member_subject(m, leaf),
        title: loc(en, de_t),
        annex,
    };
    let note = format!("(EI)_ℓ={ei_l:.0} N·m²/m from strip {strip:.2} m, (EI)_b={ei_b:.0} N·m²/m for an assumed 22 mm deck, m={mass:.0} kg/m², square panel.");
    let mut out = vec![
        assessed(
            head("f1", "spanM", "Floor fundamental frequency", "Eigenfrequenz Decke"),
            Quantity::new(QuantityKind::Dimensionless, f1_lim),
            Quantity::new(QuantityKind::Dimensionless, f1.max(1e-9)),
            loc(&format!("f₁={f1:.2} Hz vs {f1_lim:.0} Hz; {note}"), &format!("f₁={f1:.2} Hz gegen {f1_lim:.0} Hz; {note}")),
            DEPTH_LEVERS,
        ),
        assessed(
            head("stiffness", "spanM", "Floor unit-load stiffness", "Deckensteifigkeit unter 1 kN"),
            Quantity::length_m(w_unit / 1000.0),
            Quantity::length_m(a_lim / 1000.0),
            loc(&format!("w(1 kN)={w_unit:.2} mm vs a={a_lim:.1} mm/kN (7.3.3); {note}"), &format!("w(1 kN)={w_unit:.2} mm gegen a={a_lim:.1} mm/kN (7.3.3); {note}")),
            DEPTH_LEVERS,
        ),
    ];
    if f1 >= 8.0 {
        let (v, n40) = velocity_response(f1, l, l, mass, ei_l, ei_b);
        let lim = 100.0_f64.powf(f1 * zeta - 1.0);
        out.push(assessed(
            head("velocity", "massKgPerM2", "Floor unit-impulse velocity", "Einheitsimpuls-Geschwindigkeit Decke"),
            Quantity::new(QuantityKind::Dimensionless, v * 1e3),
            Quantity::new(QuantityKind::Dimensionless, lim * 1e3),
            loc(&format!("v={v:.5} m/(N·s²) (n₄₀={n40:.2}) vs b^(f₁ζ−1)={lim:.5} (b=100, ζ={zeta:.3}); {note}"), &format!("v={v:.5} m/(N·s²) (n₄₀={n40:.2}) gegen b^(f₁ζ−1)={lim:.5} (b=100, ζ={zeta:.3}); {note}")),
            MASS_LEVERS,
        ));
    }
    out
}

fn assess_fire(annex: AnnexChoice, params: &AnnexParams, m: &TimberMember, props: &TimberProperties, uls: Option<&LoadCombo>) -> Assessed<TimberMember> {
    let m_ed = uls.map(|u| u.m_ed_nm.abs()).unwrap_or_else(|| enumerate_combos(m, params).into_iter().filter(|c| c.kind == ComboKind::Uls).map(|c| c.m_ed_nm.abs()).fold(0.0_f64, f64::max));
    let t_min = m.fire_duration_s / 60.0;
    let deff = d_ef_mm(params.beta_n(props.product) * 1e3 * 60.0 * t_min, t_min) / 1000.0;
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
            &format!("d_ef={:.1} mm (β_n, k₀), k_fi={:.2}, k_mod,fi={:.2}, γ_M,fi={:.2}; M_Ed={:.1} kNm vs M_Rd,fi={:.1} kNm.", deff * 1000.0, k_fi, k_mod_fi, gamma_m_fi, m_ed / 1000.0, m_rd_fi / 1000.0),
            &format!("d_ef={:.1} mm (β_n, k₀), k_fi={:.2}, k_mod,fi={:.2}, γ_M,fi={:.2}; M_Ed={:.1} kNm gegen M_Rd,fi={:.1} kNm.", deff * 1000.0, k_fi, k_mod_fi, gamma_m_fi, m_ed / 1000.0, m_rd_fi / 1000.0),
        ),
        SECTION_LEVERS,
    )
    .timed(|s| {
        let t = s * t_min;
        let d = d_ef_mm(params.beta_n(props.product) * 1e3 * 60.0 * t, t) / 1000.0;
        let (b, h) = residual_bh(m.b_m, m.h_m, d);
        (m_ed, k_mod_fi * k_fi * props.f_m_k / gamma_m_fi * b * h * h / 6.0, d)
    })
}

/// 📚️ The demo document with a design case for every catalogue entry: the compliant beam plus [`catalogue_demo_cases`].
pub fn catalogue_demo_document() -> En1995Snapshot {
    En1995Snapshot { cases: catalogue_demo_cases(), ..En1995Snapshot::compliant_building_beam() }
}

/// 🔩️ The compliant beam with a second connection, a double-shear steel-to-timber dowel connection with a centre steel plate (EN 1995-1-1 8.2.3, catalogue entry T-73), next to the single-shear timber-to-timber bolt group of EN 1995-1-1 8.2.2 (T-71, T-72), so that both Johansen clauses run in one document.
pub fn steel_connection_demo_document() -> En1995Snapshot {
    let mut d = En1995Snapshot::compliant_building_beam();
    let mut steel = d.connections[0].clone();
    steel.id = "conn-C3".into();
    steel.label_en = "Dowel group with centre steel plate".into();
    steel.label_de = "Stabdübelgruppe mit mittlerem Stahlblech".into();
    steel.fastener_type = semio_s_artifact_norm_en1995_model::FastenerType::Dowel;
    steel.steel_plate = true;
    steel.steel_plate_thickness_m = 0.008;
    steel.shear_planes = 2;
    steel.t1_m = 0.08;
    d.connections.push(steel);
    d
}

/// 🧱️ The compliant beam with one row of every typed entity collection (two tapers, a notch and a hole, a fastener pattern, a CLT panel, a floor, a wall diaphragm, a truss, a bracing and a stress-laminated deck), all standing on the beam `beam-B1` and the connection `conn-C1`; the fixture of the entity checks, the live-unit laws and the scene.
pub fn entity_demo_document() -> En1995Snapshot {
    let mut d = En1995Snapshot::compliant_building_beam();
    let (member, connection) = ("beam-B1".to_string(), "conn-C1".to_string());
    d.tapers.push(Taper { id: "taper-1".into(), member_id: member.clone(), kind: TaperKind::DoubleTapered, taper_angle_rad: 0.1, apex_depth_m: 0.8, inner_radius_m: 15.0, lamella_thickness_m: 0.033, apex_angle_rad: 0.1, tension_side: TensionSide::Tension, rods: 0, rod_resistance_n: 0.0 });
    d.tapers.push(Taper { id: "taper-2".into(), member_id: member.clone(), kind: TaperKind::SingleTapered, taper_angle_rad: 0.08, apex_depth_m: 0.0, inner_radius_m: 0.0, lamella_thickness_m: 0.0, apex_angle_rad: 0.0, tension_side: TensionSide::Compression, rods: 0, rod_resistance_n: 0.0 });
    d.cutouts.push(Cutout { id: "cutout-1".into(), member_id: member.clone(), kind: CutoutKind::Notch, face: CutoutFace::Bottom, distance_m: 0.0, length_m: 0.15, depth_m: 0.06, centre_height_m: 0.0, slope: 0.0, rods: 0, rod_resistance_n: 0.0 });
    d.cutouts.push(Cutout { id: "cutout-2".into(), member_id: member.clone(), kind: CutoutKind::Hole, face: CutoutFace::Bottom, distance_m: 1.0, length_m: 0.05, depth_m: 0.0, centre_height_m: 0.2, slope: 0.0, rods: 0, rod_resistance_n: 0.0 });
    d.fasteners.push(Fastener { id: "fastener-1".into(), connection_id: connection, member_id: member.clone(), columns: 4, lines: 2, spacing_parallel_m: 0.08, spacing_perpendicular_m: 0.04, end_distance_m: 0.1, edge_distance_m: 0.04, offset_x_m: 0.1, offset_y_m: 0.05, load_angle_rad: 0.0, positions: Vec::new() });
    d.panels.push(Panel { id: "panel-1".into(), strength_class: semio_s_artifact_norm_en1995_model::StrengthClass::C24, layers: (1..=5).map(|k| PanelLayer { id: format!("layer-{k}"), thickness_m: 0.04, orientation: if k % 2 == 1 { LayerOrientation::Longitudinal } else { LayerOrientation::Transverse }, grade: "C24".into() }).collect(), span_m: 4.0, width_m: 1.0, load_pa: 5000.0, fire_duration_s: 3600.0 });
    d.diaphragms.push(Diaphragm { id: "diaphragm-1".into(), kind: DiaphragmKind::Wall, method: DiaphragmMethod::MethodA, panel_width_m: 1.2, wall_height_m: 2.4, fastener_spacing_m: 0.075, panels: 2, fastener_capacity_n: 3000.0, shear_ed_n: 3000.0, hold_down_n: 20000.0, diaphragm_force_n: 15000.0, diaphragm_length_m: 12.0, shear_resistance_n_per_m: 3000.0 });
    d.trusses.push(Truss { id: "truss-1".into(), kind: TrussKind::Warren, product: BracingProduct::Solid, span_m: 12.0, height_m: 1.5, bays: 6, chord_width_m: 0.1, chord_depth_m: 0.2, web_width_m: 0.08, web_depth_m: 0.12, chord_force_n: 20000.0, restraint_length_m: 6.0, bracing_capacity_n_per_m: 1500.0 });
    d.bracings.push(Bracing { id: "bracing-1".into(), kind: BracingKind::PlanBracing, member_id: member.clone(), stabilised_count: 4, spacing_m: 0.8, length_m: 6.0, brace_width_m: 0.06, brace_depth_m: 0.12, compression_force_n: 20000.0, resistance_n: 2000.0, stiffness_n_per_m: 500000.0 });
    d.decks.push(Deck { id: "deck-1".into(), lamination_product: LaminationProduct::Solid, bar_product: PrestressBarProduct::ThreadedBar, span_m: 8.0, deck_width_m: 4.0, deck_depth_m: 0.25, lamination_width_m: 0.05, bar_spacing_m: 0.8, bar_diameter_m: 0.032, plate_side_m: 0.3, initial_prestress_pa: 1.0e6, loss_relaxation: 0.15, loss_creep: 0.25, loss_moisture: 0.1, bar_resistance_pa: 700.0e6, shear_stress_pa: 0.1e6, compression_perpendicular_pa: 1.54e6, wheel_load_n: 100_000.0, wheel_contact_m: 0.4 });
    d.floors.push(Floor { id: "floor-1".into(), member_id: member, span_m: 4.0, width_m: 5.0, joist_spacing_m: 0.625, ei_l_n_m2_per_m: 1.0e6, ei_b_n_m2_per_m: 2.0e5, mass_kg_per_m2: 150.0, damping_ratio: 0.02, permanent_load_pa: 1000.0, imposed_load_pa: 2000.0, psi_2: 0.3 });
    d
}
