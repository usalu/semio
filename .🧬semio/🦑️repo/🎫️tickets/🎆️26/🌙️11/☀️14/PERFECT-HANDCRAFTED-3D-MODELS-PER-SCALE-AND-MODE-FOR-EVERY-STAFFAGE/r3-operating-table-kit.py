"""🏥 Round 3, furniture/operating-table: the measures of the real objects (a mobile operating table with a five-part top on a telescopic column, and a mobile surgical light on a spring arm) and the part primitives. A part is stated once in world coordinates and carries its solid in the frame of its bone and its three true orthographic outlines, so the drawings and the model cannot drift apart.

World frame: x from the head end (-1.0) to the foot end (+1.0) and on to the light stand, y up, z lateral (left is +). Metres.
"""
import math

BASE_HALF_X, BASE_HALF_Z = 0.47, 0.27
BASE_Y0, BASE_Y1, BASE_SHOULDER = 0.05, 0.14, 0.12
SPINE_HALF_X, SPINE_HALF_Z = 0.33, 0.15
PLINTH_Y1 = 0.16
CASTOR_X, CASTOR_Z, CASTOR_R = 0.40, 0.21, 0.05
PIVOT_Y = 0.74
FRAME_Y0, FRAME_Y1, PAD_Y1 = 0.78, 0.82, 0.90
PAD_HALF_Z, HEAD_HALF_Z = 0.25, 0.15
RAIL_Z0, RAIL_Z1, RAIL_Y0, RAIL_Y1 = 0.27, 0.28, 0.7875, 0.8125
HEAD_X = (-1.0, -0.735)
BACK_X = (-0.70, -0.225)
SEAT_X = (-0.215, 0.255)
LEG_X = (0.265, 1.0)
LEG_Z = (0.008, 0.25)
HINGE_X = (-0.22, 0.26)
STAND_X = 1.17
STAND_LEG_X, STAND_HALF_Z = 0.25, 0.24
STAND_LEG_Y = (0.085, 0.115)
STAND_CASTOR_X, STAND_CASTOR_Z, STAND_CASTOR_R = 0.22, 0.195, 0.0375
BOX_Y1 = 0.17
COLUMN_R, COLUMN_Y1 = 0.025, 1.85
ARM_Y, ARM_LEN = 1.92, 0.8
STAND_TOP = 1.96
HEAD_CX, HEAD_Y, HEAD_R = STAND_X - ARM_LEN, 1.69, 0.25
LENS_Y0, LENS_R = 1.66, 0.236
YOKE_Y, YOKE_X1, YOKE_R, YOKE_BEND = 1.85, HEAD_CX + 0.285, 0.0125, 0.06

FRAMES = {
    "base": (0.0, 0.0, 0),
    "column": (0.0, PLINTH_Y1, 90),
    "top": (0.0, PIVOT_Y, 0),
    "stand-base": (STAND_X, 0.0, 0),
    "stand-column": (STAND_X, BOX_Y1, 90),
    "stand-arm": (STAND_X, ARM_Y, 180),
    "light-head": (HEAD_CX, HEAD_Y, 0),
}

PAD, STAINLESS, STEEL, RUBBER, ALU, LENS = "pad-foam", "stainless-steel", "powder-coated-steel", "rubber-castor", "light-aluminium", "light-lens-glass"


def r6(value):
    """🔢 Rounds to a hundredth of a millimetre."""
    return round(value + 0.0, 5)


def local(bone, x, y):
    """🧭 A world point of the elevation plane in the frame of a bone."""
    ox, oy, turn = FRAMES[bone]
    if turn == 90:
        return (r6(y - oy), r6(ox - x))
    return (r6(ox - x), r6(oy - y)) if turn == 180 else (r6(x - ox), r6(y - oy))


def direction(bone, vector):
    """🧲 A world direction in the frame of a bone."""
    turn = FRAMES[bone][2]
    if turn == 90:
        return [vector[1], -vector[0], vector[2]]
    return [-vector[0], -vector[1], vector[2]] if turn == 180 else list(vector)


