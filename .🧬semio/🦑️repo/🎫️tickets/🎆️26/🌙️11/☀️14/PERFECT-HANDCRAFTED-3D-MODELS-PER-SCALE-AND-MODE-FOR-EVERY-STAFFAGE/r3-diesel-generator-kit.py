"""🧰 Round 3, machinery/diesel-generator: one table of real measures (world metres at rest: x ahead along the container, y up, z lateral, left is +, the drawn elevation side is -z) that the 2D drawings and the solids of every scale both read, the rest frames of the corrected rig and the solid and shape builders.

References: ISO 668 (20 ft 1CC), ISO 1161 (corner castings 178 x 162 x 118 mm), ISO 1496-1 (frame), see `r3-machinery-diesel-generator.md`.
"""
import math

L2 = 3.03
W2 = 1.22
H = 2.59
CAST = (0.178, 0.118, 0.162)
CX = L2 - CAST[0] / 2
CZ = W2 - CAST[2] / 2
CAST_IN_X = L2 - CAST[0]
CAST_IN_Z = W2 - CAST[2]
POST = (0.16, 0.15)
RAIL_Y = (0.013, 0.2)
RAIL_D = 0.05
TOP_RAIL_Y = (2.47, 2.582)
TOP_RAIL_D = 0.06
TOP_END_Y = (2.43, 2.582)
WALL_X = 3.0
WALL_Z = 1.19
WALL_Y = (0.2, 2.47)
ROOF_X = 2.97
ROOF_Z = 1.16
ROOF_TOP = 2.576
TANK_Z = W2 - RAIL_D
LOUVRES = ((-2.8, -1.5), (-0.2, 1.1))
LOUVRE_Y = (0.55, 2.05)
LOUVRE_FRAME = 0.05
DOORS = ((1.5, 2.25), (2.35, 2.85))
DOOR_Y = (0.45, 2.15)
END_DOOR_Z = (-0.5, 0.5)
FUEL_FILL = ((-2.75, -2.35), (0.06, 0.16))
EYE_X = (2.5, 2.8)
EYE_Z = 1.0

SIL_X = (-2.01, 0.21)
SIL_Y = 2.965
SIL_Z = -0.9
SIL_R = 0.285
SIL_HEAD = 0.145
SADDLES = (-1.54, -0.44)
INLET_X = -0.25
STACK_X = -1.2
STACK_R = 0.1
STACK_Y = (3.1, 3.47)
CAP_R = 0.15

HOOD = [(1.9, ROOF_TOP), (L2, ROOF_TOP), (L2, 3.25), (2.2, 3.25), (1.9, 2.97)]
HOOD_Z = 0.9
MOUTH_Z = 0.85
MOUTH_Y = (2.65, 3.2)
FLAP = (0.63, 0.06, 1.7)

BONES = {
    "container": {"x": -L2, "y": 0.0},
    "silencer": {"x": SIL_X[0], "y": SIL_Y},
    "outlet-hood": {"x": HOOD[0][0], "y": HOOD[0][1]},
    "stack": {"x": STACK_X, "y": 3.14, "angleDeg": 90},
    "cap-flap": {"x": -1.05, "y": 3.47, "angleDeg": 180},
    "outlet-flap": {"x": L2, "y": 3.25, "angleDeg": -90},
}


def R5(v):
    """🎯 Five decimals, no negative zero."""
    r = round(v, 5)
    return 0.0 if r == 0 else r


def origin(bone):
    """📍 World rest origin of a bone."""
    b = BONES[bone]
    return b["x"], b["y"]


def to_local(bone, x, y):
    """🔁 World (x, y) at rest into the elevation frame (u, v) of a bone."""
    ox, oy = origin(bone)
    a = math.radians(BONES[bone].get("angleDeg", 0))
    dx, dy = x - ox, y - oy
    return R5(dx * math.cos(a) + dy * math.sin(a)), R5(-dx * math.sin(a) + dy * math.cos(a))


def lp(bone, points):
    """🔁 A world point list in the frame of a bone."""
    return [list(to_local(bone, x, y)) for x, y in points]


def T(en, de):
    """🌐 Bilingual text."""
    return {"en": en, "de": de}


def base(id, bone, material, role, tone, **kw):
    """🧱 The common members of a solid, in the member order of the catalog."""
    out = {"id": id}
    out.update(kw.pop("_kind"))
    out.update({"bone": bone, "material": material, "role": role, "tone": tone})
    out.update(kw)
    return out


def _extra(kw):
    keep = {}
    for key in ("sectionFaces", "texture", "slender", "envelope", "tags", "repeat"):
        if key in kw and kw[key] is not None:
            keep[key] = kw[key]
    return keep


def box(id, bone, material, role, tone, x0, x1, y0, y1, z0, z1, radius=None, **kw):
    """📦 A box given by its world extents at rest (bones with a translation-only frame)."""
    ox, oy = origin(bone)
    out = {"id": id, "kind": "box", "bone": bone, "material": material, "role": role, "tone": tone,
           "size": [R5(x1 - x0), R5(y1 - y0), R5(z1 - z0)]}
    if radius:
        out["cornerRadius"] = radius
    out["transform"] = {"translation": [R5((x0 + x1) / 2 - ox), R5((y0 + y1) / 2 - oy), R5((z0 + z1) / 2)]}
    out.update(_extra(kw))
    return out


