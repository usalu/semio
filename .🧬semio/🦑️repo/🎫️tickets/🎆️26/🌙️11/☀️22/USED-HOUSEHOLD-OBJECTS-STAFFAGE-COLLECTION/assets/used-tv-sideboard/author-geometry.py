"""📐 Hand-chosen numbers and the part helpers of the used TV sideboard: world metres (x ahead = the front of the lowboard, y up, z left), bone frames, solids and 2D shapes.

Imported by `author-used-tv-sideboard.py`, which writes the asset document.

@see ../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/README.md — Conventions, Front and plan views, Volume
"""
import math

T = 0.016
TB = 0.003
X0, XB, XF, XD = 0.0, 0.003, 0.384, 0.400
YF, YB1, YT0, YT1 = 0.025, 0.041, 0.434, 0.450
ZS, ZSI = 0.900, 0.884
ZDO, ZDI = 0.3053, 0.2893
YM0, YM1 = 0.185, 0.201
DOOR_Y0, DOOR_Y1 = 0.041, 0.4325
DOOR_Z0, DOOR_Z1 = 0.299, 0.897
DOOR_ZC = 0.598
DRW_Y0, DRW_Y1, DRW_Z = 0.0415, 0.1835, 0.2975
DRW_X0 = 0.030
BOX_Y0, BOX_Y1, BOX_ZO, BOX_ZI, BOX_T = 0.055, 0.165, 0.2768, 0.2648, 0.012
RUN_X0, RUN_Y0, RUN_Y1 = 0.034, 0.055, 0.075
FOOT_R, FOOT_H, FOOT_XF, FOOT_XB, FOOT_Z = 0.020, 0.025, 0.344, 0.040, 0.875
HOLE_R, HOLE_Y = 0.030, 0.3175
HANDLE_Y_DOOR, HANDLE_Y_DRAWER = 0.395, 0.1125
BAR_R, BAR_HALF, LEG_HALF, LEG_C, BAR_X = 0.006, 0.074, 0.005, 0.064, 0.422
KNOB_Y = 0.1125
TAPE = 0.0001
TAPE_L, TAPE_R, TAPE_D = (0.78, 0.83), (-0.83, -0.78), (0.15, 0.20)
LABEL = ((0.31, 0.37), (0.42, 0.48))
GRIP = (0.192, 0.85)

FRAMES = {
    "ground": {"o": (0.0, 0.0, 0.0), "kind": "identity"},
    "carcass": {"o": (0.0, 0.0, 0.0), "kind": "identity"},
    "drawer": {"o": (DRW_X0, DRW_Y0, 0.0), "kind": "identity"},
    "flap-l": {"o": (XF, YB1, DOOR_ZC), "kind": "flap"},
    "flap-r": {"o": (XF, YB1, -DOOR_ZC), "kind": "flap"},
}


def r6(v):
    """🎯 Rounds a number to micrometres, recursively through lists and tuples."""
    if isinstance(v, (list, tuple)):
        return [r6(x) for x in v]
    return round(v, 6) + 0.0


def local_point(bone, p):
    """🧭 World rest point to the (u, v, w) frame of a bone."""
    f = FRAMES[bone]
    ox, oy, oz = f["o"]
    x, y, z = p
    if f["kind"] == "identity":
        return (x - ox, y - oy, z - oz)
    return (y - oy, ox - x, z - oz)


def local_ranges(bone, X, Y, Z):
    """📦 World axis ranges of a box to sorted (u, v, w) ranges of a bone frame."""
    a = local_point(bone, (X[0], Y[0], Z[0]))
    b = local_point(bone, (X[1], Y[1], Z[1]))
    return [tuple(sorted((a[i], b[i]))) for i in range(3)]


def local_2d(bone, x, y):
    """✏️ World rest (x, y) to the elevation drawing frame (u, v) of a bone."""
    u, v, _ = local_point(bone, (x, y, FRAMES[bone]["o"][2]))
    return (u, v)


def solid(sid, bone, material, role="body", tone=0.8, **extra):
    """🧱 The common fields of a volume solid."""
    out = {"id": sid, "bone": bone, "material": material, "role": role, "tone": tone}
    out.update(extra)
    return out


def box(sid, bone, material, X, Y, Z, role="body", tone=0.8, **extra):
    """⬜ An axis-aligned box given by world rest ranges, written in the bone frame."""
    rng = local_ranges(bone, X, Y, Z)
    size = [b - a for a, b in rng]
    centre = [(a + b) / 2 for a, b in rng]
    return solid(sid, bone, material, role, tone, kind="box", size=r6(size), transform={"translation": r6(centre)}, **extra)


def foot_profile(simple):
    """🦶 The turned [axial, radial] profile of a plastic glide foot, axial up from the floor."""
    if simple:
        return [[0.0, 0.0], [0.0, FOOT_R], [FOOT_H, FOOT_R], [FOOT_H, 0.0]]
    return [[0.0, 0.0], [0.0, 0.016], [0.004, FOOT_R], [0.021, FOOT_R], [FOOT_H, 0.017], [FOOT_H, 0.0]]


