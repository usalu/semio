import os as _o, sys as _s; _o.environ.get("TH_FORCE") or (_s.stderr.write("unsafe against the current tree (destructive or not idempotent), refusing to run (TH_FORCE=1 forces)\n"), _s.exit(2))  # unsafe-guard
import os, sys; sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib import *
A = Art("en1992")
CASE = '''vec![CaseRow {
                id: "case-m04".into(),
                label_en: "Strength development of the frame concrete".into(),
                label_de: "Festigkeitsentwicklung des Rahmenbetons".into(),
                entry_id: "M-04".into(),
                numbers: vec![CaseNumber { id: "fck".into(), value: 30.0 }, CaseNumber { id: "t".into(), value: 28.0 }],
                texts: vec![CaseText { id: "cement".into(), value: "N".into() }],
            }]'''
p = A.snap + "/🦀️.rs"
sub(p, "use semio_s_plugin_norm_compliance::AnnexChoice;\n", "use semio_s_plugin_norm_compliance::AnnexChoice;\npub use semio_s_plugin_norm_compliance::design_case::{CaseNumber, CaseRow, CaseText};\n")
sub(p, "    pub placements: Vec<Placement>,\n}", "    pub placements: Vec<Placement>,\n    #[dsl(table)]\n    #[state(artifact)]\n    pub cases: Vec<CaseRow>,\n}")
s = rd(p)
tok = "            placements: vec![],\n"
assert s.count(tok) >= 1, s.count(tok)
i = s.index(tok)
s = s[:i] + tok + "            cases: %s,\n" % CASE + s[i + len(tok):]
j = i + len(tok) + 10
k = s.find(tok, j)
while k != -1:
    s = s[:k] + tok + "            cases: Vec::new(),\n" + s[k + len(tok):]
    k = s.find(tok, k + len(tok) + 25)
wr(p, s)
p = A.schema + "/🦀️.rs"
sub(p, "    pub placements: Vec<Placement>,\n}\n//#endregion 🔖️Artifact", "    pub placements: Vec<Placement>,\n    #[state(artifact)]\n    pub cases: Vec<semio_s_artifact_norm_en1992_model::CaseRow>,\n}\n//#endregion 🔖️Artifact")
sub(p, "            placements: self.placements.clone(),\n", "            placements: self.placements.clone(),\n            cases: self.cases.clone(),\n")
sub(p, "            placements: snapshot.placements.clone(),\n", "            placements: snapshot.placements.clone(),\n            cases: snapshot.cases.clone(),\n")
p = A.diff + "/🦀️.rs"
sub(p, "    pub placements: Option<En1992PlacementsRows>,\n}", "    pub placements: Option<En1992PlacementsRows>,\n    #[state(artifact)]\n    pub cases: semio_s_plugin_norm_compliance::design_case::CaseRowDelta,\n}")
sub(p, "            self.placements = self.placements.take().filter(|rows| !rows.is_empty());\n        }\n", "            self.placements = self.placements.take().filter(|rows| !rows.is_empty());\n        }\n        self.cases.absorb(other.cases);\n")
sub(p, '            next.placements = rows.commit_onto(&base.placements, capability).map_err(|error| error.under(["placements"]))?;\n        }\n', '            next.placements = rows.commit_onto(&base.placements, capability).map_err(|error| error.under(["placements"]))?;\n        }\n        next.cases = self.cases.commit_onto(&base.cases, capability).map_err(|error| error.under(["cases"]))?;\n')
sub(p, "self.placements.as_ref().map_or(true, |rows| rows.is_empty())\n", "self.placements.as_ref().map_or(true, |rows| rows.is_empty()) && self.cases.is_empty()\n")
sub(p, "            placements: self.placements.as_ref().map(|rows| rows.inverse(&base.placements)).filter(|rows| !rows.is_empty()),\n", "            placements: self.placements.as_ref().map(|rows| rows.inverse(&base.placements)).filter(|rows| !rows.is_empty()),\n            cases: self.cases.inverse(&base.cases),\n")
print(len(LOG))
