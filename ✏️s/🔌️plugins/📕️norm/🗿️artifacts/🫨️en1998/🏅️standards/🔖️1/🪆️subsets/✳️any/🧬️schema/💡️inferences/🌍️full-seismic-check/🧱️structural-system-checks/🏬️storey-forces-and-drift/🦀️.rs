//! 🏬️ Storey force and drift checks: the accidental torsion, the storey shear demand, the drift and the second order effects of one structural system. See [`super`].

use super::*;

/// 🏬 Checks the accidental torsion amplified storey forces and storey shears against the resistances.
pub(super) fn check_storey_forces(
    annex: AnnexChoice,
    report: &mut CheckReport,
    building: &semio_s_artifact_norm_en1998_model::En1998Building,
    bid: &str,
    bpath: &str,
    blabel: &LocalizedCopy,
    masses: &[f64],
    heights: &[f64],
    system: &semio_s_artifact_norm_en1998_model::En1998System,
    f_b: f64,
) -> Vec<f64> {
    let plan_w = if system.direction == "y" { building.plan_length_m } else { building.plan_width_m };
    let e_ai = 0.05 * plan_w.max(0.0);
    let (delta_torsion, _e_tot) = if building.plan_width_m > 0.0 && building.plan_length_m > 0.0 {
        let st = building.storeys.first();
        let e0 = st.map(|s| {
            let cm = if system.direction == "y" { s.centre_of_mass_x_m } else { s.centre_of_mass_y_m };
            let cs = if system.direction == "y" { s.centre_of_stiffness_x_m } else { s.centre_of_stiffness_y_m };
            (cm - cs).abs()
        }).unwrap_or(0.0);
        let e_ai_eff = e_ai * (building.accidental_eccentricity_ratio / 0.05).max(0.0);
        let e_tot = e0 + e_ai_eff;
        let l_e = plan_w.max(1e-9);
        let delta = (1.0 + 1.2 * e_tot / l_e).min(3.0);
        let x = e_tot;
        if building.plan_width_m > 0.0 && building.plan_length_m > 0.0 {
            let mut tor = CheckResult::assess(
                format!("en1998.1.{bid}.{}.torsion", system.id),
                "DIN EN 1998-1",
                ClauseId::new("EN 1998-1", "1", "4.3.3.2.4"),
                SubjectRef::new(bid, format!("{bpath}.accidentalEccentricityRatio"), blabel.clone()),
                lc("Accidental torsion amplification δ", "Zufällige Torsion δ"),
            )
            .utilization(q_dim(delta), q_dim(1.5))
            .annex(annex)
            .explanation(lc(
                &format!("e_ai={e_ai_eff:.3} m (0.05·L·ratio), e0={e0:.3} m, e_tot={e_tot:.3} m, x={x:.3} m, L_e={l_e:.1} m, δ={delta:.3}"),
                &format!("e_ai={e_ai_eff:.3} m (0.05·L·Verhältnis), e0={e0:.3} m, e_tot={e_tot:.3} m, x={x:.3} m, L_e={l_e:.1} m, δ={delta:.3}"),
            ));
            if delta > 1.5 {
                tor = tor.remedy(Remedy::at_most(
                    SubjectRef::new(bid, format!("{bpath}.accidentalEccentricityRatio"), lc("Accidental eccentricity ratio", "Zufällige Exzentrizität")),
                    q_dim(building.accidental_eccentricity_ratio),
                    q_dim(0.05),
                    lc("Reduce accidental eccentricity ratio toward 0.05 or reduce structural eccentricity e₀.", "Zufällige Exzentrizität Richtung 0.05 senken oder strukturelle Exzentrizität e₀ verringern."),
                ));
            }
            report.push(tor.build());
        }
        (delta, e_tot)
    } else {
        (1.0, 0.0)
    };
    let forces_raw = part_1::storey_forces_n(f_b, masses, heights);
    let forces: Vec<f64> = forces_raw.iter().map(|f| f * delta_torsion).collect();
    let mut remaining_v = f_b * delta_torsion;
    for (sti, storey) in building.storeys.iter().enumerate() {
        let is_roof = sti + 1 == building.storeys.len();
        if is_roof {
            let mut ph = CheckResult::assess(
                format!("en1998.1.{bid}.storey.{}.roofPhi", storey.id),
                "DIN EN 1998-1",
                ClauseId::new("EN 1998-1", "1", "4.2.4"),
                SubjectRef::new(bid, format!("{}.storeys[id={}].correlatedOccupancy", bpath, storey.id), lc(&storey.id, &storey.id)),
                lc("Roof φ = 1.0 (Table 4.2)", "Dach φ = 1.0 (Tabelle 4.2)"),
            )
            .utilization(q_dim(if storey.correlated_occupancy { 0.5 } else { 1.5 }), q_dim(1.0))
            .annex(annex)
            .explanation(lc(
                &format!("correlatedOccupancy={} (roof requires true so φ=1.0)", storey.correlated_occupancy),
                &format!("korrelierteNutzung={} (Dach erfordert true, damit φ=1.0)", storey.correlated_occupancy),
            ));
            if !storey.correlated_occupancy {
                ph = ph.remedy(Remedy::exactly(
                    SubjectRef::new(bid, format!("{}.storeys[id={}].correlatedOccupancy", bpath, storey.id), lc("Correlated occupancy", "Korrelierte Nutzung")),
                    q_dim(0.0),
                    q_dim(1.0),
                    lc("Set correlatedOccupancy true for the roof storey (Table 4.2 φ=1.0).", "Korrelierte Nutzung für das Dachgeschoss auf true setzen (Tab. 4.2 φ=1.0)."),
                ));
            }
            report.push(ph.build());
        }

        let fi = forces.get(sti).copied().unwrap_or(0.0);
        let v_i = remaining_v;
        remaining_v = (remaining_v - fi).max(0.0);
        let v_rd = if storey.shear_resistance_n > 0.0 { storey.shear_resistance_n } else { system.base_shear_resistance_n };
        let stpath = format!("{}.storeys[id={}]", bpath, storey.id);
        let mut sc = CheckResult::assess(
            format!("en1998.1.{bid}.{}.storeyForces.{}", system.id, storey.id),
            "DIN EN 1998-1",
            ClauseId::new("EN 1998-1", "1", "4.3.3.2.3"),
            SubjectRef::new(bid, format!("{stpath}.shearResistanceN"), lc(&storey.id, &storey.id)),
            lc("Storey shear V_i ≤ V_Rd,i", "Geschossquerkraft V_i ≤ V_Rd,i"),
        )
        .utilization(q_force(v_i), q_force(v_rd))
        .annex(annex)
        .explanation(lc(
        &format!("Fi={fi:.0} N, Vi={v_i:.0} N, VRd,i={v_rd:.0} N",),
        &format!("Fi={fi:.0} N, Vi={v_i:.0} N, V_Rd,i={v_rd:.0} N",),
    ));
        if v_i > v_rd {
            sc = sc.remedy(Remedy::at_least(
                SubjectRef::new(bid, format!("{stpath}.shearResistanceN"), lc("Storey shear resistance", "Geschoss-Querkrafttragfähigkeit")),
                q_force(v_rd),
                q_force(v_i),
                lc(&format!("Increase storey shear resistance to at least {v_i:.0} N.",), &format!("Geschoss-Querkrafttragfähigkeit auf mindestens {v_i:.0} N erhöhen.")),
            ));
        }
        report.push(sc.build());
    }
    forces
}

