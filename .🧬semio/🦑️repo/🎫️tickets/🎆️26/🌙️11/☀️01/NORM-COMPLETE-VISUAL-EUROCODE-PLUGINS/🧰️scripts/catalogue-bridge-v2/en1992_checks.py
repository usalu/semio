import os as _o, sys as _s; _o.environ.get("TH_FORCE") or (_s.stderr.write("unsafe against the current tree (destructive or not idempotent), refusing to run (TH_FORCE=1 forces)\n"), _s.exit(2))  # unsafe-guard
import os, sys, json, glob; sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import lib2
from lib import *
import catalogue_doc as D
A = Art("en1992")
I = A.schema + "/💡️inferences"
p = I + "/🦀️.rs"
sub(p, '''        push_duplicate_ids(&mut report, document, "placements", &document.placements.iter().map(|a| a.id.clone()).collect::<Vec<_>>(), &|id| format!("placements[id={id}].id"));\n''', '''        push_duplicate_ids(&mut report, document, "placements", &document.placements.iter().map(|a| a.id.clone()).collect::<Vec<_>>(), &|id| format!("placements[id={id}].id"));
        push_duplicate_ids(&mut report, document, "cases", &document.cases.iter().map(|a| a.id.clone()).collect::<Vec<_>>(), &|id| format!("cases[id={id}].id"));\n''')
sub(p, '''        report.extend(evaluate_anchor(document, anchor));
    }
    report
}''', '''        report.extend(evaluate_anchor(document, anchor));
    }
    for (index, case) in document.cases.iter().enumerate().filter(|(index, _)| scope.admits("cases", *index)) {
        report.extend(semio_s_artifact_norm_en1992_checks::evaluate_design_case(&semio_s_plugin_norm_compliance::design_case::DesignCase::from(case), document.annex));
    }
    report
}''')
p = I + "/🧮️units/🦀️.rs"
sub(p, "        rows.finish()", '''        for (index, case) in d.cases.iter().enumerate() {
            rows = rows.row("cases", "cases", index, &case.id, CostTier::T0, &[], move |d: &En1992Snapshot| evaluate_scoped(d, Scope::Row("cases", index)).checks);
        }
        rows.finish()''')
sub(p, "one unit per member and anchor,", "one unit per member, anchor and design case,")
p = I + "/🧾outline/🦀️.rs"
sub(p, '    "placements",\n];', '    "placements",\n    "cases",\n];')
sub(p, "snapshot.anchors.len() + snapshot.placements.len())", "snapshot.anchors.len() + snapshot.placements.len() + snapshot.cases.len())")
# field meta
fm = A.any + "/✏️editor/🏷️field-meta/🦀️.rs"
print([l[:150] for l in rd(fm).splitlines() if "pub fn" in l or "^use" in l or l.startswith("use ")])
