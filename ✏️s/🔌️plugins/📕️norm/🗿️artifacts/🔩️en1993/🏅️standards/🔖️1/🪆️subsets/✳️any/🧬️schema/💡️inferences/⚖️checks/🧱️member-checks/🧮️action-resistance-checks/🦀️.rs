//! 🧮️ Action and resistance checks: the ultimate action rows against the member resistances and the serviceability deflection of one member of the full steel structure check. See [`super`].

use super::*;
use super::member_profile::{deflection_shape, moment_shape, shear_shape, util, STATIONS};

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
    for action_row in uls_rows(combinations) {
        let a = &action_row.action;
        let n_rd = part_1_1::axial_resistance_n(a_res, material.fy, params);
        let m_y_rd = part_1_1::bending_resistance_nm(w_y, material.fy, class, params);
        let m_z_rd = part_1_1::bending_resistance_nm(w_z, material.fy, class, params);
        let v_z_rd = part_1_1::shear_resistance_n(section.shear_area_z, material.fy, params);
        let v_y_rd = part_1_1::shear_resistance_n(section.shear_area_y, material.fy, params);
        let m_y_rd_red = if class <= 2 { part_1_1::shear_reduced_m_y_rd(section, class, material.fy, a.vz, v_z_rd, params) } else { m_y_rd };
        check_uls_tension_and_compression(annex, report, params, member, section, material, kind, class, w_y, action_row, a, n_rd, m_y_rd_red);
        check_uls_shear_and_axial_bending(annex, report, params, member, section, material, class, w_y, action_row, a, n_rd, m_z_rd, v_z_rd, v_y_rd, m_y_rd_red);
        check_uls_buckling(annex, report, params, member, section, material, kind, w_y, iy, iz, curve_y, curve_z, lambda_y, lambda_z, chi_y, chi_z, n_b_rd, m_cr, lambda_lt, curve_lt, chi_lt, m_b_rd, action_row, a);
        check_uls_mz_and_torsion(annex, report, params, member, section, material, kind, action_row, a, m_z_rd);
        check_uls_interaction(annex, report, params, member, section, material, kind, class, a_res, w_y, w_z, lambda_y, lambda_z, chi_y, chi_z, chi_lt, action_row, a);
    }
}

/// 🧱️ Checks the tension and the compression resistance of one governing ultimate action row.
#[allow(clippy::too_many_arguments, reason = "the extracted check keeps the member resistances of its original function as explicit parameters")]
fn check_uls_tension_and_compression(
    annex: AnnexChoice,
    report: &mut CheckReport,
    params: AnnexParams,
    member: &SteelMember,
    section: &SteelSection,
    material: &SteelMaterial,
    kind: &str,
    class: u8,
    w_y: f64,
    action_row: &CombinedEffects,
    a: &semio_s_artifact_norm_en1993_model::DesignAction,
    n_rd: f64,
    m_y_rd_red: f64,
) {
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
        ))
        .sampled(STATIONS, |_| (util(a.n, n_rd), vec![force_n(a.n), force_n(n_rd)]));
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
        report.push(builder.ndps(&[AnnexParams::GAMMA_M0]).ndp_reads().build());
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
        report.push(builder.ndps(&[AnnexParams::GAMMA_M0, AnnexParams::GAMMA_M2]).ndp_reads().build());
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
        ))
        .sampled(STATIONS, |s| {
            let m = a.my.abs() * moment_shape(member, s);
            (util(m, m_y_rd_red), vec![moment_nm(m), moment_nm(m_y_rd_red)])
        });
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
        report.push(builder.ndps(&[AnnexParams::GAMMA_M0]).ndp_reads().build());
    }
}

