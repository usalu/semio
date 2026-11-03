import os as _o, sys as _s; _o.environ.get("TH_FORCE") or (_s.stderr.write("unsafe against the current tree (destructive or not idempotent), refusing to run (TH_FORCE=1 forces)\n"), _s.exit(2))  # unsafe-guard
import os, sys; sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib import *
A = Art("en1991")
CASE = '''vec![CaseRow {
            id: "case-e11-04".into(),
            label_en: "Imposed-load reduction of the office floors".into(),
            label_de: "Nutzlastabminderung der Bürogeschosse".into(),
            entry_id: "E11.04".into(),
            numbers: vec![CaseNumber { id: "area".into(), value: 30.0 }, CaseNumber { id: "storeys".into(), value: 4.0 }],
            texts: vec![CaseText { id: "category".into(), value: "A".into() }],
        }]'''
p = A.snap + "/🦀️.rs"
sub(p, "use framework_schema::ArtifactSchema;\n", "pub use semio_s_plugin_norm_compliance::design_case::{CaseNumber, CaseRow, CaseText};\nuse framework_schema::ArtifactSchema;\n")
sub(p, "    pub thermal_sections: Vec<ThermalSection>,\n}", "    pub thermal_sections: Vec<ThermalSection>,\n    #[dsl(table)]\n    #[state(artifact)]\n    pub cases: Vec<CaseRow>,\n}")
sub(p, "thermal_sections: vec![],\n        }\n    }\n}\n//#endregion", "thermal_sections: vec![],\ncases: %s,\n        }\n    }\n}\n//#endregion" % CASE)
p = A.schema + "/🦀️.rs"
sub(p, "    pub thermal_sections: Vec<semio_s_artifact_norm_en1991_model::ThermalSection>,\n}", "    pub thermal_sections: Vec<semio_s_artifact_norm_en1991_model::ThermalSection>,\n    #[state(artifact)]\n    pub cases: Vec<semio_s_artifact_norm_en1991_model::CaseRow>,\n}")
sub(p, "            thermal_sections: self.thermal_sections.clone(),\n", "            thermal_sections: self.thermal_sections.clone(),\n            cases: self.cases.clone(),\n")
sub(p, "            thermal_sections: snapshot.thermal_sections.clone(),\n", "            thermal_sections: snapshot.thermal_sections.clone(),\n            cases: snapshot.cases.clone(),\n")
p = A.diff + "/🦀️.rs"
sub(p, "    pub thermal_sections: En1991ThermalSectionDelta,\n}", "    pub thermal_sections: En1991ThermalSectionDelta,\n    #[state(artifact)]\n    pub cases: semio_s_plugin_norm_compliance::design_case::CaseRowDelta,\n}")
sub(p, "        self.thermal_sections.absorb(other.thermal_sections);\n", "        self.thermal_sections.absorb(other.thermal_sections);\n        self.cases.absorb(other.cases);\n")
sub(p, '            thermal_sections: self.thermal_sections.commit_onto(&base.thermal_sections, capability).map_err(|error| error.under(["thermal_sections"]))?,\n', '            thermal_sections: self.thermal_sections.commit_onto(&base.thermal_sections, capability).map_err(|error| error.under(["thermal_sections"]))?,\n            cases: self.cases.commit_onto(&base.cases, capability).map_err(|error| error.under(["cases"]))?,\n')
sub(p, "            && self.thermal_sections.is_empty()\n", "            && self.thermal_sections.is_empty()\n            && self.cases.is_empty()\n")
sub(p, "            thermal_sections: self.thermal_sections.inverse(&base.thermal_sections),\n", "            thermal_sections: self.thermal_sections.inverse(&base.thermal_sections),\n            cases: self.cases.inverse(&base.cases),\n")
print(len(LOG))
