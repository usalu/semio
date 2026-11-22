"""🌿 PW-14 plant kit: world-rest primitives (ellipsoid, capsule, vertical revolve, marks) that give one 3D solid and its exact side, front and plan projections, so every scale's drawing and volume are one body.

Every number is chosen in the species scripts; this module converts world rest metres into bone-local solids and shapes and serialises the asset.
Reuses the NW-V2 helpers of the archive ticket (text, rounding, wind force, region and pose builders) by import.
@see ../../../../../🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE/🧰️scripts/NW-V2/common.py
"""
import json, math, os, sys

HERE = os.path.dirname(os.path.abspath(__file__))
TICKET = os.path.dirname(os.path.dirname(HERE))
REPO = os.path.abspath(os.path.join(TICKET, "..", "..", "..", "..", "..", "..", ".."))
NW_V2 = os.path.join(REPO, ".🧬semio", "🦑️repo", "🎫️tickets", "🎆️26", "🌙️10", "☀️29", "ARCHITECTURAL-STAFFAGE-ARCHIVE", "🧰️scripts", "NW-V2")
sys.path.insert(0, NW_V2)
from lib import T, r4, catmull, wind_force  # noqa: E402
from common import leaf_region, passive_region, transpiration, bind, wind_load, vertical_load  # noqa: E402

CATALOG = os.path.join(REPO, "🎓️teaching", "🏛️architecture", "🧍️staffage", "🗂️catalog", "🌳️vegetation")
GENERATED = os.path.join(TICKET, "🗑️generated", "PW-14")
ROWS = os.path.join(TICKET, "📥️manifest-rows")
EXPORTS = ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"]
FEATURE_MM = 0.15
GRAVITY = 9.80665


def feature(den):
    return FEATURE_MM * den / 1000


def vadd(a, b):
    return tuple(x + y for x, y in zip(a, b))


def vsub(a, b):
    return tuple(x - y for x, y in zip(a, b))


def vscale(a, k):
    return tuple(x * k for x in a)


def vlen(a):
    return math.sqrt(sum(x * x for x in a))


def lerp3(a, b, t):
    return tuple(x + (y - x) * t for x, y in zip(a, b))


def quat_w(deg):
    h = math.radians(deg) / 2
    return [0, 0, r4(math.sin(h)), r4(math.cos(h))] if abs(deg) > 1e-9 else None


class Rig:
    """🦴 A sagittal botanical rig authored in world rest metres: every bone keeps w = +z, so bone-local (u, v) is the side elevation and front and plan are world projections."""

    def __init__(s):
        s.bones, s.W = [], {}

    def add(s, id, parent, title, o, theta, length, joint, mass=None, section=None):
        if parent is None:
            lx, ly, lz, la = o[0], o[1], o[2], theta
        else:
            lx, ly, lz = s.local(parent, o)
            la = theta - s.W[parent][1]
        s.W[id] = (tuple(o), theta, length)
        b = {"id": id, "parent": parent, "title": title, "x": r4(lx), "y": r4(ly)}
        if abs(lz) > 1e-9:
            b["z"] = r4(lz)
        b.update({"angleDeg": r4(la), "lengthM": r4(length), "joint": joint})
        if mass:
            b["mass"] = mass
        if section:
            b["section"] = section
        s.bones.append(b)
        return id

    def axes(s, id):
        o, th, _ = s.W[id]
        c, sn = math.cos(math.radians(th)), math.sin(math.radians(th))
        return o, (c, sn, 0.0), (-sn, c, 0.0), (0.0, 0.0, 1.0)

    def local(s, id, p):
        o, u, v, w = s.axes(id)
        d = vsub(p, o)
        return (d[0] * u[0] + d[1] * u[1], d[0] * v[0] + d[1] * v[1], d[2])

    def world(s, id, q):
        o, u, v, w = s.axes(id)
        return (o[0] + q[0] * u[0] + q[1] * v[0], o[1] + q[0] * u[1] + q[1] * v[1], o[2] + q[2])

    def tip(s, id):
        return s.world(id, (s.W[id][2], 0, 0))

    def theta(s, id):
        return s.W[id][1]

    def ids(s):
        return [b["id"] for b in s.bones]


def revolute(lo, hi):
    return {"type": "revolute", "minDeg": lo, "maxDeg": hi}


FIXED = {"type": "fixed"}


