//! 🏠️ Envelope and heating checks: the GEG transmission loss, the monthly heating demand and the ventilation heat recovery of the document evaluation. See [`super`].

use super::*;

/// 🧱 Checks the GEG transmission heat loss coefficient or the mean U value.
pub(super) fn check_transmission_loss(doc: &Din18599Snapshot, report: &mut CheckReport, annex: AnnexChoice, derived: &BalanceDerived) {
    match doc.building_category {
        BuildingCategory::Residential => {
            let limit = geg_ht_prime_limit(doc.attachment, doc.net_floor_area_m2);
            let computed = derived.h_t_prime;
            let mut b = CheckResult::assess(
                "din18599.geg.ht-prime",
                "GEG §16 / Anlage 2",
                ClauseId::new("GEG", "Anlage 2", "H'T"),
                SubjectRef::new("", "deltaUWbWM2k", copy("Thermal-bridge surcharge ΔU_WB", "Wärmebrückenzuschlag ΔU_WB")),
                copy("Specific transmission heat loss H′T", "Spezifischer Transmissionswärmeverlust H′T"),
            )
            .utilization(heat_transfer(computed), heat_transfer(limit))
            .annex(annex)
            .explanation(copy(
                &format!("H′T = H_T/A = {computed:.3} W/(m²·K); limit {limit:.2} W/(m²·K) for {:?} with A_N={:.0} m².", doc.attachment, doc.net_floor_area_m2),
                &format!("H′T = H_T/A = {computed:.3} W/(m²·K); Grenzwert {limit:.2} W/(m²·K) für {:?} mit A_N={:.0} m².", doc.attachment, doc.net_floor_area_m2),
            ));
            if computed > limit {
                for el in &doc.elements {
                    let target = reference_u(el.kind);
                    if el.u_value_w_m2k > target + 1e-9 {
                        let path = element_u_path(el);
                        b = b.remedy(Remedy::at_most(
                            element_ref(el, &path),
                            heat_transfer(el.u_value_w_m2k),
                            heat_transfer(target),
                            copy(
                                &format!("Reduce U of {} from {:.3} to GEG reference {:.3} W/(m²·K).", el.label_en, el.u_value_w_m2k, target),
                                &format!("U-Wert von {} von {:.3} auf GEG-Referenz {:.3} W/(m²·K) senken.", el.label_de, el.u_value_w_m2k, target),
                            ),
                        ));
                    }
                }
                if doc.delta_u_wb_w_m2k > GEG_ANLAGE1_DELTA_U_WB_REF + 1e-9 {
                    b = b.remedy(Remedy::at_most(
                        SubjectRef::new("", "deltaUWbWM2k", copy("Thermal-bridge surcharge ΔU_WB", "Wärmebrückenzuschlag ΔU_WB")),
                        heat_transfer(doc.delta_u_wb_w_m2k),
                        heat_transfer(GEG_ANLAGE1_DELTA_U_WB_REF),
                        copy(
                            "Lower ΔU_WB to the GEG reference surcharge 0.05 W/(m²·K).",
                            "ΔU_WB auf den GEG-Referenzzuschlag 0.05 W/(m²·K) senken.",
                        ),
                    ));
                }
            }
            report.push(b.build());
        }
        BuildingCategory::NonResidential => {
            for kind in [ElementKind::Wall, ElementKind::Roof, ElementKind::Floor, ElementKind::Window] {
                match mean_u_for_kind(doc, kind) {
                    None => {
                        report.push(
                            CheckResult::assess(
                                format!("din18599.geg.mean-u.{kind:?}").to_lowercase(),
                                "GEG Anlage 3",
                                ClauseId::new("GEG", "Anlage 3", format!("{kind:?}")),
                                whole_building(),
                                copy(&format!("Mean U ({kind:?})"), &format!("Mittlerer U-Wert ({kind:?})")),
                            )
                            .not_applicable(copy("No elements of this kind.", "Keine Bauteile dieser Art vorhanden."))
                            .annex(annex)
                            .build(),
                        );
                    }
                    Some((mean, limit)) => {
                        let mut b = CheckResult::assess(
                            format!("din18599.geg.mean-u.{kind:?}").to_lowercase(),
                            "GEG Anlage 3",
                            ClauseId::new("GEG", "Anlage 3", format!("{kind:?}")),
                            whole_building(),
                            copy(&format!("Mean U-value ({kind:?})"), &format!("Mittlerer U-Wert ({kind:?})")),
                        )
                        .utilization(heat_transfer(mean), heat_transfer(limit))
                        .annex(annex)
                        .explanation(copy(
                            &format!("Ū = {mean:.3} W/(m²·K) vs limit {limit:.2}."),
                            &format!("Ū = {mean:.3} W/(m²·K) gegenüber Grenzwert {limit:.2}."),
                        ));
                        if mean > limit {
                            if let Some(el) = doc.elements.iter().filter(|e| e.kind == kind).max_by(|a, b| a.u_value_w_m2k.partial_cmp(&b.u_value_w_m2k).unwrap_or(std::cmp::Ordering::Equal)) {
                                let path = element_u_path(el);
                                b = b.remedy(Remedy::at_most(
                                    element_ref(el, &path),
                                    heat_transfer(el.u_value_w_m2k),
                                    heat_transfer(limit),
                                    copy(
                                        &format!("Reduce U of {} to at most {limit:.2} W/(m²·K).", el.label_en),
                                        &format!("U-Wert von {} auf höchstens {limit:.2} W/(m²·K) senken.", el.label_de),
                                    ),
                                ));
                            }
                        }
                        report.push(b.build());
                    }
                }
            }
        }
    }
}

