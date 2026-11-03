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
    sec: &Sec,
    geo: &Geo,
    params: &na_de::AnnexParams,
    uls: Option<&DesignEffects>,
) {
    let (b, h, a_s) = (member.width, geo.h, geo.a_s);
    if matches!(member.kind, MemberKind::Column | MemberKind::Wall) && member.buckling_length > 0.0 {
        let lam = part_1_1::slenderness(member.buckling_length, geo.radius());
        let n_ed_col = uls.map(|u| u.n_ed).unwrap_or(0.0);
        let fcd = params.f_cd_pa(f_ck);
        let fyd = params.f_yd_pa(f_yk);
        let n_ratio = (n_ed_col.abs() / (geo.area * fcd).max(1.0)).min(1.0);
        let omega = a_s * fyd / (geo.area * fcd).max(1e-9);
        let lim = part_1_1::lambda_limit(n_ratio, omega, annex);
        let is = (h / 2.0 - member.cover).max(0.0);
        let m2 = if lam > lim {
            let (e2, ..) = nominal_curvature(f_ck / 1e6, fcd / 1e6, fyd / 1e6, e_s / 1e6, geo.area / h.max(1e-9) * 1e3, h * 1e3, a_s * 1e6, is * 1e3, n_ed_col.abs(), member.buckling_length * 1e3, 0.0, std::f64::consts::PI * std::f64::consts::PI);
            n_ed_col.abs() * e2 / 1e3
        } else {
            0.0
        };
        let m_ed1 = uls.map(|u| u.m_ed.abs()).unwrap_or(0.0).max(n_ed_col.abs() * (h / 30.0).max(0.02));
        let m_ed_tot = m_ed1 + m2 + n_ed_col.abs() * (member.buckling_length / 400.0);
        let m_rd = member_m_rd_nm(sec, n_ed_col);
        let sl = CheckResult::assess(format!("en1992.5.8.slender.{}", member.id), part_11(), clause_11("§5.8.3"), member_ref(doc, member, &member_path(&member.id, "bucklingLength")), l("Column slenderness", "Stützenschlankheit")).remedies_attached_by("attach_minimal_remedies")
            .utilization(moment_nm(m_ed_tot.abs()), moment_nm(m_rd.max(1.0)))
            .annex(annex)
            .explanation(l(
                &format!("Slenderness λ={:.1} (limit {:.1}, n={:.2}); second-order M_Ed+M₂={:.1} kNm ≤ M_Rd={:.1} kNm", lam, lim, n_ratio, m_ed_tot / 1e3, m_rd / 1e3),
                &format!("Schlankheit λ={:.1} (Grenze {:.1}, n={:.2}); Theorie II. Ordnung M_Ed+M₂={:.1} kNm ≤ M_Rd={:.1} kNm", lam, lim, n_ratio, m_ed_tot / 1e3, m_rd / 1e3),
            ));
        out.push(ndps(sl, &["ndp.en1992.a-09.1"]).build());
    }
}

/// 🗜️ Decompression of one fibre, compression positive: the compression the prestress alone gives (`prestress`) against the tension the quasi-permanent load asks (`load`), as `(demand, capacity)` with `demand / capacity <= 1` exactly when the net stress `prestress - load` stays compressive. A fibre the prestress alone puts in tension (`prestress <= 0`) is measured against the mean prestress `reference`, so its ratio exceeds 1 until the load relieves it. See EN 1992-1-1 §7.3.1 and [`check_prestress`].
fn decompression_side(prestress: f64, load: f64, reference: f64) -> (f64, f64) {
    if prestress > 0.0 {
        (load.max(0.0), prestress)
    } else {
        let reference = reference.max(1.0);
        ((reference + load - prestress).max(0.0), reference)
    }
}