def mass(kg, com, src, ref, dist="uniform"):
    return {"kg": round(kg, 4), "comX": r4(com), "comY": 0, "distribution": dist, "source": src, "reference": ref}


def round_section(d, material, en, de):
    a = math.pi * d * d / 4
    return {"areaM2": float(f"{a:.6g}"), "secondMomentM4": float(f"{math.pi * d ** 4 / 64:.6g}"), "extremeFibreM": r4(d / 2), "material": material, "title": T(en, de)}


class Prim:
    """🧩 One world-rest primitive with its physical identity; `solid()` gives the bone-local volume solid, `shapes()` its three exact projections."""

    def __init__(s, id, bone, material, role, tone, region=None, regions=None, z=1.0, slender=False, filled=True, stroke=False):
        s.id, s.bone, s.material, s.role, s.tone = id, bone, material, role, tone
        s.region, s.regions, s.z, s.slender, s.filled, s.stroke = region, regions, z, slender, filled, stroke

    def base(s):
        d = {"id": s.id, "bone": s.bone, "material": s.material, "role": s.role, "tone": s.tone}
        if s.slender:
            d["slender"] = True
        return d

    def zview(s, view, rig):
        """🎚️ The authored paint order of a view: the nearness of the nearest point (right side in the elevation, ahead in the front, height in the plan), so nearer parts hide farther ones."""
        lo, hi = s.extent(rig)
        near = {"elevation": -lo[2], "front": hi[0], "plan": hi[1]}[view]
        return round(5 + near + 0.0001 * s.z, 5)

    def shape_base(s, view, rig=None):
        d = {"id": s.id, "bone": s.bone, "role": s.role, "z": s.zview(view, rig) if rig is not None else s.z, "filled": s.filled and not s.stroke, "tone": s.tone}
        if not s.stroke:
            if s.regions:
                d["regions"] = s.regions
            elif s.region:
                d["region"] = s.region
            d["material"] = s.material
        return d


class Ell(Prim):
    """⭕ Ellipsoid with semi-axes `r` = (a along the direction `phi` degrees in the x-y plane, b across it in that plane, c along z)."""

    def __init__(s, id, bone, c, r, phi=0.0, **kw):
        super().__init__(id, bone, **kw)
        s.c, s.r, s.phi = tuple(c), tuple(r), phi

    def extent(s, rig):
        a, b, c = s.r
        p = math.radians(s.phi)
        ex = math.sqrt((a * math.cos(p)) ** 2 + (b * math.sin(p)) ** 2)
        ey = math.sqrt((a * math.sin(p)) ** 2 + (b * math.cos(p)) ** 2)
        return [(s.c[0] - ex, s.c[1] - ey, s.c[2] - c), (s.c[0] + ex, s.c[1] + ey, s.c[2] + c)]

    def solid(s, rig):
        d = s.base()
        d.update({"kind": "ellipsoid", "radii": [r4(x) for x in s.r]})
        t = {"translation": [r4(x) for x in rig.local(s.bone, s.c)]}
        q = quat_w(s.phi - rig.theta(s.bone))
        if q:
            t["rotation"] = q
        d["transform"] = t
        return d

    def shapes(s, rig):
        a, b, c = s.r
        p = math.radians(s.phi)
        lx, ly, _ = rig.local(s.bone, s.c)
        rot = s.phi - rig.theta(s.bone)
        el = {"kind": "ellipse", "cx": r4(lx), "cy": r4(ly), "rx": r4(a), "ry": r4(b)}
        rot = (rot + 180) % 360 - 180
        if abs(rot) > 1e-6:
            el["rotationDeg"] = r4(rot)
        fr = {"kind": "ellipse", "cx": r4(s.c[2]), "cy": r4(s.c[1]), "rx": r4(c), "ry": r4(math.sqrt((a * math.sin(p)) ** 2 + (b * math.cos(p)) ** 2))}
        pl = {"kind": "ellipse", "cx": r4(s.c[0]), "cy": r4(s.c[2]), "rx": r4(math.sqrt((a * math.cos(p)) ** 2 + (b * math.sin(p)) ** 2)), "ry": r4(c)}
        return {v: {**s.shape_base(v, rig), **g} for v, g in (("elevation", el), ("front", fr), ("plan", pl))}


