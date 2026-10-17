//! ⚖️ Part evaluators of the EN 1998 inference: the integrity checks and the bridge, assessment, silo, foundation and tower evaluations. See [`super`].

use super::*;

fn push_duplicate_ids(report: &mut CheckReport, annex: AnnexChoice, table: &str, ids: &[String], path_for: &dyn Fn(&str) -> String) {
    let mut counts = std::collections::BTreeMap::<String, usize>::new();
    for id in ids {
        *counts.entry(id.clone()).or_insert(0) += 1;
    }
    for (id, count) in counts {
        if count < 2 {
            continue;
        }
        let path = path_for(&id);
        let subject = SubjectRef::new(id.clone(), &path, lc(&format!("Duplicate {table} id"), &format!("Doppelte {table}-Id")));
        let options: Vec<String> = ids.iter().filter(|x| x.as_str() != id).cloned().collect();
        let free = if options.is_empty() { vec![format!("{id}-unique")] } else { options };
        report.push(
            CheckResult::assess(
                format!("en1998.integrity.duplicate.{table}.{id}"),
                "EN 1998 integrity",
                ClauseId::new("EN 1998", "§2", "id"),
                subject.clone(),
                lc(&format!("Unique {table} id"), &format!("Eindeutige {table}-Id")),
            )
            .annex(annex)
            .explanation(lc(
                &format!("Duplicate {table} id '{id}' appears {count} times; each entity id must be unique."),
                &format!("Doppelte {table}-Id '{id}' kommt {count}-mal vor; jede Entitäts-Id muss eindeutig sein."),
            ))
            .status(semio_s_plugin_norm_compliance::CheckStatus::Fail)
            .remedy(Remedy::one_of(
                subject,
                free,
                lc(
                    &format!("Rename the duplicated '{id}' entry to a free id."),
                    &format!("Den doppelten '{id}'-Eintrag auf eine freie Id umbenennen."),
                ),
            ))
            .build(),
        );
    }
}

pub(super) fn push_referential_integrity(report: &mut CheckReport, document: &En1998Snapshot, annex: AnnexChoice) {
    push_duplicate_ids(report, annex, "buildings", &document.buildings.iter().map(|b| b.id.clone()).collect::<Vec<_>>(), &|id| format!("buildings[id={id}].id"));
    push_duplicate_ids(report, annex, "bridges", &document.bridges.iter().map(|b| b.id.clone()).collect::<Vec<_>>(), &|id| format!("bridges[id={id}].id"));
    push_duplicate_ids(report, annex, "assessments", &document.assessments.iter().map(|a| a.id.clone()).collect::<Vec<_>>(), &|id| format!("assessments[id={id}].id"));
    push_duplicate_ids(report, annex, "silos", &document.silos.iter().map(|s| s.id.clone()).collect::<Vec<_>>(), &|id| format!("silos[id={id}].id"));
    push_duplicate_ids(report, annex, "tanks", &document.tanks.iter().map(|t| t.id.clone()).collect::<Vec<_>>(), &|id| format!("tanks[id={id}].id"));
    push_duplicate_ids(report, annex, "foundations", &document.foundations.iter().map(|f| f.id.clone()).collect::<Vec<_>>(), &|id| format!("foundations[id={id}].id"));
    push_duplicate_ids(report, annex, "retainingWalls", &document.retaining_walls.iter().map(|w| w.id.clone()).collect::<Vec<_>>(), &|id| format!("retainingWalls[id={id}].id"));
    push_duplicate_ids(report, annex, "towers", &document.towers.iter().map(|t| t.id.clone()).collect::<Vec<_>>(), &|id| format!("towers[id={id}].id"));

    for building in &document.buildings {
        let bid = &building.id;
        push_duplicate_ids(report, annex, "systems", &building.systems.iter().map(|s| s.id.clone()).collect::<Vec<_>>(), &|id| format!("buildings[id={bid}].systems[id={id}].id"));
        push_duplicate_ids(report, annex, "storeys", &building.storeys.iter().map(|s| s.id.clone()).collect::<Vec<_>>(), &|id| format!("buildings[id={bid}].storeys[id={id}].id"));
        push_duplicate_ids(report, annex, "members", &building.members.iter().map(|m| m.id.clone()).collect::<Vec<_>>(), &|id| format!("buildings[id={bid}].members[id={id}].id"));
        for storey in &building.storeys {
            let sid = &storey.id;
            push_duplicate_ids(report, annex, "variables", &storey.variables.iter().map(|v| v.id.clone()).collect::<Vec<_>>(), &|id| format!("buildings[id={bid}].storeys[id={sid}].variables[id={id}].id"));
        }
    }
    for bridge in &document.bridges {
        let brid = &bridge.id;
        push_duplicate_ids(report, annex, "variables", &bridge.variables.iter().map(|v| v.id.clone()).collect::<Vec<_>>(), &|id| format!("bridges[id={brid}].variables[id={id}].id"));
    }
    for tower in &document.towers {
        let tid = &tower.id;
        push_duplicate_ids(report, annex, "variables", &tower.variables.iter().map(|v| v.id.clone()).collect::<Vec<_>>(), &|id| format!("towers[id={tid}].variables[id={id}].id"));
    }
}

