"""🌸️ PW-12 shrub kit: turns a hand-authored species description (lilac.py, hydrangea.py, rhododendron.py) into a `semio.staffage/v2`
asset document. Every stem, lobe, flower, pose and text is chosen by hand in the species file; this module only converts world rest
coordinates (x ahead, y up, z left) into bone-local ones, projects the authored 3D primitives of each scale into the three planar views
of the same scale, serialises JSON and reuses the NW-V1 vegetation helpers (wind drag, lean, sway, gust and diurnal clips) from their place.

@see ../../../../🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE/🧰️scripts/NW-V1/common.py
@see ../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/README.md"""
import json
import math
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, *[".."] * 9))
NW_V1 = os.path.join(REPO, ".🧬semio", "🦑️repo", "🎫️tickets", "🎆️26", "🌙️10", "☀️29", "ARCHITECTURAL-STAFFAGE-ARCHIVE", "🧰️scripts", "NW-V1")
sys.path.insert(0, NW_V1)
from lib import T, D2R, wind_force  # noqa: E402
import common as nw  # noqa: E402

CATALOG = os.path.join(REPO, "🎓️teaching", "🏛️architecture", "🧍️staffage", "🗂️catalog", "🌳️vegetation")
TICKET = os.path.abspath(os.path.join(HERE, "..", ".."))
SCALES = ["50", "100", "200", "500"]
RANGES = {"500": (350, 1000), "200": (150, 350), "100": (75, 150), "50": (20, 75)}
PENS = {
    "500": {"body": 0.15, "detail": None, "texture": None, "glazing": None, "ground": 0.25},
    "200": {"body": 0.2, "detail": 0.13, "texture": None, "glazing": None, "ground": 0.25},
    "100": {"body": 0.2, "detail": 0.13, "texture": 0.1, "glazing": None, "ground": 0.3},
    "50": {"body": 0.25, "detail": 0.18, "texture": 0.13, "glazing": None, "ground": 0.35},
}
HATCHES = {
    "500": [], "200": [],
    "100": [{"id": "shade-lines", "title": T("Shade line hatch of the inner crown", "Schattenschraffur der Innenkrone"), "pattern": "lines", "angleDeg": 45, "spacingMm": 0.6, "penMm": 0.1}],
    "50": [{"id": "shade-lines", "title": T("Shade line hatch of the inner crown", "Schattenschraffur der Innenkrone"), "pattern": "lines", "angleDeg": 45, "spacingMm": 0.8, "penMm": 0.1}],
}
EXPORTS = ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"]
LIMB = [0.5, 0.5, 0.5, 0.5]
SUPPORT_T = {"50": 0.06, "100": 0.065, "200": 0.07, "500": 0.08}


def r(v, n=4):
    """🔢️ Rounds to `n` decimals without negative zero."""
    x = round(v, n)
    return 0.0 if x == 0 else x


class Bone:
    """🦴️ A sagittal bone in world rest coordinates: origin `o` (x, y, z), absolute angle `ang` (deg, CCW from +x) and length `L`."""

    def __init__(self, id, parent, o, ang, L, title, kind, limit, level):
        self.id, self.parent, self.o, self.ang, self.L, self.title, self.kind, self.limit, self.level = id, parent, o, ang, L, title, kind, limit, level
        self.d0 = self.d1 = None
        self.material = None
        self.kg = None
        self.com = None
        self.dist = "uniform"
        self.ref = "wood-handbook-2021"
        self.source = "authored-estimate"
        self.seg3 = None

    def local(self, p):
        """➡️ World point (x, y, z) to bone-local (u, v, w)."""
        dx, dy = p[0] - self.o[0], p[1] - self.o[1]
        c, s = math.cos(self.ang * D2R), math.sin(self.ang * D2R)
        return (dx * c + dy * s, -dx * s + dy * c, p[2] - self.o[2])

    def local2(self, p):
        u, v, _ = self.local((p[0], p[1], self.o[2]))
        return (u, v)

    def world(self, u, v=0.0, w=0.0):
        c, s = math.cos(self.ang * D2R), math.sin(self.ang * D2R)
        return (self.o[0] + u * c - v * s, self.o[1] + u * s + v * c, self.o[2] + w)