/// 🪵️ Checks the shear resistance and the bending with axial force of one governing ultimate action row.
#[allow(clippy::too_many_arguments, reason = "the extracted check keeps the member resistances of its original function as explicit parameters")]
fn check_uls_shear_and_axial_bending(
    annex: AnnexChoice,
    report: &mut CheckReport,
    params: AnnexParams,
    member: &SteelMember,
    section: &SteelSection,
    material: &SteelMaterial,
    class: u8,
    w_y: f64,
    action_row: &CombinedEffects,
    a: &semio_s_artifact_norm_en1993_model::DesignAction,
    n_rd: f64,
    m_z_rd: f64,
    v_z_rd: f64,
    v_y_rd: f64,
    m_y_rd_red: f64,
) {
    for (axis, v_ed, v_rd, leaf, av) in [("z", a.vz.abs(), v_z_rd, "shearAreaZ", section.shear_area_z), ("y", a.vy.abs(), v_y_rd, "shearAreaY", section.shear_area_y)] {
        if v_ed <= 0.0 && axis == "y" {
            continue;
        }
        let mut builder = CheckResult::assess(
            format!("en1993.6.2.6.v{axis}.{}", action_row.combination_id),
            "DIN EN 1993-1-1",
            ClauseId::new("EN 1993-1-1", "6", "6.2.6"),
            subject_member(member, &format!("members[id={}].id", member.id)),
            loc(&format!("Shear resistance V{axis}"), &format!("Querkrafttragfähigkeit V{axis}")),
        )
        .annex(annex)
        .utilization(force_n(v_ed), force_n(v_rd))
        .explanation(loc(
            &format!("V_{axis},Ed={:.1} kN, V_pl,{axis},Rd={:.1} kN.", v_ed / 1000.0, v_rd / 1000.0),
            &format!("Querkraft V_{axis},Ed={:.1} kN gegenüber V_pl,{axis},Rd={:.1} kN.", v_ed / 1000.0, v_rd / 1000.0),
        ))
        .sampled(STATIONS, |s| {
            let v = v_ed * shear_shape(member, s);
            (util(v, v_rd), vec![force_n(v), force_n(v_rd)])
        });
        if v_ed > v_rd {
            let av_req = v_ed * 3.0_f64.sqrt() * params.gamma_m0 / material.fy;
            builder = builder.remedy(Remedy::at_least(
                subject_member(member, &format!("sections[id={}].{leaf}", section.id)),
                area_m2(av),
                area_m2(av_req),
                loc(
                    &format!("Increase shear area to at least {:.1} cm².", av_req * 1e4),
                    &format!("Schubfläche auf mindestens {:.1} cm² erhöhen.", av_req * 1e4),
                ),
            ));
        }
        report.push(builder.ndps(&[AnnexParams::GAMMA_M0]).ndp_reads().build());
    }

    {
        let eta = part_1_1::section_mn_eta(section, a.n, a.my, a.mz, n_rd, m_y_rd_red, m_z_rd, class);
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
        ))
        .sampled(STATIONS, |s| {
            let f = moment_shape(member, s);
            (part_1_1::section_mn_eta(section, a.n, a.my * f, a.mz * f, n_rd, m_y_rd_red, m_z_rd, class), vec![force_n(a.n), moment_nm(a.my * f)])
        });
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
        report.push(builder.ndps(&[AnnexParams::GAMMA_M0]).ndp_reads().build());
    }
}

