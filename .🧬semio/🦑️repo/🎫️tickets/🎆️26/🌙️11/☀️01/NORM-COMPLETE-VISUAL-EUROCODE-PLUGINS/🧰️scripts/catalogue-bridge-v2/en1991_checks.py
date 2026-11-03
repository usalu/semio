import os, sys, json, glob; sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import lib2
from lib import *
import catalogue_doc as D
A = Art("en1991")
I = A.schema + "/💡️inferences"
p = I + "/🦀️.rs"
sub(p, "    collection_checks::check_collection_rows(document, annex, &mut report, scope);\n    wave3_checks::check_rows(document, annex, &mut report, scope);\n    report\n}", '''    collection_checks::check_collection_rows(document, annex, &mut report, scope);
    wave3_checks::check_rows(document, annex, &mut report, scope);
    for (_, case) in document.cases.iter().enumerate().filter(|(index, _)| scope.admits("cases", *index)) {
        report.extend(semio_s_artifact_norm_en1991_checks::catalogue::evaluate_design_case(&semio_s_plugin_norm_compliance::design_case::DesignCase::from(case), annex));
    }
    report
}''')
sub(p, '''    push_duplicate_ids(report, document, "thermalSections", &document.thermal_sections.iter().map(|r| r.id.clone()).collect::<Vec<_>>(), &|id| format!("thermalSections[id={id}].id"));\n''', '''    push_duplicate_ids(report, document, "thermalSections", &document.thermal_sections.iter().map(|r| r.id.clone()).collect::<Vec<_>>(), &|id| format!("thermalSections[id={id}].id"));
    push_duplicate_ids(report, document, "cases", &document.cases.iter().map(|r| r.id.clone()).collect::<Vec<_>>(), &|id| format!("cases[id={id}].id"));\n''')
p = I + "/🧮️units/🦀️.rs"
s = rd(p)
s = s.replace("const ROW_COLLECTIONS: [&str; 14] = [", "const ROW_COLLECTIONS: [&str; 15] = [", 1)
i = s.index("const ROW_COLLECTIONS")
j = s.index("];", i)
s = s[:j] + ', "cases"' + s[j:]
a = '        group(rows, &d.thermal_sections, "thermalSections", |e| e.id.as_str(), &[]).finish()'
assert a in s
s = s.replace(a, '        rows = group(rows, &d.thermal_sections, "thermalSections", |e| e.id.as_str(), &[]);\n        group(rows, &d.cases, "cases", |e| e.id.as_str(), &[]).finish()', 1)
wr(p, s)
p = I + "/🧾outline/🦀️.rs"
sub(p, '    "thermal_sections",\n];', '    "thermal_sections",\n    "cases",\n];')
sub(p, "            + snapshot.thermal_sections.len()) as u32;", "            + snapshot.thermal_sections.len()\n            + snapshot.cases.len()) as u32;")
# field meta
fm = glob.glob(A.any + "/✏️editor/🏷️field-meta/🦀️.rs")[0]
s = rd(fm)
print([l for l in s.splitlines() if "lookup_norm_field_meta" in l][:3])
