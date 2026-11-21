//! 📇️ DIN 4108 worked pressure tests under [GModG §26](https://www.gesetze-im-internet.de/geg/__26.html).

use super::{AirflowBasis, PressureTest, PressureTestGap, TestScope, evaluate_pressure_test};
use semio_s_plugin_norm_compliance::design_case::{CaseCatalogue, CaseEntry, DesignCase, InputSpec};
use semio_s_plugin_norm_compliance::{AnnexChoice, CheckResult, CheckStatus, ClauseId, LocalizedCopy, Quantity, QuantityKind, Remedy, SubjectRef};

#[path = "📚️inputs/🦀️.rs"]
mod inputs;

const STANDARD: &str = "GModG";
const ENTRIES: &[CaseEntry] = &[
    CaseEntry { id: "G26-01", part: STANDARD, clause: "§26(1)–(2)", en: "Airtightness by conditioned volume", de: "Luftdichtheit mit Bezug auf das Luftvolumen", inputs: &["test_protocol", "test_ventilation", "test_volume", "test_pressurisation", "test_depressurisation"], geometry: None },
    CaseEntry { id: "G26-02", part: STANDARD, clause: "§26(1), (3)", en: "Airtightness by envelope area", de: "Luftdichtheit mit Bezug auf die Hüllfläche", inputs: &["test_protocol", "test_ventilation", "test_volume", "test_area", "test_pressurisation", "test_depressurisation"], geometry: None },
    CaseEntry { id: "G26-03", part: STANDARD, clause: "§26(1)–(4)", en: "Separate non-residential zone test", de: "Getrennter Nachweis einer Nichtwohngebäudezone", inputs: &["test_protocol", "test_zone_basis", "test_ventilation", "test_volume", "test_basis", "test_area", "test_pressurisation", "test_depressurisation"], geometry: None },
    CaseEntry { id: "G26-04", part: STANDARD, clause: "§26(1), (2), (5)", en: "Documented sample of identical units", de: "Dokumentierte Stichprobe gleichartiger Nutzeinheiten", inputs: &["test_protocol", "test_sample_basis", "test_ventilation", "test_volume", "test_pressurisation", "test_depressurisation"], geometry: None },
];

/// 🗂️ Worked airflow tests available in the DIN 4108 catalogue.
pub struct Din4108PressureCases;

impl CaseCatalogue for Din4108PressureCases {
    fn entries() -> Vec<CaseEntry> {
        ENTRIES.to_vec()
    }

    fn new_document_case(entry: &CaseEntry, id: &str) -> DesignCase {
        let mut values = semio_s_plugin_norm_compliance::design_case::CaseValues::default();
        values.texts.insert("test_protocol".into(), "unconfirmed".into());
        DesignCase { id: id.into(), label_en: entry.en.into(), label_de: entry.de.into(), entry_id: entry.id.into(), values }
    }

