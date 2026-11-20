"""🛋 Round 3, furniture/three-seat-sofa: the measures of the real sofa and the helpers that turn world coordinates into bone-local solids and 2D outlines.

The subject is an upholstered three-seat sofa, 2.00 x 0.90 x 0.85 m. It stands on four turned beech feet. The frame is a plywood plinth, the base, with a 20 mm ply deck. Two box arms are built on timber posts and rails. A leaning back frame carries the outer back. There are three loose seat cushions (HR foam 0.20 m) and three loose back cushions (0.15 to 0.17 m).

World frame: x runs from the left arm (x 0) to the right arm (x 2.0), y is up, and z is lateral. The front of the sofa faces -z and the back is at +z. The product's elevation looks from -z, the front view from +x and the plan from +y.

Every curved part is a y-loft: superelliptic plan sections stacked from the bottom up. Every station is stated in the tables below, so the solid, its outline and its section all come from the same numbers.
"""
import math

WIDTH, DEPTH, HEIGHT = 2.0, 0.9, 0.85
ARM_W = 0.2
FOOT_X, FOOT_Z, FOOT_H = (0.12, 1.88), 0.36, 0.10
BASE_X, BASE_Y, BASE_Z = (0.2, 1.8), (0.10, 0.23), (-0.44, 0.45)
DECK_Y = 0.23
SEAT_X = (0.2, 0.7333, 1.2667, 1.8)
SEAT_Z, SEAT_TOP, SEAT_CROWN = (-0.43, 0.14), 0.43, 0.44
LEAN = 0.1405
BACK_Z0 = 0.29
FRAME_TOP = 0.85
ARM_Y = (0.10, 0.62)
ARM_Z = (-0.45, 0.45)

FRAME_Y = 0.165
ORIGIN = {"ground": (0.0, 0.0, 0.0), "frame": (0.0, FRAME_Y, 0.0), "arm-left": (0.1, ARM_Y[0], 90.0), "arm-right": (1.9, ARM_Y[0], 90.0)}
for _k in range(3):
    ORIGIN[f"seat-{_k + 1}"] = (SEAT_X[_k], DECK_Y, 0.0)
    ORIGIN[f"back-{_k + 1}"] = (SEAT_X[_k], DECK_Y, 0.0)

FABRIC, FOAM, PLY, BEECH, BATT = "upholstery-fabric", "hr-foam", "birch-plywood", "beech-timber", "polyester-batting"
Q_FLAT = [0.707107, 0.0, 0.0, 0.707107]
Q_ARM = [0.0, 0.707107, 0.0, 0.707107]


def r6(value):
    """🔢 Rounds to a hundredth of a millimetre so the written numbers stay readable."""
    return round(value + 0.0, 5)


def local(bone, x, y, z=0.0):
    """📐 A world point in the rest frame of a bone (u along the bone, v its normal, w lateral)."""
    ox, oy, ang = ORIGIN[bone]
    dx, dy = x - ox, y - oy
    if ang == 90.0:
        return (r6(dy), r6(-dx), r6(z))
    return (r6(dx), r6(dy), r6(z))


def base(sid, kind, bone, material, tone, role="body"):
    """🏷 The common members of a solid."""
    return {"id": sid, "kind": kind, "bone": bone, "material": material, "role": role, "tone": tone}


def box(sid, bone, material, tone, x, y, z, radius=0.0, role="body", **more):
    """📦 A box between world extents x, y, z (each a pair); on a turned bone the sizes follow the bone axes."""
    out = base(sid, "box", bone, material, tone, role)
    sx, sy, sz = x[1] - x[0], y[1] - y[0], z[1] - z[0]
    out["size"] = [r6(sy), r6(sx), r6(sz)] if ORIGIN[bone][2] == 90.0 else [r6(sx), r6(sy), r6(sz)]
    if radius:
        out["cornerRadius"] = radius
    out["transform"] = {"translation": list(local(bone, (x[0] + x[1]) / 2, (y[0] + y[1]) / 2, (z[0] + z[1]) / 2))}
    out.update(more)
    return out


