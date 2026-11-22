"""🧰 Geometry kit of the used dog waste station: world metres in (x ahead, y up, z left), bone-local solids and shapes out.

Every bone of this rig rests in the sagittal plane at a world angle of 0, 90 or -90 degrees, so a world box stays a box in its bone frame.
See the README of the staffage archive, sections "Front and plan views", "Solid placements" and "Volume".
"""
import math

BONES = {}


def bone_frame(bone, ox, oy, theta):
    """🦴 Registers the rest frame of a bone: world origin and world angle in degrees."""
    BONES[bone] = (ox, oy, theta)


def loc(bone, x, y):
    """📍 World (x, y) to bone-local (u, v) at rest."""
    ox, oy, t = BONES[bone]
    c, s = round(math.cos(math.radians(t)), 12), round(math.sin(math.radians(t)), 12)
    dx, dy = x - ox, y - oy
    return (r6(dx * c + dy * s), r6(-dx * s + dy * c))


def r6(v):
    """🔢 Rounds to micrometres."""
    return round(v + 0.0, 6)


def local_ranges(bone, x0, x1, y0, y1):
    """📐 The local u and v ranges of a world rectangle."""
    pts = [loc(bone, x, y) for x in (x0, x1) for y in (y0, y1)]
    us = [p[0] for p in pts]
    vs = [p[1] for p in pts]
    return min(us), max(us), min(vs), max(vs)


def base(sid, bone, material, role, tone, **extra):
    """🧱 The common fields of a volume solid."""
    out = {"id": sid, "bone": bone, "material": material, "role": role, "tone": tone}
    out.update(extra)
    return out


def box(sid, bone, material, role, tone, x0, x1, y0, y1, z0, z1, radius=None, **extra):
    """📦 An axis-aligned world box as a bone-local box solid."""
    u0, u1, v0, v1 = local_ranges(bone, x0, x1, y0, y1)
    out = base(sid, bone, material, role, tone, **extra)
    out.update({"kind": "box", "size": [r6(u1 - u0), r6(v1 - v0), r6(z1 - z0)]})
    if radius:
        out["cornerRadius"] = radius
    out["transform"] = {"translation": [r6((u0 + u1) / 2), r6((v0 + v1) / 2), r6((z0 + z1) / 2)]}
    return out


def plan_extrude(sid, bone, material, role, tone, pts, y0, y1, holes=None, **extra):
    """🧊 A plan outline (world x, z) pushed from height y0 to y1."""
    ox, oy, t = BONES[bone]
    if t == 90:
        plane, f, fr, to = "yz", (lambda x, z: [r6(-(x - ox)), r6(z)]), y0 - oy, y1 - oy
    elif t == 0:
        plane, f, fr, to = "zx", (lambda x, z: [r6(z), r6(x - ox)]), y0 - oy, y1 - oy
    else:
        plane, f, fr, to = "yz", (lambda x, z: [r6(x - ox), r6(z)]), -(y1 - oy), -(y0 - oy)
    out = base(sid, bone, material, role, tone, **extra)
    out.update({"kind": "extrude", "plane": plane, "profile": [f(x, z) for x, z in pts], "from": r6(fr), "to": r6(to)})
    if holes:
        out["holes"] = [[f(x, z) for x, z in h] for h in holes]
    return out


def front_extrude(sid, bone, material, role, tone, pts, x0, x1, holes=None, **extra):
    """🪟 A front outline (world z, y) pushed from x0 to x1."""
    ox, oy, t = BONES[bone]
    if t == 90:
        plane, f, fr, to = "zx", (lambda z, y: [r6(z), r6(y - oy)]), -(x1 - ox), -(x0 - ox)
    elif t == 0:
        plane, f, fr, to = "yz", (lambda z, y: [r6(y - oy), r6(z)]), x0 - ox, x1 - ox
    else:
        plane, f, fr, to = "zx", (lambda z, y: [r6(z), r6(-(y - oy))]), x0 - ox, x1 - ox
    out = base(sid, bone, material, role, tone, **extra)
    out.update({"kind": "extrude", "plane": plane, "profile": [f(z, y) for z, y in pts], "from": r6(fr), "to": r6(to)})
    if holes:
        out["holes"] = [[f(z, y) for z, y in h] for h in holes]
    return out


def side_extrude(sid, bone, material, role, tone, pts, z0, z1, holes=None, **extra):
    """🔪 A side outline (world x, y) pushed laterally from z0 to z1."""
    out = base(sid, bone, material, role, tone, **extra)
    out.update({"kind": "extrude", "plane": "xy", "profile": [list(loc(bone, x, y)) for x, y in pts], "from": r6(z0), "to": r6(z1)})
    if holes:
        out["holes"] = [[list(loc(bone, x, y)) for x, y in h] for h in holes]
    return out


def ellipsoid(sid, bone, material, role, tone, cx, cy, cz, rx, ry, rz, **extra):
    """🥚 A world-aligned ellipsoid."""
    ox, oy, t = BONES[bone]
    u, v = loc(bone, cx, cy)
    radii = [rx, ry, rz] if t == 0 else [ry, rx, rz]
    out = base(sid, bone, material, role, tone, **extra)
    out.update({"kind": "ellipsoid", "radii": [r6(r) for r in radii], "transform": {"translation": [u, v, r6(cz)]}})
    return out


def circle(cx, cy, r, n=24, start=0.0):
    """⭕ A polygon circle, counter-clockwise."""
    return [(r6(cx + r * math.cos(start + 2 * math.pi * k / n)), r6(cy + r * math.sin(start + 2 * math.pi * k / n))) for k in range(n)]


