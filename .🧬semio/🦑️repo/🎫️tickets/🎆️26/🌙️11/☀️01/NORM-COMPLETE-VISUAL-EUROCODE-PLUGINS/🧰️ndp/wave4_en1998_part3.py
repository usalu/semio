import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).parent))
from ndplib import *
d = load()
SPEC = "EN 1998-3 target spec 3.3 (en1997-en1998-target.md AS-0x); value from the spec table"
def row(key, clause, symbol, rec, de, unit, note, spec=SPEC):
    i = put(d, "1998", key, clause, symbol, rec, de, unit, note, part="3")
    d[i]["specRef"] = spec
for kl, v in (("kl1", 1.35), ("kl2", 1.20), ("kl3", 1.00)):
    row(f"en3.cf.{kl}", "3.3 Table 3.1", f"CF {kl.upper()}", v, v, "", f"Confidence factor of knowledge level {kl.upper()}; the NA-DE value is not stated in the spec and equals the recommended one until verified")
for ls, v in (("nc", 2475.0), ("sd", 475.0), ("dl", 225.0)):
    row(f"en3.return-period.{ls}", "2.1 Table 2.1", f"T_R {ls.upper()}", v, v, "yr", f"Return period of the limit state {ls.upper()} (EN recommended)")
row("en3.return-period.exponent", "EN 1998-1 2.1(4)", "k", 3.0, 3.0, "", "Exponent of a_g(T_R) = a_g(475) (T_R/475)^(1/k)")
row("en3.na.return-period", "NA-DE EN 1998-3", "T_R NA-DE", None, 475.0, "yr", "NA-DE fixes the return period to T_R = 475 yr with a_gR of Table NA.1; the EN recommended values of en3.return-period.* stay the limit-state values until the NA text is verified")
for k, cl, sym, v, note in (
    ("axial-limit", "A.3.3", "N upper factor", 0.55, "min(N; 0.55 A_c f_c) of the axial term of V_R"),
    ("ductility-cap", "A.3.3", "mu_pl cap", 5.0, "min(5; mu_pl) of the ductility degradation of V_R"),
    ("ductility-slope", "A.3.3", "mu_pl slope", 0.05, "1 - 0.05 min(5; mu_pl)"),
    ("base", "A.3.3", "concrete term factor", 0.16, "0.16 max(0.5; 100 rho_tot) of the concrete term of V_R"),
    ("rho-min", "A.3.3", "rho_tot floor", 0.5, "max(0.5; 100 rho_tot)"),
    ("slenderness-slope", "A.3.3", "L_V/h slope", 0.16, "1 - 0.16 min(5; L_V/h)"),
    ("slenderness-cap", "A.3.3", "L_V/h cap", 5.0, "min(5; L_V/h)"),
):
    row(f"en3.shear.{k}", cl, sym, v, v, "", f"Coefficient of the cyclic shear capacity V_R: {note}")
save(d)
print(len(d))