def yloft(sid, bone, material, tone, cx, stations, exponent, role="body", **more):
    """🧁 A y-loft: superelliptic plan sections (y, half width in x, centre z, half depth in z), bottom to top, about the world plan centre x = cx."""
    out = base(sid, "loft", bone, material, tone, role)
    rot = Q_ARM if ORIGIN[bone][2] == 90.0 else Q_FLAT
    secs = []
    for y, hx, zc, hz in stations:
        at = local(bone, cx, y, zc)
        if rot is Q_ARM:
            secs.append({"at": list(at), "rx": r6(hz), "ry": r6(hx), "exponent": exponent, "rotation": rot})
        else:
            secs.append({"at": list(at), "rx": r6(hx), "ry": r6(hz), "exponent": exponent, "rotation": rot})
    out["sections"] = secs
    out["capStart"] = True
    out["capEnd"] = True
    out.update(more)
    return out


def along_x(sid, bone, material, tone, profile_yz, x, holes=None, role="body", **more):
    """➡ A prism along the sofa: a world (y, z) profile pushed from x[0] to x[1], optionally with holes."""
    ox, oy, _ = ORIGIN[bone]
    out = base(sid, "extrude", bone, material, tone, role)
    out["profile"] = [[r6(y - oy), r6(z)] for y, z in profile_yz]
    if holes:
        out["holes"] = [[[r6(y - oy), r6(z)] for y, z in h] for h in holes]
    out.update({"plane": "yz", "from": r6(x[0] - ox), "to": r6(x[1] - ox)})
    out.update(more)
    return out


def ring_plan(sid, bone, material, tone, cx, zc, outer, inner, y, exponent, role="detail", **more):
    """⭕ A plan ring (piping bead): a superelliptic outline (half x, half z) with a superelliptic hole, pushed up from y[0] to y[1]."""
    ox, oy, _ = ORIGIN[bone]
    out = base(sid, "extrude", bone, material, tone, role)
    out["profile"] = [[r6(z), r6(x)] for x, z in superellipse(outer[0], outer[1], exponent, 48)]
    out["holes"] = [list(reversed([[r6(z), r6(x)] for x, z in superellipse(inner[0], inner[1], exponent, 48)]))]
    out.update({"plane": "zx", "from": r6(y[0] - oy), "to": r6(y[1] - oy)})
    out["transform"] = {"translation": [r6(cx - ox), 0.0, r6(zc)]}
    out.update(more)
    return out


def turned(sid, bone, material, tone, profile, origin, role="body", **more):
    """🪵 A revolve of an (axial, radial) profile about the world vertical through a world origin (x, y, z)."""
    out = base(sid, "revolve", bone, material, tone, role)
    out["profile"] = [[r6(a), r6(r)] for a, r in profile]
    out["axis"] = [1, 0, 0] if ORIGIN[bone][2] == 90.0 else [0, 1, 0]
    out["origin"] = list(local(bone, *origin))
    out.update(more)
    return out


def ellipsoid(sid, bone, material, tone, centre, radii, role="detail", **more):
    """🥚 An ellipsoid at a world centre with world radii (x, y, z)."""
    out = base(sid, "ellipsoid", bone, material, tone, role)
    rx, ry, rz = radii
    out["radii"] = [r6(ry), r6(rx), r6(rz)] if ORIGIN[bone][2] == 90.0 else [r6(rx), r6(ry), r6(rz)]
    out["transform"] = {"translation": list(local(bone, *centre))}
    out.update(more)
    return out


def repeat(solid, count, du=0.0, dv=0.0, dw=0.0):
    """🔁 Copies a solid `count` times along a bone-local step."""
    solid["repeat"] = {"count": count, "translation": [r6(du), r6(dv), r6(dw)]}
    return solid


def superellipse(a, b, n, count=64):
    """🥏 Points of the superellipse |x/a|^n + |y/b|^n = 1, counter-clockwise from +x."""
    pts = []
    for k in range(count):
        t = 2 * math.pi * k / count
        c, s = math.cos(t), math.sin(t)
        pts.append((a * math.copysign(abs(c) ** (2 / n), c), b * math.copysign(abs(s) ** (2 / n), s)))
    return pts


def half_at(a, b, n, x):
    """📏 The half extent along b of a superellipse at the offset x along a."""
    if abs(x) >= a:
        return 0.0
    return b * (1 - abs(x / a) ** n) ** (1 / n)


