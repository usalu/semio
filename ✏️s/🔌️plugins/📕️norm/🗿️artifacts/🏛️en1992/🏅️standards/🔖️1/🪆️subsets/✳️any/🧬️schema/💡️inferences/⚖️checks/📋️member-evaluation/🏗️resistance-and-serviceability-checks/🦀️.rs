//! 🏗️ Resistance and serviceability checks: the ultimate resistances, the crack width, the reinforcement limits, the deflection and the serviceability stress limits of the member evaluation. See [`super`].

use super::*;

/// 🏗 Checks the ultimate resistances of the governing ultimate situation.
pub(super) fn check_ultimate_resistances(
    doc: &En1992Snapshot,
    member: &RcMember,
    out: &mut Vec<CheckResult>,
    annex: AnnexChoice,
    f_ck: f64,
    f_yk: f64,
    e_s: f64,
    eps_cu2: f64,
    n_parabola: f64,
    b: f64,
    d: f64,
    h: f64,
    a_s: f64,
    rho: f64,
    uls: Option<&DesignEffects>,
    acc: Option<&DesignEffects>,
) {
    if let Some(uls) = uls {
        let params = na_de::AnnexParams::for_situation(annex, &uls.situation);
        let m_rd = part_1_1::flexural_resistance_nm(f_ck, b, d, a_s, f_yk, uls.n_ed, annex, eps_cu2, n_parabola, e_s);
        let tension_layer = member.longitudinal.iter().find(|l| l.position == "bottom" || l.position == "tension").or_else(|| member.longitudinal.first());
        let mut flex = CheckResult::assess(format!("en1992.6.1.flexure.{}", member.id), part_11(), clause_11("§6.1"), member_ref(doc, member, &member_path(&member.id, "actions")), l("Bending with axial force", "Biegung mit Normalkraft"))
            .utilization(moment_nm(uls.m_ed.abs()), moment_nm(m_rd.max(1.0)))
            .annex(annex)
            .explanation(l(
                &format!("|M_Ed|={:.1} kNm ≤ M_Rd={:.1} kNm (α_cc={:.2}, governing {})", uls.m_ed.abs() / 1e3, m_rd / 1e3, params.alpha_cc, uls.combination_id),
                &format!("|M_Ed|={:.1} kNm ≤ M_Rd={:.1} kNm (α_cc={:.2}, maßgebend {})", uls.m_ed.abs() / 1e3, m_rd / 1e3, params.alpha_cc, uls.combination_id),
            ));
        if uls.m_ed.abs() > m_rd {
            let a_req = part_1_1::required_a_s_m2(uls.m_ed.abs(), f_ck, b, d, f_yk, uls.n_ed, annex, eps_cu2, n_parabola, e_s);
            if let Some(layer) = tension_layer {
                let bar_a = std::f64::consts::PI * (layer.diameter * 0.5).powi(2);
                let count_req = if bar_a > 0.0 { (a_req / bar_a).ceil() } else { layer.count as f64 + 1.0 };
                flex = flex.remedy(Remedy::at_least(
                    member_ref(doc, member, &layer_path(&member.id, &layer.id, "count")),
                    Quantity::new(QuantityKind::Dimensionless, layer.count as f64),
                    Quantity::new(QuantityKind::Dimensionless, count_req),
                    l(
                        &format!("Increase tension bar count of {} layer {} from {} to at least {:.0} (A_s,req≈{:.0} mm²).", member.id, layer.id, layer.count, count_req, a_req * 1e6),
                        &format!("Zugstabanzahl von {} Lage {} von {} auf mindestens {:.0} erhöhen (A_s,erf≈{:.0} mm²).", member.id, layer.id, layer.count, count_req, a_req * 1e6),
                    ),
                ));
            }
            flex = flex.remedy(Remedy::at_least(
                member_ref(doc, member, &member_path(&member.id, "effectiveDepth")),
                length_m(d),
                length_m(d * 1.15),
                l(
                    &format!("Increase effective depth of {} from {:.0} mm to at least {:.0} mm.", member.id, d * 1000.0, d * 1150.0),
                    &format!("Statische Nutzhöhe von {} von {:.0} mm auf mindestens {:.0} mm erhöhen.", member.id, d * 1000.0, d * 1150.0),
                ),
            ));
        }
        
        out.push(flex.build());
        if let Some(acc_e) = &acc {
            let params_acc = na_de::AnnexParams::for_situation(annex, "accidental");
            let f_cd_acc = params_acc.f_cd_pa(f_ck);
            let f_cd_uls = params.f_cd_pa(f_ck);
            let m_rd_acc = if f_cd_uls > 0.0 { m_rd * (f_cd_acc / f_cd_uls) } else { m_rd };
            let mut flex_acc = CheckResult::assess(format!("en1992.6.1.flexure.acc.{}", member.id), part_11(), clause_11("§6.1"), member_ref(doc, member, &member_path(&member.id, "actions")), l("Accidental bending ACC-6.11", "Außergewöhnliche Biegung ACC-6.11"))
                .utilization(moment_nm(acc_e.m_ed.abs()), moment_nm(m_rd_acc.max(1.0)))
                .annex(annex)
                .explanation(l(
                    &format!("ACC-6.11: |M_Ed|={:.1} kNm ≤ M_Rd={:.1} kNm (γ_c={:.2}, γ_s={:.2}, {})", acc_e.m_ed.abs() / 1e3, m_rd_acc / 1e3, params_acc.gamma_c, params_acc.gamma_s, acc_e.combination_id),
                    &format!("Außergewöhnlich ACC-6.11: |M_Ed|={:.1} kNm ≤ M_Rd={:.1} kNm (γ_c={:.2}, γ_s={:.2}, {})", acc_e.m_ed.abs() / 1e3, m_rd_acc / 1e3, params_acc.gamma_c, params_acc.gamma_s, acc_e.combination_id),
                ));
            if acc_e.m_ed.abs() > m_rd_acc {
                let d_req = d * (acc_e.m_ed.abs() / m_rd_acc.max(1.0)).max(1.0) * 1.05;
                flex_acc = flex_acc.remedy(Remedy::at_least(
                    member_ref(doc, member, &member_path(&member.id, "effectiveDepth")),
                    length_m(d),
                    length_m(d_req),
                    l("Increase effective depth for accidental combination.", "Statische Nutzhöhe für außergewöhnliche Kombination erhöhen."),
                ));
            }
            out.push(flex_acc.build());
        }

        let v_rd_c = part_1_1::shear_v_rd_c_n(b, d, f_ck, rho, uls.n_ed, annex);
        let z = 0.9 * d;
        let sigma_cp = if b * h > 0.0 { (-uls.n_ed).max(0.0) / (b * h) } else { 0.0 };
        let f_cd = params.f_cd_pa(f_ck);
        let cot = params.cot_theta(annex, sigma_cp, f_cd);
        let asw_s = member.asw_per_s();
        let f_ywd = params.f_yd_pa(f_yk);
        let v_rd_s = if asw_s > 0.0 { part_1_1::shear_v_rd_s_n(asw_s, z, f_ywd, cot) } else { 0.0 };
        let v_rd_max = part_1_1::shear_v_rd_max_n(b, z, f_ck, cot, annex);
        let _v_rd = if uls.v_ed <= v_rd_c { v_rd_c } else { v_rd_s.min(v_rd_max).max(v_rd_c) };
        let v_limit = if asw_s > 0.0 { v_rd_s.min(v_rd_max) } else { v_rd_c };
        let mut shear = CheckResult::assess(format!("en1992.6.2.shear.{}", member.id), part_11(), clause_11("§6.2"), member_ref(doc, member, &member_path(&member.id, "actions")), l("Shear resistance", "Querkrafttragfähigkeit"))
            .utilization(force_n(uls.v_ed), force_n(v_limit.max(1.0)))
            .annex(annex)
            .explanation(l(
                &format!("V_Ed={:.1} kN; V_Rd,c={:.1}, V_Rd,s={:.1}, V_Rd,max={:.1} kN; cotθ={:.2}; governing {}", uls.v_ed / 1e3, v_rd_c / 1e3, v_rd_s / 1e3, v_rd_max / 1e3, cot, uls.combination_id),
                &format!("V_Ed={:.1} kN; V_Rd,c={:.1}, V_Rd,s={:.1}, V_Rd,max={:.1} kN; cotθ={:.2}; maßgebend {}", uls.v_ed / 1e3, v_rd_c / 1e3, v_rd_s / 1e3, v_rd_max / 1e3, cot, uls.combination_id),
            ));
        if uls.v_ed > v_limit {
            if let Some(st) = &member.stirrups {
                let need = uls.v_ed / (z * f_ywd * cot).max(1.0);
                let a_bar = std::f64::consts::PI * (st.diameter * 0.5).powi(2) * st.legs as f64;
                let s_req = if need > 0.0 { a_bar / need } else { st.spacing };
                shear = shear.remedy(Remedy::at_most(
                    member_ref(doc, member, &member_path(&member.id, "stirrups.spacing")),
                    length_m(st.spacing),
                    length_m(s_req.min(st.spacing)),
                    l(
                        &format!("Reduce stirrup spacing of {} from {:.0} mm to at most {:.0} mm.", member.id, st.spacing * 1000.0, s_req * 1000.0),
                        &format!("Bügelabstand von {} von {:.0} mm auf höchstens {:.0} mm verringern.", member.id, st.spacing * 1000.0, s_req * 1000.0),
                    ),
                ));
            } else {
                let need = uls.v_ed / (z * f_ywd * cot).max(1.0);
                shear = shear.remedy(Remedy::at_least(
                    member_ref(doc, member, &member_path(&member.id, "stirrups")),
                    area_m2(0.0),
                    area_m2(need),
                    l(
                        &format!("Provide shear reinforcement Asw/s ≥ {:.1} mm²/m on {}.", need * 1e6, member.id),
                        &format!("Querkraftbewehrung Asw/s ≥ {:.1} mm²/m für {} anordnen.", need * 1e6, member.id),
                    ),
                ));
            }
        }
        out.push(shear.build());

        if uls.t_ed > 0.0 {
            let t_rd = part_1_1::torsion_t_rd_nm(f_ck, b, h, annex);
            let mut tor = CheckResult::assess(format!("en1992.6.3.torsion.{}", member.id), part_11(), clause_11("§6.3"), member_ref(doc, member, &member_path(&member.id, "actions")), l("Torsion resistance", "Torsionstragfähigkeit"))
                .utilization(moment_nm(uls.t_ed), moment_nm(t_rd.max(1.0)))
                .annex(annex)
                .explanation(l(
                    &format!("Torsion check: T_Ed={:.2} kNm ≤ T_Rd={:.2} kNm (gov {})", uls.t_ed / 1e3, t_rd / 1e3, uls.combination_id),
                    &format!("Torsionsnachweis: T_Ed={:.2} kNm ≤ T_Rd={:.2} kNm (maßgebend {})", uls.t_ed / 1e3, t_rd / 1e3, uls.combination_id),
                ));
            if uls.t_ed > t_rd {
                let h_req = h * (uls.t_ed / t_rd).sqrt();
                tor = tor.remedy(Remedy::at_least(
                    member_ref(doc, member, &member_path(&member.id, "height")),
                    length_m(h),
                    length_m(h_req),
                    l(
                        &format!("Increase section height of {} from {:.0} mm to at least {:.0} mm for torsion.", member.id, h * 1000.0, h_req * 1000.0),
                        &format!("Querschnittshöhe von {} von {:.0} mm auf mindestens {:.0} mm für Torsion erhöhen.", member.id, h * 1000.0, h_req * 1000.0),
                    ),
                ));
            }
            out.push(tor.build());
        }

        if matches!(member.kind, MemberKind::FlatSlab) {
            if let Some(p) = &member.punching {
                let beta = part_1_1::punching_beta(&p.column_position);
                let (u0, u1) = part_1_1::punching_perimeters(p.column_width, p.column_depth, d);
                let u0_over_d = u0 / d.max(1e-6);
                let mut v_rd_c_pa = part_1_1::punching_v_rd_c_pa(f_ck, d, rho, annex);
                if matches!(annex, AnnexChoice::De) {
                    let ppar = na_de::AnnexParams::for_choice(annex);
                    let f_ck_mpa = f_ck / 1.0e6;
                    let d_mm = d * 1000.0;
                    let k = (1.0 + (200.0 / d_mm).sqrt()).min(2.0);
                    let rho_u = rho.min(0.02);
                    let mut c_rd = 0.18 / ppar.gamma_c;
                    if u0_over_d < 4.0 {
                        c_rd *= (0.5 + 0.125 * u0_over_d).min(1.0);
                    }
                    let v_min = 0.035 * k.powf(1.5) * f_ck_mpa.sqrt();
                    v_rd_c_pa = (c_rd * k * (100.0 * rho_u * f_ck_mpa).powf(1.0 / 3.0)).max(v_min) * 1.0e6;
                }
                let v_rd_max_pa = part_1_1::punching_v_rd_max_pa(v_rd_c_pa, annex);
                let v_ed_pa = if u1 * d > 0.0 { beta * uls.v_ed_punch / (u1 * d) } else { 0.0 };
                let v_rd_cs_pa = if p.asw > 0.0 {
                    v_rd_c_pa + 0.75 * p.asw * params.f_yd_pa(f_yk) / (u1 * d).max(1e-9)
                } else {
                    v_rd_c_pa
                };
                let mut punch = CheckResult::assess(format!("en1992.6.4.punching.{}", member.id), part_11(), clause_11("§6.4"), member_ref(doc, member, &member_path(&member.id, "punching")), l("Punching shear", "Durchstanztragfähigkeit"))
                    .utilization(stress_pa(v_ed_pa), stress_pa(v_rd_cs_pa.max(1.0)))
                    .annex(annex)
                    .explanation(l(
                        &format!("Punching: v_Ed={:.2} MPa (β={:.2}, u₁={:.2} m, u₀/d={:.2}) ≤ v_Rd,c={:.2} MPa; asw={:.0} mm²/m ({})", v_ed_pa / 1e6, beta, u1, u0_over_d, v_rd_cs_pa / 1e6, p.asw * 1e6, uls.combination_id),
                        &format!("Durchstanzen: v_Ed={:.2} MPa (β={:.2}, u₁={:.2} m, u₀/d={:.2}) ≤ v_Rd,c={:.2} MPa; asw={:.0} mm²/m ({})", v_ed_pa / 1e6, beta, u1, u0_over_d, v_rd_cs_pa / 1e6, p.asw * 1e6, uls.combination_id),
                    ));
                if v_ed_pa > v_rd_cs_pa {
                    let d_req = if u1 > 0.0 && v_rd_c_pa > 0.0 { beta * uls.v_ed_punch / (u1 * v_rd_c_pa) } else { d * 1.2 };
                    punch = punch.remedy(Remedy::at_least(
                        member_ref(doc, member, &member_path(&member.id, "effectiveDepth")),
                        length_m(d),
                        length_m(d_req),
                        l(
                            &format!("Increase effective depth of {} from {:.0} mm to at least {:.0} mm.", member.id, d * 1000.0, d_req * 1000.0),
                            &format!("Statische Nutzhöhe von {} von {:.0} mm auf mindestens {:.0} mm erhöhen.", member.id, d * 1000.0, d_req * 1000.0),
                        ),
                    ));
                    punch = punch.remedy(Remedy::at_least(
                        member_ref(doc, member, &member_path(&member.id, "punching.asw")),
                        area_m2(p.asw),
                        area_m2((p.asw + 5.0e-4).max(5.0e-4)),
                        l(
                            &format!("Provide/increase punching reinforcement asw on {}.", member.id),
                            &format!("Durchstanzbewehrung asw für {} anordnen/erhöhen.", member.id),
                        ),
                    ));
                }
                out.push(punch.build());
                let mut pmax = CheckResult::assess(format!("en1992.6.4.punching-max.{}", member.id), part_11(), clause_11("§6.4.5"), member_ref(doc, member, &member_path(&member.id, "punching")), l("Punching crushing limit", "Durchstanzen Druckstrebe"))
                    .utilization(stress_pa(v_ed_pa), stress_pa(v_rd_max_pa.max(1.0)))
                    .annex(annex)
                    .explanation(l("Punching strut: v_Ed ≤ v_Rd,max (DE-NA 1.4·v_Rd,c)", "Durchstanz-Druckstrebe: v_Ed ≤ v_Rd,max (DE-NA 1,4·v_Rd,c)"));
                if v_ed_pa > v_rd_max_pa {
                    pmax = pmax.remedy(Remedy::at_least(
                        member_ref(doc, member, &member_path(&member.id, "effectiveDepth")),
                        length_m(d),
                        length_m(d * v_ed_pa / v_rd_max_pa),
                        l("Increase slab depth to satisfy punching crushing.", "Deckendicke erhöhen, um Durchstanz-Druckstrebe zu erfüllen."),
                    ));
                }
                out.push(pmax.build());
            }
        }
    } else {
        out.push(
            CheckResult::assess(format!("en1992.uls.na.{}", member.id), part_11(), clause_11("§6"), member_ref(doc, member, &member_path(&member.id, "actions")), l("ULS actions", "Einwirkungen GZT"))
                .not_applicable(l("No ULS load case on member", "Kein GZT-Lastfall am Bauteil"))
                .annex(annex)
                .build(),
        );
    }
}

