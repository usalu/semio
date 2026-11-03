import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).parent))
from ndplib import *
d = load()
EVIDENCE = "Recalled from the German annex to 6.4.4(1) (interior columns with u_0/d < 4: C_Rd,c = 0.18/gamma_c (0.1 u_0/d + 0.6) >= 0.15/gamma_c); not checked against the licensed print, the applicability to interior columns only is an assumption. The EN annex has no such reduction: limit 0 disables it (slope 0, offset 1, floor 0)."
ROWS = [
    ("limit", "u_0/d limit", 0.0, 4.0, "Limit of u_0/d below which the C_Rd,c numerator is reduced (interior columns); 0 disables the reduction."),
    ("slope", "slope of u_0/d", 0.0, 0.1, "Factor slope on u_0/d of the reduced C_Rd,c numerator."),
    ("offset", "offset", 1.0, 0.6, "Factor offset of the reduced C_Rd,c numerator."),
    ("floor", "C_Rd,c numerator floor", 0.0, 0.15, "Lower bound of the reduced C_Rd,c numerator (divided by gamma_c afterwards)."),
]
for key, symbol, en, de, note in ROWS:
    i = put(d, "1992", f"punch-small-u0.{key}", "6.4.4(1)", symbol, en, de, "", note, part="1-1")
    d[i]["evidence"] = EVIDENCE
save(d)
