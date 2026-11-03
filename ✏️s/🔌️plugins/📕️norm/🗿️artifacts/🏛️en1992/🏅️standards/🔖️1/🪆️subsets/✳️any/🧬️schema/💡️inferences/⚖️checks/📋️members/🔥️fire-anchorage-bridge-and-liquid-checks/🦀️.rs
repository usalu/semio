//! 🔥️ Fire, anchorage, bridge and liquid checks: the fire resistance, the anchorage and laps, the bridge stress and the liquid retaining tightness of the member evaluation. See [`super`].

use super::*;

/// 🔥 Checks the fire resistance minimum dimensions and axis distances.
pub(super) fn check_fire(doc: &En1992Snapshot, member: &RcMember, out: &mut Vec<CheckResult>, annex: AnnexChoice, b: f64) {
    if let Some(fire) = &member.fire {
        let (b_min, a_req) = part_1_2_fire::required_for(member.kind, member.support, fire.rating, fire.slab_system);
        let mut fa = CheckResult::assess(format!("en1992.1-2.a.{}", member.id), part_12(), ClauseId::new("EN 1992-1-2", "§5.6", "5.6"), member_ref(doc, member, &member_path(&member.id, "fire.axisDistance")), l("Fire axis distance", "Achsabstand Brandschutz"))
            .minimum(length_m(fire.axis_distance), length_m(a_req))
            .annex(annex)
            .explanation(l(
                &format!("Fire axis distance a={:.0} mm ≥ a_req={:.0} mm ({:?}; slab {:?})", fire.axis_distance * 1000.0, a_req * 1000.0, fire.rating, fire.slab_system),
                &format!("Brandschutz-Achsabstand a={:.0} mm ≥ a_erf={:.0} mm ({:?}; Platte {:?})", fire.axis_distance * 1000.0, a_req * 1000.0, fire.rating, fire.slab_system),
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
        out.push(ndps(fa, &["ndp.en1992.r-01.1"]).build());
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
        out.push(ndps(fb, &["ndp.en1992.r-01.1"]).build());
    }
}

/// ⚓ Checks the anchorage lengths and the lap lengths of the longitudinal layers.
pub(super) fn check_anchorage_and_laps(
    doc: &En1992Snapshot,
    member: &RcMember,
    out: &mut Vec<CheckResult>,
    annex: AnnexChoice,
    concrete: &semio_s_artifact_norm_en1992_model::ConcreteGrade,
    f_yk: f64,
    params: &na_de::AnnexParams,
) {
    for layer in &member.longitudinal {
        let eta1 = if layer.bond_condition == semio_s_artifact_norm_en1992_model::BondCondition::Poor { 0.7 } else { 1.0 };
        let f_bd = f_bd(annex, concrete.f_ck / 1.0e6, layer.diameter * 1000.0, layer.bond_condition != semio_s_artifact_norm_en1992_model::BondCondition::Poor) * 1.0e6;
        let sigma_sd = params.f_yd_pa(f_yk);
        let lb_rqd = part_1_1::l_b_rqd_m(layer.diameter, sigma_sd, f_bd);
        let n_bars = layer.count.max(1) as f64;
        let spacing = if n_bars > 1.0 { (member.width - 2.0 * member.cover - layer.diameter) / (n_bars - 1.0) } else { f64::MAX };
        let cd = (spacing / 2.0).min(member.cover).max(layer.diameter);
        let alpha2 = (1.0 - 0.15 * (cd - layer.diameter) / layer.diameter).clamp(0.7, 1.0);
        let lbd = part_1_1::l_bd_m(lb_rqd, layer.diameter, alpha2);
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
        }
        out.push(ndps(anc, &["ndp.en1992.b-01.1"]).build());
        let alpha6 = (100.0 * layer.lap_share.clamp(0.0, 1.0) / 25.0_f64).sqrt().clamp(1.0, 1.5);
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
        out.push(ndps(lap, &["ndp.en1992.b-04.1"]).build());
    }
}

/// 🌉 Checks the bridge stress limits and the fatigue stress range.
pub(super) fn check_bridge(doc: &En1992Snapshot, member: &RcMember, out: &mut Vec<CheckResult>, annex: AnnexChoice, f_ck: f64) {
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
            out.push(ndps(br, &[]).build());
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
            out.push(ndps(bf, &["ndp.en1992.f-03.1"]).build());
        }
    }
}

/// 💧 Checks the tightness crack limit of liquid retaining members.
pub(super) fn check_liquid_retaining(doc: &En1992Snapshot, member: &RcMember, out: &mut Vec<CheckResult>, annex: AnnexChoice, e_s: f64) {
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
                    let w_k = crack_width_value(member.liquid_s_r_max, strain_difference_alpha(member.liquid_sigma_s, e_s, e_s / 22.0e9, member.liquid_rho_p_eff.max(1e-4), 0.4, member.liquid_f_ct_eff));
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
                    out.push(ndps(liq, &["ndp.en1992.lq-01.1"]).build());
                }
            }
        }
    }
}

