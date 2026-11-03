import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).parent))
from ndplib import *
d = load()
def keep_text(i, rec, note=None, sym=None):
    e = d[i]
    e["recommended"] = e["naDe"] = rec
    if sym: e["symbol"] = sym
put(d, "1993", "v-026.2", "5.2.1", "Eq. 5.2 tolerance", 0.05, 0.05, "", d["ndp.en1993.v-026.2"]["asWritten"], part="1-1", locator="5.2.1(4)B")
d["ndp.en1993.v-026.2"]["evidence"] = "The 5 percent threshold is an engineering tolerance of the artifact derived from the oracle spread (PyNite P-Delta and scipy eigen), not a normative value."
d["ndp.en1993.v-026.2"]["specRef"] = "wave2 ndp: value as used by the artifact"
o = d["ndp.en1993.v-420.2"]
txt = "end row cp = pi m + p, nc = 2m + 0.625e + 0.5p; inner row cp = 2p, nc = p"
o["recommended"] = o["naDe"] = txt
put(d, "1993", "1-1.alpha-cr-min", "5.2.1", "alpha_cr min", 3.0, 3.0, "", "Lower bound of alpha_cr used by the frame and storey checks for the second-order drift and load limits", part="1-1")
put(d, "1993", "1-8.psi-end-plate", "6.3.1 Table 6.8", "psi", 2.7, 2.7, "", "Shape factor psi of the moment-rotation curve for bolted end-plate joints", part="1-8")
put(d, "1993", "1-8.gamma-m5", "2.2 Table 2.1", "gamma_M5", 1.0, 1.0, "", "Partial factor of joints in hollow-section lattice girders", part="1-8")
put(d, "1993", "5.pile-driving-limit-ratio", "7.? driving", "sigma_driving / f_y limit", 0.9, 0.9, "", "Driving stress limit as a fraction of f_y used by the pile driving check", part="5")
put(d, "1993", "5.pile-full-capacity-driving-ratio", "7.? driving", "sigma_driving / f_y full", 0.6, 0.6, "", "Driving stress ratio up to which the full pile capacity is kept", part="5")
for c, (e, p) in zip(["a0", "a", "b", "c", "d"], zip([350, 300, 250, 200, 150], [300, 250, 200, 150, 100])):
    put(d, "1993", f"1-1.t51.{c}.elastic", "Table 5.1", f"L/e0 curve {c} elastic", float(e), float(e), "", f"Bow imperfection e0/L = 1/{e} for buckling curve {c}, elastic analysis", part="1-1")
    put(d, "1993", f"1-1.t51.{c}.plastic", "Table 5.1", f"L/e0 curve {c} plastic", float(p), float(p), "", f"Bow imperfection e0/L = 1/{p} for buckling curve {c}, plastic analysis", part="1-1")
TEMPS = [10, 0, -10, -20, -30, -40, -50]
T21 = {"s235-jr": [60, 50, 40, 35, 30, 25, 20], "s235-j0": [90, 75, 65, 55, 45, 40, 35], "s235-j2": [125, 105, 90, 75, 65, 55, 45],
       "s355-jr": [40, 35, 30, 25, 20, 15, 15], "s355-j0": [65, 55, 45, 40, 35, 30, 25], "s355-j2": [90, 75, 65, 55, 45, 40, 35], "s355-k2": [125, 105, 90, 75, 65, 55, 45]}
for g, vals in T21.items():
    for t, v in zip(TEMPS, vals):
        put(d, "1993", f"1-10.t21.{g}.t{t}", "Table 2.1", f"t_max {g.upper()} {t} C", float(v), float(v), "mm", f"Maximum thickness for brittle fracture avoidance, {g.upper()}, sigma_Ed = 0.75 f_y(t), T_Ed = {t} C; from memory, other grades and stress levels not encoded", part="1-10")
THETA = [20, 100, 200, 300, 400, 500, 600, 700, 800, 900, 1000, 1100, 1200]
F31 = {"ky": [1.0, 1.0, 1.0, 1.0, 1.0, 0.78, 0.47, 0.23, 0.11, 0.06, 0.04, 0.02, 0.0], "kp": [1.0, 1.0, 0.807, 0.613, 0.42, 0.36, 0.18, 0.075, 0.05, 0.0375, 0.025, 0.0125, 0.0],
       "ke": [1.0, 1.0, 0.9, 0.8, 0.7, 0.6, 0.31, 0.13, 0.09, 0.0675, 0.045, 0.0225, 0.0]}
SYM = {"ky": "k_y,theta", "kp": "k_p,theta", "ke": "k_E,theta"}
for k, vals in F31.items():
    for th, v in zip(THETA, vals):
        put(d, "1993", f"1-2.t31.{k}.{th}", "Table 3.1", f"{SYM[k]} {th} C", v, v, "", f"Reduction factor {SYM[k]} of carbon steel at {th} C", part="1-2")
save(d)
print(len(d))