pub(super) fn evaluate_bridges(report: &mut CheckReport, document: &En1998Snapshot, annex_params: &AnnexParams, gamma_i: f64, a_g: f64, s: f64, tb: f64, tc: f64, td: f64) {
    let annex = annex_params.choice();
    if document.bridges.is_empty() {
        report.push(CheckResult::assess("en1998.2.na", "DIN EN 1998-2", ClauseId::new("EN 1998-2", "2", "5"), SubjectRef::whole(lc("Bridges", "Brücken")), lc("Bridge seismic checks", "Brücken-Erdbebennachweise")).not_applicable(lc("No bridges in the subject.", "Keine Brücken im Gegenstand.")).annex(annex).build());
        return;
    }
    for bridge in &document.bridges {
        let t1 = bridge.fundamental_period_s.max(0.05);
        let q_isol = part_2::isolation_reduction_factor(bridge.period_ratio);
        {
            let mut pr = CheckResult::assess(
                format!("en1998.2.{}.periodRatio", bridge.id),
                "DIN EN 1998-2",
                ClauseId::new("EN 1998-2", "2", "7.1"),
                SubjectRef::new(&bridge.id, format!("{}.periodRatio", id_sel("bridges", &bridge.id)), lc(&bridge.id, &bridge.id)),
                lc("Isolation period ratio T_isol/T_fixed", "Isolations-Periodenverhältnis T_isol/T_fixed"),
            )
            .utilization(q_dim(bridge.period_ratio), q_dim(3.0))
            .annex(annex)
            .explanation(lc(
                &format!("periodRatio={:.3}, q_isol={q_isol:.3}", bridge.period_ratio),
                &format!("Periodenverhältnis={:.3}, q_isol={q_isol:.3}", bridge.period_ratio),
            ));
            if bridge.period_ratio < 1.0 {
                pr = pr.status(semio_s_plugin_norm_compliance::CheckStatus::Fail).remedy(Remedy::at_least(
                    SubjectRef::new(&bridge.id, format!("{}.periodRatio", id_sel("bridges", &bridge.id)), lc("Period ratio", "Periodenverhältnis")),
                    q_dim(bridge.period_ratio),
                    q_dim(1.0),
                    lc("Increase isolation period ratio to at least 1.0.", "Isolations-Periodenverhältnis auf mindestens 1.0 erhöhen."),
                ));
            }
            report.push(pr.build());
        }
        let s_e = part_1::elastic_response_spectrum(a_g, s, tb, tc, td, t1);
        let s_d = part_1::design_spectrum_sd(s_e, gamma_i, q_isol, a_g);
        let mass = part_1::entity_seismic_mass_kg(bridge.permanent_gk_n, &bridge.variables, false, bridge.correlated_occupancy);
        let v = part_1::base_shear_n(s_d, mass, 1.0);
        let d_ed = part_2::bearing_displacement_m(s_d, t1);
        let path = id_sel("bridges", &bridge.id);
        let mut c = CheckResult::assess(format!("en1998.2.{}.shear", bridge.id), "DIN EN 1998-2", ClauseId::new("EN 1998-2", "2", "5.3"), SubjectRef::new(&bridge.id, format!("{path}.vRdN"), lc(&bridge.id, &bridge.id)), lc("Bridge base shear", "Brücken-Querkraft")).utilization(q_force(v), q_force(bridge.v_rd_n)).annex(annex).explanation(lc(
                &format!("T={t1:.3}s, periodRatio={:.2}, q_isol={q_isol:.2}, mass={mass:.0} kg, Ved={v:.0} N, Vrd={:.0} N", bridge.period_ratio, bridge.v_rd_n),
                &format!("T={t1:.3}s, Periodenverhältnis={:.2}, q_isol={q_isol:.2}, Masse={mass:.0} kg, V_Ed={v:.0} N, V_Rd={:.0} N", bridge.period_ratio, bridge.v_rd_n),
            ));
        if v > bridge.v_rd_n {
            c = c.remedy(Remedy::at_least(SubjectRef::new(&bridge.id, format!("{path}.vRdN"), lc("V_Rd", "V_Rd")), q_force(bridge.v_rd_n), q_force(v), lc(&format!("Increase bridge V_Rd to at least {v:.0} N."), &format!("Brücken-V_Rd auf mindestens {v:.0} N erhöhen."))));
        }
        report.push(c.build());
        let mut b = CheckResult::assess(format!("en1998.2.{}.bearing", bridge.id), "DIN EN 1998-2", ClauseId::new("EN 1998-2", "2", "7.5"), SubjectRef::new(&bridge.id, format!("{path}.bearingDRdM"), lc(&bridge.id, &bridge.id)), lc("Bearing displacement", "Lagerverschiebung")).utilization(q_len(d_ed), q_len(bridge.bearing_d_rd_m)).annex(annex).explanation(lc(&format!("ded={d_ed:.4} m from Sd(T), drd={:.4} m", bridge.bearing_d_rd_m), &format!("ded={d_ed:.4} m aus Sd(T), drd={:.4} m", bridge.bearing_d_rd_m)));
        if d_ed > bridge.bearing_d_rd_m {
            b = b.remedy(Remedy::at_least(SubjectRef::new(&bridge.id, format!("{path}.bearingDRdM"), lc("d_Rd", "d_Rd")), q_len(bridge.bearing_d_rd_m), q_len(d_ed), lc("Increase bearing displacement capacity.", "Lagerverschiebungskapazität erhöhen.")));
        }
        report.push(b.build());
    }
}