class Cap(Prim):
    """💊 Capsule between two world points; a `stroke` capsule is drawn as an open pen line (a slender stalk or cane far below the print feature)."""

    def __init__(s, id, bone, p0, p1, r0, r1, **kw):
        super().__init__(id, bone, **kw)
        s.p0, s.p1, s.r0, s.r1 = tuple(p0), tuple(p1), r0, r1

    def extent(s, rig):
        lo = tuple(min(a - s.r0, b - s.r1) for a, b in zip(s.p0, s.p1))
        hi = tuple(max(a + s.r0, b + s.r1) for a, b in zip(s.p0, s.p1))
        return [lo, hi]

    def solid(s, rig):
        d = s.base()
        d.update({"kind": "capsule", "p0": [r4(x) for x in rig.local(s.bone, s.p0)], "p1": [r4(x) for x in rig.local(s.bone, s.p1)], "r0": r4(s.r0), "r1": r4(s.r1)})
        return d

    def shapes(s, rig):
        a, b = rig.local(s.bone, s.p0), rig.local(s.bone, s.p1)
        views = {"elevation": ((a[0], a[1]), (b[0], b[1])), "front": ((s.p0[2], s.p0[1]), (s.p1[2], s.p1[1])), "plan": ((s.p0[0], s.p0[2]), (s.p1[0], s.p1[2]))}
        out = {}
        for v, (q0, q1) in views.items():
            if s.stroke:
                if math.hypot(q1[0] - q0[0], q1[1] - q0[1]) < 1e-4:
                    continue
                g = {"kind": "poly", "points": [[r4(q0[0]), r4(q0[1])], [r4(q1[0]), r4(q1[1])]], "closed": False}
            else:
                g = {"kind": "capsule", "x0": r4(q0[0]), "y0": r4(q0[1]), "x1": r4(q1[0]), "y1": r4(q1[1]), "r0": r4(s.r0), "r1": r4(s.r1)}
            out[v] = {**s.shape_base(v, rig), **g}
        return out


class Rev(Prim):
    """🏺 A solid of revolution about the world vertical through `c`, profile [(height above c, radius)] starting and ending on the axis; `lateral` scales its z radius."""

    def __init__(s, id, bone, c, profile, lateral=1.0, **kw):
        super().__init__(id, bone, **kw)
        s.c, s.profile, s.lateral = tuple(c), [tuple(p) for p in profile], lateral

    def extent(s, rig):
        rm = max(r for _, r in s.profile)
        hs = [h for h, _ in s.profile]
        return [(s.c[0] - rm, s.c[1] + min(hs), s.c[2] - rm * s.lateral), (s.c[0] + rm, s.c[1] + max(hs), s.c[2] + rm * s.lateral)]

    def solid(s, rig):
        d = s.base()
        th = math.radians(rig.theta(s.bone))
        d.update({"kind": "revolve", "profile": [[r4(h), r4(r)] for h, r in s.profile], "axis": [r4(math.sin(th)), r4(math.cos(th)), 0], "origin": [r4(x) for x in rig.local(s.bone, s.c)]})
        if abs(s.lateral - 1) > 1e-9:
            d["lateralScale"] = r4(s.lateral)
        return d

    def outline(s, lat):
        right = [(r * lat, h) for h, r in s.profile]
        left = [(-r * lat, h) for h, r in reversed(s.profile)]
        pts = right + left[1:-1]
        out = []
        for p in pts:
            if not out or math.hypot(p[0] - out[-1][0], p[1] - out[-1][1]) > 1e-5:
                out.append(p)
        return out

    def shapes(s, rig):
        el = [rig.local(s.bone, (s.c[0] + x, s.c[1] + y, s.c[2]))[:2] for x, y in s.outline(1.0)]
        fr = [(s.c[2] + x, s.c[1] + y) for x, y in s.outline(s.lateral)]
        rm = max(r for _, r in s.profile)
        g = {"elevation": {"kind": "poly", "points": [[r4(x), r4(y)] for x, y in el], "closed": True},
             "front": {"kind": "poly", "points": [[r4(x), r4(y)] for x, y in fr], "closed": True},
             "plan": {"kind": "ellipse", "cx": r4(s.c[0]), "cy": r4(s.c[2]), "rx": r4(rm), "ry": r4(rm * s.lateral)}}
        return {v: {**s.shape_base(v, rig), **x} for v, x in g.items()}


