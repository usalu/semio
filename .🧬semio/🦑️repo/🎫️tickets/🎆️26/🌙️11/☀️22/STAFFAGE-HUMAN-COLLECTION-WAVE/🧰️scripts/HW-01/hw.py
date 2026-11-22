"""🧰️ HW-01 authoring kit: loads a base human, removes or adds bones with every shape, solid, anchor and binding they carry, scales a
whole asset uniformly, and draws boxy props (desk, laptop, flipchart, marker) in the three views of every scale with matching solids.

The shipped catalog JSON is only ever written by the asset builders in this folder (`build_*.py`), never by hand.
"""
import copy
import json
import math
import os
import subprocess

R = "/Users/ueli/Documents/semio"
S = f"{R}/🎓️teaching/🏛️architecture/🧍️staffage"
CAT = f"{S}/🗂️catalog/👤️humans"
T = f"{R}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE"
W = f"{T}/🧰️scripts/HW-01"
GEN = f"{T}/🗑️generated/HW-01"
BUN = f"{R}/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun"
PK = f"{S}/📦️packages/🟦️typescript"
SCALES = ("50", "100", "200", "500")
VIEWS = ("elevation", "front", "plan")
LISTS = ("body", "thermal", "structure")


def t(en, de):
    return {"en": en, "de": de}


def load(slug):
    with open(f"{CAT}/{slug}/🔣️.json", encoding="utf-8") as f:
        return json.load(f)


def save(doc, path):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w", encoding="utf-8") as f:
        f.write(json.dumps(doc, indent=2, ensure_ascii=False) + "\n")


def r(v, n=5):
    return round(v, n) if isinstance(v, float) else v


def bun(*args, capture=True):
    out = subprocess.run([BUN, *args], cwd=PK, capture_output=capture, text=True)
    if out.returncode != 0:
        raise RuntimeError(f"[DEBUG] bun {' '.join(args)} failed:\n{out.stdout[-4000:]}\n{out.stderr[-4000:]}")
    return out.stdout


def rest_frames(path):
    """🦴️ World frames {bone: {o,u,v,w}} of the rest pose (root at the origin) of the asset JSON at `path`."""
    return json.loads(bun(f"{W}/rest.ts", path))


def bones_of(a):
    return {b["id"]: b for b in a["rig"]["bones"]}


def subtree(a, ids):
    kids = {}
    for b in a["rig"]["bones"]:
        kids.setdefault(b["parent"], []).append(b["id"])
    out, stack = set(), list(ids)
    while stack:
        i = stack.pop()
        if i in out:
            continue
        out.add(i)
        stack.extend(kids.get(i, []))
    return out


# ---------------------------------------------------------------------------------------------------------------- removal


def remove_bones(a, ids, drop_regions=(), drop_materials=()):
    """✂️ Removes the bones `ids` with their subtrees: shapes, solids (and `of` references to them), anchors, offsets in poses and clip keys,
    contact spans, and the named thermal regions with their emitters and bindings."""
    gone = subtree(a, ids)
    a["rig"]["bones"] = [b for b in a["rig"]["bones"] if b["id"] not in gone]
    anchors_gone = {x["id"] for x in a["anchors"] if x["bone"] in gone}
    a["anchors"] = [x for x in a["anchors"] if x["bone"] not in gone]
    for sc, rep in a["representations"].items():
        if rep.get("status") != "authored":
            continue
        for v in VIEWS:
            rep["geometry"][v] = [s for s in rep["geometry"].get(v, []) if s["bone"] not in gone and s.get("region") not in drop_regions]
            for s in rep["geometry"][v]:
                if "regions" in s:
                    s["regions"] = [x for x in s["regions"] if x not in drop_regions]
    for sc, vol in a["volumes"].items():
        if vol.get("status") != "authored":
            continue
        body_gone = {s["id"] for s in vol["body"] if s.get("bone") in gone}
        for L in LISTS:
            keep = []
            for s in vol[L]:
                if s.get("bone") in gone or s.get("of") in body_gone or s.get("region") in drop_regions or s.get("member") in gone:
                    continue
                if s.get("anchor") in anchors_gone:
                    continue
                if "members" in s:
                    s["members"] = [m for m in s["members"] if m not in gone]
                    if not s["members"]:
                        del s["members"]
                keep.append(s)
            vol[L] = keep
    for p in a["poses"]:
        p["bones"] = {k: v for k, v in p["bones"].items() if k not in gone}
        p["contacts"] = [c for c in p["contacts"] if c not in anchors_gone]
        p["loads"] = [l for l in p["loads"] if l.get("anchor") not in anchors_gone and l.get("bone") not in gone]
    for c in a["clips"]:
        for k in c["keys"]:
            if "bones" in k:
                k["bones"] = {b: v for b, v in k["bones"].items() if b not in gone}
        c["contacts"] = [s for s in c["contacts"] if s["anchor"] not in anchors_gone]
    th = a["thermal"]
    th["regions"] = [x for x in th["regions"] if x["id"] not in drop_regions]
    em_gone = {e["id"] for e in th["emitters"] if e.get("anchor") in anchors_gone or e.get("region") in drop_regions}
    th["emitters"] = [e for e in th["emitters"] if e["id"] not in em_gone]
    for p in a["poses"]:
        tb = p["thermal"]
        tb["regions"] = {k: v for k, v in tb["regions"].items() if k not in drop_regions}
        tb["emitters"] = {k: v for k, v in tb["emitters"].items() if k not in em_gone}
    for c in a["clips"]:
        c["tracks"] = [tr for tr in c.get("tracks", []) if not any(tr["target"] == f"emitter:{e}" for e in em_gone) and not any(tr["target"].startswith(f"region:{g}:") for g in drop_regions)]
    a["materials"] = [m for m in a["materials"] if m["id"] not in drop_materials]
    return gone