pub(super) fn evaluate_assessments(report: &mut CheckReport, document: &En1998Snapshot, annex_params: &AnnexParams, gamma_i: f64, a_g: f64, s: f64, tb: f64, tc: f64, td: f64) {
    let annex = annex_params.choice();
    if document.assessments.is_empty() {
        report.push(CheckResult::assess("en1998.3.na", "DIN EN 1998-3", ClauseId::new("EN 1998-3", "3", "2.3"), SubjectRef::whole(lc("Assessments", "Bestandsbewertung")), lc("Retrofit checks", "Ertüchtigungsnachweise")).not_applicable(lc("No retrofit assessments in the subject.", "Keine Bestandsbewertungen im Gegenstand.")).annex(annex).build());
        return;
    }
    for a in &document.assessments {
        let path = id_sel("assessments", &a.id);
        let building_ok = document.buildings.iter().any(|b| b.id == a.supported_building_id);
        {
            let mut href = CheckResult::assess(
                format!("en1998.3.{}.supportedBuilding", a.id),
                "DIN EN 1998-3",
                ClauseId::new("EN 1998-3", "3", "2"),
                SubjectRef::new(&a.id, format!("{path}.supportedBuildingId"), lc(&a.id, &a.id)),
                lc("Assessment supported building reference", "Bewertung — Bezug Gebäude"),
            )
            .utilization(q_dim(if building_ok { 0.5 } else { 1.5 }), q_dim(1.0))
            .annex(annex)
            .explanation(lc(
                &format!("supportedBuildingId={}{}", a.supported_building_id, if building_ok { " (resolved)" } else { " (dangling)" }),
                &format!("supportedBuildingId={}{}", a.supported_building_id, if building_ok { " (aufgelöst)" } else { " (hängend)" }),
            ));
            if !building_ok {
                let building_ids: Vec<String> = document.buildings.iter().map(|b| b.id.clone()).collect();
                let opts = if building_ids.is_empty() { vec![format!("{}-missing", a.supported_building_id)] } else { building_ids };
                href = href
                    .status(semio_s_plugin_norm_compliance::CheckStatus::Fail)
                    .remedy(Remedy::one_of(
                        SubjectRef::new(&a.id, format!("{path}.supportedBuildingId"), lc("Supported building id", "Gebäude-ID")),
                        opts,
                        lc("Set supportedBuildingId to one of the existing building ids.", "supportedBuildingId auf eine der vorhandenen Gebäude-Ids setzen."),
                    ));
            }
            report.push(href.build());
        }

        let ls = a.limit_state.to_ascii_lowercase();
        let ag_fac = part_3::limit_state_ag_factor(&ls);
        let cf_raw = part_3::confidence_factor(&a.knowledge_level);
        let cf = if part_3::limit_state_applies_cf(&ls) { cf_raw } else { 1.0 };
        let r_d = part_3::design_capacity_n(a.r_k_n, cf, a.gamma_el);
        let support_id = if a.supported_building_id.is_empty() {
            document.buildings.first().map(|b| b.id.clone()).unwrap_or_default()
        } else {
            a.supported_building_id.clone()
        };
        let e_d = building_base_shear_n(document, &support_id, annex_params, gamma_i, a_g * ag_fac, s, tb, tc, td);
        let path = id_sel("assessments", &a.id);
        let mut c = CheckResult::assess(format!("en1998.3.{}.capacity", a.id), "DIN EN 1998-3", ClauseId::new("EN 1998-3", "3", "2.1"), SubjectRef::new(&a.id, format!("{path}.rKN"), lc(&a.id, &a.id)), lc("Limit-state capacity check", "Grenzzustands-Tragfähigkeitsnachweis")).utilization(q_force(e_d), q_force(r_d)).annex(annex).explanation(lc(&format!("LS={}, ag×{ag_fac:.2}, CF={cf:.2}, Ed={e_d:.0} N from building {support_id}, Rd={r_d:.0} N", a.limit_state), &format!("GZ={}, ag×{ag_fac:.2}, CF={cf:.2}, Ed={e_d:.0} N aus Gebäude {support_id}, Rd={r_d:.0} N", a.limit_state)));
        if e_d > r_d {
            let req = e_d * cf * a.gamma_el;
            c = c.remedy(Remedy::at_least(SubjectRef::new(&a.id, format!("{path}.rKN"), lc("R_k", "R_k")), q_force(a.r_k_n), q_force(req), lc(&format!("Increase R_k to at least {req:.0} N (or lower limit-state demand)."), &format!("R_k auf mindestens {req:.0} N erhöhen (oder Grenzzustandslast senken)."))));
        }
        // Explicit limit-state selection check (reads limitState).
        let ls_ok = matches!(ls.as_str(), "nc" | "near_collapse" | "near-collapse" | "sd" | "significant_damage" | "significant-damage" | "dl" | "damage_limitation" | "damage-limitation");
        let mut ls_c = CheckResult::assess(format!("en1998.3.{}.limitState", a.id), "DIN EN 1998-3", ClauseId::new("EN 1998-3", "3", "2.1"), SubjectRef::new(&a.id, format!("{path}.limitState"), lc(&a.id, &a.id)), lc("EN 1998-3 limit state (NC/SD/DL)", "EN-1998-3-Grenzzustand (NC/SD/DL)")).utilization(q_dim(if ls_ok { 0.5 } else { 2.0 }), q_dim(1.0)).annex(annex).explanation(lc(
                &format!("limitState={}", a.limit_state),
                &format!("Grenzzustand={}", a.limit_state),
            ));
        if !ls_ok {
            ls_c = ls_c.remedy(Remedy::one_of(SubjectRef::new(&a.id, format!("{path}.limitState"), lc("Limit state", "Grenzzustand")), vec!["sd".into(), "nc".into(), "dl".into()], lc("Set limitState to nc, sd, or dl (Table 2.1).", "limitState auf nc, sd oder dl setzen (Tab. 2.1).")));
        }
        report.push(ls_c.build());
        report.push(c.build());
    }
}

