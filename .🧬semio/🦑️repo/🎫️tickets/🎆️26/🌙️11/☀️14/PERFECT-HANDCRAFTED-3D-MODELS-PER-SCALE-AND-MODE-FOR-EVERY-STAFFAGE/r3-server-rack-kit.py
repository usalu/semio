"""🗄 Round 3, appliances/server-rack: the measures of the real 42U cabinet and the helpers that turn world coordinates into bone-local solids and shapes.

World frame: x across the cabinet (the 0.6 m width, the axis of the side elevation drawing), y up, z through the depth (front door at -0.5, rear doors at +0.5). Every bone of the corrected rig points up (frame angle 90 degrees, the children 0 relative to it), so bone-local u = world y - joint height, v = -world x, w = world z.
"""
import math

HALF_W, HALF_D, HEIGHT = 0.30, 0.50, 2.0
U = 0.04445
RAIL_Y0 = 0.10
RAIL_Y1 = RAIL_Y0 + 42 * U
EAR = 0.4826 / 2
BODY_X = 0.2215
FRONT_FLANGE_Z = -0.40
REAR_FLANGE_Z = 0.36
BASE_Y0, BASE_Y1 = 0.07, 0.10
TOP_Y0, TOP_Y1 = 1.937, 1.967
ROOF_Y1 = HEIGHT
FRAME_Z = 0.475
POST = 0.04
POST_X1 = 0.28
DOOR_X = 0.27
DOOR_Y0, DOOR_Y1 = 0.10, 1.967
DOOR_DEPTH = 0.025
CASTER_X, CASTER_SWIVEL_Z, CASTER_WHEEL_Z = 0.25, 0.425, 0.45
WHEEL_R, WHEEL_W = 0.025, 0.02
FAN_Z = (0.10, 0.32)
FAN_R = 0.08
SLOT = (-0.15, 0.15, 0.41, 0.43)

UNITS = [
    ("storage", 1, 4, "servers"),
    ("server-5", 5, 2, "servers"),
    ("server-7", 7, 2, "servers"),
    ("server-9", 9, 2, "servers"),
    ("server-11", 11, 2, "servers"),
    ("server-13", 13, 2, "servers"),
    ("server-15", 15, 2, "servers"),
    ("patch-40", 40, 1, "blanks"),
    ("brush-41", 41, 1, "blanks"),
    ("switch-42", 42, 1, "switch"),
]


def unit_y(first, height):
    """📏 Bottom and top of a unit of `height` U whose lowest unit is `first`, with the 0.8 mm clearance of IEC 60297-3-100 split above and below."""
    y0 = RAIL_Y0 + (first - 1) * U
    return y0 + 0.0004, y0 + height * U - 0.0004


ORIGIN = {"frame": 0.0, "feet": 0.0, "equipment": RAIL_Y0, "door": RAIL_Y0}
TONE = {"rack-steel": 0.5, "mesh-steel": 0.18, "chassis": 0.42, "blank-steel": 0.38, "caster-rubber": 0.2}


def r6(value):
    """🔢 Rounds to a hundredth of a millimetre."""
    return round(value + 0.0, 5)


def uvw(bone, x, y, z):
    """🧭 A world point in the frame of an upright bone."""
    return [r6(y - ORIGIN[bone]), r6(-x), r6(z)]


def base(sid, kind, bone, material, tone=None, role="body"):
    """🏷 The common members of a solid."""
    return {"id": sid, "kind": kind, "bone": bone, "material": material, "role": role, "tone": TONE[material] if tone is None else tone}


def box(sid, bone, material, x, y, z, radius=0.0, tone=None, role="body", **more):
    """📦 A box between the world extents x, y, z (pairs)."""
    out = base(sid, "box", bone, material, tone, role)
    out["size"] = [r6(y[1] - y[0]), r6(x[1] - x[0]), r6(z[1] - z[0])]
    if radius:
        out["cornerRadius"] = radius
    out["transform"] = {"translation": uvw(bone, (x[0] + x[1]) / 2, (y[0] + y[1]) / 2, (z[0] + z[1]) / 2)}
    out.update(more)
    return out


def up(sid, bone, material, profile_xz, y, holes=(), tone=None, role="body", **more):
    """⬆ A prism standing up: a world profile of (x, z) points pushed from y[0] to y[1]."""
    oy = ORIGIN[bone]
    out = base(sid, "extrude", bone, material, tone, role)
    out.update({"profile": [[r6(-x), r6(z)] for x, z in profile_xz], "plane": "yz", "from": r6(y[0] - oy), "to": r6(y[1] - oy)})
    if holes:
        out["holes"] = [[[r6(-x), r6(z)] for x, z in ring] for ring in holes]
    out.update(more)
    return out


def across(sid, bone, material, profile_yz, x, holes=(), tone=None, role="body", **more):
    """↔ A prism across the width: a world profile of (y, z) points pushed from x[0] to x[1]."""
    oy = ORIGIN[bone]
    out = base(sid, "extrude", bone, material, tone, role)
    out.update({"profile": [[r6(z), r6(y - oy)] for y, z in profile_yz], "plane": "zx", "from": r6(-x[1]), "to": r6(-x[0])})
    if holes:
        out["holes"] = [[[r6(z), r6(y - oy)] for y, z in ring] for ring in holes]
    out.update(more)
    return out


