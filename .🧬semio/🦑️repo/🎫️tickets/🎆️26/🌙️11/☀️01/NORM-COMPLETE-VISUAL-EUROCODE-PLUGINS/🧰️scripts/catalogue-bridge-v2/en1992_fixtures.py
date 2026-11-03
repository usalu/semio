import os as _o, sys as _s; _o.environ.get("TH_FORCE") or (_s.stderr.write("unsafe against the current tree (destructive or not idempotent), refusing to run (TH_FORCE=1 forces)\n"), _s.exit(2))  # unsafe-guard
import os, sys; sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import lib2
from lib import *
import fixtures2 as F
A = Art("en1992")
print("snap", F.pad_snapshots(A, "placements"), "diff", F.pad_diffs(A, "placements"))
ca = F.case_row("case-m04", "Strength development of the frame concrete", "Festigkeitsentwicklung des Rahmenbetons", "M-04", [("fck", 30.0), ("t", 28.0)], [("cement", "N")])
cb = F.case_row("case-m02", "Concrete property table", "Betonkennwerte", "M-02", [("fck", 30.0)], [])
F.kind_fixtures(A, True, "🏋️change-action-category", ca, cb, "t", 56.0, "cement", "R", "M-02")
print(len(LOG))