class Mark:
    """✒️ A drawing-only texture or detail line (leaf dash, flower whorl, bark ridge) in one view, given in that view's world coordinates; it lies inside the body it describes and is painted just above its host, so it adds no area and hides behind nearer parts."""

    def __init__(s, id, bone, view, pts, host, role="texture", closed=False, tone=0.5):
        s.id, s.bone, s.view, s.pts, s.host, s.role, s.closed, s.tone = id, bone, view, pts, host, role, closed, tone

    def shape(s, rig):
        pts = [rig.local(s.bone, (p[0], p[1], 0))[:2] for p in s.pts] if s.view == "elevation" else s.pts
        return {"id": s.id, "bone": s.bone, "role": s.role, "z": round(s.host.zview(s.view, rig) + 0.00001, 5), "filled": False, "tone": s.tone, "kind": "poly", "points": [[r4(p[0]), r4(p[1])] for p in pts], "closed": s.closed}


def view_point(view, p):
    return {"elevation": (p[0], p[1]), "front": (p[2], p[1]), "plan": (p[0], p[2])}[view]


class Scale:
    """📐 The design of one authored scale: primitives (solid plus projections), marks, structure solids, decisions."""

    def __init__(s, den, rig):
        s.den, s.rig, s.prims, s.marks, s.structure, s.emitters = den, rig, [], [], [], []

    def add(s, p):
        s.prims.append(p)
        return p

    def mark(s, m):
        s.marks.append(m)
        return m

    def extent(s):
        lo, hi = [1e9] * 3, [-1e9] * 3
        for p in s.prims:
            a, b = p.extent(s.rig)
            lo = [min(x, y) for x, y in zip(lo, a)]
            hi = [max(x, y) for x, y in zip(hi, b)]
        return lo, hi

    def geometry(s):
        g = {"elevation": [], "front": [], "plan": []}
        for p in s.prims:
            for v, sh in p.shapes(s.rig).items():
                g[v].append(sh)
        for m in s.marks:
            g[m.view].append(m.shape(s.rig))
        return g

    def body(s):
        return [p.solid(s.rig) for p in s.prims]


def thermal_list(scale, emitter_solids):
    out = []
    for p in scale.prims:
        region = p.region or (p.regions[0] if p.regions else None)
        if region:
            out.append({"id": "th-" + p.id, "of": p.id, "region": region})
    return out + emitter_solids


def emitter_solid(rig, id, anchor, region, emitter, radius, material):
    a = anchor
    return {"id": id, "kind": "ellipsoid", "bone": a["bone"], "material": material, "role": "detail", "tone": 0.45, "radii": [r4(radius)] * 3,
            "transform": {"translation": [a["x"], a["y"], a.get("z", 0)]}, "region": region, "tags": [emitter]}


def qmul(a, b):
    ax, ay, az, aw = a
    bx, by, bz, bw = b
    return [aw * bx + ax * bw + ay * bz - az * by, aw * by - ax * bz + ay * bw + az * bx, aw * bz + ax * by - ay * bx + az * bw, aw * bw - ax * bx - ay * by - az * bz]


def member_loft(rig, id, bone, points, r, material, members=None):
    """🪢 A round member loft through world points of the sagittal plane, every section turned to its path tangent (limb rotation [0.5, 0.5, 0.5, 0.5] then about w), holding a chain of bones."""
    loc = [rig.local(bone, p) for p in points]
    secs = []
    for i, q in enumerate(loc):
        a, b = loc[max(0, i - 1)], loc[min(len(loc) - 1, i + 1)]
        beta = math.atan2(b[1] - a[1], b[0] - a[0])
        rot = qmul([0, 0, math.sin(beta / 2), math.cos(beta / 2)], [0.5, 0.5, 0.5, 0.5])
        secs.append({"at": [r4(x) for x in q], "rx": r4(r), "ry": r4(r), "rotation": [round(x, 6) for x in rot]})
    d = {"id": id, "kind": "loft", "bone": bone, "material": material, "role": "body", "tone": 0.4, "sections": secs, "capStart": True, "capEnd": True, "member": bone, "part": "member"}
    if members:
        d["members"] = members
    return d


def member_capsule(rig, id, bone, p0, p1, r, material, members=None, slender=False):
    d = {"id": id, "kind": "capsule", "bone": bone, "material": material, "role": "body", "tone": 0.4, "p0": [r4(x) for x in rig.local(bone, p0)], "p1": [r4(x) for x in rig.local(bone, p1)], "r0": r4(r), "r1": r4(r), "member": bone, "part": "member"}
    if slender:
        d["slender"] = True
    if members:
        d["members"] = members
    return d


