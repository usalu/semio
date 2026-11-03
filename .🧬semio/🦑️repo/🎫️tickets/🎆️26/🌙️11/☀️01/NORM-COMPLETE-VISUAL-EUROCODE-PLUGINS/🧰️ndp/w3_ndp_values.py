import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).parent))
from ndplib import *
d = load()
def fill(i, rec, de, unit, symbol, evidence):
    e = d[i]
    assert e["recommended"] is None and e["naDe"] is None, i
    e.update(recommended=rec, naDe=de, unit=unit, symbol=symbol, status="unverified", evidence=evidence)
def spec(file, line, quote):
    return f"ndp-values (W3): value read from the repo target spec 📐️specs/{file} line {line} ({quote}); the spec is project-written and not checked against the print. Row was a citation without a number."
S2, S3 = "en1992-target.md", "en1993-target.md"
fill("ndp.en1992.d-03.1", 0.0, 0.0, "mm", "Dc,dur,gamma", spec(S2, 76, "Dc,dur,gamma = 0 EC / DE 0; Dc,dur,st and Dc,dur,add = 0 EC / DE 0"))
fill("ndp.en1992.d-04.1", 10.0, 10.0, "mm", "Dc,dev", spec(S2, 81, "Dc,dev EC 10 mm / DE 10 mm; the reductions to 5 mm and 0 mm are separate rules"))
fill("ndp.en1992.f-03.1", 162.5, 162.5, "MPa", "Delta sigma_Rsk(N*) straight bars", spec(S2, 333, "Table 6.3N Delta sigma_Rsk(N*) = 162.5 MPa; DE NA 6.8.4 Delta sigma_Rsk = 162.5 with the open question mark on the bar diameter limit"))
fill("ndp.en1993.v-026.1", 0.5, 0.5, "", "k", spec(S3, 69, "k = 0.5 recommended; NDP k = 0.5 / [NA] 0.5 [verify]"))
fill("ndp.en1993.v-111.1", 0.4, 0.4, "", "lambda_bar_LT,0", spec(S3, 140, "lambda_bar_LT,0 = 0.4 (rec) / [NA] 0.4 and 0.75 [verify]; beta = 0.75 is not part of this row"))
fill("ndp.en1993.v-860.1", 1.5, 1.5, "", "gamma_R", spec(S3, 292, "gamma_R = 1.5 for wire ropes? rec [NA] 1.5"))
fill("ndp.en1993.v-880.1", 1.0, 1.0, "", "gamma_M0", spec(S3, 295, "gM0 1.0 in V-880, steel bridges EN 1993-2 + NA-DE; gamma_M1 and gamma_M2 are not part of this row"))
KEPT = "ndp-values (W3): the NA-DE value is not keyed. Until this row the check applied {c} as an in-code fallback under every annex; the DE column now states that same constant explicitly so the read can never fall back silently. Unverified, replace with the DIN 1054 print value."
for i, c in [("ndp.en1997.7.7.lateral-deflection-limit", "0.010 m"), ("ndp.en1997.9.blum-embedment-increase", "1.2"), ("ndp.en1997.9.seismic.gamma-sliding", "1.0")]:
    e = d[i]
    assert e["naDe"] is None and e["recommended"] is not None
    e["naDe"] = e["recommended"]
    e["evidence"] = KEPT.format(c=c)
save(d)