def feet_row(sid, x, simple, tone=0.35):
    """🦶 Three glide feet in one row (left end, middle, right end) as one revolve with a repeat."""
    return solid(sid, "carcass", "pp-feet", "body", tone, kind="revolve", profile=foot_profile(simple), axis=[0, 1, 0], origin=r6([x, 0.0, FOOT_Z]), repeat={"count": 3, "translation": [0, 0, -FOOT_Z]})


def circle(cx, cy, r, n=24):
    """⭕ A polygon approximating a circle, counter-clockwise."""
    return [r6([cx + r * math.cos(2 * math.pi * k / n), cy + r * math.sin(2 * math.pi * k / n)]) for k in range(n)]


def back_panel(sid, hole, Y=(YF, YT0), tone=0.9):
    """🟫 The 3 mm HDF back nailed onto the rear edges, with the round cable hole of the open niche (profile in the (v, w) = (y, z) plane, pushed along x)."""
    profile = [[Y[0], -ZS], [Y[1], -ZS], [Y[1], ZS], [Y[0], ZS]]
    extra = {"holes": [circle(HOLE_Y, 0.0, HOLE_R)]} if hole else {}
    return solid(sid, "carcass", "hdf-back", "body", tone, kind="extrude", profile=r6(profile), plane="yz", **{"from": X0, "to": XB}, sheet=True, **extra)


def convex_polyhedron(points):
    """💎 The outward-oriented triangles of the convex hull of a small point set (coplanar faces fanned around their centroid)."""
    n = len(points)
    faces = {}
    for i in range(n):
        for j in range(i + 1, n):
            for k in range(j + 1, n):
                a, b, c = points[i], points[j], points[k]
                ab = [b[t] - a[t] for t in range(3)]
                ac = [c[t] - a[t] for t in range(3)]
                nrm = [ab[1] * ac[2] - ab[2] * ac[1], ab[2] * ac[0] - ab[0] * ac[2], ab[0] * ac[1] - ab[1] * ac[0]]
                ln = math.sqrt(sum(x * x for x in nrm))
                if ln < 1e-12:
                    continue
                nrm = [x / ln for x in nrm]
                d = sum(nrm[t] * a[t] for t in range(3))
                side = [sum(nrm[t] * p[t] for t in range(3)) - d for p in points]
                if all(s <= 1e-9 for s in side) or all(s >= -1e-9 for s in side):
                    if all(s >= -1e-9 for s in side):
                        nrm, d = [-x for x in nrm], -d
                    key = tuple(round(x, 6) for x in nrm) + (round(d, 6),)
                    members = [m for m in range(n) if abs(sum(nrm[t] * points[m][t] for t in range(3)) - d) < 1e-9]
                    faces[key] = (nrm, members)
    tris = []
    for nrm, members in faces.values():
        cx = [sum(points[m][t] for m in members) / len(members) for t in range(3)]
        ref = [points[members[0]][t] - cx[t] for t in range(3)]
        e2 = [nrm[1] * ref[2] - nrm[2] * ref[1], nrm[2] * ref[0] - nrm[0] * ref[2], nrm[0] * ref[1] - nrm[1] * ref[0]]
        order = sorted(members, key=lambda m: math.atan2(sum((points[m][t] - cx[t]) * e2[t] for t in range(3)), sum((points[m][t] - cx[t]) * ref[t] for t in range(3))))
        for q in range(1, len(order) - 1):
            tris.append([order[0], order[q], order[q + 1]])
    return tris


def chipped_door(sid, bone, tone=0.92):
    """🪵 The right flap door with its foil-and-chipboard chip at the outer top front corner: a convex polyhedron in the flap frame."""
    U1, V0, W0, W1 = DOOR_Y1 - DOOR_Y0, -T, DOOR_Z0 - DOOR_ZC, DOOR_Z1 - DOOR_ZC
    cw, cu, cv = 0.030, 0.012, 0.006
    corners = [(u, v, w) for u in (0.0, U1) for v in (V0, 0.0) for w in (-W1, -W0)]
    cut = (U1, V0, -W1)
    pts = [p for p in corners if p != cut] + [(U1, V0, -W1 + cw), (U1 - cu, V0, -W1), (U1, V0 + cv, -W1)]
    pts = [tuple(r6(list(p))) for p in pts]
    return solid(sid, bone, "chipboard-white-foil", "body", tone, kind="polyhedron", vertices=[list(p) for p in pts], triangles=convex_polyhedron(pts))