/// 🪓 Checks the crack width of the quasi permanent situation.
pub(super) fn check_crack_width(
    doc: &En1992Snapshot,
    member: &RcMember,
    out: &mut Vec<CheckResult>,
    annex: AnnexChoice,
    f_ck: f64,
    e_s: f64,
    b: f64,
    d: f64,
    h: f64,
    a_s: f64,
    rho: f64,
    sls_qp: Option<&DesignEffects>,
) {
    if let Some(sls) = sls_qp {
        let w_max = part_1_1::w_max_m(member.exposure);
        let z = 0.9 * d;
        let sigma_s = if a_s * z > 0.0 { sls.m_ed.abs() / (a_s * z) } else { 0.0 };
        let f_ct_eff = part_1_1::f_ctm_pa(f_ck);
        let rho_p = if b * (2.5 * (member.cover + 0.5 * 0.016)).min(0.5 * h) > 0.0 {
            a_s / (b * (2.5 * (member.cover + 0.5 * 0.016)).min(0.5 * h))
        } else {
            rho
        };
        let s_r = 3.4 * member.cover + 0.425 * 0.8 * 0.5 * 0.016 / rho_p.max(1e-6);
        let w_k = part_1_1::crack_width_m(sigma_s, rho_p.max(1e-4), f_ct_eff, e_s, s_r);
        let mut crack = CheckResult::assess(format!("en1992.7.3.crack.{}", member.id), part_11(), clause_11("§7.3.4"), member_ref(doc, member, &member_path(&member.id, "actions")), l("Crack width", "Rissbreite"))
            .utilization(length_m(w_k), length_m(w_max))
            .annex(annex)
            .explanation(l(
                &format!("Crack width w_k={:.3} mm ≤ w_max={:.2} mm (SLS {})", w_k * 1000.0, w_max * 1000.0, sls.combination_id),
                &format!("Rissbreite w_k={:.3} mm ≤ w_max={:.2} mm (GZG {})", w_k * 1000.0, w_max * 1000.0, sls.combination_id),
            ));
        if w_k > w_max {
            let a_req = a_s * (w_k / w_max);
            if let Some(layer) = member.longitudinal.first() {
                let d_req = (layer.diameter * (a_req / a_s.max(1e-12)).sqrt()).max(layer.diameter * 1.05);
                crack = crack.remedy(Remedy::at_least(
                    member_ref(doc, member, &layer_path(&member.id, &layer.id, "diameter")),
                    length_m(layer.diameter),
                    length_m(d_req),
                    l(
                        &format!("Increase bar diameter of {} so A_s ≥ {:.0} mm² to limit crack width.", member.id, a_req * 1e6),
                        &format!("Stabdurchmesser von {} erhöhen, damit A_s ≥ {:.0} mm² (Rissbreite).", member.id, a_req * 1e6),
                    ),
                ));
            }
        }
        out.push(crack.build());
    }
}