def used_materials(a):
    used = set()
    for rep in a["representations"].values():
        for v in VIEWS:
            for s in rep.get("geometry", {}).get(v, []):
                if "material" in s:
                    used.add(s["material"])
    for vol in a["volumes"].values():
        for L in LISTS:
            for s in vol.get(L, []):
                if "material" in s:
                    used.add(s["material"])
    for x in a["thermal"]["regions"]:
        used.add(x["material"])
        for l in x.get("layers", []):
            used.add(l["material"])
    return used


def prune_materials(a):
    used = used_materials(a)
    a["materials"] = [m for m in a["materials"] if m["id"] in used]


# ---------------------------------------------------------------------------------------------------------------- uniform scale


def _sc(v, f):
    if isinstance(v, (int, float)) and not isinstance(v, bool):
        return r(v * f, 6)
    if isinstance(v, list):
        return [_sc(x, f) for x in v]
    return v


def scale_asset(a, f, mass_factor):
    """📏️ Scales every length of the asset by `f` (rig, anchors, shapes, 2D solids, 3D solids, offsets, advances, areas by f²) and every
    mass by `mass_factor`; paper quantities (pens, hatches, valid ranges) and physical gauges (layer thicknesses, opening diameters) stay."""
    for b in a["rig"]["bones"]:
        for k in ("x", "y", "z", "lengthM"):
            if k in b:
                b[k] = _sc(b[k], f)
        j = b["joint"]
        for k in ("minM", "maxM"):
            if k in j:
                j[k] = _sc(j[k], f)
        if "mass" in b:
            m = b["mass"]
            m["kg"] = r(m["kg"] * mass_factor, 4)
            for k in ("comX", "comY", "comZ"):
                if k in m:
                    m[k] = _sc(m[k], f)
    for x in a["anchors"]:
        for k in ("x", "y", "z", "footprintLengthM", "footprintWidthM"):
            if k in x:
                x[k] = _sc(x[k], f)
    for rep in a["representations"].values():
        if rep.get("status") != "authored":
            continue
        for v in VIEWS:
            for s in rep["geometry"].get(v, []):
                for k in ("points", "x0", "x1", "y0", "y1", "r0", "r1", "cx", "cy", "rx", "ry", "x", "y", "width", "height"):
                    if k in s:
                        s[k] = _sc(s[k], f)
                if "solid" in s:
                    for k in ("z0", "z1", "zTip", "axisX", "axisZ"):
                        if k in s["solid"]:
                            s["solid"][k] = _sc(s["solid"][k], f)
    for vol in a["volumes"].values():
        if vol.get("status") != "authored":
            continue
        for L in LISTS:
            for s in vol[L]:
                for k in ("size", "cornerRadius", "radii", "p0", "p1", "r0", "r1", "profile", "origin", "holes", "from", "to", "vertices"):
                    if k in s:
                        s[k] = _sc(s[k], f)
                for sec in s.get("sections", []):
                    for k in ("at", "rx", "ry"):
                        sec[k] = _sc(sec[k], f)
                if "transform" in s and "translation" in s["transform"]:
                    s["transform"]["translation"] = _sc(s["transform"]["translation"], f)
                if "repeat" in s:
                    for k in ("translation", "origin"):
                        if k in s["repeat"]:
                            s["repeat"][k] = _sc(s["repeat"][k], f)
    for p in a["poses"]:
        for o in p["bones"].values():
            for k in ("dx", "dy"):
                if k in o:
                    o[k] = _sc(o[k], f)
        if "root" in p:
            for k in ("x", "y"):
                if k in p["root"]:
                    p["root"][k] = _sc(p["root"][k], f)
    for c in a["clips"]:
        if "advanceM" in c:
            c["advanceM"] = _sc(c["advanceM"], f)
        for k in c["keys"]:
            for o in k.get("bones", {}).values():
                for q in ("dx", "dy"):
                    if q in o:
                        o[q] = _sc(o[q], f)
    th = a["thermal"]
    for x in th["regions"]:
        x["areaM2"] = r(x["areaM2"] * f * f, 4)
        if "characteristicLengthM" in x:
            x["characteristicLengthM"] = _sc(x["characteristicLengthM"], f)
    resp = th.get("respiration")
    if resp:
        for mo in resp.get("motion", []):
            if "dy" in mo:
                mo["dy"] = _sc(mo["dy"], f)