def rounded_rect(x0, x1, y0, y1, r, n=4):
    """▢ A rectangle with rounded corners, counter-clockwise."""
    pts = []
    for cx, cy, a0 in ((x1 - r, y0 + r, -90), (x1 - r, y1 - r, 0), (x0 + r, y1 - r, 90), (x0 + r, y0 + r, 180)):
        for k in range(n + 1):
            a = math.radians(a0 + 90 * k / n)
            pts.append((r6(cx + r * math.cos(a)), r6(cy + r * math.sin(a))))
    return pts


def rect_pts(x0, x1, y0, y1):
    """▭ A rectangle as four corners, counter-clockwise."""
    return [(x0, y0), (x1, y0), (x1, y1), (x0, y1)]


def area(pts):
    """📏 Signed shoelace area."""
    return sum(pts[i][0] * pts[(i + 1) % len(pts)][1] - pts[(i + 1) % len(pts)][0] * pts[i][1] for i in range(len(pts))) / 2


def centroid(pts):
    """🎯 Centroid of a simple polygon."""
    a = area(pts)
    cx = sum((pts[i][0] + pts[(i + 1) % len(pts)][0]) * (pts[i][0] * pts[(i + 1) % len(pts)][1] - pts[(i + 1) % len(pts)][0] * pts[i][1]) for i in range(len(pts))) / (6 * a)
    cy = sum((pts[i][1] + pts[(i + 1) % len(pts)][1]) * (pts[i][0] * pts[(i + 1) % len(pts)][1] - pts[(i + 1) % len(pts)][0] * pts[i][1]) for i in range(len(pts))) / (6 * a)
    return cx, cy


def volume_and_centre(solid):
    """⚖️ Volume (m³) and local centre (u, v, w) of a box, extrude or ellipsoid solid."""
    t = solid.get("transform", {}).get("translation", [0, 0, 0])
    if solid["kind"] == "box":
        s = solid["size"]
        return s[0] * s[1] * s[2], tuple(t)
    if solid["kind"] == "ellipsoid":
        r = solid["radii"]
        return 4 / 3 * math.pi * r[0] * r[1] * r[2], tuple(t)
    if solid["kind"] == "extrude":
        outer = solid["profile"]
        holes = solid.get("holes", [])
        ao = abs(area(outer))
        co = centroid(outer)
        a = ao
        mx, my = co[0] * ao, co[1] * ao
        for h in holes:
            ah = abs(area(h))
            ch = centroid(h)
            a -= ah
            mx -= ch[0] * ah
            my -= ch[1] * ah
        length = solid["to"] - solid["from"]
        p, q = mx / a, my / a
        m = (solid["from"] + solid["to"]) / 2
        if solid["plane"] == "xy":
            c = (p, q, m)
        elif solid["plane"] == "yz":
            c = (m, p, q)
        else:
            c = (q, m, p)
        return a * length, c
    raise ValueError(solid["kind"])


def shape(sid, bone, role, filled, tone, geometry, region=None, regions=None, material=None, pen=None, hatch=None, solid=None):
    """✏️ A 2D shape of one view (z is assigned later from the paint order)."""
    out = {"id": sid, "bone": bone, "role": role, "z": 0, "filled": filled, "tone": tone}
    out.update(geometry)
    if region:
        out["region"] = region
    if regions:
        out["regions"] = regions
    if material:
        out["material"] = material
    if pen is not None:
        out["penMm"] = pen
    if hatch:
        out["hatch"] = hatch
    if solid:
        out["solid"] = solid
    return out


def g_rect(x, y, w, h, r=None):
    """⬛ Rect geometry."""
    out = {"kind": "rect", "x": r6(x), "y": r6(y), "w": r6(w), "h": r6(h)}
    if r:
        out["r"] = r
    return out


def g_poly(pts, closed=True):
    """🔷 Poly geometry."""
    return {"kind": "poly", "points": [[r6(p[0]), r6(p[1])] for p in pts], "closed": closed}


def g_path(rings):
    """〰 Path geometry from closed rings."""
    d = " ".join("M " + " L ".join(f"{r6(p[0])} {r6(p[1])}" for p in ring) + " Z" for ring in rings)
    return {"kind": "path", "d": d}


def g_ellipse(cx, cy, rx, ry):
    """⭕ Ellipse geometry."""
    return {"kind": "ellipse", "cx": r6(cx), "cy": r6(cy), "rx": r6(rx), "ry": r6(ry)}


def el_rect(bone, x0, x1, y0, y1, r=None):
    """🖼 Elevation rect of a world rectangle in the bone frame."""
    u0, u1, v0, v1 = local_ranges(bone, x0, x1, y0, y1)
    return g_rect(u0, v0, u1 - u0, v1 - v0, r)


def el_poly(bone, pts, closed=True):
    """🖊 Elevation poly of world (x, y) points in the bone frame."""
    return g_poly([loc(bone, x, y) for x, y in pts], closed)


def el_path(bone, rings):
    """🖋 Elevation path of world rings in the bone frame."""
    return g_path([[loc(bone, x, y) for x, y in ring] for ring in rings])


def el_ellipse(bone, cx, cy, rx, ry):
    """🫧 Elevation ellipse in the bone frame."""
    u, v = loc(bone, cx, cy)
    t = BONES[bone][2]
    return g_ellipse(u, v, rx, ry) if t == 0 else g_ellipse(u, v, ry, rx)


def swept(z0, z1):
    """🌀 The extruded solid placement of a part that turns in the sagittal plane."""
    return {"kind": "extruded", "sweep": "w", "z0": r6(z0), "z1": r6(z1)}
