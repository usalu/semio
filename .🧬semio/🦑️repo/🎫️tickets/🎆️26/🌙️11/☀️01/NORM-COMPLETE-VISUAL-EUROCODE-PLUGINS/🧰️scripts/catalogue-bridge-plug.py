#!/usr/bin/env python3
"""Plugs a catalogue into the shared design-case bridge: generates `📚️inputs` (vocabulary json + rust) and `🗂️cases` (CaseCatalogue impl) next to the artifact registry and mounts them. Usage: python -I catalogue-bridge-plug.py <code> [--apply]"""
import pathlib, re, subprocess, sys
HERE = pathlib.Path(__file__).resolve().parent
ART = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts")
S = "🏅️standards/🔖️1/🪆️subsets/✳️any/"
PROFILES = {
    "en1997": {"dir": "🌍️en1997", "catalogue": S + "🧬️schema/💡️inferences/⚖️checks/📇️catalogue/", "type": "En1997Cases", "part": "part", "vec": False, "artifact": "s.norm.en1997"},
    "en1990": {"dir": "⚖️en1990", "catalogue": S + "🧬️schema/💡️inferences/⚖️checks/📇️catalogue/", "type": "En1990Cases", "part": "part", "vec": False, "artifact": "s.norm.en1990"},
    "en1996": {"dir": "🪨️en1996", "catalogue": S + "🧬️schema/⚖️masonry/📇️catalogue/", "type": "En1996Cases", "part": "part", "vec": True, "paths": True, "artifact": "s.norm.en1996"},
    "en1992": {"dir": "🏛️en1992", "catalogue": S + "🧬️schema/💡️inferences/⚖️checks/🗄️registry/", "type": "En1992Cases", "part": "part", "vec": True, "vecfield": "list", "artifact": "s.norm.en1992"},
    "en1991": {"dir": "🏋️en1991", "catalogue": S + "🧬️schema/💡️inferences/⚖️checks/📇️catalogue/", "type": "En1991Cases", "part": "standard", "vec": True, "artifact": "s.norm.en1991"},
}
code = sys.argv[1]; P = PROFILES[code]
cat = ART / P["dir"] / P["catalogue"]
tsv = HERE / f"catalogue-bridge-{code}-inputs.tsv"
vec = '''    for spec in <V>.iter().filter(|spec| spec.kind == design_case::InputKind::Vector) {
        if let Some(values) = case.values.vector(spec.key) {
            input.<F>.insert(spec.key.into(), values);
        }
    }
''' if P["vec"] else ""
paths = '''    input.paths.extend(case.values.numbers.keys().map(|key| (key.clone(), design_case::input_path(&case.id, key, true))).chain(case.values.texts.keys().map(|key| (key.clone(), design_case::input_path(&case.id, key, false)))));
''' if P.get("paths") else ""
rs = f'''//! 🗂️ Design cases of the catalogue: the registry as the shared [`CaseCatalogue`] — typed inputs (unit, EN and DE label, choices, bounds) from [`super::case_inputs`], search, validation and sizing remedies from the compliance design-case bridge. See [`super`].

use super::*;
use semio_s_plugin_norm_compliance::design_case::{{self, CaseCatalogue, CaseEntry, DesignCase, InputSpec}};

/// 📚️ The catalogue as a [`CaseCatalogue`].
pub struct {P["type"]};

fn case_entry(e: &'static Entry) -> CaseEntry {{
    CaseEntry {{ id: e.id, part: e.{P["part"]}, clause: e.clause, en: e.en, de: e.de, inputs: e.inputs, geometry: None }}
}}

/// 🧾 The evaluator input of a design case.
pub fn case_input(annex: AnnexChoice, case: &DesignCase) -> Case {{
    let mut input = Case::new(&case.id, annex);
    input.num.extend(case.values.numbers.iter().map(|(key, value)| (key.clone(), *value)));
    input.text.extend(case.values.texts.iter().map(|(key, value)| (key.clone(), value.clone())));
{vec.replace("<V>", "super::case_inputs::VOCABULARY").replace("<F>", P.get("vecfield", "vec"))}{paths}    input
}}

impl CaseCatalogue for {P["type"]} {{
    fn entries() -> Vec<CaseEntry> {{
        registry().into_iter().map(case_entry).collect()
    }}

    fn vocabulary() -> &'static [InputSpec] {{
        super::case_inputs::VOCABULARY
    }}

    fn evaluate(entry: &CaseEntry, annex: AnnexChoice, case: &DesignCase) -> Vec<CheckResult> {{
        evaluate_case(entry.id, &case_input(annex, case))
    }}
}}

/// 🧮 One design case through the shared bridge: input issues with remedies, then the catalogue checks addressed to the case row with a sizing remedy on failure.
pub fn evaluate_design_case(case: &DesignCase, annex: AnnexChoice) -> Vec<CheckResult> {{
    design_case::evaluate_case::<{P["type"]}>(case, annex)
}}
'''
if "--apply" not in sys.argv:
    print(code, "dry", cat.exists(), tsv.exists()); sys.exit(0)
subprocess.run([sys.executable, "-I", str(HERE / "catalogue-bridge-gen-inputs.py"), str(cat / "📚️inputs"), str(tsv), P["artifact"]], check=True)
(cat / "🗂️cases").mkdir(exist_ok=True); (cat / "🗂️cases" / "🦀️.rs").write_text(rs, encoding="utf-8")
m = cat / "🦀️.rs"; s = m.read_text(encoding="utf-8")
if "case_inputs" not in s:
    m.write_text(s.rstrip("\n") + '\n\n#[path = "📚️inputs/🦀️.rs"]\npub mod case_inputs;\n\n#[path = "🗂️cases/🦀️.rs"]\nmod cases;\npub use cases::*;\n', encoding="utf-8")
print("plugged", code)