class Rig:
    """🌿️ The ordered bones of one shrub."""

    def __init__(self):
        self.bones, self.order = {}, []

    def add(self, id, parent, o, ang, L, title, kind, limit, level):
        b = Bone(id, parent, o, ang, L, title, kind, limit, level)
        self.bones[id] = b
        self.order.append(id)
        return b

    def json(self):
        out = []
        for id in self.order:
            b = self.bones[id]
            if b.parent is None:
                d = {"id": id, "parent": None, "title": b.title, "x": r(b.o[0]), "y": r(b.o[1]), "angleDeg": r(b.ang, 3), "lengthM": r(b.L, 4), "joint": {"type": "fixed"}}
            else:
                p = self.bones[b.parent]
                x, y = p.local2(b.o)
                d = {"id": id, "parent": b.parent, "title": b.title, "x": r(x), "y": r(y)}
                if abs(b.o[2] - p.o[2]) > 1e-6:
                    d["z"] = r(b.o[2] - p.o[2])
                d.update({"angleDeg": r(b.ang - p.ang, 3), "lengthM": r(b.L, 4), "joint": {"type": "revolute", "minDeg": -b.limit, "maxDeg": b.limit}})
            if b.kg is not None:
                cx, cy = b.com if b.com else (b.L / 2, 0.0)
                d["mass"] = {"kg": r(b.kg, 2), "comX": r(cx, 3), "comY": r(cy, 3), "distribution": b.dist, "source": b.source, "reference": b.ref}
            if b.d0 is not None and b.material:
                dm = b.d0
                d["section"] = {"areaM2": float("%.5g" % (math.pi * dm * dm / 4)), "secondMomentM4": float("%.4g" % (math.pi * dm ** 4 / 64)), "extremeFibreM": r(dm / 2, 4), "material": b.material,
                                "title": T("%s, d = %.0f mm" % (b.title["en"], dm * 1000), "%s, d = %.0f mm" % (b.title["de"], dm * 1000))}
            out.append(d)
        return out


def build_rig(sp):
    """🌳️ Root collar, stem segments (polyline per stem) and short horizontal foliage bones on the stems."""
    rig = Rig()
    rc = sp["collar"]
    root = rig.add("root-plate", None, (0.0, 0.0, 0.0), 90.0, rc["h"], sp["collar"]["title"], "root", 0, "root")
    root.d0 = root.d1 = 2 * rc["r1"]
    root.material = sp["wood"]
    for st in sp["stems"]:
        pts = st["pts"]
        parent = "root-plate"
        for i in range(len(pts) - 1):
            p0, p1 = pts[i], pts[i + 1]
            ang = math.degrees(math.atan2(p1[1] - p0[1], p1[0] - p0[0]))
            L = math.hypot(p1[0] - p0[0], p1[1] - p0[1])
            sid = "%s-%s" % (st["id"], "abc"[i])
            seg = ["lower part", "upper part", "top"][i] if len(pts) > 2 else "stem"
            segde = ["unterer Teil", "oberer Teil", "Spitze"][i] if len(pts) > 2 else "Trieb"
            b = rig.add(sid, parent, p0, ang, L, T("%s, %s" % (st["title"]["en"], seg), "%s, %s" % (st["title"]["de"], segde)), "wood", st["limits"][i], "stem" if i == 0 else "stemtop")
            b.d0, b.d1 = st["d"][i], st["d"][i + 1]
            b.material = sp["wood"]
            b.seg3 = (p0, p1)
            parent = sid
    for f in sp["foliage"]:
        par = rig.bones[f["parent"]]
        p0, p1 = par.seg3 if par.seg3 else ((0.0, 0.0, 0.0), (0.0, par.L, 0.0))
        t = f.get("at", 1.0)
        o = tuple(p0[k] + (p1[k] - p0[k]) * t for k in range(3))
        b = rig.add(f["id"], f["parent"], o, f["ang"], sp["foliage_len"], f["title"], "foliage", f.get("limit", 15), f.get("level", "fol"))
        b.dist = "point"
        b.ref = sp["leaf_ref"]
    return rig


