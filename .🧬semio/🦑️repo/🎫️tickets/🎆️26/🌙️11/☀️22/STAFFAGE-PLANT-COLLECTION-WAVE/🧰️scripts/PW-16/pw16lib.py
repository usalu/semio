"""🧩️ PW-16 plant kit: a three-dimensional rig (the engine's chain R = R(parent)·Rw(angle)·Rv(abduction)·Ru(twist)) and twin primitives that write one hand-designed part
into the drawing of a scale (elevation, front, plan) and into the body volume of the same scale, so that the 2D views are the projections of the 3D body by construction.

Reuses the NW-V2 builders of the ivy-on-trellis and miscanthus generators (imported from their place, never copied):
`ARCHITECTURAL-STAFFAGE-ARCHIVE/🧰️scripts/NW-V2/lib.py` (T, r4, revolute, FIXED, catmull, wind_force) and `common.py` (regions, emitters, bindings, loads, anchors).

Primitives (bone-local u along the bone, v its normal, w lateral, metres):
- `ell`   ellipsoid ↔ elevation ellipse (bone plane), front and plan ellipses (world rest, exact projections of the posed ellipsoid);
- `plate` prism of a (u, v) profile swept along w ↔ the same profile with `solid: extruded, sweep w` in all three views (3D `extrude`, or a `polyhedron` when it tapers);
- `rod`   cylinder between two points ↔ `member` shape with a circular section in all three views (3D `revolve` of a constant radius).

@see ../../../../☀️29/../../🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE/🧰️scripts/NW-V2/lib.py
@see /Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/README.md
"""
import json, math, os, sys

NW = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE/🧰️scripts/NW-V2"
sys.path.insert(0, NW)
from lib import T, r4, revolute, FIXED, catmull, wind_force  # noqa: E402
from common import leaf_region, passive_region, transpiration, bind, wind_load, vertical_load, pens, decisions, anchors_support, emitter_anchor  # noqa: E402

S = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage"
WAVE = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-PLANT-COLLECTION-WAVE"
GEN = WAVE + "/🗑️generated/PW-16"
SCALES = (500, 200, 100, 50)
EXPORTS = ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"]


def r5(v):
    v = round(float(v), 5)
    return 0.0 if v == 0 else v


def feature(d):
    return 0.15 * d / 1000


def mat3_mul(a, b):
    return [[sum(a[i][k] * b[k][j] for k in range(3)) for j in range(3)] for i in range(3)]


def mat3_vec(a, v):
    return [sum(a[i][k] * v[k] for k in range(3)) for i in range(3)]


def transpose(a):
    return [[a[j][i] for j in range(3)] for i in range(3)]


def rot_w(deg):
    c, s = math.cos(math.radians(deg)), math.sin(math.radians(deg))
    return [[c, -s, 0], [s, c, 0], [0, 0, 1]]


def rot_v(deg):
    c, s = math.cos(math.radians(deg)), math.sin(math.radians(deg))
    return [[c, 0, -s], [0, 1, 0], [s, 0, c]]


def rot_u(deg):
    c, s = math.cos(math.radians(deg)), math.sin(math.radians(deg))
    return [[1, 0, 0], [0, c, s], [0, -s, c]]


def rest_rotation(angle, abduction=0.0, twist=0.0):
    return mat3_mul(mat3_mul(rot_w(angle), rot_v(abduction)), rot_u(twist))


def quat_w(deg):
    h = math.radians(deg) / 2
    return [0, 0, r5(math.sin(h)), r5(math.cos(h))]


def eig2(a, b, c):
    """Semi-axes and angle of the ellipse x^T S^-1 x = 1 with S = [[a, b], [b, c]]."""
    t = (a + c) / 2
    d = math.sqrt(max(0.0, ((a - c) / 2) ** 2 + b * b))
    l1, l2 = t + d, max(1e-12, t - d)
    ang = 0.5 * math.degrees(math.atan2(2 * b, a - c))
    return math.sqrt(l1), math.sqrt(l2), ang


