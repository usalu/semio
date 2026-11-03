import os, sys; sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib import *
A = Art("en1997")
C = A.schema + "/💡️inferences/⚖️checks"
p = C + "/🪨️layer-and-duplicate-id-helpers/🦀️.rs"
a = '''        &|id| format!("placements[id={id}].id"),
    );
}
'''
sub(p, a, '''        &|id| format!("placements[id={id}].id"),
    );
    push_duplicate_ids(
        report,
        annex,
        "cases",
        "design case",
        "Bemessungsfall",
        &doc.cases.iter().map(|x| x.id.clone()).collect::<Vec<_>>(),
        &|id| format!("cases[id={id}].id"),
    );
    for case in &doc.cases {
        push_duplicate_ids(report, annex, "caseNumbers", "case number input", "Zahleneingabe des Falls", &case.numbers.iter().map(|x| x.id.clone()).collect::<Vec<_>>(), &|id| format!("cases[id={}].numbers[id={id}].id", case.id));
        push_duplicate_ids(report, annex, "caseTexts", "case text input", "Texteingabe des Falls", &case.texts.iter().map(|x| x.id.clone()).collect::<Vec<_>>(), &|id| format!("cases[id={}].texts[id={id}].id", case.id));
    }
}
''')
p = C + "/🏗️project-checks/🦀️.rs"
sub(p, "    geo_entities::check_layer_waters(doc, &mut report, scope);\n", '''    geo_entities::check_layer_waters(doc, &mut report, scope);
    for (_, case) in doc.cases.iter().enumerate().filter(|(index, _)| scope.admits("cases", *index)) {
        report.extend(crate::catalogue::evaluate_design_case(&semio_s_plugin_norm_compliance::design_case::DesignCase::from(case), annex));
    }
''')
p = C + "/🧮️units/🦀️.rs"
sub(p, '"layerWaters"];\nconst GROUND', '"layerWaters", "cases"];\nconst GROUND')
sub(p, "const ROW_COLLECTIONS: [&str; 11]", "const ROW_COLLECTIONS: [&str; 12]")
sub(p, '        rows = group(rows, &d.layer_waters, "layerWaters", |e| e.id.as_str(), CostTier::T0, &["layers"]);\n', '        rows = group(rows, &d.layer_waters, "layerWaters", |e| e.id.as_str(), CostTier::T0, &["layers"]);\n        rows = group(rows, &d.cases, "cases", |e| e.id.as_str(), CostTier::T0, &[]);\n')
p = A.schema + "/💡️inferences/🧾outline/🦀️.rs"
sub(p, '    "placements",\n];', '    "placements",\n    "cases",\n];')
sub(p, "            + snapshot.placements.len()) as u32;", "            + snapshot.placements.len()\n            + snapshot.cases.len()) as u32;")
print(LOG)
