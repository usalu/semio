//! 🏠️ Envelope and heating checks: the GEG transmission loss, the monthly heating demand and the ventilation heat recovery of the document evaluation. See [`super`].

use super::*;

/// 🧱 Checks the GEG transmission heat loss coefficient or the mean U value.
pub(super) fn check_transmission_loss(doc: &Din18599Snapshot, report: &mut CheckReport, annex: AnnexChoice) {
    match doc.building_category {
        BuildingCategory::Residential => {
            let check = CheckResult::assess(
                "din18599.geg.ht-prime",
                "GEG § 16 / Annex 1",
                ClauseId::new("GEG", "§ 16", "Anlage 1"),
                SubjectRef::new("", "deltaUWbWM2k", copy("Thermal-bridge surcharge ΔU_WB", "Wärmebrückenzuschlag ΔU_WB")),
                copy("Specific transmission heat loss H′T", "Spezifischer Transmissionswärmeverlust H′T"),
            ).annex(annex);
            let Some(limit) = geg_ht_prime_limit(doc) else {
                report.push(check.status(CheckStatus::Fail).explanation(copy(
                    "The Annex 1 comparison requires a positive heat-transferring envelope area.",
                    "Der Vergleich nach Anlage 1 erfordert eine positive wärmeübertragende Umfassungsfläche.",
                )).build());
                return;
            };
            let computed = h_t_prime(doc);
            let mut b = check.utilization(heat_transfer(computed), heat_transfer(limit)).explanation(copy(
                &format!("Specific transmission heat loss is {computed:.3} W/(m²·K). The Annex 1 reference with the same envelope geometry gives the § 16 limit of {limit:.3} W/(m²·K)."),
                &format!("Der spezifische Transmissionswärmeverlust beträgt {computed:.3} W/(m²·K). Das Referenzgebäude nach Anlage 1 mit gleicher Hüllgeometrie ergibt den Grenzwert nach § 16 von {limit:.3} W/(m²·K)."),
            ));
            if computed > limit {
                for el in doc.elements.iter().filter(|el| el.adjacency != Adjacency::Heated) {
                    let target = reference_u(el);
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
        BuildingCategory::NonResidential => check_annex3_envelope(doc, report, annex),
    }
}

fn check_annex3_envelope(doc: &Din18599Snapshot, report: &mut CheckReport, annex: AnnexChoice) {
    let (groups, missing_basis) = annex3_envelope_groups(doc);
    for group in &groups {
        let (en, de) = group.class.labels();
        let title = copy(&format!("{en} at {} °C", group.setpoint_c), &format!("{de} bei {} °C", group.setpoint_c));
        let check = CheckResult::assess(
            group.check_id(), "GModG §19 / Annex 3",
            ClauseId::new("GModG", "Anlage 3", group.class.clause()),
            whole_building(), title,
        ).annex(annex);
        let Some(mean) = group.mean_u() else {
            report.push(check.status(CheckStatus::UnverifiedBasis).explanation(copy(
                "Verify component areas and U-values. Record ground slabs as perimeter or interior parts using the 5 m distance from the outside edge.",
                "Bauteilflächen und U-Werte prüfen. Bodenplatten sind ab 5 m Abstand vom äußeren Rand in Rand- und Innenbereiche zu unterteilen.",
            )).build());
            continue;
        };
        let limit = group.limit();
        let mut check = check.utilization(heat_transfer(mean), heat_transfer(limit)).explanation(copy(
            &format!("The area-weighted mean is {mean:.3} W/(m²·K). Annex 3 limits this group to {limit:.2} W/(m²·K)."),
            &format!("Der flächengewichtete Mittelwert beträgt {mean:.3} W/(m²·K). Anlage 3 begrenzt diese Gruppe auf {limit:.2} W/(m²·K)."),
        ));
        if mean > limit {
            for element in group.elements.iter().filter(|element| element.u_value_w_m2k > 0.0) {
                let target = element.u_value_w_m2k * limit / mean;
                let path = element_u_path(element);
                check = check.remedy(Remedy::at_most(
                    element_ref(element, &path), heat_transfer(element.u_value_w_m2k), heat_transfer(target),
                    copy(
                        &format!("Reduce the U-value to {target:.3} W/(m²·K) with the proposed changes for this group."),
                        &format!("U-Wert mit den vorgeschlagenen Änderungen dieser Gruppe auf {target:.3} W/(m²·K) senken."),
                    ),
                ));
            }
        }
        report.push(check.build());
    }
    if missing_basis || groups.is_empty() {
        let check = CheckResult::assess(
            "din18599.geg.mean-u.basis", "GModG §19 / Annex 3",
            ClauseId::new("GModG", "Anlage 3", "Anwendungsbereich"),
            whole_building(), copy("Envelope classification", "Einstufung der Gebäudehülle"),
        ).annex(annex);
        let check = if missing_basis {
            check.status(CheckStatus::UnverifiedBasis).explanation(copy(
                "Assign the heating setpoint and correct ground adjacency for the recorded envelope components.",
                "Für die erfassten Hüllbauteile Heizsolltemperatur und korrekten Erdreichkontakt angeben.",
            ))
        } else {
            check.not_applicable(copy(
                "No represented exterior components serve zones heated to at least 12 °C.",
                "Die erfassten Außenbauteile grenzen an keine Zonen mit Heizsolltemperatur ab 12 °C.",
            ))
        };
        report.push(check.build());
    }
}

/// 🏘️ Report the selected residential heat gain rate or the missing classification in a detailed monthly balance.
pub(super) fn check_residential_internal_gains(doc: &Din18599Snapshot, report: &mut CheckReport, annex: AnnexChoice) {
    if doc.building_category != BuildingCategory::Residential
        || doc.method != CalculationMethod::DetailedMonthly
        || !doc.zones.iter().any(|zone| zone.usage_profile == UsageProfile::WFH)
    {
        return;
    }

    let subject = SubjectRef::new("", "dhw.dwellingCount", copy("Number of dwellings", "Anzahl der Wohneinheiten"));
    let check = CheckResult::assess(
        "din18599.10.residential-internal-gains",
        RESIDENTIAL_GAINS_STANDARD,
        ClauseId::new(RESIDENTIAL_GAINS_STANDARD, "10", "5, Table 4"),
        subject,
        copy("Residential internal heat gains", "Interne Wärmegewinne im Wohngebäude"),
    ).annex(annex);

    let check = if let Some(daily) = residential_daily_gain_wh_m2(doc) {
        let power = daily / 24.0;
        check.status(CheckStatus::Pass).explanation(copy(
            &format!("{RESIDENTIAL_GAINS_STANDARD} § 5, Table 4 applies {daily:.0} Wh/(m² NGF·day), equivalent to {power} W/m². Monthly internal heat gains use the residential zone’s net floor area and calendar days."),
            &format!("{RESIDENTIAL_GAINS_STANDARD} § 5, Tabelle 4 setzt {daily:.0} Wh/(m² NGF·Tag) an. Dies entspricht {power} W/m². Die monatlichen internen Wärmegewinne ergeben sich aus Nettogrundfläche und Kalendertagen."),
        ))
    } else {
        check.status(CheckStatus::UnverifiedBasis).explanation(copy(
            "Enter the number of dwellings to calculate residential internal gains under Table 4. The balance currently uses the entered gains for the residential zones.",
            "Für interne Wärmegewinne nach Tabelle 4 ist die Anzahl der Wohneinheiten erforderlich. Die Bilanz verwendet derzeit die eingegebenen Wärmegewinne der Wohnzonen.",
        ))
    };
    report.push(check.build());
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
            copy("Net heating demand against modelled reference", "Heizwärmebedarf gegenüber modellierter Referenz"),
        )
        .utilization(energy(q_h), energy(q_h_ref))
        .annex(annex)
        .explanation(copy(
            &format!("Q_H,nd = {q_h:.0} kWh/a vs reference building Q_H,nd,Ref = {q_h_ref:.0} kWh/a (H_T={:.1} W/K, H_V={:.1} W/K).", derived.h_t, derived.h_v),
            &format!("Q_H,nd = {q_h:.0} kWh/a gegenüber Referenzgebäude Q_H,nd,Ref = {q_h_ref:.0} kWh/a (H_T={:.1} W/K, H_V={:.1} W/K).", derived.h_t, derived.h_v),
        ));
        if q_h > q_h_ref {
            for el in &doc.elements {
                let reference = energy_reference_element(doc, el);
                let target = reference.u_value_w_m2k;
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
                if el.kind.transmits_solar_radiation() {
                    if el.g_value + 1e-9 < reference.g_value {
                        let path = element_g_path(el);
                        b = b.remedy(Remedy::at_least(
                            element_ref(el, &path),
                            dimensionless(el.g_value),
                            dimensionless(reference.g_value),
                            copy(
                                &format!("Raise g-value of {} to at least {:.2} (GEG reference) to recover solar gains toward Q_H,nd,Ref.", el.label_en, reference.g_value),
                                &format!("g-Wert von {} auf mindestens {:.2} anheben (GEG-Referenz), um solare Gewinne Richtung Q_H,nd,Ref zu stärken.", el.label_de, reference.g_value),
                            ),
                        ));
                    }
                    if el.fc + 1e-9 < reference.fc {
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
