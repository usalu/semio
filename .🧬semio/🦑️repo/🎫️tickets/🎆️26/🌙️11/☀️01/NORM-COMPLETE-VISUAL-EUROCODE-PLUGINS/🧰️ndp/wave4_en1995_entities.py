import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).parent))
from ndplib import *
MEM = "Written from memory by the EN 1995 owner in wave 2 (v4) and not checked against the printed standard; a human must verify it against the print before the status flips."
ROWS = [
    ("creep.time-constant-years", "EN 1995-1-1 2.3.2.2 and 3.2 (model assumption)", "time constant tau of the creep deflection curve", 5.0, 5.0, "a", "1-1", "w(t) = w_inst (1 + k_def (1 - exp(-t/tau))): the standard fixes only the end value w_fin = w_inst (1 + k_def); the shape of the time curve is a modelling assumption, not a normative value"),
    ("floor.f1-limit", "EN 1995-1-1 7.3.3 (3) and NA-DE", "lower limit of the floor fundamental frequency", 8.0, 6.0, "Hz", "1-1", "EN recommends 8 Hz; the German NA value 6 Hz is not confirmed by a public source (ndp-public-verification E95-vibration)"),
    ("floor.unit-load-a", "EN 1995-1-1 7.3.3 (4) and NA-DE", "unit point load deflection limit a", 1.5, 0.5, "mm/kN", "1-1", "a = 1.5 mm/kN recommended in the EN; NA-DE 0.5 mm/kN, unverified"),
    ("floor.na-de-additional-criterion", "EN 1995-1-1/NA 7.3.3 NCI", "NA-DE additional unit-load criterion applies for f1 below", 8.0, 8.0, "Hz", "1-1", "NA-DE: between 6 Hz and 8 Hz the unit-load criterion is mandatory (unverified)"),
    ("panel.fire-load-ratio", "EN 1990-1-2 / EN 1991-1-2 2.4.2", "reduction factor eta_fi of the load in fire", 0.6, 0.6, "", "1-2", "E_fi,d = eta_fi E_d with the recommended eta_fi = 0.6 (unverified, depends on psi_fi and the load ratio)"),
]
d = load()
for key, clause, symbol, rec, de, unit, part, written in ROWS:
    i = put(d, "1995", key, clause, symbol, rec, de, unit, written, part=part)
    d[i]["evidence"] = MEM
    d[i]["specRef"] = "wave4: constant of the EN 1995 floor, creep and CLT entities"
save(d)
print(len(ROWS))
