//! 🗂️ Design-case surface: the catalogue browser table, the unit-aware field specification of case inputs and the document-aware input rows the properties window edits. See [`super`] and `⚖️compliance/🧰️design-case`.

use super::*;
use semio_s_plugin_norm_compliance::design_case::{self, CaseCatalogue, CaseEntry, DesignCase, InputKind, InputSpec};
use std::collections::BTreeMap;
use std::sync::{Mutex, OnceLock};

//#region 🔖️FieldSpecs
fn leaked_choices(spec: &'static InputSpec) -> &'static [NormFieldChoice] {
    static CACHE: OnceLock<Mutex<BTreeMap<usize, &'static [NormFieldChoice]>>> = OnceLock::new();
    let mut cache = CACHE.get_or_init(Default::default).lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    *cache.entry(spec as *const InputSpec as usize).or_insert_with(|| &*Box::leak(spec.choices.iter().map(|c| NormFieldChoice { value: c.value, label_en: c.en, label_de: c.de }).collect::<Vec<_>>().into_boxed_slice()))
}

/// 🏷️ The typed, unit-aware field specification of one case input: numbers edit in their stored unit (any compatible unit is offered, bare numbers take the stored unit), choices and texts keep their labels.
pub fn input_field_spec(spec: &'static InputSpec, order: u16) -> FieldSpec {
    let base = FieldSpec { group: "inputs", order, affects: Affects::Entities(&["cases"]), ..FieldSpec::new(spec.en, spec.de) };
    match spec.kind {
        InputKind::Number | InputKind::Vector | InputKind::Series => FieldSpec {
            kind: FieldKind::Quantity { stored: spec.unit, display: spec.unit, units: if spec.unit == UnitId::One { &[UnitId::One] } else { &[] } },
            range: spec.range.map(|(min, max)| FieldRange::new(min, max)),
            ..base
        },
        InputKind::Choice => FieldSpec { kind: FieldKind::Choice(leaked_choices(spec)), ..base },
        InputKind::Text => FieldSpec { kind: FieldKind::Text, ..base },
    }
}

/// 🧭 The value-tree path template `cases[].numbers[].value` or `cases[].texts[].value` and the input key of a concrete case-input path such as `cases[id=C1].numbers[id=h].value`.
pub fn case_input_key(path: &str) -> Option<(&str, bool)> {
    let rest = path.strip_prefix("cases[")?;
    let (_, rest) = rest.split_once("].")?;
    let (number, rest) = if let Some(rest) = rest.strip_prefix("numbers[") { (true, rest) } else { (false, rest.strip_prefix("texts[")?) };
    let (selector, tail) = rest.split_once(']')?;
    (tail == ".value").then_some((selector.strip_prefix("id=")?, number))
}

/// 🏷️ Field metadata of a concrete case-input path from a vocabulary; keys that several entries define differently resolve to the first row, the document-aware [`case_input_rows`] resolve by entry.
pub fn case_input_field_meta(vocabulary: &'static [InputSpec], path: &str) -> Option<FieldSpec> {
    let (key, _) = case_input_key(path)?;
    let index = vocabulary.iter().position(|row| row.key == key && !row.is_sequence()).or_else(|| vocabulary.iter().position(|row| row.is_sequence() && row.element_index(key).is_some()))?;
    Some(input_field_spec(&vocabulary[index], index as u16))
}
//#endregion 🔖️FieldSpecs

//#region 🔖️Rows
fn vector_row(spec: &'static InputSpec, case: &DesignCase, order: usize, index: usize, declared: bool) -> CaseInputRow {
    let key = spec.element_key(index);
    CaseInputRow { spec, field: input_field_spec(spec, order as u16), path: design_case::input_path(&case.id, &key, true), number: case.values.numbers.get(&key).copied(), text: None, declared }
}

/// 📝 One editable input row of a case: the vocabulary definition, its field specification, its value-tree path and the current value (absent inputs are listed empty so they can be set).
#[derive(Clone, Debug, PartialEq)]
pub struct CaseInputRow {
    pub spec: &'static InputSpec,
    pub field: FieldSpec,
    pub path: String,
    pub number: Option<f64>,
    pub text: Option<String>,
    pub declared: bool,
}

/// 📝 The inputs of `case` for the properties window: the inputs its entry declares in declaration order (set or empty), then every other input the case stores.
pub fn case_input_rows<C: CaseCatalogue>(case: &DesignCase) -> Vec<CaseInputRow> {
    let Some(entry) = design_case::entry::<C>(&case.entry_id) else { return Vec::new() };
    let row = |spec: &'static InputSpec, order: usize, declared: bool| CaseInputRow {
        spec,
        field: input_field_spec(spec, order as u16),
        path: design_case::input_path(&case.id, spec.key, spec.is_number()),
        number: case.values.numbers.get(spec.key).copied(),
        text: case.values.texts.get(spec.key).cloned(),
        declared,
    };
    let mut rows: Vec<CaseInputRow> = Vec::new();
    let declared_vectors: Vec<&'static InputSpec> = design_case::declared::<C>(&entry).into_iter().filter(|spec| spec.is_sequence()).collect();
    for spec in design_case::declared::<C>(&entry) {
        if spec.is_sequence() {
            let count = (0..).take_while(|index| case.values.numbers.contains_key(&spec.element_key(*index))).count();
            for index in 0..=count {
                rows.push(vector_row(spec, case, rows.len(), index, true));
            }
        } else {
            rows.push(row(spec, rows.len(), true));
        }
    }
    let extras = case.values.numbers.keys().chain(case.values.texts.keys()).filter(|key| !entry.inputs.contains(&key.as_str()) && !declared_vectors.iter().any(|spec| spec.element_index(key).is_some()));
    for key in extras {
        match design_case::spec::<C>(entry.id, key) {
            Some(spec) if spec.is_sequence() => rows.push(vector_row(spec, case, rows.len(), spec.element_index(key).unwrap_or(0), false)),
            Some(spec) => rows.push(row(spec, rows.len(), false)),
            None => {}
        }
    }
    rows
}
//#endregion 🔖️Rows

