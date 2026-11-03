//! ⚓️ EN 1992 anchor evaluation: EN 1990 combination of anchor actions and the EN 1992-4 resistances.

use super::*;

/// ⚖️ Combine characteristic anchor actions (N_k, V_k and allied extras) per EN 1990 into design N_Ed, V_Ed.
pub fn combine_anchor_actions(anchor: &Anchor, factors: &semio_s_plugin_norm_compliance::ActionFactors) -> Vec<DesignEffects> {
    let mut g = (0.0_f64, 0.0_f64);
    let mut variables: Vec<(&LoadCaseActions, (f64, f64))> = Vec::new();
    let mut accidentals = Vec::new();
    for a in &anchor.actions {
        let mut n = a.n_k;
        let mut v = a.v_k;
        match a.source {
            semio_s_artifact_norm_en1992_model::ActionSource::Udl => {
                n += a.g_k_line.abs() + a.q_k_line.abs();
            }
            semio_s_artifact_norm_en1992_model::ActionSource::Point => {
                n += a.point_force.abs();
            }
            semio_s_artifact_norm_en1992_model::ActionSource::External => {}
        }
        if anchor.h_ef > 0.0 {
            n += a.m_k.abs() / anchor.h_ef;
        }
        if anchor.c1 > 0.0 {
            v += a.t_k.abs() / anchor.c1;
        }
        v += a.v_k_punch.abs();
        let eff = (n, v);
        match a.kind {
            semio_s_artifact_norm_en1992_model::ActionKind::Permanent | semio_s_artifact_norm_en1992_model::ActionKind::Prestress => { g.0 += eff.0; g.1 += eff.1; }
            semio_s_artifact_norm_en1992_model::ActionKind::Accidental => accidentals.push((a, eff)),
            semio_s_artifact_norm_en1992_model::ActionKind::Imposed | semio_s_artifact_norm_en1992_model::ActionKind::Snow | semio_s_artifact_norm_en1992_model::ActionKind::Wind => variables.push((a, eff)),
        }
    }
    let (gamma_g, gamma_q) = (factors.gamma_g_sup(), factors.gamma_q_building());
    let mut out = Vec::new();
    if variables.is_empty() {
        out.push(DesignEffects {
            m_ed: 0.0, n_ed: gamma_g * g.0, v_ed: gamma_g * g.1, t_ed: 0.0, v_ed_punch: 0.0,
            combination_id: "ULS-6.10-G".into(), situation: "uls".into(),
        });
    } else {
        for (lead_i, (lead_a, lead_e)) in variables.iter().enumerate() {
            let mut n = gamma_g * g.0 + gamma_q * lead_e.0;
            let mut v = gamma_g * g.1 + gamma_q * lead_e.1;
            for (j, (oa, oe)) in variables.iter().enumerate() {
                if j == lead_i { continue; }
                let (psi0, _, _) = psi_factors(factors, oa.category);
                n += gamma_q * psi0 * oe.0;
                v += gamma_q * psi0 * oe.1;
            }
            out.push(DesignEffects {
                m_ed: 0.0, n_ed: n, v_ed: v, t_ed: 0.0, v_ed_punch: 0.0,
                combination_id: format!("ULS-6.10b-{}", lead_a.id), situation: "uls".into(),
            });
            let (_, _, psi2) = psi_factors(factors, lead_a.category);
            let mut n_qp = g.0 + psi2 * lead_e.0;
            let mut v_qp = g.1 + psi2 * lead_e.1;
            for (j, (oa, oe)) in variables.iter().enumerate() {
                if j == lead_i { continue; }
                let (_, _, p2) = psi_factors(factors, oa.category);
                n_qp += p2 * oe.0;
                v_qp += p2 * oe.1;
            }
            out.push(DesignEffects {
                m_ed: 0.0, n_ed: n_qp, v_ed: v_qp, t_ed: 0.0, v_ed_punch: 0.0,
                combination_id: format!("SLS-qp-{}", lead_a.id), situation: "sls_qp".into(),
            });
        }
    }
    for (acc_a, acc_e) in &accidentals {
        let mut n = 1.0 * g.0 + acc_e.0;
        let mut v = 1.0 * g.1 + acc_e.1;
        for (oa, oe) in &variables {
            let (_, psi1, _) = psi_factors(factors, oa.category);
            n += psi1 * oe.0;
            v += psi1 * oe.1;
        }
        out.push(DesignEffects {
            m_ed: 0.0, n_ed: n, v_ed: v, t_ed: 0.0, v_ed_punch: 0.0,
            combination_id: format!("ACC-6.11-{}", acc_a.id), situation: "accidental".into(),
        });
    }
    out
}

