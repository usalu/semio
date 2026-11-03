import os, sys; sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib import *
A = Art("en1990")
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
sub(p, '''/// 🏷️ Exact / `[]`-wildcard / longest-prefix metadata for the EN 1990 basis-of-design subject.
pub fn en1990_field_meta(path: &str) -> Option<FieldSpec> {
    lookup_norm_field_meta(TABLE, path)
}''', '''/// 🏷️ Exact / `[]`-wildcard / longest-prefix metadata for the EN 1990 basis-of-design subject; the inputs of a design case (`cases[id=…].numbers[id=KEY].value`) take their unit, label, bounds and choices from the typed case-input vocabulary.
pub fn en1990_field_meta(path: &str) -> Option<FieldSpec> {
    case_input_field_meta(semio_s_artifact_norm_en1990_checks::catalogue::case_inputs::VOCABULARY, path).or_else(|| lookup_norm_field_meta(TABLE, path))
}''')
# catalogue demo cases + test
c = A.any + "/🧬️schema/💡️inferences/⚖️checks/📇️catalogue/🗂️cases/🦀️.rs"
s = rd(c)
s = s.rstrip("\n") + '''

/// 📚️ The demo document of the catalogue: one seeded case per catalogue entry, so that every entry is reachable from a document.
pub fn catalogue_demo_cases() -> Vec<design_case::CaseRow> {
    En1990Cases::entries().iter().map(|entry| design_case::CaseRow::from(&design_case::seed::<En1990Cases>(entry, &demo_case_id(entry.id)))).collect()
}

/// 🆔 The id of the demo case of one catalogue entry: the entry id without characters a value path cannot address.
pub fn demo_case_id(entry_id: &str) -> String {
    format!("case-{}", entry_id.to_lowercase().chars().map(|c| if c.is_ascii_alphanumeric() { c } else { '-' }).collect::<String>())
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
'''
wr(c, s)
wr(A.any + "/🧬️schema/💡️inferences/⚖️checks/📇️catalogue/🗂️cases/🧪️tests/🔬️unit/🦀️.rs", '''use super::*;

#[test]
fn every_catalogue_entry_evaluates_from_the_demo_document() {
    let rows = catalogue_demo_cases();
    assert_eq!(rows.len(), En1990Cases::entries().len());
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
print(len(LOG))