/// 🔩 Checks the minimum and maximum tension reinforcement.
pub(super) fn check_reinforcement_limits(
    doc: &En1992Snapshot,
    member: &RcMember,
    out: &mut Vec<CheckResult>,
    annex: AnnexChoice,
    f_ck: f64,
    f_yk: f64,
    b: f64,
    d: f64,
    h: f64,
    a_s: f64,
) {
    let a_min = part_1_1::a_s_min_m2(f_ck, f_yk, b, d);
    let a_max = part_1_1::a_s_max_m2(b, h);
    let mut amin = CheckResult::assess(format!("en1992.9.minas.{}", member.id), part_11(), clause_11("§9.2.1"), member_ref(doc, member, &member_path(&member.id, "longitudinal")), l("Minimum reinforcement", "Mindestbewehrung"))
        .minimum(area_m2(a_s), area_m2(a_min))
        .annex(annex)
        .explanation(l(
            &format!("Minimum steel: A_s={:.0} mm² ≥ A_s,min={:.0} mm²", a_s * 1e6, a_min * 1e6),
            &format!("Mindestbewehrung: A_s={:.0} mm² ≥ A_s,min={:.0} mm²", a_s * 1e6, a_min * 1e6),
        ));
    if a_s < a_min {
        if let Some(layer) = member.longitudinal.first() {
            let d_req = (layer.diameter * (a_min / a_s.max(1e-12)).sqrt()).max(layer.diameter * 1.05);
            amin = amin.remedy(Remedy::at_least(
                member_ref(doc, member, &layer_path(&member.id, &layer.id, "diameter")),
                length_m(layer.diameter),
                length_m(d_req),
                l(
                    &format!("Increase bar diameter of {} so A_s ≥ {:.0} mm².", member.id, a_min * 1e6),
                    &format!("Stabdurchmesser von {} erhöhen, damit A_s ≥ {:.0} mm².", member.id, a_min * 1e6),
                ),
            ));
        }
    }
    out.push(amin.build());
    let mut amax = CheckResult::assess(format!("en1992.9.maxas.{}", member.id), part_11(), clause_11("§9.2.1"), member_ref(doc, member, &member_path(&member.id, "longitudinal")), l("Maximum reinforcement", "Höchstbewehrung"))
        .utilization(area_m2(a_s), area_m2(a_max.max(1e-9)))
        .annex(annex)
        .explanation(l(
            &format!("Maximum steel: A_s={:.0} mm² ≤ A_s,max={:.0} mm²", a_s * 1e6, a_max * 1e6),
            &format!("Höchstbewehrung: A_s={:.0} mm² ≤ A_s,max={:.0} mm²", a_s * 1e6, a_max * 1e6),
        ));
    if a_s > a_max {
        if let Some(layer) = member.longitudinal.first() {
            let d_req = (layer.diameter * (a_max / a_s.max(1e-12)).sqrt()).min(layer.diameter * 0.95).max(0.006);
            amax = amax.remedy(Remedy::at_most(
                member_ref(doc, member, &layer_path(&member.id, &layer.id, "diameter")),
                length_m(layer.diameter),
                length_m(d_req),
                l("Reduce bar diameter so reinforcement stays below 4% of Ac.", "Stabdurchmesser reduzieren, damit Bewehrung unter 4 % von Ac bleibt."),
            ));
        }
    }
    out.push(amax.build());
}

