import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).parent))
from ndplib import *
d = load()
B = "Spec value, print not at hand: "
put(d, "1991", "1-7.helicopter-c", "4.7", "C in F_d = C sqrt(m)", 3000.0, 3000.0, "N/sqrt(kg)", B + "C = 3 kN per square root of kg, m in kg (spec E17.05)", part="1-7")
put(d, "1991", "1-7.tie-peripheral", "Annex A.5.1", "peripheral tie factor", 0.4, 0.4, "", B + "T_p = 0.4 (g_k + psi q_k) s L (spec E17.06)", part="1-7")
put(d, "1991", "1-7.tie-internal", "Annex A.5.1", "internal tie factor", 0.8, 0.8, "", B + "T_i = 0.8 (g_k + psi q_k) s L (spec E17.06)", part="1-7")
put(d, "1991", "1-7.tie-cap", "Annex A.5.1", "tie force cap", 75000.0, 75000.0, "N", B + "75 kN, the lesser value governs; the reading must be confirmed in the print", part="1-7")
put(d, "1991", "1-7.derailment-table", "4.4 Tables 4.4-4.5", "derailment forces F_dx, F_dy", 0.0, 0.0, "", "Tables 4.4-4.5 are not reproduced; the design forces of a derailment case are entered", part="1-7")
put(d, "1991", "1-5.isotherm-t-max-base", "Annex A", "T_max base", 37.0, 37.0, "degC", B + "base value at sea level (spec E15.01)", part="1-5")
put(d, "1991", "1-5.isotherm-t-min-base", "Annex A", "T_min base", -24.0, -24.0, "degC", B + "base value at sea level (spec E15.01)", part="1-5")
put(d, "1991", "1-5.isotherm-lapse", "Annex A", "altitude correction", 0.5, 0.5, "K/100 m", B + "T_max and T_min both change by this amount per 100 m above sea level (spec E15.01, direction as written)", part="1-5")
put(d, "1991", "1-6.combination-psi0", "Annex A.2", "psi_0 of construction loads", 1.0, 1.0, "", B + "Q_c = sum Q_ci with psi_0 = 1.0 (spec E16.06)", part="1-6")
save(d); print(len(d))
