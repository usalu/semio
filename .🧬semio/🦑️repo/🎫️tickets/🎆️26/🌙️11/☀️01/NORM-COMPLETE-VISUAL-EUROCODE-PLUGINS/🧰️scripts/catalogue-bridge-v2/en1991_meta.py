import os, sys, json; sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib import *
import catalogue_doc as D
A = Art("en1991")
p = A.any + "/✏️editor/🏷️field-meta/🦀️.rs"
sub(p, "use semio_s_plugin_norm_app_surface::{NormFieldChoice, FieldSpec, UnitId};", "use semio_s_plugin_norm_app_surface::{case_input_field_meta, NormFieldChoice, FieldSpec, UnitId};")
sub(p, '''/// 🏷️ Exact, `[]`-templated (never parent-collection) or top-level leaf-segment metadata for the EN 1991 design-load subject.
pub fn en1991_field_meta(path: &str) -> Option<FieldSpec> {
''', '''/// 🏷️ Exact, `[]`-templated (never parent-collection) or top-level leaf-segment metadata for the EN 1991 design-load subject; the inputs of a design case (`cases[id=…].numbers[id=KEY].value`) take their unit, label, bounds and choices from the typed case-input vocabulary.
pub fn en1991_field_meta(path: &str) -> Option<FieldSpec> {
    if let Some(m) = case_input_field_meta(semio_s_artifact_norm_en1991_checks::catalogue::case_inputs::VOCABULARY, path) {
        return Some(m);
    }
''')
sub(p, '        "internalExplosions" => FieldSpec::new("Internal explosions", "Innere Explosionen"),\n', '''        "cases" => FieldSpec::new("Design cases", "Bemessungsfälle"),
        "cases[].id" => FieldSpec::new("Case id", "Fall-Id"),
        "cases[].labelEn" => FieldSpec::new("Case label (English)", "Fallbezeichnung (Englisch)"),
        "cases[].labelDe" => FieldSpec::new("Case label (German)", "Fallbezeichnung (Deutsch)"),
        "cases[].entryId" => FieldSpec::new("Catalogue entry", "Katalogeintrag"),
        "cases[].numbers" => FieldSpec::new("Case numbers", "Fallzahlen"),
        "cases[].numbers[].id" => FieldSpec::new("Number id", "Zahlen-Id"),
        "cases[].numbers[].value" => FieldSpec::new("Number value", "Zahlenwert"),
        "cases[].texts" => FieldSpec::new("Case texts", "Falltexte"),
        "cases[].texts[].id" => FieldSpec::new("Text id", "Text-Id"),
        "cases[].texts[].value" => FieldSpec::new("Text value", "Textwert"),
        "internalExplosions" => FieldSpec::new("Internal explosions", "Innere Explosionen"),
''')
C = A.schema + "/💡️inferences/⚖️checks/📇️catalogue"
c = C + "/🗂️cases/🦀️.rs"
s = rd(c)
wr(c, s.rstrip("\n") + '''

/// 📚️ The demo document of the catalogue: one seeded case per catalogue entry, so that every entry is reachable from a document.
pub fn catalogue_demo_cases() -> Vec<design_case::CaseRow> {
    En1991Cases::entries().iter().map(|entry| design_case::CaseRow::from(&design_case::seed::<En1991Cases>(entry, &demo_case_id(entry.id)))).collect()
}

/// 🆔 The id of the demo case of one catalogue entry: the entry id without characters a value path cannot address.
pub fn demo_case_id(entry_id: &str) -> String {
    format!("case-{}", entry_id.to_lowercase().chars().map(|c| if c.is_ascii_alphanumeric() { c } else { '-' }).collect::<String>())
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
''')
wr(C + "/🗂️cases/🧪️tests/🔬️unit/🦀️.rs", rd(A.schema.replace("en1991", "en1991") and (ARTS + "/⚖️en1990" + ANY + "/🧬️schema/💡️inferences/⚖️checks/📇️catalogue/🗂️cases/🧪️tests/🔬️unit/🦀️.rs")).replace("En1990", "En1991"))
ents = D.entries("".join(rd(f) for f in __import__("glob").glob(C + "/**/🦀️.rs", recursive=True) if "🧪️tests" not in f and "🗂️cases" not in f))
vocab = json.loads(rd(C + "/📚️inputs/🔣️.json"))
base = json.loads(rd(A.fixd + "/❄️change-altitude/✅apply/🔣️.before.json"))
base["cases"] = D.seed_cases(vocab, ents, D.demo_id)
wr(A.any + "/🧪️tests/🧫️fixtures/📇️catalogue/📚️demo-document/🔣️.json", json.dumps(base, indent=2, ensure_ascii=False) + "\n")
print(len(ents), len(LOG))
