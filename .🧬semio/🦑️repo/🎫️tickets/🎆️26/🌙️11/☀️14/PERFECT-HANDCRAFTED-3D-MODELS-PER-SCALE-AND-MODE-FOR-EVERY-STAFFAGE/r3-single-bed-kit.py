"""🛏 Round 3, furniture/single-bed: the measures of the real bed (pine frame for a 0.90 x 2.00 m mattress, sprung slatted base with a lifting head section) and the small helpers that turn world coordinates into bone-local solids.

World frame: x from the head end to the foot end, y up, z lateral (left is +). Every measure below is metres.
"""
import math

POST = 0.06
LENGTH = 2.12
POST_Z = 0.475
HEAD_H = 0.95
FOOT_H = 0.55
CLEAR_Z = 0.455
RAIL_T = 0.03
RAIL_Y0, RAIL_Y1 = 0.14, 0.30
BOARD_Z = 0.445
LEDGER = 0.025
LEDGER_Y1 = 0.205
BASE_Y0, BASE_Y1 = 0.205, 0.25
BASE_Z0, BASE_Z1 = 0.42, 0.445
LIFT_Y0, LIFT_Y1 = 0.21, 0.25
LIFT_Z0, LIFT_Z1 = 0.39, 0.415
HINGE_X, HINGE_Y = 0.78, 0.23
FOOT_X = 1.38
SLAT_W, SLAT_T = 0.038, 0.008
SLAT_END_Y, SLAT_CAMBER = 0.266, 0.015
CAP = (0.05, 0.03, 0.025)
MAT_Y0, MAT_T = 0.285, 0.20
MAT_Z = 0.45
MAT_X0, MAT_X1 = 0.06, 2.06
MAT_E = 10
HEAD_SLATS = [0.15 + 0.065 * k for k in range(10)]
MID_SLATS = [0.815 + 0.068 * k for k in range(9)]
FOOT_SLATS = [1.427 + 0.068 * k for k in range(9)]

ORIGIN = {
    "ground": (0.0, 0.0),
    "frame": (0.0, 0.0),
    "slat-head": (HINGE_X, HINGE_Y),
    "slat-mid": (HINGE_X, HINGE_Y),
    "slat-foot": (FOOT_X, HINGE_Y),
    "mat-head": (HINGE_X, MAT_Y0),
    "mat-mid": (HINGE_X, MAT_Y0),
    "mat-foot": (FOOT_X, MAT_Y0),
}

PINE, BIRCH, QUILT, FOAM, RUBBER, STEEL = "pine-timber", "birch-slat", "quilted-cover", "cold-foam", "cap-rubber", "steel-fitting"
TONE = {PINE: 0.5, BIRCH: 0.6, QUILT: 0.85, FOAM: 0.75, RUBBER: 0.25, STEEL: 0.35}


def r6(value):
    """🔢 Rounds to a hundredth of a millimetre so the written numbers stay readable."""
    return round(value + 0.0, 5)


def base(sid, kind, bone, material, tone=None, role="body"):
    """🏷 The common members of a solid."""
    return {"id": sid, "kind": kind, "bone": bone, "material": material, "role": role, "tone": TONE[material] if tone is None else tone}


def box(sid, bone, material, x, y, z, radius=0.0, tone=None, role="body", **more):
    """📦 A box between world extents x, y, z (each a pair)."""
    ox, oy = ORIGIN[bone]
    out = base(sid, "box", bone, material, tone, role)
    out["size"] = [r6(x[1] - x[0]), r6(y[1] - y[0]), r6(z[1] - z[0])]
    if radius:
        out["cornerRadius"] = radius
    out["transform"] = {"translation": [r6((x[0] + x[1]) / 2 - ox), r6((y[0] + y[1]) / 2 - oy), r6((z[0] + z[1]) / 2)]}
    out.update(more)
    return out


def along_x(sid, bone, material, profile_yz, x, tone=None, role="body", **more):
    """➡ A prism along the bed: a world profile of (y, z) points pushed from x[0] to x[1]."""
    ox, oy = ORIGIN[bone]
    out = base(sid, "extrude", bone, material, tone, role)
    out.update({"profile": [[r6(y - oy), r6(z)] for y, z in profile_yz], "plane": "yz", "from": r6(x[0] - ox), "to": r6(x[1] - ox)})
    out.update(more)
    return out


