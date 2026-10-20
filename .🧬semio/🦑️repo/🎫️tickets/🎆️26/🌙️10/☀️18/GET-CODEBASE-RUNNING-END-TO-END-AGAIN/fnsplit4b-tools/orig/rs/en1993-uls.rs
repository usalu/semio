//! 🧮️ Action and resistance checks: the ultimate action rows against the member resistances and the serviceability deflection of one member of the full steel structure check. See [`super`].

use super::*;

/// 🧮 Checks every governing ultimate action row of the member against the axial, bending, shear, buckling and interaction resistances.
pub(super) fn check_uls_actions(
    annex: AnnexChoice,
    report: &mut CheckReport,
    params: AnnexParams,
    member: &SteelMember,
    section: &SteelSection,
    material: &SteelMaterial,
    kind: &str,
    class: u8,
    a_res: f64,
    w_y: f64,
    w_z: f64,
    iy: f64,
    iz: f64,
    curve_y: part_1_1::BucklingCurve,
    curve_z: part_1_1::BucklingCurve,
    lambda_y: f64,
    lambda_z: f64,
    chi_y: f64,
    chi_z: f64,
    n_b_rd_y: f64,
    n_b_rd_z: f64,
    n_b_rd: f64,
    m_cr: f64,
    lambda_lt: f64,
    curve_lt: part_1_1::BucklingCurve,
    chi_lt: f64,
    m_b_rd: f64,
    combinations: &[CombinedEffects],
) {
    let uls_actions: Vec<&CombinedEffects> = governing_effects(combinations, "uls").into_iter().collect();
    for action_row in uls_actions {
        let a = &action_row.action;
        let n_rd = part_1_1::axial_resistance_n(a_res, material.fy, params);
        let m_y_rd = part_1_1::bending_resistance_nm(w_y, material.fy, class, params);
        let m_z_rd = part_1_1::bending_resistance_nm(w_z, material.fy, class, params);
        let v_z_rd = part_1_1::shear_resistance_n(section.shear_area_z, material.fy, params);
        let v_y_rd = part_1_1::shear_resistance_n(section.shear_area_y, material.fy, params);
        let m_y_rd_red = part_1_1::shear_reduced_m_rd(m_y_rd, a.vz, v_z_rd);

        if a.n >= 0.0 {
        if kind_allows_compression(kind) {
            let mut builder = CheckResult::assess(
                format!("en1993.6.2.4.n.{}", action_row.combination_id),
                "DIN EN 1993-1-1",
                ClauseId::new("EN 1993-1-1", "6", "6.2.4"),
                subject_member(member, &format!("members[id={}].id", member.id)),
                loc("Cross-section compression", "Querschnitt Druck"),
            )
            .annex(annex)
            .utilization(force_n(a.n), force_n(n_rd))
            .explanation(loc(
                &format!("N_Ed={:.1} kN, N_Rd={:.1} kN (A·f_y/γ_M0).", a.n / 1000.0, n_rd / 1000.0),
                &format!("Druckkraft N_Ed={:.1} kN gegenüber Querschnittstragfähigkeit N_Rd={:.1} kN (A·f_y/γ_M0).", a.n / 1000.0, n_rd / 1000.0),
            ));
            if a.n > n_rd {
                let a_req = a.n * params.gamma_m0 / material.fy;
                let options = next_section_options(section, a_req, w_y);
                if !options.is_empty() {
                    builder = builder.remedy(Remedy::one_of(
                        subject_member(member, &format!("members[id={}].sectionId", member.id)),
                        options.clone(),
                        loc(
                            &format!("Upsize section: {}.", options.join(", ")),
                            &format!("Querschnitt vergrößern: {}.", options.join(", ")),
                        ),
                    ));
                } else {
                    builder = builder.remedy(Remedy::at_least(
                        subject_member(member, &format!("sections[id={}].area", section.id)),
                        area_m2(section.area),
                        area_m2(a_req),
                        loc(
                            &format!("Increase section area to at least {:.1} cm².", a_req * 1e4),
                            &format!("Querschnittsfläche auf mindestens {:.1} cm² erhöhen.", a_req * 1e4),
                        ),
                    ));
                }
            }
            report.push(builder.build());
        }
        } else {
            let n_t_rd = part_1_1::net_tension_resistance_n(section.area, section.area_net, material.fy, material.fu, params);
            let n_t = -a.n;
            let mut builder = CheckResult::assess(
                format!("en1993.6.2.3.nt.{}", action_row.combination_id),
                "DIN EN 1993-1-1",
                ClauseId::new("EN 1993-1-1", "6", "6.2.3"),
                subject_member(member, &format!("members[id={}].id", member.id)),
                loc("Net section tension", "Zugtragfähigkeit Nettoquerschnitt"),
            )
            .annex(annex)
            .utilization(force_n(n_t), force_n(n_t_rd))
            .explanation(loc(
                &format!("N_t,Ed={:.1} kN, N_t,Rd={:.1} kN.", n_t / 1000.0, n_t_rd / 1000.0),
                &format!("Zug N_t,Ed={:.1} kN, N_t,Rd={:.1} kN.", n_t / 1000.0, n_t_rd / 1000.0),
            ));
            if n_t > n_t_rd {
                let a_net_req = n_t * params.gamma_m2 / (0.9 * material.fu);
                builder = builder.remedy(Remedy::at_least(
                    subject_member(member, &format!("sections[id={}].areaNet", section.id)),
                    area_m2(section.area_net),
                    area_m2(a_net_req),
                    loc(
                        &format!("Increase net area from {:.1} cm² to at least {:.1} cm².", section.area_net * 1e4, a_net_req * 1e4),
                        &format!("Nettoquerschnitt von {:.1} cm² auf mindestens {:.1} cm² erhöhen.", section.area_net * 1e4, a_net_req * 1e4),
                    ),
                ));
            }
            report.push(builder.build());
        }

        if kind_allows_bending(kind) {
            let mut builder = CheckResult::assess(
                format!("en1993.6.2.5.my.{}", action_row.combination_id),
                "DIN EN 1993-1-1",
                ClauseId::new("EN 1993-1-1", "6", "6.2.5"),
                subject_member(member, &format!("members[id={}].id", member.id)),
                loc("Bending resistance My", "Biegetragfähigkeit My"),
            )
            .annex(annex)
            .utilization(moment_nm(a.my.abs()), moment_nm(m_y_rd_red))
            .explanation(loc(
                &format!("M_y,Ed={:.1} kNm, M_c,Rd={:.1} kNm (shear-reduced).", a.my.abs() / 1000.0, m_y_rd_red / 1000.0),
                &format!("M_y,Ed={:.1} kNm, M_c,Rd={:.1} kNm (abgemindert bei Querkraft).", a.my.abs() / 1000.0, m_y_rd_red / 1000.0),
            ));
            if a.my.abs() > m_y_rd_red {
                let w_req = a.my.abs() * params.gamma_m0 / material.fy;
                let options = next_section_options(section, section.area, w_req);
                if !options.is_empty() {
                    builder = builder.remedy(Remedy::one_of(
                        subject_member(member, &format!("members[id={}].sectionId", member.id)),
                        options.clone(),
                        loc(
                            &format!("Upsize section: {}.", options.join(", ")),
                            &format!("Querschnitt vergrößern: {}.", options.join(", ")),
                        ),
                    ));
                } else {
                    let (leaf, current_w) = if class <= 2 { ("wPlY", section.w_pl_y) } else { ("wElY", section.w_el_y) };
                    let w_clear = w_req.max(current_w) * 1.01;
                    builder = builder.remedy(Remedy::at_least(
                        subject_member(member, &format!("sections[id={}].{leaf}", section.id)),
                        moment_nm(current_w),
                        moment_nm(w_clear),
                        loc(
                            &format!("Increase {leaf} so M_c,Rd covers M_y,Ed."),
                            &format!("{leaf} erhöhen, damit M_c,Rd die M_y,Ed deckt."),
                        ),
                    ));
                }
            }
            report.push(builder.build());
        }

        {
            let v_ed = a.vz.abs().max(a.vy.abs());
            let v_rd = if a.vz.abs() >= a.vy.abs() { v_z_rd } else { v_y_rd };
            let mut builder = CheckResult::assess(
                format!("en1993.6.2.6.v.{}", action_row.combination_id),
                "DIN EN 1993-1-1",
                ClauseId::new("EN 1993-1-1", "6", "6.2.6"),
                subject_member(member, &format!("members[id={}].id", member.id)),
                loc("Shear resistance", "Querkrafttragfähigkeit"),
            )
            .annex(annex)
            .utilization(force_n(v_ed), force_n(v_rd))
            .explanation(loc(
                &format!("V_Ed={:.1} kN, V_pl,Rd={:.1} kN.", v_ed / 1000.0, v_rd / 1000.0),
                &format!("Querkraft V_Ed={:.1} kN gegenüber V_pl,Rd={:.1} kN.", v_ed / 1000.0, v_rd / 1000.0),
            ));
            if v_ed > v_rd {
                let av_req = v_ed * 3.0_f64.sqrt() * params.gamma_m0 / material.fy;
                builder = builder.remedy(Remedy::at_least(
                    subject_member(member, &format!("sections[id={}].shearAreaZ", section.id)),
                    area_m2(section.shear_area_z),
                    area_m2(av_req),
                    loc(
                        &format!("Increase shear area to at least {:.1} cm².", av_req * 1e4),
                        &format!("Schubfläche auf mindestens {:.1} cm² erhöhen.", av_req * 1e4),
                    ),
                ));
            }
            report.push(builder.build());
        }

        {
            let eta = part_1_1::mn_interaction_eta(
                a.n,
                a.my,
                a.mz,
                n_rd,
                m_y_rd_red,
                m_z_rd,
                section.area,
                section.b,
                section.tf,
                class,
            );
            let mut builder = CheckResult::assess(
                format!("en1993.6.2.9.mn.{}", action_row.combination_id),
                "DIN EN 1993-1-1",
                ClauseId::new("EN 1993-1-1", "6", "6.2.9"),
                subject_member(member, &format!("members[id={}].id", member.id)),
                loc("M+N interaction", "M+N Interaktion"),
            )
            .annex(annex)
            .utilization(dimensionless(eta), dimensionless(1.0))
            .explanation(loc(
                &format!("η={:.3} from §6.2.9/6.2.10 (n, a, M_N,y/M_N,z, α/β).", eta),
                &format!("η={:.3} nach §6.2.9/6.2.10 (n, a, M_N,y/M_N,z, α/β).", eta),
            ));
            if eta > 1.0 || !eta.is_finite() {
                let options = next_section_options(section, section.area * eta.max(1.0), w_y * eta.max(1.0));
                if !options.is_empty() {
                    builder = builder.remedy(Remedy::one_of(
                        subject_member(member, &format!("members[id={}].sectionId", member.id)),
                        options.clone(),
                        loc(
                            &format!("Upsize section: {}.", options.join(", ")),
                            &format!("Querschnitt vergrößern: {}.", options.join(", ")),
                        ),
                    ));
                } else {
                    builder = builder.remedy(Remedy::at_least(
                        subject_member(member, &format!("sections[id={}].area", section.id)),
                        area_m2(section.area),
                        area_m2(section.area * eta.max(1.0)),
                        loc(
                            &format!("Increase section area to clear M+N interaction."),
                            &format!("Querschnittsfläche erhöhen, damit M+N-Interaktion erfüllt ist."),
                        ),
                    ));
                }
            }
            report.push(builder.build());
        }

        if a.n > 0.0 && kind_allows_flexural_buckling(kind) {
            let mut builder = CheckResult::assess(
                format!("en1993.6.3.1.nb.{}", action_row.combination_id),
                "DIN EN 1993-1-1",
                ClauseId::new("EN 1993-1-1", "6", "6.3.1"),
                subject_member(member, &format!("members[id={}].bucklingLengthY", member.id)),
                loc("Flexural buckling", "Biegeknicken"),
            )
            .annex(annex)
            .utilization(force_n(a.n), force_n(n_b_rd))
            .explanation(loc(
                &format!(
                    "N_Ed={:.1} kN, N_b,Rd={:.1} kN (χ_y={:.3} λ̄_y={:.3}, χ_z={:.3} λ̄_z={:.3}, γ_M1={}).",
                    a.n / 1000.0,
                    n_b_rd / 1000.0,
                    chi_y,
                    lambda_y,
                    chi_z,
                    lambda_z,
                    params.gamma_m1
                ),
                &format!(
                    "Biegeknicken: N_Ed={:.1} kN, N_b,Rd={:.1} kN (χ_y={:.3} λ̄_y={:.3}, χ_z={:.3} λ̄_z={:.3}, γ_M1={}).",
                    a.n / 1000.0,
                    n_b_rd / 1000.0,
                    chi_y,
                    lambda_y,
                    chi_z,
                    lambda_z,
                    params.gamma_m1
                ),
            ));
            if a.n > n_b_rd {
                let mut lo = 0.1;
                let mut hi = member.buckling_length_y;
                for _ in 0..40 {
                    let mid = 0.5 * (lo + hi);
                    let lam = part_1_1::lambda_bar(mid, iy.min(iz), material.e_modulus, material.fy);
                    let curve = if curve_y.alpha() >= curve_z.alpha() { curve_y } else { curve_z };
                    let ch = part_1_1::chi(lam, curve);
                    let nb = part_1_1::buckling_resistance_n(section.area, material.fy, ch, params);
                    if nb >= a.n {
                        lo = mid;
                    } else {
                        hi = mid;
                    }
                }
                let l_req = lo;
                builder = builder.remedy(Remedy::at_most(
                    subject_member(member, &format!("members[id={}].bucklingLengthY", member.id)),
                    length_m(member.buckling_length_y),
                    length_m(l_req),
                    loc(
                        &format!("Reduce buckling length from {:.2} m to at most {:.2} m (add restraints).", member.buckling_length_y, l_req),
                        &format!("Knicklänge von {:.2} m auf höchstens {:.2} m reduzieren (Halterungen).", member.buckling_length_y, l_req),
                    ),
                ));
                let options = next_section_options(section, section.area * (a.n / n_b_rd), w_y);
                if !options.is_empty() {
                    builder = builder.remedy(Remedy::one_of(
                        subject_member(member, &format!("members[id={}].sectionId", member.id)),
                        options.clone(),
                        loc(
                            &format!("Or select larger section: {}.", options.join(", ")),
                            &format!("Oder größeren Querschnitt wählen: {}.", options.join(", ")),
                        ),
                    ));
                }
            }
            report.push(builder.build());
        }

        if a.my.abs() > 0.0 && kind_allows_ltb(kind) {
            let mut builder = CheckResult::assess(
                format!("en1993.6.3.2.mb.{}", action_row.combination_id),
                "DIN EN 1993-1-1",
                ClauseId::new("EN 1993-1-1", "6", "6.3.2"),
                subject_member(member, &format!("members[id={}].ltbRestraintSpacing", member.id)),
                loc("Lateral-torsional buckling", "Biegedrillknicken"),
            )
            .annex(annex)
            .utilization(moment_nm(a.my.abs()), moment_nm(m_b_rd))
            .explanation(loc(
                &format!("M_y,Ed={:.1} kNm, M_b,Rd={:.1} kNm (M_cr={:.1} kNm, χ_LT={:.3}, λ̄_LT={:.3}).", a.my.abs() / 1000.0, m_b_rd / 1000.0, m_cr / 1000.0, chi_lt, lambda_lt),
                &format!("Biegedrillknicken M_y,Ed={:.1} kNm, M_b,Rd={:.1} kNm (M_cr={:.1} kNm, χ_LT={:.3}, λ̄_LT={:.3}).", a.my.abs() / 1000.0, m_b_rd / 1000.0, m_cr / 1000.0, chi_lt, lambda_lt),
            ));
            if a.my.abs() > m_b_rd {
                let mut lo = 0.2;
                let mut hi = member.ltb_restraint_spacing.max(member.ltb_length);
                for _ in 0..40 {
                    let mid = 0.5 * (lo + hi);
                    let mcr = part_1_1::m_cr_rolled_nm(section, material, mid, member.end_moment_ratio_psi, &member.load_application, &member.moment_diagram);
                    let lam = part_1_1::lambda_lt(w_y, material.fy, mcr);
                    let ch = part_1_1::chi_lt(lam, curve_lt);
                    let mb = part_1_1::ltb_resistance_nm(w_y, material.fy, ch, params);
                    if mb >= a.my.abs() {
                        lo = mid;
                        lo = mid;
                    } else {
                        hi = mid;
                    }
                }
                let mut ok = 0.2;
                let mut step = member.ltb_length;
                while step > 0.05 {
                    let trial = ok + step;
                    let mcr = part_1_1::m_cr_rolled_nm(section, material, trial, member.end_moment_ratio_psi, &member.load_application, &member.moment_diagram);
                    let lam = part_1_1::lambda_lt(w_y, material.fy, mcr);
                    let ch = part_1_1::chi_lt(lam, curve_lt);
                    let mb = part_1_1::ltb_resistance_nm(w_y, material.fy, ch, params);
                    if mb >= a.my.abs() {
                        ok = trial;
                    }
                    step *= 0.5;
                }
                let mut lo2 = 0.1;
                let mut hi2 = member.ltb_length.max(1.0);
                for _ in 0..40 {
                    let mid = 0.5 * (lo2 + hi2);
                    let mcr = part_1_1::m_cr_rolled_nm(section, material, mid, member.end_moment_ratio_psi, &member.load_application, &member.moment_diagram);
                    let lam = part_1_1::lambda_lt(w_y, material.fy, mcr);
                    let ch = part_1_1::chi_lt(lam, curve_lt);
                    let mb = part_1_1::ltb_resistance_nm(w_y, material.fy, ch, params);
                    if mb >= a.my.abs() {
                        lo2 = mid;
                    } else {
                        hi2 = mid;
                    }
                }
                let spacing_req = lo2;
                builder = builder.remedy(Remedy::at_most(
                    subject_member(member, &format!("members[id={}].ltbRestraintSpacing", member.id)),
                    length_m(member.ltb_restraint_spacing),
                    length_m(spacing_req),
                    loc(
                        &format!("Provide LTB restraints at most every {:.2} m (currently {:.2} m).", spacing_req, member.ltb_restraint_spacing),
                        &format!("BDK-Halterungen höchstens alle {:.2} m anordnen (aktuell {:.2} m).", spacing_req, member.ltb_restraint_spacing),
                    ),
                ));
            }
            report.push(builder.build());
        }

        if kind_allows_member_interaction(kind) && (a.n > 0.0 || a.my.abs() > 0.0 || a.mz.abs() > 0.0) {
            let n_rk = section.area * material.fy;
            let (kyy, kyz, kzy, kzz) = part_1_1::interaction_kij(a.n, n_rk, lambda_y, lambda_z, member.end_moment_ratio_psi, class);
            let (eta61, eta62) = part_1_1::interaction_eta(a.n, n_b_rd_y, n_b_rd_z, a.my, a.mz, m_b_rd.max(m_y_rd_red), m_z_rd, kyy, kyz, kzy, kzz);
            let eta = eta61.max(eta62);
            let mut builder = CheckResult::assess(
                format!("en1993.6.3.3.int.{}", action_row.combination_id),
                "DIN EN 1993-1-1",
                ClauseId::new("EN 1993-1-1", "6", "6.3.3"),
                subject_member(member, &format!("members[id={}].id", member.id)),
                loc("Member buckling interaction 6.61/6.62", "Bauteilinteraktion 6.61/6.62"),
            )
            .annex(annex)
            .utilization(dimensionless(eta), dimensionless(1.0))
            .explanation(loc(
                &format!("η_61={:.3}, η_62={:.3} (k_yy={:.3}, k_zz={:.3}).", eta61, eta62, kyy, kzz),
                &format!("Nachweis: η_61={:.3}, η_62={:.3} (k_yy={:.3}, k_zz={:.3}).", eta61, eta62, kyy, kzz),
            ));
            if eta > 1.0 || !eta.is_finite() {
                let options = next_section_options(section, section.area * eta.max(1.0), w_y * eta.max(1.0));
                if !options.is_empty() {
                    builder = builder.remedy(Remedy::one_of(
                        subject_member(member, &format!("members[id={}].sectionId", member.id)),
                        options.clone(),
                        loc(
                            &format!("Upsize section: {}.", options.join(", ")),
                            &format!("Querschnitt vergrößern: {}.", options.join(", ")),
                        ),
                    ));
                }
                let l_req = (member.buckling_length_y / eta.max(1.01)).max(0.1);
                builder = builder.remedy(Remedy::at_most(
                    subject_member(member, &format!("members[id={}].bucklingLengthY", member.id)),
                    length_m(member.buckling_length_y),
                    length_m(l_req),
                    loc(
                        &format!("Reduce buckling length from {:.2} m to at most {:.2} m.", member.buckling_length_y, l_req),
                        &format!("Knicklänge von {:.2} m auf höchstens {:.2} m reduzieren.", member.buckling_length_y, l_req),
                    ),
                ));
            }
            report.push(builder.build());
        }
    }
}