/// 🔩️ Checks the flexural and the lateral torsional buckling resistance of one governing ultimate action row.
#[allow(clippy::too_many_arguments, reason = "the extracted check keeps the member resistances of its original function as explicit parameters")]
fn check_uls_buckling(
    annex: AnnexChoice,
    report: &mut CheckReport,
    params: AnnexParams,
    member: &SteelMember,
    section: &SteelSection,
    material: &SteelMaterial,
    kind: &str,
    w_y: f64,
    iy: f64,
    iz: f64,
    curve_y: part_1_1::BucklingCurve,
    curve_z: part_1_1::BucklingCurve,
    lambda_y: f64,
    lambda_z: f64,
    chi_y: f64,
    chi_z: f64,
    n_b_rd: f64,
    m_cr: f64,
    lambda_lt: f64,
    curve_lt: part_1_1::BucklingCurve,
    chi_lt: f64,
    m_b_rd: f64,
    action_row: &CombinedEffects,
    a: &semio_s_artifact_norm_en1993_model::DesignAction,
) {
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
        ))
        .sampled(STATIONS, |_| (util(a.n, n_b_rd), vec![force_n(a.n), force_n(n_b_rd)]));
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
        report.push(builder.ndps(&[AnnexParams::GAMMA_M1]).ndp_reads().build());
    }

    if a.my.abs() > 0.0 && kind_allows_ltb(kind) && part_1_1::ltb_not_applicable(section) {
        report.push(
            CheckResult::assess(
                format!("en1993.6.3.2.mb.{}", action_row.combination_id),
                "DIN EN 1993-1-1",
                ClauseId::new("EN 1993-1-1", "6", "6.3.2"),
                subject_member(member, &format!("sections[id={}].kind", section.id)),
                loc("Lateral-torsional buckling", "Biegedrillknicken"),
            )
            .annex(annex)
            .not_applicable(loc(
                "EN 1993-1-1 6.3.2 and Table 6.4 give the lateral-torsional buckling resistance of rolled and welded I or H sections; they contain no rule for an angle, so no M_b,Rd is derived. Bending of the angle is verified by the cross-section rows (6.2.9.2).",
                "DIN EN 1993-1-1 6.3.2 und Tabelle 6.4 geben den Biegedrillknickwiderstand gewalzter und geschweißter I- und H-Querschnitte an; für einen Winkel enthalten sie keine Regel, daher wird kein M_b,Rd bestimmt. Die Biegung des Winkels wird mit den Querschnittszeilen (6.2.9.2) nachgewiesen.",
            ))
            .build(),
        );
    }
    if a.my.abs() > 0.0 && kind_allows_ltb(kind) && !part_1_1::ltb_not_applicable(section) {
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
        ))
        .sampled(STATIONS, |s| {
            let m = a.my.abs() * moment_shape(member, s);
            (util(m, m_b_rd), vec![moment_nm(m), moment_nm(m_b_rd)])
        });
        if a.my.abs() > m_b_rd {
            let mut lo2 = 0.1;
            let mut hi2 = member.ltb_length.max(1.0);
            for _ in 0..40 {
                let mid = 0.5 * (lo2 + hi2);
                let mcr = part_1_1::m_cr_rolled_nm(section, material, mid, member.end_moment_ratio_psi, member.load_application.as_str(), member.moment_diagram.as_str());
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
        report.push(builder.ndps(&[AnnexParams::GAMMA_M1]).ndp_reads().build());
    }
}

/// 🪢️ English name of the end conditions of the warping support for the explanation.
fn warping_support_en(support: semio_s_artifact_norm_en1993_model::WarpingSupport) -> &'static str {
    match support {
        semio_s_artifact_norm_en1993_model::WarpingSupport::Fixed => "warping-restrained",
        semio_s_artifact_norm_en1993_model::WarpingSupport::FixedFork => "one warping-restrained and one fork",
        semio_s_artifact_norm_en1993_model::WarpingSupport::Cantilever => "cantilever (restrained and free)",
        semio_s_artifact_norm_en1993_model::WarpingSupport::Fork => "fork",
    }
}

/// 🪡️ German name of the end conditions of the warping support for the explanation.
fn warping_support_de(support: semio_s_artifact_norm_en1993_model::WarpingSupport) -> &'static str {
    match support {
        semio_s_artifact_norm_en1993_model::WarpingSupport::Fixed => "beidseitig wölbbehinderten Enden",
        semio_s_artifact_norm_en1993_model::WarpingSupport::FixedFork => "einem wölbbehinderten und einem Gabelende",
        semio_s_artifact_norm_en1993_model::WarpingSupport::Cantilever => "Kragarmlagerung (eingespannt und frei)",
        semio_s_artifact_norm_en1993_model::WarpingSupport::Fork => "Gabellagerung",
    }
}

