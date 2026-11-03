import os as _o, sys as _s; _o.environ.get("TH_FORCE") or (_s.stderr.write("unsafe against the current tree (destructive or not idempotent), refusing to run (TH_FORCE=1 forces)\n"), _s.exit(2))  # unsafe-guard
import os, sys; sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import lib2
from lib import *
import fixtures2 as F
A = Art("en1991")
print("snap", F.pad_snapshots(A, "thermalSections"), "diff", F.pad_diffs(A, "thermalSections"))
ca = F.case_row("case-e11-04", "Imposed-load reduction of the office floors", "Nutzlastabminderung der Bürogeschosse", "E11.04", [("area", 30.0), ("storeys", 4.0)], [("category", "A")])
cb = F.case_row("case-e11-03", "Imposed loads by category", "Nutzlasten nach Kategorie", "E11.03", [("depth", 0.2)], [("category", "A")])
F.kind_fixtures(A, True, "❄️change-altitude", ca, cb, "area", 60.0, "category", "B", "E11.03")
print(len(LOG))
