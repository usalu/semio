//! 🧱️ Structural system checks: the per system checks of every building of the full seismic check. See [`super`].

use super::*;

/// 🧱 Checks every structural system of one building.
pub(super) fn check_building_systems(
    document: &En1998Snapshot,
    annex: AnnexChoice,
    report: &mut CheckReport,
    gamma_i: f64,
    a_g: f64,
    s: f64,
    tb: f64,
    tc: f64,
    td: f64,
    building: &semio_s_artifact_norm_en1998_model::En1998Building,
    bid: &String,
    bpath: &String,
    blabel: &LocalizedCopy,
    masses: &Vec<f64>,
    heights: &Vec<f64>,
    total_mass: f64,
    t1: f64,
    systems: &Vec<semio_s_artifact_norm_en1998_model::En1998System>,
) {
    for system in systems {
        {
            let dup = building
                .systems
                .iter()
                .filter(|s| s.direction.to_ascii_lowercase() == system.direction.to_ascii_lowercase())
                .count()
                > 1;
            let known = matches!(system.direction.to_ascii_lowercase().as_str(), "x" | "y");
            let mut dch = CheckResult::assess(
                format!("en1998.1.{bid}.{}.direction", system.id),
                "DIN EN 1998-1",
                ClauseId::new("EN 1998-1", "1", "4.3.3.2.1"),
                SubjectRef::new(bid, format!("{}.systems[id={}].direction", bpath, system.id), lc(&system.id, &system.id)),
                lc("Horizontal analysis direction", "Horizontale Analyserichtung"),
            )
            .utilization(q_dim(if !known || dup { 1.5 } else { 0.5 }), q_dim(1.0))
            .annex(annex)
            .explanation(lc(
                &format!("direction={}, known={}, duplicate={}", system.direction, known, dup),
                &format!("Richtung={}, gültig={}, doppelt={}", system.direction, known, dup),
            ));
            if !known || dup {
                dch = dch.remedy(Remedy::exactly(
                    SubjectRef::new(bid, format!("{}.systems[id={}].direction", bpath, system.id), lc("Direction", "Richtung")),
                    q_dim(0.0),
                    q_dim(1.0),
                    lc(
                        "Assign unique orthogonal directions x and y to resisting systems.",
                        "Den Aussteifungssystemen eindeutige orthogonale Richtungen x und y zuweisen.",
                    ),
                ));
            }
            report.push(dch.build());
        }

        let q = part_1::behaviour_factor(system.q0, system.alpha_u_over_alpha_1, system.k_w);
        let q_max = part_1::q_max_for_system(&system.system_type, &system.material, &system.ductility_class);
        let s_e = part_1::elastic_response_spectrum(a_g, s, tb, tc, td, t1);
        let s_d = part_1::design_spectrum_sd(s_e, gamma_i, q.min(q_max), a_g);
        let lambda = part_1::lambda_factor(t1, tc, building.storeys.len());
        let f_b = part_1::base_shear_n(s_d, total_mass, lambda);
        let spath = format!("{}.systems[id={}]", bpath, system.id);

        // q limit vs tabulated max
        {
            let ok = q <= q_max + 1e-9;
            let mut c = CheckResult::assess(
                format!("en1998.1.{bid}.{}.qLimit", system.id),
                "DIN EN 1998-1",
                ClauseId::new("EN 1998-1", "1", "5.2.2.2"),
                SubjectRef::new(bid, format!("{spath}.q0"), lc(
            &format!("System {}", system.id),
            &format!("Nachweis: System {}", system.id),
        )),
                lc("Behaviour factor q within tabulated limit", "Verhaltensbeiwert q innerhalb Tabellengrenze"),
            )
            .utilization(q_dim(if ok { q.min(q_max) } else { q }), q_dim(q_max))
            .annex(annex)
            .explanation(lc(
                &format!("q={q:.2} from q0={:.2}·αu/α1·kw; q_max={q_max:.2} for {}/{}/{}", system.q0, system.system_type, system.material, system.ductility_class),
                &format!("q={q:.2} aus q0={:.2}·αu/α1·kw; q_max={q_max:.2} für {}/{}/{}", system.q0, system.system_type, system.material, system.ductility_class),
            ));
            if !ok {
                let req_q0 = (q_max / (system.alpha_u_over_alpha_1 * system.k_w).max(1e-9)).max(1.0);
                c = c.remedy(Remedy::at_most(
                    SubjectRef::new(bid, format!("{spath}.q0"), lc("q₀", "q₀")),
                    q_dim(system.q0),
                    q_dim(req_q0),
                    lc(&format!("Reduce q0 to at most {req_q0:.2} so q ≤ {q_max:.2}.",), &format!("q0 auf höchstens {req_q0:.2} senken, damit q ≤ {q_max:.2}.")),
                ));
            }
            report.push(c.build());
        }

        let mut shear = CheckResult::assess(
            format!("en1998.1.{bid}.{}.baseShear", system.id),
            "DIN EN 1998-1",
            ClauseId::new("EN 1998-1", "1", "4.3.3.2.2"),
            SubjectRef::new(bid, format!("{spath}.baseShearResistanceN"), lc(
            &format!("System {}", system.id),
            &format!("Nachweis: System {}", system.id),
        )),
            lc("Seismic base shear F_b ≤ V_Rd", "Erdbeben-Querkraft F_b ≤ V_Rd"),
        )
        .utilization(q_force(f_b), q_force(system.base_shear_resistance_n))
        .annex(annex)
        .explanation(lc(
            &format!("dir={}, Sd(T1={t1:.3}s)={s_d:.3} m/s², λ={lambda:.2}, Fb={:.0} N, VRd={:.0} N, q={q:.2}", system.direction, f_b, system.base_shear_resistance_n),
            &format!("Richtung={}, Sd(T1={t1:.3}s)={s_d:.3} m/s², λ={lambda:.2}, F_b={:.0} N, V_Rd={:.0} N, q={q:.2}", system.direction, f_b, system.base_shear_resistance_n),
        ));
        if f_b > system.base_shear_resistance_n {
            let req_v = f_b;
            let req_mass = (system.base_shear_resistance_n / (s_d * lambda)).max(0.0);
            shear = shear
                .remedy(Remedy::at_least(
                    SubjectRef::new(bid, format!("{spath}.baseShearResistanceN"), lc("Base shear resistance", "Querkrafttragfähigkeit")),
                    q_force(system.base_shear_resistance_n),
                    q_force(req_v),
                    lc(&format!("Increase V_Rd from {:.0} N to at least {:.0} N (or add walls).", system.base_shear_resistance_n, req_v), &format!("V_Rd von {:.0} N auf mindestens {:.0} N erhöhen (oder Wände ergänzen).", system.base_shear_resistance_n, req_v)),
                ))
                .remedy(Remedy::at_most(
                    SubjectRef::new(bid, format!("{bpath}.storeys[id={}].permanentGkN", building.storeys.first().map(|s| s.id.as_str()).unwrap_or("s1")), lc("Permanent G_k", "Ständige Einwirkung G_k")),
                    q_force(building.storeys.first().map(|s| s.permanent_gk_n).unwrap_or(0.0)),
                    q_force((req_mass * 9.81 * 0.85).max(0.0)),
                    lc(&format!("Reduce permanent G_k so seismic mass approaches {req_mass:.0} kg.",), &format!("Ständige G_k senken, damit seismische Masse Richtung {req_mass:.0} kg geht.")),
                ));
        }
        report.push(shear.build());

        // Accidental torsion §4.3.3.2.4: e_ai = ±0.05 L_i; δ = 1 + 0.6 x/L_e amplifies storey forces.
        let plan_w = if system.direction == "y" { building.plan_length_m } else { building.plan_width_m };
        let e_ai = 0.05 * plan_w.max(0.0);
        let (delta_torsion, _e_tot) = if building.plan_width_m > 0.0 && building.plan_length_m > 0.0 {
            let st = building.storeys.first();
            let e0 = st.map(|s| {
                let cm = if system.direction == "y" { s.centre_of_mass_x_m } else { s.centre_of_mass_y_m };
                let cs = if system.direction == "y" { s.centre_of_stiffness_x_m } else { s.centre_of_stiffness_y_m };
                (cm - cs).abs()
            }).unwrap_or(0.0);
            // Design eccentricity includes accidental; ratio field scales e_ai (normative 0.05 → ratio=1).
            let e_ai_eff = e_ai * (building.accidental_eccentricity_ratio / 0.05).max(0.0);
            let e_tot = e0 + e_ai_eff;
            let l_e = plan_w.max(1e-9);
            // δ = 1 + 0.6·(e_tot)/(L_e/2) = 1 + 1.2·e_tot/L_e (EN 1998-1 §4.3.3.2.4).
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
        // Storey shear demand vs resistance — F_i and V_i amplified by δ (§4.3.3.2.4 / §4.3.3.3).
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
            // storey shear demand = sum of forces at and above this storey
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

        // Drift + P-Δ
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

        // Material-specific ductility detailing
        let gov_dc = &system.ductility_class;
        for member in &building.members {
            let mpath = format!("{}.members[id={}]", bpath, member.id);
            let mat = member.material.to_ascii_lowercase();
            if mat == "rc" || mat == "concrete" {
                let min_b = part_1::rc_column_min_b_m(gov_dc);
                if member.role.to_ascii_lowercase() == "column" {
                    let ok = member.min_dimension_m + 1e-12 >= min_b;
                    let mut c = CheckResult::assess(
                        format!("en1998.1.{bid}.member.{}.rc.minB", member.id),
                        "DIN EN 1998-1",
                        ClauseId::new("EN 1998-1", "1", "5.4.3.2.1"),
                        SubjectRef::new(bid, format!("{mpath}.minDimensionM"), lc(&member.id, &member.id)),
                        lc("RC column minimum dimension", "Stahlbetonstütze Mindestabmessung"),
                    )
                    .minimum(q_len(member.min_dimension_m), q_len(min_b))
                    .annex(annex)
                    .explanation(lc(&format!("b={:.3} m, required≥{min_b:.3} m", member.min_dimension_m), &format!("b={:.3} m, erforderlich≥{min_b:.3} m", member.min_dimension_m)));
                    if !ok {
                        c = c.remedy(Remedy::at_least(
                            SubjectRef::new(bid, format!("{mpath}.minDimensionM"), lc("Min dimension", "Mindestabmessung")),
                            q_len(member.min_dimension_m),
                            q_len(min_b),
                            lc(&format!("Increase column min dimension to {min_b:.3} m.",), &format!("Stützenmindestabmessung auf {min_b:.3} m erhöhen.")),
                        ));
                    }
                    report.push(c.build());
                }
                
                {
                    let mut c = CheckResult::assess(
                        format!("en1998.1.{bid}.member.{}.rc.rhoPrimeAbs", member.id),
                        "DIN EN 1998-1",
                        ClauseId::new("EN 1998-1", "1", "5.4.3"),
                        SubjectRef::new(bid, format!("{mpath}.rhoPrime"), lc(&member.id, &member.id)),
                        lc("RC reinforcement ρ′", "Stahlbetonbewehrung ρ′"),
                    )
                    .utilization(q_dim(member.rho_prime), q_dim(member.rho.max(1e-6)))
                    .annex(annex)
                    .explanation(lc(
                        &format!("ρ′={:.4}, ρ={:.4}", member.rho_prime, member.rho),
                        &format!("ρ′={:.4}, ρ={:.4} (Bewehrung)", member.rho_prime, member.rho),
                    ));
                    if member.rho_prime > member.rho + 1e-12 {
                        c = c.remedy(Remedy::at_most(
                            SubjectRef::new(bid, format!("{mpath}.rhoPrime"), lc("ρ′", "ρ′")),
                            q_dim(member.rho_prime),
                            q_dim(member.rho),
                            lc("Reduce ρ′ to at most ρ.", "ρ′ auf höchstens ρ senken."),
                        ));
                    }
                    report.push(c.build());
                }
if member.role.to_ascii_lowercase() == "beam" {
                    let min_b = 0.20; // EN 1998-1 §5.4.1.2.1 / §5.5.1.2.1 order-of-magnitude beam width floor for DCH detailing path
                    let ok = member.min_dimension_m + 1e-12 >= min_b;
                    let mut c = CheckResult::assess(
                        format!("en1998.1.{bid}.member.{}.rc.beamMinB", member.id),
                        "DIN EN 1998-1",
                        ClauseId::new("EN 1998-1", "1", "5.4.1.2.1"),
                        SubjectRef::new(bid, format!("{mpath}.minDimensionM"), lc(&member.id, &member.id)),
                        lc("RC beam minimum width", "Stahlbetonbalken Mindestbreite"),
                    )
                    .minimum(q_len(member.min_dimension_m), q_len(min_b))
                    .annex(annex)
                    .explanation(lc(
                        &format!("b={:.3} m, required≥{min_b:.3} m", member.min_dimension_m),
                        &format!("b={:.3} m, erforderlich≥{min_b:.3} m", member.min_dimension_m),
                    ));
                    if !ok {
                        c = c.remedy(Remedy::at_least(
                            SubjectRef::new(bid, format!("{mpath}.minDimensionM"), lc("Min dimension", "Mindestabmessung")),
                            q_len(member.min_dimension_m),
                            q_len(min_b),
                            lc(&format!("Increase beam min width to {min_b:.3} m."), &format!("Balkenmindestbreite auf {min_b:.3} m erhöhen.")),
                        ));
                    }
                    report.push(c.build());
                }
                let (rho_min, rho_max) = part_1::rc_rho_limits(&member.role, gov_dc);
                let ok_rho = member.rho + 1e-12 >= rho_min && member.rho <= rho_max + 1e-12;
                let mut c = CheckResult::assess(
                    format!("en1998.1.{bid}.member.{}.rc.rho", member.id),
                    "DIN EN 1998-1",
                    ClauseId::new("EN 1998-1", "1", "5.4.3"),
                    SubjectRef::new(bid, format!("{mpath}.rho"), lc(&member.id, &member.id)),
                    lc("RC longitudinal reinforcement ratio ρ", "Stahlbeton-Längsbewehrungsgrad ρ"),
                )
                .utilization(q_dim(member.rho), q_dim(if member.rho < rho_min { rho_min } else { rho_max }))
                .annex(annex)
                .explanation(lc(&format!("ρ={:.4}, limits [{rho_min:.4}, {rho_max:.4}]", member.rho), &format!("ρ={:.4}, Grenzen [{rho_min:.4}, {rho_max:.4}]", member.rho)));
                if !ok_rho {
                    c = c.status(semio_s_plugin_norm_compliance::CheckStatus::Fail);
                }
                if member.rho + 1e-12 < rho_min {
                    c = c.remedy(Remedy::at_least(
                        SubjectRef::new(bid, format!("{mpath}.rho"), lc("ρ", "ρ")),
                        q_dim(member.rho),
                        q_dim(rho_min),
                        lc(&format!("Increase ρ to at least {rho_min:.4}.",), &format!("ρ auf mindestens {rho_min:.4} erhöhen.")),
                    ));
                } else if member.rho > rho_max + 1e-12 {
                    c = c.remedy(Remedy::at_most(
                        SubjectRef::new(bid, format!("{mpath}.rho"), lc("ρ", "ρ")),
                        q_dim(member.rho),
                        q_dim(rho_max),
                        lc(&format!("Reduce ρ to at most {rho_max:.4}.",), &format!("ρ auf höchstens {rho_max:.4} senken.")),
                    ));
                }
                report.push(c.build());

                {
                    let mut sc = CheckResult::assess(
                        format!("en1998.1.{bid}.member.{}.rc.noSteelClass", member.id),
                        "DIN EN 1998-1",
                        ClauseId::new("EN 1998-1", "1", "5"),
                        SubjectRef::new(bid, format!("{mpath}.steelSectionClass"), lc(&member.id, &member.id)),
                        lc("RC member has no steel section class", "Stahlbetonbauteil ohne Stahl-Querschnittsklasse"),
                    )
                    .utilization(q_dim(member.steel_section_class as f64), q_dim(0.5))
                    .annex(annex)
                    .explanation(lc(
                        &format!("steelSectionClass={} (must be 0 for RC)", member.steel_section_class),
                        &format!("Stahl-Querschnittsklasse={} (muss 0 für Stahlbeton sein)", member.steel_section_class),
                    ));
                    if member.steel_section_class != 0 {
                        sc = sc.status(semio_s_plugin_norm_compliance::CheckStatus::Fail).remedy(Remedy::exactly(
                            SubjectRef::new(bid, format!("{mpath}.steelSectionClass"), lc("Steel section class", "Stahl-Querschnittsklasse")),
                            q_dim(member.steel_section_class as f64),
                            q_dim(0.0),
                            lc("Set steelSectionClass to 0 for RC members.", "Stahl-Querschnittsklasse für Stahlbeton auf 0 setzen."),
                        ));
                    }
                    report.push(sc.build());
                }

                let w_min = part_1::rc_omega_wd_min(gov_dc);
                if w_min > 0.0 && member.role.to_ascii_lowercase() == "column" {
                    let ok = member.omega_wd + 1e-12 >= w_min;
                    let mut c = CheckResult::assess(
                        format!("en1998.1.{bid}.member.{}.rc.omegaWd", member.id),
                        "DIN EN 1998-1",
                        ClauseId::new("EN 1998-1", "1", "5.4.3.2.2"),
                        SubjectRef::new(bid, format!("{mpath}.omegaWd"), lc(&member.id, &member.id)),
                        lc("RC confinement ω_wd", "Stahlbeton-Umschnürung ω_wd"),
                    )
                    .minimum(q_dim(member.omega_wd), q_dim(w_min))
                    .annex(annex)
                    .explanation(lc(&format!("ω_wd={:.3}, required≥{w_min:.3}", member.omega_wd), &format!("ω_wd={:.3}, erforderlich≥{w_min:.3}", member.omega_wd)));
                    if !ok {
                        c = c.remedy(Remedy::at_least(
                            SubjectRef::new(bid, format!("{mpath}.omegaWd"), lc("ω_wd", "ω_wd")),
                            q_dim(member.omega_wd),
                            q_dim(w_min),
                            lc(&format!("Increase ω_wd to at least {w_min:.3}.",), &format!("ω_wd auf mindestens {w_min:.3} erhöhen.")),
                        ));
                    }
                    report.push(c.build());
                }
                if member.role.to_ascii_lowercase() == "beam" {
                    let ratio = if member.rho > 1e-12 { member.rho_prime / member.rho } else { 0.0 };
                    let ok = ratio + 1e-12 >= 0.5;
                    let mut c = CheckResult::assess(
                        format!("en1998.1.{bid}.member.{}.rc.rhoPrime", member.id),
                        "DIN EN 1998-1",
                        ClauseId::new("EN 1998-1", "1", "5.4.3.1.2"),
                        SubjectRef::new(bid, format!("{mpath}.rhoPrime"), lc(&member.id, &member.id)),
                        lc("Beam ρ′/ρ ≥ 0.5", "Träger ρ′/ρ ≥ 0.5"),
                    )
                    .minimum(q_dim(ratio), q_dim(0.5))
                    .annex(annex)
                    .explanation(lc(&format!("beam reinforcement ratio ρ′/ρ={ratio:.3}"), &format!("Träger-Bewehrungsverhältnis ρ′/ρ={ratio:.3}")));
                    if !ok {
                        let req = 0.5 * member.rho;
                        c = c.remedy(Remedy::at_least(
                            SubjectRef::new(bid, format!("{mpath}.rhoPrime"), lc("ρ′", "ρ′")),
                            q_dim(member.rho_prime),
                            q_dim(req),
                            lc(&format!("Increase ρ′ to at least {req:.4}.",), &format!("ρ′ auf mindestens {req:.4} erhöhen.")),
                        ));
                    }
                    report.push(c.build());
                }
                if member.role.to_ascii_lowercase() != "column" {
                    let mut c = CheckResult::assess(
                        format!("en1998.1.{bid}.member.{}.rc.omegaWdBeam", member.id),
                        "DIN EN 1998-1",
                        ClauseId::new("EN 1998-1", "1", "5.4.3.2.2"),
                        SubjectRef::new(bid, format!("{mpath}.omegaWd"), lc(&member.id, &member.id)),
                        lc("RC beam confinement ω_wd (recorded)", "Stahlbetonbalken-Umschnürung ω_wd (dokumentiert)"),
                    )
                    .utilization(q_dim(member.omega_wd), q_dim(0.30))
                    .annex(annex)
                    .explanation(lc(
                        &format!("ω_wd={:.3} (beam; limit 0.30 for excessive confinement steel)", member.omega_wd),
                        &format!("ω_wd={:.3} (Balken; Grenze 0.30 für übermässige Umschnürung)", member.omega_wd),
                    ));
                    if member.omega_wd > 0.30 {
                        c = c.remedy(Remedy::at_most(
                            SubjectRef::new(bid, format!("{mpath}.omegaWd"), lc("ω_wd", "ω_wd")),
                            q_dim(member.omega_wd),
                            q_dim(0.30),
                            lc("Reduce beam ω_wd to at most 0.30.", "Balken-ω_wd auf höchstens 0.30 senken."),
                        ));
                    }
                    report.push(c.build());
                }
            }
            if mat == "steel" {
                let max_class = part_1::steel_max_section_class(q);
                let ok = member.steel_section_class > 0 && member.steel_section_class <= max_class;
                let mut c = CheckResult::assess(
                    format!("en1998.1.{bid}.member.{}.steel.class", member.id),
                    "DIN EN 1998-1",
                    ClauseId::new("EN 1998-1", "1", "6.5"),
                    SubjectRef::new(bid, format!("{mpath}.steelSectionClass"), lc(&member.id, &member.id)),
                    lc("Steel section class vs q", "Stahl-Querschnittsklasse vs q"),
                )
                .utilization(q_dim(if ok { (member.steel_section_class as f64).min(max_class as f64) } else { member.steel_section_class as f64 }), q_dim(max_class as f64))
                .annex(annex)
                .explanation(lc(&format!("class={}, max for q={q:.2} is {max_class}", member.steel_section_class), &format!("Klasse={}, max für q={q:.2} ist {max_class}", member.steel_section_class)));
                if !ok {
                    c = c.remedy(Remedy::at_most(
                        SubjectRef::new(bid, format!("{mpath}.steelSectionClass"), lc("Section class", "Querschnittsklasse")),
                        q_dim(member.steel_section_class as f64),
                        q_dim(max_class as f64),
                        lc(&format!("Use section class ≤ {max_class} (or reduce q0).",), &format!("Querschnittsklasse ≤ {max_class} verwenden (oder q0 senken).")),
                    ));
                }
                report.push(c.build());
            }

            // Legacy flag still reported (summary of detailing claim)
            let ok = member.detailing_compatible_with_q;
            let mut mcheck = CheckResult::assess(
                format!("en1998.1.{bid}.member.{}.qDetail", member.id),
                "DIN EN 1998-1",
                ClauseId::new("EN 1998-1", "1", "5.2"),
                SubjectRef::new(bid, format!("{mpath}.detailingCompatibleWithQ"), lc(&member.id, &member.id)),
                lc("q-compatible detailing declared", "q-gerechte Durchbildung erklärt"),
            )
            .utilization(q_dim(if ok { 0.5 } else { 2.0 }), q_dim(1.0))
            .annex(annex)
            .explanation(lc(
                &format!("Member {} detailing_compatible_with_q={ok} for q={q:.2}", member.id),
                &format!("Bauteil {} detailing_compatible_with_q={ok} für q={q:.2}", member.id),
            ));
            if !ok {
                mcheck = mcheck.remedy(Remedy::exactly(
                    SubjectRef::new(bid, format!("{mpath}.detailingCompatibleWithQ"), lc("Detailing flag", "Durchbildungsflag")),
                    q_dim(0.0),
                    q_dim(1.0),
                    lc("Provide q-compatible detailing (or reduce q0).", "q-gerechte Durchbildung herstellen (oder q0 reduzieren)."),
                ));
            }
            report.push(mcheck.build());
        }

        {
            let zone = document.site.seismic_zone.as_u8();
            let limit = part_1::masonry_wall_ratio_limit(zone, &document.site.importance_class);
            let claiming = building.claims_simple_masonry || system.system_type == "masonry";
            let mut m = CheckResult::assess(
                format!("en1998.1.{bid}.masonry.simple"),
                "DIN EN 1998-1/NA",
                ClauseId::new("EN 1998-1", "1", "9.7.2"),
                SubjectRef::new(bid, format!("{bpath}.masonryWallAreaRatio"), blabel.clone()),
                lc("Simple masonry building wall ratio (DE Table NA.12)", "Einfache Mauerwerksbauten Wandanteil (DE Tab. NA.12)"),
            )
            .utilization(q_dim(building.masonry_wall_area_ratio), q_dim(1.0))
            .annex(annex)
            .explanation(lc(
                &format!("claims={}, wall ratio={:.3}, required≥{:.3}", claiming, building.masonry_wall_area_ratio, limit),
                &format!("Beansprucht={}, Wandanteil={:.3}, erforderlich≥{:.3}", claiming, building.masonry_wall_area_ratio, limit),
            ));
            if claiming && building.masonry_wall_area_ratio + 1e-12 < limit {
                m = m.status(semio_s_plugin_norm_compliance::CheckStatus::Fail).remedy(Remedy::at_least(
                    SubjectRef::new(bid, format!("{bpath}.masonryWallAreaRatio"), lc("Wall area ratio", "Wandflächenanteil")),
                    q_dim(building.masonry_wall_area_ratio),
                    q_dim(limit),
                    lc(&format!("Increase masonry wall area ratio to at least {limit:.3}."), &format!("Mauerwerks-Wandanteil auf mindestens {limit:.3} erhöhen.")),
                ));
            } else {
                m = m.status(semio_s_plugin_norm_compliance::CheckStatus::Pass);
            }
            report.push(m.build());
        }
    }
}