pub(super) fn evaluate_silos_tanks(report: &mut CheckReport, document: &En1998Snapshot, annex_params: &AnnexParams, gamma_i: f64, a_g: f64, s: f64, tb: f64, tc: f64, td: f64) {
    let annex = annex_params.choice();
    if document.silos.is_empty() {
        report.push(CheckResult::assess("en1998.4.silo.na", "DIN EN 1998-4", ClauseId::new("EN 1998-4", "4", "3"), SubjectRef::whole(lc("Silos", "Silos")), lc("Silo checks", "Silo-Nachweise")).not_applicable(lc("No silos in the subject.", "Keine Silos im Gegenstand.")).annex(annex).build());
    }
    for silo in &document.silos {
        let hr = silo.height_m / silo.radius_m.max(1e-9);
        let q = part_4::silo_behaviour_factor(silo.q_nominal);
        let s_e = part_1::elastic_response_spectrum(a_g, s, tb, tc, td, 0.3);
        let s_d = part_1::design_spectrum_sd(s_e, gamma_i, q, a_g);
        let mass = part_1::filled_content_seismic_mass_kg(silo.permanent_gk_n, silo.content_qk_n, &silo.content_category, silo.filling_ratio);
        let v_i = part_1::base_shear_n(s_d, mass * part_4::impulsive_mass_ratio(hr), 1.0);
        let t_c = part_4::convective_period_s(silo.radius_m);
        let s_e_c = part_1::elastic_response_spectrum(a_g, s, tb, tc, td, t_c);
        let s_d_c = part_1::design_spectrum_sd(s_e_c, gamma_i, q, a_g);
        let v_c = part_1::base_shear_n(s_d_c, mass * part_4::convective_mass_ratio(hr), 1.0);
        let v = part_4::srss(v_i, v_c);
        let path = id_sel("silos", &silo.id);
        let mut c = CheckResult::assess(format!("en1998.4.{}.wall", silo.id), "DIN EN 1998-4", ClauseId::new("EN 1998-4", "4", "3.4"), SubjectRef::new(&silo.id, format!("{path}.nRdN"), lc(&silo.id, &silo.id)), lc("Silo wall / base shear", "Silowand / Fußquerkraft")).utilization(q_force(v), q_force(silo.n_rd_n)).annex(annex).explanation(lc(
                &format!("q={:.2}, V={v:.0} N, Nrd={:.0} N", silo.q_nominal, silo.n_rd_n),
                &format!("q={:.2}, V={v:.0} N, N_Rd={:.0} N", silo.q_nominal, silo.n_rd_n),
            ));
        if v > silo.n_rd_n {
            c = c.remedy(Remedy::at_least(SubjectRef::new(&silo.id, format!("{path}.nRdN"), lc("N_Rd", "N_Rd")), q_force(silo.n_rd_n), q_force(v), lc("Increase silo wall resistance.", "Silowandtragfähigkeit erhöhen.")));
        }
        report.push(c.build());
        let mut a = CheckResult::assess(format!("en1998.4.{}.anchor", silo.id), "DIN EN 1998-4", ClauseId::new("EN 1998-4", "4", "3.5"), SubjectRef::new(&silo.id, format!("{path}.vRdN"), lc(&silo.id, &silo.id)), lc("Silo anchorage", "Silo-Verankerung")).utilization(q_force(v), q_force(silo.v_rd_n)).annex(annex).explanation(lc(&format!("Ved={v:.0} from impulsive/convective, Vrd={:.0}", silo.v_rd_n), &format!("Ved={v:.0} aus Impuls/Konvektion, Vrd={:.0}", silo.v_rd_n)));
        if v > silo.v_rd_n {
            a = a.remedy(Remedy::at_least(SubjectRef::new(&silo.id, format!("{path}.vRdN"), lc("V_Rd", "V_Rd")), q_force(silo.v_rd_n), q_force(v), lc("Increase anchorage resistance.", "Verankerungstragfähigkeit erhöhen.")));
        }
        report.push(a.build());
    }
    if document.tanks.is_empty() {
        report.push(CheckResult::assess("en1998.4.tank.na", "DIN EN 1998-4", ClauseId::new("EN 1998-4", "4", "4"), SubjectRef::whole(lc("Tanks", "Behälter")), lc("Tank checks", "Tank-Nachweise")).not_applicable(lc("No tanks in the subject.", "Keine Tanks im Gegenstand.")).annex(annex).build());
    }
    for tank in &document.tanks {
        let hr = tank.height_m / tank.radius_m.max(1e-9);
        let t_i = part_4::impulsive_period_s(tank.height_m, tank.radius_m);
        let t_c = part_4::convective_period_s(tank.radius_m);
        let s_e_i = part_1::elastic_response_spectrum(a_g, s, tb, tc, td, t_i);
        let s_e_c = part_1::elastic_response_spectrum(a_g, s, tb, tc, td, t_c);
        let mass = part_1::filled_content_seismic_mass_kg(tank.permanent_gk_n, tank.content_qk_n, &tank.content_category, tank.filling_ratio);
        let v = mass * part_4::impulsive_mass_ratio(hr) * s_e_i + mass * part_4::convective_mass_ratio(hr) * s_e_c;
        let path = id_sel("tanks", &tank.id);
        let mut c = CheckResult::assess(format!("en1998.4.{}.shear", tank.id), "DIN EN 1998-4", ClauseId::new("EN 1998-4", "4", "4.3"), SubjectRef::new(&tank.id, format!("{path}.vRdN"), lc(&tank.id, &tank.id)), lc("Tank base shear", "Tank-Fußquerkraft")).utilization(q_force(v), q_force(tank.v_rd_n)).annex(annex).explanation(lc(&format!("tank base shear V_Ed={v:.0} N"), &format!("Tank-Fußquerkraft V_Ed={v:.0} N")));
        if v > tank.v_rd_n {
            c = c.remedy(Remedy::at_least(SubjectRef::new(&tank.id, format!("{path}.vRdN"), lc("V_Rd", "V_Rd")), q_force(tank.v_rd_n), q_force(v), lc("Increase tank shear resistance.", "Tank-Querkrafttragfähigkeit erhöhen.")));
        }
        report.push(c.build());
    }
}

