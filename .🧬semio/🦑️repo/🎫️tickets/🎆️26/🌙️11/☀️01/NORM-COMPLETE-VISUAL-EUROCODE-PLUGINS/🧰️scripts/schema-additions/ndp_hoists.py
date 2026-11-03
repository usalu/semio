import sys
sys.path.insert(0, "/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS/🧰️ndp")
from ndplib import *
d = load()
def p(key, clause, symbol, rec, unit, note):
    put(d, "1991", key, clause, symbol, rec, rec, unit, note + " UNVERIFIED: the spec line carries the value as [V]/[N rec]; to be compared with the printed text.", part="3", locator=clause, keep=True)
p("3.test-static", "2.9", "Q_T static factor", 1.1, "", "Static test load Q_T = 1.1 Q_H (spec line E3.04, quoted as written).")
p("3.test-dynamic", "2.9", "Q_T dynamic factor", 1.25, "", "Dynamic test load Q_T = 1.25 Q_H (spec line E3.04, quoted as written).")
p("3.beta-3-grab", "2.5", "beta_3 grab", 0.5, "", "Release factor beta_3 of a grab or similar device in phi_3 = 1 - (dm/m)(1 + beta_3) (spec line E3.01).")
p("3.beta-3-magnet", "2.5", "beta_3 magnet", 1.0, "", "Release factor beta_3 of a magnet or spreader with active release in phi_3 = 1 - (dm/m)(1 + beta_3) (spec line E3.01).")
save(d); print(len(d))
