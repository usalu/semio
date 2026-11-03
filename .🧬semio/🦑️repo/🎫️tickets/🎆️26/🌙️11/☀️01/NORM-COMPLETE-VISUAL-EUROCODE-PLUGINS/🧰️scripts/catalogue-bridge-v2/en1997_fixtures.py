import os as _o, sys as _s; _o.environ.get("TH_FORCE") or (_s.stderr.write("unsafe against the current tree (destructive or not idempotent), refusing to run (TH_FORCE=1 forces)\n"), _s.exit(2))  # unsafe-guard
import os, sys; sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib import *
import fixtures as F
A = Art("en1997")
ca = F.case_row("case-equ-01", "Overturning of the retaining wall", "Kippen der Stützwand", "EQU-01", [("m_dst_g", 450000.0), ("m_dst_q", 150000.0), ("m_stb_g", 900000.0)], [("situation", "bsP")])
cb = F.case_row("case-upl-01", "Uplift of the pit base", "Auftrieb der Baugrubensohle", "UPL-01", [("g_dst", 300000.0), ("g_stb", 420000.0), ("head", 2.0), ("area", 25.0)], [("situation", "bsP")])
F.kind_fixtures(A, False, "💧️change-water-flow-length", ca, cb, "m_dst_g", 500000.0, "situation", "bsT", "UPL-01")
print(len(LOG))
