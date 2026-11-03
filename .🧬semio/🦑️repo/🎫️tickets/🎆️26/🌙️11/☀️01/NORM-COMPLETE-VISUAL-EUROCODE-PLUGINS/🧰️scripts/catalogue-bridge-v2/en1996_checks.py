import os as _o, sys as _s; _o.environ.get("TH_FORCE") or (_s.stderr.write("unsafe against the current tree (destructive or not idempotent), refusing to run (TH_FORCE=1 forces)\n"), _s.exit(2))  # unsafe-guard
import os, sys, json; sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib import *
import catalogue_doc as D
A = Art("en1996")
M = A.schema + "/⚖️masonry"
p = M + "/🛡️conformance-and-integrity/🦀️.rs"
s = rd(p)
a = "/// 🎯️ The evaluation restricted to `scope` (global integrity and verdict, or one wall row)"
assert s.count(a) == 1
s = s.replace(a, '''/// 📇️ The design cases of the document restricted to `scope` (one case row): the duplicate-id check of a repeated id runs on its first row, then the catalogue verification through the shared design-case bridge.
pub fn evaluate_design_cases(annex: AnnexChoice, cases: &[semio_s_artifact_norm_en1996_model::CaseRow], scope: Scope<'_>) -> CheckReport {
    let mut report = CheckReport::default();
    for (index, case) in cases.iter().enumerate().filter(|(index, _)| scope.admits("cases", *index)) {
        if cases.iter().position(|other| other.id == case.id) == Some(index) {
            let same: Vec<String> = cases.iter().filter(|other| other.id == case.id).map(|other| other.id.clone()).collect();
            push_duplicate_ids(&mut report, annex, "cases", "design case", "Bemessungsfall", &same, &|id| format!("cases[id={id}].id"));
        }
        report.extend(evaluate_design_case(&semio_s_plugin_norm_compliance::design_case::DesignCase::from(case), annex));
    }
    report
}

''' + a, 1)
wr(p, s)
p = M + "/🧮️units/🦀️.rs"
s = rd(p)
s = s.replace("use super::{evaluate_scoped, Scope};", "use super::{evaluate_design_cases, evaluate_scoped, Scope};", 1)
a = "        rows.finish()"
assert s.count(a) == 1
s = s.replace(a, '''        for (index, case) in d.cases.iter().enumerate() {
            rows = rows.row("cases", "cases", index, &case.id, CostTier::T0, &[], move |d: &En1996Snapshot| evaluate_design_cases(d.annex, &d.cases, Scope::Row("cases", index)).checks);
        }
        rows.finish()''', 1)