/// 📉 Checks the span to depth deflection limit.
pub(super) fn check_deflection(doc: &En1992Snapshot, member: &RcMember, out: &mut Vec<CheckResult>, annex: AnnexChoice, d: f64, rho: f64) {
    if member.span > 0.0 && matches!(member.kind, MemberKind::Beam | MemberKind::Slab | MemberKind::FlatSlab) {
        let lim_table = part_1_1::basic_ld_limit(member.support, rho.max(1e-4), 0.0);
        let k = part_1_1::support_k(member.support);
        let lim_de = part_1_1::de_ld_caps(k, member.span, member.deflection_sensitive);
        let lim = lim_table.min(lim_de);
        let ld = member.span / d.max(1e-6);
        let mut defl = CheckResult::assess(format!("en1992.7.4.ld.{}", member.id), part_11(), clause_11("§7.4.2"), member_ref(doc, member, &member_path(&member.id, "effectiveDepth")), l("Span-to-depth limit", "Biegeschlankheit l/d"))
            .utilization(Quantity::new(QuantityKind::Dimensionless, ld), Quantity::new(QuantityKind::Dimensionless, lim.max(1.0)))
            .annex(annex)
            .explanation(l(
                &format!("l/d={:.1} ≤ {:.1} (Table 7.4N={:.1}; DE K·35 / K²·150/l={:.1}; K={:.2}; sensitive={})", ld, lim, lim_table, lim_de, k, member.deflection_sensitive),
                &format!("l/d={:.1} ≤ {:.1} (Tab. 7.4N={:.1}; DE K·35 / K²·150/l={:.1}; K={:.2}; empfindlich={})", ld, lim, lim_table, lim_de, k, member.deflection_sensitive),
            ));
        if ld > lim {
            let d_req = member.span / lim;
            defl = defl.remedy(Remedy::at_least(
                member_ref(doc, member, &member_path(&member.id, "effectiveDepth")),
                length_m(d),
                length_m(d_req),
                l(
                    &format!("Increase effective depth of {} from {:.0} mm to at least {:.0} mm.", member.id, d * 1000.0, d_req * 1000.0),
                    &format!("Statische Nutzhöhe von {} von {:.0} mm auf mindestens {:.0} mm erhöhen.", member.id, d * 1000.0, d_req * 1000.0),
                ),
            ));
        }
        out.push(defl.build());
    }
}

