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
            "GModG §§15, 18, 20, 21",
            ClauseId::new("GModG", if doc.building_category == BuildingCategory::Residential { "§§15, 20" } else { "§§18, 21" }, "Q_P"),
            SubjectRef::new("", "heating.energyCarrier", copy("Heating energy carrier", "Energieträger Heizung")),
            copy("Primary energy screening", "Vorprüfung der Primärenergie"),
        )
        .utilization(energy(q_p), energy(limit))
        .annex(annex)
        .explanation(copy(
            &format!("Q_P = {q_p:.0} kWh/a; Q_P,Ref = {:.0}; limit = 0.55·Q_P,Ref = {limit:.0} kWh/a.", derived.q_p_ref_kwh),
            &format!("Q_P = {q_p:.0} kWh/a; Q_P,Ref = {:.0}; Grenzwert = 0,55·Q_P,Ref = {limit:.0} kWh/a.", derived.q_p_ref_kwh),
        ));
        let unresolved_carrier = [
            ("heating.energyCarrier", doc.heating.energy_carrier.as_str()),
            ("dhw.energyCarrier", doc.dhw.energy_carrier.as_str()),
            ("cooling.plant.energyCarrier", doc.cooling.energy_carrier()),
        ].into_iter().find(|(path, carrier)| {
            (*path != "cooling.plant.energyCarrier" || doc.cooling.is_installed()) && !has_direct_primary_energy_factor(carrier)
        });
        let factor_basis = CheckResult::assess(
            "din18599.geg.primary-energy-factor-basis", "GModG §22",
            ClauseId::new("GModG", "§22", "Anlage 4"),
            SubjectRef::new("", unresolved_carrier.map_or("heating.energyCarrier", |(path, _)| path), copy("Energy carrier", "Energieträger")),
            copy("Primary-energy factor basis", "Grundlage des Primärenergiefaktors"),
        ).annex(annex);
        let factor_basis = if unresolved_carrier.is_some() {
            factor_basis.status(CheckStatus::UnverifiedBasis).explanation(copy(
                "Verify the supply or plant-specific primary-energy factor. The current balance is indicative.",
                "Der Primärenergiefaktor der Versorgung oder Anlage ist nachzuweisen. Die aktuelle Bilanz ist überschlägig.",
            ))
        } else {
            factor_basis.status(CheckStatus::Info).explanation(copy(
                "The calculation uses the Annex 4 default factors for the entered energy carriers.",
                "Die Berechnung verwendet die Standardfaktoren aus Anlage 4 für die erfassten Energieträger.",
            ))
        };
        report.push(factor_basis.build());
        if doc.building_category == BuildingCategory::NonResidential {
            b = b.explanation(copy(
                &format!("Indicative Q_P = {q_p:.0} kWh/a and reference limit {limit:.0} kWh/a. Annex 2 envelope values are used where known. The reference plant and solar-control glazing require verification."),
                &format!("Überschlägiger Q_P = {q_p:.0} kWh/a und Referenzgrenzwert {limit:.0} kWh/a. Bekannte Hüllwerte nach Anlage 2 werden verwendet. Referenzanlagentechnik und Sonnenschutzverglasung sind nachzuweisen."),
            ));
        }
        if doc.method == CalculationMethod::Tabular
            || doc.building_category == BuildingCategory::NonResidential
            || unresolved_carrier.is_some()
            || doc.renewables.pv_area_m2 > 0.0
            || doc.renewables.solar_thermal_kwh_a > 0.0
            || (doc.building_category == BuildingCategory::Residential && residential_dhw_profile_kwh_a(doc).is_none())
        {
            b = b.status(CheckStatus::UnverifiedBasis);
        }
        if q_p > limit {
            b = b.remedy(Remedy::one_of(
                SubjectRef::new("", "heating.energyCarrier", copy("Heating energy carrier", "Energieträger Heizung")),
                vec!["natural_gas".into(), "biomass".into(), "district_heating".into()],
                copy(
                    &format!("Review the heating carrier '{}' (f_p={:.2}) and consider verified alternatives.", doc.heating.energy_carrier, primary_energy_factor(&doc.heating.energy_carrier)),
                    &format!("Energieträger Heizung '{}' (f_p={:.2}) prüfen und verifizierte Alternativen erwägen.", doc.heating.energy_carrier, primary_energy_factor(&doc.heating.energy_carrier)),
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
    let generated = derived.renewable_electricity_months.iter().map(|month| month.generation_kwh).sum::<f64>();
    let credit = derived.q_pv_kwh;
    let subject = SubjectRef::new("", "renewables.pvAreaM2", copy("Renewable electricity credit", "Anrechnung erneuerbaren Stroms"));
    let b = CheckResult::assess(
        "din18599.geg.renewable-electricity", "GModG §23", ClauseId::new("GModG", "§23", "Strom"),
        subject, copy("Monthly renewable electricity credit", "Monatliche Anrechnung erneuerbaren Stroms"),
    ).annex(annex).utilization(energy(credit), energy(generated));
    let thermal = doc.renewables.solar_thermal_kwh_a;
    let b = if doc.renewables.pv_area_m2 > 0.0 || thermal > 0.0 {
        b.status(CheckStatus::UnverifiedBasis).explanation(copy(
            &format!("Indicative PV production {generated:.0} kWh/a. Monthly demand limits the modelled electricity credit to {credit:.0} kWh/a. A §23 assessment needs qualified equipment and Potsdam irradiation data. Recorded solar thermal heat {thermal:.0} kWh/a requires allocation to thermal services."),
            &format!("Überschlägiger PV-Ertrag {generated:.0} kWh/a. Der monatliche Bedarf begrenzt die modellierte Stromanrechnung auf {credit:.0} kWh/a. Für §23 sind qualifizierte Anlagen- und Potsdamer Strahlungsdaten erforderlich. Erfasste Solarwärme von {thermal:.0} kWh/a ist den thermischen Diensten zuzuordnen."),
        ))
    } else {
        b.not_applicable(copy("The building has no recorded renewable generation.", "Für das Gebäude ist keine erneuerbare Erzeugung erfasst."))
    };
    report.push(b.build());
}

/// 📊 Selects the reported scope of the residential tabular method.
pub(super) fn check_tabular_method(doc: &Din18599Snapshot, report: &mut CheckReport, annex: AnnexChoice) {
    let subject = SubjectRef::new("", "method", copy("Calculation method", "Berechnungsverfahren"));
    let result = CheckResult::assess(
        "din18599.12.tabular", "DIN V 18599-12",
        ClauseId::new("DIN V 18599", "12", "Anwendungsbereich"),
        subject.clone(), copy("Residential tabular method", "Tabellenverfahren für Wohngebäude"),
    ).annex(annex);
    let result = match (doc.method, doc.building_category) {
        (CalculationMethod::DetailedMonthly, _) => result.not_applicable(copy(
            "The detailed monthly method is selected.",
            "Das detaillierte Monatsbilanzverfahren ist gewählt.",
        )),
        (CalculationMethod::Tabular, BuildingCategory::NonResidential) => result
            .status(CheckStatus::NotApplicable)
            .explanation(copy(
                "Part 12 applies to residential buildings. Select the detailed monthly method for this building.",
                "Teil 12 gilt für Wohngebäude. Für dieses Gebäude ist das detaillierte Monatsbilanzverfahren zu wählen.",
            )).remedy(Remedy::one_of(
                subject, vec!["DetailedMonthly".into()],
                copy("Select the detailed monthly method.", "Detailliertes Monatsbilanzverfahren wählen."),
            )),
        (CalculationMethod::Tabular, BuildingCategory::Residential) => result
            .status(CheckStatus::UnverifiedBasis)
            .explanation(copy(
                "The residential tabular method requires the applicable building and system tables from the selected edition.",
                "Das Tabellenverfahren für Wohngebäude erfordert die auf Gebäude und Anlagentechnik anwendbaren Tabellen der gewählten Ausgabe.",
            )),
    };
    report.push(result.build());
}