class Rig:
    """🦴️ Bones with their rest frames in the world (x ahead, y up, z left)."""

    def __init__(s):
        s.bones = []
        s.frame = {}

    def add(s, id, parent, title, x, y, angle, length, joint, z=0.0, abduction=0.0, twist=0.0, mass=None, section=None):
        twist = (twist + 180.0) % 360.0 - 180.0
        if parent is None:
            o, R = [x, y, z], rest_rotation(angle, abduction, twist)
        else:
            po, PR = s.frame[parent]
            o = [po[i] + mat3_vec(PR, [x, y, z])[i] for i in range(3)]
            R = mat3_mul(PR, rest_rotation(angle, abduction, twist))
        s.frame[id] = (o, R)
        b = {"id": id, "parent": parent, "title": title, "x": r5(x), "y": r5(y)}
        if z:
            b["z"] = r5(z)
        b["angleDeg"] = r5(angle)
        if abduction:
            b["abductionDeg"] = r5(abduction)
        if twist:
            b["twistDeg"] = r5(twist)
        b.update({"lengthM": r5(length), "joint": joint})
        if mass:
            kg, src, ref = mass[:3]
            com = mass[3] if len(mass) > 3 else (length / 2, 0)
            b["mass"] = {"kg": round(kg, 3), "comX": r5(com[0]), "comY": r5(com[1]), "distribution": "uniform" if length > 0 and len(mass) <= 3 else "point", "source": src, "reference": ref}
        if section:
            b["section"] = section
        s.bones.append(b)
        return b

    def add_world(s, id, parent, title, origin, angle_world, length, joint, **kw):
        """A sagittal bone from a world origin and a world angle of its axis in the side elevation."""
        po, PR = s.frame[parent]
        local = mat3_vec(transpose(PR), [origin[i] - po[i] for i in range(3)])
        pa = math.degrees(math.atan2(PR[1][0], PR[0][0]))
        return s.add(id, parent, title, local[0], local[1], angle_world - pa, length, joint, z=local[2], **kw)

    def world(s, bone, p):
        o, R = s.frame[bone]
        q = mat3_vec(R, p)
        return [o[i] + q[i] for i in range(3)]

    def local(s, bone, p):
        o, R = s.frame[bone]
        return mat3_vec(transpose(R), [p[i] - o[i] for i in range(3)])

    def sagittal(s, bone):
        R = s.frame[bone][1]
        return abs(R[2][2] - 1) < 1e-9

    def tip(s, bone):
        b = next(x for x in s.bones if x["id"] == bone)
        return s.world(bone, [b["lengthM"], 0, 0])


