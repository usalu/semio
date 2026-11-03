import os as _o, sys as _s; _o.environ.get("TH_FORCE") or (_s.stderr.write("unsafe against the current tree (destructive or not idempotent), refusing to run (TH_FORCE=1 forces)\n"), _s.exit(2))  # unsafe-guard
import os, sys; sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib import *
A = Art("en1997")
CASE = '''CaseRow {
            id: "case-equ-01".into(),
            label_en: "Overturning of the retaining wall".into(),
            label_de: "Kippen der Stützwand".into(),
            entry_id: "EQU-01".into(),
            numbers: vec![CaseNumber { id: "m_dst_g".into(), value: 450_000.0 }, CaseNumber { id: "m_dst_q".into(), value: 150_000.0 }, CaseNumber { id: "m_stb_g".into(), value: 900_000.0 }],
            texts: vec![CaseText { id: "situation".into(), value: "bsP".into() }],
        }'''
p = A.snap + "/🦀️.rs"
sub(p, "use semio_s_plugin_norm_compliance::AnnexChoice;\n", "use semio_s_plugin_norm_compliance::AnnexChoice;\npub use semio_s_plugin_norm_compliance::design_case::{CaseNumber, CaseRow, CaseText};\n")
sub(p, "    pub placements: Vec<Placement>,\n}\n//#endregion 🔖️Snapshot", "    pub placements: Vec<Placement>,\n    #[dsl(table)]\n    #[state(artifact)]\n    pub cases: Vec<CaseRow>,\n}\n//#endregion 🔖️Snapshot")
sub(p, "        placements: vec![],\n    }\n}", "        placements: vec![],\n        cases: vec![%s],\n    }\n}" % CASE)
p = A.schema + "/🦀️.rs"
sub(p, "use semio_s_artifact_norm_en1997_model::{Anchor, EmbeddedWall,", "use semio_s_artifact_norm_en1997_model::{Anchor, CaseRow, EmbeddedWall,")
sub(p, "    pub placements: Vec<Placement>,\n}\n//#endregion 🔖️Artifact", "    pub placements: Vec<Placement>,\n    #[state(artifact)]\n    pub cases: Vec<CaseRow>,\n}\n//#endregion 🔖️Artifact")
sub(p, "            placements: self.placements.clone(),\n", "            placements: self.placements.clone(),\n            cases: self.cases.clone(),\n")
sub(p, "            placements: snapshot.placements,\n", "            placements: snapshot.placements,\n            cases: snapshot.cases,\n")
p = A.diff + "/🦀️.rs"
sub(p, "    pub placements: Option<En1997PlacementsRows>,\n}", "    pub placements: Option<En1997PlacementsRows>,\n    #[state(artifact)]\n    pub cases: semio_s_plugin_norm_compliance::design_case::CaseRowDelta,\n}")
sub(p, "            self.placements = self.placements.take().filter(|rows| !rows.is_empty());\n        }\n", "            self.placements = self.placements.take().filter(|rows| !rows.is_empty());\n        }\n        self.cases.absorb(other.cases);\n")
sub(p, '            next.placements = rows.commit_onto(&base.placements, capability).map_err(|error| error.under(["placements"]))?;\n        }\n', '            next.placements = rows.commit_onto(&base.placements, capability).map_err(|error| error.under(["placements"]))?;\n        }\n        next.cases = self.cases.commit_onto(&base.cases, capability).map_err(|error| error.under(["cases"]))?;\n')
sub(p, "self.placements.as_ref().map_or(true, |rows| rows.is_empty())\n", "self.placements.as_ref().map_or(true, |rows| rows.is_empty()) && self.cases.is_empty()\n")
sub(p, "            placements: self.placements.as_ref().map(|rows| rows.inverse(&base.placements)).filter(|rows| !rows.is_empty()),\n", "            placements: self.placements.as_ref().map(|rows| rows.inverse(&base.placements)).filter(|rows| !rows.is_empty()),\n            cases: self.cases.inverse(&base.cases),\n")
print("\n".join(LOG))
