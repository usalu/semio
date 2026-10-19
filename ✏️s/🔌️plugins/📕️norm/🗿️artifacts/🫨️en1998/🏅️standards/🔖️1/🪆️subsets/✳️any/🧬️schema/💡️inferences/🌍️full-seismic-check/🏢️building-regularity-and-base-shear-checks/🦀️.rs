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

    let t1 = match building.t1_method.to_ascii_lowercase().as_str() {
        "given" => building.t1_given_s,
        "rayleigh" => {
            let mut z = 0.0;
            let mut disp = Vec::new();
            for h in &heights {
                z += h;
                disp.push(z / total_height.max(1e-9));
            }
            part_1::t1_rayleigh(&masses, &disp)
        }
        _ => part_1::t1_from_ct(building.ct, total_height),
    };

    {
        let using_given = building.t1_method.to_ascii_lowercase() == "given";
        let mut tg = CheckResult::assess(
            format!("en1998.1.{bid}.t1Given"),
            "DIN EN 1998-1",
            ClauseId::new("EN 1998-1", "1", "4.3.3.2.2"),
            SubjectRef::new(bid, format!("{bpath}.t1GivenS"), blabel.clone()),
            lc("Given fundamental period T₁", "Vorgegebene Fundamentalschwingzeit T₁"),
        )
        .utilization(q_dim(building.t1_given_s), q_dim(if using_given { t1.max(0.05) } else { 0.05 }))
        .annex(annex)
        .explanation(lc(
            &format!("t1Method={}, t1Given={:.3} s, T1={t1:.3} s", building.t1_method, building.t1_given_s),
            &format!("t1Verfahren={}, t1Given={:.3} s, T1={t1:.3} s", building.t1_method, building.t1_given_s),
        ));
        if !using_given && building.t1_given_s > 0.05 {
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
        .utilization(q_dim(if ok { 0.5 } else { 1.5 }), q_dim(1.0))
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
        for storey in &building.storeys {
            let (e0x, e0y, rx, ry, ls) = part_1::plan_regularity_metrics(
                storey.centre_of_mass_x_m,
                storey.centre_of_mass_y_m,
                storey.centre_of_stiffness_x_m,
                storey.centre_of_stiffness_y_m,
                storey.stiffness_x,
                storey.stiffness_y,
                building.plan_width_m,
                building.plan_length_m,
            );
            let e0 = e0x.max(e0y);
            let r_min = rx.min(ry);
            let limit_e = 0.30 * r_min;
            let stpath = format!("{}.storeys[id={}]", bpath, storey.id);
            let mut ch = CheckResult::assess(
                format!("en1998.1.{bid}.storey.{}.planRegularity", storey.id),
                "DIN EN 1998-1",
                ClauseId::new("EN 1998-1", "1", "4.2.3.2"),
                SubjectRef::new(bid, format!("{stpath}.centreOfMassXM"), lc(&storey.id, &storey.id)),
                lc("Plan regularity e₀ ≤ 0.30 r and r ≥ ℓₛ", "Grundrissregelmässigkeit e₀ ≤ 0.30 r und r ≥ ℓₛ"),
            )
            .utilization(q_len(e0), q_len(limit_e.max(1e-6)))
            .annex(annex)
            .explanation(lc(
                &format!("e0x={e0x:.3} m, e0y={e0y:.3} m, rx={rx:.3} m, ry={ry:.3} m, ls={ls:.3} m, kx={:.0}, ky={:.0}", storey.stiffness_x, storey.stiffness_y),
                &format!("e0x={e0x:.3} m, e0y={e0y:.3} m, rx={rx:.3} m, ry={ry:.3} m, ℓₛ={ls:.3} m, kx={:.0}, ky={:.0}", storey.stiffness_x, storey.stiffness_y),
            ));
            if e0 > limit_e || ls > r_min {
                ch = ch.remedy(Remedy::at_most(
                    SubjectRef::new(bid, format!("{stpath}.centreOfMassXM"), lc("Centre of mass x", "Massenschwerpunkt x")),
                    q_len(storey.centre_of_mass_x_m),
                    q_len(storey.centre_of_stiffness_x_m),
                    lc("Reduce structural eccentricity e₀ (align CM and CS) or increase torsional stiffness.", "Strukturelle Exzentrizität e₀ verringern (SM und SS ausrichten) oder Torsionssteifigkeit erhöhen."),
                ));
            }
            report.push(ch.build());
        }

        let n = building.storeys.len();
        if n >= 2 {
            for i in 0..n - 1 {
                let lo = &building.storeys[i];
                let hi = &building.storeys[i + 1];
                let k_lo = lo.stiffness_x + lo.stiffness_y;
                let k_hi = hi.stiffness_x + hi.stiffness_y;
                let m_lo = part_1::storey_seismic_mass_kg(lo, false);
                let m_hi = part_1::storey_seismic_mass_kg(hi, i + 2 == n);
                let k_ratio = if k_lo > 0.0 { k_hi / k_lo } else { 0.0 };
                let m_ratio = if m_lo > 0.0 { m_hi / m_lo } else { 0.0 };
                let util = (0.70 / k_ratio.max(1e-9)).max(m_ratio / 1.50);
                let mut ch = CheckResult::assess(
                    format!("en1998.1.{bid}.elevationRegularity.{}-{}", lo.id, hi.id),
                    "DIN EN 1998-1",
                    ClauseId::new("EN 1998-1", "1", "4.2.3.3"),
                    SubjectRef::new(bid, format!("{}.storeys[id={}].stiffnessX", bpath, hi.id), lc(&hi.id, &hi.id)),
                    lc("Elevation regularity (stiffness/mass variation)", "Aufrissregelmässigkeit (Steifigkeits-/Masseänderung)"),
                )
                .utilization(q_dim(util), q_dim(1.0))
                .annex(annex)
                .explanation(lc(
                    &format!("k_ratio={k_ratio:.3} (≥0.70), m_ratio={m_ratio:.3} (≤1.50), correlated_lo={}, correlated_hi={}", lo.correlated_occupancy, hi.correlated_occupancy),
                    &format!("k-Verhältnis={k_ratio:.3} (≥0.70), m-Verhältnis={m_ratio:.3} (≤1.50), korreliert_unten={}, korreliert_oben={}", lo.correlated_occupancy, hi.correlated_occupancy),
                ));
                if util > 1.0 {
                    ch = ch.remedy(Remedy::at_least(
                        SubjectRef::new(bid, format!("{}.storeys[id={}].stiffnessX", bpath, hi.id), lc("Storey stiffness X", "Geschosssteifigkeit X")),
                        q_dim(hi.stiffness_x),
                        q_dim(0.70 * lo.stiffness_x),
                        lc("Increase upper-storey stiffness (avoid soft storey) or smooth mass variation.", "Obere Geschosssteifigkeit erhöhen (weiches Geschoss vermeiden) oder Masseübergang glätten."),
                    ));
                }
                report.push(ch.build());
            }
        }
    }

    {
        let ok = building.plan_width_m > 0.0 && building.plan_length_m > 0.0;
        let mut c = CheckResult::assess(
            format!("en1998.1.{bid}.planDims"),
            "DIN EN 1998-1",
            ClauseId::new("EN 1998-1", "1", "4.3.2"),
            SubjectRef::new(bid, format!("{bpath}.planWidthM"), blabel.clone()),
            lc("Plan dimensions Lx, Ly", "Grundrissabmessungen Lx, Ly"),
        )
        .utilization(q_len(building.plan_width_m), q_len(building.plan_length_m.max(0.01)))
        .annex(annex)
        .status(if ok { semio_s_plugin_norm_compliance::CheckStatus::Pass } else { semio_s_plugin_norm_compliance::CheckStatus::Fail })
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
        let types: Vec<String> = building.systems.iter().map(|s| s.system_type.to_ascii_lowercase()).collect();
        let has_frame = types.iter().any(|t| t == "frame" || t == "dual");
        let has_wall = types.iter().any(|t| t == "wall" || t == "dual" || t.contains("wall"));
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
        .utilization(q_dim(if ok { 0.5 } else { 1.5 }), q_dim(1.0))
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
    report.push(
        CheckResult::assess(
            format!("en1998.1.{bid}.lfm"),
            "DIN EN 1998-1",
            ClauseId::new("EN 1998-1", "1", "4.3.3.2"),
            SubjectRef::new(bid, format!("{bpath}.elevationRegular"), blabel.clone()),
            lc("Lateral force method applicability", "Anwendbarkeit des Kraftgrössenverfahrens"),
        )
        .utilization(q_dim(if lfm_ok { 0.5 } else { 0.95 }), q_dim(1.0))
        .status(if lfm_ok { semio_s_plugin_norm_compliance::CheckStatus::Pass } else { semio_s_plugin_norm_compliance::CheckStatus::Warning })
        .annex(annex)
        .explanation(lc(
            &format!("T1={t1:.3}s (method={}, t1Given={:.3}s), TC={tc:.3}s, elevationRegular={}", building.t1_method, building.t1_given_s, building.elevation_regular),
            &format!("T1={t1:.3}s (Verfahren={}, t1Given={:.3}s), TC={tc:.3}s, Aufrissregelmässigkeit={}", building.t1_method, building.t1_given_s, building.elevation_regular),
        ))
        .build(),
    );
    (masses, heights, total_mass, t1)
}
