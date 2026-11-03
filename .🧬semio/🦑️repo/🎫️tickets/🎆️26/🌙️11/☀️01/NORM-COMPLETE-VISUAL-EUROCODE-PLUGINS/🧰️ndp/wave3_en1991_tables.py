import sys, pathlib, re
sys.path.insert(0, str(pathlib.Path(__file__).parent))
from ndplib import *
d = load()
def row(key, clause, symbol, v, note, unit=""):
    put(d, "1991", key, clause, symbol, v, v, unit, f"{note}; as used by the artifact", part="1")
for z, a, b, mn, f in (("z1", .19, .91, .65, 1.0), ("z1a", .19, .91, .65, 1.25), ("z2", .25, 1.91, .85, 1.0), ("z2a", .25, 1.91, .85, 1.25), ("z3", .31, 2.91, 1.10, 1.0)):
    for k, sym, v in (("a", "a", a), ("b", "b", b), ("min", "s_k minimum", mn), ("factor", "zone factor", f)):
        put(d, "1991", f"1-3.snow-zone.{z}.{k}", "NA.4 ground snow", f"s_k zone {z} {sym}", None, v, "kN/m2" if k in ("a", "b", "min") else "", f"NA-DE ground snow s_k = factor max(a + b ((H + offset)/scale)^2, min); EN gives the value in the National Annex; as used by the artifact", part="1")
row("1-3.snow-altitude-offset", "NA.4", "altitude offset", 140.0, "Offset of the altitude term of the NA-DE ground snow formula", "m")
row("1-3.snow-altitude-scale", "NA.4", "altitude scale", 760.0, "Scale of the altitude term of the NA-DE ground snow formula", "m")
row("1-3.exceptional-factor", "NA.4", "s_Ad/s_k", 2.3, "Exceptional snow load factor of the North German lowland")
for a, v in ((0, .8), (30, .8), (60, 0.0)):
    row(f"1-3.mu1.a{a}", "Table 5.2", f"mu_1 at {a} deg", v, "Shape coefficient of pitched roofs, linear between the pitch angles")
row("1-3.mu2.base", "5.3.4", "mu_2 base", .8, "mu_2 = base + slope alpha/alpha_ref of valleys")
row("1-3.mu2.slope", "5.3.4", "mu_2 slope", .8, "mu_2 = base + slope alpha/alpha_ref of valleys")
row("1-3.mu2.alpha-ref", "5.3.4", "mu_2 reference angle", 30.0, "mu_2 = base + slope alpha/alpha_ref of valleys", "deg")
row("1-3.mu2.multi-span-cap", "5.3.4", "mu_2 multi-span cap", 1.6, "Upper bound of mu_2 for multi-span roofs")
row("1-3.mu3.base", "5.3.5", "mu_3 base", .2, "mu_3 = base + slope h/b of cylindrical roofs")
row("1-3.mu3.slope", "5.3.5", "mu_3 slope", 10.0, "mu_3 = base + slope h/b of cylindrical roofs")
for c, z0, zm in (("zero", .003, 1.0), ("i", .01, 1.0), ("ii", .05, 2.0), ("iii", .3, 5.0), ("iv", 1.0, 10.0)):
    row(f"1-4.terrain.{c}.z0", "Table 4.1", f"z_0 category {c}", z0, "Roughness length", "m")
    row(f"1-4.terrain.{c}.zmin", "Table 4.1", f"z_min category {c}", zm, "Minimum height", "m")
row("1-4.k-r.coef", "4.3.2", "k_r coefficient", .19, "k_r = coef (z0/z0_ref)^exp")
row("1-4.k-r.z0-ref", "4.3.2", "k_r reference roughness", .05, "k_r = coef (z0/z0_ref)^exp", "m")
row("1-4.k-r.exp", "4.3.2", "k_r exponent", .07, "k_r = coef (z0/z0_ref)^exp")
for kind, lim1, c1, c2, e2, c3, e3 in (("inland", 7.0, 1.5, 1.7, .37, 2.1, .24), ("coast", 4.0, 1.8, 2.3, .27, 2.6, .19)):
    for k, v in (("limit1", lim1), ("limit2", 50.0), ("c1", c1), ("c2", c2), ("e2", e2), ("c3", c3), ("e3", e3)):
        row(f"1-4.na-profile.{kind}.{k}", "NA.B simplified profile", f"q_p profile {kind} {k}", v, "NA-DE simplified q_p(z)/q_b,0: c1 up to limit1, c2 (z/10)^e2 up to limit2, c3 (z/10)^e3 above", "m" if k.startswith("limit") else "")
WALL = {"A": [(-1.2, -1.4)] * 3, "B": [(-.8, -1.1)] * 3, "C": [(-.5, -.5)] * 3, "D": [(.7, 1.0), (.8, 1.0), (.8, 1.0)], "E": [(-.3, -.3), (-.5, -.5), (-.7, -.7)]}
for z, rows in WALL.items():
    for h, (c10, c1) in zip(("h0-25", "h1", "h5"), rows):
        row(f"1-4.cpe.wall.{z.lower()}.{h}.c10", "Table 7.1", f"c_pe,10 zone {z} h/d {h}", c10, "External pressure coefficient of vertical walls, linear in h/d")
        row(f"1-4.cpe.wall.{z.lower()}.{h}.c1", "Table 7.1", f"c_pe,1 zone {z} h/d {h}", c1, "External pressure coefficient of vertical walls, linear in h/d")
