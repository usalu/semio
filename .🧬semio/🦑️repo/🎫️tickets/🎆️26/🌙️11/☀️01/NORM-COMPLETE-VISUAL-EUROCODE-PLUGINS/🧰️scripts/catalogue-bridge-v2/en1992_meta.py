import os, sys, json, glob; sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import lib2
from lib import *
import catalogue_doc as D
A = Art("en1992")
p = A.any + "/✏️editor/🏷️field-meta/🦀️.rs"
sub(p, "use semio_s_plugin_norm_app_surface::{NormFieldChoice, FieldSpec, UnitId};", "use semio_s_plugin_norm_app_surface::{case_input_field_meta, NormFieldChoice, FieldSpec, UnitId};")
sub(p, '''    ("placements[].yaw", m("Yaw angle about z", "Drehwinkel um z", Some(UnitId::Radian), None)),
];''', '''    ("placements[].yaw", m("Yaw angle about z", "Drehwinkel um z", Some(UnitId::Radian), None)),
    ("cases", m("Design cases", "Bemessungsfälle", None, None)),
    ("cases[].id", m("Case id", "Fall-Id", None, None)),
    ("cases[].labelEn", m("Case label (English)", "Fallbezeichnung (Englisch)", None, None)),
    ("cases[].labelDe", m("Case label (German)", "Fallbezeichnung (Deutsch)", None, None)),
    ("cases[].entryId", m("Catalogue entry", "Katalogeintrag", None, None)),
    ("cases[].numbers", m("Case numbers", "Fallzahlen", None, None)),
    ("cases[].numbers[].id", m("Number id", "Zahlen-Id", None, None)),
    ("cases[].numbers[].value", m("Number value", "Zahlenwert", None, None)),
    ("cases[].texts", m("Case texts", "Falltexte", None, None)),
    ("cases[].texts[].id", m("Text id", "Text-Id", None, None)),
    ("cases[].texts[].value", m("Text value", "Textwert", None, None)),
];''')
sub(p, '''pub fn en1992_field_meta(path: &str) -> Option<FieldSpec> {
''', '''pub fn en1992_field_meta(path: &str) -> Option<FieldSpec> {
    if let Some(meta) = case_input_field_meta(semio_s_artifact_norm_en1992_checks::catalogue_registry::case_inputs::VOCABULARY, path) {
        return Some(meta);
    }
''')
sub(p, "/// 🏷️ Exact / `[]`-wildcard lookup only — no longest-prefix fallback (every leaf has an explicit label).", "/// 🏷️ Exact / `[]`-wildcard lookup only — no longest-prefix fallback (every leaf has an explicit label); the inputs of a design case (`cases[id=…].numbers[id=KEY].value`) take their unit, label, bounds and choices from the typed case-input vocabulary.")
C = A.schema + "/💡️inferences/⚖️checks/🗄️registry"
c = C + "/🗂️cases/🦀️.rs"
s = rd(c)
wr(c, s.rstrip("\n") + '''

/// 📚️ The demo document of the catalogue: one seeded case per catalogue entry, so that every entry is reachable from a document.
pub fn catalogue_demo_cases() -> Vec<design_case::CaseRow> {
    En1992Cases::entries().iter().map(|entry| design_case::CaseRow::from(&design_case::seed::<En1992Cases>(entry, &demo_case_id(entry.id)))).collect()
}

/// 🆔 The id of the demo case of one catalogue entry: the entry id without characters a value path cannot address.
pub fn demo_case_id(entry_id: &str) -> String {
    format!("case-{}", entry_id.to_lowercase().chars().map(|c| if c.is_ascii_alphanumeric() { c } else { '-' }).collect::<String>())
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
''')
wr(C + "/🗂️cases/🧪️tests/🔬️unit/🦀️.rs", rd(ARTS + "/⚖️en1990" + ANY + "/🧬️schema/💡️inferences/⚖️checks/📇️catalogue/🗂️cases/🧪️tests/🔬️unit/🦀️.rs").replace("En1990", "En1992"))
# fixture document: entry_of(...) form
import re
src = "".join(rd(f) for f in glob.glob(A.schema + "/💡️inferences/⚖️checks/**/🦀️.rs", recursive=True) if "🧪️tests" not in f and "🗂️cases" not in f)
ents = []
for m in re.finditer(r'entry_of\("([^"]+)", "[^"]*", "[^"]*", "((?:[^"\\]|\\.)*)", "((?:[^"\\]|\\.)*)", &\[([^\]]*)\]', src):
    ents.append({"id": m.group(1), "en": json.loads('"%s"' % m.group(2)), "de": json.loads('"%s"' % m.group(3)), "inputs": re.findall(r'"([^"]+)"', m.group(4))})
vocab = json.loads(rd(C + "/📚️inputs/🔣️.json"))
base = json.loads(rd(A.fixd + "/🏋️change-action-category/✅apply/🔣️.before.json"))
base["cases"] = D.seed_cases(vocab, ents, D.demo_id)
wr(A.any + "/🧪️tests/🧫️fixtures/📇️catalogue/📚️demo-document/🔣️.json", json.dumps(base, indent=2, ensure_ascii=False) + "\n")
print(len(ents), len(LOG))
