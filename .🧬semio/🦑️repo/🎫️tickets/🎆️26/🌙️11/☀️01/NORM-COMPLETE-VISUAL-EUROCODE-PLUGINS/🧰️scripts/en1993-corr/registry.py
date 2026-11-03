"""W3-EN1993-CORR registry step: the four EN 1993-1-9 Table 3.1 gamma_Mf rows (assessment method x consequence class) and the bracing deflection note.
Usage: /home/deck/semio/.venv/bin/python -I registry.py"""
import subprocess, sys
sys.path.insert(0, "/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS/🧰️ndp")
from ndplib import *

TEXT = "📐️specs/en1993-target.md V-502: partial factor gamma_M,f of EN 1993-1-9 Table 3.1, damage tolerant: low consequence 1.00, high consequence 1.15; safe life: low consequence 1.15, high consequence 1.35"
NA = "; DIN EN 1993-1-9/NA keeps the same table (spec V-502, NA-DE may restrict damage tolerant to inspected details, to be verified)"
CELLS = [("damage-tolerant.low", "damage tolerant, low consequence", 1.0), ("damage-tolerant.high", "damage tolerant, high consequence", 1.15), ("safe-life.low", "safe life, low consequence", 1.15), ("safe-life.high", "safe life, high consequence", 1.35)]
BRACING = ("No spec text in the repo; value as hard-coded by the artifact before the registry row, not checked against the print; delta_q <= L/500 of a bracing system. "
           "CLAUSE: EN 1993-1-1 5.3.3 (imperfections for analysis of bracing systems, Eq. 5.12 q_d = sum N_Ed 8 (e_0 + delta_q)/L^2, delta_q the in-plane deflection of the bracing system). "
           "CANDIDATES: L/500 (this row, same divisor as the bow e_0 = alpha_m L/500) and L/2500 (recalled by the ORACLES-2 reviewer for the stiffness criterion of delta_q). "
           "Do not change before the print is read; the check output is marked UNVERIFIED.")

d = load()
for key, label, v in CELLS:
    i = put(d, "1993", f"1-9.gamma-mf.{key}", "Table 3.1", f"gamma_Mf {label}", v, v, "", f"gamma_Mf {label}{NA}", part="1-9", locator="Table 3.1")
    d[i]["specRef"] = TEXT
d["ndp.en1993.bracing.deflection-limit-divisor"]["asWritten"] = BRACING
save(d)
CODEGEN = "/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS/🧰️ndp/codegen.py"
NDP = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/⚖️compliance/📜️annex-and-tables/🔖️ndp/"
subprocess.run([sys.executable, "-I", CODEGEN, NDP + "🔣️.json", NDP + "🗃️table.rs"], check=True)