/// 📐 Checks the deflection of the characteristic combination against the member limit.
pub(super) fn check_sls_deflection(
    annex: AnnexChoice,
    report: &mut CheckReport,
    member: &SteelMember,
    section: &SteelSection,
    material: &SteelMaterial,
    w_y: f64,
    combinations: &[CombinedEffects],
) {
    if let Some(action_row) = governing_effects(combinations, "sls") {
        let delta = part_1_1::deflection_from_moment(action_row.action.my.abs(), member.length, material.e_modulus, section.iy);
        let limit = member.length / member.deflection_limit_ratio.max(1.0);
        let mut builder = CheckResult::assess(
            format!("en1993.7.sls.{}", action_row.combination_id),
            "DIN EN 1993-1-1",
            ClauseId::new("EN 1993-1-1", "7", "7.2"),
            subject_member(member, &format!("members[id={}].deflectionLimitRatio", member.id)),
            loc("SLS deflection", "Verformung GZG"),
        )
        .annex(annex)
        .utilization(length_m(delta), length_m(limit))
        .explanation(loc(
            &format!(
                "δ={:.1} mm ≤ L/{}={:.1} mm (characteristic {}).",
                delta * 1000.0, member.deflection_limit_ratio, limit * 1000.0, action_row.combination_id
            ),
            &format!(
                "δ={:.1} mm ≤ L/{}={:.1} mm (charakteristische Kombination {}).",
                delta * 1000.0, member.deflection_limit_ratio, limit * 1000.0, action_row.combination_id
            ),
        ));
        if delta > limit {
            let options = next_section_options(section, section.area, w_y);
            if !options.is_empty() {
                builder = builder.remedy(Remedy::one_of(
                    subject_member(member, &format!("members[id={}].sectionId", member.id)),
                    options.clone(),
                    loc(
                        &format!("Upsize section: {}.", options.join(", ")),
                        &format!("Querschnitt vergrößern: {}.", options.join(", ")),
                    ),
                ));
            } else {
                let i_req = section.iy * (delta / limit.max(1e-12)) * 1.01;
                builder = builder.remedy(Remedy::at_least(
                    subject_member(member, &format!("sections[id={}].iy", section.id)),
                    moment_nm(section.iy),
                    moment_nm(i_req),
                    loc(
                        "Increase I_y so the characteristic deflection is within L/limit.",
                        "I_y erhöhen, damit die charakteristische Durchbiegung innerhalb L/Grenzwert liegt.",
                    ),
                ));
            }
        }
        report.push(builder.build());
    }
}
