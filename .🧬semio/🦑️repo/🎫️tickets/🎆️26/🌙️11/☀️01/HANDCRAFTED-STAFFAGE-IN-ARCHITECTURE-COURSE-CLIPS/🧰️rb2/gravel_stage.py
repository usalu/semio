import sys, importlib, itertools
sys.stdout.reconfigure(encoding="utf-8", errors="backslashreplace")
sys.path.insert(0, "C:/git/semio")
from types import SimpleNamespace
from shapely.geometry import MultiPoint, Polygon
from shapely.ops import unary_union
api = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
model = api.load_model()
tri = Polygon([[8.128, 10.763], [6.79788636, 11.638462621], [8.128, 16.889]])
_slab = api._find(model, "SL-R")
_slab["geometry"]["finish_voids"] = [v for v in _slab["geometry"]["finish_voids"] if v["tag"] == "RES-FIN-ROOF-GRAVEL-SL-R-705"]
gravel = api._find(model, "FIN-ROOF-GRAVEL")
def verts(p):
    return [tuple(float(c) for c in v) for v in api.part_vertices(p)]
def section(p, z):
    vs = verts(p); pts = []
    for a, b in itertools.combinations(vs, 2):
        if (a[2]-z)*(b[2]-z) < 0:
            t = (z-a[2])/(b[2]-a[2]); pts.append((a[0]+t*(b[0]-a[0]), a[1]+t*(b[1]-a[1])))
        elif a[2] == z: pts.append((a[0], a[1]))
    return MultiPoint(pts).convex_hull if len(pts) >= 3 else None
def near(p):
    vs = verts(p); return MultiPoint([(v[0], v[1]) for v in vs]).convex_hull.intersects(tri)
def coverage(label, parts, zs=(6.515, 6.545, 6.575, 6.595)):
    sel = [p for p in parts if near(p)]
    out = []
    for z in zs:
        u = unary_union([s for s in (section(p, z) for p in sel) if s is not None and s.area > 0])
        out.append((z, round(tri.difference(u).area, 6)))
    print(label, len(sel), "uncovered area by z", out, flush=True)
g = gravel["geometry"]
raw = api._flooring_layer_backend().parts(SimpleNamespace(**vars(api)), model, gravel, exact=True)
raw = api._solid_stock_parts(model, gravel, raw, exact=True)
coverage("layer backend", raw)
blockers = api._declared_material_interfaces(model, gravel)
print("blockers", len(blockers), set(b["owner"] for b in blockers))
cut = [cell for part in raw if near(part) for cell in api._reference_material_parts(part, [b for b in blockers])]
coverage("after blockers (raw floor, near only)", cut)
nb = [b for b in blockers if MultiPoint([(v[0], v[1]) for v in verts(b)]).convex_hull.intersects(tri)]
print("blockers near", len(nb))
for b in nb:
    vs = verts(b); xs = [v[0] for v in vs]; ys = [v[1] for v in vs]; zs = [v[2] for v in vs]
    print("  ", b["owner"], b.get("role"), round(min(xs), 4), round(max(xs), 4), round(min(ys), 4), round(max(ys), 4), round(min(zs), 4), round(max(zs), 4))
print("---- gravel cells covering strip")
from shapely.geometry import box as sbox
strip = sbox(8.04, 11.0, 8.07, 12.5)
for p in cut:
    s = section(p, 6.545)
    if s is not None and s.area > 0 and s.intersects(strip) and s.intersection(strip).area > 0:
        vs = verts(p); xs=[v[0] for v in vs]; ys=[v[1] for v in vs]; zs=[v[2] for v in vs]
        print(p.get("role"), p.get("material"), "x", round(min(xs),4), round(max(xs),4), "y", round(min(ys),4), round(max(ys),4), "z", round(min(zs),4), round(max(zs),4), "faces", len(api.part_faces(p)), "area", round(s.intersection(strip).area, 6))
