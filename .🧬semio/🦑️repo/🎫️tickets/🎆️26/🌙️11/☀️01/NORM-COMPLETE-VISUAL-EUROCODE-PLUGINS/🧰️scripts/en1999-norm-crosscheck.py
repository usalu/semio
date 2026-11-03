"""🧮 Third-party cross-checks of the W3-EN1999-NORM values: the gross section of the trapezoidal rib against sectionproperties (shapely outline of the mid-line with thickness) and the protected member temperature against a scipy ODE solution. Run with /home/deck/semio/.venv/bin/python -I."""
import importlib.util, math, sys
import numpy as np
from scipy.integrate import solve_ivp
from shapely.geometry import LineString
from sectionproperties.pre.geometry import Geometry
ORACLE = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪶️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🔮️oracles/⚖️compliance/🐍️.py"
spec = importlib.util.spec_from_file_location("oracle", ORACLE)
oracle = importlib.util.module_from_spec(spec)
spec.loader.exec_module(oracle)
bad = 0


def check(name, ours, theirs, tol):
    global bad
    ok = abs(ours - theirs) <= tol * abs(theirs)
    bad += not ok
    print(("ok  " if ok else "FAIL"), name, f"ours={ours:.6g} library={theirs:.6g} rel={(ours - theirs) / theirs:+.2e} tol={tol:g}")


def rib_polyline(p, top, bot, h):
    proj = (p - top - bot) / 2
    return [(0.0, 0.0), (bot, 0.0), (bot + proj, h), (bot + proj + top, h), (bot + 2 * proj + top, 0.0)]


for (p, top, bot, h, t) in ((0.2, 0.06, 0.06, 0.04, 0.001), (0.25, 0.1, 0.04, 0.05, 0.0015)):
    area, zc, inertia, _, _ = oracle.rib_sections((p, top, bot, h), t, [1.0, 1.0, 1.0], 1500)
    outline = LineString(rib_polyline(p, top, bot, h)).buffer(t / 2, cap_style=2, join_style=2)
    geom = Geometry(outline)
    geom.create_mesh(mesh_sizes=[(t / 3) ** 2])
    from sectionproperties.analysis.section import Section
    sec = Section(geom)
    sec.calculate_geometric_properties()
    cx, cy = sec.get_c()
    check(f"rib {p}/{top}/{bot}/{h}: A", area, sec.get_area(), 1e-6)
    check(f"rib {p}/{top}/{bot}/{h}: z_c", zc, cy, 1e-3)
    check(f"rib {p}/{top}/{bot}/{h}: I_x", inertia, sec.get_ic()[0], 1e-3)
ins = dict(thickness=0.02, conductivity=0.2, specificHeat=1200.0, density=800.0)
av = 200.0
k1, k2 = oracle.ndp("fire-protected-denominator"), oracle.ndp("fire-protected-exponent")


def gdot(t):
    return 345 * 8 / 60 / ((8 * t / 60 + 1) * math.log(10))


def rhs(t, y):
    th = y[0]
    ca = 0.41 * th + 903
    phi = ins["specificHeat"] * ins["density"] / (ca * 2700) * ins["thickness"] * av
    rate = ins["conductivity"] / ins["thickness"] * av / (ca * 2700) * (oracle.gas(t) - th) / (1 + phi / k1) - (math.exp(phi / k2) - 1) * gdot(t)
    return [max(rate, 0.0)]


solved = solve_ivp(rhs, (0, 1800), [20.0], t_eval=[300, 900, 1800], max_step=1.0, rtol=1e-8)
for t, ode in zip((300, 900, 1800), solved.y[0]):
    check(f"protected member after {t} s", oracle.protected_temperature(av, t, 20.0, ins) - 20.0, ode - 20.0, 0.02)
sys.exit(1 if bad else 0)