fn building_base_shear_n(document: &En1998Snapshot, building_id: &str, _annex_params: &AnnexParams, gamma_i: f64, a_g: f64, s: f64, tb: f64, tc: f64, td: f64) -> f64 {
    let Some(building) = document.buildings.iter().find(|b| b.id == building_id) else { return 0.0 };
    let total_height: f64 = building.storeys.iter().map(|st| st.height_m).sum();
    let n = building.storeys.len();
    let total_mass: f64 = building.storeys.iter().enumerate().map(|(i, st)| part_1::storey_seismic_mass_kg(st, i + 1 == n)).sum();
    let t1 = match building.t1_method.to_ascii_lowercase().as_str() {
        "given" => building.t1_given_s,
        _ => part_1::t1_from_ct(building.ct, total_height),
    };
    let system = building.systems.first();
    let q = system.map(|sys| part_1::behaviour_factor(sys.q0, sys.alpha_u_over_alpha_1, sys.k_w)).unwrap_or(1.5);
    let s_e = part_1::elastic_response_spectrum(a_g, s, tb, tc, td, t1);
    let s_d = part_1::design_spectrum_sd(s_e, gamma_i, q, a_g);
    let lambda = part_1::lambda_factor(t1, tc, building.storeys.len());
    part_1::base_shear_n(s_d, total_mass, lambda)
}

