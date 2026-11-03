import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).parent))
from ndplib import *
d = load()
put(d, "1994", "gamma-ap", "EN 1994-1-1 9.7.2 / EN 1993-1-3", "γ_ap", 1.0, 1.0, "", "Partial factor of the profiled steel sheeting as formwork and as composite reinforcement, 1.0 as the artifact used it (equal to γ_M0 of EN 1993-1-3); clause as recalled, unverified.")
put(d, "1994", "gamma-vs", "EN 1994-1-1 9.7.3(4)", "γ_VS", 1.25, 1.25, "", "Partial factor of the longitudinal shear resistance of a composite slab (m-k method), 1.25 as the artifact used it; clause as recalled, unverified.")
save(d)