/// 🧭️ Checks the minor-axis bending resistance (6.2.5), the St. Venant torsion shear and the warping normal stress of the bimoment (6.2.7) of one ultimate action row.
fn check_uls_mz_and_torsion(
    annex: AnnexChoice,
    report: &mut CheckReport,
    params: AnnexParams,
    member: &SteelMember,
    section: &SteelSection,
    material: &SteelMaterial,
    kind: &str,
    action_row: &CombinedEffects,
    a: &semio_s_artifact_norm_en1993_model::DesignAction,
    m_z_rd: f64,
) {
    if kind_allows_bending(kind) && a.mz.abs() > 0.0 {
        let mut builder = CheckResult::assess(
            format!("en1993.6.2.5.mz.{}", action_row.combination_id),
            "DIN EN 1993-1-1",
            ClauseId::new("EN 1993-1-1", "6", "6.2.5"),
            subject_member(member, &format!("members[id={}].id", member.id)),
            loc("Bending resistance Mz", "Biegetragfähigkeit Mz"),
        )
        .annex(annex)
        .utilization(moment_nm(a.mz.abs()), moment_nm(m_z_rd))
        .explanation(loc(
            &format!("M_z,Ed={:.1} kNm, M_z,c,Rd={:.1} kNm.", a.mz.abs() / 1000.0, m_z_rd / 1000.0),
            &format!("M_z,Ed={:.1} kNm, M_z,c,Rd={:.1} kNm.", a.mz.abs() / 1000.0, m_z_rd / 1000.0),
        ))
        .sampled(STATIONS, |s| {
            let m = a.mz.abs() * moment_shape(member, s);
            (util(m, m_z_rd), vec![moment_nm(m), moment_nm(m_z_rd)])
        });
        if a.mz.abs() > m_z_rd {
            let options = next_section_options(section, section.area, section.w_pl_y);
            if !options.is_empty() {
                builder = builder.remedy(Remedy::one_of(
                    subject_member(member, &format!("members[id={}].sectionId", member.id)),
                    options.clone(),
                    loc(&format!("Upsize section: {}.", options.join(", ")), &format!("Querschnitt vergrößern: {}.", options.join(", "))),
                ));
            }
        }
        report.push(builder.ndps(&[AnnexParams::GAMMA_M0]).ndp_reads().build());
    }
    if a.t.abs() > 0.0 && section.it > 0.0 {
        let t_max = section.tf.max(section.tw);
        let tau = a.t.abs() * t_max / section.it;
        let tau_rd = material.fy / 3.0_f64.sqrt() / params.gamma_m0;
        let mut builder = CheckResult::assess(
            format!("en1993.6.2.7.t.{}", action_row.combination_id),
            "DIN EN 1993-1-1",
            ClauseId::new("EN 1993-1-1", "6", "6.2.7"),
            subject_member(member, &format!("members[id={}].id", member.id)),
            loc("Torsion shear stress", "Torsionsschubspannung"),
        )
        .annex(annex)
        .utilization(stress_pa(tau), stress_pa(tau_rd))
        .explanation(loc(
            &format!("τ_t,Ed={:.1} MPa = T_Ed t_max/I_t, τ_Rd={:.1} MPa (St. Venant share, warping neglected).", tau / 1e6, tau_rd / 1e6),
            &format!("τ_t,Ed={:.1} MPa = T_Ed t_max/I_t, τ_Rd={:.1} MPa (St.-Venant-Anteil, Wölbkrafttorsion vernachlässigt).", tau / 1e6, tau_rd / 1e6),
        ));
        if tau > tau_rd {
            let it_req = a.t.abs() * t_max / tau_rd;
            builder = builder.remedy(Remedy::at_least(
                subject_member(member, &format!("sections[id={}].it", section.id)),
                moment_nm(section.it),
                moment_nm(it_req),
                loc(&format!("Increase I_t to at least {:.1} cm⁴.", it_req * 1e8), &format!("I_t auf mindestens {:.1} cm⁴ erhöhen.", it_req * 1e8)),
            ));
        }
        report.push(builder.ndps(&[AnnexParams::GAMMA_M0]).ndp_reads().build());
    }

    if a.t.abs() > 0.0 && section.iw > 0.0 && section.it > 0.0 && member.length > 0.0 && part_1_1::is_i_section(section.kind) {
        let udl = member.torsion_load == semio_s_artifact_norm_en1993_model::TorsionLoad::Udl;
        let l = member.length;
        let mt = if udl { a.t.abs() / l } else { a.t.abs() };
        let xa = member.torque_position.clamp(0.0, 1.0) * l;
        let ends = crate::clauses::ends_of(member.warping_support);
        let profile = crate::clauses::warping_torsion::warping_torsion(l, material.e_modulus * section.iw, material.g_modulus * section.it, mt, xa, udl, ends, 200).unwrap_or_default();
        let b_max = profile.maxima().3;
        let omega_max = section.b * (section.h - section.tf) / 4.0;
        let w_w = section.iw / omega_max;
        let sigma_w = b_max / w_w;
        let sigma_n = a.n.abs() / section.area + a.my.abs() / section.w_el_y + a.mz.abs() / section.w_el_z;
        let sigma = sigma_n + sigma_w;
        let limit = material.fy / params.gamma_m0;
        let mut builder = CheckResult::assess(
            format!("en1993.6.2.7.w.{}", action_row.combination_id),
            "DIN EN 1993-1-1",
            ClauseId::new("EN 1993-1-1", "6", "6.2.7"),
            subject_member(member, &format!("members[id={}].torsionLoad", member.id)),
            loc("Warping normal stress from the bimoment", "Wölbnormalspannung aus dem Wölbbimoment"),
        )
        .annex(annex)
        .utilization(stress_pa(sigma), stress_pa(limit))
        .explanation(loc(
            &format!("Member with {} ends, {} torque T_Ed={:.2} kNm{}: B_Ed,max={:.3} kNm² (exact solution of the warping torsion equation E I_w θ'''' - G I_t θ'' = m for the end conditions), W_ω=I_w/ω_max={:.3e} m⁴ with ω_max=b(h-t_f)/4 of the doubly symmetric I section, σ_w,Ed={:.1} MPa; σ_N+M={:.1} MPa; σ_tot={:.1} MPa against f_y/γM0={:.1} MPa (elastic criterion; the combination of 6.2.7(5)-(9) is not verified against the print).", warping_support_en(member.warping_support), if udl { "uniform" } else { "point" }, a.t.abs() / 1e3, if udl { String::new() } else { format!(" at {:.2} m", xa) }, b_max / 1e3, w_w, sigma_w / 1e6, sigma_n / 1e6, sigma / 1e6, limit / 1e6),
            &format!("Bauteil mit {}, {} Torsionsmoment T_Ed={:.2} kNm{}: B_Ed,max={:.3} kNm² (exakte Lösung der Wölbtorsionsgleichung E I_w θ'''' - G I_t θ'' = m für die Lagerbedingungen), W_ω=I_w/ω_max={:.3e} m⁴ mit ω_max=b(h-t_f)/4 des doppeltsymmetrischen I-Querschnitts, σ_w,Ed={:.1} MPa; σ_N+M={:.1} MPa; σ_tot={:.1} MPa gegen f_y/γM0={:.1} MPa (elastisches Kriterium; die Kombination nach 6.2.7(5)-(9) ist nicht am Druck geprüft).", warping_support_de(member.warping_support), if udl { "gleichmäßig verteiltes" } else { "Einzel-" }, a.t.abs() / 1e3, if udl { String::new() } else { format!(" bei {:.2} m", xa) }, b_max / 1e3, w_w, sigma_w / 1e6, sigma_n / 1e6, sigma / 1e6, limit / 1e6),
        ));
        if sigma > limit {
            let required = section.iw * (sigma_w / (limit - sigma_n).max(limit * 0.05));
            builder = builder.remedy(Remedy::at_least(
                subject_member(member, &format!("sections[id={}].iw", section.id)),
                moment_nm(section.iw),
                moment_nm(required),
                loc(&format!("Increase the warping constant I_w to about {:.3e} m⁶ (first estimate) or add torsional restraints.", required), &format!("Wölbwiderstand I_w auf etwa {:.3e} m⁶ erhöhen (erste Schätzung) oder Torsionshalterungen anordnen.", required)),
            ));
        }
        report.push(builder.ndps(&[AnnexParams::GAMMA_M0]).ndp_reads().build());
    }

    let (hw, tw, fy) = ((section.h - 2.0 * section.tf) * 1e3, section.tw * 1e3, material.fy / 1e6);
    let eps = crate::clauses::classification::epsilon(fy);
    let eta = crate::clauses::resistance::eta_shear(fy);
    if a.vz.abs() > 0.0 && crate::clauses::resistance::shear_buckling_check_required(hw, tw, eps, eta, None) {
        let lambda_w = hw / (86.4 * tw * eps);
        let chi_w = crate::clauses::resistance::chi_w(lambda_w, eta, true);
        let v_bw = crate::clauses::resistance::v_bw_rd(chi_w, fy, hw, tw, params.gamma_m1);
        let mut vbw =
            CheckResult::assess(
                format!("en1993.1-5.5.vbw.{}", action_row.combination_id),
                "DIN EN 1993-1-5",
                ClauseId::new("EN 1993-1-5", "5", "5.2"),
                subject_member(member, &format!("members[id={}].sectionId", member.id)),
                loc("Shear buckling of the web", "Schubbeulen des Stegs"),
            )
            .annex(annex)
            .utilization(force_n(a.vz.abs()), force_n(v_bw))
            .ndp(AnnexParams::GAMMA_M1)
            .explanation(loc(
                &format!("V_Ed={:.1} kN, V_bw,Rd={:.1} kN (λ̄_w={:.3}, χ_w={:.3}, unstiffened web, rigid end post).", a.vz.abs() / 1000.0, v_bw / 1000.0, lambda_w, chi_w),
                &format!("V_Ed={:.1} kN, V_bw,Rd={:.1} kN (λ̄_w={:.3}, χ_w={:.3}, unausgesteifter Steg, starre Endsteife).", a.vz.abs() / 1000.0, v_bw / 1000.0, lambda_w, chi_w),
            ));
        if !(a.vz.abs() <= v_bw) {
            let k = if (a.vz.abs() / v_bw).is_finite() { a.vz.abs() / v_bw } else { 2.0 };
            vbw = vbw.remedy(Remedy::at_least(
                subject_member(member, &format!("sections[id={}].tw", section.id)),
                length_m(section.tw),
                length_m(section.tw * k),
                loc(&format!("Increase the web thickness to about {:.1} mm (first estimate) or add transverse web stiffeners.", section.tw * k * 1e3), &format!("Stegdicke auf etwa {:.1} mm erhöhen (erste Schätzung) oder Quersteifen anordnen.", section.tw * k * 1e3)),
            ));
        }
        report.push(vbw.build());
    }
}

