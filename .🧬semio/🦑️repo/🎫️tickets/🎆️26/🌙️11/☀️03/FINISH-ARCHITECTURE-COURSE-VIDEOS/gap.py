"""🔬 Measure the exact unoccupied solid of one roof-gravel finish reserve."""
import importlib, sys, time
from fractions import Fraction
sys.stdout.reconfigure(encoding="utf-8", errors="backslashreplace")
sys.path.insert(0, r"C:\git\semio")
api = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
TAG = "RES-FIN-ROOF-GRAVEL-SL-R-705"
started = time.time()

def log(*items):
    """📣 Print one timed probe line."""
    print(round(time.time() - started, 1), *items, flush=True)

def vol(parts):
    """🎲 Sum exact native cell volume."""
    return api._native_parts_exact_volume(parts)

def box(part):
    """📦 Report float bounds of one native cell."""
    vs = [tuple(float(c) for c in p) for p in api.part_vertices(part)]
    return tuple(round(f(v[k] for v in vs), 5) for k in range(3) for f in (min, max))

def overlap(a, b):
    """🤝 Reject only exact disjoint native bounding intervals."""
    return all(x[0] < y[1] and y[0] < x[1] for x, y in zip(api._material_bounds(a), api._material_bounds(b)))

def contains(outer, inner):
    """🥡 Prove complete native-cell containment without any geometric tolerance."""
    from fractions import Fraction as Q
    def planes(part):
        vertices = [tuple(Q(v) for v in p) for p in api.part_vertices(part)]
        found = []
        for face in api.part_faces(part):
            a = vertices[face[0]]
            for i in range(1, len(face) - 1):
                u, v = [tuple(vertices[j][k] - a[k] for k in range(3)) for j in (face[i], face[i + 1])]
                n = (u[1] * v[2] - u[2] * v[1], u[2] * v[0] - u[0] * v[2], u[0] * v[1] - u[1] * v[0])
                if any(n):
                    break
            offset = sum(n[k] * a[k] for k in range(3))
            values = [sum(n[k] * p[k] for k in range(3)) - offset for p in vertices]
            if max(values) > 0:
                n = tuple(-c for c in n)
                offset = -offset
            found.append((n, offset))
        return found, [tuple(Q(v) for v in p) for p in api.part_vertices(inner)]
    half, pts = planes(outer)
    return all(sum(n[k] * p[k] for k in range(3)) <= offset for n, offset in half for p in pts)

def intersect(left, right):
    """🎯 Intersect original convex native cells before publishing any clipped coordinate."""
    if not overlap(left, right):
        return []
    if contains(right, left):
        return [left]
    if contains(left, right):
        kept = {k: v for k, v in left.items() if k not in ("outline", "holes", "top_outline", "section", "mesh", "z0", "z1")}
        return [{**kept, "mesh": right["mesh"]}]
    return api._reference_material_parts(left, [], [right])

model = api.load_model()
slab = api._find(model, "SL-R")
reserve = next(row for row in slab["geometry"]["finish_voids"] if row["tag"] == TAG)
slab["geometry"]["finish_voids"] = []
log("building slab finish host")
geom = api._slab_geometry_uncached(model, slab, exact=True)
host = geom["finish_parts"]
log("host parts", len(host), "volume", vol(host))
slab["geometry"]["finish_voids"] = [reserve]
low, high = min(p["z0"] for p in host), max(p["z1"] for p in host)
cells = []
for part in host:
    material = part.get("material")
    if material is None:
        layers = api._material(model, slab)
        material = next((layer["material"] for layer in layers.get("layers", ()) if layer["id"] == part.get("layer")), None)
    cells.extend(api._native_material_cells({**part, "material": material, "role": part.get("role", "finish")}))
bounds = api._native_material_cells({"outline": api._expand(model, reserve["outline"]), "z0": low, "z1": high, "material": cells[0]["material"], "role": "finish_reserve_extent"})
targets = [cell for part in cells for boundary in bounds for cell in intersect(part, boundary)]
log("targets", len(targets), "volume", vol(targets), "z", float(low), float(high))
log("owner solid")
owner = api._find(model, reserve["owner"])
solid = api._solid_geometry(model, owner, resolve_embedded=False, exact=True)
log("owner parts", len(solid["parts"]), "volume", vol(solid["parts"]))
owners = [reserve["owner"], *owner["geometry"].get("interface_owners", ())]
common = []
for tag in owners:
    obj = api._find(model, tag)
    if tag == reserve["owner"]:
        native = solid["parts"]
    elif obj["class"] in ("IfcRamp", "IfcRampFlight", "IfcColumn", "IfcSlab"):
        native = api._interface_owner_parts(model, tag)
    else:
        native = api._solid_geometry(model, obj, resolve_embedded=False, exact=True)["parts"]
    native = [cell for part in native for cell in api._native_material_cells(part)]
    owned = []
    for target in targets:
        for part in native:
            if not overlap(part, target):
                continue
            for cell in intersect(part, target):
                prior = [other for other in common if other["owner"] == tag and overlap(cell, other)]
                retained = api._reference_material_parts(cell, prior) if prior else [cell]
                for item in retained:
                    row = {**item, "owner": tag}
                    common.append(row)
                    owned.append(row)
    log("owner", tag, "cells", len(owned), "volume", vol(owned))
missing = vol(targets) - vol(common)
log("missing", missing, float(missing))
gap = [cell for target in targets for cell in api._reference_material_parts(target, common)]
log("gap cells", len(gap), "volume", vol(gap))
for cell in gap:
    log("gap", cell.get("owner"), cell.get("role"), cell.get("material"), box(cell), "volume", float(vol([cell])))
