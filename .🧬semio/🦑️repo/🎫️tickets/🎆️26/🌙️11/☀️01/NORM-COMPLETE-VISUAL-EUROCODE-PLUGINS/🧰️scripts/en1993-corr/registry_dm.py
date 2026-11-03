"""W3-EN1993-CORR-2 registry step: bolt head / nut width rows for d_m of the punching resistance (EN 1993-1-8 Table 3.4).
Usage: /home/deck/semio/.venv/bin/python -I registry_dm.py"""
import subprocess, sys
sys.path.insert(0, "/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS/🧰️ndp")
from ndplib import *

SIZES = [("m12", 17.73, 20.03), ("m16", 23.67, 26.75), ("m20", 29.16, 32.95), ("m22", 33.00, 37.29), ("m24", 35.00, 39.55), ("m27", 40.00, 45.20), ("m30", 45.00, 50.85), ("m36", 53.80, 60.79)]
SPEC = "📐️specs/en1993-target.md V-402 (Table 3.4: d_m is the mean of the across points and across flats dimensions of the bolt head or the nut, whichever is smaller); 🔍️audits/wave2/printed-standards-checklist.md W3E-C01"
EVIDENCE = ("Recalled from the EN ISO 4032 hexagon nut tables (style 1, product grades A and B); NOT in the repo and NOT double-keyed from the print. The nut is the smaller of head and nut "
            "(EN ISO 4014 heads have the same across-flats and larger across-points), the minimum across-flats s_min and the minimum across-points e_min are used so that d_m and B_p,Rd err on the safe side. "
            "Preloaded EN 14399 assemblies have larger heads and nuts, so these rows stay conservative for them. A human must confirm the values against the printed product standard before the status can change.")
d = load()
for size, s_min, e_min in SIZES:
    for key, label, v in (("across-flats", "across flats s_min", s_min), ("across-points", "across points e_min", e_min)):
        i = put(d, "1993", f"1-8.3.6.head-nut.{size}.{key}", "Table 3.4", f"{size.upper()} bolt nut {label}", v, v, "mm",
                f"Bolt nut {label} of {size.upper()} in mm for d_m = (across points + across flats) / 2 of Table 3.4; EN ISO 4032, UNVERIFIED (recalled)", part="1-8", locator="Table 3.4")
        d[i]["specRef"] = SPEC
        d[i]["evidence"] = EVIDENCE
save(d)
CODEGEN = "/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS/🧰️ndp/codegen.py"
NDP = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/⚖️compliance/📜️annex-and-tables/🔖️ndp/"
subprocess.run([sys.executable, "-I", CODEGEN, NDP + "🔣️.json", NDP + "🗃️table.rs"], check=True)
