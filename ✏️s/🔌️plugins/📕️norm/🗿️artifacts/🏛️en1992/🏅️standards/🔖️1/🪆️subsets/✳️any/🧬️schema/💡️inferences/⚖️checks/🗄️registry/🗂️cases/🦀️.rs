//! 🗂️ Design cases of the catalogue: the registry as the shared [`CaseCatalogue`] — typed inputs (unit, EN and DE label, choices, bounds) from [`super::case_inputs`], search, validation and sizing remedies from the compliance design-case bridge. See [`super`].

use super::*;
use semio_s_plugin_norm_compliance::design_case::{self, CaseCatalogue, CaseEntry, DesignCase, InputSpec};

/// 📚️ The catalogue as a [`CaseCatalogue`].
pub struct En1992Cases;

fn case_entry(e: &'static Entry) -> CaseEntry {
    CaseEntry { id: e.id, part: e.part, clause: e.clause, en: e.en, de: e.de, inputs: e.inputs, geometry: geometry::geometry_of(e.id) }
}

/// 🧾 The evaluator input of a design case.
pub fn case_input(annex: AnnexChoice, case: &DesignCase) -> Case {
    let mut input = Case::new(&case.id, annex);
    input.num.extend(case.values.numbers.iter().map(|(key, value)| (key.clone(), *value)));
    input.text.extend(case.values.texts.iter().map(|(key, value)| (key.clone(), value.clone())));
    for spec in super::case_inputs::VOCABULARY.iter().filter(|spec| spec.kind == design_case::InputKind::Vector) {
        if let Some(values) = case.values.vector(spec.key) {
            input.list.insert(spec.key.into(), values);
        }
    }
    input
}

impl CaseCatalogue for En1992Cases {
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
    design_case::evaluate_case::<En1992Cases>(case, annex)
}

/// 📚️ The demo document of the catalogue: one seeded case per catalogue entry, so that every entry is reachable from a document.
pub fn catalogue_demo_cases() -> Vec<design_case::CaseRow> {
    En1992Cases::entries()
        .iter()
        .map(|entry| {
            let mut case = design_case::seed::<En1992Cases>(entry, &demo_case_id(entry.id));
            case.values = demo::demo_values(entry.id, case.values);
            design_case::CaseRow::from(&case)
        })
        .collect()
}

/// 🆔 The id of the demo case of one catalogue entry: the entry id without characters a value path cannot address.
pub fn demo_case_id(entry_id: &str) -> String {
    format!("case-{}", entry_id.to_lowercase().chars().map(|c| if c.is_ascii_alphanumeric() { c } else { '-' }).collect::<String>())
}

/// 📐️ The geometry declarations of the cases: what each entry draws in 3D.
#[path = "📐️geometry/🦀️.rs"]
pub mod geometry;

#[path = "🌱️demo/🦀️.rs"]
mod demo;

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
