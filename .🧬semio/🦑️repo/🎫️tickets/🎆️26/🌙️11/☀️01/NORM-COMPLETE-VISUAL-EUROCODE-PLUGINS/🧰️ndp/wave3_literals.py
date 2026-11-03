import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).parent))
from ndplib import *
d = load()
B = "No spec text in the repo; value as hard-coded by the artifact before the registry row, not checked against the print; "
def p(fam, key, clause, symbol, rec, de, unit, note, part):
    put(d, fam, key, clause, symbol, rec, de, unit, B + note, part=part)
A = "anchor-gamma-ms"
for k, v, u, n in [("f-uk-max", 800.0, "MPa", "validity limit f_uk of the steel partial factor formulas"), ("ratio-max", 0.8, "", "validity limit f_yk/f_uk of the steel partial factor formulas"),
    ("n.slope", 1.2, "", "gamma_Ms,N = slope f_uk/f_yk in tension"), ("n.min", 1.4, "", "lower bound of gamma_Ms,N"),
    ("v.slope", 1.0, "", "gamma_Ms,V = slope f_uk/f_yk in shear"), ("v.min", 1.25, "", "lower bound of gamma_Ms,V"),
    ("other", 1.5, "", "gamma_Ms outside the validity limits; the artifact uses one value for tension and shear, whether Table 4.1 splits it is to be verified")]:
    p("1992", f"{A}.{k}", "EN 1992-4 Table 4.1", f"gamma_ms {k}", v, v, u, n, "4")
p("1992", "defl-limit-qp", "7.4.1(4)", "deflection limit span divisor quasi-permanent", 250.0, 250.0, "", "w <= L/250 of the quasi-permanent combination", "1-1")
p("1992", "defl-limit-after", "7.4.1(5)", "deflection limit span divisor after construction", 500.0, 500.0, "", "deflection after construction w <= L/500", "1-1")
p("1992", "vrdcc-k", "6.2.3(2)", "vrdcc_k", None, 0.48, "", "coefficient 0.48 of the NA-DE V_Rd,cc = c 0.48 f_ck^(1/3) (1 - 1.2 sigma_cd/f_cd) b_w z with c = ndp.en1992.vrdcc-c; checked in the E-03 worked example, no double count", "1-1")
p("1992", "nu2-offset", "6.2.3(103)", "nu2_offset", None, 1.1, "", "nu2 = min(1.1 - f_ck/500, 1), offset 1.1 of nu1 = 0.75 nu2", "1-1")
p("1992", "nu2-fck-divisor", "6.2.3(103)", "nu2_fck_divisor", None, 500.0, "MPa", "nu2 = min(1.1 - f_ck/500, 1), divisor 500", "1-1")
p("1994", "slab.deflection-divisor", "9.8.2", "composite slab deflection span divisor", 250.0, 250.0, "", "delta <= L/250 of a composite slab; ndp.en1994.deflection-divisor (DE 300) is the beam row and is not applied here", "1-1")
p("1993", "bracing.deflection-limit-divisor", "5.3.3", "bracing deflection span divisor", 500.0, 500.0, "", "delta_q <= L/500 of a bracing system", "1-1")
p("1993", "bracing.imperfection-divisor", "5.3.3(1)", "bracing imperfection span divisor", 500.0, 500.0, "", "e_0 = alpha_m L/500", "1-1")
p("1993", "built-up.bow-divisor", "6.4.1", "built-up member bow divisor", 500.0, 500.0, "", "equivalent bow e_0 = L/500 of laced and battened members", "1-1")
p("1996", "fvlt.f-b-factor", "6.2", "f_vlt EN factor of f_b", 0.065, None, "", "EN f_vlt = 0.065 f_b; the NA-DE uses the f_bt,cal rows instead", "1-1")
p("1996", "fvlt.de-k", "6.2", "f_vlt NA-DE factor", None, 0.45, "", "NA-DE f_vlt = 0.45 f_bt,cal sqrt(1 + sigma/f_bt,cal)", "1-1")
for g, v, n in [("g1", 0.032, "solid units, group 1"), ("g2", 0.026, "perforated units, group 2"), ("g34", 0.020, "hollow units, groups 3 and 4")]:
    p("1996", f"fvlt.de-share.{g}", "6.2", f"f_bt,cal share {g}", None, v, "", f"NA-DE f_bt,cal = share f_b for {n}", "1-1")
for x, v in [("mx1", 0.020), ("mx2", 0.020), ("mx3", 0.025), ("mx4", 0.030), ("mx5", 0.040)]:
    p("1996", f"m60.cover.{x}", "M-60", f"minimum cover {x}", v, v, "m", f"minimum reinforcement cover of exposure class {x.upper()}", "1-1")
p("1992", "imperfection-theta0-divisor", "5.2(5)", "theta0 divisor", 200.0, 200.0, "", "theta_0 = 1/divisor, the basic value of the inclination", "1-1")
save(d)
