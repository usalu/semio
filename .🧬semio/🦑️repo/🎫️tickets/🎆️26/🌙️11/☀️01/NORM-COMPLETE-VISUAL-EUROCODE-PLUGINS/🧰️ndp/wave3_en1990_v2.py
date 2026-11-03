import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).parent))
from ndplib import *
d = load()
N = "wave 3 en1990b, UNVERIFIED: value recalled from the standard, not checked against the printed edition"
put(d, "1990", "form.alpha_e_secondary_factor", "C.7", "form.alpha_e,secondary / alpha_e", 0.4, 0.4, "", f"Factor on the sensitivity factor alpha_E of the leading action for the accompanying action of g = R - E1 - E2 (alpha_E = -0.4 * -0.7) ({N})")
save(d)
