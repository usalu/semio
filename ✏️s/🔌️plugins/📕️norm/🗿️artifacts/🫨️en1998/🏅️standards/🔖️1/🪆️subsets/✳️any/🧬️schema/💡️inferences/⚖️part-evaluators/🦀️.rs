//! ⚖️ Part evaluators of the EN 1998 inference: the integrity checks and the bridge, assessment, silo, foundation and tower evaluations. See [`super`].

use super::*;
use semio_s_artifact_norm_en1998_model::{En1998Foundation, En1998RetainingWall, En1998Slope, En1998TieBeam, EnSpectrumType};
use semio_s_plugin_norm_mechanics::geotechnics::earth_pressure::{thrust, Acceleration, Interface, Side, Water, WESTERGAARD_ARM};

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

fn push_dangling_reference(report: &mut CheckReport, annex: AnnexChoice, table: &str, id: &str, field: &str, target: &str, options: Vec<String>) {
    let path = format!("{table}[id={id}].{field}");
    let subject = SubjectRef::new(id.to_owned(), &path, lc(&format!("{table} {id}"), &format!("{table} {id}")));
    report.push(
        CheckResult::assess(format!("en1998.integrity.reference.{table}.{id}.{field}"), "EN 1998 integrity", ClauseId::new("EN 1998", "§2", "reference"), subject.clone(), lc(&format!("Existing {target}"), &format!("Vorhandene Referenz auf {target}")))
            .annex(annex)
            .explanation(lc(&format!("{table}[{id}].{field} names no existing {target}."), &format!("{table}[{id}].{field} nennt kein vorhandenes Ziel ({target}).")))
            .status(semio_s_plugin_norm_compliance::CheckStatus::Fail)
            .remedy(Remedy::one_of(subject, options, lc(&format!("Point {field} at an existing {target}."), &format!("{field} auf ein vorhandenes Ziel ({target}) setzen."))))
            .build(),
    );
}

fn push_bridge_references(report: &mut CheckReport, document: &En1998Snapshot, annex: AnnexChoice) {
    let bridges: Vec<String> = document.bridges.iter().map(|b| b.id.clone()).collect();
    for p in document.piers.iter().filter(|p| !bridges.contains(&p.bridge_id)) {
        push_dangling_reference(report, annex, "piers", &p.id, "bridgeId", "bridge", bridges.clone());
    }
    for k in &document.bearings {
        if !bridges.contains(&k.bridge_id) {
            push_dangling_reference(report, annex, "bearings", &k.id, "bridgeId", "bridge", bridges.clone());
        } else if !k.pier_id.is_empty() && !document.piers.iter().any(|p| p.id == k.pier_id && p.bridge_id == k.bridge_id) {
            let options = document.piers.iter().filter(|p| p.bridge_id == k.bridge_id).map(|p| p.id.clone()).chain([String::new()]).collect();
            push_dangling_reference(report, annex, "bearings", &k.id, "pierId", "pier", options);
        }
    }
}

fn push_isolation_references(report: &mut CheckReport, document: &En1998Snapshot, annex: AnnexChoice) {
    let bridges: Vec<String> = document.bridges.iter().map(|b| b.id.clone()).collect();
    let buildings: Vec<String> = document.buildings.iter().map(|b| b.id.clone()).collect();
    let or_missing = |options: Vec<String>, id: &str| if options.is_empty() { vec![format!("{id}-missing")] } else { options };
    for system in &document.isolation_systems {
        let (bridge_named, building_named) = (!system.bridge_id.is_empty(), !system.building_id.is_empty());
        if bridge_named && building_named {
            push_dangling_reference(report, annex, "isolationSystems", &system.id, "buildingId", "empty building (a system isolates a bridge or a building, not both)", vec![String::new()]);
        } else if bridge_named && !bridges.contains(&system.bridge_id) {
            push_dangling_reference(report, annex, "isolationSystems", &system.id, "bridgeId", "bridge", or_missing(bridges.clone(), &system.bridge_id));
        } else if building_named && !buildings.contains(&system.building_id) {
            push_dangling_reference(report, annex, "isolationSystems", &system.id, "buildingId", "building", or_missing(buildings.clone(), &system.building_id));
        } else if !bridge_named && !building_named {
            push_dangling_reference(report, annex, "isolationSystems", &system.id, "bridgeId", "bridge or building", or_missing(bridges.iter().chain(&buildings).cloned().collect(), &system.id));
        }
    }
    for base in &document.pier_bases {
        if !bridges.contains(&base.bridge_id) {
            push_dangling_reference(report, annex, "pierBases", &base.id, "bridgeId", "bridge", or_missing(bridges.clone(), &base.bridge_id));
        }
    }
}

fn push_horizontal_references(report: &mut CheckReport, document: &En1998Snapshot, annex: AnnexChoice) {
    let buildings: Vec<String> = document.buildings.iter().map(|b| b.id.clone()).collect();
    for d in &document.diaphragms {
        match document.buildings.iter().find(|b| b.id == d.building_id) {
            None => push_dangling_reference(report, annex, "diaphragms", &d.id, "buildingId", "building", buildings.clone()),
            Some(b) if !b.storeys.iter().any(|s| s.id == d.storey_id) => push_dangling_reference(report, annex, "diaphragms", &d.id, "storeyId", "storey", b.storeys.iter().map(|s| s.id.clone()).collect()),
            Some(_) => {}
        }
    }
    for a in document.accelerograms.iter().filter(|a| !buildings.contains(&a.building_id)) {
        push_dangling_reference(report, annex, "accelerograms", &a.id, "buildingId", "building", buildings.clone());
    }
}