# ---------------------------------------------------------------------------------------------------------------- props


def rect(u0, u1, v0, v1):
    return [[r(u0), r(v0)], [r(u1), r(v0)], [r(u1), r(v1)], [r(u0), r(v1)]]


def poly(id, bone, pts, z, tone, region=None, material=None, role="body", filled=True, closed=True, **extra):
    s = {"id": id, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone}
    if region:
        s["region"] = region
    if material:
        s["material"] = material
    s.update({"kind": "poly", "points": pts, "closed": closed})
    s.update(extra)
    return s


def world_of(frame, p):
    o, u, v, w = frame["o"], frame["u"], frame["v"], frame["w"]
    return [o[k] + p[0] * u[k] + p[1] * v[k] + p[2] * w[k] for k in range(3)]


def box_corners(size, at):
    hu, hv, hw = size[0] / 2, size[1] / 2, size[2] / 2
    return [[at[0] + a * hu, at[1] + b * hv, at[2] + c * hw] for a in (-1, 1) for b in (-1, 1) for c in (-1, 1)]


def hull2(points):
    pts = sorted(set((round(p[0], 6), round(p[1], 6)) for p in points))
    if len(pts) < 3:
        return [list(p) for p in pts]

    def cross(o, a, b):
        return (a[0] - o[0]) * (b[1] - o[1]) - (a[1] - o[1]) * (b[0] - o[0])

    lower, upper = [], []
    for p in pts:
        while len(lower) >= 2 and cross(lower[-2], lower[-1], p) <= 0:
            lower.pop()
        lower.append(p)
    for p in reversed(pts):
        while len(upper) >= 2 and cross(upper[-2], upper[-1], p) <= 0:
            upper.pop()
        upper.append(p)
    return [[r(x), r(y)] for x, y in lower[:-1] + upper[:-1]]