    fn vocabulary() -> &'static [InputSpec] {
        inputs::VOCABULARY
    }

    fn evaluate(entry: &CaseEntry, _annex: AnnexChoice, case: &DesignCase) -> Vec<CheckResult> {
        let subject = SubjectRef::new(&case.id, format!("cases[id={}]", case.id), LocalizedCopy::new(entry.en, entry.de));
        let check = |key: &str, title: LocalizedCopy| {
            CheckResult::assess(format!("gmodg.26.{}.{key}.{}", entry.id.to_ascii_lowercase(), case.id), STANDARD, ClauseId::new(STANDARD, "§26", entry.clause), subject.clone(), title).annex(AnnexChoice::De)
        };
        let evaluated = pressure_input(entry.id, case).and_then(|test| evaluate_pressure_test(test).map(|result| (test, result)));
        let (test, result) = match evaluated {
            Ok(value) => value,
            Err(gap) => return vec![check("input", LocalizedCopy::new(entry.en, entry.de)).not_applicable(missing_basis(gap)).build()],
        };
        [
            ("pressurisation", "test_pressurisation", LocalizedCopy::new("Airflow at +50 Pa", "Volumenstrom bei +50 Pa"), test.pressurisation_m3_h, result.pressurisation_passes),
            ("depressurisation", "test_depressurisation", LocalizedCopy::new("Airflow at −50 Pa", "Volumenstrom bei −50 Pa"), test.depressurisation_m3_h, result.depressurisation_passes),
        ].into_iter().map(|(key, input, title, measured, passes)| {
            let current = normalized_flow(test, measured);
            let maximum = normalized_flow(test, result.limit_m3_h);
            let assessment = check(key, title.clone())
                .utilization(current, maximum)
                .status(if passes { CheckStatus::Pass } else { CheckStatus::Fail })
                .explanation(LocalizedCopy::new(
                    &format!("Measured {measured:.2} m³/h against the maximum {:.2} m³/h. Both pressure directions must meet the limit.", result.limit_m3_h),
                    &format!("Gemessen {measured:.2} m³/h gegenüber dem Höchstwert {:.2} m³/h. Beide Druckrichtungen müssen den Grenzwert einhalten.", result.limit_m3_h),
                ));
            if passes {
                return assessment.build();
            }
            let mut remedy = Remedy::at_most(
                SubjectRef::new(&case.id, semio_s_plugin_norm_compliance::design_case::input_path(&case.id, input, true), title),
                current,
                maximum,
                LocalizedCopy::new(
                    "Seal the leakage paths and repeat the pressure test. Record the new measured airflow.",
                    "Leckagen abdichten und die Differenzdruckmessung wiederholen. Den neu gemessenen Volumenstrom erfassen.",
                ),
            );
            remedy.applicable = false;
            assessment.remedy(remedy).build()
        }).collect()
    }
}

fn normalized_flow(test: PressureTest, cubic_metres_per_hour: f64) -> Quantity {
    match test.basis {
        AirflowBasis::Volume => Quantity::new(QuantityKind::VentilationRate, cubic_metres_per_hour / test.conditioned_volume_m3),
        AirflowBasis::EnvelopeArea { envelope_area_m2 } => Quantity::new(QuantityKind::AirPermeability, cubic_metres_per_hour / envelope_area_m2),
    }
}

fn pressure_input(entry_id: &str, case: &DesignCase) -> Result<PressureTest, PressureTestGap> {
    let text = |key: &str| case.values.texts.get(key).map(String::as_str).unwrap_or_default();
    let number = |key: &str| case.values.numbers.get(key).copied().unwrap_or(f64::NAN);
    let confirmed = text("test_protocol") == "confirmed";
    let mechanical_ventilation = match text("test_ventilation") {
        "mechanical" => true,
        "natural" => false,
        _ => return Err(PressureTestGap::VentilationDeclaration),
    };
    let basis = match (entry_id, text("test_basis")) {
        ("G26-02", _) | ("G26-03", "area") => AirflowBasis::EnvelopeArea { envelope_area_m2: number("test_area") },
        ("G26-01" | "G26-04", _) | ("G26-03", "volume") => AirflowBasis::Volume,
        _ => return Err(PressureTestGap::NormalisationBasis),
    };
    let scope = match entry_id {
        "G26-03" => TestScope::NonResidentialZone {
            non_residential: true,
            only_selected_zones: text("test_zone_basis") == "selected",
            different_requirements: text("test_zone_basis") == "different",
        },
        "G26-04" => {
            let sample = text("test_sample_basis") == "confirmed";
            TestScope::ExteriorUnitSample { identical_units: sample, exterior_access_only: sample, annex_nb_confirmed: sample }
        }
        _ => TestScope::WholeBuilding,
    };
    Ok(PressureTest {
        before_completion: confirmed,
        annex_na_confirmed: confirmed,
        conditioned_volume_m3: number("test_volume"),
        mechanical_ventilation,
        pressurisation_m3_h: number("test_pressurisation"),
        depressurisation_m3_h: number("test_depressurisation"),
        scope,
        basis,
    })
}