//#region 🔖️Browser
/// 📚️ The catalogue browser: entries matching `query` (id, part, clause, English and German title) as a table with id, clause, part, title in `locale` and the number of declared inputs; row ids are the catalogue ids the add-case action takes.
pub fn case_catalogue_table(family: &str, entries: &[CaseEntry], query: &str, locale: Locale) -> CatalogueTable {
    let column = |id, label_en, label_de| CatalogueColumn { id, label_en, label_de, unit: None };
    CatalogueTable {
        id: "design-cases",
        title_en: "Catalogue entries",
        title_de: "Katalogeinträge",
        clause: ClauseId::new(family, "§0", "0"),
        columns: vec![column("id", "Id", "Id"), column("clause", "Clause", "Klausel"), column("part", "Part", "Teil"), column("title", "Title", "Titel"), CatalogueColumn { id: "inputs", label_en: "Inputs", label_de: "Eingaben", unit: None }],
        rows: design_case::search(entries, query)
            .iter()
            .map(|entry| CatalogueRow {
                id: entry.id.to_string(),
                cells: vec![CatalogueCell::text(entry.id), CatalogueCell::text(semio_s_plugin_norm_compliance::designator(entry.clause, &locale)), CatalogueCell::text(entry.part), CatalogueCell::text(entry.title(&locale)), CatalogueCell::number(entry.inputs.len() as f64, 0)],
            })
            .collect(),
    }
}
//#endregion 🔖️Browser

//#region 🔖️Verification
/// 🧪️ The verification-case catalogue of an artifact: every entry matching `query` evaluated once on its example inputs (`annex`), as id, clause, part, title, worst status, utilization and the governing explanation in `locale`; the table of the catalogue window shows what each normative case verifies without opening a design case.
pub fn verification_catalogue_table<C: CaseCatalogue>(family: &str, annex: semio_s_plugin_norm_compliance::AnnexChoice, query: &str, locale: Locale) -> CatalogueTable {
    let column = |id, label_en, label_de| CatalogueColumn { id, label_en, label_de, unit: None };
    let rank = |status: CheckStatus| match status {
        CheckStatus::Fail => 5,
        CheckStatus::Warning => 4,
        CheckStatus::UnverifiedBasis => 3,
        CheckStatus::Pass => 2,
        CheckStatus::Info => 1,
        CheckStatus::NotApplicable => 0,
    };
    CatalogueTable {
        id: "verification-cases",
        title_en: "Verification cases",
        title_de: "Nachweisfälle",
        clause: ClauseId::new(family, "§0", "0"),
        columns: vec![
            column("id", "Id", "Id"),
            column("clause", "Clause", "Klausel"),
            column("part", "Part", "Teil"),
            column("title", "Title", "Titel"),
            column("status", "Status", "Status"),
            column("utilization", "Utilization", "Ausnutzung"),
            column("explanation", "Governing result", "Maßgebendes Ergebnis"),
        ],
        rows: design_case::search(&C::entries(), query)
            .iter()
            .map(|entry| {
                let results = design_case::evaluate_case::<C>(&design_case::seed::<C>(entry, entry.id), annex);
                let governing = results.iter().max_by(|a, b| rank(a.status).cmp(&rank(b.status)).then(a.utilization.total_cmp(&b.utilization)));
                let cells = match governing {
                    Some(result) => vec![
                        CatalogueCell::text(entry.id),
                        CatalogueCell::text(semio_s_plugin_norm_compliance::designator(entry.clause, &locale)),
                        CatalogueCell::text(entry.part),
                        CatalogueCell::text(entry.title(&locale)),
                        CatalogueCell::text(status_label(result.status, locale)),
                        if result.utilization.is_finite() && result.status != CheckStatus::NotApplicable { CatalogueCell::number(result.utilization, 2) } else { CatalogueCell::Empty },
                        CatalogueCell::localized(result.explanation.en.clone(), result.explanation.de.clone()),
                    ],
                    None => vec![CatalogueCell::text(entry.id), CatalogueCell::text(semio_s_plugin_norm_compliance::designator(entry.clause, &locale)), CatalogueCell::text(entry.part), CatalogueCell::text(entry.title(&locale)), CatalogueCell::text(status_label(CheckStatus::NotApplicable, locale)), CatalogueCell::Empty, CatalogueCell::Empty],
                };
                CatalogueRow { id: entry.id.to_string(), cells }
            })
            .collect(),
    }
}
//#endregion 🔖️Verification

#[path = "✏️case-edits/🦀️.rs"]
mod case_edits;
pub use case_edits::*;

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
