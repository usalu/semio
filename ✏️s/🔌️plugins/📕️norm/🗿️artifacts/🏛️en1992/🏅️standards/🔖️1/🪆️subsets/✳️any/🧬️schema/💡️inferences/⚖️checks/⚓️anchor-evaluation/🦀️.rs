//! ⚓️ EN 1992 anchor evaluation: EN 1990 combination of anchor actions and the EN 1992-4 resistances.

use super::*;

/// ⚖️ Combine characteristic anchor actions (N_k, V_k and allied extras) per EN 1990 into design N_Ed, V_Ed.
pub fn combine_anchor_actions(anchor: &Anchor) -> Vec<DesignEffects> {
    let mut g = (0.0_f64, 0.0_f64);
    let mut variables: Vec<(&LoadCaseActions, (f64, f64))> = Vec::new();
    let mut accidentals = Vec::new();
    for a in &anchor.actions {
        let mut n = a.n_k;
        let mut v = a.v_k;
        match a.source.as_str() {
            "udl" => {
                n += a.g_k_line.abs() + a.q_k_line.abs();
            }
            "point" => {
                n += a.point_force.abs();
            }
            "external" => {}
            _ => {
                n += a.point_force.abs() + a.g_k_line.abs() + a.q_k_line.abs();
            }
        }
        if anchor.h_ef > 0.0 {
            n += a.m_k.abs() / anchor.h_ef;
        }
        if anchor.c1 > 0.0 {
            v += a.t_k.abs() / anchor.c1;
        }
        v += a.v_k_punch.abs();
        let eff = (n, v);
        match a.kind.as_str() {
            "permanent" | "prestress" => { g.0 += eff.0; g.1 += eff.1; }
            "accidental" => accidentals.push((a, eff)),
            _ => variables.push((a, eff)),
        }
    }
    let gamma_g = 1.35;
    let gamma_q = 1.5;
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
                let (psi0, _, _) = psi_factors(&oa.category);
                n += gamma_q * psi0 * oe.0;
                v += gamma_q * psi0 * oe.1;
            }
            out.push(DesignEffects {
                m_ed: 0.0, n_ed: n, v_ed: v, t_ed: 0.0, v_ed_punch: 0.0,
                combination_id: format!("ULS-6.10b-{}", lead_a.id), situation: "uls".into(),
            });
            let (_, _, psi2) = psi_factors(&lead_a.category);
            let mut n_qp = g.0 + psi2 * lead_e.0;
            let mut v_qp = g.1 + psi2 * lead_e.1;
            for (j, (oa, oe)) in variables.iter().enumerate() {
                if j == lead_i { continue; }
                let (_, _, p2) = psi_factors(&oa.category);
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
            let (_, psi1, _) = psi_factors(&oa.category);
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

/// ⚓️ Evaluate anchors with EN 1990-combined characteristic actions + EN 1992-4 resistances.
pub fn evaluate_anchor(doc: &En1992Snapshot, anchor: &Anchor) -> Vec<CheckResult> {
    let annex = doc.annex;
    let label = l(
        &format!("{} — Anchor {}", doc.title, anchor.id),
        &format!("{} — Dübel {}", doc.title, anchor.id),
    );
    let mut out = Vec::new();
    let effects = combine_anchor_actions(anchor);
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
    let mut s = CheckResult::assess(format!("en1992-4.steel.{}", anchor.id), part_4(), ClauseId::new("EN 1992-4", "§7.2.1.4", "7.2.1.4"), SubjectRef::new(&anchor.id, format!("anchors[id={}].aS", anchor.id), label.clone()), l("Anchor steel tension", "Dübel Stahlzugtragfähigkeit"))
        .utilization(force_n(n_ed), force_n(n_rd_s.max(1.0)))
        .annex(annex)
        .explanation(l(
            &format!("Anchor steel: N_Ed={:.1} kN ≤ N_Rd,s={:.1} kN ({})", n_ed / 1e3, n_rd_s / 1e3, uls.combination_id),
            &format!("Dübel Stahlzug: N_Ed={:.1} kN ≤ N_Rd,s={:.1} kN ({})", n_ed / 1e3, n_rd_s / 1e3, uls.combination_id),
        ));
    if n_ed > n_rd_s {
        let a_req = anchor.a_s * n_ed / n_rd_s;
        s = s.remedy(Remedy::at_least(
            SubjectRef::new(&anchor.id, format!("anchors[id={}].aS", anchor.id), label.clone()),
            area_m2(anchor.a_s), area_m2(a_req),
            l(&format!("Increase anchor steel area to ≥ {:.1} mm².", a_req * 1e6), &format!("Dübelstahlquerschnitt auf ≥ {:.1} mm² erhöhen.", a_req * 1e6)),
        ));
    }
    out.push(s.build());

    if let Some(sls) = governing(&effects, "sls_qp") {
        let n_sls = sls.n_ed.max(0.0);
        let n_rd_sls = anchor.a_s * anchor.f_yk / 1.15;
        let mut ss = CheckResult::assess(format!("en1992-4.steel.sls.{}", anchor.id), part_4(), ClauseId::new("EN 1992-4", "§7.2.1.4", "7.2.1.4"), SubjectRef::new(&anchor.id, format!("anchors[id={}].actions", anchor.id), label.clone()), l("Anchor steel SLS (ψ₂)", "Dübel Stahlzug GZG (ψ₂)"))
            .utilization(force_n(n_sls), force_n(n_rd_sls.max(1.0)))
            .annex(annex)
            .explanation(l(
                &format!("Anchor SLS: N_Ed,qp={:.1} kN ≤ N_Rd,s={:.1} kN ({})", n_sls / 1e3, n_rd_sls / 1e3, sls.combination_id),
                &format!("Dübel GZG: N_Ed,qp={:.1} kN ≤ N_Rd,s={:.1} kN ({})", n_sls / 1e3, n_rd_sls / 1e3, sls.combination_id),
            ));
        if n_sls > n_rd_sls {
            ss = ss.remedy(Remedy::at_least(
                SubjectRef::new(&anchor.id, format!("anchors[id={}].aS", anchor.id), label.clone()),
                area_m2(anchor.a_s), area_m2(anchor.a_s * n_sls / n_rd_sls),
                l("Increase anchor steel area for SLS.", "Dübelstahlquerschnitt für GZG erhöhen."),
            ));
        }
        out.push(ss.build());
    }

    let n_rd_c = part_4::cone_resistance_n(anchor.f_ck, anchor.h_ef, anchor.cracked);
    let mut c = CheckResult::assess(format!("en1992-4.cone.{}", anchor.id), part_4(), ClauseId::new("EN 1992-4", "§7.2.1.5", "7.2.1.5"), SubjectRef::new(&anchor.id, format!("anchors[id={}].hEf", anchor.id), label.clone()), l("Concrete cone", "Betonausbruchkegel"))
        .utilization(force_n(n_ed), force_n(n_rd_c.max(1.0)))
        .annex(annex)
        .explanation(l(
            &format!("Concrete cone: N_Ed={:.1} kN ≤ N_Rd,c={:.1} kN", n_ed / 1e3, n_rd_c / 1e3),
            &format!("Betonausbruchkegel: N_Ed={:.1} kN ≤ N_Rd,c={:.1} kN", n_ed / 1e3, n_rd_c / 1e3),
        ));
    if n_ed > n_rd_c {
        let h_req = anchor.h_ef * (n_ed / n_rd_c.max(1e-12)).powf(2.0 / 3.0) * 1.05;
        c = c.remedy(Remedy::at_least(
            SubjectRef::new(&anchor.id, format!("anchors[id={}].hEf", anchor.id), label.clone()),
            length_m(anchor.h_ef), length_m(h_req),
            l(&format!("Increase effective embedment to ≥ {:.0} mm.", h_req * 1000.0), &format!("Effektive Verankerungstiefe auf ≥ {:.0} mm erhöhen.", h_req * 1000.0)),
        ));
    }
    out.push(c.build());

    let n_rd_p = part_4::pullout_resistance_n(anchor.a_s, anchor.f_uk);
    let mut p = CheckResult::assess(format!("en1992-4.pullout.{}", anchor.id), part_4(), ClauseId::new("EN 1992-4", "§7.2.1.6", "7.2.1.6"), SubjectRef::new(&anchor.id, format!("anchors[id={}].aS", anchor.id), label.clone()), l("Pull-out", "Herausziehen"))
        .utilization(force_n(n_ed), force_n(n_rd_p.max(1.0)))
        .annex(annex)
        .explanation(l(
            &format!("Pull-out: N_Ed={:.1} kN ≤ N_Rd,p={:.1} kN", n_ed / 1e3, n_rd_p / 1e3),
            &format!("Herausziehen: N_Ed={:.1} kN ≤ N_Rd,p={:.1} kN", n_ed / 1e3, n_rd_p / 1e3),
        ));
    if n_ed > n_rd_p {
        p = p.remedy(Remedy::at_least(
            SubjectRef::new(&anchor.id, format!("anchors[id={}].aS", anchor.id), label.clone()),
            area_m2(anchor.a_s), area_m2(anchor.a_s * n_ed / n_rd_p),
            l("Increase steel area against pull-out.", "Stahlquerschnitt gegen Herausziehen vergrößern."),
        ));
    }
    out.push(p.build());

    let n_rd_sp = part_4::splitting_resistance_n(anchor.f_ck, anchor.h_ef, anchor.c1);
    let mut sp = CheckResult::assess(format!("en1992-4.splitting.{}", anchor.id), part_4(), ClauseId::new("EN 1992-4", "§7.2.1.7", "7.2.1.7"), SubjectRef::new(&anchor.id, format!("anchors[id={}].c1", anchor.id), label.clone()), l("Splitting", "Spalten"))
        .utilization(force_n(n_ed), force_n(n_rd_sp.max(1.0)))
        .annex(annex)
        .explanation(l(
            &format!("Splitting: N_Ed={:.1} kN ≤ N_Rd,sp={:.1} kN", n_ed / 1e3, n_rd_sp / 1e3),
            &format!("Spalten: N_Ed={:.1} kN ≤ N_Rd,sp={:.1} kN", n_ed / 1e3, n_rd_sp / 1e3),
        ));
    if n_ed > n_rd_sp {
        let ratio = (n_ed / n_rd_sp.max(1e-12)).max(1.0) * 1.05;
        let h_cap = (anchor.c1 / 1.5).max(anchor.h_ef);
        let gain_at_cap = (h_cap / anchor.h_ef.max(1e-12)).powf(1.5);
        let h_req = if gain_at_cap + 1e-12 >= ratio {
            anchor.h_ef * ratio.powf(2.0 / 3.0)
        } else {
            let need_sqrt_h = ratio * anchor.h_ef.powf(1.5) * 1.5 / anchor.c1.max(1e-12);
            need_sqrt_h.powi(2)
        };
        sp = sp.remedy(Remedy::at_least(
            SubjectRef::new(&anchor.id, format!("anchors[id={}].hEf", anchor.id), label.clone()),
            length_m(anchor.h_ef), length_m(h_req),
            l(
                &format!("Increase effective embedment to ≥ {:.0} mm against splitting.", h_req * 1000.0),
                &format!("Effektive Verankerungstiefe auf ≥ {:.0} mm gegen Spalten erhöhen.", h_req * 1000.0),
            ),
        ));
    }
    out.push(sp.build());

    let n_rd_pr = part_4::pryout_resistance_n(anchor.f_ck, anchor.h_ef, anchor.cracked);
    let mut pr = CheckResult::assess(format!("en1992-4.pryout.{}", anchor.id), part_4(), ClauseId::new("EN 1992-4", "§7.2.2.4", "7.2.2.4"), SubjectRef::new(&anchor.id, format!("anchors[id={}].hEf", anchor.id), label.clone()), l("Pry-out", "Hebelausbruch"))
        .utilization(force_n(v_ed), force_n(n_rd_pr.max(1.0)))
        .annex(annex)
        .explanation(l(
            &format!("Pry-out: V_Ed={:.1} kN ≤ V_Rd,cp={:.1} kN", v_ed / 1e3, n_rd_pr / 1e3),
            &format!("Hebelausbruch: V_Ed={:.1} kN ≤ V_Rd,cp={:.1} kN", v_ed / 1e3, n_rd_pr / 1e3),
        ));
    if v_ed > n_rd_pr {
        pr = pr.remedy(Remedy::at_least(
            SubjectRef::new(&anchor.id, format!("anchors[id={}].hEf", anchor.id), label.clone()),
            length_m(anchor.h_ef), length_m(anchor.h_ef * 1.2),
            l("Increase embedment against pry-out.", "Verankerungstiefe gegen Hebelausbruch erhöhen."),
        ));
    }
    out.push(pr.build());

    let v_rd = part_4::edge_resistance_n(anchor.d, anchor.h_ef, anchor.f_ck, anchor.c1);
    let mut e = CheckResult::assess(format!("en1992-4.edge.{}", anchor.id), part_4(), ClauseId::new("EN 1992-4", "§7.2.2.5", "7.2.2.5"), SubjectRef::new(&anchor.id, format!("anchors[id={}].c1", anchor.id), label.clone()), l("Edge breakout shear", "Randausbruch Querkraft"))
        .utilization(force_n(v_ed), force_n(v_rd.max(1.0)))
        .annex(annex)
        .explanation(l(
            &format!("Edge shear: V_Ed={:.1} kN ≤ V_Rd,c={:.1} kN", v_ed / 1e3, v_rd / 1e3),
            &format!("Randausbruch Querkraft: V_Ed={:.1} kN ≤ V_Rd,c={:.1} kN", v_ed / 1e3, v_rd / 1e3),
        ));
    if v_ed > v_rd {
        let c_req = anchor.c1 * (v_ed / v_rd).powf(2.0 / 3.0);
        e = e.remedy(Remedy::at_least(
            SubjectRef::new(&anchor.id, format!("anchors[id={}].c1", anchor.id), label.clone()),
            length_m(anchor.c1), length_m(c_req),
            l(&format!("Increase edge distance c1 to ≥ {:.0} mm.", c_req * 1000.0), &format!("Randabstand c1 auf ≥ {:.0} mm erhöhen.", c_req * 1000.0)),
        ));
    }
    out.push(e.build());

    let n_rd_t = n_rd_s.min(n_rd_c).min(n_rd_p).min(n_rd_sp);
    let v_rd_t = v_rd.min(n_rd_pr);
    let inter = part_4::interaction_utilization(n_ed, n_rd_t, v_ed, v_rd_t);
    let mut ix = CheckResult::assess(format!("en1992-4.interaction.{}", anchor.id), part_4(), ClauseId::new("EN 1992-4", "§7.2.3", "7.2.3"), SubjectRef::new(&anchor.id, format!("anchors[id={}]", anchor.id), label.clone()), l("Tension–shear interaction", "Zug–Querkraft-Interaktion"))
        .utilization(Quantity::new(QuantityKind::Dimensionless, inter), Quantity::new(QuantityKind::Dimensionless, 1.0))
        .annex(annex)
        .explanation(l(
            &format!("Interaction: (N/N_Rd)^1.5+(V/V_Rd)^1.5={:.3} ≤ 1 (N_Ed={:.1} kN, V_Ed={:.1} kN)", inter, n_ed / 1e3, v_ed / 1e3),
            &format!("Interaktion: (N/N_Rd)^1,5+(V/V_Rd)^1,5={:.3} ≤ 1 (N_Ed={:.1} kN, V_Ed={:.1} kN)", inter, n_ed / 1e3, v_ed / 1e3),
        ));
    if inter > 1.0 {
        let scale = inter.max(1.0) * 1.15;
        let h_req = anchor.h_ef * scale.powf(2.0 / 3.0);
        let c_req = anchor.c1 * scale.powf(2.0 / 3.0);
        let a_req = anchor.a_s * scale;
        ix = ix.remedy(Remedy::at_least(
            SubjectRef::new(&anchor.id, format!("anchors[id={}].hEf", anchor.id), label.clone()),
            length_m(anchor.h_ef), length_m(h_req),
            l(
                &format!("Increase effective embedment to ≥ {:.0} mm for tension–shear interaction.", h_req * 1000.0),
                &format!("Effektive Verankerungstiefe auf ≥ {:.0} mm für Zug–Querkraft-Interaktion erhöhen.", h_req * 1000.0),
            ),
        ));
        ix = ix.remedy(Remedy::at_least(
            SubjectRef::new(&anchor.id, format!("anchors[id={}].c1", anchor.id), label.clone()),
            length_m(anchor.c1), length_m(c_req),
            l(
                &format!("Increase edge distance to ≥ {:.0} mm for tension–shear interaction.", c_req * 1000.0),
                &format!("Randabstand auf ≥ {:.0} mm für Zug–Querkraft-Interaktion erhöhen.", c_req * 1000.0),
            ),
        ));
        ix = ix.remedy(Remedy::at_least(
            SubjectRef::new(&anchor.id, format!("anchors[id={}].aS", anchor.id), label),
            area_m2(anchor.a_s), area_m2(a_req),
            l(
                &format!("Increase anchor steel area to ≥ {:.1} mm² for tension–shear interaction.", a_req * 1e6),
                &format!("Dübelstahlquerschnitt auf ≥ {:.1} mm² für Zug–Querkraft-Interaktion erhöhen.", a_req * 1e6),
            ),
        ));
    }
    out.push(ix.build());
    out
}