/// 📐 Checks the interstorey drift and the second order sensitivity of every storey.
pub(super) fn check_drift_and_second_order(
    annex: AnnexChoice,
    report: &mut CheckReport,
    building: &semio_s_artifact_norm_en1998_model::En1998Building,
    bid: &str,
    bpath: &str,
    system: &semio_s_artifact_norm_en1998_model::En1998System,
    f_b: f64,
    forces: &[f64],
) {
    let theta_lim = part_1::drift_theta(&building.drift_limit_class);
    let mut remaining_weight: f64 = building.storeys.iter().enumerate().map(|(i, st)| {
        part_1::storey_seismic_action_n(st, i + 1 == building.storeys.len())
    }).sum();
    let mut shear_above = f_b;
    for storey in &building.storeys {
        let drift = if system.direction == "y" { storey.drift_y_m } else { storey.drift_x_m };
        let limit = part_1::interstorey_drift_limit_m(storey.height_m, building.nu, theta_lim);
        let dfield = if system.direction == "y" { "driftYM" } else { "driftXM" };
        let dpath = format!("{}.storeys[id={}].{}", bpath, storey.id, dfield);
        let mut drift_check = CheckResult::assess(
            format!("en1998.1.{bid}.{}.drift.{}", system.id, storey.id),
            "DIN EN 1998-1",
            ClauseId::new("EN 1998-1", "1", "4.4.3.2"),
            SubjectRef::new(bid, dpath.clone(), lc(&storey.id, &storey.id)),
            lc("Interstorey drift", "Stockwerksverschiebung"),
        )
        .utilization(q_len(drift), q_len(limit))
        .annex(annex)
        .explanation(lc(&format!("interstorey drift d={drift:.4} m ≤ limit {limit:.4} m"), &format!("Stockwerksverschiebung d={drift:.4} m ≤ Grenzwert {limit:.4} m")));
        if drift > limit {
            drift_check = drift_check.remedy(Remedy::at_most(
                SubjectRef::new(bid, dpath, lc("Drift", "Verschiebung")),
                q_len(drift),
                q_len(limit),
                lc("Reduce interstorey drift (stiffen or lower q).", "Stockwerksverschiebung reduzieren (aussteifen oder q senken)."),
            ));
        }
        report.push(drift_check.build());

        let sti = building.storeys.iter().position(|s| s.id == storey.id).unwrap_or(0);
        let weight = part_1::storey_seismic_action_n(storey, sti + 1 == building.storeys.len());
        let theta = if shear_above * storey.height_m > 1e-9 { remaining_weight * drift / (shear_above * storey.height_m) } else { 0.0 };
        let dpath_pd = format!("{}.storeys[id={}].{}", bpath, storey.id, dfield);
        let mut pdelta = CheckResult::assess(
            format!("en1998.1.{bid}.{}.pdelta.{}", system.id, storey.id),
            "DIN EN 1998-1",
            ClauseId::new("EN 1998-1", "1", "4.4.2.2"),
            SubjectRef::new(bid, dpath_pd.clone(), lc(&storey.id, &storey.id)),
            lc("P-Δ sensitivity θ", "P-Δ-Empfindlichkeit θ"),
        )
        .utilization(q_dim(theta), q_dim(0.3))
        .annex(annex)
        .explanation(lc(
            &format!("P-Δ sensitivity θ={theta:.3} (d={drift:.4} m, V={shear_above:.0} N, P={remaining_weight:.0} N)"),
            &format!("P-Δ-Empfindlichkeit θ={theta:.3} (d={drift:.4} m, V={shear_above:.0} N, P={remaining_weight:.0} N)"),
        ));
        if theta > 0.3 {
            let req_drift = if remaining_weight > 1e-9 {
                0.3 * shear_above * storey.height_m / remaining_weight
            } else {
                0.0
            };
            pdelta = pdelta.remedy(Remedy::at_most(
                SubjectRef::new(bid, dpath_pd, lc("Interstorey drift", "Stockwerksverschiebung")),
                q_len(drift),
                q_len(req_drift),
                lc(
                    &format!("Reduce interstorey drift to at most {req_drift:.4} m so θ≤0.3 (EN 1998-1 §4.4.2.2)."),
                    &format!("Stockwerksverschiebung auf höchstens {req_drift:.4} m reduzieren, damit θ≤0.3 (EN 1998-1 §4.4.2.2)."),
                ),
            ));
        }
        report.push(pdelta.build());
        remaining_weight = (remaining_weight - weight).max(0.0);
        let fi = forces.get(building.storeys.iter().position(|s| s.id == storey.id).unwrap_or(0)).copied().unwrap_or(0.0);
        shear_above = (shear_above - fi).max(0.0);
    }
}