def edge(y0, y1, bottom, top):
    """🧵 Stations (y, inset) of a block between y0 and y1 whose bottom and top edges are rounded by elliptic quarters: bottom = (horizontal radius, vertical radius, steps), top likewise; steps 0 means a square edge."""
    out = []
    hb, vb, nb = bottom
    if nb:
        for k in range(nb + 1):
            a = math.pi / 2 * k / nb
            out.append((y0 + vb - vb * math.cos(a), hb - hb * math.sin(a)))
    else:
        out.append((y0, 0.0))
    ht, vt, nt = top
    if nt:
        for k in range(nt + 1):
            a = math.pi / 2 * k / nt
            out.append((y1 - vt + vt * math.sin(a), ht - ht * math.cos(a)))
    else:
        out.append((y1, 0.0))
    return out


def seat_stations(scale):
    """🟫 The stations of a seat cushion (y, half x, centre z, half z) per scale: 0.5333 x 0.57 m, 0.20 m on the deck with a 10 mm crown."""
    hx, zc, hz = (SEAT_X[1] - SEAT_X[0]) / 2, (SEAT_Z[0] + SEAT_Z[1]) / 2, (SEAT_Z[1] - SEAT_Z[0]) / 2
    spec = {"50": ((0.015, 0.012, 3), (0.055, 0.04, 4)), "100": ((0.015, 0.012, 1), (0.05, 0.035, 2)), "200": ((0.0, 0.0, 0), (0.04, 0.03, 1))}[scale]
    return [(y, hx - d, zc, hz - d) for y, d in edge(DECK_Y, SEAT_CROWN, *spec)]


def arm_stations(scale):
    """🟪 The stations of an arm (y, half x, centre z, half z) per scale: 0.20 x 0.90 m from 0.10 to 0.62 m, top edges round 60 mm, bottom 15 mm."""
    hx, zc, hz = ARM_W / 2, 0.0, (ARM_Z[1] - ARM_Z[0]) / 2
    spec = {"50": ((0.015, 0.015, 3), (0.06, 0.06, 4)), "100": ((0.015, 0.015, 1), (0.06, 0.06, 2)), "200": ((0.0, 0.0, 0), (0.06, 0.06, 1))}[scale]
    return [(y, hx - d, zc, hz - d) for y, d in edge(ARM_Y[0], ARM_Y[1], *spec)]


def frame_face(y):
    """📐 The z of the leaning inner face of the back frame at height y (8 degrees back from the vertical)."""
    return BACK_Z0 + (y - DECK_Y) * LEAN


def backframe_stations(scale):
    """🟧 The stations of the back frame (y, half x, centre z, half z): rear face vertical at z 0.45, inner face leaning 8 degrees from z 0.29, top rolled over a radius of 40 mm to 0.85 m."""
    hx = (BASE_X[1] - BASE_X[0]) / 2
    r = 0.04
    yc = FRAME_TOP - r
    out = [(DECK_Y, hx, (frame_face(DECK_Y) + 0.45) / 2, (0.45 - frame_face(DECK_Y)) / 2)]
    if scale == "50":
        out.append((0.52, hx, (frame_face(0.52) + 0.45) / 2, (0.45 - frame_face(0.52)) / 2))
    steps = {"50": 4, "100": 2, "200": 1}[scale]
    for k in range(steps + 1):
        a = math.pi / 2 * k / steps
        out.append((yc + r * math.sin(a), hx - r * (1 - math.cos(a)), 0.45 - r, r * math.cos(a) if k < steps else r * 0.25))
    return out


BACK_ROWS = {
    "50": [(0.23, 0.150, 0.012), (0.24, 0.147, 0.004), (0.26, 0.146, 0.0), (0.35, 0.145, 0.0), (0.45, 0.150, 0.0), (0.55, 0.163, 0.0), (0.63, 0.182, 0.0), (0.69, 0.203, 0.0), (0.735, 0.224, 0.006), (0.765, 0.245, 0.018), (0.785, 0.268, 0.035), (0.797, 0.293, 0.06), (0.80, 0.318, 0.09)],
    "100": [(0.23, 0.150, 0.01), (0.25, 0.146, 0.0), (0.45, 0.150, 0.0), (0.60, 0.172, 0.0), (0.72, 0.215, 0.008), (0.775, 0.255, 0.03), (0.80, 0.31, 0.08)],
    "200": [(0.23, 0.148, 0.0), (0.50, 0.152, 0.0), (0.74, 0.225, 0.0), (0.80, 0.30, 0.06)],
}


