import sys, importlib, itertools
sys.stdout.reconfigure(encoding="utf-8", errors="backslashreplace")
sys.path.insert(0, "C:/git/semio")
from shapely.geometry import MultiPoint
from shapely.ops import unary_union
api = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
model = api.load_model()
guard = api._find(model, "GUARD-RM-2")
print("guard geometry keys", list(guard["geometry"]), "storey", guard["storey"])
def info(label, parts):
    rows = []
    for p in parts:
        vs = [tuple(float(c) for c in v) for v in api.part_vertices(p)]
        xs, ys, zs = [v[0] for v in vs], [v[1] for v in vs], [v[2] for v in vs]
        if max(xs) > 7.9 and min(xs) < 8.2 and max(zs) > 6.4 and min(zs) < 6.7:
            rows.append((p.get("role"), p.get("material"), "x", round(min(xs), 4), round(max(xs), 4), "y", round(min(ys), 4), round(max(ys), 4), "z", round(min(zs), 4), round(max(zs), 4), "faces", len(api.part_faces(p))))
    print(label, len(parts), "near region", len(rows))
    for r in rows[:30]:
        print("  ", r)
info("solid exact=False", api._solid_geometry(model, guard, resolve_embedded=False)["parts"])
info("solid exact=True", api._solid_geometry(model, guard, resolve_embedded=False, exact=True)["parts"])
info("interface_owner_parts", api._interface_owner_parts(model, "GUARD-RM-2"))
