//! 🌬️ DIN 4108-7:2026-04 §4 airtightness concept documentation and quality assurance.

use semio_s_plugin_norm_compliance::design_case::{CaseCatalogue, CaseEntry, CaseValues, DesignCase, InputSpec};
use semio_s_plugin_norm_compliance::{AnnexChoice, CheckResult, CheckStatus, ClauseId, LocalizedCopy, SubjectRef};

const STANDARD: &str = "DIN 4108-7:2026-04";
pub(crate) const PLAN_ENTRY: &str = "DINL-01";
const KEYS: &[&str] = &["plan_layer", "plan_interfaces", "plan_materials", "plan_assurance"];
const INPUTS: &[InputSpec] = &[
    InputSpec::text("plan_layer", "Route or drawing of the continuous airtight layer", "Verlauf oder Zeichnung der durchgehenden Luftdichtheitsebene").scope(PLAN_ENTRY),
    InputSpec::text("plan_interfaces", "Junction and penetration concepts or references", "Anschluss- und Durchdringungskonzepte oder Verweise").scope(PLAN_ENTRY),
    InputSpec::text("plan_materials", "Specified airtight materials and their locations", "Vorgesehene luftdichte Baustoffe und ihre Zuordnung").scope(PLAN_ENTRY),
    InputSpec::text("plan_assurance", "Quality assurance measures, timing and responsibility", "Qualitätssicherung, Zeitpunkt und Zuständigkeit").scope(PLAN_ENTRY),
];

/// 📋️ Identifies authored documentation fields with no substantive entry.
pub(crate) fn documentation_missing(records: [&str; 4]) -> [bool; 4] {
    records.map(|record| record.trim().is_empty())
}

fn plan_records(values: &CaseValues) -> [&str; 4] {
    std::array::from_fn(|index| values.texts.get(KEYS[index]).map_or("", String::as_str))
}

/// 🌬️ The persistent airtightness concept in the DIN 4108 design-case editor.
pub(crate) struct Din4108AirtightnessPlans;

impl CaseCatalogue for Din4108AirtightnessPlans {
    fn entries() -> Vec<CaseEntry> {
        vec![CaseEntry {
            id: PLAN_ENTRY, part: STANDARD, clause: "4 / Annex B",
            en: "Airtightness concept", de: "Luftdichtheitskonzept", inputs: KEYS, geometry: None,
        }]
    }

    fn vocabulary() -> &'static [InputSpec] { INPUTS }

    fn evaluate(entry: &CaseEntry, annex: AnnexChoice, case: &DesignCase) -> Vec<CheckResult> {
        INPUTS.iter().zip(documentation_missing(plan_records(&case.values))).map(|(input, missing)| {
            let title = LocalizedCopy::new(input.en, input.de);
            let subject = SubjectRef::new(&case.id, format!("cases[id={}].{}", case.id, input.key), title.clone());
            let (status, explanation) = if missing {
                (CheckStatus::UnverifiedBasis, LocalizedCopy::new(
                    "Record the evidence or enter the project document reference.",
                    "Nachweis erfassen oder auf die Projektunterlage verweisen.",
                ))
            } else {
                (CheckStatus::Info, LocalizedCopy::new(
                    "Evidence recorded for technical review.",
                    "Nachweis zur fachlichen Prüfung erfasst.",
                ))
            };
            CheckResult::assess(
                format!("din4108-7.plan.{}.{}", input.key, case.id),
                STANDARD, ClauseId::new("DIN 4108-7", "4", entry.clause), subject, title,
            ).annex(annex).status(status).explanation(explanation).build()
        }).collect()
    }
}
