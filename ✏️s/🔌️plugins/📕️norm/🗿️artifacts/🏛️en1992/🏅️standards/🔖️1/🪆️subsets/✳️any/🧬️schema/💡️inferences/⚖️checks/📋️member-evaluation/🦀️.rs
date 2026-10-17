//! 📋️ EN 1992 member evaluation against EN 1992-1-1 and 1-2, with optional bridge and liquid-retaining checks.

use super::*;

/// 📋️ Evaluate one member against EN 1992-1-1/1-2 (+ optional bridge/liquid).
pub fn evaluate_member(doc: &En1992Snapshot, member: &RcMember) -> Vec<CheckResult> {
    let mut out = Vec::new();
    let annex = doc.annex;
    let Some(concrete) = doc.concrete(&member.concrete_grade_id) else {
        let options: Vec<String> = doc.concrete_grades.iter().map(|g| g.id.clone()).collect();
        let mut missing = CheckResult::assess(format!("en1992.materials.concrete.{}", member.id), part_11(), clause_11("§3.1"), member_ref(doc, member, &member_path(&member.id, "concreteGradeId")), l("Concrete grade", "Betonfestigkeitsklasse"))
            .annex(annex)
            .explanation(l(
                &format!("Concrete grade id '{}' is missing.", member.concrete_grade_id),
                &format!("Betonfestigkeitsklasse-Id '{}' fehlt.", member.concrete_grade_id),
            ))
            .status(CheckStatus::Fail);
        missing = missing.remedy(Remedy::one_of(
            member_ref(doc, member, &member_path(&member.id, "concreteGradeId")),
            if options.is_empty() { vec!["c30".into()] } else { options },
            l("Set concreteGradeId to a declared concrete grade.", "concreteGradeId auf eine deklarierte Betonfestigkeitsklasse setzen."),
        ));
        out.push(missing.build());
        return out;
    };
    let Some(reinf) = doc.reinforcement(&member.reinforcement_grade_id) else {
        let options: Vec<String> = doc.reinforcement_grades.iter().map(|g| g.id.clone()).collect();
        let mut missing = CheckResult::assess(format!("en1992.materials.steel.{}", member.id), part_11(), clause_11("§3.2"), member_ref(doc, member, &member_path(&member.id, "reinforcementGradeId")), l("Reinforcement grade", "Betonstahl"))
            .annex(annex)
            .explanation(l(
                &format!("Reinforcement grade id '{}' is missing.", member.reinforcement_grade_id),
                &format!("Betonstahl-Id '{}' fehlt.", member.reinforcement_grade_id),
            ))
            .status(CheckStatus::Fail);
        missing = missing.remedy(Remedy::one_of(
            member_ref(doc, member, &member_path(&member.id, "reinforcementGradeId")),
            if options.is_empty() { vec!["b500".into()] } else { options },
            l("Set reinforcementGradeId to a declared steel grade.", "reinforcementGradeId auf einen deklarierten Betonstahl setzen."),
        ));
        out.push(missing.build());
        return out;
    };
    let f_ck = concrete.f_ck;
    let f_yk = reinf.f_yk;
    let e_s = reinf.e_s;
    let eps_cu2 = concrete.eps_cu2();
    let n_parabola = concrete.n_parabola();
    let f_ck_cube = concrete.f_ck_cube();
    let b = member.width;
    let d = member.effective_depth;
    let h = member.height;
    let a_s = member.a_s_tension();
    let rho = member.rho_l();
    let params = na_de::AnnexParams::for_choice(annex);

    // --- Durability cover: c_min,b + c_min,dur (life/cement/class) + Δc_dev ---
    let phi0 = member.longitudinal.first().map(|l| l.diameter).unwrap_or(0.012);
    let agg0 = member.longitudinal.first().map(|l| l.aggregate_size).unwrap_or(0.016);
    let bundle0 = 1u32;
    let c_min_b = part_1_1::c_min_b_m(phi0, agg0, bundle0);
    let c_min_dur = part_1_1::c_min_dur_adjusted_m(member.exposure, doc.design_working_life_years, &doc.cement_type, f_ck, member.kind);
    let (c_min, c_nom) = part_1_1::c_nom_m(c_min_b, c_min_dur, doc.delta_c_dev);
    let cover_path = member_path(&member.id, "cover");
    let mut cover_check = CheckResult::assess(format!("en1992.4.4.cover.{}", member.id), part_11(), clause_11("§4.4"), member_ref(doc, member, &cover_path), l("Concrete cover for durability", "Betondeckung für Dauerhaftigkeit"))
        .minimum(length_m(member.cover), length_m(c_nom))
        .annex(annex)
        .explanation(l(
            &format!("c={:.0} mm ≥ c_nom={:.0} mm (c_min={:.0}=max(c_min,b={:.0}, c_min,dur={:.0}), life={:.0} a, cement={}, Δc_dev={:.0})", member.cover * 1000.0, c_nom * 1000.0, c_min * 1000.0, c_min_b * 1000.0, c_min_dur * 1000.0, doc.design_working_life_years, doc.cement_type, doc.delta_c_dev * 1000.0),
            &format!("c={:.0} mm ≥ c_nom={:.0} mm (c_min={:.0}=max(c_min,b={:.0}, c_min,dur={:.0}), Nutzungsdauer={:.0} a, Zement={}, Δc_dev={:.0})", member.cover * 1000.0, c_nom * 1000.0, c_min * 1000.0, c_min_b * 1000.0, c_min_dur * 1000.0, doc.design_working_life_years, doc.cement_type, doc.delta_c_dev * 1000.0),
        ));
    if member.cover < c_nom {
        cover_check = cover_check.remedy(Remedy::at_least(
            member_ref(doc, member, &cover_path),
            length_m(member.cover),
            length_m(c_nom),
            l(
                &format!("Increase cover of {} from {:.0} mm to at least {:.0} mm (c_nom).", member.id, member.cover * 1000.0, c_nom * 1000.0),
                &format!("Betondeckung von {} von {:.0} mm auf mindestens {:.0} mm (c_nom) erhöhen.", member.id, member.cover * 1000.0, c_nom * 1000.0),
            ),
        ));
    }
    out.push(cover_check.build());

    // §3.1 Table 3.1 — cylinder/cube + constitutive ε_cu2, n enter the utilization
    {
        let computed = f_ck * concrete.eps_c2() * n_parabola;
        let limit = f_ck_cube * eps_cu2 * 2.0;
        let mut mat = CheckResult::assess(format!("en1992.3.1.constitutive.{}", member.id), part_11(), clause_11("§3.1.7"), member_ref(doc, member, &format!("concreteGrades[id={}].fCk", member.concrete_grade_id)), l("Concrete Table 3.1 constitutive", "Beton Tabelle 3.1 Stoffgesetz"))
            .utilization(stress_pa(computed), stress_pa(limit.max(1.0)))
            .annex(annex)
            .explanation(l(
                &format!("Table 3.1: f_ck={:.0} MPa / f_ck,cube={:.0} MPa; ε_c2={:.2}‰ / ε_cu2={:.2}‰; n={:.2}", f_ck / 1e6, f_ck_cube / 1e6, concrete.eps_c2() * 1e3, eps_cu2 * 1e3, n_parabola),
                &format!("Tabelle 3.1: f_ck={:.0} MPa / f_ck,cube={:.0} MPa; ε_c2={:.2}‰ / ε_cu2={:.2}‰; n={:.2}", f_ck / 1e6, f_ck_cube / 1e6, concrete.eps_c2() * 1e3, eps_cu2 * 1e3, n_parabola),
            ));
        if computed > limit {
            mat = mat.remedy(Remedy::at_most(
                member_ref(doc, member, &format!("concreteGrades[id={}].fCk", member.concrete_grade_id)),
                stress_pa(f_ck),
                stress_pa(f_ck * limit / computed.max(1.0)),
                l("Reduce f_ck toward a Table 3.1 class so constitutive ratios stay consistent.", "f_ck auf eine Klasse nach Tabelle 3.1 absenken, damit die Stoffgesetz-Verhältnisse konsistent bleiben."),
            ));
        }
        out.push(mat.build());
    }

    let effects = combine_member_actions(member);
    let uls = governing(&effects, "uls");
    let acc = governing(&effects, "accidental");
    let sls_qp = governing(&effects, "sls_qp");
    let sls_char = governing(&effects, "sls_char");

    if let Some(uls) = uls {
        let params = na_de::AnnexParams::for_situation(annex, &uls.situation);
        // Flexure + axial
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

        // Shear V_Rd,c / V_Rd,s / V_Rd,max
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
        // If V_Ed > V_Rd,c need stirrups capacity
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
                // required spacing: V_Rd,s = (Asw/s)*z*fywd*cot >= V_Ed → Asw/s >= V_Ed/(z*fywd*cot)
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

        // Torsion
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

        // Punching — β, u0/u1 from column geometry; DE NA C_Rd,c=0.18/γ_c with u0/d reduction; asw
        if matches!(member.kind, MemberKind::FlatSlab) {
            if let Some(p) = &member.punching {
                let beta = part_1_1::punching_beta(&p.column_position);
                let (u0, u1) = part_1_1::punching_perimeters(p.column_width, p.column_depth, d);
                let u0_over_d = u0 / d.max(1e-6);
                let mut v_rd_c_pa = part_1_1::punching_v_rd_c_pa(f_ck, d, rho, annex);
                // DE NA: C_Rd,c = 0.18/γ_c and reduction when u0/d < 4
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

    // Crack width SLS
    if let Some(sls) = sls_qp {
        let w_max = part_1_1::w_max_m(member.exposure);
        // approx sigma_s from M / (z As)
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

    // Min reinforcement
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

    // Deflection l/d — Table 7.4N + DE-NA K·35 and K²·150/l when deflection-sensitive
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

    // Column slenderness — DE NA λ_lim + nominal curvature §5.8.8 when exceeded
    if matches!(member.kind, MemberKind::Column | MemberKind::Wall) && member.buckling_length > 0.0 {
        let lam = part_1_1::slenderness(member.buckling_length, b, h);
        let n_ed_col = uls.map(|u| u.n_ed).unwrap_or(0.0);
        let n_ratio = (n_ed_col.abs() / ((b * h) * params.f_cd_pa(f_ck)).max(1.0)).min(1.0);
        let lim = part_1_1::lambda_lim_de(n_ratio);
        let m2 = if lam > lim {
            part_1_1::second_order_moment_nm(n_ed_col, member.buckling_length, d, params.f_yd_pa(f_yk), e_s)
        } else {
            0.0
        };
        let m_ed1 = uls.map(|u| u.m_ed.abs()).unwrap_or(0.0);
        let m_ed_tot = m_ed1 + m2;
        let m_rd = part_1_1::flexural_resistance_nm(f_ck, b, d, a_s, f_yk, n_ed_col, annex, eps_cu2, n_parabola, e_s);
        let _ok_cap = m_ed_tot <= m_rd + 1.0;
        let mut sl = CheckResult::assess(format!("en1992.5.8.slender.{}", member.id), part_11(), clause_11("§5.8.3"), member_ref(doc, member, &member_path(&member.id, "bucklingLength")), l("Column slenderness", "Stützenschlankheit"))
            .utilization(moment_nm(m_ed_tot.abs()), moment_nm(m_rd.max(1.0)))
            .annex(annex)
            .explanation(l(
                &format!("Slenderness λ={:.1} (limit {:.1}, n={:.2}); second-order M_Ed+M₂={:.1} kNm ≤ M_Rd={:.1} kNm", lam, lim, n_ratio, m_ed_tot / 1e3, m_rd / 1e3),
                &format!("Schlankheit λ={:.1} (Grenze {:.1}, n={:.2}); Theorie II. Ordnung M_Ed+M₂={:.1} kNm ≤ M_Rd={:.1} kNm", lam, lim, n_ratio, m_ed_tot / 1e3, m_rd / 1e3),
            ));
        if lam > lim {
            let b_req = member.buckling_length / lim * 12.0f64.sqrt();
            sl = sl.remedy(Remedy::at_least(
                member_ref(doc, member, &member_path(&member.id, "width")),
                length_m(b),
                length_m(b_req),
                l(
                    &format!("Increase column width of {} to at least {:.0} mm to reduce slenderness.", member.id, b_req * 1000.0),
                    &format!("Stützenbreite von {} auf mindestens {:.0} mm erhöhen, um die Schlankheit zu verringern.", member.id, b_req * 1000.0),
                ),
            ));
            sl = sl.remedy(Remedy::at_most(
                member_ref(doc, member, &member_path(&member.id, "bucklingLength")),
                length_m(member.buckling_length),
                length_m(member.buckling_length * lim / lam),
                l(
                    &format!("Reduce buckling length of {} to ≤ {:.2} m.", member.id, member.buckling_length * lim / lam),
                    &format!("Knicklänge von {} auf ≤ {:.2} m verringern.", member.id, member.buckling_length * lim / lam),
                ),
            ));
        }
        out.push(sl.build());
    }

    // Prestress §5.10.2.1 / §5.10.3 stress limits using f_pk and f_p0,1k
    if let Some(ps) = &member.prestress {
        let steel = doc.prestress_steels.iter().find(|s| s.id == member.prestress_steel_id);
        if member.prestress_steel_id.is_empty() || steel.is_none() {
            let options: Vec<String> = doc.prestress_steels.iter().map(|s| s.id.clone()).collect();
            let mut dangling = CheckResult::assess(
                format!("en1992.integrity.prestressSteel.{}", member.id),
                part_11(),
                clause_11("§5.10"),
                member_ref(doc, member, &member_path(&member.id, "prestressSteelId")),
                l("Prestressing steel reference", "Spannstahl-Referenz"),
            )
            .annex(annex)
            .explanation(l(
                &format!("Member {} references prestressSteelId='{}' which is missing; choose an existing prestressing steel id.", member.id, member.prestress_steel_id),
                &format!("Bauteil {} verweist auf prestressSteelId='{}', das fehlt; eine vorhandene Spannstahl-Id wählen.", member.id, member.prestress_steel_id),
            ))
            .status(CheckStatus::Fail);
            dangling = dangling.remedy(Remedy::one_of(
                member_ref(doc, member, &member_path(&member.id, "prestressSteelId")),
                if options.is_empty() { vec!["yp1860".into()] } else { options },
                l(
                    "Set prestressSteelId to one of the declared prestressing steel ids.",
                    "prestressSteelId auf eine der deklarierten Spannstahl-Ids setzen.",
                ),
            ));
            out.push(dangling.build());
        } else {
        let steel = steel.unwrap();
        let p_m0 = ps.force;
        let p_minf = p_m0 * (1.0 - ps.loss_ratio.clamp(0.0, 0.5));
        let a_p = ps.area.max(1e-12);
        let sigma_pm0 = p_m0 / a_p;
        let sigma_pminf = p_minf / a_p;
        let f_pk = steel.f_pk;
        let f_p01k = steel.f_p0_1k;
        let sigma_p_max = (0.8 * f_pk).min(0.9 * f_p01k);
        let sigma_pm0_lim = (0.75 * f_pk).min(0.85 * f_p01k);
        let ac = (b * h).max(1e-12);
        let i = b * h.powi(3) / 12.0;
        let sigma_c = p_m0 / ac + p_m0 * ps.eccentricity.abs() * (h * 0.5) / i.max(1e-18);
        let lim_c = 0.6 * f_ck;
        let util_fpk = sigma_pm0 / (0.75 * f_pk).max(1.0);
        let util_fp01 = sigma_pm0 / (0.85 * f_p01k).max(1.0);
        let util_pmax = sigma_pm0 / sigma_p_max.max(1.0);
        let util_c = sigma_c / lim_c.max(1.0);
        let util_loss = ps.loss_ratio / 0.30;
        let util_inf = sigma_pminf / sigma_pm0_lim.max(1.0);
        let util = util_fpk.max(util_fp01).max(util_pmax).max(util_c).max(util_loss).max(util_inf);
        let mut pre = CheckResult::assess(format!("en1992.5.10.prestress.{}", member.id), part_11(), clause_11("§5.10.2"), member_ref(doc, member, &member_path(&member.id, "prestress")), l("Prestress stress limits", "Vorspannungs-Spannungsgrenzen"))
            .utilization(stress_pa(util * lim_c), stress_pa(lim_c.max(1.0)))
            .annex(annex)
            .explanation(l(
                &format!("Prestress: σ_pm0={:.0} MPa ≤ min(0.75 f_pk, 0.85 f_p0,1k)={:.0} MPa; σ_pm,∞={:.0} MPa; σ_p,max={:.0} MPa; σ_c={:.1} MPa ≤ 0.6 f_ck; P_m,∞={:.0} kN (losses {:.0}%, steel {})",
                    sigma_pm0 / 1e6, sigma_pm0_lim / 1e6, sigma_pminf / 1e6, sigma_p_max / 1e6, sigma_c / 1e6, p_minf / 1e3, ps.loss_ratio * 100.0, steel.id),
                &format!("Vorspannung: σ_pm0={:.0} MPa ≤ min(0,75 f_pk, 0,85 f_p0,1k)={:.0} MPa; σ_pm,∞={:.0} MPa; σ_p,max={:.0} MPa; σ_c={:.1} MPa ≤ 0,6 f_ck; P_m,∞={:.0} kN (Verluste {:.0} %, Stahl {})",
                    sigma_pm0 / 1e6, sigma_pm0_lim / 1e6, sigma_pminf / 1e6, sigma_p_max / 1e6, sigma_c / 1e6, p_minf / 1e3, ps.loss_ratio * 100.0, steel.id),
            ));
        if util > 1.0 {
            pre = pre.remedy(Remedy::at_most(
                member_ref(doc, member, &member_path(&member.id, "prestress.force")),
                force_n(p_m0),
                force_n(p_m0 / util),
                l("Reduce prestressing force to satisfy §5.10 stress limits.", "Vorspannkraft reduzieren, damit Spannungsgrenzen nach §5.10 eingehalten werden."),
            ));
        }
        let mut decomp = CheckResult::assess(format!("en1992.5.10.decompression.{}", member.id), part_11(), clause_11("§5.10.9"), member_ref(doc, member, &member_path(&member.id, "prestress")), l("Decompression SLS", "Dekompression GZG"))
            .utilization(stress_pa((-sigma_c).max(0.0)), stress_pa(0.45 * f_ck))
            .annex(annex)
            .explanation(l(
                &format!("Decompression SLS: concrete stress under prestress σ_c={:.2} MPa (compression positive); steel id={}", sigma_c / 1e6, member.prestress_steel_id),
                &format!("Dekompression GZG: Betonspannung unter Vorspannung σ_c={:.2} MPa (Druck positiv); Spannstahl-Id={}", sigma_c / 1e6, member.prestress_steel_id),
            ));
        if sigma_c < 0.0 {
            decomp = decomp.remedy(Remedy::at_least(
                member_ref(doc, member, &member_path(&member.id, "prestress.force")),
                force_n(p_m0),
                force_n(p_m0 * 1.1),
                l("Increase prestress to maintain decompression.", "Vorspannung erhöhen, um Dekompression sicherzustellen."),
            ));
        }
        out.push(pre.build());
        out.push(decomp.build());
        }
    } else if !member.prestress_steel_id.is_empty() {
        let options: Vec<String> = doc.prestress_steels.iter().map(|s| s.id.clone()).collect();
        let mut dangling = CheckResult::assess(
            format!("en1992.integrity.prestressSteel.orphan.{}", member.id),
            part_11(),
            clause_11("§5.10"),
            member_ref(doc, member, &member_path(&member.id, "prestressSteelId")),
            l("Orphan prestressing steel id", "Verwaiste Spannstahl-Id"),
        )
        .annex(annex)
        .explanation(l(
            &format!("Member {} sets prestressSteelId='{}' without a prestress specification.", member.id, member.prestress_steel_id),
            &format!("Bauteil {} setzt prestressSteelId='{}' ohne Vorspannungsangabe.", member.id, member.prestress_steel_id),
        ))
        .status(CheckStatus::Fail);
        dangling = dangling.remedy(Remedy::one_of(
            member_ref(doc, member, &member_path(&member.id, "prestressSteelId")),
            if options.is_empty() { vec![String::new()] } else { options },
            l("Clear prestressSteelId or add a prestress block linked to a declared steel.", "prestressSteelId leeren oder einen Vorspannungsblock mit deklariertem Stahl ergänzen."),
        ));
        out.push(dangling.build());
    }

    // Reinforcement ductility Annex C Table C.1 — k and ε_uk vs declared class
    {
        let (k_min, eps_min) = reinf.ductility.table_c1_minima();
        let ok_k = reinf.k >= k_min;
        let ok_e = reinf.eps_uk >= eps_min;
        let util_d = (k_min / reinf.k.max(1e-6)).max(eps_min / reinf.eps_uk.max(1e-9));
        let eps_ud = 0.9 * reinf.eps_uk;
        let mut duct = CheckResult::assess(format!("en1992.annexC.ductility.{}", member.id), part_11(), clause_11("§3.2.7"), member_ref(doc, member, &format!("reinforcementGrades[id={}].ductility", member.reinforcement_grade_id)), l("Reinforcement ductility class", "Duktilitätsklasse Betonstahl"))
            .utilization(Quantity::new(QuantityKind::Dimensionless, util_d), Quantity::new(QuantityKind::Dimensionless, 1.0))
            .annex(annex)
            .explanation(l(
                &format!("Ductility {:?}: k={:.2} ≥ {:.2}, ε_uk={:.1}% ≥ {:.1}%, ε_ud=0.9ε_uk={:.2}% (inclined top branch)", reinf.ductility, reinf.k, k_min, reinf.eps_uk * 100.0, eps_min * 100.0, eps_ud * 100.0),
                &format!("Duktilität {:?}: k={:.2} ≥ {:.2}, ε_uk={:.1} % ≥ {:.1} %, ε_ud=0,9ε_uk={:.2} % (geneigter oberer Ast)", reinf.ductility, reinf.k, k_min, reinf.eps_uk * 100.0, eps_min * 100.0, eps_ud * 100.0),
            ));
        if !(ok_k && ok_e) {
            duct = duct.remedy(Remedy::at_least(
                member_ref(doc, member, &format!("reinforcementGrades[id={}].k", member.reinforcement_grade_id)),
                Quantity::new(QuantityKind::Dimensionless, reinf.k),
                Quantity::new(QuantityKind::Dimensionless, k_min),
                l("Raise k / ε_uk to Annex C Table C.1 minima for the declared ductility class.", "k / ε_uk auf die Mindestwerte nach Anhang C Tabelle C.1 der angegebenen Duktilitätsklasse anheben."),
            ));
        }
        out.push(duct.build());
    }

// Fire — Tables 5.2a–5.11 by element type
    if let Some(fire) = &member.fire {
        let (b_min, a_req) = part_1_2_fire::required_for(member.kind, member.support, fire.rating, &fire.column_method, &fire.slab_system);
        let mut fa = CheckResult::assess(format!("en1992.1-2.a.{}", member.id), part_12(), ClauseId::new("EN 1992-1-2", "§5.6", "5.6"), member_ref(doc, member, &member_path(&member.id, "fire.axisDistance")), l("Fire axis distance", "Achsabstand Brandschutz"))
            .minimum(length_m(fire.axis_distance), length_m(a_req))
            .annex(annex)
            .explanation(l(
                &format!("Fire axis distance a={:.0} mm ≥ a_req={:.0} mm ({:?}; method {}; slab {})", fire.axis_distance * 1000.0, a_req * 1000.0, fire.rating, fire.column_method, fire.slab_system),
                &format!("Brandschutz-Achsabstand a={:.0} mm ≥ a_erf={:.0} mm ({:?}; Verfahren {}; Platte {})", fire.axis_distance * 1000.0, a_req * 1000.0, fire.rating, fire.column_method, fire.slab_system),
            ));
        if fire.axis_distance < a_req {
            fa = fa.remedy(Remedy::at_least(
                member_ref(doc, member, &member_path(&member.id, "fire.axisDistance")),
                length_m(fire.axis_distance),
                length_m(a_req),
                l(
                    &format!("Increase fire axis distance of {} from {:.0} mm to at least {:.0} mm.", member.id, fire.axis_distance * 1000.0, a_req * 1000.0),
                    &format!("Achsabstand Brandschutz von {} von {:.0} mm auf mindestens {:.0} mm erhöhen.", member.id, fire.axis_distance * 1000.0, a_req * 1000.0),
                ),
            ));
        }
        out.push(fa.build());
        let mut fb = CheckResult::assess(format!("en1992.1-2.bmin.{}", member.id), part_12(), ClauseId::new("EN 1992-1-2", "§5.6", "5.6"), member_ref(doc, member, &member_path(&member.id, "width")), l("Fire minimum width", "Mindestbreite Brandschutz"))
            .minimum(length_m(b), length_m(b_min))
            .annex(annex)
            .explanation(l(
                &format!("Fire min. dimension b/h={:.0} mm ≥ {:.0} mm", b * 1000.0, b_min * 1000.0),
                &format!("Brandschutz Mindestdickemaß b/h={:.0} mm ≥ {:.0} mm", b * 1000.0, b_min * 1000.0),
            ));
        if b < b_min {
            fb = fb.remedy(Remedy::at_least(
                member_ref(doc, member, &member_path(&member.id, "width")),
                length_m(b),
                length_m(b_min),
                l(
                    &format!("Increase width of {} from {:.0} mm to at least {:.0} mm for fire.", member.id, b * 1000.0, b_min * 1000.0),
                    &format!("Breite von {} von {:.0} mm auf mindestens {:.0} mm für Brandschutz erhöhen.", member.id, b * 1000.0, b_min * 1000.0),
                ),
            ));
        }
        out.push(fb.build());
    }

    // SLS stress limits §7.2 DE NA (k1–k4 style: 0.6 f_ck char in XD/XF/XS; 0.45 f_ck qp; 0.8 f_yk)
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

    // Anchorage §8.4 and laps §8.7
    for layer in &member.longitudinal {
        let eta1 = if layer.bond_condition == "poor" { 0.7 } else { 1.0 };
        let eta2 = if layer.diameter <= 0.032 { 1.0 } else { ((132.0 - layer.diameter * 1000.0) / 100.0).clamp(0.7, 1.0) };
        let eta_agg = if layer.aggregate_size > 0.016 { 0.95 } else { 1.0 };
        let f_bd = part_1_1::f_bd_pa(concrete.f_ctk_005(), eta1, eta2) * eta_agg;
        let sigma_sd = params.f_yd_pa(f_yk);
        let lb_rqd = part_1_1::l_b_rqd_m(layer.diameter, sigma_sd, f_bd);
        let lbd = part_1_1::l_bd_m(lb_rqd, layer.diameter, 1.0);
        let mut anc = CheckResult::assess(format!("en1992.8.4.anchorage.{}.{}", member.id, layer.id), part_11(), clause_11("§8.4"), member_ref(doc, member, &layer_path(&member.id, &layer.id, "anchorageLength")), l("Anchorage length", "Verankerungslänge"))
            .minimum(length_m(layer.anchorage_length), length_m(lbd))
            .annex(annex)
            .explanation(l(
                &format!("l_bd,prov={:.0} mm ≥ l_bd={:.0} mm (f_bd={:.2} MPa, η1={:.2})", layer.anchorage_length * 1000.0, lbd * 1000.0, f_bd / 1e6, eta1),
                &format!("l_bd,vorh={:.0} mm ≥ l_bd={:.0} mm (f_bd={:.2} MPa, η1={:.2})", layer.anchorage_length * 1000.0, lbd * 1000.0, f_bd / 1e6, eta1),
            ));
        if layer.anchorage_length < lbd {
            anc = anc.remedy(Remedy::at_least(
                member_ref(doc, member, &layer_path(&member.id, &layer.id, "anchorageLength")),
                length_m(layer.anchorage_length),
                length_m(lbd),
                l(
                    &format!("Increase anchorage of {}/{} to ≥ {:.0} mm.", member.id, layer.id, lbd * 1000.0),
                    &format!("Verankerung von {}/{} auf ≥ {:.0} mm erhöhen.", member.id, layer.id, lbd * 1000.0),
                ),
            ));
            anc = anc.remedy(Remedy::at_most(
                member_ref(doc, member, &layer_path(&member.id, &layer.id, "diameter")),
                length_m(layer.diameter),
                length_m((layer.diameter - 0.002).max(0.008)),
                l("Reduce bar diameter to shorten required anchorage.", "Stabdurchmesser verringern, um die erforderliche Verankerung zu verkürzen."),
            ));
        }
        out.push(anc.build());
        let alpha6 = 1.5;
        let l0 = part_1_1::l_0_m(lbd, layer.diameter, alpha6);
        let mut lap = CheckResult::assess(format!("en1992.8.7.lap.{}.{}", member.id, layer.id), part_11(), clause_11("§8.7"), member_ref(doc, member, &layer_path(&member.id, &layer.id, "lapLength")), l("Lap length", "Übergreifungslänge"))
            .minimum(length_m(layer.lap_length), length_m(l0))
            .annex(annex)
            .explanation(l(
                &format!("l_0,prov={:.0} mm ≥ l_0={:.0} mm (α6={:.1})", layer.lap_length * 1000.0, l0 * 1000.0, alpha6),
                &format!("l_0,vorh={:.0} mm ≥ l_0={:.0} mm (α6={:.1})", layer.lap_length * 1000.0, l0 * 1000.0, alpha6),
            ));
        if layer.lap_length < l0 {
            lap = lap.remedy(Remedy::at_least(
                member_ref(doc, member, &layer_path(&member.id, &layer.id, "lapLength")),
                length_m(layer.lap_length),
                length_m(l0),
                l(
                    &format!("Increase lap of {}/{} to ≥ {:.0} mm.", member.id, layer.id, l0 * 1000.0),
                    &format!("Übergreifung von {}/{} auf ≥ {:.0} mm erhöhen.", member.id, layer.id, l0 * 1000.0),
                ),
            ));
        }
        out.push(lap.build());
    }

    // Bridge
    if matches!(member.kind, MemberKind::Bridge) || member.bridge_sigma_c > 0.0 || member.bridge_delta_sigma_s > 0.0 {
        if member.bridge_sigma_c > 0.0 {
            let lim = part_2::concrete_stress_limit_pa(f_ck);
            let mut br = CheckResult::assess(format!("en1992-2.7.2.{}", member.id), part_2(), ClauseId::new("EN 1992-2", "§7.2", "7.2"), member_ref(doc, member, &member_path(&member.id, "bridgeSigmaC")), l("Bridge concrete stress (frequent)", "Brücken Betonspannung (häufig)"))
                .utilization(stress_pa(member.bridge_sigma_c), stress_pa(lim))
                .annex(annex)
                .explanation(l("Bridge frequent concrete stress σ_c ≤ 0.6 f_ck", "Brücke häufige Betondruckspannung σ_c ≤ 0,6 f_ck"));
            if member.bridge_sigma_c > lim {
                br = br.remedy(Remedy::at_most(
                    member_ref(doc, member, &member_path(&member.id, "bridgeSigmaC")),
                    stress_pa(member.bridge_sigma_c),
                    stress_pa(lim),
                    l("Reduce frequent concrete stress to ≤ 0.6 f_ck.", "Häufige Betonspannung auf ≤ 0,6 f_ck reduzieren."),
                ));
            }
            out.push(br.build());
        }
        if member.bridge_delta_sigma_s > 0.0 {
            let lim = part_2::fatigue_limit_pa();
            let mut bf = CheckResult::assess(format!("en1992-2.6.8.{}", member.id), part_2(), ClauseId::new("EN 1992-2", "§6.8.4", "6.8.4"), member_ref(doc, member, &member_path(&member.id, "bridgeDeltaSigmaS")), l("Bridge reinforcement fatigue", "Brücken Ermüdung Betonstahl"))
                .utilization(stress_pa(member.bridge_delta_sigma_s), stress_pa(lim))
                .annex(annex)
                .explanation(l("Bridge reinforcement fatigue Δσ_s ≤ Δσ_Rsk/γ_S,fat", "Brücke Betonstahl-Ermüdung Δσ_s ≤ Δσ_Rsk/γ_S,fat"));
            if member.bridge_delta_sigma_s > lim {
                bf = bf.remedy(Remedy::at_most(
                    member_ref(doc, member, &member_path(&member.id, "bridgeDeltaSigmaS")),
                    stress_pa(member.bridge_delta_sigma_s),
                    stress_pa(lim),
                    l("Reduce fatigue stress range.", "Ermüdungsspannungsschwingbreite reduzieren."),
                ));
            }
            out.push(bf.build());
        }
    }

    // Liquid retaining
    if matches!(member.kind, MemberKind::LiquidRetaining) || member.tightness.is_some() {
        if let Some(tc) = member.tightness {
            match part_3_liquid::tightness_crack_limit_m(tc, member.hd_over_h) {
                None => out.push(
                    CheckResult::assess(format!("en1992-3.crack.{}", member.id), part_3(), ClauseId::new("EN 1992-3", "§7.3", "7.3"), member_ref(doc, member, &member_path(&member.id, "tightness")), l("Liquid-retaining crack width", "Rissbreite Flüssigkeitsbehälter"))
                        .not_applicable(l("TC0: no crack-width requirement", "TC0: keine Rissbreitenanforderung"))
                        .annex(annex)
                        .build(),
                ),
                Some(lim) => {
                    let w_k = part_1_1::crack_width_m(member.liquid_sigma_s, member.liquid_rho_p_eff.max(1e-4), member.liquid_f_ct_eff, e_s, member.liquid_s_r_max);
                    let mut liq = CheckResult::assess(format!("en1992-3.crack.{}", member.id), part_3(), ClauseId::new("EN 1992-3", "§7.3", "7.3"), member_ref(doc, member, &member_path(&member.id, "liquidSigmaS")), l("Liquid-retaining crack width", "Rissbreite Flüssigkeitsbehälter"))
                        .utilization(length_m(w_k), length_m(lim))
                        .annex(annex)
                        .explanation(l(&format!("w_k={:.3} mm, limit={:.3} mm", w_k * 1000.0, lim * 1000.0), &format!("w_k={:.3} mm, Grenzwert={:.3} mm", w_k * 1000.0, lim * 1000.0)));
                    if w_k > lim {
                        let sig_req = member.liquid_sigma_s * lim / w_k;
                        liq = liq.remedy(Remedy::at_most(
                            member_ref(doc, member, &member_path(&member.id, "liquidSigmaS")),
                            stress_pa(member.liquid_sigma_s),
                            stress_pa(sig_req),
                            l("Reduce steel stress under quasi-permanent combination.", "Stahlspannung unter quasi-ständiger Einwirkungskombination reduzieren."),
                        ));
                    }
                    out.push(liq.build());
                }
            }
        }
    }

    out
}