#region outlines
def scallop(cx, cy, rx, ry, k, amp, phase=0.0, n=None):
    """☁️ A closed scalloped ellipse whose radius swings ±amp/2 about the true ellipse (k scallops), so its mean is the ellipsoid outline."""
    n = n or max(48, 6 * k)
    pts = []
    for i in range(n):
        phi = 2 * math.pi * i / n
        f = 1.0 + amp * (abs(math.sin(k * phi / 2 + phase)) ** 0.7 - 0.6) if k else 1.0
        pts.append((cx + rx * f * math.cos(phi), cy + ry * f * math.sin(phi)))
    return pts


def capsule_ring(a, b, r0, r1, n=10):
    """💊️ The tangent hull of two circles as a ring (for own previews and extents)."""
    dx, dy = b[0] - a[0], b[1] - a[1]
    L = math.hypot(dx, dy) or 1e-9
    ang = math.atan2(dy, dx)
    s = max(-1.0, min(1.0, (r0 - r1) / L))
    al = math.acos(s)
    pts = []
    for i in range(n + 1):
        t = ang + al + (2 * math.pi - 2 * al) * i / n
        pts.append((a[0] + r0 * math.cos(t), a[1] + r0 * math.sin(t)))
    for i in range(n + 1):
        t = ang - al + 2 * al * i / n
        pts.append((b[0] + r1 * math.cos(t), b[1] + r1 * math.sin(t)))
    return pts
#endregion outlines


#region primitives
class Lobe:
    """🍃️ A foliage or flower ellipsoid, axis-aligned in the world at rest, bound to a horizontal bone. Leaf masses are drawn as the true
    ellipse of their ellipsoid in every view (the drawing and the volume are one body to within a smallest feature); only flower heads keep
    the small floret scallops of `k` and `amp`, which stay within that feature."""

    def __init__(self, id, bone, c, rad, region, material, tone, k=0, amp=0.0, phase=0.0, hatch=None, role="body", texture=None):
        self.id, self.bone, self.c, self.rad, self.region, self.material, self.tone = id, bone, c, rad, region, material, tone
        self.k, self.amp, self.phase, self.hatch, self.role, self.texture = k, amp, phase, hatch, role, texture
        if region != "flowers":
            self.k, self.amp = 0, 0.0


class Cap:
    """🪵️ A tapered capsule between two world points (stem, panicle), bound to a bone."""

    def __init__(self, id, bone, p0, p1, r0, r1, region, material, tone, role="body", slender=None):
        self.id, self.bone, self.p0, self.p1, self.r0, self.r1, self.region, self.material, self.tone, self.role, self.slender = id, bone, p0, p1, r0, r1, region, material, tone, role, slender


def lobe_views(lb, b, zi):
    """✏️ The elevation (bone-local), front (z, y) and plan (x, z) outlines of a world-aligned ellipsoid lobe."""
    (cx, cy, cz), (rx, ry, rz) = lb.c, lb.rad
    u, v, _ = b.local(lb.c)
    flip = math.cos(b.ang * D2R) < 0
    e = scallop(u, v, rx, ry, lb.k, lb.amp, lb.phase + (math.pi if flip else 0))
    f = scallop(cz, cy, rz, ry, lb.k, lb.amp, lb.phase + 0.7)
    p = scallop(cx, cz, rx, rz, lb.k, lb.amp, lb.phase + 1.3)
    base = {"bone": b.id, "role": lb.role, "z": zi, "filled": True, "tone": lb.tone, "region": lb.region, "material": lb.material}
    if getattr(lb, "regions", None):
        base.pop("region")
        base["regions"] = lb.regions
    if lb.hatch:
        base["hatch"] = lb.hatch
    out = []
    for view, pts in (("elevation", e), ("front", f), ("plan", p)):
        s = {"id": lb.id, **base, "kind": "poly", "closed": True, "points": [[r(x), r(y)] for x, y in pts]}
        out.append((view, s))
    return out