class Scale:
    """✏️ The drawing and the body volume of one preset scale, written together."""

    def __init__(s, rig, denominator):
        s.rig, s.d = rig, denominator
        s.views = {"elevation": [], "front": [], "plan": []}
        s.body, s.regions, s.member_of, s.rods = [], {}, {}, []
        s.z = 0.0
        s.extent = [[1e9, -1e9] for _ in range(3)]

    def _z(s, z):
        if z is None:
            s.z += 0.001
            return r5(s.z)
        return z

    def _grow(s, pts):
        for p in pts:
            for k in range(3):
                s.extent[k][0] = min(s.extent[k][0], p[k])
                s.extent[k][1] = max(s.extent[k][1], p[k])

    def _shape(s, view, id, bone, geom, role, z, filled, tone, region, material, pen=None, solid=None):
        sh = {"id": id, "bone": bone, "role": role, "z": z, "filled": filled, "tone": r5(tone)}
        if region:
            sh["region"] = region
        if material:
            sh["material"] = material
        if pen:
            sh["penMm"] = pen
        sh.update(geom)
        if solid:
            sh["solid"] = solid
        s.views[view].append(sh)

    def _solid(s, solid, region, member=True):
        s.body.append(solid)
        s.regions[solid["id"]] = region
        if member and solid["bone"] not in s.member_of:
            s.member_of[solid["bone"]] = solid["id"]

    def ell(s, id, bone, c, r, rot=0.0, region=None, material=None, tone=0.4, role="body", z=None, views=("elevation", "front", "plan"), member=True):
        assert s.rig.sagittal(bone), bone
        z = s._z(z)
        o, R = s.rig.frame[bone]
        A = mat3_mul(mat3_mul(R, rot_w(rot)), [[r[0], 0, 0], [0, r[1], 0], [0, 0, r[2]]])
        C = s.rig.world(bone, c)
        AA = mat3_mul(A, transpose(A))
        if "elevation" in views:
            g = {"kind": "ellipse", "cx": r5(c[0]), "cy": r5(c[1]), "rx": r5(r[0]), "ry": r5(r[1])}
            if rot:
                g["rotationDeg"] = r5(rot)
            s._shape("elevation", id, bone, g, role, z, True, tone, region, material)
        for view, (i, j) in (("front", (2, 1)), ("plan", (0, 2))):
            if view not in views:
                continue
            ra, rb, ang = eig2(AA[i][i], AA[i][j], AA[j][j])
            g = {"kind": "ellipse", "cx": r5(C[i]), "cy": r5(C[j]), "rx": r5(ra), "ry": r5(rb)}
            if abs(ang) > 1e-6:
                g["rotationDeg"] = r5(ang)
            s._shape(view, id, bone, g, role, z, True, tone, region, material)
        s._solid({"id": id, "kind": "ellipsoid", "bone": bone, "material": material, "role": role, "tone": r5(tone), "radii": [r5(x) for x in r], "transform": {"translation": [r5(x) for x in c], "rotation": quat_w(rot)}}, region, member)
        ext = [math.sqrt(AA[k][k]) for k in range(3)]
        s._grow([[C[k] - ext[k] for k in range(3)], [C[k] + ext[k] for k in range(3)]])

    def plate(s, id, bone, profile, w0, w1, region=None, material=None, tone=0.4, role="body", z=None, views=("elevation", "front", "plan"), tip=None, sheet=False, slender=False, member=True, filled=True):
        z = s._z(z)
        zr = s.rig.frame[bone][0][2]
        solid2 = {"kind": "extruded", "sweep": "w", "z0": r5(zr + w0), "z1": r5(zr + w1)}
        length = next(b for b in s.rig.bones if b["id"] == bone)["lengthM"]
        if tip is not None:
            solid2["zTip"] = [r5(zr + tip[0]), r5(zr + tip[1])]
        pts = [[r5(p[0]), r5(p[1])] for p in profile]
        for view in views:
            s._shape(view, id, bone, {"kind": "poly", "points": pts, "closed": True}, role, z, filled, tone, region, material, solid=solid2)
        if tip is None:
            sol = {"id": id, "kind": "extrude", "bone": bone, "material": material, "role": role, "tone": r5(tone), "profile": pts, "plane": "xy", "from": r5(w0), "to": r5(w1)}
        else:
            sol = {"id": id, "kind": "polyhedron", "bone": bone, "material": material, "role": role, "tone": r5(tone), **tapered_prism(profile, w0, w1, tip, length)}
        if sheet:
            sol["sheet"] = True
        if slender:
            sol["slender"] = True
        s._solid(sol, region, member)
        for p in profile:
            for w in (w0, w1) if tip is None else (w0 + (tip[0] - w0) * p[0] / length, w1 + (tip[1] - w1) * p[0] / length):
                s._grow([s.rig.world(bone, [p[0], p[1], w])])

    def rod(s, id, bone, p0, p1, d, region=None, material=None, tone=0.4, role="body", z=None, views=("elevation", "front", "plan"), filled=True, member=True, pen=None):
        z = s._z(z)
        g = {"kind": "member", "from": {"x": r5(p0[0]), "y": r5(p0[1]), "z": r5(p0[2])}, "to": {"x": r5(p1[0]), "y": r5(p1[1]), "z": r5(p1[2])}, "section": {"kind": "circle", "d": r5(d)}}
        for view in views:
            s._shape(view, id, bone, g, role, z, filled, tone, region, material, pen=pen)
        L = math.dist(p0, p1)
        axis = [(p1[k] - p0[k]) / L for k in range(3)]
        sol = {"id": id, "kind": "revolve", "bone": bone, "material": material, "role": role, "tone": r5(tone), "profile": [[0, r5(d / 2)], [r5(L), r5(d / 2)]], "axis": [r5(a) for a in axis], "origin": [r5(x) for x in p0]}
        if L / d >= 20:
            sol["slender"] = True
        s._solid(sol, region, member)
        wa = [s.rig.world(bone, p1)[k] - s.rig.world(bone, p0)[k] for k in range(3)]
        wl = math.sqrt(sum(x * x for x in wa))
        pad = [d / 2 * math.sqrt(max(0.0, 1 - (x / wl) ** 2)) for x in wa]
        for p in (p0, p1):
            q = s.rig.world(bone, p)
            s._grow([[q[k] - pad[k] for k in range(3)], [q[k] + pad[k] for k in range(3)]])
        s.rods.append(id)

    def stroke(s, id, bone, pts, role="detail", z=None, tone=0.2, view="elevation", closed=False, pen=None):
        z = s._z(z)
        s._shape(view, id, bone, {"kind": "poly", "points": [[r5(p[0]), r5(p[1])] for p in pts], "closed": closed}, role, z, False, tone, None, None, pen=pen)