fn push_joint_references(report: &mut CheckReport, document: &En1998Snapshot, annex: AnnexChoice) {
    let buildings: Vec<String> = document.buildings.iter().map(|b| b.id.clone()).collect();
    for j in &document.joints {
        let Some(b) = document.buildings.iter().find(|b| b.id == j.building_id) else {
            push_dangling_reference(report, annex, "joints", &j.id, "buildingId", "building", buildings.clone());
            continue;
        };
        if !b.storeys.iter().any(|s| s.id == j.storey_id) {
            push_dangling_reference(report, annex, "joints", &j.id, "storeyId", "storey", b.storeys.iter().map(|s| s.id.clone()).collect());
        }
        for (field, id, role) in [("columnId", &j.column_id, semio_s_artifact_norm_en1998_model::MemberKind::Column), ("beamId", &j.beam_id, semio_s_artifact_norm_en1998_model::MemberKind::Beam)] {
            if !b.members.iter().any(|m| &m.id == id && m.role == role) {
                push_dangling_reference(report, annex, "joints", &j.id, field, "member", b.members.iter().filter(|m| m.role == role).map(|m| m.id.clone()).collect());
            }
        }
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
    push_duplicate_ids(report, annex, "soilLayers", &document.soil_layers.iter().map(|l| l.id.clone()).collect::<Vec<_>>(), &|id| format!("soilLayers[id={id}].id"));
    push_duplicate_ids(report, annex, "diaphragms", &document.diaphragms.iter().map(|l| l.id.clone()).collect::<Vec<_>>(), &|id| format!("diaphragms[id={id}].id"));
    push_duplicate_ids(report, annex, "accelerograms", &document.accelerograms.iter().map(|l| l.id.clone()).collect::<Vec<_>>(), &|id| format!("accelerograms[id={id}].id"));
    push_duplicate_ids(report, annex, "piers", &document.piers.iter().map(|l| l.id.clone()).collect::<Vec<_>>(), &|id| format!("piers[id={id}].id"));
    push_duplicate_ids(report, annex, "bearings", &document.bearings.iter().map(|l| l.id.clone()).collect::<Vec<_>>(), &|id| format!("bearings[id={id}].id"));
    push_duplicate_ids(report, annex, "joints", &document.joints.iter().map(|l| l.id.clone()).collect::<Vec<_>>(), &|id| format!("joints[id={id}].id"));
    push_duplicate_ids(report, annex, "tieBeams", &document.tie_beams.iter().map(|l| l.id.clone()).collect::<Vec<_>>(), &|id| format!("tie_beams[id={id}].id"));
    push_duplicate_ids(report, annex, "slopes", &document.slopes.iter().map(|l| l.id.clone()).collect::<Vec<_>>(), &|id| format!("slopes[id={id}].id"));
    push_duplicate_ids(report, annex, "placements", &document.placements.iter().map(|t| t.id.clone()).collect::<Vec<_>>(), &|id| format!("placements[id={id}].id"));
    push_duplicate_ids(report, annex, "isolationSystems", &document.isolation_systems.iter().map(|l| l.id.clone()).collect::<Vec<_>>(), &|id| format!("isolationSystems[id={id}].id"));
    push_duplicate_ids(report, annex, "pierBases", &document.pier_bases.iter().map(|l| l.id.clone()).collect::<Vec<_>>(), &|id| format!("pierBases[id={id}].id"));
    push_isolation_references(report, document, annex);
    push_horizontal_references(report, document, annex);
    push_bridge_references(report, document, annex);
    push_joint_references(report, document, annex);

    for building in &document.buildings {
        let bid = &building.id;
        push_duplicate_ids(report, annex, "systems", &building.systems.iter().map(|s| s.id.clone()).collect::<Vec<_>>(), &|id| format!("buildings[id={bid}].systems[id={id}].id"));
        push_duplicate_ids(report, annex, "storeys", &building.storeys.iter().map(|s| s.id.clone()).collect::<Vec<_>>(), &|id| format!("buildings[id={bid}].storeys[id={id}].id"));
        push_duplicate_ids(report, annex, "members", &building.members.iter().map(|m| m.id.clone()).collect::<Vec<_>>(), &|id| format!("buildings[id={bid}].members[id={id}].id"));
        for member in building.members.iter().filter(|m| !m.wall_storey_id.is_empty() && !building.storeys.iter().any(|s| s.id == m.wall_storey_id)) {
            push_dangling_reference(report, annex, &format!("buildings[id={bid}].members"), &member.id, "wallStoreyId", "storey", building.storeys.iter().map(|s| s.id.clone()).chain([String::new()]).collect());
        }
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

pub(super) fn evaluate_bridges(report: &mut CheckReport, document: &En1998Snapshot, annex_params: &AnnexParams, gamma_i: f64, a_g: f64, s: f64, tb: f64, tc: f64, td: f64, scope: Scope<'_>) {
    let annex = annex_params.choice();
    if document.bridges.is_empty() && scope.part("bridges-na") {
        report.push(CheckResult::assess("en1998.2.na", "DIN EN 1998-2", ClauseId::new("EN 1998-2", "2", "5"), SubjectRef::whole(lc("Bridges", "Brücken")), lc("Bridge seismic checks", "Brücken-Erdbebennachweise")).not_applicable(lc("No bridges in the subject.", "Keine Brücken im Gegenstand.")).annex(annex).build());
        return;
    }
    for (_, bridge) in document.bridges.iter().enumerate().filter(|(index, _)| scope.admits("bridges", *index)) {
        let gamma_i = part_2::bridge_gamma_i(bridge.importance_class);
        let q_isol = part_2::design_behaviour_factor(bridge.period_ratio);
        let mass = part_1::entity_seismic_mass_kg(bridge.permanent_gk_n, &bridge.variables, false, bridge.correlated_occupancy);
        let ground = Ground { a_g, s, tb, tc, td, gamma_i };
        let analysis = bridge_analysis(document, bridge, &ground, q_isol, mass, annex);
        check_bridge_action(report, annex, bridge, a_g);
        let t1 = analysis.as_ref().and_then(|a| a.longitudinal.as_ref()).and_then(|r| r.periods_s.first().copied()).unwrap_or_else(|| bridge.fundamental_period_s.max(0.05));
        {
            let mut pr = CheckResult::assess(
                format!("en1998.2.{}.periodRatio", bridge.id),
                "DIN EN 1998-2",
                ClauseId::new("EN 1998-2", "2", "7.1"),
                SubjectRef::new(&bridge.id, format!("{}.periodRatio", id_sel("bridges", &bridge.id)), lc(&bridge.id, &bridge.id)),
                lc("Isolation period ratio T_isol/T_fixed", "Isolations-Periodenverhältnis T_isol/T_fixed"),
            )
            .minimum(q_dim(bridge.period_ratio), q_dim(1.0))
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
        let s_d = part_1::design_spectrum_at(a_g, gamma_i, s, tb, tc, td, q_isol, t1);
        let v = part_1::base_shear_n(s_d, mass, 1.0);
        let d_ed = part_2::bearing_displacement_m(s_e * gamma_i, t1);
        let path = id_sel("bridges", &bridge.id);
        let mut c = CheckResult::assess(format!("en1998.2.{}.shear", bridge.id), "DIN EN 1998-2", ClauseId::new("EN 1998-2", "2", "5.3"), SubjectRef::new(&bridge.id, format!("{path}.vRdN"), lc(&bridge.id, &bridge.id)), lc("Bridge base shear", "Brücken-Querkraft")).utilization(q_force(v), floor_limit(q_force(bridge.v_rd_n))).annex(annex).explanation(lc(
                &format!("T={t1:.3}s, periodRatio={:.2}, q_isol={q_isol:.2}, mass={mass:.0} kg, Ved={v:.0} N, Vrd={:.0} N", bridge.period_ratio, bridge.v_rd_n),
                &format!("T={t1:.3}s, Periodenverhältnis={:.2}, q_isol={q_isol:.2}, Masse={mass:.0} kg, V_Ed={v:.0} N, V_Rd={:.0} N", bridge.period_ratio, bridge.v_rd_n),
            ));
        if v > bridge.v_rd_n {
            c = c.remedy(Remedy::at_least(SubjectRef::new(&bridge.id, format!("{path}.vRdN"), lc("V_Rd", "V_Rd")), q_force(bridge.v_rd_n), q_force(v), lc(&format!("Increase bridge V_Rd to at least {v:.0} N."), &format!("Brücken-V_Rd auf mindestens {v:.0} N erhöhen."))));
        }
        report.push(c.build());
        if let Some(rows) = &analysis {
            check_bridge_rows(report, annex, document, bridge, rows, &ground);
            check_bridge_piers(report, annex, document, bridge, rows);
            continue;
        }
        let mut b = CheckResult::assess(format!("en1998.2.{}.bearing", bridge.id), "DIN EN 1998-2", ClauseId::new("EN 1998-2", "2", "7.5"), SubjectRef::new(&bridge.id, format!("{path}.bearingDRdM"), lc(&bridge.id, &bridge.id)), lc("Bearing displacement", "Lagerverschiebung")).utilization(q_len(d_ed), floor_limit(q_len(bridge.bearing_d_rd_m))).annex(annex).explanation(lc(&format!("ded={d_ed:.4} m from Sd(T), drd={:.4} m", bridge.bearing_d_rd_m), &format!("ded={d_ed:.4} m aus Sd(T), drd={:.4} m", bridge.bearing_d_rd_m)));
        if d_ed > bridge.bearing_d_rd_m {
            b = b.remedy(Remedy::at_least(SubjectRef::new(&bridge.id, format!("{path}.bearingDRdM"), lc("d_Rd", "d_Rd")), q_len(bridge.bearing_d_rd_m), q_len(d_ed), lc("Increase bearing displacement capacity.", "Lagerverschiebungskapazität erhöhen.")));
        }
        report.push(b.build());
        check_pier(report, annex, bridge);
    }
}

/// 🛞 The isolation systems of bridges and buildings: the equivalent linear analysis of each system on the mass and the fixed-base period of the structure it names; a system that names no resolvable structure is reported by the integrity checks.
pub(super) fn evaluate_isolation_systems(report: &mut CheckReport, document: &En1998Snapshot, annex_params: &AnnexParams, gamma_i: f64, a_g: f64, s: f64, tb: f64, tc: f64, td: f64, scope: Scope<'_>) {
    let annex = annex_params.choice();
    for (_, system) in document.isolation_systems.iter().enumerate().filter(|(index, _)| scope.admits("isolationSystems", *index)) {
        let bridge = document.bridges.iter().find(|b| !system.bridge_id.is_empty() && system.building_id.is_empty() && b.id == system.bridge_id);
        let building = document.buildings.iter().find(|b| !system.building_id.is_empty() && system.bridge_id.is_empty() && b.id == system.building_id);
        let (supported, importance) = match (bridge, building) {
            (Some(b), _) => (IsolationSupported { is_bridge: true, id: b.id.clone(), mass_kg: part_1::entity_seismic_mass_kg(b.permanent_gk_n, &b.variables, false, b.correlated_occupancy), fixed_period_s: b.fundamental_period_s, shear_rd_n: b.v_rd_n }, part_2::bridge_gamma_i(b.importance_class)),
            (None, Some(b)) => {
                let n = b.storeys.len();
                let masses: Vec<f64> = b.storeys.iter().enumerate().map(|(i, st)| part_1::storey_seismic_mass_kg(st, i + 1 == n)).collect();
                let (stiff_x, stiff_y) = semio_s_artifact_norm_en1998_schema::cracked_stiffness::storeys_xy(b);
                let (t1, _) = part_1::fundamental_period_s(b.t1_method, b.ct, b.t1_given_s, b.storeys.iter().map(|st| st.height_m).sum(), &masses, &stiff_x, &stiff_y);
                (IsolationSupported { is_bridge: false, id: b.id.clone(), mass_kg: masses.iter().sum(), fixed_period_s: t1, shear_rd_n: 0.0 }, gamma_i)
            }
            (None, None) => continue,
        };
        check_isolation_system(report, annex, system, &supported, &Spectrum { a_g: importance * a_g, s, tb, tc, td });
    }
}

/// 🪨 The foundation springs of the bridge piers: Gazetas springs with the degraded shear modulus, the period lengthening of the pier and the bearing displacement with it, on the fixed-base period of the bridge.
pub(super) fn evaluate_pier_bases(report: &mut CheckReport, document: &En1998Snapshot, annex_params: &AnnexParams, a_g: f64, s: f64, tb: f64, tc: f64, td: f64, scope: Scope<'_>) {
    let annex = annex_params.choice();
    for (_, base) in document.pier_bases.iter().enumerate().filter(|(index, _)| scope.admits("pierBases", *index)) {
        let Some(bridge) = document.bridges.iter().find(|b| b.id == base.bridge_id) else { continue };
        let ground = Ground { a_g, s, tb, tc, td, gamma_i: part_2::bridge_gamma_i(bridge.importance_class) };
        let mass = part_1::entity_seismic_mass_kg(bridge.permanent_gk_n, &bridge.variables, false, bridge.correlated_occupancy);
        let analysis = bridge_analysis(document, bridge, &ground, part_2::design_behaviour_factor(bridge.period_ratio), mass, annex);
        let t_fixed = analysis.as_ref().and_then(|a| a.longitudinal.as_ref()).and_then(|r| r.periods_s.first().copied()).unwrap_or_else(|| bridge.fundamental_period_s.max(0.05));
        check_pier_base(report, annex, document, bridge, base, &ground, t_fixed);
    }
}

pub(super) fn evaluate_assessments(report: &mut CheckReport, document: &En1998Snapshot, annex_params: &AnnexParams, gamma_i: f64, a_g: f64, s: f64, tb: f64, tc: f64, td: f64, scope: Scope<'_>) {
    let annex = annex_params.choice();
    if document.assessments.is_empty() && scope.part("assessments-na") {
        report.push(CheckResult::assess("en1998.3.na", "DIN EN 1998-3", ClauseId::new("EN 1998-3", "3", "2.3"), SubjectRef::whole(lc("Assessments", "Bestandsbewertung")), lc("Retrofit checks", "Ertüchtigungsnachweise")).not_applicable(lc("No retrofit assessments in the subject.", "Keine Bestandsbewertungen im Gegenstand.")).annex(annex).build());
        return;
    }
    for (_, a) in document.assessments.iter().enumerate().filter(|(index, _)| scope.admits("assessments", *index)) {
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
            .status(if building_ok { semio_s_plugin_norm_compliance::CheckStatus::Pass } else { semio_s_plugin_norm_compliance::CheckStatus::Fail })
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

        let basis = part_3_state::basis(a, annex);
        let ls = a.limit_state.token();
        let ag_fac = part_3::limit_state_ag_factor(&ls, annex);
        let cf_raw = part_3::confidence_factor(a.knowledge_level);
        let cf = if part_3::limit_state_applies_cf(&ls) { cf_raw } else { 1.0 };
        let r_d = part_3::design_capacity_n(a.r_k_n, cf, a.gamma_el);
        let support_id = if a.supported_building_id.is_empty() {
            document.buildings.first().map(|b| b.id.clone()).unwrap_or_default()
        } else {
            a.supported_building_id.clone()
        };
        let e_d = building_base_shear_n(document, &support_id, annex_params, gamma_i, a_g * ag_fac, s, tb, tc, td);
        let path = id_sel("assessments", &a.id);
        let mut c = CheckResult::assess(format!("en1998.3.{}.capacity", a.id), "DIN EN 1998-3", ClauseId::new("EN 1998-3", "3", "2.1"), SubjectRef::new(&a.id, format!("{path}.rKN"), lc(&a.id, &a.id)), lc("Limit-state capacity check", "Grenzzustands-Tragfähigkeitsnachweis")).utilization(q_force(e_d), floor_limit(q_force(r_d))).annex(annex).explanation(lc(&format!("LS={}, ag×{ag_fac:.2}, CF={cf:.2}, Ed={e_d:.0} N from building {support_id}, Rd={r_d:.0} N", a.limit_state.token()), &format!("GZ={}, ag×{ag_fac:.2}, CF={cf:.2}, Ed={e_d:.0} N aus Gebäude {support_id}, Rd={r_d:.0} N", a.limit_state.token())));
        if e_d > r_d {
            let req = e_d * cf * a.gamma_el;
            c = c.remedy(Remedy::at_least(SubjectRef::new(&a.id, format!("{path}.rKN"), lc("R_k", "R_k")), q_force(a.r_k_n), q_force(req), lc(&format!("Increase R_k to at least {req:.0} N (or lower limit-state demand)."), &format!("R_k auf mindestens {req:.0} N erhöhen (oder Grenzzustandslast senken)."))));
        }
        let ls_c = CheckResult::assess(format!("en1998.3.{}.limitState", a.id), "DIN EN 1998-3", ClauseId::new("EN 1998-3", "3", "2.1"), SubjectRef::new(&a.id, format!("{path}.limitState"), lc(&a.id, &a.id)), lc("EN 1998-3 limit state (NC/SD/DL)", "EN-1998-3-Grenzzustand (NC/SD/DL)")).status(semio_s_plugin_norm_compliance::CheckStatus::Pass).annex(annex).explanation(lc(
                &format!("limitState={}, T_R = {:.0} yr (a_g factor {:.3}), CF {:.2}{}", a.limit_state.token(), basis.return_period_yr, basis.ag_factor, basis.cf, na_return_period_note(annex, false)),
                &format!("Grenzzustand={}, T_R = {:.0} a (a_g-Faktor {:.3}), CF {:.2}{}", a.limit_state.token(), basis.return_period_yr, basis.ag_factor, basis.cf, na_return_period_note(annex, true)),
            ));
        report.push(ls_c.build());
        report.push(c.build());
        check_chord_rotation(report, annex, a);
        check_cyclic_shear(report, annex, a);
        check_analysis_method(report, annex, document, a);
        check_n2_performance(report, annex, document, a, gamma_i, a_g, s, tb, tc, td);
    }
}

pub(super) fn evaluate_silos_tanks(report: &mut CheckReport, document: &En1998Snapshot, annex_params: &AnnexParams, gamma_i: f64, a_g: f64, s: f64, tb: f64, tc: f64, td: f64, scope: Scope<'_>) {
    let annex = annex_params.choice();
    if document.silos.is_empty() && scope.part("silos-na") {
        report.push(CheckResult::assess("en1998.4.silo.na", "DIN EN 1998-4", ClauseId::new("EN 1998-4", "4", "3"), SubjectRef::whole(lc("Silos", "Silos")), lc("Silo checks", "Silo-Nachweise")).not_applicable(lc("No silos in the subject.", "Keine Silos im Gegenstand.")).annex(annex).build());
    }
    for (_, silo) in document.silos.iter().enumerate().filter(|(index, _)| scope.admits("silos", *index)) {
        let q = part_4::silo_behaviour_factor(silo.q_nominal);
        let s_e = part_1::elastic_response_spectrum(a_g, s, tb, tc, td, tc);
        let s_d = part_1::design_spectrum_at(a_g, gamma_i, s, tb, tc, td, q, tc);
        let mass = part_1::filled_content_seismic_mass_kg(silo.permanent_gk_n, silo.content_qk_n, silo.content_category, silo.filling_ratio);
        let v = part_1::base_shear_n(s_d, mass, 1.0);
        let n_ed = part_4::silo_axial_demand_n(mass, a_g);
        let path = id_sel("silos", &silo.id);
        let mut c = CheckResult::assess(format!("en1998.4.{}.wall", silo.id), "DIN EN 1998-4", ClauseId::new("EN 1998-4", "4", "3.4"), SubjectRef::new(&silo.id, format!("{path}.nRdN"), lc(&silo.id, &silo.id)), lc("Silo shell axial resistance", "Silowand-Normalkrafttragfähigkeit")).utilization(q_force(n_ed), floor_limit(q_force(silo.n_rd_n))).annex(annex).explanation(lc(
                &format!("q={:.2}, N_Ed={n_ed:.0} N (m·(g+0.9 a_g)), N_Rd={:.0} N, plateau S_d used (stiffness unknown)", silo.q_nominal, silo.n_rd_n),
                &format!("q={:.2}, N_Ed={n_ed:.0} N (m·(g+0,9 a_g)), N_Rd={:.0} N, Plateau-S_d angesetzt (Steifigkeit unbekannt)", silo.q_nominal, silo.n_rd_n),
            ));
        if n_ed > silo.n_rd_n {
            c = c.remedy(Remedy::at_least(SubjectRef::new(&silo.id, format!("{path}.nRdN"), lc("N_Rd", "N_Rd")), q_force(silo.n_rd_n), q_force(n_ed), lc("Increase silo wall resistance.", "Silowandtragfähigkeit erhöhen.")));
        }
        report.push(c.build());
        let mut a = CheckResult::assess(format!("en1998.4.{}.anchor", silo.id), "DIN EN 1998-4", ClauseId::new("EN 1998-4", "4", "3.5"), SubjectRef::new(&silo.id, format!("{path}.vRdN"), lc(&silo.id, &silo.id)), lc("Silo anchorage", "Silo-Verankerung")).utilization(q_force(v), floor_limit(q_force(silo.v_rd_n))).annex(annex).explanation(lc(&format!("Ved={v:.0} from impulsive/convective, Vrd={:.0}", silo.v_rd_n), &format!("Ved={v:.0} aus Impuls/Konvektion, Vrd={:.0}", silo.v_rd_n)));
        if v > silo.v_rd_n {
            a = a.remedy(Remedy::at_least(SubjectRef::new(&silo.id, format!("{path}.vRdN"), lc("V_Rd", "V_Rd")), q_force(silo.v_rd_n), q_force(v), lc("Increase anchorage resistance.", "Verankerungstragfähigkeit erhöhen.")));
        }
        report.push(a.build());
    }
    if document.tanks.is_empty() && scope.part("tanks-na") {
        report.push(CheckResult::assess("en1998.4.tank.na", "DIN EN 1998-4", ClauseId::new("EN 1998-4", "4", "4"), SubjectRef::whole(lc("Tanks", "Behälter")), lc("Tank checks", "Tank-Nachweise")).not_applicable(lc("No tanks in the subject.", "Keine Tanks im Gegenstand.")).annex(annex).build());
    }
    for (_, tank) in document.tanks.iter().enumerate().filter(|(index, _)| scope.admits("tanks", *index)) {
        let fluid_height = semio_s_artifact_norm_en1998_schema::tank::fluid_height_m(tank.height_m, tank.filling_ratio);
        let hr = fluid_height / tank.radius_m.max(1e-9);
        let t_c = part_4::convective_period_s(fluid_height, tank.radius_m);
        let s_e_i = gamma_i * part_1::elastic_response_spectrum(a_g, s, tb, tc, td, tc);
        let s_e_c = gamma_i * part_1::elastic_response_spectrum_eta(a_g, s, tb, tc, td, t_c, part_4::convective_damping_eta());
        let fluid_mass = tank.filling_ratio.clamp(0.0, 1.0) * tank.content_qk_n / 9.81;
        let wall_mass = tank.permanent_gk_n / 9.81;
        let v_i = (fluid_mass * part_4::impulsive_mass_ratio(hr) + wall_mass) * s_e_i;
        let v_c = fluid_mass * part_4::convective_mass_ratio(hr) * s_e_c;
        let v = part_4::srss(v_i, v_c);
        let path = id_sel("tanks", &tank.id);
        let mut c = CheckResult::assess(format!("en1998.4.{}.shear", tank.id), "DIN EN 1998-4", ClauseId::new("EN 1998-4", "4", "4.3"), SubjectRef::new(&tank.id, format!("{path}.vRdN"), lc(&tank.id, &tank.id)), lc("Tank base shear", "Tank-Fußquerkraft")).utilization(q_force(v), floor_limit(q_force(tank.v_rd_n))).annex(annex).explanation(lc(&format!("V_i={v_i:.0} N, V_c={v_c:.0} N (T_c={t_c:.2} s, η=1.348), SRSS V_Ed={v:.0} N"), &format!("V_i={v_i:.0} N, V_c={v_c:.0} N (T_c={t_c:.2} s, η=1,348), SRSS V_Ed={v:.0} N")));
        if v > tank.v_rd_n {
            c = c.remedy(Remedy::at_least(SubjectRef::new(&tank.id, format!("{path}.vRdN"), lc("V_Rd", "V_Rd")), q_force(tank.v_rd_n), q_force(v), lc("Increase tank shear resistance.", "Tank-Querkrafttragfähigkeit erhöhen.")));
        }
        report.push(c.build());
        check_tank_hydrodynamics(report, annex, tank, s_e_i, s_e_c, t_c);
    }
}

fn building_gravity_and_height(document: &En1998Snapshot, building_id: &str) -> (f64, f64) {
    let Some(building) = document.buildings.iter().find(|b| b.id == building_id) else { return (0.0, 0.0) };
    let n = building.storeys.len();
    let weight: f64 = building.storeys.iter().enumerate().map(|(i, st)| part_1::storey_seismic_action_n(st, i + 1 == n)).sum();
    (weight, building.storeys.iter().map(|st| st.height_m).sum())
}

fn building_base_shear_n(document: &En1998Snapshot, building_id: &str, _annex_params: &AnnexParams, gamma_i: f64, a_g: f64, s: f64, tb: f64, tc: f64, td: f64) -> f64 {
    let Some(building) = document.buildings.iter().find(|b| b.id == building_id) else { return 0.0 };
    let total_height: f64 = building.storeys.iter().map(|st| st.height_m).sum();
    let n = building.storeys.len();
    let total_mass: f64 = building.storeys.iter().enumerate().map(|(i, st)| part_1::storey_seismic_mass_kg(st, i + 1 == n)).sum();
    let masses: Vec<f64> = building.storeys.iter().enumerate().map(|(i, st)| part_1::storey_seismic_mass_kg(st, i + 1 == n)).collect();
    let (stiff_x, stiff_y) = semio_s_artifact_norm_en1998_schema::cracked_stiffness::storeys_xy(building);
    let (t1, _) = part_1::fundamental_period_s(building.t1_method, building.ct, building.t1_given_s, total_height, &masses, &stiff_x, &stiff_y);
    part_1::design_base_shear(building.systems.first(), building.elevation_regular, n, total_mass, t1, a_g, gamma_i, s, tb, tc, td).f_b
}

pub(super) fn evaluate_foundations_walls(report: &mut CheckReport, document: &En1998Snapshot, annex_params: &AnnexParams, a_g: f64, s: f64, tb: f64, tc: f64, td: f64, gamma_i: f64, scope: Scope<'_>) {
    let annex = annex_params.choice();
    if document.foundations.is_empty() && scope.part("foundations-na") {
        report.push(CheckResult::assess("en1998.5.foundation.na", "DIN EN 1998-5", ClauseId::new("EN 1998-5", "5", "7"), SubjectRef::whole(lc("Foundations", "Gründungen")), lc("Foundation checks", "Gründungsnachweise")).not_applicable(lc("No foundations in the subject.", "Keine Gründungen im Gegenstand.")).annex(annex).build());
    }
    for (_, f) in document.foundations.iter().enumerate().filter(|(index, _)| scope.admits("foundations", *index)) {
        let path = id_sel("foundations", &f.id);
        check_liquefaction(report, annex, f, gamma_i, a_g, s);
        check_soil_profile(report, annex, document, f, gamma_i, a_g, s);
        check_bearing_annex_f(report, annex, f, gamma_i, a_g, s);
        check_pile(report, annex, f, gamma_i, a_g, s);
        check_liquefaction_hazard(report, annex, document, f, gamma_i, a_g, s);
        check_pile_liquefied(report, annex, document, f, gamma_i, a_g, s);
        check_degraded_modulus(report, annex, document, f, gamma_i, a_g, s);
        let building_ok = document.buildings.iter().any(|b| b.id == f.supported_building_id);
        {
            let mut href = CheckResult::assess(
                format!("en1998.5.{}.supportedBuilding", f.id),
                "DIN EN 1998-5",
                ClauseId::new("EN 1998-5", "5", "5.3"),
                SubjectRef::new(&f.id, format!("{path}.supportedBuildingId"), lc(&f.id, &f.id)),
                lc("Foundation supported building reference", "Gründung — Bezug Gebäude"),
            )
            .status(if building_ok { semio_s_plugin_norm_compliance::CheckStatus::Pass } else { semio_s_plugin_norm_compliance::CheckStatus::Fail })
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
        let p_rd = f.p_rd_pa / part_5::bearing_gamma_rd();
        let (n_ed, h_total) = building_gravity_and_height(document, &support_id);
        let p_ed = part_5::eccentric_bearing_pressure_pa(n_ed, v_b * h_total * 2.0 / 3.0, f.area_m2);
        let mut c = CheckResult::assess(format!("en1998.5.{}.bearing", f.id), "DIN EN 1998-5", ClauseId::new("EN 1998-5", "5", "7.3"), SubjectRef::new(&f.id, format!("{path}.pRdPa"), lc(&f.id, &f.id)), lc("Seismic bearing pressure", "Seismische Sohldruckspannung")).utilization(q_press(p_ed), floor_limit(q_press(p_rd))).annex(annex).explanation(lc(&format!("building={support_id}, Vb={v_b:.0} N, ped={p_ed:.0} Pa, prd_red={p_rd:.0} Pa",), &format!("Gebäude={support_id}, Vb={v_b:.0} N, ped={p_ed:.0} Pa, prd_red={p_rd:.0} Pa")));
        if p_ed > p_rd {
            let req_area = n_ed / p_rd.max(1.0) * 1.5;
            c = c.remedy(Remedy::at_least(SubjectRef::new(&f.id, format!("{path}.areaM2"), lc("Area", "Fläche")), Quantity::area_m2(f.area_m2), Quantity::area_m2(req_area), lc(&format!("Increase foundation area to at least {req_area:.2} m²."), &format!("Gründungsfläche auf mindestens {req_area:.2} m² erhöhen."))));
        }
        report.push(c.build());
        let h_ed = v_b;
        let gamma_rd = semio_s_plugin_norm_compliance::ndp_number("ndp.en1998.en5.sliding.gamma-rd", annex);
        let u_n = f.base_pore_pressure_pa * f.area_m2;
        let friction = part_5::sliding_friction_resistance_n(n_ed, u_n, f.base_friction_angle_deg, gamma_rd);
        let h_rd = friction + f.passive_resistance_n;
        let mut sld = CheckResult::assess(format!("en1998.5.{}.sliding", f.id), "DIN EN 1998-5", ClauseId::new("EN 1998-5", "5", "5.3"), SubjectRef::new(&f.id, format!("{path}.baseFrictionAngleDeg"), lc(&f.id, &f.id)), lc("Foundation sliding", "Gleiten der Gründung")).utilization(q_force(h_ed), floor_limit(q_force(h_rd))).annex(annex).ndps(&["ndp.en1998.en5.sliding.gamma-rd"]).explanation(lc(
            &format!("H_Ed=V_b={h_ed:.0} N from {support_id} against H_Rd=(V-u) tan(δ)/γ_Rd + passive={friction:.0}+{:.0} N with V={n_ed:.0} N, u={u_n:.0} N, δ={:.1} deg, γ_Rd={gamma_rd:.2}", f.passive_resistance_n, f.base_friction_angle_deg),
            &format!("H_Ed=V_b={h_ed:.0} N aus {support_id} gegen H_Rd=(V-u) tan(δ)/γ_Rd + passiv={friction:.0}+{:.0} N mit V={n_ed:.0} N, u={u_n:.0} N, δ={:.1} Grad, γ_Rd={gamma_rd:.2}", f.passive_resistance_n, f.base_friction_angle_deg),
        ));
        if h_ed > h_rd {
            sld = sld.remedy(Remedy::at_least(SubjectRef::new(&f.id, format!("{path}.passiveResistanceN"), lc("Passive resistance", "Passiver Widerstand")), q_force(f.passive_resistance_n), q_force((h_ed - friction).max(0.0)), lc("Credit more passive resistance at the sides of the footing.", "Mehr passiven Widerstand an den Seiten des Fundaments ansetzen.")));
            if let Some(delta) = part_5::required_friction_angle_deg(h_ed, f.passive_resistance_n, n_ed, u_n, gamma_rd).filter(|delta| *delta < 90.0) {
                sld = sld.remedy(Remedy::at_least(SubjectRef::new(&f.id, format!("{path}.baseFrictionAngleDeg"), lc("Interface friction angle δ", "Sohlreibungswinkel δ")), Quantity::new(QuantityKind::Dimensionless, f.base_friction_angle_deg), Quantity::new(QuantityKind::Dimensionless, delta), lc(&format!("Raise the base interface friction angle to {delta:.1} deg."), &format!("Sohlreibungswinkel auf {delta:.1} Grad erhöhen."))));
            }
        }
        report.push(sld.build());
        let xi = part_5::radiation_damping(part_5::stiffness_ratio(f.k_foundation, f.k_soil));
        let ratio = part_5::stiffness_ratio(f.k_foundation, f.k_soil);
        let ratio_min = 1.5;
        let mut rd = CheckResult::assess(
            format!("en1998.5.{}.radiationDamping", f.id),
            "DIN EN 1998-5",
            ClauseId::new("EN 1998-5", "5", "5.3"),
            SubjectRef::new(&f.id, format!("{path}.kFoundation"), lc(&f.id, &f.id)),
            lc("Rigid foundation k_f/k_s ≥ 1.5 for crediting radiation damping", "Starre Gründung k_f/k_s ≥ 1,5 für angerechnete Abstrahlungsdämpfung"),
        )
        .minimum(q_dim(ratio), q_dim(ratio_min))
        .annex(annex)
        .explanation(lc(&format!("k_f/k_s={ratio:.3}, ξ={xi:.3} from k_f/k_s",), &format!("k_f/k_s={ratio:.3}, ξ={xi:.3} aus k_f/k_s")));
        if ratio < ratio_min {
            rd = rd.remedy(Remedy::at_least(
                SubjectRef::new(&f.id, format!("{path}.kFoundation"), lc("k_foundation", "k_Gründung")),
                q_dim(f.k_foundation),
                q_dim(ratio_min * f.k_soil),
                lc("Stiffen the foundation to at least 1.5 times the soil stiffness before crediting radiation damping.", "Gründung auf mindestens das 1,5-fache der Bodensteifigkeit versteifen, bevor Abstrahlungsdämpfung angerechnet wird."),
            ));
        }
        report.push(rd.build());
    }
    if document.retaining_walls.is_empty() && scope.part("walls-na") {
        report.push(CheckResult::assess("en1998.5.wall.na", "DIN EN 1998-5", ClauseId::new("EN 1998-5", "5", "7.3.2"), SubjectRef::whole(lc("Retaining walls", "Stützwände")), lc("Retaining wall checks", "Stützwandnachweise")).not_applicable(lc("No retaining walls in the subject.", "Keine Stützwände im Gegenstand.")).annex(annex).build());
    }
    for (_, w) in document.retaining_walls.iter().enumerate().filter(|(index, _)| scope.admits("retainingWalls", *index)) {
        check_wall_thrust(report, annex, document, w, gamma_i, a_g, s);
    }
    if document.tie_beams.is_empty() && scope.part("tie-beams-na") {
        report.push(CheckResult::assess("en1998.5.tie-beam.na", "DIN EN 1998-5", ClauseId::new("EN 1998-5", "5", "5.4.1.2"), SubjectRef::whole(lc("Tie beams", "Fundamentbalken")), lc("Tie beam checks", "Fundamentbalkennachweise")).not_applicable(lc("No tie beams in the subject.", "Keine Fundamentbalken im Gegenstand.")).annex(annex).build());
    }
    for (_, t) in document.tie_beams.iter().enumerate().filter(|(index, _)| scope.admits("tieBeams", *index)) {
        check_tie_beam(report, annex, document, t, gamma_i, a_g, s);
    }
    if document.slopes.is_empty() && scope.part("slopes-na") {
        report.push(CheckResult::assess("en1998.5.slope.na", "DIN EN 1998-5", ClauseId::new("EN 1998-5", "5", "4.1.3.3"), SubjectRef::whole(lc("Slopes", "Böschungen")), lc("Slope checks", "Böschungsnachweise")).not_applicable(lc("No slopes in the subject.", "Keine Böschungen im Gegenstand.")).annex(annex).build());
    }
    for (_, sl) in document.slopes.iter().enumerate().filter(|(index, _)| scope.admits("slopes", *index)) {
        check_slope(report, annex, document, sl, gamma_i, a_g, s);
    }
}

fn foundation_gravity_n(document: &En1998Snapshot, f: &En1998Foundation) -> f64 {
    let support_id = if f.supported_building_id.is_empty() { document.buildings.first().map(|b| b.id.clone()).unwrap_or_default() } else { f.supported_building_id.clone() };
    building_gravity_and_height(document, &support_id).0
}

fn least_passing(mut lo: f64, mut hi: f64, passes: impl Fn(f64) -> bool) -> Option<f64> {
    if !passes(hi) {
        return None;
    }
    for _ in 0..16 {
        let mid = 0.5 * (lo + hi);
        if passes(mid) { hi = mid } else { lo = mid }
    }
    Some(hi)
}

fn check_tie_beam(report: &mut CheckReport, annex: AnnexChoice, document: &En1998Snapshot, t: &En1998TieBeam, gamma_i: f64, a_g: f64, s: f64) {
    use semio_s_plugin_norm_compliance::{ndp_number, CheckStatus};
    let path = id_sel("tieBeams", &t.id);
    let clause = || ClauseId::new("EN 1998-5", "5", "5.4.1.2");
    let ends = [(&t.foundation_a_id, "foundationAId", "First foundation", "Erstes Fundament"), (&t.foundation_b_id, "foundationBId", "Second foundation", "Zweites Fundament")];
    let foundation_ids: Vec<String> = document.foundations.iter().map(|f| f.id.clone()).collect();
    let mut resolved = true;
    for (id, field, en, de) in ends {
        let ok = foundation_ids.iter().any(|f| f == id);
        resolved &= ok;
        let mut c = CheckResult::assess(format!("en1998.5.{}.{}", t.id, field), "DIN EN 1998-5", clause(), SubjectRef::new(&t.id, format!("{path}.{field}"), lc(&t.id, &t.id)), lc(&format!("Tie beam end: {en}"), &format!("Fundamentbalken-Ende: {de}")))
            .status(if ok { CheckStatus::Pass } else { CheckStatus::Fail })
            .annex(annex)
            .explanation(lc(&format!("{field}={id}{}", if ok { " (resolved)" } else { " (dangling)" }), &format!("{field}={id}{}", if ok { " (aufgelöst)" } else { " (hängend)" })));
        if !ok {
            let options = if foundation_ids.is_empty() { vec![format!("{id}-missing")] } else { foundation_ids.clone() };
            c = c.remedy(Remedy::one_of(SubjectRef::new(&t.id, format!("{path}.{field}"), lc(en, de)), options, lc("Set the end to one of the existing foundation ids.", "Das Ende auf eine der vorhandenen Fundament-Ids setzen.")));
        }
        report.push(c.build());
    }
    let ground = document.site.en_ground_type.letter();
    let coefficient_id = match ground {
        'B' => Some("ndp.en1998.en5.tie.coefficient.b"),
        'C' => Some("ndp.en1998.en5.tie.coefficient.c"),
        'D' => Some("ndp.en1998.en5.tie.coefficient.d"),
        _ => None,
    };
    let subject = SubjectRef::new(&t.id, format!("{path}.steelArea"), lc(&t.id, &t.id));
    let title = lc("Tie beam force", "Fundamentbalkenkraft");
    let n_ed = document.foundations.iter().filter(|f| f.id == t.foundation_a_id || f.id == t.foundation_b_id).map(|f| foundation_gravity_n(document, f)).fold(0.0, f64::max);
    match (coefficient_id, resolved) {
        (None, _) => report.push(CheckResult::assess(format!("en1998.5.{}.tie", t.id), "DIN EN 1998-5", clause(), subject, title).not_applicable(lc(&format!("No tie force coefficient is registered for ground type {ground}."), &format!("Für die Baugrundklasse {ground} ist kein Zugkraftbeiwert registriert."))).annex(annex).build()),
        (Some(id), true) => {
            let coefficient = ndp_number(id, annex);
            let n = part_5::tie_force_n(coefficient, a_g * gamma_i, s, n_ed);
            let n_rd = part_5::tie_resistance_n(t.steel_area, t.steel_design_strength);
            let mut c = CheckResult::assess(format!("en1998.5.{}.tie", t.id), "DIN EN 1998-5", clause(), subject, title).utilization(q_force(n), floor_limit(q_force(n_rd))).annex(annex).ndps(&[id]).explanation(lc(
                &format!("N=±{coefficient:.2}·α·S·N_Ed={n:.0} N with ground type {ground}, N_Ed={n_ed:.0} N (larger supported weight of the two foundations), against N_Rd=A_s·f_yd={n_rd:.0} N"),
                &format!("N=±{coefficient:.2}·α·S·N_Ed={n:.0} N mit Baugrundklasse {ground}, N_Ed={n_ed:.0} N (größeres getragenes Gewicht der beiden Fundamente), gegen N_Rd=A_s·f_yd={n_rd:.0} N"),
            ));
            if n > n_rd {
                c = c.remedy(Remedy::at_least(SubjectRef::new(&t.id, format!("{path}.steelArea"), lc("Steel area", "Stahlquerschnitt")), Quantity::area_m2(t.steel_area), Quantity::area_m2(n / t.steel_design_strength.max(1.0)), lc("Increase the tie steel area.", "Stahlquerschnitt des Fundamentbalkens erhöhen.")));
            }
            report.push(c.build());
        }
        (Some(_), false) => {}
    }
    let mut d = CheckResult::assess(format!("en1998.5.{}.displacement", t.id), "DIN EN 1998-5", clause(), SubjectRef::new(&t.id, format!("{path}.displacementLimit"), lc(&t.id, &t.id)), lc("Tie beam relative displacement", "Relative Verschiebung des Fundamentbalkens")).utilization(q_len(t.relative_displacement.abs()), floor_limit(q_len(t.displacement_limit))).annex(annex).explanation(lc(
        &format!("d={:.4} m between the footings against the admissible {:.4} m over the span {:.2} m (the limit term of the repository spec is elided and therefore an input)", t.relative_displacement, t.displacement_limit, t.span),
        &format!("d={:.4} m zwischen den Fundamenten gegen zulässig {:.4} m auf der Spannweite {:.2} m (der Grenzterm der Spezifikation ist ausgelassen und daher eine Eingabe)", t.relative_displacement, t.displacement_limit, t.span),
    ));
    if t.relative_displacement.abs() > t.displacement_limit {
        d = d.remedy(Remedy::at_least(SubjectRef::new(&t.id, format!("{path}.displacementLimit"), lc("Displacement limit", "Verschiebungsgrenze")), q_len(t.displacement_limit), q_len(t.relative_displacement.abs()), lc("Design the tie beam for the larger relative displacement.", "Den Fundamentbalken für die größere Relativverschiebung bemessen.")));
    }
    report.push(d.build());
}

fn check_slope(report: &mut CheckReport, annex: AnnexChoice, document: &En1998Snapshot, sl: &En1998Slope, gamma_i: f64, a_g: f64, s: f64) {
    use semio_s_plugin_norm_compliance::CheckStatus;
    let path = id_sel("slopes", &sl.id);
    let clause = || ClauseId::new("EN 1998-5", "5", "4.1.3.3");
    let Some(forces) = part_5::slope_forces(document, sl, annex, a_g * gamma_i, s) else {
        let ids: Vec<String> = document.soil_layers.iter().map(|l| l.id.clone()).collect();
        let options = if ids.is_empty() { vec![format!("{}-missing", sl.governing_layer_id)] } else { ids };
        report.push(CheckResult::assess(format!("en1998.5.{}.slope", sl.id), "DIN EN 1998-5", clause(), SubjectRef::new(&sl.id, format!("{path}.governingLayerId"), lc(&sl.id, &sl.id)), lc("Seismic slope stability", "Seismische Böschungsstandsicherheit")).status(CheckStatus::Fail).annex(annex).explanation(lc(&format!("governingLayerId={} is not a soil layer: the unit weight of the slope is unknown.", sl.governing_layer_id), &format!("governingLayerId={} ist keine Bodenschicht: die Wichte der Böschung ist unbekannt.", sl.governing_layer_id))).remedy(Remedy::one_of(SubjectRef::new(&sl.id, format!("{path}.governingLayerId"), lc("Governing soil layer", "Maßgebende Bodenschicht")), options, lc("Set the governing layer to one of the existing soil layers.", "Die maßgebende Schicht auf eine der vorhandenen Bodenschichten setzen."))).build());
        return;
    };
    let avg_id = if document.site.en_spectrum_type == EnSpectrumType::Type2 { "ndp.en1998.vertical-spectrum.avg-ratio-type2" } else { "ndp.en1998.vertical-spectrum.avg-ratio" };
    let ids = ["ndp.en1998.en5.slope.horizontal-factor", avg_id, "ndp.en1998.en5.vertical-force.threshold", "ndp.en1998.en5.vertical-force.ratio-high", "ndp.en1998.en5.vertical-force.ratio-low", "ndp.en1997.m2.phi", "ndp.en1997.m2.c"];
    let (gamma_phi, gamma_c, topographic, share) = (forces.gamma_phi, forces.gamma_c, forces.topographic, forces.vertical_share);
    let subject = SubjectRef::new(&sl.id, format!("{path}.frictionAngle"), lc(&sl.id, &sl.id));
    let title = lc("Seismic slope stability", "Seismische Böschungsstandsicherheit");
    let result = part_5::slope_result(&forces, sl, sl.height, sl.angle, sl.friction_angle);
    let Some(fos) = result.least() else {
        report.push(CheckResult::assess(format!("en1998.5.{}.slope", sl.id), "DIN EN 1998-5", clause(), subject, title).status(CheckStatus::Pass).annex(annex).ndps(&ids).explanation(lc("No circle develops a driving moment: the slope is stable under the pseudo-static forces.", "Kein Kreis entwickelt ein treibendes Moment: die Böschung ist unter den pseudostatischen Kräften standsicher.")).build());
        return;
    };
    let (cx, cz, cr) = result.critical.as_ref().map_or((f64::NAN, f64::NAN, f64::NAN), |(c, _)| (c.xc, c.yc, c.r));
    let given = result.fixed.as_ref().map(|(_, f)| *f);
    let mut c = CheckResult::assess(format!("en1998.5.{}.slope", sl.id), "DIN EN 1998-5", clause(), subject, title).minimum(q_dim(fos), q_dim(1.0)).annex(annex).ndps(&ids).explanation(lc(
        &format!("F_s={fos:.3} ≥ 1.0 (Bishop, M2 γ_φ={gamma_phi:.2}, γ_c={gamma_c:.2}) under F_H={:.3} W and F_V=±{share:.2} F_H with S_T={topographic:.2}; given circle {}, critical circle centre ({cx:.2}, {cz:.2}) m, radius {cr:.2} m", forces.pseudostatic.horizontal, given.map_or("without sliding mass".to_owned(), |f| format!("F_s={f:.3}"))),
        &format!("F_s={fos:.3} ≥ 1,0 (Bishop, M2 γ_φ={gamma_phi:.2}, γ_c={gamma_c:.2}) unter F_H={:.3} W und F_V=±{share:.2} F_H mit S_T={topographic:.2}; gegebener Kreis {}, kritischer Kreis Mittelpunkt ({cx:.2}, {cz:.2}) m, Radius {cr:.2} m", forces.pseudostatic.horizontal, given.map_or("ohne Gleitmasse".to_owned(), |f| format!("F_s={f:.3}"))),
    ));
    let stable = |height: f64, angle: f64, phi: f64| part_5::slope_result(&forces, sl, height, angle, phi).least().is_none_or(|f| f >= 1.0);
    if fos < 1.0 {
        if let Some(phi) = least_passing(sl.friction_angle, 89.0, |phi| stable(sl.height, sl.angle, phi)) {
            c = c.remedy(Remedy::at_least(SubjectRef::new(&sl.id, format!("{path}.frictionAngle"), lc("Friction angle", "Reibungswinkel")), Quantity::new(QuantityKind::Dimensionless, sl.friction_angle), Quantity::new(QuantityKind::Dimensionless, phi), lc(&format!("Raise the friction angle to {phi:.1} degrees."), &format!("Reibungswinkel auf {phi:.1} Grad erhöhen."))));
        }
        if let Some(negated) = least_passing(-sl.angle, -1.0, |x| stable(sl.height, -x, sl.friction_angle)) {
            c = c.remedy(Remedy::at_most(SubjectRef::new(&sl.id, format!("{path}.angle"), lc("Slope angle", "Böschungswinkel")), Quantity::new(QuantityKind::Dimensionless, sl.angle), Quantity::new(QuantityKind::Dimensionless, -negated), lc(&format!("Flatten the slope to at most {:.1} degrees.", -negated), &format!("Böschung auf höchstens {:.1} Grad abflachen.", -negated))));
        }
    }
    report.push(c.build());
}

pub(super) fn evaluate_towers(report: &mut CheckReport, document: &En1998Snapshot, annex_params: &AnnexParams, gamma_i: f64, a_g: f64, s: f64, tb: f64, tc: f64, td: f64, scope: Scope<'_>) {
    let annex = annex_params.choice();
    if document.towers.is_empty() && scope.part("towers-na") {
        report.push(CheckResult::assess("en1998.6.na", "DIN EN 1998-6", ClauseId::new("EN 1998-6", "6", "4.3"), SubjectRef::whole(lc("Towers", "Türme")), lc("Tower checks", "Turmnachweise")).not_applicable(lc("No towers in the subject.", "Keine Türme im Gegenstand.")).annex(annex).build());
        return;
    }
    for (_, t) in document.towers.iter().enumerate().filter(|(index, _)| scope.admits("towers", *index)) {
        let q = part_6::tower_behaviour_factor(t.q_nominal, t.is_chimney);
        let mass = part_1::entity_seismic_mass_kg(t.permanent_gk_n, &t.variables, false, t.correlated_occupancy);
        let t_read = tower_period(report, annex, t, mass, tc);
        check_tower_modal(report, annex, t, mass, &Ground { a_g, s, tb, tc, td, gamma_i }, q);
        let s_d = part_1::design_spectrum_at(a_g, gamma_i, s, tb, tc, td, q, t_read);
        let v = part_6::tower_base_shear_n(part_6::cantilever_force_factor(), s_d, mass);
        let m_ed = part_6::tower_overturning_moment_nm(v, t.height_m);
        let path = id_sel("towers", &t.id);
        let mut c = CheckResult::assess(format!("en1998.6.{}.overturning", t.id), "DIN EN 1998-6", ClauseId::new("EN 1998-6", "6", "4.3.2"), SubjectRef::new(&t.id, format!("{path}.mRdNm"), lc(&t.id, &t.id)), lc("Tower overturning moment", "Turm-Kippmoment")).utilization(q_mom(m_ed), floor_limit(q_mom(t.m_rd_nm))).annex(annex).explanation(lc(
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

fn check_wall_thrust(report: &mut CheckReport, annex: AnnexChoice, document: &En1998Snapshot, w: &En1998RetainingWall, gamma_i: f64, a_g: f64, s: f64) {
    use semio_s_plugin_norm_compliance::{ndp_common, ndp_number};
    let path = id_sel("retainingWalls", &w.id);
    let k_h = part_5::horizontal_seismic_coefficient(a_g * gamma_i, s, w.r);
    let avg_id = if document.site.en_spectrum_type == EnSpectrumType::Type2 { "ndp.en1998.vertical-spectrum.avg-ratio-type2" } else { "ndp.en1998.vertical-spectrum.avg-ratio" };
    let avg = ndp_common(avg_id);
    let share = part_5::vertical_force_ratio(avg, ndp_number("ndp.en1998.en5.vertical-force.threshold", annex), ndp_number("ndp.en1998.en5.vertical-force.ratio-high", annex), ndp_number("ndp.en1998.en5.vertical-force.ratio-low", annex));
    let water = Water { height_m: w.water_height_m, saturated_unit_weight: w.saturated_gamma, permeable: w.permeability_m_s >= ndp_number("ndp.en1998.en5.annex-e.permeability-limit", annex) };
    let interface = Interface::vertical(w.phi_deg, 0.0);
    let ndps = [avg_id, "ndp.en1998.en5.vertical-force.threshold", "ndp.en1998.en5.vertical-force.ratio-high", "ndp.en1998.en5.vertical-force.ratio-low", "ndp.en1998.en5.annex-e.permeability-limit"];
    let subject = SubjectRef::new(&w.id, format!("{path}.hRdNPerM"), lc(&w.id, &w.id));
    let title = lc("Retaining wall thrust", "Stützwandschub");
    let Some(load) = thrust(Side::Active, &interface, w.soil_gamma, w.height_m, water, Acceleration { kh: k_h, kv: share * k_h }) else {
        let required = (1..=400).map(|i| w.r + 0.05 * f64::from(i)).find(|r| thrust(Side::Active, &interface, w.soil_gamma, w.height_m, water, Acceleration { kh: part_5::horizontal_seismic_coefficient(a_g * gamma_i, s, *r), kv: share * part_5::horizontal_seismic_coefficient(a_g * gamma_i, s, *r) }).is_some()).unwrap_or(w.r * 2.0);
        report.push(CheckResult::assess(format!("en1998.5.{}.thrust", w.id), "DIN EN 1998-5", ClauseId::new("EN 1998-5", "5", "E.2"), subject, title).status(semio_s_plugin_norm_compliance::CheckStatus::Fail).annex(annex).ndps(&ndps).explanation(lc(
            &format!("k_h={k_h:.3} with k_v={:.3} leaves no limit equilibrium of the backfill (θ > φ′): the Mononobe-Okabe thrust does not exist.", share * k_h),
            &format!("k_h={k_h:.3} mit k_v={:.3} lässt kein Grenzgleichgewicht der Hinterfüllung zu (θ > φ′): der Mononobe-Okabe-Erddruck existiert nicht.", share * k_h),
        )).remedy(Remedy::at_least(SubjectRef::new(&w.id, format!("{path}.r"), lc("Factor r", "Beiwert r")), Quantity::new(QuantityKind::Dimensionless, w.r), Quantity::new(QuantityKind::Dimensionless, required), lc(&format!("Raise the factor r to {required:.2} so that k_h leaves a limit equilibrium."), &format!("Beiwert r auf {required:.2} erhöhen, damit k_h ein Grenzgleichgewicht zulässt.")))).build());
        return;
    };
    let h_ed = load.total();
    let mut c = CheckResult::assess(format!("en1998.5.{}.thrust", w.id), "DIN EN 1998-5", ClauseId::new("EN 1998-5", "5", "E.2"), subject, title).utilization(q_force(h_ed), floor_limit(q_force(w.h_rd_n_per_m))).annex(annex).ndps(&ndps).explanation(lc(
        &format!("H_Ed={h_ed:.0} N/m (soil {:.0}, water {:.0}) from k_h={k_h:.3}, k_v={:.3}, K_AE={:.3}, H_Rd={:.0} N/m", load.soil, load.water(), share * k_h, load.coefficient, w.h_rd_n_per_m),
        &format!("H_Ed={h_ed:.0} N/m (Boden {:.0}, Wasser {:.0}) aus k_h={k_h:.3}, k_v={:.3}, K_AE={:.3}, H_Rd={:.0} N/m", load.soil, load.water(), share * k_h, load.coefficient, w.h_rd_n_per_m),
    ));
    if h_ed > w.h_rd_n_per_m {
        c = c.remedy(Remedy::at_least(SubjectRef::new(&w.id, format!("{path}.hRdNPerM"), lc("H_Rd", "H_Rd")), q_force(w.h_rd_n_per_m), q_force(h_ed), lc("Increase wall sliding resistance.", "Wand-Gleitwiderstand erhöhen.")));
    }
    report.push(c.build());
    if w.water_height_m > 0.0 {
        let wet = w.water_height_m.min(w.height_m);
        let mut water_check = CheckResult::assess(format!("en1998.5.{}.water", w.id), "DIN EN 1998-5", ClauseId::new("EN 1998-5", "5", "E.8"), SubjectRef::new(&w.id, format!("{path}.waterHeightM"), lc(&w.id, &w.id)), lc("Hydrostatic and hydrodynamic water thrust", "Hydrostatischer und hydrodynamischer Wasserschub")).utilization(q_force(load.water()), floor_limit(q_force(w.h_rd_n_per_m))).annex(annex).ndps(&ndps).explanation(lc(
            &format!("{} backfill, water height {wet:.2} m: hydrostatic {:.0} N/m, Westergaard {:.0} N/m (7/12 k_h γ_w H² acting at {:.2} m above the base) against H_Rd={:.0} N/m", if water.permeable { "Permeable" } else { "Low permeability" }, load.water_static, load.water_dynamic, WESTERGAARD_ARM * wet, w.h_rd_n_per_m),
            &format!("{}e Hinterfüllung, Wasserhöhe {wet:.2} m: hydrostatisch {:.0} N/m, Westergaard {:.0} N/m (7/12 k_h γ_w H² in {:.2} m über der Sohle) gegen H_Rd={:.0} N/m", if water.permeable { "Durchlässig" } else { "Gering durchlässig" }, load.water_static, load.water_dynamic, WESTERGAARD_ARM * wet, w.h_rd_n_per_m),
        ));
        if load.water() > w.h_rd_n_per_m {
            water_check = water_check.remedy(Remedy::at_most(SubjectRef::new(&w.id, format!("{path}.waterHeightM"), lc("Retained water height", "Angestaute Wasserhöhe")), Quantity::length_m(w.water_height_m), Quantity::length_m(wet * (w.h_rd_n_per_m / load.water()).sqrt()), lc("Lower the retained water, for example by drainage.", "Das angestaute Wasser absenken, zum Beispiel durch Dränung.")));
        }
        report.push(water_check.build());
    }
}
