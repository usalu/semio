//! 🏢️ Building regularity and base shear checks: the building mass, period, regularity and lateral force method checks of the full seismic check. See [`super`].

use super::*;

/// 🏢 Checks the building masses, the fundamental period, the regularity and the lateral force method of one building.
pub(super) fn check_building_base_shear(
    annex: AnnexChoice,
    report: &mut CheckReport,
    s: f64,
    tc: f64,
    building: &semio_s_artifact_norm_en1998_model::En1998Building,
    bid: &str,
    bpath: &str,
    blabel: &LocalizedCopy,
) -> (Vec<f64>, Vec<f64>, f64, f64) {
    let total_height: f64 = building.storeys.iter().map(|s| s.height_m).sum();
    let n_storeys = building.storeys.len();
    let masses: Vec<f64> = building.storeys.iter().enumerate().map(|(i, s)| {
        part_1::storey_seismic_mass_kg(s, i + 1 == n_storeys)
    }).collect();
    let heights: Vec<f64> = building.storeys.iter().map(|s| s.height_m).collect();
    let total_mass: f64 = masses.iter().sum();

    let (stiff_x, stiff_y) = cracked_stiffness::storeys_xy(building);
    let (t1, t1_input_ok) = part_1::fundamental_period_s(building.t1_method, building.ct, building.t1_given_s, total_height, &masses, &stiff_x, &stiff_y);
    if !t1_input_ok && building.t1_method == semio_s_artifact_norm_en1998_model::T1Method::Rayleigh {
        report.push(
            CheckResult::assess(
                format!("en1998.1.{bid}.t1Rayleigh"),
                "DIN EN 1998-1",
                ClauseId::new("EN 1998-1", "1", "4.3.3.2.2"),
                SubjectRef::new(bid, format!("{bpath}.t1Method"), blabel.clone()),
                lc("Rayleigh period needs storey stiffnesses", "Rayleigh-Periode braucht Geschosssteifigkeiten"),
            )
            .status(semio_s_plugin_norm_compliance::CheckStatus::Fail)
            .annex(annex)
            .explanation(lc(
                "Method rayleigh needs positive storey stiffnesses in x or y; the C_t estimate is used for the remaining checks.",
                "Das Verfahren rayleigh braucht positive Geschosssteifigkeiten in x oder y; für die übrigen Nachweise gilt die C_t-Schätzung.",
            ))
            .remedy(Remedy::one_of(
                SubjectRef::new(bid, format!("{bpath}.t1Method"), lc("T1 method", "T1-Verfahren")),
                vec!["ct".into(), "given".into()],
                lc("Use method ct or given, or provide storey stiffnesses.", "Verfahren ct oder given wählen oder Geschosssteifigkeiten angeben."),
            ))
            .build(),
        );
    }

    {
        let using_given = building.t1_method == semio_s_artifact_norm_en1998_model::T1Method::Given;
        let t1_ct = part_1::t1_from_ct(building.ct, total_height);
        let (computed, limit) = if using_given { (building.t1_given_s, 1.5 * t1_ct) } else { (building.t1_given_s, 0.05) };
        let mut tg = CheckResult::assess(
            format!("en1998.1.{bid}.t1Given"),
            "DIN EN 1998-1",
            ClauseId::new("EN 1998-1", "1", "4.3.3.2.2"),
            SubjectRef::new(bid, format!("{bpath}.t1GivenS"), blabel.clone()),
            lc("Given fundamental period T₁ against the C_t estimate", "Vorgegebene Eigenperiode T₁ gegen die C_t-Schätzung"),
        )
        .utilization(q_len(computed), q_len(limit.max(1e-9)))
        .annex(annex)
        .explanation(lc(
            &format!("t1Method={}, t1Given={:.3} s, T1(Ct)={t1_ct:.3} s, T1 used={t1:.3} s with storey stiffness {}; a given period must lie in (0, 1.5·T1(Ct)]", building.t1_method.as_str(), building.t1_given_s, cracked_stiffness::building_basis_text(building).0),
            &format!("t1Verfahren={}, t1Given={:.3} s, T1(Ct)={t1_ct:.3} s, T1 verwendet={t1:.3} s mit Geschosssteifigkeit {}; vorgegebene Periode muss in (0, 1,5·T1(Ct)] liegen", building.t1_method.as_str(), building.t1_given_s, cracked_stiffness::building_basis_text(building).1),
        ));
        if building.storey_stiffness_basis == semio_s_artifact_norm_en1998_model::StiffnessBasis::Declared && !cracked_stiffness::is_valid_ratio(building.storey_stiffness_ratio) {
            tg = tg.status(semio_s_plugin_norm_compliance::CheckStatus::Fail).remedy(Remedy::one_of(
                SubjectRef::new(bid, format!("{bpath}.storeyStiffnessBasis"), lc("Stiffness basis", "Steifigkeitsbasis")),
                vec!["simplified".into(), "declared".into()],
                lc("A declared effective stiffness share must lie in (0, 1]: correct storeyStiffnessRatio or switch the basis to simplified.", "Ein angegebener wirksamer Steifigkeitsanteil muss in (0, 1] liegen: storeyStiffnessRatio korrigieren oder die Basis auf vereinfacht stellen."),
            ));
        } else if using_given && building.t1_given_s <= 0.0 {
            tg = tg.status(semio_s_plugin_norm_compliance::CheckStatus::Fail).remedy(Remedy::at_least(
                SubjectRef::new(bid, format!("{bpath}.t1GivenS"), lc("Given T1", "Vorgegebenes T1")),
                q_len(building.t1_given_s),
                q_len(t1_ct),
                lc("Provide a positive given period or switch t1Method to ct.", "Positive Eigenperiode vorgeben oder t1Verfahren auf ct stellen."),
            ));
        } else if using_given && building.t1_given_s > limit {
            tg = tg.remedy(Remedy::at_most(
                SubjectRef::new(bid, format!("{bpath}.t1GivenS"), lc("Given T1", "Vorgegebenes T1")),
                q_len(building.t1_given_s),
                q_len(limit),
                lc("Reduce the given period to at most 1.5·T1(Ct) or justify with a Rayleigh analysis.", "Vorgegebene Periode auf höchstens 1,5·T1(Ct) senken oder per Rayleigh-Analyse belegen."),
            ));
        } else if !using_given && building.t1_given_s > 0.05 {
            tg = tg.status(semio_s_plugin_norm_compliance::CheckStatus::Fail).remedy(Remedy::at_most(
                SubjectRef::new(bid, format!("{bpath}.t1GivenS"), lc("Given T1", "Vorgegebenes T1")),
                q_len(building.t1_given_s),
                q_len(0.0),
                lc("Clear t1GivenS when t1Method is not given.", "t1GivenS leeren, wenn t1Method nicht given ist."),
            ));
        }
        report.push(tg.build());
    }

    report.push({
        let ok = building.plan_regular && building.elevation_regular;
        let mut b = CheckResult::assess(
            format!("en1998.1.{bid}.regularity.claim"),
            "DIN EN 1998-1",
            ClauseId::new("EN 1998-1", "1", "4.2.3"),
            SubjectRef::new(bid, format!("{bpath}.planRegular"), blabel.clone()),
            lc("Claimed regularity in plan and elevation", "Beanspruchte Regelmässigkeit im Grund- und Aufriss"),
        )
        .status(if ok { semio_s_plugin_norm_compliance::CheckStatus::Pass } else { semio_s_plugin_norm_compliance::CheckStatus::Fail })
        .annex(annex)
        .explanation(lc(
            &format!("planRegular={}, elevationRegular={}", building.plan_regular, building.elevation_regular),
            &format!("Grundrissregelmässigkeit={}, Aufrissregelmässigkeit={}", building.plan_regular, building.elevation_regular),
        ));
        if !building.plan_regular {
            b = b.remedy(Remedy::exactly(
                SubjectRef::new(bid, format!("{bpath}.planRegular"), lc("Plan regularity", "Regelmässigkeit im Grundriss")),
                q_dim(0.0),
                q_dim(1.0),
                lc("Restore plan regularity (or use modal analysis).", "Regelmässigkeit im Grundriss herstellen (oder Modale Analyse verwenden)."),
            ));
        }
        if !building.elevation_regular {
            b = b.remedy(Remedy::exactly(
                SubjectRef::new(bid, format!("{bpath}.elevationRegular"), lc("Elevation regularity", "Regelmässigkeit im Aufriss")),
                q_dim(0.0),
                q_dim(1.0),
                lc("Restore elevation regularity (or use modal analysis).", "Regelmässigkeit im Aufriss herstellen (oder Modale Analyse verwenden)."),
            ));
        }
        b.build()
    });

    if building.plan_width_m > 0.0 && building.plan_length_m > 0.0 {
        let walls = wall_plan::placed_walls_of(&building.members);
        for storey in &building.storeys {
            let wall_plan_of_storey = wall_plan::wall_plan_of_storey(&walls, storey);
            let (e0x, e0y, rx, ry, ls) = match &wall_plan_of_storey {
                Some(plan) => wall_plan::plan_metrics(plan, storey.centre_of_mass_x_m, storey.centre_of_mass_y_m, building.plan_width_m, building.plan_length_m),
                None => part_1::plan_regularity_metrics(
                    storey.centre_of_mass_x_m,
                    storey.centre_of_mass_y_m,
                    storey.centre_of_stiffness_x_m,
                    storey.centre_of_stiffness_y_m,
                    cracked_stiffness::storey_x(building, storey),
                    cracked_stiffness::storey_y(building, storey),
                    building.plan_width_m,
                    building.plan_length_m,
                ),
            };
            let (k_x, k_y) = wall_plan_of_storey.as_ref().map_or((cracked_stiffness::storey_x(building, storey), cracked_stiffness::storey_y(building, storey)), |plan| (plan.stiffness_x, plan.stiffness_y));
            let (cs_x, cs_y) = wall_plan_of_storey.as_ref().map_or((storey.centre_of_stiffness_x_m, storey.centre_of_stiffness_y_m), |plan| (plan.centre_of_stiffness_x_m, plan.centre_of_stiffness_y_m));
            let source = |de: bool| match (&wall_plan_of_storey, de) {
                (Some(plan), false) => format!("stiffness centre ({cs_x:.3}, {cs_y:.3}) m and radii from {} positioned wall rows (cantilever bending and shear, ν = {}, stiffness {}), kx/E={k_x:.4} m, ky/E={k_y:.4} m", plan.wall_count, plan.poisson_ratio_text(), plan.stiffness_factor_text()),
                (Some(plan), true) => format!("Steifigkeitsmittelpunkt ({cs_x:.3}, {cs_y:.3}) m und Radien aus {} angeordneten Wandzeilen (Kragarm mit Biegung und Schub, ν = {}, Steifigkeit {}), kx/E={k_x:.4} m, ky/E={k_y:.4} m", plan.wall_count, plan.poisson_ratio_text(), plan.stiffness_factor_text()),
                (None, false) => format!("stiffness centre and storey stiffnesses as entered, kx={k_x:.0}, ky={k_y:.0}"),
                (None, true) => format!("Steifigkeitsmittelpunkt und Geschosssteifigkeiten wie eingegeben, kx={k_x:.0}, ky={k_y:.0}"),
            };
            let r_min = rx.min(ry);
            let u_e = part_1::plan_eccentricity_utilization(e0x, e0y, rx, ry);
            let y_governs = e0y / ry.max(1e-9) > e0x / rx.max(1e-9);
            let stpath = format!("{}.storeys[id={}]", bpath, storey.id);
            let mut ch = CheckResult::assess(
                format!("en1998.1.{bid}.storey.{}.planRegularity", storey.id),
                "DIN EN 1998-1",
                ClauseId::new("EN 1998-1", "1", "4.2.3.2"),
                SubjectRef::new(bid, format!("{stpath}.centreOfMassXM"), lc(&storey.id, &storey.id)),
                lc("Plan regularity e₀ ≤ 0.30 r and r ≥ ℓₛ", "Grundrissregelmässigkeit e₀ ≤ 0.30 r und r ≥ ℓₛ"),
            )
            .utilization(q_dim(if r_min <= 0.0 { 1e9 } else { u_e.max(ls / r_min).min(1e9) }), q_dim(1.0))
            .annex(annex)
            .explanation(lc(
                &format!("e0x={e0x:.3} m, e0y={e0y:.3} m, rx={rx:.3} m, ry={ry:.3} m, ls={ls:.3} m, {}", source(false)),
                &format!("e0x={e0x:.3} m, e0y={e0y:.3} m, rx={rx:.3} m, ry={ry:.3} m, ℓₛ={ls:.3} m, {}", source(true)),
            ));
            if u_e > 1.0 || ls > r_min {
                let (axis, field, com, cos) = if y_governs { ("y", "centreOfMassYM", storey.centre_of_mass_y_m, cs_y) } else { ("x", "centreOfMassXM", storey.centre_of_mass_x_m, cs_x) };
                ch = ch.remedy(Remedy::at_most(
                    SubjectRef::new(bid, format!("{stpath}.{field}"), lc(&format!("Centre of mass {axis}"), &format!("Massenschwerpunkt {axis}"))),
                    q_len(com),
                    q_len(cos),
                    lc("Reduce structural eccentricity e₀ (align CM and CS) or increase torsional stiffness.", "Strukturelle Exzentrizität e₀ verringern (SM und SS ausrichten) oder Torsionssteifigkeit erhöhen."),
                ));
            }
            report.push(ch.build());
        }
    }

    {
        let n = building.storeys.len();
        if n >= 2 {
            for i in 0..n - 1 {
                let lo = &building.storeys[i];
                let hi = &building.storeys[i + 1];
                let m_lo = part_1::storey_seismic_mass_kg(lo, false);
                let m_hi = part_1::storey_seismic_mass_kg(hi, i + 2 == n);
                let u_x = part_1::soft_storey_utilization(lo.stiffness_x, hi.stiffness_x);
                let u_y = part_1::soft_storey_utilization(lo.stiffness_y, hi.stiffness_y);
                let m_ratio = if m_lo > 0.0 { m_hi / m_lo } else { 0.0 };
                let util = u_x.max(u_y).max(m_ratio / 1.50).min(1e9);
                let mut ch = CheckResult::assess(
                    format!("en1998.1.{bid}.elevationRegularity.{}-{}", lo.id, hi.id),
                    "DIN EN 1998-1",
                    ClauseId::new("EN 1998-1", "1", "4.2.3.3"),
                    SubjectRef::new(bid, format!("{}.storeys[id={}].stiffnessX", bpath, lo.id), lc(&lo.id, &lo.id)),
                    lc("Elevation regularity (soft storey, mass variation)", "Aufrissregelmässigkeit (weiches Geschoss, Masseänderung)"),
                )
                .utilization(q_dim(util), q_dim(1.0))
                .annex(annex)
                .explanation(lc(
                    &format!("0.70·k_hi/k_lo: x={u_x:.3}, y={u_y:.3} (≤1: lower storey not softer than 0.70 of the upper), m_hi/m_lo={m_ratio:.3} (≤1.50)"),
                    &format!("0,70·k_oben/k_unten: x={u_x:.3}, y={u_y:.3} (≤1: unteres Geschoss nicht weicher als 0,70 des oberen), m_oben/m_unten={m_ratio:.3} (≤1,50)"),
                ));
                if util > 1.0 {
                    let (field, path_field, current, required) = if u_x >= u_y {
                        ("Storey stiffness X", "stiffnessX", lo.stiffness_x, 0.70 * hi.stiffness_x)
                    } else {
                        ("Storey stiffness Y", "stiffnessY", lo.stiffness_y, 0.70 * hi.stiffness_y)
                    };
                    ch = ch.remedy(Remedy::at_least(
                        SubjectRef::new(bid, format!("{}.storeys[id={}].{path_field}", bpath, lo.id), lc(field, field)),
                        q_dim(current),
                        q_dim(required),
                        lc("Increase the lower-storey stiffness (avoid soft storey) or smooth the mass variation.", "Steifigkeit des unteren Geschosses erhöhen (weiches Geschoss vermeiden) oder Masseübergang glätten."),
                    ));
                }
                report.push(ch.build());
            }
        }
    }

    {
        let mut c = CheckResult::assess(
            format!("en1998.1.{bid}.planDims"),
            "DIN EN 1998-1",
            ClauseId::new("EN 1998-1", "1", "4.3.2"),
            SubjectRef::new(bid, format!("{bpath}.planWidthM"), blabel.clone()),
            lc("Plan dimensions Lx, Ly", "Grundrissabmessungen Lx, Ly"),
        )
        .minimum(q_len(building.plan_width_m.min(building.plan_length_m)), q_len(0.01))
        .annex(annex)
        .explanation(lc(
            &format!("plan width Lx={:.2} m, plan length Ly={:.2} m", building.plan_width_m, building.plan_length_m),
            &format!("Grundrissbreite Lx={:.2} m, Grundrisslänge Ly={:.2} m", building.plan_width_m, building.plan_length_m),
        ));
        if building.plan_width_m <= 0.0 {
            c = c.remedy(Remedy::at_least(
                SubjectRef::new(bid, format!("{bpath}.planWidthM"), lc("Plan width Lx", "Grundrissbreite Lx")),
                q_len(building.plan_width_m),
                q_len(1.0),
                lc("Provide a positive plan width Lx.", "Positive Grundrissbreite Lx angeben."),
            ));
        }
        if building.plan_length_m <= 0.0 {
            c = c.remedy(Remedy::at_least(
                SubjectRef::new(bid, format!("{bpath}.planLengthM"), lc("Plan length Ly", "Grundrisslänge Ly")),
                q_len(building.plan_length_m),
                q_len(1.0),
                lc("Provide a positive plan length Ly.", "Positive Grundrisslänge Ly angeben."),
            ));
        }
        report.push(c.build());
    }

    {
        let has_frame = building.systems.iter().any(|s| matches!(s.system_type, semio_s_artifact_norm_en1998_model::SystemType::Frame | semio_s_artifact_norm_en1998_model::SystemType::Dual));
        let has_wall = building.systems.iter().any(|s| matches!(s.system_type, semio_s_artifact_norm_en1998_model::SystemType::Wall | semio_s_artifact_norm_en1998_model::SystemType::Dual));
        let dual_ok = has_frame && has_wall;
        let tor_ok = building.plan_regular;
        let claimed = building.multiple_resisting_systems;
        let ok = if claimed { dual_ok && tor_ok } else { true };
        let mut c = CheckResult::assess(
            format!("en1998.1.{bid}.multipleSystems"),
            "DIN EN 1998-1",
            ClauseId::new("EN 1998-1", "1", "5.2.2.1"),
            SubjectRef::new(bid, format!("{bpath}.multipleResistingSystems"), blabel.clone()),
            lc("Multiple resisting systems / dual classification", "Mehrere Aussteifungssysteme / Dualklassifikation"),
        )
        .status(if ok { semio_s_plugin_norm_compliance::CheckStatus::Pass } else { semio_s_plugin_norm_compliance::CheckStatus::Fail })
        .annex(annex)
        .explanation(lc(
            &format!("multipleResistingSystems={claimed}, frame={has_frame}, wall={has_wall}, planRegular={}", building.plan_regular),
            &format!("mehrere Aussteifungssysteme={claimed}, Rahmen={has_frame}, Wand={has_wall}, Grundrissregelmässigkeit={}", building.plan_regular),
        ));
        if claimed && !dual_ok {
            c = c.remedy(Remedy::exactly(
                SubjectRef::new(bid, format!("{bpath}.multipleResistingSystems"), lc("Multiple resisting systems", "Mehrere Aussteifungssysteme")),
                q_dim(1.0),
                q_dim(0.0),
                lc("Clear multipleResistingSystems or provide frame+wall (dual) systems per §5.2.2.1.", "multipleResistingSystems deaktivieren oder Rahmen+Wand (Dual) nach §5.2.2.1 vorsehen."),
            ));
        }
        if claimed && !tor_ok {
            c = c.remedy(Remedy::exactly(
                SubjectRef::new(bid, format!("{bpath}.planRegular"), lc("Plan regularity", "Regelmässigkeit im Grundriss")),
                q_dim(0.0),
                q_dim(1.0),
                lc("Restore plan regularity for multi-system torsional flexibility (§4.2.3.1).", "Grundrissregelmässigkeit für Torsion bei Mehrfachsystemen herstellen (§4.2.3.1)."),
            ));
        }
        report.push(c.build());
    }

    let lfm_ok = part_1::lateral_force_method_applicable(t1, tc, building.elevation_regular);
    let mut lfm = CheckResult::assess(
        format!("en1998.1.{bid}.lfm"),
        "DIN EN 1998-1",
        ClauseId::new("EN 1998-1", "1", "4.3.3.2"),
        SubjectRef::new(bid, format!("{bpath}.elevationRegular"), blabel.clone()),
        lc("Lateral force method applicability", "Anwendbarkeit des Kraftgrössenverfahrens"),
    )
    .status(if lfm_ok { semio_s_plugin_norm_compliance::CheckStatus::Pass } else { semio_s_plugin_norm_compliance::CheckStatus::Warning })
    .annex(annex)
    .explanation(lc(
        &format!("T1={t1:.3}s (method={}, t1Given={:.3}s), TC={tc:.3}s, elevationRegular={}", building.t1_method.as_str(), building.t1_given_s, building.elevation_regular),
        &format!("T1={t1:.3}s (Verfahren={}, t1Given={:.3}s), TC={tc:.3}s, Aufrissregelmässigkeit={}", building.t1_method.as_str(), building.t1_given_s, building.elevation_regular),
    ));
    if !lfm_ok {
        let subject = SubjectRef::new(bid, format!("{bpath}.elevationRegular"), blabel.clone());
        lfm = lfm.remedy(if building.elevation_regular {
            Remedy { applicable: false, ..Remedy::one_of(subject, Vec::new(), lc("Stiffen the structure so that T1 does not exceed 4 TC and 2.0 s, or use the modal response spectrum analysis instead of the lateral force method.", "Das Tragwerk so aussteifen, dass T1 weder 4 TC noch 2,0 s überschreitet, oder statt des Kraftgrössenverfahrens das multimodale Antwortspektrenverfahren verwenden.")) }
        } else {
            Remedy::exactly(subject, q_dim(0.0), q_dim(1.0), lc("Make the building regular in elevation (§4.2.3.3) or use the modal response spectrum analysis instead of the lateral force method.", "Das Gebäude im Aufriss regelmässig ausbilden (§4.2.3.3) oder statt des Kraftgrössenverfahrens das multimodale Antwortspektrenverfahren verwenden."))
        });
    }
    report.push(lfm.build());
    (masses, heights, total_mass, t1)
}