pub(super) fn evaluate_foundations_walls(report: &mut CheckReport, document: &En1998Snapshot, annex_params: &AnnexParams, a_g: f64, s: f64, tb: f64, tc: f64, td: f64, gamma_i: f64) {
    let annex = annex_params.choice();
    if document.foundations.is_empty() {
        report.push(CheckResult::assess("en1998.5.foundation.na", "DIN EN 1998-5", ClauseId::new("EN 1998-5", "5", "7"), SubjectRef::whole(lc("Foundations", "Gründungen")), lc("Foundation checks", "Gründungsnachweise")).not_applicable(lc("No foundations in the subject.", "Keine Gründungen im Gegenstand.")).annex(annex).build());
    }
    for f in &document.foundations {
        let path = id_sel("foundations", &f.id);
        let building_ok = document.buildings.iter().any(|b| b.id == f.supported_building_id);
        {
            let mut href = CheckResult::assess(
                format!("en1998.5.{}.supportedBuilding", f.id),
                "DIN EN 1998-5",
                ClauseId::new("EN 1998-5", "5", "5.3"),
                SubjectRef::new(&f.id, format!("{path}.supportedBuildingId"), lc(&f.id, &f.id)),
                lc("Foundation supported building reference", "Gründung — Bezug Gebäude"),
            )
            .utilization(q_dim(if building_ok { 0.5 } else { 1.5 }), q_dim(1.0))
            .annex(annex)
            .explanation(lc(
                &format!("supportedBuildingId={}{}", f.supported_building_id, if building_ok { " (resolved)" } else { " (dangling)" }),
                &format!("supportedBuildingId={}{}", f.supported_building_id, if building_ok { " (aufgelöst)" } else { " (hängend)" }),
            ));
            if !building_ok {
                let building_ids: Vec<String> = document.buildings.iter().map(|b| b.id.clone()).collect();
                let opts = if building_ids.is_empty() { vec![format!("{}-missing", f.supported_building_id)] } else { building_ids };
                href = href
                    .status(semio_s_plugin_norm_compliance::CheckStatus::Fail)
                    .remedy(Remedy::one_of(
                        SubjectRef::new(&f.id, format!("{path}.supportedBuildingId"), lc("Supported building id", "Gebäude-ID")),
                        opts,
                        lc("Set supportedBuildingId to one of the existing building ids.", "supportedBuildingId auf eine der vorhandenen Gebäude-Ids setzen."),
                    ));
            }
            report.push(href.build());
        }

        let support_id = if f.supported_building_id.is_empty() {
            document.buildings.first().map(|b| b.id.clone()).unwrap_or_default()
        } else {
            f.supported_building_id.clone()
        };
        let v_b = building_base_shear_n(document, &support_id, annex_params, gamma_i, a_g, s, tb, tc, td);
        let red = part_5::bearing_reduction_factor(a_g);
        let p_rd = f.p_rd_pa * red;
        let p_ed = part_5::seismic_bearing_pressure_pa(v_b, f.area_m2);
        let mut c = CheckResult::assess(format!("en1998.5.{}.bearing", f.id), "DIN EN 1998-5", ClauseId::new("EN 1998-5", "5", "7.3"), SubjectRef::new(&f.id, format!("{path}.pRdPa"), lc(&f.id, &f.id)), lc("Seismic bearing pressure", "Seismische Sohldruckspannung")).utilization(q_press(p_ed), q_press(p_rd)).annex(annex).explanation(lc(&format!("building={support_id}, Vb={v_b:.0} N, ped={p_ed:.0} Pa, prd_red={p_rd:.0} Pa",), &format!("Gebäude={support_id}, Vb={v_b:.0} N, ped={p_ed:.0} Pa, prd_red={p_rd:.0} Pa")));
        if p_ed > p_rd {
            let req_area = v_b / p_rd.max(1.0);
            c = c.remedy(Remedy::at_least(SubjectRef::new(&f.id, format!("{path}.areaM2"), lc("Area", "Fläche")), Quantity::area_m2(f.area_m2), Quantity::area_m2(req_area), lc(&format!("Increase foundation area to at least {req_area:.2} m²."), &format!("Gründungsfläche auf mindestens {req_area:.2} m² erhöhen."))));
        }
        report.push(c.build());
        let h_ed = v_b; // EN 1998-5 §5.3 — horizontal demand from supported building base shear
        let mut sld = CheckResult::assess(format!("en1998.5.{}.sliding", f.id), "DIN EN 1998-5", ClauseId::new("EN 1998-5", "5", "5.3"), SubjectRef::new(&f.id, format!("{path}.hRdN"), lc(&f.id, &f.id)), lc("Foundation sliding", "Gleiten der Gründung")).utilization(q_force(h_ed), q_force(f.h_rd_n)).annex(annex).explanation(lc(&format!("Hed=Vb={h_ed:.0} N from {support_id}, Hrd={:.0}", f.h_rd_n), &format!("Hed=Vb={h_ed:.0} N aus {support_id}, Hrd={:.0}", f.h_rd_n)));
        if h_ed > f.h_rd_n {
            sld = sld.remedy(Remedy::at_least(SubjectRef::new(&f.id, format!("{path}.hRdN"), lc("H_Rd", "H_Rd")), q_force(f.h_rd_n), q_force(h_ed), lc("Increase sliding resistance.", "Gleitwiderstand erhöhen.")));
        }
        report.push(sld.build());
        let xi = part_5::radiation_damping(part_5::stiffness_ratio(f.k_foundation, f.k_soil));
        report.push(
            CheckResult::assess(
                format!("en1998.5.{}.radiationDamping", f.id),
                "DIN EN 1998-5",
                ClauseId::new("EN 1998-5", "5", "5.3"),
                SubjectRef::new(&f.id, format!("{path}.kFoundation"), lc(&f.id, &f.id)),
                lc("Foundation radiation damping ξ", "Abstrahlungsdämpfung ξ der Gründung"),
            )
            .status(semio_s_plugin_norm_compliance::CheckStatus::Pass)
            .utilization(q_dim(xi), q_dim(1.0))
            .annex(annex)
            .explanation(lc(&format!("ξ={xi:.3} from k_f/k_s",), &format!("ξ={xi:.3} aus k_f/k_s")))
            .build(),
        );
    }
    if document.retaining_walls.is_empty() {
        report.push(CheckResult::assess("en1998.5.wall.na", "DIN EN 1998-5", ClauseId::new("EN 1998-5", "5", "7.3.2"), SubjectRef::whole(lc("Retaining walls", "Stützwände")), lc("Retaining wall checks", "Stützwandnachweise")).not_applicable(lc("No retaining walls in the subject.", "Keine Stützwände im Gegenstand.")).annex(annex).build());
    }
    for w in &document.retaining_walls {
        let k_h = part_5::horizontal_seismic_coefficient(a_g, s, w.r);
        let k_ae = part_5::mononobe_okabe_k_ae(w.phi_deg, k_h);
        let h_ed = part_5::retaining_wall_thrust_n_per_m(w.soil_gamma, w.height_m, k_ae);
        let path = id_sel("retainingWalls", &w.id);
        let mut c = CheckResult::assess(format!("en1998.5.{}.thrust", w.id), "DIN EN 1998-5", ClauseId::new("EN 1998-5", "5", "E.2"), SubjectRef::new(&w.id, format!("{path}.hRdNPerM"), lc(&w.id, &w.id)), lc("Retaining wall thrust", "Stützwandschub")).utilization(q_force(h_ed), q_force(w.h_rd_n_per_m)).annex(annex).explanation(lc(
                &format!("Hed={h_ed:.0} N/m, Hrd={:.0} N/m", w.h_rd_n_per_m),
                &format!("H_Ed={h_ed:.0} N/m, H_Rd={:.0} N/m", w.h_rd_n_per_m),
            ));
        if h_ed > w.h_rd_n_per_m {
            c = c.remedy(Remedy::at_least(SubjectRef::new(&w.id, format!("{path}.hRdNPerM"), lc("H_Rd", "H_Rd")), q_force(w.h_rd_n_per_m), q_force(h_ed), lc("Increase wall sliding resistance.", "Wand-Gleitwiderstand erhöhen.")));
        }
        report.push(c.build());
    }
}