def cap_views(c, b, zi):
    u0, v0, _ = b.local(c.p0)
    u1, v1, _ = b.local(c.p1)
    base = {"bone": b.id, "role": c.role, "z": zi, "filled": True, "tone": c.tone, "region": c.region, "material": c.material, "kind": "capsule"}
    e = {"id": c.id, **base, "x0": r(u0), "y0": r(v0), "x1": r(u1), "y1": r(v1), "r0": r(c.r0), "r1": r(c.r1)}
    f = {"id": c.id, **base, "x0": r(c.p0[2]), "y0": r(c.p0[1]), "x1": r(c.p1[2]), "y1": r(c.p1[1]), "r0": r(c.r0), "r1": r(c.r1)}
    p = {"id": c.id, **base, "x0": r(c.p0[0]), "y0": r(c.p0[2]), "x1": r(c.p1[0]), "y1": r(c.p1[2]), "r0": r(c.r0), "r1": r(c.r1)}
    return [("elevation", e), ("front", f), ("plan", p)]


def lobe_solid(lb, b):
    u, v, w = b.local(lb.c)
    flip = math.cos(b.ang * D2R) < 0
    rx, ry, rz = lb.rad
    s = {"id": lb.id, "kind": "ellipsoid", "bone": b.id, "material": lb.material, "role": lb.role, "tone": lb.tone, "radii": [r(rx), r(ry), r(rz)], "transform": {"translation": [r(u), r(v), r(w)]}}
    return s


def cap_solid(c, b, slender_ratio):
    p0, p1 = b.local(c.p0), b.local(c.p1)
    s = {"id": c.id, "kind": "capsule", "bone": b.id, "material": c.material, "role": c.role, "tone": c.tone, "p0": [r(x) for x in p0], "p1": [r(x) for x in p1], "r0": r(c.r0), "r1": r(c.r1)}
    L = math.dist(c.p0, c.p1) + c.r0 + c.r1
    if (c.slender if c.slender is not None else L >= slender_ratio * 2 * max(c.r0, c.r1)):
        s["slender"] = True
    return s
#endregion primitives


def collar_parts(sp, b, rad, tone):
    """🪨️ The root collar as a frustum: 2D trapezoids in elevation and front, a circle in plan, a capped loft in 3D."""
    h, r0, r1 = (rad[2] if len(rad) > 2 else sp["collar"]["h"]), rad[0], rad[1]
    base = {"bone": b.id, "role": "body", "z": 0.5, "filled": True, "tone": tone, "region": "stem-bark", "material": sp["bark"]}
    tr = [(0, -r0), (h, -r1), (h, r1), (0, r0)]
    e = {"id": "root-collar", **base, "kind": "poly", "closed": True, "points": [[r(x), r(y)] for x, y in tr]}
    f = {"id": "root-collar", **base, "kind": "poly", "closed": True, "points": [[r(-r0), 0], [r(-r1), r(h)], [r(r1), r(h)], [r(r0), 0]]}
    p = {"id": "root-collar", **base, "kind": "poly", "closed": True, "points": [[r(r0 * math.cos(2 * math.pi * i / 24)), r(r0 * math.sin(2 * math.pi * i / 24))] for i in range(24)]}
    solid = {"id": "root-collar", "kind": "loft", "bone": b.id, "material": sp["bark"], "role": "body", "tone": tone,
             "sections": [{"at": [0, 0, 0], "rx": r(r0), "ry": r(r0), "rotation": LIMB}, {"at": [r(h), 0, 0], "rx": r(r1), "ry": r(r1), "rotation": LIMB}], "capStart": True, "capEnd": True}
    return [("elevation", e), ("front", f), ("plan", p)], solid


def marks(sp, lb, b, scale):
    """🖊️ Leaf texture strokes of one lobe from the species template: small open polylines well inside the lobe in the elevation and the front."""
    tpl = sp["texture"][scale]
    out = []
    (cx, cy, cz), (rx, ry, rz) = lb.c, lb.rad
    for view in ("elevation", "front"):
        for i, (fx, fy, ang, ln, bend) in enumerate(tpl):
            if view == "elevation":
                u, v, _ = b.local((cx + fx * rx, cy + fy * ry, cz))
                flip = -1 if math.cos(b.ang * D2R) < 0 else 1
                a = ang if flip > 0 else 180 - ang
                X, Y = u, v
                s = 1 if flip > 0 else -1
            else:
                X, Y = cz + fx * rz, cy + fy * ry
                a = ang + 20
                s = 1
            ca, sa = math.cos(a * D2R), math.sin(a * D2R)
            pts = []
            for j in range(5):
                t = (j / 4 - 0.5) * ln
                off = bend * ln * (1 - (2 * j / 4 - 1) ** 2)
                pts.append((X + t * ca - off * sa * s, Y + t * sa + off * ca))
            out.append((view, {"id": "%s-tex-%d" % (lb.id, i + 1), "bone": b.id, "role": "texture", "z": 9, "filled": False, "tone": 0.3, "kind": "poly", "closed": False, "points": [[r(x), r(y)] for x, y in pts]}))
    return out


