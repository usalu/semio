import sys
sys.path.insert(0, "/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS/🧰️ndp")
from ndplib import *
d = load()
def p(part, key, clause, symbol, rec, unit, note, line):
    put(d, "1991", key, clause, symbol, rec, rec, unit, note + f" UNVERIFIED: spec line {line} carries the value as [N]/[V]; to be compared with the printed text.", part=part, locator=clause, keep=True)
p("2", "2.footbridge-q-base", "5.3.2.1", "q_fk base", 2000.0, "Pa", "Footbridge uniform load q_fk = 2.0 + 120/(L + 30) kN/m2: constant term.", "E2.12")
p("2", "2.footbridge-q-slope", "5.3.2.1", "q_fk numerator", 120000.0, "N/m", "Footbridge uniform load numerator 120 kN/m of q_fk = 2.0 + 120/(L + 30) kN/m2.", "E2.12")
p("2", "2.footbridge-q-offset", "5.3.2.1", "q_fk offset", 30.0, "m", "Footbridge uniform load length offset 30 m of q_fk = 2.0 + 120/(L + 30) kN/m2.", "E2.12")
p("2", "2.footbridge-q-min", "5.3.2.1", "q_fk,min", 2500.0, "Pa", "Lower bound 2.5 kN/m2 of the footbridge uniform load.", "E2.12")
p("2", "2.footbridge-q-max", "5.3.2.1", "q_fk,max", 5000.0, "Pa", "Upper bound 5.0 kN/m2 of the footbridge uniform load.", "E2.12")
p("2", "2.footbridge-horizontal-fraction", "5.4", "Q_flk fraction", 0.10, "", "Horizontal footbridge force Q_flk as a fraction of the total vertical load.", "E2.12")
p("2", "2.footbridge-horizontal-min", "5.4", "Q_flk,min", 10000.0, "N", "Lower bound 10 kN of the horizontal footbridge force (spec line shows a question mark).", "E2.12")
p("2", "2.footbridge-walk-f-min", "5.7", "f walking min", 1.7, "Hz", "Lower bound of the vertical walking frequency range of pedestrians.", "E2.13")
p("2", "2.footbridge-walk-f-max", "5.7", "f walking max", 2.1, "Hz", "Upper bound of the vertical walking frequency range of pedestrians.", "E2.13")
p("2", "2.dispersal-ratio", "4.3.6", "dispersal tan(alpha)", 1.0, "", "Load spreading through pavement and slab at 45 degrees to the mid-plane: horizontal run per unit depth.", "E2.06")
p("4", "4.k-mean-factor", "E.3", "K_m factor", 1.1, "", "Mean lateral pressure ratio K_m = 1.1 (1 - sin phi_i).", "E4.02")
save(d); print(len(d))