s = s.replace("one unit per wall, in exactly the order of `evaluate_building`", "one unit per wall and one per design case, in exactly the order of `evaluate_building` followed by `evaluate_design_cases`", 1)
wr(p, s)
p = A.schema + "/💡️inferences/🦀️.rs"
sub(p, '''    semio_s_artifact_norm_en1996_schema::checks::masonry::evaluate_building(
        document.annex,
        document.masonry_class,
        document.design_situation,
        document.storeys,
        &document.walls,
    )
}''', '''    let mut report = semio_s_artifact_norm_en1996_schema::checks::masonry::evaluate_building(
        document.annex,
        document.masonry_class,
        document.design_situation,
        document.storeys,
        &document.walls,
    );
    report.extend(semio_s_artifact_norm_en1996_schema::checks::masonry::evaluate_design_cases(document.annex, &document.cases, semio_s_plugin_norm_compliance::incremental_evaluation::Scope::All).checks);
    report
}''')
# field meta
p = A.any + "/✏️editor/🏷️field-meta/🦀️.rs"
sub(p, "use semio_s_plugin_norm_app_surface::{lookup_norm_field_meta,", "use semio_s_plugin_norm_app_surface::{case_input_field_meta, lookup_norm_field_meta,")
sub(p, '''    ("placements[].yaw", FieldSpec::quantity("Placement yaw about Z", "Platzierung Gierwinkel um Z", UnitId::Radian)),
];''', '''    ("placements[].yaw", FieldSpec::quantity("Placement yaw about Z", "Platzierung Gierwinkel um Z", UnitId::Radian)),
    ("cases", FieldSpec::new("Design cases", "Bemessungsfälle")),
    ("cases[].id", FieldSpec::new("Case id", "Fall-Id")),
    ("cases[].labelEn", FieldSpec::new("Case label (English)", "Fallbezeichnung (Englisch)")),
    ("cases[].labelDe", FieldSpec::new("Case label (German)", "Fallbezeichnung (Deutsch)")),
    ("cases[].entryId", FieldSpec::new("Catalogue entry", "Katalogeintrag")),
    ("cases[].numbers", FieldSpec::new("Case numbers", "Fallzahlen")),
    ("cases[].numbers[].id", FieldSpec::new("Number id", "Zahlen-Id")),
    ("cases[].numbers[].value", FieldSpec::new("Number value", "Zahlenwert")),
    ("cases[].texts", FieldSpec::new("Case texts", "Falltexte")),
    ("cases[].texts[].id", FieldSpec::new("Text id", "Text-Id")),
    ("cases[].texts[].value", FieldSpec::new("Text value", "Textwert")),
];''')
sub(p, '''/// 🏷️ Exact / `[]`-wildcard / longest-prefix metadata for the EN 1996 masonry subject.
pub fn en1996_field_meta(path: &str) -> Option<FieldSpec> {
    lookup_norm_field_meta(TABLE, path)
}''', '''/// 🏷️ Exact / `[]`-wildcard / longest-prefix metadata for the EN 1996 masonry subject; the inputs of a design case (`cases[id=…].numbers[id=KEY].value`) take their unit, label, bounds and choices from the typed case-input vocabulary.
pub fn en1996_field_meta(path: &str) -> Option<FieldSpec> {
    case_input_field_meta(semio_s_artifact_norm_en1996_schema::checks::masonry::case_inputs::VOCABULARY, path).or_else(|| lookup_norm_field_meta(TABLE, path))
}''')
# demo cases
c = M + "/📇️catalogue/🗂️cases/🦀️.rs"
s = rd(c)
wr(c, s.rstrip("\n") + '''

/// 📚️ The demo document of the catalogue: one seeded case per catalogue entry, so that every entry is reachable from a document.
pub fn catalogue_demo_cases() -> Vec<design_case::CaseRow> {
    En1996Cases::entries().iter().map(|entry| design_case::CaseRow::from(&design_case::seed::<En1996Cases>(entry, &demo_case_id(entry.id)))).collect()
}

/// 🆔 The id of the demo case of one catalogue entry: the entry id without characters a value path cannot address.
pub fn demo_case_id(entry_id: &str) -> String {
    format!("case-{}", entry_id.to_lowercase().chars().map(|c| if c.is_ascii_alphanumeric() { c } else { '-' }).collect::<String>())
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
''')
wr(M + "/📇️catalogue/🗂️cases/🧪️tests/🔬️unit/🦀️.rs", '''use super::*;

#[test]
fn every_catalogue_entry_evaluates_from_the_demo_document() {
    let rows = catalogue_demo_cases();
    assert_eq!(rows.len(), En1996Cases::entries().len());
    for row in &rows {
        let results = evaluate_design_case(&design_case::DesignCase::from(row), AnnexChoice::De);
        assert!(!results.is_empty(), "entry {} yields no result", row.entry_id);
        let prefix = format!("cases[id={}]", row.id);
        assert!(results.iter().all(|result| result.subject.path.starts_with(&prefix)), "entry {} is not addressed to its case row", row.entry_id);
    }
}

#[test]
fn demo_case_ids_are_unique_and_addressable() {
    let rows = catalogue_demo_cases();
    let ids: std::collections::BTreeSet<&str> = rows.iter().map(|row| row.id.as_str()).collect();
    assert_eq!(ids.len(), rows.len());
    assert!(ids.iter().all(|id| !id.contains(['.', ']', '='])));
}
''')
# catalogue document fixture
import re
text = "".join(rd(f) for f in __import__("glob").glob(M + "/📇️catalogue/**/🦀️.rs", recursive=True) if "🧪️tests" not in f and "🗂️cases" not in f)
ents = D.entries(text)
vocab = json.loads(rd(M + "/📇️catalogue/📚️inputs/🔣️.json"))
base = json.loads(rd(A.fixd + "/↔️change-slab-span/✅apply/🔣️.before.json"))
base["cases"] = D.seed_cases(vocab, ents, D.demo_id)
wr(A.any + "/🧪️tests/🧫️fixtures/📇️catalogue/📚️demo-document/🔣️.json", json.dumps(base, indent=2, ensure_ascii=False) + "\n")
print(len(ents), len(LOG))
