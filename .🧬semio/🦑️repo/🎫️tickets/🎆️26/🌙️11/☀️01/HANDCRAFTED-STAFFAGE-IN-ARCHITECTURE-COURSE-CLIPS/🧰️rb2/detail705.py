import sys, importlib
from fractions import Fraction
sys.stdout.reconfigure(encoding="utf-8", errors="backslashreplace")
sys.path.insert(0, "C:/git/semio")
from shapely.geometry import MultiPoint, Polygon
from shapely.ops import unary_union
api = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
model = api.load_model()
tag = sys.argv[1] if len(sys.argv) > 1 else "RES-FIN-ROOF-GRAVEL-SL-R-705"
slab = api._find(model, "SL-R")
v = next(x for x in slab["geometry"]["finish_voids"] if x["tag"] == tag)
tri = Polygon(v["outline"])
calls = []
orig = api._native_parts_exact_volume
def spy(parts):
    calls.append(list(parts))
    return orig(parts)
api._native_parts_exact_volume = spy
slab["geometry"]["finish_voids"] = [v]
try:
    api._slab_geometry_uncached(model, slab, exact=True)
except Exception as e:
    print("ERR", str(e)[:120])
expected, common = calls[-2], calls[-1]
def foot(part):
    vs = [tuple(float(c) for c in p) for p in api.part_vertices(part)]
    return vs
def bands(parts):
    out = {}
    for p in parts:
        vs = foot(p)
        z0, z1 = round(min(x[2] for x in vs), 6), round(max(x[2] for x in vs), 6)
        out.setdefault((z0, z1), []).append(MultiPoint([(x[0], x[1]) for x in vs]).convex_hull)
    return out
eb, cb = bands(expected), bands(common)
print("expected bands", {k: (len(x), round(sum(g.area for g in x), 6)) for k, x in eb.items()})
print("common bands", {k: (len(x), round(sum(g.area for g in x), 6)) for k, x in cb.items()})
print("tri", tri.area)
for k, x in cb.items():
    u = unary_union(x)
    d = tri.difference(u)
    print(k, "union area", u.area, "uncovered", d.area, d.wkt[:400])
    print(" overlap sum", sum(g.area for g in x) - u.area)
for k, x in eb.items():
    u = unary_union(x)
    print("exp", k, "union", u.area, "tri-u", tri.difference(u).area, "u-tri", u.difference(tri).area)
print("---- common cells")
for p in common:
    vs = foot(p)
    xs=[a[0] for a in vs]; ys=[a[1] for a in vs]; zs=[a[2] for a in vs]
    print(p.get("owner"), p.get("role"), p.get("material"), "x", round(min(xs),4), round(max(xs),4), "y", round(min(ys),4), round(max(ys),4), "z", round(min(zs),4), round(max(zs),4))
print("---- slices")
import itertools
def section(p, z):
    vs = foot(p); pts = []
    for a, b in itertools.combinations(vs, 2):
        if (a[2]-z)*(b[2]-z) < 0:
            t = (z-a[2])/(b[2]-a[2]); pts.append((a[0]+t*(b[0]-a[0]), a[1]+t*(b[1]-a[1])))
        elif a[2] == z: pts.append((a[0], a[1]))
    return MultiPoint(pts).convex_hull if len(pts) >= 3 else None
for z in (6.515, 6.545, 6.565, 6.575, 6.585, 6.595):
    e = unary_union([s for s in (section(p, z) for p in expected) if s is not None and s.area > 0])
    parts_by_owner = {}
    for p in common:
        s = section(p, z)
        if s is not None and s.area > 0: parts_by_owner.setdefault(p["owner"], []).append(s)
    c = unary_union([s for l in parts_by_owner.values() for s in l])
    d = e.difference(c)
    print(z, "exp", round(e.area, 6), "common", round(c.area, 6), "missing", round(d.area, 8), d.wkt[:300])
    for o, l in parts_by_owner.items():
        print("   ", o, round(unary_union(l).area, 6))
    g = unary_union(parts_by_owner.get("GUARD-RM-2", [Polygon()])); w = unary_union(parts_by_owner.get("FIN-ROOF-GRAVEL", [Polygon()]))
    print("    guard/gravel overlap", g.intersection(w).area)