def back_stations(scale):
    """🟨 The stations of a back cushion (y, half x, centre z, half z): front profile by hand (bottom 0.15 m thick, lumbar belly, rounded pillow top), rear on the leaning frame face."""
    hx = (SEAT_X[1] - SEAT_X[0]) / 2
    out = []
    for y, zf, d in BACK_ROWS[scale]:
        zr = frame_face(y)
        out.append((y, hx - d, (zf + zr) / 2, (zr - zf) / 2))
    return out


def base_profile(scale, inset=0.0):
    """🟫 The (y, z) outline of the plinth: 0.10 to 0.23 m, z -0.44 to 0.45, the front top edge round 20 mm (1:50, 1:100), others eased 6 mm; `inset` offsets it inwards (a nested layer shell)."""
    y0, y1 = BASE_Y[0] + inset, BASE_Y[1] - inset
    z0, z1 = BASE_Z[0] + inset, BASE_Z[1] - inset
    if scale == "500":
        return [(y0, z0), (y0, z1), (y1, z1), (y1, z0)]
    rf, re = (0.02 - inset, 0.006 - inset) if scale in ("50", "100") else (0.03, 0.0)
    pts = []
    def arc(cy, cz, r, a0, a1, n):
        for k in range(n + 1):
            a = a0 + (a1 - a0) * k / n
            pts.append((cy + r * math.sin(a), cz + r * math.cos(a)))
    if re:
        arc(y0 + re, z0 + re, re, math.pi, 3 * math.pi / 2, 2)
        arc(y0 + re, z1 - re, re, 3 * math.pi / 2, 2 * math.pi, 2)
        arc(y1 - re, z1 - re, re, 0, math.pi / 2, 2)
    else:
        pts += [(y0, z0), (y0, z1), (y1, z1)]
    arc(y1 - rf, z0 + rf, rf, math.pi / 2, math.pi, 4 if scale == "50" else 2)
    return pts


FOOT = {
    "50": [(0.0, 0.0), (0.0, 0.021), (0.003, 0.0225), (0.068, 0.0275), (0.0695, 0.0305), (0.0785, 0.0305), (0.08, 0.0285), (0.10, 0.0295), (0.10, 0.0)],
    "100": [(0.0, 0.0), (0.0, 0.0225), (0.07, 0.028), (0.10, 0.03), (0.10, 0.0)],
    "200": [(0.0, 0.0), (0.0, 0.025), (0.10, 0.03), (0.10, 0.0)],
}


def lofted_outline(stations, cx, axis):
    """✏ The silhouette polygon of a y-loft: `axis` "x" gives the elevation (x, y), "z" gives the side outline (z, y)."""
    left, right = [], []
    for y, hx, zc, hz in stations:
        if axis == "x":
            left.append((cx - hx, y))
            right.append((cx + hx, y))
        else:
            left.append((zc - hz, y))
            right.append((zc + hz, y))
    return left + list(reversed(right))


def lofted_plan(stations, cx, exponent, samples=40):
    """🗺 The plan polygon (x, z) of a y-loft: for every x the front-most and rear-most reach over all stations."""
    hmax = max(s[1] for s in stations)
    xs = [hmax * math.sin(math.pi / 2 * (2 * k / samples - 1)) for k in range(samples + 1)]
    front, rear = [], []
    for dx in xs:
        lo, hi = None, None
        for y, hx, zc, hz in stations:
            h = half_at(hx, hz, exponent, dx) if abs(dx) < hx else (0.0 if abs(dx) <= hx + 1e-9 else None)
            if h is None:
                continue
            lo = zc - h if lo is None else min(lo, zc - h)
            hi = zc + h if hi is None else max(hi, zc + h)
        if lo is not None:
            front.append((cx + dx, lo))
            rear.append((cx + dx, hi))
    return front + list(reversed(rear))


def masses():
    """⚖ The masses of the bones as the asset declares them (authored estimates, kept) with the centres of the modelled parts in world x, y."""
    return {
        "frame": (26.0, 1.0, 0.33),
        "arm": (3.6, None, 0.35),
        "seat": (3.6, None, DECK_Y + 0.105),
        "back": (2.6, None, 0.51),
    }
