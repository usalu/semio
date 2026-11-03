"""Task 1 layer D: inference, scene, editor and spec-trace code on the typed reliability class (K_FI only from the registry)."""
from lib import *
import glob
NUM = re.compile(r"\b((?:document|d)\.reliability_class)\b(?!\.number)")
def number(t):
    return NUM.sub(r"\1.number()", t)

for rel in ("/🧬️schema/💡️inferences/🎚️factor-overrides/🦀️.rs", "/🧬️schema/💡️inferences/📊️annex-analyses/🦀️.rs", "/🧬️schema/💡️inferences/🌍️working-life-and-bridge-annexes/🦀️.rs", "/🧬️schema/💡️inferences/📐️annex-criteria/🦀️.rs", "/🧬️schema/💡️inferences/📋️compliance-evaluation/🦀️.rs", "/🧬️schema/💡️inferences/🧰️report-helpers/🦀️.rs", "/🎬️scene/⚓️actions/🦀️.rs", "/🎬️scene/📉️envelope-and-targets/🦀️.rs", "/🎬️scene/📈️diagrams/🦀️.rs", "/🎬️scene/🧪️tests/🔬️unit/🦀️.rs"):
    edit(SUB + rel, number)

PFS = INF + "/🧮️partial-factor-sets/🦀️.rs"
def pfs(t):
    t = number(t)
    if "push_k_fi(" not in t:
        return t
    t = sub(t, r"pub\(super\) fn push_k_fi\(report: &mut CheckReport, document: &En1990Snapshot\) \{.*?\n\}\n\n", "", flags=re.S)
    t = rep(t, '''    for (field, value) in [("reliabilityClass", document.reliability_class.number()), ("consequenceClass", document.consequence_class)] {
        if !matches!(value, 1..=3) {
            invalid_input(report, document, format!("en1990.input.{field}"), field.into(), (if field == "reliabilityClass" { "Reliability class" } else { "Consequence class" }, if field == "reliabilityClass" { "Zuverlässigkeitsklasse" } else { "Schadensfolgeklasse" }), &value.to_string(), vec!["1".into(), "2".into(), "3".into()]);
        }
    }
''', '''    if !matches!(document.consequence_class, 1..=3) {
        invalid_input(report, document, "en1990.input.consequenceClass".into(), "consequenceClass".into(), ("Consequence class", "Schadensfolgeklasse"), &document.consequence_class.to_string(), vec!["1".into(), "2".into(), "3".into()]);
    }
''')
    t = rep(t, "    if !matches!(document.consequence_class, 1..=3) || !matches!(document.reliability_class.number(), 1..=3) {\n        return;\n    }\n    let equal = document.consequence_class == document.reliability_class.number();", "    if !matches!(document.consequence_class, 1..=3) {\n        return;\n    }\n    let equal = document.consequence_class == document.reliability_class.number();")
    return t
edit(PFS, pfs)
edit(INF + "/📋️compliance-evaluation/🦀️.rs", lambda t: rep(t, "        push_k_fi(&mut report, document);\n", "", guard="@@") if "push_k_fi(&mut" in t else t)

edit(SUB + "/🧭️spec-trace/🦀️.rs", lambda t: rep(t, '"en1990.annex-b.rc-vs-cc", "en1990.annex-b.kfi"]', '"en1990.annex-b.rc-vs-cc", "en1990.annex-b.kfi-applied"]') if 'annex-b.kfi"]' in t else t)
edit(SUB + "/🧭️spec-trace/🔣️.json", lambda t: rep(t, '"en1990.annex-b.kfi"\n', '"en1990.annex-b.kfi-applied"\n') if '"en1990.annex-b.kfi"\n' in t else t)

def actions(t):
    t = rep(t, "use super::checks::{psi_entry_of, psi_is_ordered, psi_lookup, resolve_psi_category, target_reliability_index_for};", "use super::checks::{k_fi_for_rc, psi_entry_of, psi_is_ordered, psi_lookup, resolve_psi_category, target_reliability_index_for};", guard="k_fi_for_rc, psi_entry_of")
    return rep(t, "d.k_fi_declared", "k_fi_for_rc(d.reliability_class.number()).unwrap_or(1.0)", count=2, guard="@@") if "d.k_fi_declared" in t else t
edit(SUB + "/🎬️scene/⚓️actions/🦀️.rs", actions)

def scene_tests(t):
    return rep(t, "    mismatch.reliability_class = 3;", "    mismatch.reliability_class = model::ReliabilityClass::RC3;", guard="model::ReliabilityClass::RC3")
edit(SUB + "/🎬️scene/🧪️tests/🔬️unit/🦀️.rs", scene_tests)

FM = SUB + "/✏️editor/🏷️field-meta"
def fm(t):
    t = rep(t, 'const RC: &[NormFieldChoice] = &[choice("1", "RC1", "RC1"), choice("2", "RC2", "RC2"), choice("3", "RC3", "RC3")];', 'const RC: &[NormFieldChoice] = &[choice("RC1", "RC1 (K_FI 0.9)", "RC1 (K_FI 0,9)"), choice("RC2", "RC2 (K_FI 1.0)", "RC2 (K_FI 1,0)"), choice("RC3", "RC3 (K_FI 1.1)", "RC3 (K_FI 1,1)")];', guard='choice("RC1"')
    t = sub(t, r'const K_FI: &\[NormFieldChoice\] = &\[\n(?:    choice\([^\n]*\n){3}\];\n', "") if "const K_FI" in t else t
    t = rep(t, '    ("kFiDeclared", FieldSpec::choice("Declared K_FI (Table B3)", "Deklariertes K_FI (Tabelle B3)", K_FI)),\n', "", guard="@@") if "kFiDeclared" in t else t
    return t
edit(FM + "/🦀️.rs", fm)
edit(FM + "/🧪️tests/🔬️unit/🦀️.rs", lambda t: t.replace('        "kFiDeclared",\n', "") if "kFiDeclared" in t else t)