fn missing_basis(gap: PressureTestGap) -> LocalizedCopy {
    let (en, de) = match gap {
        PressureTestGap::MeasurementBasis => ("Confirm a pre-completion pressure test according to DIN EN ISO 9972:2018-12 Annex NA.", "Eine Prüfung vor Fertigstellung nach DIN EN ISO 9972:2018-12 Anhang NA bestätigen."),
        PressureTestGap::InvalidTestScope => ("Confirm eligibility for separate zone testing or sampling identical units.", "Zulässigkeit einer Zonenprüfung oder Stichprobe gleichartiger Nutzeinheiten bestätigen."),
        PressureTestGap::ConditionedVolume => ("Enter the heated or cooled air volume.", "Das beheizte oder gekühlte Luftvolumen eingeben."),
        PressureTestGap::MeasuredAirflow => ("Enter the measured airflows at +50 Pa and −50 Pa.", "Die gemessenen Volumenströme bei +50 Pa und −50 Pa eingeben."),
        PressureTestGap::IneligibleAreaMethod => ("The envelope-area method requires a conditioned air volume above 1,500 m³.", "Das Hüllflächenverfahren setzt ein konditioniertes Luftvolumen über 1.500 m³ voraus."),
        PressureTestGap::EnvelopeArea => ("Enter the envelope area for the tested subject.", "Die Hüllfläche des geprüften Bereichs eingeben."),
        PressureTestGap::VentilationDeclaration => ("Specify whether the tested subject has mechanical ventilation.", "Angeben, ob der geprüfte Bereich eine raumlufttechnische Anlage hat."),
        PressureTestGap::NormalisationBasis => ("Select the normalisation method for the tested zone.", "Bezugsgröße für die geprüfte Zone wählen."),
    };
    LocalizedCopy::new(en, de)
}

#[cfg(test)]
mod tests {
    use super::*;
    use semio_s_plugin_norm_compliance::design_case::{evaluate_case, seed};

    #[test]
    fn pressure_examples_show_both_directions_and_surface_input_gaps() {
        for entry in Din4108PressureCases::entries() {
            let mut case = seed::<Din4108PressureCases>(&entry, "example");
            let results = evaluate_case::<Din4108PressureCases>(&case, AnnexChoice::De);
            assert_eq!(results.len(), 2, "{}", entry.id);
            assert!(results.iter().all(|row| row.status == CheckStatus::Pass));
            let test = pressure_input(entry.id, &case).unwrap();
            let (kind, divisor) = match test.basis {
                AirflowBasis::Volume => (QuantityKind::VentilationRate, test.conditioned_volume_m3),
                AirflowBasis::EnvelopeArea { envelope_area_m2 } => (QuantityKind::AirPermeability, envelope_area_m2),
            };
            assert_eq!(results[0].computed, Quantity::new(kind, test.pressurisation_m3_h / divisor));
            assert_eq!(results[1].computed, Quantity::new(kind, test.depressurisation_m3_h / divisor));
            assert_eq!(results[0].limit.kind, kind);
            case.values.numbers.insert("test_depressurisation".into(), 50_000.0);
            let results = evaluate_case::<Din4108PressureCases>(&case, AnnexChoice::De);
            assert_eq!(results[0].status, CheckStatus::Pass);
            assert_eq!(results[1].status, CheckStatus::Fail);
            assert!(results[1].remedies.iter().any(|remedy| !remedy.applicable && remedy.target.path.ends_with("numbers[id=test_depressurisation].value")));
            case.values.texts.insert("test_protocol".into(), "unconfirmed".into());
            let results = evaluate_case::<Din4108PressureCases>(&case, AnnexChoice::De);
            assert_eq!(results.len(), 1);
            assert_eq!(results[0].status, CheckStatus::NotApplicable);
        }
    }
}
