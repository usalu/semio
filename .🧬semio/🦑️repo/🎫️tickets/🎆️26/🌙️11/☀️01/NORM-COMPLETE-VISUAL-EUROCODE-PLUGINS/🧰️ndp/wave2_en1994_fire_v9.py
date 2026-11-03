import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).parent))
from ndplib import *
d = load()
T = [20, 100, 200, 300, 400, 500, 600, 700, 800, 900, 1000, 1100, 1200]
HOT = [1.0, 1.0, 1.0, 1.0, 1.0, 0.78, 0.47, 0.23, 0.11, 0.06, 0.04, 0.02, 0.0]
COLD = [1.0, 1.0, 1.0, 1.0, 0.94, 0.67, 0.40, 0.12, 0.11, 0.08, 0.05, 0.03, 0.0]
for key, vals, what in (("hot", HOT, "hot rolled"), ("cold", COLD, "cold worked")):
    for t, v in zip(T, vals):
        put(d, "1994", f"1-2.ks.{key}.{t}", "EN 1992-1-2 Table 3.2a", f"k_s,theta {what} {t} C", v, v, "", f"Strength reduction k_s(theta) of {what} reinforcing steel (class N) at {t} C as recalled from EN 1992-1-2 Table 3.2a, referenced by EN 1994-1-2 3.2; not compared with the print. Reinforcement never uses the structural-steel k_y,theta of EN 1993-1-2.", part="1-2", locator="Table 3.2a")
put(d, "1994", "1-2.alpha-unexposed", "EN 1991-1-2 3.2", "alpha_c,unexposed", 9.0, 9.0, "W/m2K", "Combined convective and radiative heat transfer coefficient of the unexposed side, 9 W/m2K as recalled from EN 1991-1-2 / EN 1992-1-2; not compared with the print.", part="1-2", locator="EN 1991-1-2 3.2")
put(d, "1994", "1-2.rho-concrete", "EN 1994-1-2 3.3.2", "rho_c", 2300.0, 2300.0, "kg/m3", "Unit mass of normal weight concrete in the temperature field, 2300 kg/m3 as recalled from EN 1994-1-2 3.3.2; not compared with the print.", part="1-2", locator="3.3.2")
put(d, "1994", "1-2.heff", "EN 1994-1-2 4.3.2", "h_eff", None, None, "m", "Effective thickness of a composite slab on profiled sheeting: the artifact idealises the rib as h_c + h_p * b_0 / pitch (dovetail 1.25 b_0 / pitch) for the slab insulation check and the beam temperature field; the closed formula of the print (h1, h2, l1, l2, l3) is not transcribed.", part="1-2", locator="4.3.2")
save(d)
