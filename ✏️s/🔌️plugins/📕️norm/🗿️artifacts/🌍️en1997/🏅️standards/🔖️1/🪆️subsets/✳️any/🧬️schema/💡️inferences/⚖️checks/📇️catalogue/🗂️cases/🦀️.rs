//! 🗂️ Design cases of the catalogue: the registry as the shared [`CaseCatalogue`] — typed inputs (unit, EN and DE label, choices, bounds) from [`super::case_inputs`], search, validation and sizing remedies from the compliance design-case bridge. See [`super`].

use super::*;
use semio_s_plugin_norm_compliance::design_case::{self, CaseCatalogue, CaseEntry, DesignCase, InputSpec};

/// 📚️ The catalogue as a [`CaseCatalogue`].
pub struct En1997Cases;

fn case_entry(e: &'static Entry) -> CaseEntry {
    CaseEntry { id: e.id, part: e.part, clause: e.clause, en: e.en, de: e.de, inputs: e.inputs, geometry: geometry::geometry_of(e.id) }
}

/// 🧾 The evaluator input of a design case.
pub fn case_input(annex: AnnexChoice, case: &DesignCase) -> Case {
    let mut input = Case::new(&case.id, annex);
    input.num.extend(case.values.numbers.iter().map(|(key, value)| (key.clone(), *value)));
    input.text.extend(case.values.texts.iter().map(|(key, value)| (key.clone(), value.clone())));
    input
}

impl CaseCatalogue for En1997Cases {
    fn entries() -> Vec<CaseEntry> {
        registry().into_iter().map(case_entry).collect()
    }

    fn vocabulary() -> &'static [InputSpec] {
        super::case_inputs::VOCABULARY
    }

    fn evaluate(entry: &CaseEntry, annex: AnnexChoice, case: &DesignCase) -> Vec<CheckResult> {
        evaluate_case(entry.id, &case_input(annex, case))
    }
}

/// 🧮 One design case through the shared bridge: input issues with remedies, then the catalogue checks addressed to the case row with a sizing remedy on failure.
pub fn evaluate_design_case(case: &DesignCase, annex: AnnexChoice) -> Vec<CheckResult> {
    design_case::evaluate_case::<En1997Cases>(case, annex)
}

/// 🆔 The id of the demo case of one catalogue entry: the entry id without characters a value path cannot address.
pub fn demo_case_id(entry_id: &str) -> String {
    format!("case-{}", entry_id.to_lowercase().chars().map(|c| if c.is_ascii_alphanumeric() { c } else { '-' }).collect::<String>())
}

/// 📚️ The full-catalogue demo document: one seeded design case for every catalogue entry, so that every id is reachable from a document. See [`evaluate_design_case`].
pub fn catalogue_demo_cases() -> Vec<semio_s_plugin_norm_compliance::design_case::CaseRow> {
    En1997Cases::entries()
        .iter()
        .map(|entry| {
            let mut case = design_case::seed::<En1997Cases>(entry, &demo_case_id(entry.id));
            case.values = demo::demo_values(entry.id, case.values);
            semio_s_plugin_norm_compliance::design_case::CaseRow::from(&case)
        })
        .collect()
}

/// 📐️ The geometry declarations of the cases: what each entry draws in 3D.
#[path = "📐️geometry/🦀️.rs"]
pub mod geometry;

#[path = "🌱️demo/🦀️.rs"]
mod demo;