#[path = "🩹️minimal-remedies/🦀️.rs"]
mod minimal_remedies;

/// ⚓️ Evaluate anchors with EN 1990-combined characteristic actions + EN 1992-4 resistances; every failing check carries the minimal one-click remedies of [`minimal_remedies`].
pub fn evaluate_anchor(doc: &En1992Snapshot, anchor: &Anchor) -> Vec<CheckResult> {
    let mut results = evaluate_anchor_unsolved(doc, anchor);
    minimal_remedies::attach_minimal_remedies(doc, anchor, &mut results);
    results
}

/// ⚓️ The anchor checks without the solved remedies; the remedy search re-evaluates through this.
pub(super) fn evaluate_anchor_unsolved(doc: &En1992Snapshot, anchor: &Anchor) -> Vec<CheckResult> {
    let annex = doc.annex;
    let label = l(
        &format!("{} — Anchor {}", doc.title, anchor.id),
        &format!("{} — Dübel {}", doc.title, anchor.id),
    );
    let mut out = Vec::new();
    let effects = combine_anchor_actions(anchor, &doc.action_factors());
    let uls = governing(&effects, "uls").cloned().or_else(|| effects.first().cloned());
    let Some(uls) = uls else {
        out.push(CheckResult::assess(format!("en1992-4.na.{}", anchor.id), part_4(), ClauseId::new("EN 1992-4", "§7", "7"), SubjectRef::new(&anchor.id, format!("anchors[id={}]", anchor.id), label), l("Anchor actions", "Dübeleinwirkungen"))
            .not_applicable(l("No characteristic actions declared for anchor.", "Keine charakteristischen Einwirkungen für den Dübel angegeben."))
            .build());
        return out;
    };
    let n_ed = uls.n_ed.max(0.0);
    let v_ed = uls.v_ed.max(0.0);

    let n_rd_s = part_4::steel_resistance_n(anchor.a_s, anchor.f_uk, anchor.f_yk);
    let s = CheckResult::assess(format!("en1992-4.steel.{}", anchor.id), part_4(), ClauseId::new("EN 1992-4", "§7.2.1.4", "7.2.1.4"), SubjectRef::new(&anchor.id, format!("anchors[id={}].aS", anchor.id), label.clone()), l("Anchor steel tension", "Dübel Stahlzugtragfähigkeit")).remedies_attached_by("attach_minimal_remedies")
        .utilization(force_n(n_ed), force_n(n_rd_s.max(1.0)))
        .annex(annex)
        .explanation(l(
            &format!("Anchor steel: N_Ed={:.1} kN ≤ N_Rd,s={:.1} kN ({})", n_ed / 1e3, n_rd_s / 1e3, uls.combination_id),
            &format!("Dübel Stahlzug: N_Ed={:.1} kN ≤ N_Rd,s={:.1} kN ({})", n_ed / 1e3, n_rd_s / 1e3, uls.combination_id),
        ));
    out.push(ndps(s, &[]).build());

    if let Some(sls) = governing(&effects, "sls_qp") {
        let n_sls = sls.n_ed.max(0.0);
        let n_rd_sls = anchor.a_s * anchor.f_yk / ndp(annex, "ndp.en1992.gamma-s");
        let ss = CheckResult::assess(format!("en1992-4.steel.sls.{}", anchor.id), part_4(), ClauseId::new("EN 1992-4", "§7.2.1.4", "7.2.1.4"), SubjectRef::new(&anchor.id, format!("anchors[id={}].actions", anchor.id), label.clone()), l("Anchor steel SLS (ψ₂)", "Dübel Stahlzug GZG (ψ₂)")).remedies_attached_by("attach_minimal_remedies")
            .utilization(force_n(n_sls), force_n(n_rd_sls.max(1.0)))
            .annex(annex)
            .explanation(l(
                &format!("Anchor SLS: N_Ed,qp={:.1} kN ≤ N_Rd,s={:.1} kN ({})", n_sls / 1e3, n_rd_sls / 1e3, sls.combination_id),
                &format!("Dübel GZG: N_Ed,qp={:.1} kN ≤ N_Rd,s={:.1} kN ({})", n_sls / 1e3, n_rd_sls / 1e3, sls.combination_id),
            ));
        out.push(ndps(ss, &[]).build());
    }

    let n_rd_c = part_4::cone_resistance_n(annex, anchor.f_ck, anchor.h_ef, anchor.cracked, anchor.c1);
    let c = CheckResult::assess(format!("en1992-4.cone.{}", anchor.id), part_4(), ClauseId::new("EN 1992-4", "§7.2.1.5", "7.2.1.5"), SubjectRef::new(&anchor.id, format!("anchors[id={}].hEf", anchor.id), label.clone()), l("Concrete cone", "Betonausbruchkegel")).remedies_attached_by("attach_minimal_remedies")
        .utilization(force_n(n_ed), force_n(n_rd_c.max(1.0)))
        .annex(annex)
        .explanation(l(
            &format!("Concrete cone: N_Ed={:.1} kN ≤ N_Rd,c={:.1} kN", n_ed / 1e3, n_rd_c / 1e3),
            &format!("Betonausbruchkegel: N_Ed={:.1} kN ≤ N_Rd,c={:.1} kN", n_ed / 1e3, n_rd_c / 1e3),
        ));
    out.push(ndps(c.field("anchor-cone", vec![Station::ratio_of(&[0.0, 0.0], force_n(n_ed), force_n(n_rd_c.max(1.0)))]), &[]).build());

    let n_rd_p = part_4::pullout_resistance_n(annex, anchor.d, anchor.head_diameter, anchor.f_ck, anchor.cracked);
    let p = CheckResult::assess(format!("en1992-4.pullout.{}", anchor.id), part_4(), ClauseId::new("EN 1992-4", "§7.2.1.6", "7.2.1.6"), SubjectRef::new(&anchor.id, format!("anchors[id={}].aS", anchor.id), label.clone()), l("Pull-out", "Herausziehen")).remedies_attached_by("attach_minimal_remedies")
        .utilization(force_n(n_ed), force_n(n_rd_p.max(1.0)))
        .annex(annex)
        .explanation(l(
            &format!("Pull-out: N_Ed={:.1} kN ≤ N_Rd,p={:.1} kN", n_ed / 1e3, n_rd_p / 1e3),
            &format!("Herausziehen: N_Ed={:.1} kN ≤ N_Rd,p={:.1} kN", n_ed / 1e3, n_rd_p / 1e3),
        ));
    out.push(ndps(p, &["ndp.en1992.fa-04.1"]).build());

    let n_rd_sp = part_4::splitting_resistance_n(annex, anchor.f_ck, anchor.h_ef, anchor.c1, anchor.d, anchor.head_diameter, anchor.cracked);
    let sp = CheckResult::assess(format!("en1992-4.splitting.{}", anchor.id), part_4(), ClauseId::new("EN 1992-4", "§7.2.1.7", "7.2.1.7"), SubjectRef::new(&anchor.id, format!("anchors[id={}].c1", anchor.id), label.clone()), l("Splitting", "Spalten")).remedies_attached_by("attach_minimal_remedies")
        .utilization(force_n(n_ed), force_n(n_rd_sp.max(1.0)))
        .annex(annex)
        .explanation(l(
            &format!("Splitting: N_Ed={:.1} kN ≤ N_Rd,sp={:.1} kN", n_ed / 1e3, n_rd_sp / 1e3),
            &format!("Spalten: N_Ed={:.1} kN ≤ N_Rd,sp={:.1} kN", n_ed / 1e3, n_rd_sp / 1e3),
        ));
    out.push(ndps(sp, &[]).build());

    let n_rd_pr = part_4::pryout_resistance_n(annex, anchor.f_ck, anchor.h_ef, anchor.cracked, anchor.c1);
    let pr = CheckResult::assess(format!("en1992-4.pryout.{}", anchor.id), part_4(), ClauseId::new("EN 1992-4", "§7.2.2.4", "7.2.2.4"), SubjectRef::new(&anchor.id, format!("anchors[id={}].hEf", anchor.id), label.clone()), l("Pry-out", "Hebelausbruch")).remedies_attached_by("attach_minimal_remedies")
        .utilization(force_n(v_ed), force_n(n_rd_pr.max(1.0)))
        .annex(annex)
        .explanation(l(
            &format!("Pry-out: V_Ed={:.1} kN ≤ V_Rd,cp={:.1} kN", v_ed / 1e3, n_rd_pr / 1e3),
            &format!("Hebelausbruch: V_Ed={:.1} kN ≤ V_Rd,cp={:.1} kN", v_ed / 1e3, n_rd_pr / 1e3),
        ));
    out.push(ndps(pr, &["ndp.en1992.fa-03.1"]).build());

    let v_rd = part_4::edge_resistance_n(annex, anchor.d, anchor.h_ef, anchor.f_ck, anchor.c1, anchor.cracked);
    let e = CheckResult::assess(format!("en1992-4.edge.{}", anchor.id), part_4(), ClauseId::new("EN 1992-4", "§7.2.2.5", "7.2.2.5"), SubjectRef::new(&anchor.id, format!("anchors[id={}].c1", anchor.id), label.clone()), l("Edge breakout shear", "Randausbruch Querkraft")).remedies_attached_by("attach_minimal_remedies")
        .utilization(force_n(v_ed), force_n(v_rd.max(1.0)))
        .annex(annex)
        .explanation(l(
            &format!("Edge shear: V_Ed={:.1} kN ≤ V_Rd,c={:.1} kN", v_ed / 1e3, v_rd / 1e3),
            &format!("Randausbruch Querkraft: V_Ed={:.1} kN ≤ V_Rd,c={:.1} kN", v_ed / 1e3, v_rd / 1e3),
        ));
    out.push(ndps(e, &["ndp.en1992.fa-03.1"]).build());

    let v_rd_s = part_4::steel_shear_resistance_n(anchor.a_s, anchor.f_uk, anchor.f_yk);
    let vs = CheckResult::assess(format!("en1992-4.steel-shear.{}", anchor.id), part_4(), ClauseId::new("EN 1992-4", "§7.2.2.3", "7.2.2.3"), SubjectRef::new(&anchor.id, format!("anchors[id={}].aS", anchor.id), label.clone()), l("Anchor steel shear", "Dübel Stahlversagen Querkraft")).remedies_attached_by("attach_minimal_remedies")
        .utilization(force_n(v_ed), force_n(v_rd_s.max(1.0)))
        .annex(annex)
        .explanation(l(
            &format!("Steel shear: V_Ed={:.1} kN ≤ V_Rd,s={:.1} kN (0.5 A_s f_uk/γ_Ms,V)", v_ed / 1e3, v_rd_s / 1e3),
            &format!("Stahlquerkraft: V_Ed={:.1} kN ≤ V_Rd,s={:.1} kN (0,5 A_s f_uk/γ_Ms,V)", v_ed / 1e3, v_rd_s / 1e3),
        ));
    out.push(ndps(vs, &[]).build());

    let n_rd_t = n_rd_c.min(n_rd_p).min(n_rd_sp);
    let v_rd_t = v_rd.min(n_rd_pr);
    let inter = part_4::interaction_utilization(n_ed, n_rd_t, v_ed, v_rd_t).max(part_4::steel_interaction_utilization(n_ed, n_rd_s, v_ed, v_rd_s));
    let ix = CheckResult::assess(format!("en1992-4.interaction.{}", anchor.id), part_4(), ClauseId::new("EN 1992-4", "§7.2.3", "7.2.3"), SubjectRef::new(&anchor.id, format!("anchors[id={}]", anchor.id), label.clone()), l("Tension–shear interaction", "Zug–Querkraft-Interaktion")).remedies_attached_by("attach_minimal_remedies")
        .utilization(Quantity::new(QuantityKind::Dimensionless, inter), Quantity::new(QuantityKind::Dimensionless, 1.0))
        .annex(annex)
        .explanation(l(
            &format!("Interaction: (N/N_Rd)^1.5+(V/V_Rd)^1.5 and steel (N/N_Rd,s)²+(V/V_Rd,s)²: {:.3} ≤ 1 (N_Ed={:.1} kN, V_Ed={:.1} kN)", inter, n_ed / 1e3, v_ed / 1e3),
            &format!("Interaktion: (N/N_Rd)^1,5+(V/V_Rd)^1,5 und Stahl (N/N_Rd,s)²+(V/V_Rd,s)²: {:.3} ≤ 1 (N_Ed={:.1} kN, V_Ed={:.1} kN)", inter, n_ed / 1e3, v_ed / 1e3),
        ));
    out.push(ndps(ix, &["ndp.en1992.fa-03.1"]).build());
    out
}