/// 🔥 Checks the monthly heating demand against the GEG reference.
pub(super) fn check_monthly_heating_demand(doc: &Din18599Snapshot, report: &mut CheckReport, annex: AnnexChoice, derived: &BalanceDerived) {
    {
        let q_h = derived.q_h_nd_kwh;
        let q_h_ref = derived.q_h_ref_kwh.max(1.0);
        let mut b = CheckResult::assess(
            "din18599.2.heating-demand",
            "DIN V 18599-2",
            ClauseId::new("DIN V 18599", "2", "5"),
            whole_building(),
            copy("Net heating demand Q_H,nd", "Heizwärmebedarf Q_H,nd"),
        )
        .utilization(energy(q_h), energy(q_h_ref))
        .annex(annex)
        .explanation(copy(
            &format!("Q_H,nd = {q_h:.0} kWh/a vs reference building Q_H,nd,Ref = {q_h_ref:.0} kWh/a (H_T={:.1} W/K, H_V={:.1} W/K).", derived.h_t, derived.h_v),
            &format!("Q_H,nd = {q_h:.0} kWh/a gegenüber Referenzgebäude Q_H,nd,Ref = {q_h_ref:.0} kWh/a (H_T={:.1} W/K, H_V={:.1} W/K).", derived.h_t, derived.h_v),
        ));
        if q_h > q_h_ref {
            for el in &doc.elements {
                let target = reference_u(el.kind);
                if el.u_value_w_m2k > target + 1e-9 {
                    let path = element_u_path(el);
                    b = b.remedy(Remedy::at_most(
                        element_ref(el, &path),
                        heat_transfer(el.u_value_w_m2k),
                        heat_transfer(target),
                        copy(
                            &format!("Improve envelope: set U of {} ≤ {:.3} W/(m²·K) (GEG reference).", el.label_en, target),
                            &format!("Hülle verbessern: U von {} ≤ {:.3} W/(m²·K) (GEG-Referenz).", el.label_de, target),
                        ),
                    ));
                }
                if el.kind == ElementKind::Window {
                    if el.g_value + 1e-9 < GEG_ANLAGE1_WINDOW_G_REF {
                        let path = element_g_path(el);
                        b = b.remedy(Remedy::at_least(
                            element_ref(el, &path),
                            dimensionless(el.g_value),
                            dimensionless(GEG_ANLAGE1_WINDOW_G_REF),
                            copy(
                                &format!("Raise g-value of {} to at least {:.2} (GEG reference) to recover solar gains toward Q_H,nd,Ref.", el.label_en, GEG_ANLAGE1_WINDOW_G_REF),
                                &format!("g-Wert von {} auf mindestens {:.2} anheben (GEG-Referenz), um solare Gewinne Richtung Q_H,nd,Ref zu stärken.", el.label_de, GEG_ANLAGE1_WINDOW_G_REF),
                            ),
                        ));
                    }
                    if el.fc + 1e-9 < 1.0 {
                        let path = element_fc_path(el);
                        b = b.remedy(Remedy::at_least(
                            element_ref(el, &path),
                            dimensionless(el.fc),
                            dimensionless(1.0),
                            copy(
                                &format!("Raise shading factor F_c of {} to 1.0 so solar gains match the GEG reference building.", el.label_en),
                                &format!("Abminderungsfaktor F_c von {} auf 1.0 anheben, damit solare Gewinne dem GEG-Referenzgebäude entsprechen.", el.label_de),
                            ),
                        ));
                    }
                }
            }
            if doc.delta_u_wb_w_m2k > GEG_ANLAGE1_DELTA_U_WB_REF + 1e-9 {
                b = b.remedy(Remedy::at_most(
                    SubjectRef::new("", "deltaUWbWM2k", copy("Thermal-bridge surcharge ΔU_WB", "Wärmebrückenzuschlag ΔU_WB")),
                    heat_transfer(doc.delta_u_wb_w_m2k),
                    heat_transfer(GEG_ANLAGE1_DELTA_U_WB_REF),
                    copy("Lower ΔU_WB to 0.05 W/(m²·K) to match the reference heat demand.", "ΔU_WB auf 0.05 W/(m²·K) senken, um den Referenz-Heizwärmebedarf zu erreichen."),
                ));
            }
        }
        report.push(b.build());
    }
}

