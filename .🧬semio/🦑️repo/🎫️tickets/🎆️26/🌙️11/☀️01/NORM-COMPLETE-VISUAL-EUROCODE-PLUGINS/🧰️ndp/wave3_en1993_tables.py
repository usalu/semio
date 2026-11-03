import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).parent))
from ndplib import *
d = load()
def row(key, clause, symbol, v, note, unit=""):
    put(d, "1993", key, clause, symbol, v, v, unit, f"{note}; as used by the artifact")
row("1-1.e-modulus", "3.2.6", "E", 210000.0, "Modulus of elasticity of structural steel", "MPa")
row("1-1.g-modulus", "3.2.6", "G", 81000.0, "Shear modulus of structural steel", "MPa")
row("1-1.poisson", "3.2.6", "nu", 0.3, "Poisson ratio of structural steel")
row("1-1.alpha-t", "3.2.6", "alpha_T", 12e-6, "Coefficient of linear thermal expansion", "1/K")
GR = [("1-4301", 210.0, 520.0, 6.0), ("1-4401", 220.0, 520.0, 7.0), ("1-4404", 220.0, 520.0, 7.0), ("1-4571", 220.0, 520.0, 7.0), ("1-4462", 460.0, 640.0, 5.0), ("1-4003", 250.0, 450.0, 14.0)]
for g, fy, fu, n in GR:
    for sym, v, unit in (("fy", fy, "MPa"), ("fu", fu, "MPa"), ("e", 200000.0, "MPa"), ("n", n, "")):
        row(f"1-4.grade.{g}.{sym}", "Table 2.1 / Annex C", f"{sym} grade {g.replace('-', '.')}", v, "Stainless steel grade property (hot-rolled strip, flagged unverified in the report)", unit)
for key, sym, v in (("rho-internal.a", "internal element a", .772), ("rho-internal.b.cold", "internal element b cold-formed", .125), ("rho-internal.b.welded", "internal element b welded", .079), ("rho-outstand.a", "outstand element a", 1.0), ("rho-outstand.b.cold", "outstand element b cold-formed", .231), ("rho-outstand.b.welded", "outstand element b welded", .242)):
    row(f"1-4.{key}", "Table 5.3", f"rho {sym}", v, "Coefficients of rho = a/lambda - b/lambda^2")
for tbl, clause, kinds in (("flexural", "Table 5.4", {"welded-open": (.49, .2), "other": (.49, .4)}), ("ltb", "5.5.2", {"welded-open": (.76, .2), "other": (.34, .4)}), ("torsional", "5.5.2", {"all": (.34, .2)})):
    for kind, (a, l0) in kinds.items():
        row(f"1-4.{tbl}.{kind}.alpha", clause, f"alpha {tbl} {kind}", a, f"Imperfection factor of {tbl} buckling")
        row(f"1-4.{tbl}.{kind}.lambda0", clause, f"lambda_0 {tbl} {kind}", l0, f"Limiting slenderness of {tbl} buckling")
FIRE = [(20.0, 1.00, 1.00, 1.00), (100.0, 0.82, 0.96, 0.87), (200.0, 0.68, 0.92, 0.77), (300.0, 0.64, 0.88, 0.73), (400.0, 0.60, 0.84, 0.72), (500.0, 0.54, 0.80, 0.67), (600.0, 0.49, 0.76, 0.58), (700.0, 0.40, 0.71, 0.43), (800.0, 0.27, 0.63, 0.27), (900.0, 0.14, 0.45, 0.15), (1000.0, 0.06, 0.20, 0.07), (1100.0, 0.03, 0.10, 0.03), (1200.0, 0.0, 0.0, 0.0)]
for th, a, b, c in FIRE:
    for col, v in (("k02p", a), ("ke", b), ("ku", c)):
        row(f"1-2.stainless-fire.1-4301.{col}.t{int(th)}", "Table C.1", f"{col} grade 1.4301 at {int(th)} C", v, "Stainless steel reduction factor at steel temperature (unverified, copied from memory)")
for c in (160, 140, 125, 112, 100, 90, 80, 71, 63, 56, 50, 45, 40, 36):
    row(f"1-9.category.c{c}", "Table 8.1", f"detail category {c}", float(c), "Detail category of the fatigue ladder", "MPa")
for k, v in (("rib-to-deck-full-penetration", 125.0), ("rib-to-deck-partial-penetration-throat", 36.0), ("rib-to-deck-partial-penetration-plate", 71.0), ("rib-to-crossbeam-web-cutout", 71.0), ("rib-splice-butt-weld", 90.0)):
    row(f"1-9.deck.{k}", "Table 8.8", f"orthotropic deck detail {k}", v, "Orthotropic deck detail category (unverified, from memory)", "MPa")
for k, sym, v in (("n-c", "N_C", 2.0e6), ("n-d", "N_D", 5.0e6), ("n-l", "N_L", 1.0e8), ("m-1", "m1", 3.0), ("m-2", "m2", 5.0)):
    row(f"1-9.sn.{k}", "7.1 / Figure 7.1", sym, v, "Corner points and slopes of the fatigue strength curve")
row("1-9.size-effect.t-ref", "Table 8.1 notes", "reference thickness", 25.0, "Thickness above which the size effect applies", "mm")
row("1-9.size-effect.exponent", "Table 8.1 notes", "size effect exponent", 0.2, "Exponent of (t_ref/t)")
for k, sym, v in (("cat1.alpha", "alpha category 1", .075), ("cat1.la", "l_a category 1", 10.0), ("cat2.alpha", "alpha category 2", .15)):
    row(f"1-3.web-crippling.{k}", "6.1.7.2 Table 6.? values", sym, v, "Web crippling coefficient of category 1 and 2")
save(d); print(len(d))