def rrect(a, b, radius, steps=4):
    """⬜ A rounded rectangle a = (a0, a1) by b = (b0, b1), counter-clockwise."""
    corners = [(a[0], b[0], 180), (a[1], b[0], 270), (a[1], b[1], 0), (a[0], b[1], 90)]
    points = []
    for ca, cb, start in corners:
        if radius <= 0:
            points.append((ca, cb))
            continue
        centre_a = ca + radius if ca == a[0] else ca - radius
        centre_b = cb + radius if cb == b[0] else cb - radius
        for k in range(steps + 1):
            angle = math.radians(start + 90 * k / steps)
            points.append((centre_a + radius * math.cos(angle), centre_b + radius * math.sin(angle)))
    return points


def arc(cx, cy, radius, start, end, steps):
    """🌙 Points of an arc in degrees."""
    return [(cx + radius * math.cos(math.radians(start + (end - start) * k / steps)), cy + radius * math.sin(math.radians(start + (end - start) * k / steps))) for k in range(steps + 1)]


def part(sid, bone, material, region, tone, solid, e, f, p, depth, zr, role="body", views="efp", **more):
    """🧩 A part: its solid, its outlines in elevation (x, y), front (z, y) and plan (x, z), the nearest coordinate per view (elevation -z, front +x, plan +y) and its lateral span."""
    solid = dict({"id": sid, "kind": solid.pop("kind"), "bone": bone, "material": material, "role": role, "tone": tone}, **solid)
    solid.update(more)
    return {"id": sid, "bone": bone, "material": material, "region": region, "tone": tone, "role": role, "solid": solid, "e": e, "f": f, "p": p, "depth": depth, "zr": zr, "views": views}


def box(sid, bone, material, region, tone, x, y, z, radius=0.0, **kw):
    """📦 A box between world extents."""
    cu, cv = local(bone, (x[0] + x[1]) / 2, (y[0] + y[1]) / 2)
    size = [r6(y[1] - y[0]), r6(x[1] - x[0])] if FRAMES[bone][2] == 90 else [r6(x[1] - x[0]), r6(y[1] - y[0])]
    solid = {"kind": "box", "size": size + [r6(z[1] - z[0])], "transform": {"translation": [cu, cv, r6((z[0] + z[1]) / 2)]}}
    if radius:
        solid["cornerRadius"] = radius
    return part(sid, bone, material, region, tone, solid, ("rect", x[0], x[1], y[0], y[1], radius), ("rect", z[0], z[1], y[0], y[1], radius), ("rect", x[0], x[1], z[0], z[1], radius), (-z[0], x[1], y[1]), z, **kw)


def turned_y(sid, bone, material, region, tone, x, z, profile, **kw):
    """🏺 A revolve about the vertical through (x, z); profile of (world height, radius) from the axis to the axis."""
    y0 = min(a for a, _ in profile)
    ou, ov = local(bone, x, y0)
    solid = {"kind": "revolve", "profile": [[r6(a - y0), r6(r)] for a, r in profile], "axis": direction(bone, (0, 1, 0)), "origin": [ou, ov, r6(z)]}
    rmax, y1 = max(r for _, r in profile), max(a for a, _ in profile)
    off = [(a, r) for a, r in profile if r > 0]
    side = lambda c: ("poly", [(c + r, a) for a, r in off] + [(c - r, a) for a, r in reversed(off)])
    return part(sid, bone, material, region, tone, solid, side(x), side(z), ("ellipse", x, z, rmax, rmax), (-(z - rmax), x + rmax, y1), (z - rmax, z + rmax), **kw)


def cyl_y(sid, bone, material, region, tone, x, z, y, radius, **kw):
    """🛢 A vertical cylinder."""
    return turned_y(sid, bone, material, region, tone, x, z, [(y[0], 0), (y[0], radius), (y[1], radius), (y[1], 0)], **kw)


def turned_z(sid, bone, material, region, tone, x, y, z, profile, **kw):
    """🛞 A revolve about a lateral axle through (x, y) starting at z; profile of (offset along z, radius)."""
    ou, ov = local(bone, x, y)
    solid = {"kind": "revolve", "profile": [[r6(a), r6(r)] for a, r in profile], "axis": [0, 0, 1], "origin": [ou, ov, r6(z)]}
    rmax = max(r for _, r in profile)
    z0, z1 = z + min(a for a, _ in profile), z + max(a for a, _ in profile)
    return part(sid, bone, material, region, tone, solid, ("ellipse", x, y, rmax, rmax), ("rect", z0, z1, y - rmax, y + rmax, 0), ("rect", x - rmax, x + rmax, z0, z1, 0), (-z0, x + rmax, y + rmax), (z0, z1), **kw)


