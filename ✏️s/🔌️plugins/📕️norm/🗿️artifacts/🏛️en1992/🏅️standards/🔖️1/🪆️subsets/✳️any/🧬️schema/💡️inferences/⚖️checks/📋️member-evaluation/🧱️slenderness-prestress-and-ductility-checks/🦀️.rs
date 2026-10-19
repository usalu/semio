//! 🧱️ Slenderness, prestress and ductility checks: the column slenderness, the prestress limits and the reinforcement ductility of the member evaluation. See [`super`].

use super::*;

/// 🏛 Checks the column slenderness and the nominal curvature.
pub(super) fn check_column_slenderness(
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
    params: &na_de::AnnexParams,
    uls: Option<&DesignEffects>,
) {
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
}

/// 🎻 Checks the prestress steel and the stress limits.
pub(super) fn check_prestress(doc: &En1992Snapshot, member: &RcMember, out: &mut Vec<CheckResult>, annex: AnnexChoice, f_ck: f64, b: f64, h: f64) {
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
}

/// 🧵 Checks the reinforcement ductility class against Annex C.
pub(super) fn check_ductility(doc: &En1992Snapshot, member: &RcMember, out: &mut Vec<CheckResult>, annex: AnnexChoice, reinf: &semio_s_artifact_norm_en1992_model::ReinforcementGrade) {
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
}