/// 🎻 Checks the prestress steel and the stress limits.
pub(super) fn check_prestress(doc: &En1992Snapshot, member: &RcMember, out: &mut Vec<CheckResult>, annex: AnnexChoice, f_ck: f64, geo: &Geo, m_qp: f64) {
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
        let ecc = member.tendon_eccentricity_mid().unwrap_or(ps.eccentricity);
        let p_m0 = ps.force;
        let p_minf = p_m0 * (1.0 - ps.loss_ratio.clamp(0.0, 1.0));
        let a_p = ps.area.max(1e-12);
        let sigma_pm0 = p_m0 / a_p;
        let sigma_pminf = p_minf / a_p;
        let f_pk = steel.f_pk;
        let f_p01k = steel.f_p0_1k;
        let sigma_p_max = (0.8 * f_pk).min(0.9 * f_p01k);
        let sigma_pm0_lim = (0.75 * f_pk).min(0.85 * f_p01k);
        let ac = geo.area.max(1e-12);
        let i = geo.i_y;
        let (z_bot, z_top) = (geo.zc_bot.max(1e-9), geo.zc_top.max(1e-9));
        let sigma_c = p_m0 / ac + p_m0 * ecc.abs() * (if ecc >= 0.0 { z_bot } else { z_top }) / i.max(1e-18);
        let lim_c = 0.6 * f_ck;
        let util_fpk = sigma_pm0 / (0.75 * f_pk).max(1.0);
        let util_fp01 = sigma_pm0 / (0.85 * f_p01k).max(1.0);
        let util_pmax = sigma_pm0 / sigma_p_max.max(1.0);
        let util_c = sigma_c / lim_c.max(1.0);
        let util_inf = sigma_pminf / sigma_pm0_lim.max(1.0);
        let util = util_fpk.max(util_fp01).max(util_pmax).max(util_c).max(util_inf);
        let pre = CheckResult::assess(format!("en1992.5.10.prestress.{}", member.id), part_11(), clause_11("§5.10.2"), member_ref(doc, member, &member_path(&member.id, "prestress")), l("Prestress stress limits", "Vorspannungs-Spannungsgrenzen")).remedies_attached_by("attach_minimal_remedies")
            .utilization(stress_pa(util * lim_c), stress_pa(lim_c.max(1.0)))
            .annex(annex)
            .explanation(l(
                &format!("Prestress: σ_pm0={:.0} MPa ≤ min(0.75 f_pk, 0.85 f_p0,1k)={:.0} MPa; σ_pm,∞={:.0} MPa; σ_p,max={:.0} MPa; σ_c={:.1} MPa ≤ 0.6 f_ck; P_m,∞={:.0} kN (losses {:.0}%, steel {})",
                    sigma_pm0 / 1e6, sigma_pm0_lim / 1e6, sigma_pminf / 1e6, sigma_p_max / 1e6, sigma_c / 1e6, p_minf / 1e3, ps.loss_ratio * 100.0, steel.id),
                &format!("Vorspannung: σ_pm0={:.0} MPa ≤ min(0,75 f_pk, 0,85 f_p0,1k)={:.0} MPa; σ_pm,∞={:.0} MPa; σ_p,max={:.0} MPa; σ_c={:.1} MPa ≤ 0,6 f_ck; P_m,∞={:.0} kN (Verluste {:.0} %, Stahl {})",
                    sigma_pm0 / 1e6, sigma_pm0_lim / 1e6, sigma_pminf / 1e6, sigma_p_max / 1e6, sigma_c / 1e6, p_minf / 1e3, ps.loss_ratio * 100.0, steel.id),
            ));
        let (w_bot, w_top) = (i.max(1e-18) / z_bot, i.max(1e-18) / z_top);
        let mean = p_minf / ac;
        let (demand, capacity) = [(mean + p_minf * ecc / w_bot, m_qp / w_bot), (mean - p_minf * ecc / w_top, -m_qp / w_top)]
            .map(|(prestress, load)| decompression_side(prestress, load, mean))
            .into_iter()
            .max_by(|a, b| (a.0 / a.1).total_cmp(&(b.0 / b.1)))
            .unwrap_or((0.0, 1.0));
        let sigma_bot = mean + p_minf * ecc / w_bot - m_qp / w_bot;
        let sigma_top = mean - p_minf * ecc / w_top + m_qp / w_top;
        let decomp = CheckResult::assess(format!("en1992.5.10.decompression.{}", member.id), part_11(), clause_11("§7.3.1"), member_ref(doc, member, &member_path(&member.id, "prestress")), l("Decompression SLS", "Dekompression GZG")).remedies_attached_by("attach_minimal_remedies")
            .utilization(stress_pa(demand), stress_pa(capacity))
            .annex(annex)
            .explanation(l(
                &format!("Decompression under the quasi-permanent combination: σ_bottom={:.2} MPa, σ_top={:.2} MPa (compression positive, P_m,∞={:.0} kN, e={:.0} mm, M_qp={:.1} kNm).", sigma_bot / 1e6, sigma_top / 1e6, p_minf / 1e3, ecc * 1e3, m_qp / 1e3),
                &format!("Dekompression unter quasi-ständiger Kombination: σ_unten={:.2} MPa, σ_oben={:.2} MPa (Druck positiv, P_m,∞={:.0} kN, e={:.0} mm, M_qp={:.1} kNm).", sigma_bot / 1e6, sigma_top / 1e6, p_minf / 1e3, ecc * 1e3, m_qp / 1e3),
            ));
        out.push(ndps(pre, &["ndp.en1992.pr-01.1", "ndp.en1992.pr-05.1", "ndp.en1992.sl-02.1"]).build());
        out.push(ndps(decomp, &["ndp.en1992.pr-05.1", "ndp.en1992.sl-02.1"]).build());
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
        out.push(ndps(duct, &[]).build());
    }
}
