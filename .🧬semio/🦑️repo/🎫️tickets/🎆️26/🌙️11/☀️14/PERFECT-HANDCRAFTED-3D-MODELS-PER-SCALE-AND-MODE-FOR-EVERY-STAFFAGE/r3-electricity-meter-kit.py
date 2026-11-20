"""🧰 Round 3, appliances/electricity-meter: the frames and the few drawing helpers shared by the model, the 2D drawings and the root corrections.

World frame of the meter (metres): X across the wall (the paper x of the face view), Y up, Z normal to the wall (the room is at -Z, the wall plane at Z = +0.059).
Bone `body` runs from the lower fixing screws up to the hanging lug in the plane of the centre of mass: origin (0, Y0, ZB), u = +Y, v = -X, w = Z - ZB.
Bone `disc` is the rotor spindle from its lower bearing up to the upper bearing: origin (0, YD, ZD), u = +Y, v = -X, w = Z - ZD.
"""
import math

Y0 = -0.058
LUG = 0.100
COM = 0.005
YD = -0.030
ZD = -0.004
ZB = 0.015
WALL = 0.059


def r6(value):
    return round(value + 0.0, 6)


def rr(cx, cy, hw, hh, r, n=4):
    """⬜ A rounded rectangle about (cx, cy) with half sizes hw, hh, corner radius r and n chords per corner, counter-clockwise from the lower right corner."""
    points = []
    for (sx, sy, start) in ((1, -1, -90), (1, 1, 0), (-1, 1, 90), (-1, -1, 180)):
        ox, oy = cx + sx * (hw - r), cy + sy * (hh - r)
        for k in range(n + 1):
            a = math.radians(start + 90 * k / n)
            points.append((ox + r * math.cos(a), oy + r * math.sin(a)))
    return points


def circle(cx, cy, r, n=16):
    """⭕ A circle as an n-gon, counter-clockwise."""
    return [(cx + r * math.cos(2 * math.pi * k / n), cy + r * math.sin(2 * math.pi * k / n)) for k in range(n)]


def uv(points, y0=Y0):
    """🧭 World (X, Y) points as bone-local (u, v) of an upright bone with its origin at height y0."""
    return [[r6(y - y0), r6(-x)] for (x, y) in points]


def vw(points):
    """🧭 World (X, Z) points as bone-local (v, w) of the body bone."""
    return [[r6(-x), r6(z - ZB)] for (x, z) in points]


def wu(points, y0=Y0, z0=ZB):
    """🧭 World (Z, Y) points as bone-local (w, u) of an upright bone with its origin at height y0 and depth z0."""
    return [[r6(z - z0), r6(y - y0)] for (z, y) in points]


def solid(ident, kind, role, tone, material="abs-plastic", bone="body", **fields):
    """🧱 One volume solid with the common fields first."""
    return {"id": ident, "kind": kind, "bone": bone, "material": material, "role": role, "tone": tone, **fields}


def box(ident, x0, x1, y0, y1, z0, z1, role, tone, material="abs-plastic", radius=None, **more):
    """📦 A box given by its world extents on the body bone."""
    fields = {"size": [r6(y1 - y0), r6(x1 - x0), r6(z1 - z0)]}
    if radius is not None:
        fields["cornerRadius"] = radius
    fields["transform"] = {"translation": [r6((y0 + y1) / 2 - Y0), r6(-(x0 + x1) / 2), r6((z0 + z1) / 2 - ZB)]}
    return solid(ident, "box", role, tone, material, **fields, **more)


def plate(ident, outline, z0, z1, role, tone, material="abs-plastic", holes=None, taper=None, **more):
    """🪟 A world (X, Y) outline pushed along Z from z0 to z1 on the body bone, with optional holes and taper [scale at z0, scale at z1]."""
    fields = {"profile": uv(outline)}
    if holes:
        fields["holes"] = [uv(hole) for hole in holes]
    fields.update({"plane": "xy", "from": r6(z0 - ZB), "to": r6(z1 - ZB)})
    if taper is not None:
        fields["taperTo"] = taper
    return solid(ident, "extrude", role, tone, material, **fields, **more)


def turned_z(ident, x, y, profile, role, tone, material="abs-plastic", **more):
    """🔩 A profile of [Z, radius] turned about the wall normal through world (x, y) on the body bone."""
    return solid(ident, "revolve", role, tone, material, profile=[[r6(a - ZB), r6(b)] for (a, b) in profile], axis=[0, 0, 1], origin=[r6(y - Y0), r6(-x), 0], **more)


def rect2(ident, x0, y0, w, h, **fields):
    """▭ A 2D rect in plain view coordinates (front and plan)."""
    return {"id": ident, "kind": "rect", "x": r6(x0), "y": r6(y0), "w": r6(w), "h": r6(h), **fields}


def rect_e(ident, x0, y0, w, h, **fields):
    """▯ A world (X, Y) rect of the face view as an elevation rect in the frame of the upright body bone."""
    return {"id": ident, "kind": "rect", "x": r6(y0 - Y0), "y": r6(-(x0 + w)), "w": r6(h), "h": r6(w), **fields}


def poly_e(ident, points, closed=True, y0=Y0, **fields):
    """🔷 World (X, Y) points of the face view as an elevation poly in the frame of an upright bone."""
    return {"id": ident, "kind": "poly", "points": uv(points, y0), "closed": closed, **fields}


def poly2(ident, points, closed=True, **fields):
    """🔶 A 2D poly in plain view coordinates (front and plan)."""
    return {"id": ident, "kind": "poly", "points": [[r6(a), r6(b)] for (a, b) in points], "closed": closed, **fields}
