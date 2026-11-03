import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).parent))
from ndplib import *
d = load()
def p(part, key, clause, symbol, rec, unit, note, line):
    put(d, "1991", key, clause, symbol, rec, rec, unit, note + f" UNVERIFIED: {line}; compare with the printed text before use.", part=part, locator=clause, keep=True)
S = "spec line E2.10 (fatigue load models)"
p("2", "2.fatigue-flm1-q-factor", "4.6.2", "FLM1 Q factor", 0.7, "", "FLM1 axle loads are 0.7 Q_ik of the lane of LM1.", S)
p("2", "2.fatigue-flm1-udl-factor", "4.6.2", "FLM1 q factor", 0.3, "", "FLM1 distributed loads are 0.3 q_ik of the lane of LM1.", S)
p("2", "2.fatigue-flm3-axle", "4.6.4", "FLM3 axle load", 120000.0, "N", "FLM3 single vehicle with four axles of 120 kN.", S)
for i, v in enumerate((1.2, 6.0, 1.2), 1):
    p("2", f"2.fatigue-flm3-spacing-{i}", "4.6.4", f"FLM3 axle spacing {i}", v, "m", f"FLM3 axle spacing {i} of 1.2 m, 6.0 m, 1.2 m.", S)
p("2", "2.fatigue-flm3-track", "4.6.4", "FLM3 track", 2.0, "m", "FLM3 wheel track 2.0 m.", S)
p("2", "2.fatigue-flm3-wheel", "4.6.4", "FLM3 wheel contact", 0.4, "m", "FLM3 wheel contact side 0.40 m (square).", S)
C = "spec line E2.11 (vehicle collision on supports; the spec marks the values [V])"
p("2", "2.collision-deck-fx", "4.7.3.1", "F_dx deck level", 1000000.0, "N", "Collision force in driving direction on the support at deck level.", C)
p("2", "2.collision-deck-fy", "4.7.3.1", "F_dy deck level", 500000.0, "N", "Collision force across the driving direction on the support at deck level.", C)
p("2", "2.collision-soffit-fx", "4.7.3.1", "F_dx soffit", 500000.0, "N", "Collision force on the soffit (the spec line carries a question mark).", C)
A = "EN 1991-1-2 Annex C recalled; the spec shows only L_f, z_0 and theta for the localised fire"
p("1-2", "1-2.local-lh-coefficient", "C.2", "L_H coefficient", 2.9, "", "Horizontal flame length L_h = (2.9 Q_H*^0.33 - 1) H of a fire reaching the ceiling.", A)
p("1-2", "1-2.local-lh-exponent", "C.2", "L_H exponent", 0.33, "", "Exponent of Q_H* in the horizontal flame length.", A)
p("1-2", "1-2.local-q-denominator", "C.2", "Q* denominator", 1110000.0, "W", "Dimensionless heat release Q* = Q/(1.11e6 D^2.5) for D and H.", A)
p("1-2", "1-2.local-zprime-factor", "C.2", "z' factor", 2.4, "", "Vertical position of the virtual heat source z' = 2.4 D (Q_D*^(2/5) - Q_D*^(2/3)) for Q_D* < 1 and 2.4 D (1.0 - Q_D*^(2/5)) otherwise.", A)
p("1-2", "1-2.local-flux-plateau", "C.2", "Hasemi flux plateau", 100000.0, "W/m2", "Heat flux 100 kW/m2 for y <= 0.30.", A)
p("1-2", "1-2.local-flux-intercept", "C.2", "Hasemi flux intercept", 136300.0, "W/m2", "Heat flux 136.3e3 - 121e3 y for 0.30 < y < 1.0.", A)
p("1-2", "1-2.local-flux-slope", "C.2", "Hasemi flux slope", 121000.0, "W/m2", "Heat flux slope 121e3 W/m2 per unit y.", A)
p("1-2", "1-2.local-flux-tail", "C.2", "Hasemi flux tail", 15000.0, "W/m2", "Heat flux 15e3 y^-3.7 for y >= 1.0.", A)
p("1-2", "1-2.local-flux-tail-exponent", "C.2", "Hasemi tail exponent", -3.7, "", "Exponent -3.7 of the Hasemi heat flux tail.", A)
save(d); print(len(d))