/// 🌬 Checks the ventilation heat recovery of parts 6 and 7.
pub(super) fn check_ventilation_heat_recovery(doc: &Din18599Snapshot, report: &mut CheckReport, annex: AnnexChoice, derived: &BalanceDerived) {
    {
        let eta = doc.ventilation.heat_recovery_eta;
        let min_eta = GEG_ANLAGE1_TABELLE1_ETA_WRG_REF;
        let mut b = CheckResult::assess(
            "din18599.6.heat-recovery",
            "DIN V 18599-6",
            ClauseId::new("DIN V 18599", "6", "6"),
            SubjectRef::new("", "ventilation.heatRecoveryEta", copy("Heat recovery η_WRG", "Wärmerückgewinnung η_WRG")),
            copy("Ventilation heat recovery", "Wärmerückgewinnung Lüftung"),
        )
        .minimum(dimensionless(eta), dimensionless(min_eta))
        .annex(annex)
        .explanation(copy(
            &format!("η_WRG = {eta:.2}; H_V = {:.1} W/K at {:.0} m³/h.", derived.h_v, doc.ventilation.airflow_m3_h),
            &format!("η_WRG = {eta:.2}; H_V = {:.1} W/K bei {:.0} m³/h.", derived.h_v, doc.ventilation.airflow_m3_h),
        ));
        if eta < min_eta {
            b = b.remedy(Remedy::at_least(
                SubjectRef::new("", "ventilation.heatRecoveryEta", copy("Heat recovery η_WRG", "Wärmerückgewinnung η_WRG")),
                dimensionless(eta),
                dimensionless(min_eta),
                copy(
                    &format!("Increase heat recovery from {:.0}% to at least {:.0}%.", eta * 100.0, min_eta * 100.0),
                    &format!("Wärmerückgewinnung von {:.0}% auf mindestens {:.0}% erhöhen.", eta * 100.0, min_eta * 100.0),
                ),
            ));
        }
        report.push(b.build());
    }
}