pub(super) fn evaluate_towers(report: &mut CheckReport, document: &En1998Snapshot, annex_params: &AnnexParams, gamma_i: f64, a_g: f64, s: f64, tb: f64, tc: f64, td: f64) {
    let annex = annex_params.choice();
    if document.towers.is_empty() {
        report.push(CheckResult::assess("en1998.6.na", "DIN EN 1998-6", ClauseId::new("EN 1998-6", "6", "4.3"), SubjectRef::whole(lc("Towers", "Türme")), lc("Tower checks", "Turmnachweise")).not_applicable(lc("No towers in the subject.", "Keine Türme im Gegenstand.")).annex(annex).build());
        return;
    }
    for t in &document.towers {
        let q = part_6::tower_behaviour_factor(t.q_nominal, t.is_chimney);
        let s_e = part_1::elastic_response_spectrum(a_g, s, tb, tc, td, 0.5);
        let s_d = part_1::design_spectrum_sd(s_e, gamma_i, q, a_g);
        let mass = part_1::entity_seismic_mass_kg(t.permanent_gk_n, &t.variables, false, t.correlated_occupancy);
        let v = part_6::tower_base_shear_n(part_6::cantilever_modal_participation_factor(), s_d, mass);
        let m_ed = part_6::tower_overturning_moment_nm(v, t.height_m);
        let path = id_sel("towers", &t.id);
        let mut c = CheckResult::assess(format!("en1998.6.{}.overturning", t.id), "DIN EN 1998-6", ClauseId::new("EN 1998-6", "6", "4.3.2"), SubjectRef::new(&t.id, format!("{path}.mRdNm"), lc(&t.id, &t.id)), lc("Tower overturning moment", "Turm-Kippmoment")).utilization(q_mom(m_ed), q_mom(t.m_rd_nm)).annex(annex).explanation(lc(
                &format!("V={v:.0} N, Med={m_ed:.0} Nm, Mrd={:.0} Nm", t.m_rd_nm),
                &format!("V={v:.0} N, M_Ed={m_ed:.0} Nm, M_Rd={:.0} Nm", t.m_rd_nm),
            ));
        if m_ed > t.m_rd_nm {
            c = c.remedy(Remedy::at_least(SubjectRef::new(&t.id, format!("{path}.mRdNm"), lc("M_Rd", "M_Rd")), q_mom(t.m_rd_nm), q_mom(m_ed), lc("Increase overturning resistance.", "Kippwiderstand erhöhen.")));
        }
        report.push(c.build());
    }
}

pub fn evaluate(document: &En1998Snapshot) -> CheckReport {
    check_full_seismic(document)
}
