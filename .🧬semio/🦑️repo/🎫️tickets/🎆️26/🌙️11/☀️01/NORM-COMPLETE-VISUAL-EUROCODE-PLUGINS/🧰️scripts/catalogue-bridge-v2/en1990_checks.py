import os, sys; sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib import *
A = Art("en1990")
I = A.schema + "/💡️inferences"
p = I + "/📋️compliance-evaluation/🦀️.rs"
sub(p, '''        push_test_series(report, document, series);
    }
}''', '''        push_test_series(report, document, series);
    }
    for (_, case) in document.cases.iter().enumerate().filter(|(index, _)| scope.admits("cases", *index)) {
        report.extend(catalogue::evaluate_design_case(&semio_s_plugin_norm_compliance::design_case::DesignCase::from(case), document.annex));
    }
}''')
sub(p, "/// 📊️ The reliability analyses and test series of the document", "/// 📊️ The reliability analyses, test series and design cases of the document")
p = I + "/⛓️referential-integrity/🦀️.rs"
sub(p, '''    push_duplicate_ids(report, document, "seismics", &document.seismics.iter().map(|a| a.id.clone()).collect::<Vec<_>>(), &|id| format!("seismics[id={id}].id"));\n''', '''    push_duplicate_ids(report, document, "seismics", &document.seismics.iter().map(|a| a.id.clone()).collect::<Vec<_>>(), &|id| format!("seismics[id={id}].id"));
    push_duplicate_ids(report, document, "cases", &document.cases.iter().map(|a| a.id.clone()).collect::<Vec<_>>(), &|id| format!("cases[id={id}].id"));\n''')
p = I + "/🧮️units/🦀️.rs"
sub(p, '&["members", "reliabilityAnalyses", "testSeries"]', '&["members", "reliabilityAnalyses", "testSeries", "cases"]')
sub(p, "        rows.finish()", '''        for (index, case) in d.cases.iter().enumerate() {
            rows = rows.row("cases", "cases", index, &case.id, CostTier::T0, &[], move |d: &En1990Snapshot| evaluate_scoped(d, Scope::Row("cases", index)).checks);
        }
        rows.finish()''')
print(len(LOG))