def cyl_z(sid, bone, material, region, tone, x, y, z, radius, **kw):
    """🥫 A lateral cylinder from z[0] to z[1]."""
    return turned_z(sid, bone, material, region, tone, x, y, z[0], [(0, 0), (0, radius), (z[1] - z[0], radius), (z[1] - z[0], 0)], **kw)


def wheel(sid, bone, region, x, y, z, radius, width, bevel=0.005, **kw):
    """⚫ A castor wheel with a rounded tyre, centred on z."""
    h = width / 2
    return turned_z(sid, bone, RUBBER, region, 0.2, x, y, z, [(-h, 0), (-h, radius - bevel), (-h + bevel, radius), (h - bevel, radius), (h, radius - bevel), (h, 0)], **kw)


def cyl_x(sid, bone, material, region, tone, x, y, z, radius, **kw):
    """➖ A cylinder along the table from x[0] to x[1]."""
    a, b = local(bone, x[0], y), local(bone, x[1], y)
    start, sign = (a, 1) if a[0] < b[0] else (b, 1)
    solid = {"kind": "revolve", "profile": [[0, 0], [0, r6(radius)], [r6(x[1] - x[0]), r6(radius)], [r6(x[1] - x[0]), 0]], "axis": [sign, 0, 0], "origin": [start[0], start[1], r6(z)]}
    return part(sid, bone, material, region, tone, solid, ("rect", x[0], x[1], y - radius, y + radius, 0), ("ellipse", z, y, radius, radius), ("rect", x[0], x[1], z - radius, z + radius, 0), (-(z - radius), x[1], y + radius), (z - radius, z + radius), **kw)


def prism_z(sid, bone, material, region, tone, profile, z, **kw):
    """↔ A world (x, y) profile pushed across the table from z[0] to z[1]."""
    solid = {"kind": "extrude", "profile": [list(local(bone, x, y)) for x, y in profile], "plane": "xy", "from": r6(z[0]), "to": r6(z[1])}
    xs, ys = [x for x, _ in profile], [y for _, y in profile]
    return part(sid, bone, material, region, tone, solid, ("poly", list(profile)), ("rect", z[0], z[1], min(ys), max(ys), 0), ("rect", min(xs), max(xs), z[0], z[1], 0), (-z[0], max(xs), max(ys)), z, **kw)


def prism_x(sid, bone, material, region, tone, profile, x, **kw):
    """➡ A world (y, z) profile pushed along the table from x[0] to x[1]; bones without a turn only."""
    ox, oy, _ = FRAMES[bone]
    solid = {"kind": "extrude", "profile": [[r6(y - oy), r6(z)] for y, z in profile], "plane": "yz", "from": r6(x[0] - ox), "to": r6(x[1] - ox)}
    ys, zs = [y for y, _ in profile], [z for _, z in profile]
    return part(sid, bone, material, region, tone, solid, ("rect", x[0], x[1], min(ys), max(ys), 0), ("poly", [(z, y) for y, z in profile]), ("rect", x[0], x[1], min(zs), max(zs), 0), (-min(zs), x[1], max(ys)), (min(zs), max(zs)), **kw)


def prism_y(sid, bone, material, region, tone, profile, y, holes=None, **kw):
    """⬆ A world (z, x) profile standing from y[0] to y[1]; on an upright bone it is pushed along the bone."""
    ox, oy, turn = FRAMES[bone]
    ring = (lambda points: [[r6(ox - x), r6(z)] for z, x in points]) if turn == 90 else (lambda points: [[r6(z), r6(x - ox)] for z, x in points])
    solid = {"kind": "extrude", "profile": ring(profile), "plane": "yz" if turn == 90 else "zx", "from": r6(y[0] - oy), "to": r6(y[1] - oy)}
    if holes:
        solid["holes"] = [ring(hole) for hole in holes]
    zs, xs = [z for z, _ in profile], [x for _, x in profile]
    return part(sid, bone, material, region, tone, solid, ("rect", min(xs), max(xs), y[0], y[1], 0), ("rect", min(zs), max(zs), y[0], y[1], 0), ("poly", [(x, z) for z, x in profile]), (-min(zs), max(xs), y[1]), (min(zs), max(zs)), **kw)


