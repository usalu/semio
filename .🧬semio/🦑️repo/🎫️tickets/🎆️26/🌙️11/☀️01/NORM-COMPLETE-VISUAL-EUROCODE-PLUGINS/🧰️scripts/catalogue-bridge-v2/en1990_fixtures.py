import os as _o, sys as _s; _o.environ.get("TH_FORCE") or (_s.stderr.write("unsafe against the current tree (destructive or not idempotent), refusing to run (TH_FORCE=1 forces)\n"), _s.exit(2))  # unsafe-guard
import os, sys; sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib import *
import fixtures as F
A = Art("en1990")
print("snap", F.pad_snapshots(A, "placements"), "diff", F.pad_diffs(A, "placements"))
ca = F.case_row("case-psi", "Other representative values of the office", "Weitere repräsentative Werte des Büros", "E90.09", [("altitude", 1200.0)], [("category", "office")])
cb = F.case_row("case-durability", "Durability period", "Dauerhaftigkeitszeitraum", "E90.04", [("years", 50.0)], [])
F.kind_fixtures(A, False, "💥️change-accidental-ad", ca, cb, "altitude", 1500.0, "category", "residential", "E90.04")
print(len(LOG))
