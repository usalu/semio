"""Appends the demo-values tests (every demo case evaluates; the committed demo document equals the seeded cases; vector demo values reach the evaluator input) to the case unit tests of each code (idempotent). Run with python3 -I wire_tests.py."""
import sys
sys.path.insert(0, "."); from dv_lib import *
CASES = {"en1990": "🧬️schema/💡️inferences/⚖️checks/📇️catalogue/🗂️cases", "en1991": "🧬️schema/💡️inferences/⚖️checks/📇️catalogue/🗂️cases", "en1995": "🧬️schema/⚖️kernel/📇️catalogue/🗂️cases", "en1996": "🧬️schema/📇️masonry-catalogue/🗂️cases", "en1997": "🧬️schema/💡️inferences/⚖️checks/📇️catalogue/🗂️cases"}
TYPE = {"en1990": "En1990Cases", "en1991": "En1991Cases", "en1995": "En1995Cases", "en1996": "En1996Cases", "en1997": "En1997Cases"}
UPS = {"en1990": 7, "en1991": 7, "en1995": 6, "en1996": 5, "en1997": 7}
VEC = {"en1991": ("E12.10", "times_min", 5), "en1996": ("M-21", "l_m", 2)}
EVAL = '''
#[test]
fn every_demo_case_evaluates_its_entry_instead_of_reporting_missing_or_invalid_inputs() {
    let silent: Vec<String> = catalogue_demo_cases()
        .iter()
        .filter(|row| {
            let results = evaluate_design_case(&design_case::DesignCase::from(*row), AnnexChoice::De);
            results.iter().all(|result| result.status == semio_s_plugin_norm_compliance::CheckStatus::NotApplicable) || results.iter().any(|result| result.id.contains(".input."))
        })
        .map(|row| row.entry_id.clone())
        .collect();
    assert!(silent.is_empty(), "entries whose demo case only reports not-applicable or invalid inputs: {silent:?}");
}
'''
DRIFT = '''
#[test]
fn the_committed_demo_document_equals_the_seeded_cases_and_covers_the_registry() {
    let fixture = include_str!("%s🧪️tests/🧫️fixtures/📇️catalogue/📚️demo-document/🔣️.json");
    let rows = catalogue_demo_cases();
    assert_eq!(fixture.matches("\\"entryId\\":").count(), rows.len(), "the demo document has a different number of cases than the seeded demo");
    for row in &rows {
        assert!(fixture.contains(&format!("\\"id\\": \\"{}\\"", row.id)) && fixture.contains(&format!("\\"entryId\\": \\"{}\\"", row.entry_id)), "{} is missing from the demo document", row.id);
    }
    let rows_with_values: usize = rows.iter().map(|row| row.numbers.len() + row.texts.len()).sum();
    assert_eq!(fixture.matches("\\"value\\":").count(), rows_with_values, "the demo document values drifted from the seeded demo");
    for entry in %s::entries() {
        assert!(rows.iter().any(|row| row.entry_id == entry.id), "{} has no case in the demo document", entry.id);
    }
}
'''
VECT = '''
#[test]
fn the_vector_demo_values_reach_the_evaluator_input() {
    let rows = catalogue_demo_cases();
    let row = rows.iter().find(|row| row.entry_id == "%s").expect("%s demo case");
    let input = case_input(AnnexChoice::De, &design_case::DesignCase::from(row));
    assert_eq!(input.vec.get("%s").map(Vec::len), Some(%d));
}
'''
for code, rel in CASES.items():
    d = art(code) / rel
    tp = d / "🧪️tests/🔬️unit/🦀️.rs"
    if code == "en1997":
        tp = art(code) / "🧬️schema/💡️inferences/⚖️checks/📇️catalogue/🧪️tests/📚️demo-document/🦀️.rs"
    if code == "en1995" and not tp.exists():
        tp.parent.mkdir(parents=True, exist_ok=True)
        tp.write_text("use super::*;\n\n#[test]\nfn demo_case_ids_are_unique_and_addressable() {\n    let rows = catalogue_demo_cases();\n    let ids: std::collections::BTreeSet<&str> = rows.iter().map(|row| row.id.as_str()).collect();\n    assert_eq!(ids.len(), rows.len());\n    assert!(ids.iter().all(|id| !id.contains(['.', ']', '='])));\n}\n", encoding="utf-8")
        cp = d / "🦀️.rs"
        cp.write_text(cp.read_text(encoding="utf-8").rstrip("\n") + '\n\n#[cfg(test)]\n#[path = "🧪️tests/🔬️unit/🦀️.rs"]\nmod tests;\n', encoding="utf-8")
    t = tp.read_text(encoding="utf-8")
    add = ""
    if "instead_of_reporting_missing_or_invalid_inputs" not in t: add += EVAL
    if "the_committed_demo_document_equals_the_seeded_cases" not in t: add += DRIFT % ("../" * UPS[code], TYPE[code])
    if code in VEC and "the_vector_demo_values_reach_the_evaluator_input" not in t: add += VECT % (VEC[code][0], VEC[code][0], VEC[code][1], VEC[code][2])
    if add:
        tp.write_text(t.rstrip("\n") + "\n" + add, encoding="utf-8")
    print(code, "tests added" if add else "unchanged", tp.name)
