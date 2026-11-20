"""📚 Round 3, furniture/bookshelf: the measures of the real bookcase (flat-pack carcass of 18 mm melamine-faced chipboard, 0.80 x 0.30 x 1.80 m, sides down to the floor, fixed top and bottom between the sides on cam-and-bolt fittings, a recessed kick board, a 3 mm hardboard back in grooves, five shelves on 5 mm steel pins in System 32 rows) and the helpers that turn world coordinates into bone-local solids.

World frame: x across the width from the left side panel (0) to the right (0.80), y up from the floor, z through the depth (open front at -0.15, back at +0.15). Every measure is metres.
"""

W, H, D = 0.8, 1.8, 0.3
T = 0.018
ZF, ZB = -D / 2, D / 2
BOTTOM_Y0, BOTTOM_Y1 = 0.06, 0.078
TOP_Y0 = H - T
KICK_Z0, KICK_Z1 = ZF + 0.03, ZF + 0.03 + T
GROOVE_Z0, GROOVE_Z1, GROOVE_DEPTH = 0.137, 0.141, 0.008
BACK_T = 0.003
BACK_Z0 = (GROOVE_Z0 + GROOVE_Z1 - BACK_T) / 2
BACK_PLAY = 0.0005
PITCH = 0.032
SHELF_Y = tuple(round(0.352 + 9 * PITCH * k, 3) for k in range(5))
SHELF_PLAY = 0.001
SPAN = W - 2 * T
HALF = SPAN / 2
SHELF_Z0, SHELF_Z1 = ZF + 0.005, ZF + 0.005 + 0.28
PIN_R, PIN_IN, PIN_OUT, COLLAR_R, COLLAR_T = 0.0025, 0.008, 0.009, 0.0035, 0.001
PIN_Z0, PIN_ROWS = ZF + 0.037, 7 * PITCH
CAM_R, CAM_DEPTH, CAM_CAP, CAM_X = 0.0075, 0.0125, 0.0005, 0.034
CAM_Z0, CAM_STEP = -0.1, 0.187
BOLT_R, BOLT_IN = 0.0035, 0.011

BOARD, HARDBOARD, STEEL = "melamine-chipboard", "hardboard-back", "zinc-plated-steel"
TONE = {BOARD: 0.7, HARDBOARD: 0.9, STEEL: 0.3}


def r6(value):
    """🔢 Rounds to a hundredth of a millimetre so the written numbers stay readable."""
    return round(value + 0.0, 5)


def frame_of(bone):
    """🧭 Origin (x, y) and hand of a bone frame: the frame, a shelf carrier `shelf-N`, its left half `shelf-N-l` (same frame) or its right half `shelf-N-r` (origin at the right support, turned by 180 degrees)."""
    if bone in ("frame", "ground"):
        return 0.0, 0.0, 1
    n = int(bone.split("-")[1])
    if bone.endswith("-r"):
        return W - T, SHELF_Y[n - 1], -1
    return T, SHELF_Y[n - 1], 1


def local(bone, x, y):
    """📍 A world point (x, y) in the frame of a bone."""
    ox, oy, hand = frame_of(bone)
    return r6(hand * (x - ox)), r6(hand * (y - oy))


def base(sid, kind, bone, material, tone=None, role="body"):
    """🏷 The common members of a solid."""
    return {"id": sid, "kind": kind, "bone": bone, "material": material, "role": role, "tone": TONE[material] if tone is None else tone}


def box(sid, bone, material, x, y, z, tone=None, role="body", **more):
    """📦 A box between world extents x, y, z (each a pair)."""
    out = base(sid, "box", bone, material, tone, role)
    cx, cy = local(bone, (x[0] + x[1]) / 2, (y[0] + y[1]) / 2)
    out["size"] = [r6(x[1] - x[0]), r6(y[1] - y[0]), r6(z[1] - z[0])]
    out["transform"] = {"translation": [cx, cy, r6((z[0] + z[1]) / 2)]}
    out.update(more)
    return out


def upright(sid, profile_zx, y, material=BOARD, tone=None, **more):
    """⬆ A panel standing on its plan section: world (z, x) points pushed up from y[0] to y[1] (frame bone)."""
    out = base(sid, "extrude", "frame", material, tone)
    out.update({"profile": [[r6(z), r6(x)] for z, x in profile_zx], "plane": "zx", "from": r6(y[0]), "to": r6(y[1])})
    out.update(more)
    return out


def across(sid, profile_yz, x, material=BOARD, tone=None, **more):
    """➡ A panel lying across the width: world (y, z) points pushed from x[0] to x[1] (frame bone)."""
    out = base(sid, "extrude", "frame", material, tone)
    out.update({"profile": [[r6(y), r6(z)] for y, z in profile_yz], "plane": "yz", "from": r6(x[0]), "to": r6(x[1])})
    out.update(more)
    return out


def facing(sid, profile_xy, holes, z, material=BOARD, tone=None, **more):
    """🖼 A plate seen in the elevation: world (x, y) outline with holes pushed through the depth from z[0] to z[1] (frame bone)."""
    out = base(sid, "extrude", "frame", material, tone)
    out.update({"profile": [[r6(x), r6(y)] for x, y in profile_xy], "plane": "xy", "from": r6(z[0]), "to": r6(z[1])})
    if holes:
        out["holes"] = [[[r6(x), r6(y)] for x, y in hole] for hole in holes]
    out.update(more)
    return out


def turned(sid, bone, material, profile, axis, origin, tone=None, role="detail", **more):
    """⭕ A revolve of an (axial, radial) profile about a world axis direction through a world origin."""
    ox, oy, hand = frame_of(bone)
    out = base(sid, "revolve", bone, material, tone, role)
    lx, ly = local(bone, origin[0], origin[1])
    out.update({"profile": [[r6(a), r6(r)] for a, r in profile], "axis": [hand * axis[0], hand * axis[1], axis[2]], "origin": [lx, ly, r6(origin[2])]})
    out.update(more)
    return out


def ellipsoid(sid, bone, material, centre, radii, tone=None, role="detail", **more):
    """🥚 An ellipsoid at a world centre."""
    out = base(sid, "ellipsoid", bone, material, tone, role)
    cx, cy = local(bone, centre[0], centre[1])
    out["radii"] = [r6(r) for r in radii]
    out["transform"] = {"translation": [cx, cy, r6(centre[2])]}
    out.update(more)
    return out


def repeat(solid, count, dw):
    """🔁 Copies a solid `count` times through the depth."""
    solid["repeat"] = {"count": count, "translation": [0, 0, r6(dw)]}
    return solid


def masses():
    """⚖ Mass (kg) and height of the centre of the carcass from the modelled boards (chipboard 650 kg/m3, hardboard 900 kg/m3) plus 0.25 kg of fittings, and the mass of one shelf half."""
    parts = [
        (2 * H * D * T * 650, H / 2),
        (SPAN * D * T * 650, TOP_Y0 + T / 2),
        (SPAN * D * T * 650, BOTTOM_Y0 + T / 2),
        ((SPAN + 0.015) * 1.719 * BACK_T * 900, 0.93),
        (SPAN * BOTTOM_Y0 * T * 650, BOTTOM_Y0 / 2),
        (0.25, H / 2),
    ]
    kg = sum(p[0] for p in parts)
    return {"frame": (round(kg, 1), round(sum(p[0] * p[1] for p in parts) / kg, 3)), "half": round((SPAN - 2 * SHELF_PLAY) / 2 * 0.28 * T * 650, 2)}


if __name__ == "__main__":
    print(SHELF_Y, masses())