def support_box(rig, bone, anchor, size, centre, material):
    return {"id": "support-" + anchor, "kind": "box", "bone": bone, "material": material, "role": "detail", "tone": 0.25, "size": [r4(x) for x in size], "transform": {"translation": [r4(x) for x in rig.local(bone, centre)]}, "member": bone, "part": "support", "anchor": anchor}


def mass_ball(rig, bone, at, r, material):
    return {"id": "mass-" + bone, "kind": "ellipsoid", "bone": bone, "material": material, "role": "detail", "tone": 0.3, "radii": [r4(r)] * 3, "transform": {"translation": [r4(x) for x in rig.local(bone, at)]}, "member": bone, "part": "mass"}


def representation(den, lo, hi, pens, hatches, geometry, decisions):
    return {"status": "authored", "denominator": den, "validRange": {"minDenominator": lo, "maxDenominator": hi}, "pens": pens, "hatches": hatches, "geometry": geometry, "decisions": decisions}


def pens(body, detail, texture, glazing, ground):
    return {"body": body, "detail": detail, "texture": texture, "glazing": glazing, "ground": ground}


def decisions(**k):
    keys = ["silhouette", "retained", "merged", "omitted", "thinElements", "hatching", "styles", "overlays"]
    return {key: T(*k[key]) for key in keys}


def volume(den, body, thermal, structure, notes):
    return {"status": "authored", "denominator": den, "body": body, "thermal": thermal, "structure": structure, "decisions": [T(*n) for n in notes]}


def sway_keys(pose, duration, n, base, specs, limits):
    """〰️ Periodic sway keys: specs bone -> (amplitude deg, phase deg, harmonic); the first key equals the last."""
    keys = []
    for k in range(n + 1):
        kk = k % n
        bones = {}
        for b, (amp, ph, h) in specs.items():
            lo, hi = limits[b]
            a = base.get(b, 0) + amp * math.sin(2 * math.pi * h * kk / n + math.radians(ph))
            bones[b] = {"angleDeg": r4(max(lo, min(hi, a)))}
        keys.append({"t": r4(duration * k / n), "pose": pose, "ease": "smooth", "bones": bones})
    return keys


def limits_of(rig):
    return {b["id"]: (b["joint"].get("minDeg", -180), b["joint"].get("maxDeg", 180)) for b in rig.bones}


def write_asset(asset):
    folder = os.path.join(CATALOG, asset["id"])
    os.makedirs(folder, exist_ok=True)
    doc = {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": asset}
    path = os.path.join(folder, "🔣️.json")
    with open(path, "w", encoding="utf8") as f:
        json.dump(doc, f, ensure_ascii=False, indent=1)
        f.write("\n")
    return path


def write_row(row):
    os.makedirs(ROWS, exist_ok=True)
    path = os.path.join(ROWS, row["id"] + ".json")
    with open(path, "w", encoding="utf8") as f:
        json.dump(row, f, ensure_ascii=False, indent=1)
        f.write("\n")
    return path


def manifest_row(asset, distinction):
    reps = {k: v["status"] for k, v in asset["representations"].items()}
    return {"id": asset["id"], "category": asset["category"], "title": asset["title"], "heightM": asset["dimensions"]["heightM"], "widthM": asset["dimensions"]["widthM"], "powerW": 0,
            "poses": [p["id"] for p in asset["poses"] if not p.get("key")], "clips": [c["id"] for c in asset["clips"]], "representations": {k: reps[k] for k in ("50", "100", "200", "500")},
            "symbols": [s["id"] for s in asset["symbols"]], "analytics": ["thermal", "transport", "structural"], "exports": EXPORTS, "distinction": distinction, "completion": asset["completion"]}


def overrides(asset_id):
    path = os.path.join(HERE, f"{asset_id}.areas.json")
    return json.load(open(path)) if os.path.exists(path) else {}


def ring(cx, cy, rx, ry, n, rot=0.0, jitter=None):
    out = []
    for i in range(n):
        t = 2 * math.pi * i / n
        k = 1.0 if jitter is None else jitter[i % len(jitter)]
        x, y = rx * k * math.cos(t), ry * k * math.sin(t)
        c, s_ = math.cos(rot), math.sin(rot)
        out.append((cx + c * x - s_ * y, cy + s_ * x + c * y))
    return out
