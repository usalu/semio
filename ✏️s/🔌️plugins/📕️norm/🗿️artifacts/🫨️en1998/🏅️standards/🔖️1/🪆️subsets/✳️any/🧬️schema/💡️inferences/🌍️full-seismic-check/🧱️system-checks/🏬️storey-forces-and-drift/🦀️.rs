//! 🏬️ Storey force and drift checks: the accidental torsion, the storey shear demand, the drift and the second order effects of one structural system. See [`super`].

use super::*;

/// 🏬 Checks the accidental torsion amplified storey forces and storey shears against the resistances.
pub fn check_storey_forces(
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
    let plan_w = if system.direction == semio_s_artifact_norm_en1998_model::AnalysisDirection::Y { building.plan_width_m } else { building.plan_length_m };
    let delta_torsion = if building.plan_width_m > 0.0 && building.plan_length_m > 0.0 {
        let e0 = building.storeys.iter().map(|s| {
            let cm = if system.direction == semio_s_artifact_norm_en1998_model::AnalysisDirection::Y { s.centre_of_mass_x_m } else { s.centre_of_mass_y_m };
            let (cs_x, cs_y, _) = wall_plan::stiffness_centre_of(&building.members, s);
            let cs = if system.direction == semio_s_artifact_norm_en1998_model::AnalysisDirection::Y { cs_x } else { cs_y };
            (cm - cs).abs()
        }).fold(0.0_f64, f64::max);
        let e_ai = building.accidental_eccentricity_ratio.max(0.0) * plan_w;
        let l_e = plan_w.max(1e-9);
        let x = 0.5 * l_e + e0;
        let delta = part_1::torsion_amplification(x, l_e);
        let limit = part_1::torsion_amplification_simplified_limit();
        let mut tor = CheckResult::assess(
            format!("en1998.1.{bid}.{}.torsion", system.id),
            "DIN EN 1998-1",
            ClauseId::new("EN 1998-1", "1", "4.3.3.2.4"),
            SubjectRef::new(bid, format!("{bpath}.accidentalEccentricityRatio"), blabel.clone()),
            lc("Accidental torsion amplification δ = 1 + 0.6·x/L_e", "Zufällige Torsion δ = 1 + 0.6·x/L_e"),
        )
        .utilization(q_dim(delta), q_dim(limit))
        .annex(annex)
        .explanation(lc(
            &format!("e_ai={e_ai:.3} m, e0(max over storeys)={e0:.3} m, x={x:.3} m, L_e={l_e:.1} m, δ={delta:.3} (simplified-procedure bound {limit:.2}), simplified procedure allowed only for planRegular={}", building.plan_regular),
            &format!("e_ai={e_ai:.3} m, e0 (Maximum über Geschosse)={e0:.3} m, x={x:.3} m, L_e={l_e:.1} m, δ={delta:.3} (Grenze vereinfachtes Verfahren {limit:.2}), vereinfachtes Verfahren nur bei planRegular={}", building.plan_regular),
        ));
        if delta > limit + 1e-12 || !building.plan_regular {
            tor = tor.status(semio_s_plugin_norm_compliance::CheckStatus::Fail).remedy(Remedy::at_most(
                SubjectRef::new(bid, format!("{bpath}.accidentalEccentricityRatio"), lc("Accidental eccentricity ratio", "Zufällige Exzentrizität")),
                q_dim(building.accidental_eccentricity_ratio),
                q_dim(0.0),
                lc("Align centre of mass and stiffness (reduce e₀) and restore plan regularity, or use a spatial model with torsional moments.", "Massen- und Steifigkeitsmittelpunkt angleichen (e₀ → 0) und Grundrissregelmässigkeit herstellen oder räumliches Modell mit Torsionsmomenten verwenden."),
            ));
        }
        report.push(tor.build());
        delta
    } else {
        1.0
    };
    let forces_raw = part_1::storey_forces_n(f_b, masses, heights);
    let forces: Vec<f64> = forces_raw;
    let mut remaining_v = f_b;
    for (sti, storey) in building.storeys.iter().enumerate() {
        let fi = forces.get(sti).copied().unwrap_or(0.0);
        let v_i = remaining_v;
        remaining_v = (remaining_v - fi).max(0.0);
        let v_rd = storey.shear_resistance_n;
        let p_above: f64 = building.storeys.iter().enumerate().skip(sti).map(|(i, st)| part_1::storey_seismic_action_n(st, i + 1 == building.storeys.len())).sum();
        let drift_i = if system.direction == semio_s_artifact_norm_en1998_model::AnalysisDirection::Y { storey.drift_y_m } else { storey.drift_x_m };
        let theta_i = part_1::drift_sensitivity(p_above, drift_i, v_i, storey.height_m);
        let v_i = v_i * delta_torsion * part_1::p_delta_amplification(theta_i).unwrap_or(1.0 / 0.7);
        let stpath = format!("{}.storeys[id={}]", bpath, storey.id);
        let mut sc = CheckResult::assess(
            format!("en1998.1.{bid}.{}.storeyForces.{}", system.id, storey.id),
            "DIN EN 1998-1",
            ClauseId::new("EN 1998-1", "1", "4.3.3.2.3"),
            SubjectRef::new(bid, format!("{stpath}.shearResistanceN"), lc(&storey.id, &storey.id)),
            lc("Storey shear V_i ≤ V_Rd,i", "Geschossquerkraft V_i ≤ V_Rd,i"),
        )
        .utilization(q_force(v_i), q_force(v_rd.max(1e-9)))
        .annex(annex)
        .explanation(lc(
        &format!("Fi={fi:.0} N, Vi={v_i:.0} N, VRd,i={v_rd:.0} N",),
        &format!("Fi={fi:.0} N, Vi={v_i:.0} N, V_Rd,i={v_rd:.0} N",),
    ));
        sc = sc.field("storey-shear", vec![semio_s_plugin_norm_compliance::Station::ratio_of(&[storey_station(building, sti)], q_force(v_i), q_force(v_rd.max(1e-9)))]);
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
pub fn check_drift_and_second_order(
    annex: AnnexChoice,
    report: &mut CheckReport,
    building: &semio_s_artifact_norm_en1998_model::En1998Building,
    bid: &str,
    bpath: &str,
    system: &semio_s_artifact_norm_en1998_model::En1998System,
    f_b: f64,
    forces: &[f64],
) {
    let (theta_lim, theta_ndps) = semio_s_plugin_norm_compliance::record_ndp_reads(|| part_1::drift_theta(building.drift_limit_class));
    let mut remaining_weight: f64 = building.storeys.iter().enumerate().map(|(i, st)| {
        part_1::storey_seismic_action_n(st, i + 1 == building.storeys.len())
    }).sum();
    let mut shear_above = f_b;
    for storey in &building.storeys {
        let drift = if system.direction == semio_s_artifact_norm_en1998_model::AnalysisDirection::Y { storey.drift_y_m } else { storey.drift_x_m };
        let limit = theta_lim * storey.height_m;
        let drift_util = part_1::drift_utilization(drift, building.nu, theta_lim, storey.height_m);
        let dfield = if system.direction == semio_s_artifact_norm_en1998_model::AnalysisDirection::Y { "driftYM" } else { "driftXM" };
        let dpath = format!("{}.storeys[id={}].{}", bpath, storey.id, dfield);
        let mut drift_check = CheckResult::assess(
            format!("en1998.1.{bid}.{}.drift.{}", system.id, storey.id),
            "DIN EN 1998-1",
            ClauseId::new("EN 1998-1", "1", "4.4.3.2"),
            SubjectRef::new(bid, dpath.clone(), lc(&storey.id, &storey.id)),
            lc("Interstorey drift", "Stockwerksverschiebung"),
        )
        .utilization(q_dim(drift_util.min(1e9)), q_dim(1.0))
        .annex(annex)
        .explanation(lc(&format!("d_r·ν={:.5} m ≤ θ·h={limit:.5} m (d_r={drift:.4} m, ν={:.2})", drift * building.nu, building.nu), &format!("d_r·ν={:.5} m ≤ θ·h={limit:.5} m (d_r={drift:.4} m, ν={:.2})", drift * building.nu, building.nu)));
        drift_check = drift_check.field("storey-drift", vec![semio_s_plugin_norm_compliance::Station::ratio_of(&[storey_station(building, storey_index(building, &storey.id))], q_len(drift * building.nu), q_len(limit.max(1e-9)))]);
        if drift_util > 1.0 {
            drift_check = drift_check.remedy(Remedy::at_most(
                SubjectRef::new(bid, dpath, lc("Drift", "Verschiebung")),
                q_len(drift),
                q_len(limit / building.nu.max(1e-9)),
                lc("Reduce interstorey drift (stiffen or lower q).", "Stockwerksverschiebung reduzieren (aussteifen oder q senken)."),
            ));
        }
        report.push(drift_check.ndps(&theta_ndps).build());

        let sti = building.storeys.iter().position(|s| s.id == storey.id).unwrap_or(0);
        let weight = part_1::storey_seismic_action_n(storey, sti + 1 == building.storeys.len());
        let theta = part_1::drift_sensitivity(remaining_weight, drift, shear_above, storey.height_m);
        let dpath_pd = format!("{}.storeys[id={}].{}", bpath, storey.id, dfield);
        let mut pdelta = CheckResult::assess(
            format!("en1998.1.{bid}.{}.pdelta.{}", system.id, storey.id),
            "DIN EN 1998-1",
            ClauseId::new("EN 1998-1", "1", "4.4.2.2"),
            SubjectRef::new(bid, dpath_pd.clone(), lc(&storey.id, &storey.id)),
            lc("P-Δ sensitivity θ", "P-Δ-Empfindlichkeit θ"),
        )
        .utilization(q_dim(theta.min(1e9)), q_dim(0.2))
        .annex(annex)
        .explanation(lc(
            &format!("P-Δ sensitivity θ={theta:.3} (d={drift:.4} m, V={shear_above:.0} N, P={remaining_weight:.0} N); effects multiplied by {:.3} (θ ≤ 0.1: 1.0, 0.1 < θ ≤ 0.2: 1/(1-θ), 0.2 < θ ≤ 0.3: second-order analysis)", part_1::p_delta_amplification(theta).unwrap_or(f64::INFINITY)),
            &format!("P-Δ-Empfindlichkeit θ={theta:.3} (d={drift:.4} m, V={shear_above:.0} N, P={remaining_weight:.0} N); Beanspruchungen mit {:.3} multipliziert (θ ≤ 0,1: 1,0; 0,1 < θ ≤ 0,2: 1/(1-θ); 0,2 < θ ≤ 0,3: Theorie 2. Ordnung)", part_1::p_delta_amplification(theta).unwrap_or(f64::INFINITY)),
        ));
        pdelta = pdelta.field("storey-pdelta", vec![semio_s_plugin_norm_compliance::Station::ratio_of(&[storey_station(building, sti)], q_dim(theta.min(1e9)), q_dim(0.2))]);
        if theta > 0.2 {
            let req_drift = if remaining_weight > 1e-9 {
                0.2 * shear_above * storey.height_m / remaining_weight
            } else {
                0.0
            };
            pdelta = pdelta.remedy(Remedy::at_most(
                SubjectRef::new(bid, dpath_pd, lc("Interstorey drift", "Stockwerksverschiebung")),
                q_len(drift),
                q_len(req_drift),
                lc(
                    &format!("Reduce interstorey drift to at most {req_drift:.4} m so θ≤0.2, the limit of the simplified amplification (EN 1998-1 §4.4.2.2)."),
                    &format!("Stockwerksverschiebung auf höchstens {req_drift:.4} m reduzieren, damit θ≤0.2, der Grenze der vereinfachten Vergrößerung (EN 1998-1 §4.4.2.2)."),
                ),
            ));
        }
        report.push(pdelta.build());
        remaining_weight = (remaining_weight - weight).max(0.0);
        let fi = forces.get(building.storeys.iter().position(|s| s.id == storey.id).unwrap_or(0)).copied().unwrap_or(0.0);
        shear_above = (shear_above - fi).max(0.0);
    }
}

/// 📏️ Station coordinate of the storey at `index`: `s = index / (storeys − 1)` in `[0, 1]` (check-station contract for storey fields).
fn storey_station(building: &semio_s_artifact_norm_en1998_model::En1998Building, index: usize) -> f64 {
    let n = building.storeys.len();
    if n > 1 { index as f64 / (n - 1) as f64 } else { 0.0 }
}

/// 🔢 Position of the storey with `id` in the building.
fn storey_index(building: &semio_s_artifact_norm_en1998_model::En1998Building, id: &str) -> usize {
    building.storeys.iter().position(|s| s.id == id).unwrap_or(0)
}
