import sys, pathlib
sys.path.insert(0, "/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS/🧰️ndp")
from ndplib import *
d = load()
def p(key, clause, symbol, rec, unit, note):
    put(d, "1991", key, clause, symbol, rec, rec, unit, note + " UNVERIFIED: value from the author's recollection of the standard, to be compared with the printed text.", part="2", locator=clause, keep=True)
for lane in (1, 2, 3):
    p(f"2.alpha-q.{lane}", "4.3.2 (3)", f"alpha_Q{lane}", 1.0, "", f"Adjustment factor of the tandem axle load Q_{lane}k of lane {lane} (national choice; 1.0 is the interim value used by the artifact).")
for lane in (1, 2, 3, 4):
    p(f"2.alpha-q-udl.{lane}", "4.3.2 (3)", f"alpha_q{lane}", 1.0, "", f"Adjustment factor of the UDL q_{lane}k of lane {lane} (4 = remaining area; interim 1.0).")
p("2.braking-q-factor", "4.4.1 (2) Eq. 4.4", "0.6", 0.6, "", "Factor of the tandem weight 2 Q_1k in the braking force Q_lk.")
p("2.braking-udl-factor", "4.4.1 (2) Eq. 4.4", "0.10", 0.10, "", "Factor of alpha_q1 q_1k w_1 L in the braking force Q_lk.")
p("2.braking-min-per-alpha", "4.4.1 (2)", "180 alpha_Q1", 180000.0, "N", "Lower bound of Q_lk per unit alpha_Q1: 180 alpha_Q1 kN.")
p("2.braking-max", "4.4.1 (2)", "900", 900000.0, "N", "Upper bound of Q_lk: 900 kN.")
p("2.centrifugal-r-low", "4.4.2 Table 4.3", "r lower", 200.0, "m", "Radius below which Q_tk = 0.2 Q_v.")
p("2.centrifugal-r-high", "4.4.2 Table 4.3", "r upper", 1500.0, "m", "Radius above which Q_tk = 0.")
p("2.centrifugal-factor-low", "4.4.2 Table 4.3", "0.2", 0.2, "", "Factor of Q_v in Q_tk for r < 200 m.")
p("2.centrifugal-numerator", "4.4.2 Table 4.3", "40", 40.0, "m", "Numerator of Q_tk = 40 Q_v / r (r in m) for 200 m <= r <= 1500 m.")
save(d); print(len(d))