def tube(sid, bone, material, region, tone, a, b, radius, **kw):
    """🧵 A round bar with spherical ends between two world points (x, y, z)."""
    pa, pb = local(bone, a[0], a[1]), local(bone, b[0], b[1])
    solid = {"kind": "capsule", "p0": [pa[0], pa[1], r6(a[2])], "p1": [pb[0], pb[1], r6(b[2])], "r0": radius, "r1": radius}
    zmin, zmax = min(a[2], b[2]) - radius, max(a[2], b[2]) + radius
    return part(sid, bone, material, region, tone, solid, ("capsule", a[0], a[1], b[0], b[1], radius), ("capsule", a[2], a[1], b[2], b[1], radius), ("capsule", a[0], a[2], b[0], b[2], radius), (-zmin, max(a[0], b[0]) + radius, max(a[1], b[1]) + radius), (zmin, zmax), **kw)


def loft_x(sid, bone, material, region, tone, sections, outline_radius=0.0, **kw):
    """🎢 A loft along the table through world sections (x, y centre, z centre, half width, half height, exponent)."""
    solid = {"kind": "loft", "sections": [], "capStart": True, "capEnd": True}
    for x, y, z, hw, hh, exponent in sections:
        u, v = local(bone, x, y)
        solid["sections"].append({"at": [u, v, r6(z)], "rx": r6(hw), "ry": r6(hh), "exponent": exponent})
    if FRAMES[bone][2] == 180:
        solid["sections"].reverse()
    xs = [s[0] for s in sections]
    y0, y1 = min(s[1] - s[4] for s in sections), max(s[1] + s[4] for s in sections)
    z0, z1 = min(s[2] - s[3] for s in sections), max(s[2] + s[3] for s in sections)
    return part(sid, bone, material, region, tone, solid, ("rect", min(xs), max(xs), y0, y1, outline_radius), ("rect", z0, z1, y0, y1, outline_radius), ("rect", min(xs), max(xs), z0, z1, outline_radius), (-z0, max(xs), y1), (z0, z1), **kw)


def loft_y(sid, bone, material, region, tone, x, z, sections, **kw):
    """🗼 A loft standing on (x, z) through sections (y, half x, half z, exponent); bones without a turn only."""
    ox, oy, _ = FRAMES[bone]
    quarter = [0.5, 0.5, 0.5, -0.5]
    solid = {"kind": "loft", "sections": [{"at": [r6(x - ox), r6(y - oy), r6(z)], "rx": r6(hz), "ry": r6(hx), "exponent": e, "rotation": quarter} for y, hx, hz, e in sections], "capStart": True, "capEnd": True}
    left = [(x - hx, y) for y, hx, _, _ in sections]
    right = [(x + hx, y) for y, hx, _, _ in sections]
    front_l = [(z - hz, y) for y, _, hz, _ in sections]
    front_r = [(z + hz, y) for y, _, hz, _ in sections]
    hx, hz = max(s[1] for s in sections), max(s[2] for s in sections)
    return part(sid, bone, material, region, tone, solid, ("poly", left + right[::-1]), ("poly", front_l + front_r[::-1]), ("rect", x - hx, x + hx, z - hz, z + hz, min(hx, hz) * 0.25), (-(z - hz), x + hx, sections[-1][0]), (z - hz, z + hz), **kw)


def mirrored(maker, sid, *args, **kw):
    """🪞 The right (-z, near in the elevation) and left (+z) twin of a part whose maker takes `side` (+1 left, -1 right)."""
    near = maker(sid + "-r", -1, *args, **kw)
    far = maker(sid + "-l", 1, *args, **kw)
    far["views"] = far["views"].replace("e", "")
    return [near, far]


def span(side, z0, z1):
    """📏 A lateral span on one side of the table."""
    return (z0, z1) if side > 0 else (-z1, -z0)