def representation(sp, rig, scale):
    """🖼️ One authored scale: 2D shapes in the three views and the matching body, thermal and structure solids."""
    S = sp["scales"][scale]
    geo = {"elevation": [], "front": [], "plan": []}
    body, thermal, structure = [], [], []
    root = rig.bones["root-plate"]
    views, solid = collar_parts(sp, root, S["collar"], 0.32)
    for v, s in views:
        geo[v].append(s)
    body.append(solid)
    thermal.append({"id": "th-root-collar", "of": "root-collar", "region": "stem-bark"})
    zi = 1.0
    for c in S.get("caps", []):
        b = rig.bones[c.bone]
        if c.role != "thermal-only":
            for v, s in cap_views(c, b, zi):
                geo[v].append(s)
            body.append(cap_solid(c, b, 20))
            thermal.append({"id": "th-" + c.id, "of": c.id, "region": c.region})
        else:
            s = cap_solid(c, b, 20)
            s["role"] = "body"
            s["region"] = c.region
            thermal.append(s)
        zi += 0.01
    zi = 3.0
    for lb in S["lobes"]:
        b = rig.bones[lb.bone]
        for v, s in lobe_views(lb, b, round(zi + (lb.c[2] if v == "front" else 0) * 0.0, 3)):
            geo[v].append(s)
        body.append(lobe_solid(lb, b))
        thermal.append({"id": "th-" + lb.id, "of": lb.id, "region": lb.region})
        if S.get("texture") and lb.texture:
            for v, s in marks(sp, lb, b, scale):
                geo[v].append(s)
        zi += 0.01
    for th in S.get("thermal_only", []):
        b = rig.bones[th.bone]
        s = lobe_solid(th, b) if isinstance(th, Lobe) else cap_solid(th, b, 20)
        s["region"] = th.region
        if getattr(th, "tags", None):
            s["tags"] = th.tags
        thermal.append(s)
    for m in members_for(sp, rig, scale):
        structure.append(m)
    structure.append({"id": "support-base-support", "kind": "revolve", "bone": "root-plate", "material": sp["wood"], "role": "detail", "tone": 0.25, "profile": [[-SUPPORT_T[scale], r(S["support_r"])], [0, r(S["support_r"])]], "axis": [1, 0, 0], "origin": [0, 0, 0], "member": "root-plate", "part": "support", "anchor": "base-support"})
    return geo, body, thermal, structure


def member(id, bone, kind, rig, sections=None, cap=None, members=None, slender=True, material=None, sp=None):
    """🏗️ A structural member solid of a stem bone: a loft through bone-local stations (u, w, radius) or a capsule."""
    b = rig.bones[bone]
    base = {"id": id, "bone": bone, "material": material, "role": "body", "tone": 0.4}
    if kind == "loft":
        base.update({"kind": "loft", "sections": [{"at": [r(u), 0, r(w)], "rx": r(rad, 5), "ry": r(rad, 5), "rotation": LIMB} for u, w, rad in sections], "capStart": True, "capEnd": True})
    else:
        p0, p1, r0, r1 = cap
        base.update({"kind": "capsule", "p0": [r(x) for x in p0], "p1": [r(x) for x in p1], "r0": r(r0, 5), "r1": r(r1, 5)})
    if slender:
        base["slender"] = True
    base["member"] = bone
    if members:
        base["members"] = members
    base["part"] = "member"
    return base


MEMBER_STYLE = {"50": ("loft", 4, 0.97), "100": ("loft", 3, 0.96), "200": ("loft", 2, 0.95), "500": ("constant", 2, 1.0)}


