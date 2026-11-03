//! 🏛️ Composite column checks: EN 1994-1-1 §6.7 (C-25..C-30).

use super::*;

fn cref(col: &CompositeColumn, leaf: &str) -> SubjectRef {
    SubjectRef::new(col.id.clone(), format!("columns[id={}].{leaf}", col.id), lc(format!("Column {}", col.id), format!("Stütze {}", col.id)))
}

pub(crate) fn evaluate_column(report: &mut CheckReport, doc: &En1994Snapshot, col: &CompositeColumn) {
    let a = doc.annex;
    let uls = part_en1990::uls_column(&col.actions, a, doc.reliability_class);
    let n_ed = uls.n_n.abs();
    let m1 = uls.m_nm.abs().max(uls.m_hog_nm.abs());
    let filled = col.kind == semio_s_artifact_norm_en1994_model::ColumnKind::ConcreteFilled;
    let n_pl = part_column::n_pl_rd_n(col, a);
    let n_pl_rk = part_column::n_pl_rk_n(col);
    let delta = part_column::steel_contribution(col, a);
    let ecm = cat::ecm_pa(col.concrete_f_ck_pa);
    let lam0 = part_column::relative_slenderness(col, a);
    let ng: f64 = col.actions.iter().filter(|x| x.kind == semio_s_artifact_norm_en1994_model::ActionKind::Permanent).map(|x| part_en1990::gamma_g(a, doc.reliability_class) * x.n_k_n).sum::<f64>().abs();
    let threshold = if filled { 0.8 } else { 0.5 };
    let ecm_eff = if lam0 > threshold { cat::ecm_eff(ecm, ng, n_ed, cat::constants::CREEP_PHI) } else { ecm };
    let ncr = part_column::n_cr_n(col, a, ecm_eff);
    let lam = (n_pl_rk / ncr.max(1.0)).sqrt();
    let chi = cat::chi_column(lam, part_column::alpha_curve(col.buckling_curve.as_str()));
    let n_rd = chi * n_pl;
    let lb = part_column::local_buckling_util(col);
    let rho_s = if col.concrete_a_m2 > 0.0 { col.reinforcement_as_m2 / col.concrete_a_m2 } else { 0.0 };

    let violation = if !(0.2..=0.9).contains(&delta) {
        Some(format!("steel contribution δ = {delta:.2} outside 0.2..0.9"))
    } else if lam > 2.0 {
        Some(format!("relative slenderness {lam:.2} above 2.0"))
    } else if col.steel_f_y_pa > 460e6 {
        Some("f_y above 460 MPa".to_string())
    } else {
        None
    };
    let mut scope = head(a, format!("en1994.6.7.1.scope.{}", col.id), "1994-1-1", "6.7.1", cref(col, "kind"), "Simplified method: scope", "Vereinfachtes Verfahren: Anwendungsbereich")
        .utilization(dim((delta / 0.9).max(0.2 / delta.max(1e-9)).max(lam / 2.0)), dim(1.0))
        .explanation(lc(format!("δ = {delta:.2} ∈ [0.2, 0.9], λ̄ = {lam:.2} ≤ 2.0, ρ_s = {:.1} % (contribution capped at 6 %), local buckling {lb:.2}.", rho_s * 100.0), format!("δ = {delta:.2} ∈ [0,2; 0,9], λ̄ = {lam:.2} ≤ 2,0, ρ_s = {:.1} % (Anrechnung bis 6 %), lokales Beulen {lb:.2}.", rho_s * 100.0)));
    if let Some(v) = &violation {
        scope = scope.not_applicable(lc(format!("Out of scope of the simplified method: {v}."), format!("Außerhalb des vereinfachten Verfahrens: {v}.")));
    }
    report.push(scope.build());

    let mut nchk = head(a, format!("en1994.6.7.3.npl.{}", col.id), "1994-1-1", "6.7.3", cref(col, "actions"), "Composite column axial resistance", "Normalkrafttragfähigkeit Verbundstütze")
        .utilization(force(n_ed), force(n_rd))
        .explanation(lc(format!("{}: N_Ed = {:.0} kN, N_pl,Rd = {:.0} kN, N_cr = {:.0} kN, λ̄ = {lam:.3}, χ = {chi:.3}, N_Rd = {:.0} kN, local buckling {lb:.2}.", uls.label_en, n_ed / 1e3, n_pl / 1e3, ncr / 1e3, n_rd / 1e3), format!("{}: N_Ed = {:.0} kN, N_pl,Rd = {:.0} kN, N_cr = {:.0} kN, λ̄ = {lam:.3}, χ = {chi:.3}, N_Rd = {:.0} kN, lokales Beulen {lb:.2}.", uls.label_de, n_ed / 1e3, n_pl / 1e3, ncr / 1e3, n_rd / 1e3)));
    if n_ed > n_rd || lb > 1.0 {
        let ratio = lb.max(n_ed / n_rd.max(1.0));
        nchk = nchk.remedy(Remedy::at_least(cref(col, "wallThicknessM"), length(col.wall_thickness_m), length(col.wall_thickness_m * ratio * 1.05), lc("Increase the wall thickness.", "Wanddicke erhöhen.")));
        nchk = nchk.remedy(Remedy::at_least(cref(col, "concreteFCkPa"), stress(col.concrete_f_ck_pa), stress((col.concrete_f_ck_pa * ratio).min(60e6)), lc("Use a higher concrete class.", "Höhere Betonfestigkeitsklasse wählen.")));
    }
    report.push(nchk.build());

    let ei2 = 0.9 * cat::ei_eff(col.i_m4, part_column::i_rebar_m4(col, a), part_column::i_concrete_m4(col, a), ecm_eff, 0.5);
    let ncr_eff = std::f64::consts::PI.powi(2) * ei2 / col.length_m.max(0.1).powi(2);
    let k = cat::second_order_k(n_ed, ncr_eff, 1.0);
    let m_ed = k * m1;
    let util = part_column::interaction_utilization(col, a, n_ed, m_ed);
    let poly = part_column::interaction_polygon(col, a);
    let mut mn = head(a, format!("en1994.6.7.3.mn.{}", col.id), "1994-1-1", "6.7.3.6", cref(col, "kind"), "Composite column M–N interaction", "M–N-Interaktion Verbundstütze")
        .utilization(dim(util), dim(1.0))
        .explanation(lc(format!("{}: M_Ed = k·M₁ = {k:.2}·{:.1} = {:.1} kNm, α_M·μ_d·M_pl,Rd = {:.1} kNm; polygon A({:.0} kN, 0), C({:.0} kN, {:.1} kNm), D({:.0} kN, {:.1} kNm), B(0, {:.1} kNm).", uls.label_en, m1 / 1e3, m_ed / 1e3, part_column::moment_resistance_at_n(col, a, n_ed) / 1e3, poly[0].n_n / 1e3, poly[1].n_n / 1e3, poly[1].m_nm / 1e3, poly[2].n_n / 1e3, poly[2].m_nm / 1e3, poly[3].m_nm / 1e3), format!("{}: M_Ed = k·M₁ = {k:.2}·{:.1} = {:.1} kNm, α_M·μ_d·M_pl,Rd = {:.1} kNm; Polygon A({:.0} kN, 0), C({:.0} kN, {:.1} kNm), D({:.0} kN, {:.1} kNm), B(0, {:.1} kNm).", uls.label_de, m1 / 1e3, m_ed / 1e3, part_column::moment_resistance_at_n(col, a, n_ed) / 1e3, poly[0].n_n / 1e3, poly[1].n_n / 1e3, poly[1].m_nm / 1e3, poly[2].n_n / 1e3, poly[2].m_nm / 1e3, poly[3].m_nm / 1e3)));
    if util > 1.0 {
        if let Some(act) = col.actions.iter().find(|x| x.kind == semio_s_artifact_norm_en1994_model::ActionKind::Imposed).or_else(|| col.actions.iter().find(|x| x.n_k_n.abs() > 1.0)) {
            let f = (1.0 / util * 0.9).clamp(0.0, 1.0);
            mn = mn.remedy(Remedy::at_most(cref(col, &format!("actions[id={}].nKN", act.id)), force(act.n_k_n), force(act.n_k_n * f), lc("Reduce the axial and moment demand.", "Normalkraft und Moment reduzieren.")));
        }
        mn = mn.remedy(Remedy::at_least(cref(col, "wallThicknessM"), length(col.wall_thickness_m), length(col.wall_thickness_m * util * 1.1), lc("Increase the wall thickness.", "Wanddicke erhöhen.")));
    }
    report.push(mn.build());
}
