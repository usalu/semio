//! 📚️ The DIN 4108 design-case catalogue combining the supported standard procedures.

use crate::part_3::catalogue::Din4108PeriodCases;
use crate::part_7::pressure_test::catalogue::Din4108PressureCases;
use crate::part_7::concept::{Din4108AirtightnessPlans, PLAN_ENTRY};
use semio_s_plugin_norm_compliance::design_case::{CaseCatalogue, CaseEntry, DesignCase, InputSpec};
use semio_s_plugin_norm_compliance::{AnnexChoice, CheckResult};
use std::sync::OnceLock;

/// 📋️ Design cases for the DIN 4108 document editor.
pub struct Din4108Cases;

impl CaseCatalogue for Din4108Cases {
    fn entries() -> Vec<CaseEntry> {
        let mut entries = Din4108PressureCases::entries();
        entries.extend(Din4108PeriodCases::entries());
        entries.extend(Din4108AirtightnessPlans::entries());
        entries
    }

    fn vocabulary() -> &'static [InputSpec] {
        static VOCABULARY: OnceLock<Vec<InputSpec>> = OnceLock::new();
        VOCABULARY.get_or_init(|| {
            Din4108PressureCases::vocabulary().iter().chain(Din4108PeriodCases::vocabulary()).chain(Din4108AirtightnessPlans::vocabulary()).copied().collect()
        })
    }

    fn new_document_case(entry: &CaseEntry, id: &str) -> DesignCase {
        if Din4108PeriodCases::contains(entry.id) {
            Din4108PeriodCases::new_document_case(entry, id)
        } else if entry.id == PLAN_ENTRY {
            Din4108AirtightnessPlans::new_document_case(entry, id)
        } else {
            Din4108PressureCases::new_document_case(entry, id)
        }
    }

    fn evaluate(entry: &CaseEntry, annex: AnnexChoice, case: &DesignCase) -> Vec<CheckResult> {
        if Din4108PeriodCases::contains(entry.id) {
            Din4108PeriodCases::evaluate(entry, annex, case)
        } else if entry.id == PLAN_ENTRY {
            Din4108AirtightnessPlans::evaluate(entry, annex, case)
        } else {
            Din4108PressureCases::evaluate(entry, annex, case)
        }
    }
}
