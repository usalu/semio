import os as _o, sys as _s; _o.environ.get("TH_FORCE") or (_s.stderr.write("unsafe against the current tree (destructive or not idempotent), refusing to run (TH_FORCE=1 forces)\n"), _s.exit(2))  # unsafe-guard
import os, sys; sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib import *
import fixtures as F
A = Art("en1996")
print("snap", F.pad_snapshots(A, "placements"), "diff", F.pad_diffs(A, "placements"))
ca = F.case_row("case-m30", "One-way vertical span of the north wall", "Einachsig vertikal gespannte Nordwand", "M-30", [("t_m", 0.365), ("h_m", 2.75), ("w_ed_pa", 1500.0), ("fxk1_pa", 400000.0), ("gamma_m", 1.5), ("sigma_d_pa", 500000.0)], [("support", "pinned")])
cb = F.case_row("case-m32", "Lateral load with arching", "Horizontallast mit Gewölbewirkung", "M-32", [("t_m", 0.365), ("la_m", 2.5), ("fd_pa", 3500000.0), ("w_ed_pa", 1500.0)], [])
F.kind_fixtures(A, True, "↔️change-slab-span", ca, cb, "w_ed_pa", 2000.0, "support", "simple", "M-32")
print(len(LOG))