def members_for(sp, rig, scale):
    """🏗️ The structural idealisation of a scale: every stem segment as a member of green wood along its true 3D axis (1:50 a three-station
    tapered loft, 1:100 a three-station loft, 1:200 a two-station loft, 1:500 a two-station loft of the declared section) and the root collar as a frustum."""
    kind, n, k = MEMBER_STYLE[scale]
    out = []
    rc = sp["scales"][scale]["collar"]
    h = rc[2] if len(rc) > 2 else sp["collar"]["h"]
    st = [(0, rc[0] * k), (h, rc[1] * k)] if scale != "100" else [(0, rc[0] * k), (h / 2, (rc[0] + rc[1]) / 2 * k), (h, rc[1] * k)]
    out.append({"id": "member-root-plate", "kind": "loft", "bone": "root-plate", "material": sp["wood"], "role": "body", "tone": 0.4,
                "sections": [{"at": [r(u), 0, 0], "rx": r(rad, 5), "ry": r(rad, 5), "rotation": LIMB} for u, rad in st], "capStart": True, "capEnd": True, "member": "root-plate", "part": "member"})
    for id in rig.order:
        b = rig.bones[id]
        if b.kind != "wood":
            continue
        p0, p1 = b.seg3
        u1, v1, w1 = b.local(p1)
        r0, r1 = b.d0 / 2 * k, b.d1 / 2 * k
        if kind == "loft":
            secs = [(u1 * i / (n - 1), w1 * i / (n - 1), r0 + (r1 - r0) * i / (n - 1)) for i in range(n)]
            out.append(member("member-" + id, id, "loft", rig, sections=secs, material=sp["wood"]))
        elif kind == "capsule":
            out.append(member("member-" + id, id, "capsule", rig, cap=((0, 0, 0), (u1, v1, w1), r0, r1), material=sp["wood"]))
        else:
            out.append(member("member-" + id, id, "loft", rig, sections=[(0, 0, b.d0 / 2), (u1, w1, b.d0 / 2)], material=sp["wood"]))
    return out


def extents(body_world):
    xs, ys, zs = [], [], []
    for (lo, hi) in body_world:
        xs += [lo[0], hi[0]]
        ys += [lo[1], hi[1]]
        zs += [lo[2], hi[2]]
    return min(xs), max(xs), min(ys), max(ys), min(zs), max(zs)


def world_boxes(sp, rig, scale):
    """📦️ World boxes of the body primitives of a scale at rest (for `dimensions`)."""
    S = sp["scales"][scale]
    out = []
    rc = S["collar"]
    out.append(((-rc[0], 0, -rc[0]), (rc[0], rc[2] if len(rc) > 2 else sp["collar"]["h"], rc[0])))
    for c in S.get("caps", []):
        if c.role == "thermal-only":
            continue
        for p, rr in ((c.p0, c.r0), (c.p1, c.r1)):
            out.append(((p[0] - rr, p[1] - rr, p[2] - rr), (p[0] + rr, p[1] + rr, p[2] + rr)))
    for lb in S["lobes"]:
        (cx, cy, cz), (rx, ry, rz) = lb.c, lb.rad
        out.append(((cx - rx, cy - ry, cz - rz), (cx + rx, cy + ry, cz + rz)))
    return out


def write(doc, slug):
    path = os.path.join(CATALOG, slug, "🔣️.json")
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w", encoding="utf-8") as f:
        json.dump(doc, f, ensure_ascii=False, indent=1)
        f.write("\n")
    return path


def manifest_row(asset, distinction):
    reps = {k: asset["representations"][k]["status"] for k in SCALES}
    row = {"id": asset["id"], "category": "vegetation", "title": asset["title"], "heightM": asset["dimensions"]["heightM"], "widthM": asset["dimensions"]["widthM"], "powerW": 0,
           "poses": [p["id"] for p in asset["poses"]], "clips": [c["id"] for c in asset["clips"]], "representations": reps, "symbols": [s["id"] for s in asset["symbols"]],
           "analytics": ["thermal", "transport", "structural"], "exports": asset["exports"], "distinction": distinction, "completion": "authored"}
    path = os.path.join(TICKET, "📥️manifest-rows", asset["id"] + ".json")
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w", encoding="utf-8") as f:
        json.dump(row, f, ensure_ascii=False, indent=1)
        f.write("\n")