FLAT = {"parapet": [("hp0", .0, [(-1.8, -2.5), (-1.2, -2.0), (-.7, -1.2)]), ("hp0-025", .025, [(-1.6, -2.2), (-1.1, -1.8), (-.7, -1.2)]), ("hp0-05", .05, [(-1.4, -2.0), (-.9, -1.6), (-.7, -1.2)]), ("hp0-10", .10, [(-1.2, -1.8), (-.8, -1.4), (-.7, -1.2)])],
        "curved": [("r0-05", .05, [(-1.0, -1.5), (-1.2, -1.8), (-.4, -.4)]), ("r0-10", .10, [(-.7, -1.2), (-.8, -1.4), (-.3, -.3)]), ("r0-20", .20, [(-.5, -.8), (-.5, -.8), (-.3, -.3)])],
        "mansard": [("a30", 30.0, [(-1.0, -1.5), (-1.0, -1.5), (-.3, -.3)]), ("a45", 45.0, [(-1.2, -1.8), (-1.3, -1.9), (-.4, -.4)]), ("a60", 60.0, [(-1.3, -1.9), (-1.3, -1.9), (-.5, -.5)])]}
for kind, rows in FLAT.items():
    for key, x, zs in rows:
        for z, (c10, c1) in zip("fgh", zs):
            row(f"1-4.cpe.flat.{kind}.{z}.{key}.c10", "Table 7.2", f"c_pe,10 flat roof {kind} zone {z.upper()} {key}", c10, "Flat roof coefficient, linear in the row parameter")
            row(f"1-4.cpe.flat.{kind}.{z}.{key}.c1", "Table 7.2", f"c_pe,1 flat roof {kind} zone {z.upper()} {key}", c1, "Flat roof coefficient, linear in the row parameter")
row("1-4.cpe.flat.i.minus", "Table 7.2", "c_pe flat roof zone I suction", -0.2, "Zone I, both load cases")
row("1-4.cpe.flat.i.plus", "Table 7.2", "c_pe flat roof zone I pressure", 0.2, "Zone I, both load cases")
P = [(-45, [((-.6, -.6), None), ((-.6, -.6), None), ((-.8, -.8), None), ((-.7, -.7), None), ((-1.0, -1.5), None)]),
(-30, [((-1.1, -2.0), None), ((-.8, -1.5), None), ((-.8, -.8), None), ((-.6, -.6), None), ((-.8, -1.4), None)]),
(-15, [((-2.5, -2.8), None), ((-1.3, -2.0), None), ((-.9, -1.2), None), ((-.5, -.5), None), ((-.7, -1.2), None)]),
(-5, [((-2.3, -2.5), None), ((-1.2, -2.0), None), ((-.8, -1.2), None), ((-.3, -.3), None), ((-.3, -.3), None)]),
(5, [((-1.7, -2.5), (0.0, 0.0)), ((-1.2, -2.0), (0.0, 0.0)), ((-.6, -1.2), (0.0, 0.0)), ((-.6, -.6), None), ((-.6, -.6), (.2, .2))]),
(15, [((-.9, -2.0), (.2, .2)), ((-.8, -1.5), (.2, .2)), ((-.3, -.3), (.2, .2)), ((-.4, -.4), None), ((-1.0, -1.5), None)]),
(30, [((-.5, -1.5), (.7, .7)), ((-.5, -1.5), (.7, .7)), ((-.2, -.2), (.4, .4)), ((-.4, -.4), None), ((-.5, -.5), None)]),
(45, [((0.0, 0.0), (.7, .7)), ((0.0, 0.0), (.7, .7)), ((0.0, 0.0), (.6, .6)), ((-.2, -.2), None), ((-.3, -.3), None)]),
(60, [((.7, .7), None), ((.7, .7), None), ((.7, .7), None), ((-.2, -.2), None), ((-.3, -.3), None)]),
(75, [((.8, .8), None), ((.8, .8), None), ((.8, .8), None), ((-.2, -.2), None), ((-.3, -.3), None)])]
for ang, zs in P:
    a = f"m{-ang}" if ang < 0 else f"p{ang}"
    for z, (prim, sec) in zip("fghij", zs):
        for series, pair in (("primary", prim), ("secondary", sec)):
            if pair is None: continue
            for k, v in zip(("c10", "c1"), pair):
                row(f"1-4.cpe.pitched0.{z}.{series}.{a}.{k}", "Tables 7.3a/7.4a", f"c_pe,{k[1:]} pitched roof zone {z.upper()} {series} {ang} deg", v, "Pitched roof coefficient for wind direction 0 deg; the series split into suction and pressure is made by the sign of the primary c_pe,10")
row("1-4.cf.signboard-default", "7.4", "c_f signboard default", 1.8, "Force coefficient of a free-standing signboard when none is given")
for k, v in (("framed", .6), ("core", 1.0), ("cantilever", 1.5), ("lattice", 2.0)):
    row(f"1-4.mode-exponent.{k}", "Annex F", f"mode shape exponent {k}", v, "Exponent zeta of the mode shape Phi = (z/h)^zeta")
for k, v in (("concrete", .10), ("steel", .05), ("mixed", .08)):
    row(f"1-4.delta-s.{k}", "Annex F", f"structural damping delta_s {k}", v, "Logarithmic decrement of structural damping")
save(d); print(len(d))
