//! 🗂️ Design cases of the catalogue: the registry as the shared [`CaseCatalogue`] — typed inputs (unit, EN and DE label, choices, bounds) from [`super::case_inputs`], search, validation and sizing remedies from the compliance design-case bridge. See [`super`].

use super::*;
use semio_s_plugin_norm_compliance::design_case::{self, CaseCatalogue, CaseEntry, DesignCase, InputSpec};

/// 📚️ The catalogue as a [`CaseCatalogue`].
pub struct En1996Cases;

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
            input.vec.insert(spec.key.into(), values);
        }
    }
    input.paths.extend(case.values.numbers.keys().map(|key| (key.clone(), design_case::input_path(&case.id, key, true))).chain(case.values.texts.keys().map(|key| (key.clone(), design_case::input_path(&case.id, key, false)))));
    input
}

impl CaseCatalogue for En1996Cases {
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
    design_case::evaluate_case::<En1996Cases>(case, annex)
}

/// 🧱️ The catalogue entries a typed entity of the document already realises with its own checks (walls and their strength chain, load cases, openings with lintels, ties, bars, confining columns, chases, movement joints): they keep their stand-alone calculator but have no demo case, because the spec trace reads the checks of the typed rows instead.
pub const TYPED_ENTRIES: [&str; 37] = ["M-01", "M-02", "M-03", "M-04", "M-05", "M-06", "M-07", "M-10", "M-11", "M-12", "M-13", "M-14", "M-15", "M-16", "M-20", "M-21", "M-30", "M-31", "M-32", "M-33", "M-34", "M-40", "M-41", "M-42", "M-43", "M-50", "M-51", "M-52", "M-60", "M-61", "M-70", "M-80", "M-81", "M-82", "M-83", "M-90", "M-91"];

/// 📚️ The demo document of the catalogue: one seeded case per catalogue entry that no typed entity realises ([`TYPED_ENTRIES`]), so that every other entry is reachable from a document.
pub fn catalogue_demo_cases() -> Vec<design_case::CaseRow> {
    En1996Cases::entries()
        .iter()
        .filter(|entry| !TYPED_ENTRIES.contains(&entry.id))
        .map(|entry| {
            let mut case = design_case::seed::<En1996Cases>(entry, &demo_case_id(entry.id));
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
mod geometry;

#[path = "🌱️demo/🦀️.rs"]
mod demo;

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