def deep(sid, bone, material, profile_xy, z, holes=(), tone=None, role="body", **more):
    """↘ A prism through the depth: a world profile of (x, y) points pushed from z[0] to z[1]."""
    oy = ORIGIN[bone]
    out = base(sid, "extrude", bone, material, tone, role)
    out.update({"profile": [[r6(y - oy), r6(-x)] for x, y in profile_xy], "plane": "xy", "from": r6(z[0]), "to": r6(z[1])})
    if holes:
        out["holes"] = [[[r6(y - oy), r6(-x)] for x, y in ring] for ring in holes]
    out.update(more)
    return out


def turned(sid, bone, material, profile, axis, origin, tone=None, role="body", **more):
    """⭕ A revolve of an (axial, radial) profile about a world axis ('x', 'y' or 'z') through a world origin."""
    out = base(sid, "revolve", bone, material, tone, role)
    out.update({"profile": [[r6(a), r6(r)] for a, r in profile], "axis": {"x": [0, -1, 0], "y": [1, 0, 0], "z": [0, 0, 1]}[axis], "origin": uvw(bone, *origin)})
    out.update(more)
    return out


def blob(sid, bone, material, centre, radii, tone=None, role="body", **more):
    """🥚 An ellipsoid at a world centre with world semi-axes (rx, ry, rz)."""
    out = base(sid, "ellipsoid", bone, material, tone, role)
    out.update({"radii": [r6(radii[1]), r6(radii[0]), r6(radii[2])], "transform": {"translation": uvw(bone, *centre)}})
    out.update(more)
    return out


def repeat(solid, count, dx=0.0, dy=0.0, dz=0.0):
    """🔁 Copies a solid `count` times along a world step."""
    solid["repeat"] = {"count": count, "translation": [r6(dy), r6(-dx), r6(dz)]}
    return solid


def rect(x0, x1, z0, z1):
    """⬜ A counter-clockwise rectangle."""
    return [(x0, z0), (x1, z0), (x1, z1), (x0, z1)]


def rounded(a, b, radius, steps=3):
    """🔲 A counter-clockwise rectangle a = (a0, a1) by b = (b0, b1) with one corner radius."""
    if radius <= 0:
        return rect(a[0], a[1], b[0], b[1])
    points = []
    for ca, cb, start in ((a[0] + radius, b[0] + radius, 180), (a[1] - radius, b[0] + radius, 270), (a[1] - radius, b[1] - radius, 0), (a[0] + radius, b[1] - radius, 90)):
        for k in range(steps + 1):
            angle = math.radians(start + 90 * k / steps)
            points.append((ca + radius * math.cos(angle), cb + radius * math.sin(angle)))
    return points


def circle(cx, cz, radius, steps=24):
    """⚪ A counter-clockwise polygon of a circle."""
    return [(cx + radius * math.cos(2 * math.pi * k / steps), cz + radius * math.sin(2 * math.pi * k / steps)) for k in range(steps)]


def grid(x0, x1, y0, y1, hole, pitch, shift=(0.0, 0.0)):
    """🔳 The square holes of a perforated field: `hole` wide at `pitch`, the pattern shifted by `shift`, only whole holes inside the field."""
    out = []
    nx = int((x1 - x0 - hole) / pitch + 1e-9)
    ny = int((y1 - y0 - hole) / pitch + 1e-9)
    mx = (x1 - x0 - hole - nx * pitch) / 2 + shift[0]
    my = (y1 - y0 - hole - ny * pitch) / 2 + shift[1]
    for j in range(ny + 1):
        for i in range(nx + 1):
            hx, hy = x0 + mx + i * pitch, y0 + my + j * pitch
            if hx >= x0 - 1e-9 and hx + hole <= x1 + 1e-9 and hy >= y0 - 1e-9 and hy + hole <= y1 + 1e-9:
                out.append([(r6(hx), r6(hy)), (r6(hx + hole), r6(hy)), (r6(hx + hole), r6(hy + hole)), (r6(hx), r6(hy + hole))])
    return out


def rig(old):
    """🦴 The corrected rig: every bone stands upright from its real joint, the frame from the floor, the casters under it, the equipment stack and the front door from the lowest height unit; masses at the real centres."""
    spec = {
        "frame": dict(x=0, y=0, angleDeg=90, lengthM=2.0, com=(1.0, 0)),
        "feet": dict(x=0, y=0, angleDeg=0, lengthM=0.07, com=(0.035, 0)),
        "equipment": dict(x=RAIL_Y0, y=0, angleDeg=0, lengthM=r6(RAIL_Y1 - RAIL_Y0), com=(0.458, 0.0)),
        "door": dict(x=RAIL_Y0, y=0, angleDeg=0, lengthM=r6(DOOR_Y1 - DOOR_Y0), com=(0.9335, 0.0)),
    }
    bones = []
    for bone in old["bones"]:
        s = spec[bone["id"]]
        bone = dict(bone, x=s["x"], y=s["y"], angleDeg=s["angleDeg"], lengthM=s["lengthM"])
        bone["mass"] = dict(bone["mass"], comX=s["com"][0], comY=s["com"][1])
        bones.append(bone)
    return dict(old, bones=bones)
