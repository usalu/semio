import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).parent))
from ndplib import *
MEM = "Written from memory by the EN 1995 owner in wave 2 (v6) and not checked against the printed standard; a human must verify it against the print before the status flips."
SPEC = "wave6: constant of the EN 1995 diaphragm, truss and bracing entities (T-94 to T-96)"
ROWS = [
    ("diaphragm.method-b-k-d", "EN 1995-1-1 9.2.4.2 (Method B)", "Method B modification factor k_d of the racking resistance", 1.0, 1.0, "", "1-1", "Neutral placeholder 1.0 used by the kernel when the factor is not given; the tabulated Method B factors are not recorded, so every Method B verdict is an unverified basis"),
    ("diaphragm.method-b-k-q", "EN 1995-1-1 9.2.4.2 (Method B)", "Method B modification factor k_q of the racking resistance", 1.0, 1.0, "", "1-1", "Neutral placeholder 1.0 (see k_d)"),
    ("diaphragm.method-b-k-n", "EN 1995-1-1 9.2.4.2 (Method B)", "Method B modification factor k_n of the racking resistance", 1.0, 1.0, "", "1-1", "Neutral placeholder 1.0 (see k_d)"),
    ("diaphragm.method-b-k-op", "EN 1995-1-1 9.2.4.2 (Method B)", "Method B modification factor k_op of the racking resistance", 1.0, 1.0, "", "1-1", "Neutral placeholder 1.0 (see k_d)"),
    ("bracing.stabilising-divisor", "EN 1995-1-1 9.2.5.2", "divisor of the stabilising load q_d = k_l N_d / (divisor l)", 30.0, 30.0, "", "1-1", "q_d = k_l N_d / (30 l) as implemented in the kernel (T-95, T-96); unverified"),
    ("bracing.kl-reference-length", "EN 1995-1-1 9.2.5.2", "reference length of k_l = min(1, sqrt(L / l))", 15.0, 15.0, "m", "1-1", "k_l = min(1, sqrt(15 / l)) with l in m as implemented in the kernel; unverified"),
    ("bracing.force-divisor-solid", "EN 1995-1-1 9.2.5.3", "divisor of the stabilising force F_d = N_d / divisor for solid timber", 50.0, 50.0, "", "1-1", "F_d = N_d / 50 for solid timber as implemented in the kernel (T-95); unverified"),
    ("bracing.force-divisor-engineered", "EN 1995-1-1 9.2.5.3", "divisor of the stabilising force F_d = N_d / divisor for glued laminated timber and LVL", 80.0, 80.0, "", "1-1", "F_d = N_d / 80 for glulam and LVL as implemented in the kernel (T-95); unverified"),
    ("bracing.deflection-limit-divisor", "EN 1995-1-1 9.2.5.2", "bracing deflection limit L / divisor", 500.0, 500.0, "", "1-1", "bracing deflection limit L/500 as implemented in the kernel (T-96); unverified"),
]
d = load()
for key, clause, symbol, rec, de, unit, part, written in ROWS:
    i = put(d, "1995", key, clause, symbol, rec, de, unit, written, part=part)
    d[i]["evidence"] = MEM
    d[i]["specRef"] = SPEC
save(d)
print(len(ROWS))
