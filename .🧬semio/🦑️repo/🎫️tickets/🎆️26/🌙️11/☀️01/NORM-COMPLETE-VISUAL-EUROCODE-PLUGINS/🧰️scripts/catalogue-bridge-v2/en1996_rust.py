import os as _o, sys as _s; _o.environ.get("TH_FORCE") or (_s.stderr.write("unsafe against the current tree (destructive or not idempotent), refusing to run (TH_FORCE=1 forces)\n"), _s.exit(2))  # unsafe-guard
import os, sys; sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib import *
A = Art("en1996")
CASE = '''vec![CaseRow {
                id: "case-m32".into(),
                label_en: "Lateral load with arching of the north wall".into(),
                label_de: "Horizontallast mit Gewölbewirkung der Nordwand".into(),
                entry_id: "M-32".into(),
                numbers: vec![CaseNumber { id: "t_m".into(), value: 0.365 }, CaseNumber { id: "la_m".into(), value: 2.5 }, CaseNumber { id: "fd_pa".into(), value: 3.5e6 }, CaseNumber { id: "w_ed_pa".into(), value: 1500.0 }],
                texts: vec![],
            }]'''
p = A.snap + "/🦀️.rs"
sub(p, "use semio_s_plugin_norm_compliance::{AnnexChoice, DesignSituation};\n", "use semio_s_plugin_norm_compliance::{AnnexChoice, DesignSituation};\npub use semio_s_plugin_norm_compliance::design_case::{CaseNumber, CaseRow, CaseText};\n")
sub(p, "    pub placements: Vec<Placement>,\n}", "    pub placements: Vec<Placement>,\n    #[dsl(table)]\n    #[state(artifact)]\n    pub cases: Vec<CaseRow>,\n}")
s = rd(p)
tok = "            placements: Vec::new(),\n"
assert s.count(tok) == 2
i = s.index(tok)
s = s[:i] + tok + "            cases: %s,\n" % CASE + s[i + len(tok):]
j = s.index(tok, i + len(tok) + 10)
s = s[:j] + tok + "            cases: Vec::new(),\n" + s[j + len(tok):]
wr(p, s)
p = A.schema + "/🦀️.rs"
sub(p, "    pub placements: Vec<semio_s_artifact_norm_en1996_model::Placement>,\n}", "    pub placements: Vec<semio_s_artifact_norm_en1996_model::Placement>,\n    #[state(artifact)]\n    pub cases: Vec<semio_s_artifact_norm_en1996_model::CaseRow>,\n}")
sub(p, "            placements: self.placements.clone(),\n", "            placements: self.placements.clone(),\n            cases: self.cases.clone(),\n")
sub(p, "            placements: snapshot.placements,\n", "            placements: snapshot.placements,\n            cases: snapshot.cases,\n")
p = A.diff + "/🦀️.rs"
sub(p, "    pub placements: En1996PlacementDelta,\n}", "    pub placements: En1996PlacementDelta,\n    #[state(artifact)]\n    pub cases: semio_s_plugin_norm_compliance::design_case::CaseRowDelta,\n}")
sub(p, "        self.placements.absorb(other.placements);\n", "        self.placements.absorb(other.placements);\n        self.cases.absorb(other.cases);\n")
s = rd(p)
a = '            walls: self.walls.commit_onto(&base.walls, capability).map_err(|error| error.under(["walls"]))?,\n        };'
assert s.count(a) == 1
wr(p, s.replace(a, '            walls: self.walls.commit_onto(&base.walls, capability).map_err(|error| error.under(["walls"]))?,\n            placements: self.placements.commit_onto(&base.placements, capability).map_err(|error| error.under(["placements"]))?,\n            cases: self.cases.commit_onto(&base.cases, capability).map_err(|error| error.under(["cases"]))?,\n        };'))
sub(p, "            && self.placements.is_empty()\n", "            && self.placements.is_empty()\n            && self.cases.is_empty()\n")
sub(p, "            placements: self.placements.inverse(&base.placements),\n", "            placements: self.placements.inverse(&base.placements),\n            cases: self.cases.inverse(&base.cases),\n")
print(len(LOG))