def tapered_prism(profile, w0, w1, tip, length):
    """Closed polyhedron of a (u, v) profile whose lateral span runs linearly from [w0, w1] at u = 0 to `tip` at u = length (the 3D twin of `zTip`)."""
    n = len(profile)
    area = sum(profile[i][0] * profile[(i + 1) % n][1] - profile[(i + 1) % n][0] * profile[i][1] for i in range(n))
    pts = profile if area > 0 else list(reversed(profile))
    lo = lambda u: w0 + (tip[0] - w0) * u / length
    hi = lambda u: w1 + (tip[1] - w1) * u / length
    verts = [[r5(p[0]), r5(p[1]), r5(lo(p[0]))] for p in pts] + [[r5(p[0]), r5(p[1]), r5(hi(p[0]))] for p in pts]
    tris = []
    for a, b, c in ear_clip(pts):
        tris.append([c, b, a])
        tris.append([a + n, b + n, c + n])
    for i in range(n):
        j = (i + 1) % n
        tris.append([i, j, j + n])
        tris.append([i, j + n, i + n])
    return {"vertices": verts, "triangles": tris}


def ear_clip(pts):
    n = len(pts)
    idx = list(range(n))
    out = []
    cross = lambda o, a, b: (a[0] - o[0]) * (b[1] - o[1]) - (a[1] - o[1]) * (b[0] - o[0])

    def inside(p, a, b, c):
        return cross(a, b, p) >= 0 and cross(b, c, p) >= 0 and cross(c, a, p) >= 0

    guard = 0
    while len(idx) > 3 and guard < 10000:
        guard += 1
        for k in range(len(idx)):
            i, j, l = idx[k - 1], idx[k], idx[(k + 1) % len(idx)]
            a, b, c = pts[i], pts[j], pts[l]
            if cross(a, b, c) <= 1e-12:
                continue
            if any(inside(pts[m], a, b, c) for m in idx if m not in (i, j, l)):
                continue
            out.append((i, j, l))
            idx.pop(k)
            break
    out.append(tuple(idx))
    return out


def band(spine, half, n=None):
    """Closed outline of a ribbon of half-thickness `half(t)` around a polyline spine (u, v) sampled at its vertices."""
    m = len(spine)
    left, right = [], []
    for i, p in enumerate(spine):
        a = spine[max(0, i - 1)]
        b = spine[min(m - 1, i + 1)]
        dx, dy = b[0] - a[0], b[1] - a[1]
        L = math.hypot(dx, dy) or 1
        nx, ny = -dy / L, dx / L
        h = half(i / (m - 1))
        left.append((p[0] + nx * h, p[1] + ny * h))
        right.append((p[0] - nx * h, p[1] - ny * h))
    return left + list(reversed(right))


def arc(p0, p1, sag, n):
    """Points of a circular-ish arc from p0 to p1 bulging by `sag` to the left of the chord."""
    dx, dy = p1[0] - p0[0], p1[1] - p0[1]
    L = math.hypot(dx, dy)
    nx, ny = -dy / L, dx / L
    return [(p0[0] + dx * t + nx * sag * 4 * t * (1 - t), p0[1] + dy * t + ny * sag * 4 * t * (1 - t)) for t in (i / (n - 1) for i in range(n))]


def thermal_and_structure(sc, emitter_solids, supports, extra_members=None):
    """The thermal list (every body solid by reference in its region, one outlet per emitter) and the structure list (one member per bone by reference, one support per anchor)."""
    thermal = [{"id": "th-" + b["id"], "of": b["id"], "region": sc.regions[b["id"]]} for b in sc.body]
    thermal += emitter_solids
    rods = [b for b in sc.body if b["id"] in sc.rods]
    structure = [{"id": "m-" + b["id"], "of": b["id"], "member": b["bone"], "part": "member"} for b in rods]
    with_rods = {b["bone"] for b in rods}
    structure += [{"id": "m-" + bone, "of": sid, "member": bone, "part": "member"} for bone, sid in sc.member_of.items() if bone not in with_rods]
    structure += extra_members or []
    structure += supports
    return thermal, structure


def rep(sc, lo, hi, pens_, decisions_, hatches=None):
    return {"status": "authored", "denominator": sc.d, "validRange": {"minDenominator": lo, "maxDenominator": hi}, "pens": pens_, "hatches": hatches or [], "geometry": sc.views, "decisions": decisions_}


