//! 🤖️ Automation and energy checks: the building automation, the photovoltaic credit, the primary energy and the tabular method of the document evaluation. See [`super`].

use super::*;

/// 🤖 Checks the building automation class.
pub(super) fn check_automation(doc: &Din18599Snapshot, report: &mut CheckReport, annex: AnnexChoice) {
    {
        let f = automation_factor(doc.automation_class);
        let computed = if matches!(doc.automation_class, AutomationClass::D) { 1.1 } else { f };
        let mut b = CheckResult::assess(
            "din18599.11.automation",
            "DIN V 18599-11",
            ClauseId::new("DIN V 18599", "11", "5"),
            SubjectRef::new("", "automationClass", copy("Automation class", "Automationsklasse")),
            copy("Building automation class", "Gebäudeautomation"),
        )
        .utilization(dimensionless(computed), dimensionless(1.0))
        .annex(annex)
        .explanation(copy(
            &format!("BACS class {:?}, factor f_BAC = {f:.2}.", doc.automation_class),
            &format!("GA-Klasse {:?}, Faktor f_BAC = {f:.2}.", doc.automation_class),
        ));
        if matches!(doc.automation_class, AutomationClass::D) {
            b = b.remedy(Remedy::one_of(
                SubjectRef::new("", "automationClass", copy("Automation class", "Automationsklasse")),
                vec!["A".into(), "B".into(), "C".into()],
                copy("Upgrade building automation to class C or better.", "Gebäudeautomation auf Klasse C oder besser anheben."),
            ));
        }
        report.push(b.build());
    }
}

/// ⚡ Checks the primary energy against the reference and the photovoltaic credit.
pub(super) fn check_primary_energy(doc: &Din18599Snapshot, report: &mut CheckReport, annex: AnnexChoice, derived: &BalanceDerived) {
    {
        let q_p = derived.q_p_kwh;
        let limit = derived.q_p_limit_kwh.max(1.0);
        let mut b = CheckResult::assess(
            "din18599.geg.qp",
            "GEG §10 / Anlage 1–2",
            ClauseId::new("GEG", "§10", "Q_P"),
            SubjectRef::new("", "heating.energyCarrier", copy("Heating energy carrier", "Energieträger Heizung")),
            copy("Primary energy Q_P ≤ 0.55 · Q_P,Ref", "Primärenergie Q_P ≤ 0,55 · Q_P,Ref"),
        )
        .utilization(energy(q_p), energy(limit))
        .annex(annex)
        .explanation(copy(
            &format!("Q_P = {q_p:.0} kWh/a; Q_P,Ref = {:.0}; limit = 0.55·Q_P,Ref = {limit:.0} kWh/a.", derived.q_p_ref_kwh),
            &format!("Q_P = {q_p:.0} kWh/a; Q_P,Ref = {:.0}; Grenzwert = 0,55·Q_P,Ref = {limit:.0} kWh/a.", derived.q_p_ref_kwh),
        ));
        if q_p > limit {
            let deficit = q_p - limit;
            let fp_el = primary_energy_factor("electricity");
            let annual_g = doc.climate.g_h_w_m2.iter().sum::<f64>() / 12.0 * 8760.0 / 1000.0;
            let pv_extra = if doc.renewables.pv_efficiency * annual_g * fp_el <= 1e-9 {
                50.0
            } else {
                deficit / (doc.renewables.pv_efficiency.max(DIN_V_18599_5_ETA_SYS_FLOOR) * annual_g * fp_el)
            };
            let pv_req = doc.renewables.pv_area_m2 + pv_extra;
            b = b
                .remedy(Remedy::at_least(
                    SubjectRef::new("", "renewables.pvAreaM2", copy("PV area", "PV-Fläche")),
                    Quantity::new(QuantityKind::Area, doc.renewables.pv_area_m2),
                    Quantity::new(QuantityKind::Area, pv_req),
                    copy(
                        &format!("Increase PV area from {:.1} m² to at least {:.1} m².", doc.renewables.pv_area_m2, pv_req),
                        &format!("PV-Fläche von {:.1} m² auf mindestens {:.1} m² erhöhen.", doc.renewables.pv_area_m2, pv_req),
                    ),
                ))
                .remedy(Remedy::one_of(
                    SubjectRef::new("", "heating.energyCarrier", copy("Heating energy carrier", "Energieträger Heizung")),
                    vec!["natural_gas".into(), "biomass".into(), "district_heating".into()],
                    copy(
                        &format!("Switch heating carrier from '{}' (f_p={:.2}) to biomass or district heating.", doc.heating.energy_carrier, primary_energy_factor(&doc.heating.energy_carrier)),
                        &format!("Energieträger Heizung von '{}' (f_p={:.2}) auf Biomasse oder Fernwärme umstellen.", doc.heating.energy_carrier, primary_energy_factor(&doc.heating.energy_carrier)),
                    ),
                ));
            let gen_req = (doc.heating.generation_efficiency * (q_p / limit).min(1.4)).min(1.05);
            if gen_req > doc.heating.generation_efficiency {
                b = b.remedy(Remedy::at_least(
                    SubjectRef::new("", "heating.generationEfficiency", copy("Generation efficiency", "Erzeugungswirkungsgrad")),
                    dimensionless(doc.heating.generation_efficiency),
                    dimensionless(gen_req),
                    copy(
                        &format!("Raise generation efficiency from {:.2} to at least {:.2}.", doc.heating.generation_efficiency, gen_req),
                        &format!("Erzeugungswirkungsgrad von {:.2} auf mindestens {:.2} erhöhen.", doc.heating.generation_efficiency, gen_req),
                    ),
                ));
            }
        }
        report.push(b.build());
    }
}