class Part:
    """🧱️ One rectangular prop part in bone-local metres (`size` [u, v, w] centred at `at`), drawn in all three views and modelled as a box.
    `views` limits the drawn views; `solid` False draws without a 3D twin (an inner line)."""

    def __init__(self, id, bone, size, at, region, material, tone, z=None, role="body", filled=True, corner=0.0, views=VIEWS, solid=True, sheet=False, slender=False, penMm=None):
        self.id, self.bone, self.size, self.at, self.region, self.material, self.tone = id, bone, size, at, region, material, tone
        self.z, self.role, self.filled, self.corner, self.views, self.solid, self.sheet, self.slender, self.penMm = z, role, filled, corner, views, solid, sheet, slender, penMm

    def shapes(self, frames, z):
        f = frames[self.bone]
        out = []
        zv = getattr(self, "zv", {})
        zz = self.z if self.z is not None else z
        extra = {"penMm": self.penMm} if self.penMm is not None else {}
        u0, u1 = self.at[0] - self.size[0] / 2, self.at[0] + self.size[0] / 2
        v0, v1 = self.at[1] - self.size[1] / 2, self.at[1] + self.size[1] / 2
        if "elevation" in self.views:
            out.append(("elevation", poly(self.id, self.bone, rect(u0, u1, v0, v1), zv.get("elevation", zz), self.tone, self.region if self.filled else None, self.material, self.role, self.filled, **extra)))
        if getattr(self, "swept", False):
            zc = f["o"][2] + self.at[2]
            hw2 = getattr(self, "draw_w", self.size[2]) / 2
            solid = {"kind": "extruded", "sweep": "w", "z0": r(zc - hw2), "z1": r(zc + hw2)}
            for view in ("front", "plan"):
                if view in self.views:
                    out.append((view, poly(self.id, self.bone, rect(u0, u1, v0, v1), zv.get(view, zz), self.tone, self.region if self.filled else None, self.material, self.role, self.filled, solid=solid, **extra)))
            return out
        corners = [world_of(f, c) for c in box_corners(self.size, self.at)]
        if "front" in self.views:
            out.append(("front", poly(self.id, self.bone, hull2([[c[2], c[1]] for c in corners]), zv.get("front", zz), self.tone, self.region if self.filled else None, self.material, self.role, self.filled, **extra)))
        if "plan" in self.views:
            out.append(("plan", poly(self.id, self.bone, hull2([[c[0], c[2]] for c in corners]), zv.get("plan", zz), self.tone, self.region if self.filled else None, self.material, self.role, self.filled, **extra)))
        return out

    def body(self):
        s = {"id": self.id, "kind": "box", "bone": self.bone, "material": self.material, "role": self.role, "tone": self.tone, "size": [r(x) for x in self.size]}
        if self.corner:
            s["cornerRadius"] = self.corner
        s["transform"] = {"translation": [r(x) for x in self.at]}
        if self.sheet:
            s["sheet"] = True
        if self.slender:
            s["slender"] = True
        return s


def add_parts(a, scale, parts, frames, z0):
    """➕️ Adds the parts to the representation and the body volume of `scale`; returns the body solid ids."""
    rep = a["representations"][scale]
    ids = []
    for i, p in enumerate(parts):
        for view, shape in p.shapes(frames, z0 + i * 0.01):
            rep["geometry"][view].append(shape)
        if p.solid:
            a["volumes"][scale]["body"].append(p.body())
            ids.append(p.id)
    return ids


def merge_texts(base, extra):
    return {"en": f"{base['en']} {extra['en']}".strip(), "de": f"{base['de']} {extra['de']}".strip()}


def dubois(mass, height):
    return 0.007184 * mass ** 0.425 * (height * 100) ** 0.725


def mifflin(mass, height, age, male):
    return (10 * mass + 6.25 * height * 100 - 5 * age + (5 if male else -161)) * 4.184e3 / 86400


def manifest_row(slug, distinction):
    """📥️ The manifest entry of the catalog asset `slug` in the shape of the existing `entries` rows, values from the evaluated asset."""
    path = f"{CAT}/{slug}/🔣️.json"
    a = json.load(open(path, encoding="utf-8"))["asset"]
    dims = json.loads(bun(f"{W}/dims.ts", path))
    row = {
        "id": a["id"], "category": a["category"], "title": a["title"],
        "heightM": a["dimensions"]["heightM"], "widthM": a["dimensions"]["widthM"], "powerW": dims["powerW"],
        "poses": [p["id"] for p in a["poses"] if not p.get("key")], "clips": [c["id"] for c in a["clips"]],
        "representations": {sc: a["representations"][sc]["status"] for sc in SCALES},
        "symbols": [s["id"] for s in a["symbols"]], "analytics": ["thermal", "transport", "structural"],
        "exports": a["exports"], "distinction": distinction, "completion": a["completion"],
    }
    save(row, f"{T}/📥️manifest-rows/{slug}.json")
    return row


def sweep_from_elevation(a, bones):
    """🧊️ Redraws the front and plan shapes of `bones` as prisms: the elevation outline of the same id (bone-local u, v) swept across
    the lateral range the old front (or plan) shape covered at rest, so a forearm and hand that leave the hanging rest pose read right
    in all three views (`solid.extruded`, sweep `w`). Shapes without an elevation twin keep their drawing."""
    for sc, rep in a["representations"].items():
        if rep.get("status") != "authored":
            continue
        g = rep["geometry"]
        elev = {(s["bone"], s["id"]): s for s in g["elevation"]}
        for view, axis in (("front", 0), ("plan", 1)):
            for s in g[view]:
                if s["bone"] not in bones or "solid" in s:
                    continue
                twin = elev.get((s["bone"], s["id"]))
                if twin is None or twin.get("kind") != "poly" or s.get("kind") != "poly" or not twin.get("closed", True):
                    continue
                lat = [p[axis] for p in s["points"]]
                s["points"] = [list(p) for p in twin["points"]]
                s["solid"] = {"kind": "extruded", "sweep": "w", "z0": r(min(lat)), "z1": r(max(lat))}


