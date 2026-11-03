import os as _o, sys as _s; _o.environ.get("TH_FORCE") or (_s.stderr.write("unsafe against the current tree (destructive or not idempotent), refusing to run (TH_FORCE=1 forces)\n"), _s.exit(2))  # unsafe-guard
import os, sys; sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import glob
from lib import *
A = Art("en1990")
CASE = '''CaseRow {
                id: "case-durability".into(),
                label_en: "Durability period of the office".into(),
                label_de: "Dauerhaftigkeitszeitraum des Bürogebäudes".into(),
                entry_id: "E90.04".into(),
                numbers: vec![CaseNumber { id: "years".into(), value: 50.0 }],
                texts: vec![],
            }'''
p = A.snap + "/🦀️.rs"
sub(p, "use semio_s_plugin_norm_compliance::AnnexChoice;\n", "use semio_s_plugin_norm_compliance::AnnexChoice;\npub use semio_s_plugin_norm_compliance::design_case::{CaseNumber, CaseRow, CaseText};\n")
sub(p, "    pub placements: Vec<Placement>,\n}", "    pub placements: Vec<Placement>,\n    #[dsl(table)]\n    #[state(artifact)]\n    pub cases: Vec<CaseRow>,\n}")
sub(p, "            placements: vec![],\n        }\n    }\n}\n//#endregion", "            placements: vec![],\n            cases: vec![%s],\n        }\n    }\n}\n//#endregion" % CASE)
p = A.schema + "/🦀️.rs"
sub(p, "    pub placements: Vec<semio_s_artifact_norm_en1990_model::Placement>,\n}", "    pub placements: Vec<semio_s_artifact_norm_en1990_model::Placement>,\n    #[state(artifact)]\n    pub cases: Vec<semio_s_artifact_norm_en1990_model::CaseRow>,\n}")
sub(p, "            placements: self.placements.clone(),\n", "            placements: self.placements.clone(),\n            cases: self.cases.clone(),\n")
sub(p, "            placements: snapshot.placements,\n", "            placements: snapshot.placements,\n            cases: snapshot.cases,\n")
p = A.diff + "/🦀️.rs"
sub(p, "    pub placements: En1990PlacementDelta,\n}", "    pub placements: En1990PlacementDelta,\n    #[state(artifact)]\n    pub cases: semio_s_plugin_norm_compliance::design_case::CaseRowDelta,\n}")
sub(p, "        self.placements.absorb(other.placements);\n", "        self.placements.absorb(other.placements);\n        self.cases.absorb(other.cases);\n")
sub(p, '        next.placements = self.placements.commit_onto(&base.placements, capability).map_err(|error| error.under(["placements"]))?;\n', '        next.placements = self.placements.commit_onto(&base.placements, capability).map_err(|error| error.under(["placements"]))?;\n        next.cases = self.cases.commit_onto(&base.cases, capability).map_err(|error| error.under(["cases"]))?;\n')
sub(p, "            && self.placements.is_empty()\n", "            && self.placements.is_empty()\n            && self.cases.is_empty()\n")
sub(p, "            placements: self.placements.inverse(&base.placements),\n", "            placements: self.placements.inverse(&base.placements),\n            cases: self.cases.inverse(&base.cases),\n")
for f in glob.glob(A.any + "/📚️examples/*/🦀️.rs"):
    s = rd(f)
    if "        placements: vec![],\n" in s and "cases:" not in s:
        wr(f, s.replace("        placements: vec![],\n", "        placements: vec![],\n        cases: vec![],\n", 1))
print(len(LOG))