/// ⚖ Checks the serviceability stress limits of the characteristic and quasi permanent situations.
pub(super) fn check_sls_stress_limits(
    doc: &En1992Snapshot,
    member: &RcMember,
    out: &mut Vec<CheckResult>,
    annex: AnnexChoice,
    concrete: &semio_s_artifact_norm_en1992_model::ConcreteGrade,
    f_ck: f64,
    f_yk: f64,
    e_s: f64,
    b: f64,
    d: f64,
    a_s: f64,
    sls_qp: Option<&DesignEffects>,
    sls_char: Option<&DesignEffects>,
) {
    if let Some(sc) = sls_char {
        let sig_s = part_1_1::cracked_sigma_s_pa(sc.m_ed, b, d, a_s, e_s, concrete.e_cm());
        let sig_c = part_1_1::cracked_sigma_c_pa(sc.m_ed, b, d, a_s, e_s, concrete.e_cm());
        let lim_s = 0.8 * f_yk;
        let harsh = matches!(member.exposure, ExposureClass::Xd1 | ExposureClass::Xd2 | ExposureClass::Xd3 | ExposureClass::Xs1 | ExposureClass::Xs2 | ExposureClass::Xs3 | ExposureClass::Xf1 | ExposureClass::Xf2 | ExposureClass::Xf3 | ExposureClass::Xf4);
        let lim_c = if harsh { 0.60 * f_ck } else { 0.60 * f_ck };
        let mut ss = CheckResult::assess(format!("en1992.7.2.sigma-s.{}", member.id), part_11(), clause_11("§7.2"), member_ref(doc, member, &member_path(&member.id, "longitudinal")), l("SLS steel stress", "GZG Stahlspannung"))
            .utilization(stress_pa(sig_s), stress_pa(lim_s.max(1.0)))
            .annex(annex)
            .explanation(l(
                &format!("σ_s={:.0} MPa ≤ 0.8 f_yk={:.0} MPa (characteristic {})", sig_s / 1e6, lim_s / 1e6, sc.combination_id),
                &format!("σ_s={:.0} MPa ≤ 0,8 f_yk={:.0} MPa (charakteristisch {})", sig_s / 1e6, lim_s / 1e6, sc.combination_id),
            ));
        if sig_s > lim_s {
            if let Some(layer) = member.longitudinal.first() {
                let a_need = a_s * sig_s / lim_s.max(1e-12);
                let d_req = (layer.diameter * (a_need / a_s.max(1e-12)).sqrt() * 1.2).max(layer.diameter * 1.05);
                ss = ss.remedy(Remedy::at_least(
                    member_ref(doc, member, &layer_path(&member.id, &layer.id, "diameter")),
                    length_m(layer.diameter),
                    length_m(d_req),
                    l("Increase bar diameter to reduce SLS steel stress.", "Stabdurchmesser erhöhen, um die GZG-Stahlspannung zu senken."),
                ));
            }
        }
        out.push(ss.build());
        let mut scc = CheckResult::assess(format!("en1992.7.2.sigma-c.{}", member.id), part_11(), clause_11("§7.2"), member_ref(doc, member, &member_path(&member.id, "concreteGradeId")), l("SLS concrete stress", "GZG Betondruckspannung"))
            .utilization(stress_pa(sig_c), stress_pa(lim_c.max(1.0)))
            .annex(annex)
            .explanation(l(
                &format!("σ_c={:.1} MPa ≤ {:.0}% f_ck={:.1} MPa (char {})", sig_c / 1e6, if harsh {60} else {60}, lim_c / 1e6, sc.combination_id),
                &format!("σ_c={:.1} MPa ≤ {:.0}% f_ck={:.1} MPa (char {})", sig_c / 1e6, if harsh {60} else {60}, lim_c / 1e6, sc.combination_id),
            ));
        scc = scc.explanation(l(
            &format!("Concrete stress σ_c={:.1} MPa ≤ 0.60 f_ck under characteristic SLS ({})", sig_c / 1e6, sc.combination_id),
            &format!("Betondruckspannung σ_c={:.1} MPa ≤ 0,60 f_ck unter charakteristischer GZG ({})", sig_c / 1e6, sc.combination_id),
        ));
        if sig_c > lim_c {
            let d_req = d * (sig_c / lim_c.max(1e-12)).max(1.0) * 1.05;
            scc = scc.remedy(Remedy::at_least(
                member_ref(doc, member, &member_path(&member.id, "effectiveDepth")),
                length_m(d),
                length_m(d_req),
                l("Increase effective depth to reduce concrete stress.", "Statische Nutzhöhe erhöhen, um die Betonspannung zu senken."),
            ));
        }
        out.push(scc.build());
    }
    if let Some(qp) = sls_qp {
        let sig_c = part_1_1::cracked_sigma_c_pa(qp.m_ed, b, d, a_s, e_s, concrete.e_cm());
        let lim = 0.45 * f_ck;
        let mut qp_c = CheckResult::assess(format!("en1992.7.2.creep.{}", member.id), part_11(), clause_11("§7.2"), member_ref(doc, member, &member_path(&member.id, "concreteGradeId")), l("Quasi-permanent concrete stress", "Quasi-ständige Betondruckspannung"))
            .utilization(stress_pa(sig_c), stress_pa(lim.max(1.0)))
            .annex(annex)
            .explanation(l(
                &format!("σ_c={:.1} MPa ≤ 0.45 f_ck={:.1} MPa for linear creep ({})", sig_c / 1e6, lim / 1e6, qp.combination_id),
                &format!("σ_c={:.1} MPa ≤ 0,45 f_ck={:.1} MPa für lineares Kriechen ({})", sig_c / 1e6, lim / 1e6, qp.combination_id),
            ));
        if sig_c > lim {
            let d_req = d * (sig_c / lim.max(1e-12)).max(1.0) * 1.05;
            qp_c = qp_c.remedy(Remedy::at_least(
                member_ref(doc, member, &member_path(&member.id, "effectiveDepth")),
                length_m(d),
                length_m(d_req),
                l("Increase depth to satisfy creep stress limit.", "Nutzhöhe erhöhen für Kriechspannungsgrenze."),
            ));
        }
        out.push(qp_c.build());
    }
}