def along_y(sid, bone, material, profile_zx, y, tone=None, role="body", **more):
    """⬆ A prism standing up: a world profile of (z, x) points pushed from y[0] to y[1]."""
    ox, oy = ORIGIN[bone]
    out = base(sid, "extrude", bone, material, tone, role)
    out.update({"profile": [[r6(z), r6(x - ox)] for z, x in profile_zx], "plane": "zx", "from": r6(y[0] - oy), "to": r6(y[1] - oy)})
    out.update(more)
    return out


def along_z(sid, bone, material, profile_xy, z, tone=None, role="body", **more):
    """↔ A prism across the bed: a world profile of (x, y) points pushed from z[0] to z[1]."""
    ox, oy = ORIGIN[bone]
    out = base(sid, "extrude", bone, material, tone, role)
    out.update({"profile": [[r6(x - ox), r6(y - oy)] for x, y in profile_xy], "plane": "xy", "from": r6(z[0]), "to": r6(z[1])})
    out.update(more)
    return out


def rounded(a, b, radii, steps=3):
    """⬜ A rectangle a = (a0, a1) by b = (b0, b1) with the four corner radii (a0b0, a1b0, a1b1, a0b1), counter-clockwise."""
    corners = [(a[0], b[0], 180), (a[1], b[0], 270), (a[1], b[1], 0), (a[0], b[1], 90)]
    points = []
    for (ca, cb, start), radius in zip(corners, radii):
        if radius <= 0:
            points.append((ca, cb))
            continue
        centre_a = ca + radius if ca == a[0] else ca - radius
        centre_b = cb + radius if cb == b[0] else cb - radius
        for k in range(steps + 1):
            angle = math.radians(start + 90 * k / steps)
            points.append((centre_a + radius * math.cos(angle), centre_b + radius * math.sin(angle)))
    return [p for k, p in enumerate(points) if math.dist(p, points[k - 1]) > 1e-9]


def loft(sid, bone, material, sections, tone=None, role="body", **more):
    """🎢 A loft through world sections (x, y, z, rx, ry, exponent)."""
    ox, oy = ORIGIN[bone]
    out = base(sid, "loft", bone, material, tone, role)
    out["sections"] = [{"at": [r6(x - ox), r6(y - oy), r6(z)], "rx": r6(rx), "ry": r6(ry), "exponent": e} for x, y, z, rx, ry, e in sections]
    out["capStart"] = True
    out["capEnd"] = True
    out.update(more)
    return out


def turned(sid, bone, material, profile, axis, origin, tone=None, role="body", **more):
    """⭕ A revolve of an (axial, radial) profile about a bone axis through a world origin."""
    ox, oy = ORIGIN[bone]
    out = base(sid, "revolve", bone, material, tone, role)
    out.update({"profile": [[r6(a), r6(r)] for a, r in profile], "axis": {"u": [1, 0, 0], "v": [0, 1, 0], "w": [0, 0, 1]}[axis], "origin": [r6(origin[0] - ox), r6(origin[1] - oy), r6(origin[2])]})
    out.update(more)
    return out


def repeat(solid, count, du=0.0, dv=0.0, dw=0.0):
    """🔁 Copies a solid `count` times along a step."""
    solid["repeat"] = {"count": count, "translation": [r6(du), r6(dv), r6(dw)]}
    return solid


def rig(old):
    """🦴 The corrected rig: the slat sections hinge on the bolt through the base rails (y 0.23 m), the mattress parts ride 0.055 m above it on the slat crowns, the floor reference carries no length."""
    bones = []
    for bone in old["bones"]:
        bone = dict(bone)
        if bone["id"] == "ground":
            bone["lengthM"] = 0
        if bone["id"].startswith("slat-"):
            bone["y"] = HINGE_Y
            bone["mass"] = dict(bone["mass"], comY=0.015)
        if bone["id"].startswith("mat-"):
            bone["y"] = r6(MAT_Y0 - HINGE_Y)
        bones.append(bone)
    return dict(old, bones=bones)