def write(asset, folder):
    doc = {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": asset}
    path = os.path.join(S, "🗂️catalog/🌳️vegetation", folder, "🔣️.json")
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w", encoding="utf8") as f:
        json.dump(doc, f, ensure_ascii=False, indent=1)
        f.write("\n")
    return path


def manifest_row(asset, distinction):
    a = asset
    row = {"id": a["id"], "category": a["category"], "title": a["title"], "heightM": a["dimensions"]["heightM"], "widthM": a["dimensions"]["widthM"], "powerW": 0,
           "poses": [p["id"] for p in a["poses"] if not p.get("key")], "clips": [c["id"] for c in a["clips"]],
           "representations": {k: a["representations"][k]["status"] for k in ("50", "100", "200", "500")}, "symbols": [x["id"] for x in a["symbols"]],
           "analytics": ["thermal", "transport", "structural"], "exports": a["exports"], "distinction": distinction, "completion": a["completion"]}
    os.makedirs(WAVE + "/📥️manifest-rows", exist_ok=True)
    with open(WAVE + "/📥️manifest-rows/" + a["id"] + ".json", "w", encoding="utf8") as f:
        json.dump(row, f, ensure_ascii=False, indent=1)
        f.write("\n")


def sway(rig_bones, keys_pose, duration, base, specs, n=12):
    """A looping, periodic wind-sway clip body: integer harmonics so the last key equals the first (wraps the NW-V2 `sway_clip` key schedule)."""
    from lib import sway_clip
    c = sway_clip(rig_bones, "x", T("x", "x"), T("x", "x"), duration, base, specs, n=n)
    for k in c["keys"]:
        k["pose"] = keys_pose
    return c["keys"]


def orient(rig, parent, direction, roll=0.0, up=(0.0, 1.0, 0.0)):
    """Rest `angleDeg`, `abductionDeg` and `twistDeg` of a child of `parent` whose axis u points along the world `direction` and whose normal v is the
    world `up` made perpendicular to it, then rolled by `roll` degrees about u (a leaf whose arch bulges upwards and whose blade is tilted by the roll)."""
    PR = rig.frame[parent][1]
    n = math.sqrt(sum(x * x for x in direction))
    d = mat3_vec(transpose(PR), [x / n for x in direction])
    b = math.degrees(math.asin(max(-1.0, min(1.0, d[2]))))
    a = math.degrees(math.atan2(d[1], d[0]))
    R0 = mat3_mul(PR, rest_rotation(a, b, 0))
    v0 = [R0[k][1] for k in range(3)]
    w0 = [R0[k][2] for k in range(3)]
    t = math.degrees(math.atan2(-sum(up[k] * w0[k] for k in range(3)), sum(up[k] * v0[k] for k in range(3)))) + roll
    return a, b, t


def direction(azimuth, elevation):
    """World unit vector at `azimuth` degrees from +x towards +z (left) and `elevation` degrees above the horizontal."""
    ce = math.cos(math.radians(elevation))
    return [ce * math.cos(math.radians(azimuth)), math.sin(math.radians(elevation)), ce * math.sin(math.radians(azimuth))]


def make_pose(id, title, desc, bones, thermal, loads_):
    return {"id": id, "title": title, "description": desc, "bones": bones, "ground": "authored", "contacts": [], "thermal": thermal, "loads": loads_}


def wind_sway_clip(rig_, pose_id, duration, base, specs, description, emitter):
    """Looping, periodic sway about the lean of `pose_id`: integer harmonics of the NW-V2 key schedule, so the last key is the first."""
    return {"id": "wind-sway", "title": T("Wind sway", "Windschwingung"), "description": description, "durationS": duration, "fps": 30, "loop": True, "rootMotion": "none", "keys": sway(rig_.bones, pose_id, duration, base, specs, n=12), "contacts": [],
            "phases": [{"id": "sway", "fromS": 0, "toS": duration, "title": T("Sway cycle", "Schwingungszyklus")}], "events": [{"t": 0, "kind": "motion", "title": T("Cycle start at the mean lean", "Zyklusbeginn in mittlerer Neigung")}],
            "tracks": [{"target": f"emitter:{emitter}", "ease": "smooth", "keys": [{"t": 0, "v": 1}, {"t": r5(duration / 2), "v": 0.92}, {"t": duration, "v": 1}]}]}


GUST = [(0, 0.05), (1.2, 0.45), (2.5, 2.75), (3.6, 1.4), (5.0, 0.5), (8.0, 0.05)]


def gust_clip(pose_id, lean, load_ids, description):
    """A single 8 s gust after EN 1991-1-4 (peak factor 1 + 7 Iv with Iv 0.25 on the mean pressure): load tracks scale the loads of `pose_id`, `lean(f)` gives the bone offsets at a share f of the storm lean."""
    keys = [{"t": t, "pose": pose_id, "ease": "smooth", "bones": lean(min(1.0, f / 2.75) * 0.7)} for t, f in GUST]
    return {"id": "gust", "title": T("Gust", "Windböe"), "description": description, "durationS": 8, "fps": 30, "loop": False, "rootMotion": "none", "keys": keys, "contacts": [],
            "phases": [{"id": "rise", "fromS": 0, "toS": 2.5, "title": T("Gust front", "Böenfront")}, {"id": "peak", "fromS": 2.5, "toS": 3.6, "title": T("Peak", "Spitze")}, {"id": "decay", "fromS": 3.6, "toS": 8, "title": T("Decay and recovery", "Abklingen und Erholung")}],
            "events": [{"t": 2.5, "kind": "load-on", "title": T("Peak gust load", "Böenspitze")}, {"t": 5, "kind": "load-off", "title": T("Load below the mean again", "Last wieder unter dem Mittelwert")}],
            "tracks": [{"target": f"load:{i}", "ease": "smooth", "keys": [{"t": t, "v": f} for t, f in GUST]} for i in load_ids]}


SUN = [(0, 0), (6, 0), (8, 0.43), (10, 0.79), (12, 1), (14, 0.79), (16, 0.36), (18, 0), (24, 0)]


def diurnal_clip(pose_id, emitter, strength, peaks, description):
    """A 24 h day compressed to 24 s (1 s = 1 h): sunlit fractions follow the sun (`peaks` per region at noon), the emitter follows `strength` (time, value)."""
    return {"id": "diurnal-transpiration", "title": T("Day of transpiration", "Transpirationstag"), "description": description, "durationS": 24, "fps": 10, "loop": True, "rootMotion": "none",
            "keys": [{"t": 0, "pose": pose_id, "ease": "smooth"}, {"t": 12, "pose": pose_id, "ease": "smooth"}, {"t": 24, "pose": pose_id, "ease": "smooth"}], "contacts": [],
            "phases": [{"id": "night", "fromS": 0, "toS": 6, "title": T("Night", "Nacht")}, {"id": "morning", "fromS": 6, "toS": 12, "title": T("Morning to noon", "Morgen bis Mittag")}, {"id": "afternoon", "fromS": 12, "toS": 18, "title": T("Afternoon", "Nachmittag")}, {"id": "evening", "fromS": 18, "toS": 24, "title": T("Evening and night", "Abend und Nacht")}],
            "events": [{"t": 6, "kind": "thermal", "title": T("Sunrise", "Sonnenaufgang")}, {"t": 12, "kind": "thermal", "title": T("Solar noon, peak transpiration", "Sonnenhöchststand, maximale Transpiration")}, {"t": 18, "kind": "thermal", "title": T("Sunset", "Sonnenuntergang")}],
            "tracks": [{"target": f"emitter:{emitter}", "ease": "smooth", "keys": [{"t": t, "v": v} for t, v in strength]}] + [{"target": f"region:{rid}:sunlitFraction", "ease": "smooth", "keys": [{"t": t, "v": r5(v * peak)} for t, v in SUN]} for rid, peak in peaks]}


SUPPORT_EDGE = {50: 0.06, 100: 0.07, 200: 0.075, 500: 0.08}


def support_blocks(d, supports, material_of):
    """One support block per anchor and scale (edge per scale, at least the smallest feature), half in the ground at the foot of its bone."""
    a = SUPPORT_EDGE[d]
    return [{"id": f"support-{aid}", "kind": "box", "bone": bone, "material": material_of(bone), "role": "body", "tone": 0.6, "size": [a, a, a], "transform": {"translation": [a / 2 - 0.02, 0, 0]}, "member": bone, "part": "support", "anchor": aid} for aid, bone in supports]


def outlet(d, emitter, bone, u, material, region):
    """The thermal solid tagged with the emitter id (no surface, it marks where the vapour leaves), sized and placed per scale."""
    r = {50: 0.03, 100: 0.035, 200: 0.04, 500: 0.045}[d]
    du = {50: 0.0, 100: 0.02, 200: 0.04, 500: 0.06}[d]
    return {"id": f"outlet-{emitter}", "kind": "ellipsoid", "bone": bone, "material": material, "role": "detail", "tone": 0.5, "radii": [r, r, r], "transform": {"translation": [u + du, 0, 0], "rotation": [0, 0, 0, 1]}, "region": region, "tags": [emitter]}