def apply_ops(a, slug, keep_solids=()):
    """🩹️ Applies the silhouette repair ops of `ops/<slug>-<scale>.json` (written by the guard's greedy trace repair, humansb-D-auto.ts of
    ticket STAFFAGE-CATALOG-SILHOUETTE-GUARD-PASSES-FOR-EVERY-ASSET): a drawn shape replaced by, or completed with, the traced
    outline of its bone's solids in the default pose, expressed in the rest frame of the bone; then the handcrafted ops of
    `ops-manual/<slug>.json` (also `shape-delete`). A flat front or plan trace the tool hung on a bone without solids (a clavicle,
    which no pose moves against its parent) is hung on that parent, where it draws the same; a shape with a `solid` on a bone of
    `keep_solids` (one that a pose turns out of its rest plane) keeps its solid drawing. These files are design inputs."""
    import glob
    parents = {b["id"]: b["parent"] for b in a["rig"]["bones"]}
    solid_bones = {s.get("bone") for vol in a["volumes"].values() for s in vol.get("body", [])}
    for path in sorted(glob.glob(f"{W}/ops/{slug}-*.json")) + sorted(glob.glob(f"{W}/ops-manual/{slug}.json")):
        for o in json.load(open(path, encoding="utf-8")):
            sh = o.get("shape")
            if sh is not None and o["view"] != "elevation" and sh["bone"] not in solid_bones:
                sh["bone"] = parents[sh["bone"]]
            g = a["representations"][o["scale"]]["geometry"][o["view"]]
            if o["op"] == "shape-add":
                g.insert(next(i for i, x in enumerate(g) if x["id"] == o["after"]) + 1, o["shape"])
            elif o["op"] == "shape":
                i = next(i for i, x in enumerate(g) if x["id"] == o["id"])
                if "solid" in g[i] and "solid" not in o["shape"] and g[i]["bone"] in keep_solids:
                    continue
                g[i] = o["shape"]
            elif o["op"] == "shape-set":
                next(x for x in g if x["id"] == o["id"]).update(o["set"])
            elif o["op"] == "shape-delete":
                g[:] = [x for x in g if x["id"] != o["id"]]


def sweep_trace(a, frames, scales, shape_id, front_trace, depth):
    """🧊️ Redraws the front and plan shape `shape_id` as a prism along the bone normal (`solid.extruded`, sweep `v`): the profile is
    the front trace of the body (rest-world lateral, height) re-expressed in the bone's own (u, w) plane, swept over the forward range
    `depth` (bone-local v, the sleeve's own thickness at rest). A sleeve whose revolved drawing is round while the body is oval then reads with its true width and stays
    right when a pose swings the arm."""
    for scale in scales:
        g = a["representations"][scale]["geometry"]
        for view in ("front", "plan"):
            for s in g[view]:
                if s["id"] != shape_id:
                    continue
                f = frames[s["bone"]]
                o, u, v, w = f["o"], f["u"], f["v"], f["w"]
                pts = []
                for zz, yy in front_trace:
                    d = [0.0, yy - o[1], zz - o[2]]
                    pts.append([r(sum(d[k] * u[k] for k in range(3))), r(sum(d[k] * w[k] for k in range(3)))])
                s.update({"kind": "poly", "points": pts, "closed": True, "solid": {"kind": "extruded", "sweep": "v", "z0": r(depth[0]), "z1": r(depth[1])}})


def sweep_shape(a, scale, shape_id, z0, z1):
    """🧊️ Redraws the front and plan shape `shape_id` of one scale as its elevation outline swept across the rest lateral range z0..z1
    (`solid.extruded`, sweep `w`): a sleeve whose revolved drawing is round while the body is oval reads with the true width."""
    g = a["representations"][scale]["geometry"]
    twin = next(s for s in g["elevation"] if s["id"] == shape_id)
    for view in ("front", "plan"):
        for s in g[view]:
            if s["id"] == shape_id:
                s.update({"kind": "poly", "points": [list(p) for p in twin["points"]], "closed": True, "solid": {"kind": "extruded", "sweep": "w", "z0": r(z0), "z1": r(z1)}})
