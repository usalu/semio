import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).parent))
from ndplib import *
MEM = "Written from memory by the EN 1995 owner in wave 2 (v2) and not checked against the printed standard; a human must verify it against the print before the status flips."
ROWS = [
    ("fire.reduced-bending-divisor", "EN 1995-1-2 4.2.3", "k_mod,fi bending divisor", 200.0, "", "1-2", "k_mod,fi = 1 - p/(200 A_r) for bending"),
    ("fire.reduced-compression-divisor", "EN 1995-1-2 4.2.3", "k_mod,fi compression divisor", 125.0, "", "1-2", "k_mod,fi = 1 - p/(125 A_r) for compression"),
    ("fire.reduced-tension-divisor", "EN 1995-1-2 4.2.3", "k_mod,fi tension and stiffness divisor", 330.0, "", "1-2", "k_mod,fi = 1 - p/(330 A_r) for tension and modulus"),
    ("fire.reduced-time-limit", "EN 1995-1-2 4.2.3", "t of linear k_mod,fi growth", 20.0, "min", "1-2", "k_mod,fi interpolated linearly in time below 20 min"),
    ("fire.clt-fall-off-factor", "EN 1995-1-2 CLT research model", "CLT char rate factor after glue line fall-off", 2.0, "", "1-2", "char rate factor 2 after a glue line (Schmid/Frangi research model, not in the printed standard)"),
    ("fire.clt-fall-off-protect", "EN 1995-1-2 CLT research model", "new char depth protecting the next CLT layer", 25.0, "mm", "1-2", "factor applies until 25 mm of new char protect the layer (research model)"),
    ("bridge.crowd-force", "EN 1995-2 7.3 / Setra", "dynamic force per pedestrian", 280.0, "N", "2", "d·280 N·10.8·√(ξ/n)·ψ (Setra footbridge guide)"),
    ("bridge.crowd-factor", "EN 1995-2 7.3 / Setra", "crowd synchronisation factor", 10.8, "", "2", "10.8·√(ξ/n) (Setra footbridge guide)"),
    ("bridge.crowd-psi-start", "EN 1995-2 7.3 / Setra", "psi starts above frequency", 0.5, "Hz", "2", "ψ = 0 below 0.5 Hz"),
    ("bridge.crowd-psi-full-from", "EN 1995-2 7.3 / Setra", "psi equals 1 from frequency", 1.0, "Hz", "2", "ψ = 1 between 1.0 and 1.7 Hz"),
    ("bridge.crowd-psi-full-to", "EN 1995-2 7.3 / Setra", "psi equals 1 up to frequency", 1.7, "Hz", "2", "ψ = 1 between 1.0 and 1.7 Hz"),
    ("bridge.crowd-psi-end", "EN 1995-2 7.3 / Setra", "psi vanishes above frequency", 2.1, "Hz", "2", "ψ = 0 above 2.1 Hz"),
    ("annex-c.eta.glued", "EN 1995-1-1 Table C.1", "eta glued", 1.0, "", "1-1", "Annex C eta for glued joints"),
    ("annex-c.eta.nailed", "EN 1995-1-1 Table C.1", "eta nailed", 4.0, "", "1-1", "Annex C eta for nailed joints (default of the catalogue)"),
    ("annex-c.eta.connectors", "EN 1995-1-1 Table C.1", "eta connectors", 3.0, "", "1-1", "Annex C eta for connectors"),
    ("annex-c.eta.bolted", "EN 1995-1-1 Table C.1", "eta bolted", 6.0, "", "1-1", "Annex C eta for bolted joints"),
    ("fire.connection-reduced-load-k", "EN 1995-1-2 6.2", "k of t_fi,d = -ln(eta)/k", None, "1/min", "1-2", "reduced-load connection constant; no value is available, the case supplies k_fire"),
]
d = load()
for key, clause, symbol, value, unit, part, written in ROWS:
    i = put(d, "1995", key, clause, symbol, value, value, unit, written, part=part)
    d[i]["evidence"] = MEM
    d[i]["specRef"] = "wave3: constant of the EN 1995 kernel formerly hard-coded"
save(d)
print(len(ROWS))