def lbox(id, bone, material, role, tone, u0, u1, v0, v1, w0, w1, radius=None, rotation=None, **kw):
    """📦 A box given by its extents in the bone-local frame."""
    out = {"id": id, "kind": "box", "bone": bone, "material": material, "role": role, "tone": tone,
           "size": [R5(u1 - u0), R5(v1 - v0), R5(w1 - w0)]}
    if radius:
        out["cornerRadius"] = radius
    t = {"translation": [R5((u0 + u1) / 2), R5((v0 + v1) / 2), R5((w0 + w1) / 2)]}
    if rotation:
        t["rotation"] = rotation
    out["transform"] = t
    out.update(_extra(kw))
    return out


def extrude(id, bone, material, role, tone, plane, profile, z0, z1, holes=None, world=True, **kw):
    """🏗️ An extrude; with `world` the profile and range are world coordinates of a translation-only bone."""
    ox, oy = origin(bone) if world else (0.0, 0.0)
    if plane == "xy":
        pr = [[R5(a - ox), R5(b - oy)] for a, b in profile]
        hs = [[[R5(a - ox), R5(b - oy)] for a, b in h] for h in (holes or [])]
        f, t = z0, z1
    elif plane == "yz":
        pr = [[R5(a - oy), R5(b)] for a, b in profile]
        hs = [[[R5(a - oy), R5(b)] for a, b in h] for h in (holes or [])]
        f, t = z0 - ox, z1 - ox
    else:
        pr = [[R5(a), R5(b - ox)] for a, b in profile]
        hs = [[[R5(a), R5(b - ox)] for a, b in h] for h in (holes or [])]
        f, t = z0 - oy, z1 - oy
    out = {"id": id, "kind": "extrude", "bone": bone, "material": material, "role": role, "tone": tone, "profile": pr}
    if hs:
        out["holes"] = hs
    out.update({"plane": plane, "from": R5(f), "to": R5(t)})
    out.update(_extra(kw))
    return out


def revolve(id, bone, material, role, tone, profile, axis, at, world=True, sweep=None, lateral=None, rotation=None, **kw):
    """🌀 A revolve about `axis` through `at` (world point of a translation-only bone, else bone-local)."""
    ox, oy = origin(bone) if world else (0.0, 0.0)
    out = {"id": id, "kind": "revolve", "bone": bone, "material": material, "role": role, "tone": tone,
           "profile": [[R5(a), R5(r)] for a, r in profile], "axis": axis}
    o = [R5(at[0] - ox), R5(at[1] - oy), R5(at[2])]
    if rotation:
        out["origin"] = [0.0, 0.0, 0.0]
        out["transform"] = {"translation": o, "rotation": rotation}
    else:
        out["origin"] = o
    if sweep:
        out["sweepRad"] = sweep
    if lateral:
        out["lateralScale"] = lateral
    out.update(_extra(kw))
    return out


def ellipsoid(id, bone, material, role, tone, c, radii, **kw):
    """🥚 An ellipsoid at a world centre of a translation-only bone."""
    ox, oy = origin(bone)
    out = {"id": id, "kind": "ellipsoid", "bone": bone, "material": material, "role": role, "tone": tone,
           "radii": [R5(r) for r in radii], "transform": {"translation": [R5(c[0] - ox), R5(c[1] - oy), R5(c[2])]}}
    out.update(_extra(kw))
    return out


def ref(id, of, **kw):
    """🔗 A thermal or structural reference to a body solid."""
    out = {"id": id, "of": of}
    out.update(kw)
    return out


def silencer_profile(steps):
    """🛢️ The silencer shell as [axial, radial] about its centre: a cylinder with 2:1 dished heads, `steps` points per head quarter."""
    half = (SIL_X[1] - SIL_X[0]) / 2
    flat = half - SIL_HEAD
    pts = []
    for i in range(steps + 1):
        t = math.pi / 2 * i / steps
        pts.append((-flat - SIL_HEAD * math.cos(t), SIL_R * math.sin(t)))
    for i in range(steps + 1):
        t = math.pi / 2 * i / steps
        pts.append((flat + SIL_HEAD * math.sin(t), SIL_R * math.cos(t)))
    return pts


def silencer_outline(steps):
    """🛢️ The silencer outline in world (x, offset) for the elevation (offset = y) and plan (offset = z)."""
    prof = silencer_profile(steps)
    cx = (SIL_X[0] + SIL_X[1]) / 2
    top = [(cx + a, r) for a, r in prof]
    bottom = [(cx + a, -r) for a, r in reversed(prof)]
    out = []
    for p in top + bottom:
        if not out or abs(out[-1][0] - p[0]) > 1e-9 or abs(out[-1][1] - p[1]) > 1e-9:
            out.append(p)
    if abs(out[0][0] - out[-1][0]) < 1e-9 and abs(out[0][1] - out[-1][1]) < 1e-9:
        out.pop()
    return out


def saddle_profile(web_r, half, base_y):
    """🪑 The saddle web as world (y, z): from the roof plate up to a cradle arc of radius `web_r` under the silencer axis, `half` wide each side."""
    top = SIL_Y - math.sqrt(web_r ** 2 - half ** 2)
    pts = [(base_y, SIL_Z - half), (base_y, SIL_Z + half), (top, SIL_Z + half)]
    for i in range(1, 8):
        dz = half - 2 * half * i / 8
        pts.append((SIL_Y - math.sqrt(web_r ** 2 - dz ** 2), SIL_Z + dz))
    pts.append((top, SIL_Z - half))
    return pts


def quat(axis, deg):
    """🔄 A unit quaternion [x, y, z, w] about a principal axis."""
    h = math.radians(deg) / 2
    s = math.sin(h)
    v = {"x": [s, 0, 0], "y": [0, s, 0], "z": [0, 0, s]}[axis]
    return [R5(v[0]), R5(v[1]), R5(v[2]), R5(math.cos(h))]