/// ⚙️ Checks the combined bending and axial interaction 6.61/6.62 by Annex B and, where defined, Annex A for one ultimate action row.
#[allow(clippy::too_many_arguments, reason = "the extracted check keeps the member resistances of its original function as explicit parameters")]
fn check_uls_interaction(
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
    lambda_y: f64,
    lambda_z: f64,
    chi_y: f64,
    chi_z: f64,
    chi_lt: f64,
    action_row: &CombinedEffects,
    a: &semio_s_artifact_norm_en1993_model::DesignAction,
) {
    if !(kind_allows_member_interaction(kind) && (a.n > 0.0 || a.my.abs() > 0.0 || a.mz.abs() > 0.0)) {
        return;
    }
    use crate::clauses::stability as st;
    let n_ed = a.n.max(0.0);
    let cm = match member.moment_diagram {
        semio_s_artifact_norm_en1993_model::MomentDiagram::Uniform => 1.0,
        semio_s_artifact_norm_en1993_model::MomentDiagram::Parabolic => st::cm_alpha_h(0.0, member.end_moment_ratio_psi, true),
        _ => st::cm_linear(member.end_moment_ratio_psi),
    };
    let chi_lt_used = if kind_allows_ltb(kind) { chi_lt } else { 1.0 };
    let base = st::InteractionInput {
        n_ed,
        my_ed: a.my.abs(),
        mz_ed: a.mz.abs(),
        n_rk: a_res * material.fy,
        my_rk: w_y * material.fy,
        mz_rk: w_z * material.fy,
        chi_y,
        chi_z,
        chi_lt: chi_lt_used,
        lambda_y,
        lambda_z,
        cmy: cm,
        cmz: cm,
        cm_lt: cm,
        class,
        susceptible_torsion: kind_allows_ltb(kind),
        rhs: false,
        gm1: params.gamma_m1,
        delta_my: if class == 4 { n_ed * part_1_1::e_n_y_class4(section, material.fy).abs() } else { 0.0 },
        delta_mz: 0.0,
    };
    let b = st::interaction_annex_b(&base);
    let eta_b = b.eta_61.max(b.eta_62);
    let along_b = |s: f64| {
        let f = moment_shape(member, s);
        let r = st::interaction_annex_b(&st::InteractionInput { my_ed: base.my_ed * f, mz_ed: base.mz_ed * f, ..base });
        r.eta_61.max(r.eta_62)
    };
    let mut results: Vec<(&str, f64, String, String, Box<dyn Fn(f64) -> f64 + '_>)> = vec![(
        "B",
        eta_b,
        format!("Annex B: η_61={:.3}, η_62={:.3} (k_yy={:.3}, k_yz={:.3}, k_zy={:.3}, k_zz={:.3}).", b.eta_61, b.eta_62, b.kyy, b.kyz, b.kzy, b.kzz),
        format!("Anhang B: η_61={:.3}, η_62={:.3} (k_yy={:.3}, k_yz={:.3}, k_zy={:.3}, k_zz={:.3}).", b.eta_61, b.eta_62, b.kyy, b.kyz, b.kzy, b.kzz),
        Box::new(along_b),
    )];
    let ncr_y = st::n_cr(material.e_modulus * section.iy, member.buckling_length_y);
    let ncr_z = st::n_cr(material.e_modulus * section.iz, member.buckling_length_z);
    let l_t = member.ltb_length.max(member.ltb_restraint_spacing).max(1e-6);
    let i0_sq = (section.iy + section.iz) / section.area;
    let ncr_t = st::n_cr_torsional(material.g_modulus, section.it, material.e_modulus, section.iw, l_t, i0_sq);
    if n_ed > 1e-6 * base.n_rk && n_ed < ncr_y.min(ncr_z).min(ncr_t) && class <= 4 && kind_allows_ltb(kind) {
        let mcr0 = st::mcr_closed_form(material.e_modulus, material.g_modulus, section.iz, section.it, section.iw, l_t, 1.0, 1.0, 1.0, 0.0, 0.0);
        let aa = st::AnnexAInput {
            base,
            ncr_y,
            ncr_z,
            ncr_t,
            ncr_tf: ncr_t,
            wel_y: section.w_el_y,
            wpl_y: section.w_pl_y,
            wel_z: section.w_el_z,
            wpl_z: section.w_pl_z,
            mpl_y_rd: section.w_pl_y * material.fy / params.gamma_m1,
            mpl_z_rd: section.w_pl_z * material.fy / params.gamma_m1,
            it: section.it,
            iy: section.iy,
            a_area: a_res,
            c1: st::c1_end_moments(member.end_moment_ratio_psi),
            lambda_0: (w_y * material.fy / mcr0).sqrt(),
            cmy0: cm,
            cmz0: cm,
        };
        let r = st::interaction_annex_a(&aa);
        let eta_a = r.eta_61.max(r.eta_62);
        if eta_a.is_finite() {
            results.push((
                "A",
                eta_a,
                format!("Annex A: η_61={:.3}, η_62={:.3} (k_yy={:.3}, k_zz={:.3}).", r.eta_61, r.eta_62, r.kyy, r.kzz),
                format!("Anhang A: η_61={:.3}, η_62={:.3} (k_yy={:.3}, k_zz={:.3}).", r.eta_61, r.eta_62, r.kyy, r.kzz),
                Box::new(move |s: f64| {
                    let f = moment_shape(member, s);
                    let r = st::interaction_annex_a(&st::AnnexAInput { base: st::InteractionInput { my_ed: aa.base.my_ed * f, mz_ed: aa.base.mz_ed * f, ..aa.base }, ..aa });
                    r.eta_61.max(r.eta_62)
                }),
            ));
        }
    }
    for (method, eta, en, de, along) in results {
        let mut builder = CheckResult::assess(
            format!("en1993.6.3.3.int{}.{}", method.to_ascii_lowercase(), action_row.combination_id),
            "DIN EN 1993-1-1",
            ClauseId::new("EN 1993-1-1", "6", if method == "A" { "6.3.3/A" } else { "6.3.3/B" }),
            subject_member(member, &format!("members[id={}].id", member.id)),
            loc(&format!("Member buckling interaction 6.61/6.62, Annex {method}"), &format!("Bauteilinteraktion 6.61/6.62, Anhang {method}")),
        )
        .annex(annex)
        .utilization(dimensionless(eta), dimensionless(1.0))
        .explanation(loc(&en, &de))
        .sampled(STATIONS, |s| (along(s), vec![dimensionless(along(s))]));
        if eta > 1.0 || !eta.is_finite() {
            let options = next_section_options(section, section.area * eta.max(1.0), w_y * eta.max(1.0));
            if !options.is_empty() {
                builder = builder.remedy(Remedy::one_of(
                    subject_member(member, &format!("members[id={}].sectionId", member.id)),
                    options.clone(),
                    loc(&format!("Upsize section: {}.", options.join(", ")), &format!("Querschnitt vergrößern: {}.", options.join(", "))),
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
        report.push(builder.ndps(&[AnnexParams::GAMMA_M1]).ndp_reads().build());
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
        ))
        .sampled(STATIONS, |s| {
            let w = delta * deflection_shape(member, s);
            (util(w, limit), vec![length_m(w), moment_nm(action_row.action.my.abs())])
        });
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
        report.push(builder.ndps(&[AnnexParams::GAMMA_M1]).ndp_reads().build());
    }
}