/// 📊 Checks the tabular method where the calculation method is applicable.
pub(super) fn check_tabular_method(doc: &Din18599Snapshot, report: &mut CheckReport, annex: AnnexChoice, derived: &BalanceDerived) {
    match doc.method {
        CalculationMethod::Tabular => {
            let q_tab = tabular_specific_primary_energy_kwh_m2(doc.use_class) * doc.net_floor_area_m2 * automation_factor(doc.automation_class);
            let limit = derived.q_p_limit_kwh.max(1.0);
            let mut b = CheckResult::assess(
                "din18599.12.tabular",
                "DIN V 18599-12",
                ClauseId::new("DIN V 18599", "12", "4"),
                whole_building(),
                copy("Tabular primary energy", "Tabellarische Primärenergie"),
            )
            .utilization(energy(q_tab), energy(limit))
            .annex(annex)
            .explanation(copy(
                &format!("Q_P,tab = {q_tab:.0} kWh/a vs GEG limit {limit:.0}."),
                &format!("Q_P,tab = {q_tab:.0} kWh/a gegenüber GEG-Grenzwert {limit:.0}."),
            ));
            if q_tab > limit {
                b = b.remedy(Remedy::one_of(
                    SubjectRef::new("", "method", copy("Calculation method", "Berechnungsverfahren")),
                    vec!["DetailedMonthly".into()],
                    copy("Switch to detailed monthly method to resolve tabular exceedance.", "Auf detailliertes Monatsbilanzverfahren wechseln."),
                ));
            }
            report.push(b.build());
        }
        CalculationMethod::DetailedMonthly => {
            report.push(
                CheckResult::assess(
                    "din18599.12.tabular",
                    "DIN V 18599-12",
                    ClauseId::new("DIN V 18599", "12", "4"),
                    whole_building(),
                    copy("Tabular primary energy", "Tabellarische Primärenergie"),
                )
                .not_applicable(copy("Detailed monthly method selected; tabular method not applicable.", "Detailliertes Monatsverfahren gewählt; Tabellenverfahren nicht anwendbar."))
                .annex(annex)
                .build(),
            );
        }
    }
}