def bar_handle(prefix, bone, y, zc, tone=0.5):
    """🔩 A 128 mm bar handle of satin-anodised aluminium: the round bar and its two square legs, written in the frame of its bone."""
    bar_a = local_point(bone, (BAR_X, y, zc - BAR_HALF))
    bar_b = local_point(bone, (BAR_X, y, zc + BAR_HALF))
    bar = solid(f"{prefix}-bar", bone, "aluminium-handle", "detail", tone, kind="capsule", p0=r6(list(bar_a)), p1=r6(list(bar_b)), r0=BAR_R, r1=BAR_R)
    leg = box(f"{prefix}-legs", bone, "aluminium-handle", (XD, BAR_X), (y - LEG_HALF, y + LEG_HALF), (zc - LEG_C - LEG_HALF, zc - LEG_C + LEG_HALF), "detail", tone)
    leg["repeat"] = {"count": 2, "translation": [0, 0, 2 * LEG_C]}
    return [bar, leg]


def knob_profile(simple):
    """🍄 The turned beech knob that replaced the lost bar handle of the drawer: [axial out of the drawer front, radial]."""
    if simple:
        return [[0.0, 0.0], [0.0, 0.015], [0.025, 0.015], [0.025, 0.0]]
    return [[0.0, 0.0], [0.0, 0.009], [0.010, 0.009], [0.013, 0.015], [0.021, 0.015], [0.025, 0.010], [0.025, 0.0]]


def knob(sid, simple, tone=0.55):
    """🍄 The beech knob in the centre of the drawer front as a revolve about the front normal."""
    origin = local_point("drawer", (XD, KNOB_Y, 0.0))
    return solid(sid, "drawer", "beech-knob", "detail", tone, kind="revolve", profile=knob_profile(simple), axis=[1, 0, 0], origin=r6(list(origin)))


def l_extrude(sid, bone, material, pts_world, Z, role="texture", tone=0.75, **extra):
    """📼 A thin bent strip (tape over an edge) as an extrude of a world (x, y) profile along the lateral axis."""
    prof = [list(local_2d(bone, x, y)) for x, y in pts_world]
    oz = FRAMES[bone]["o"][2]
    return solid(sid, bone, material, role, tone, kind="extrude", profile=r6(prof), plane="xy", **{"from": r6(Z[0] - oz), "to": r6(Z[1] - oz)}, **extra)


def shape(sid, bone, role, z, tone, geom, filled=True, **extra):
    """✏️ A 2D shape of one view."""
    out = {"id": sid, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone}
    out.update(geom)
    out.update({k: v for k, v in extra.items() if v is not None})
    return out


def rect(x0, x1, y0, y1):
    """⬛ A rect geometry from two ranges."""
    return {"kind": "rect", "x": r6(min(x0, x1)), "y": r6(min(y0, y1)), "w": r6(abs(x1 - x0)), "h": r6(abs(y1 - y0))}


def poly(points, closed=True):
    """🔷 A poly geometry."""
    return {"kind": "poly", "points": r6(points), "closed": closed}


def ellipse(cx, cy, rx, ry=None):
    """⭕ An ellipse geometry."""
    return {"kind": "ellipse", "cx": r6(cx), "cy": r6(cy), "rx": r6(rx), "ry": r6(ry if ry is not None else rx)}


def flap_profile_rect(bone, X, Y):
    """🚪 A rect in the elevation frame (u, v) of a flap from world ranges."""
    a = local_2d(bone, X[0], Y[0])
    b = local_2d(bone, X[1], Y[1])
    return rect(a[0], b[0], a[1], b[1])


def sweep_w(Z):
    """🌀 The extruded solid placement of a flap shape across the lateral range Z (rest asset coordinates)."""
    return {"kind": "extruded", "sweep": "w", "z0": r6(Z[0]), "z1": r6(Z[1])}


def foot_poly_elev(x, simple):
    """🦶 The side outline of a glide foot centred at forward position x."""
    if simple:
        return poly([[x - FOOT_R, 0], [x + FOOT_R, 0], [x + FOOT_R, FOOT_H], [x - FOOT_R, FOOT_H]])
    return poly([[x - 0.016, 0], [x + 0.016, 0], [x + FOOT_R, 0.004], [x + FOOT_R, 0.021], [x + 0.017, FOOT_H], [x - 0.017, FOOT_H], [x - FOOT_R, 0.021], [x - FOOT_R, 0.004]])


def knob_poly_side(simple, x0=XD, yc=KNOB_Y):
    """🍄 The side outline of the knob (axial along +x from the door face)."""
    prof = knob_profile(simple)
    upper = [[x0 + a, yc + r] for a, r in prof]
    lower = [[x0 + a, yc - r] for a, r in reversed(prof)]
    pts = []
    for p in upper + lower:
        if not pts or abs(pts[-1][0] - p[0]) > 1e-9 or abs(pts[-1][1] - p[1]) > 1e-9:
            pts.append(p)
    return pts


def box_volume(s):
    """📏 The volume of a box solid times its repeat count."""
    sx, sy, sz = s["size"]
    return sx * sy * sz * s.get("repeat", {}).get("count", 1)
