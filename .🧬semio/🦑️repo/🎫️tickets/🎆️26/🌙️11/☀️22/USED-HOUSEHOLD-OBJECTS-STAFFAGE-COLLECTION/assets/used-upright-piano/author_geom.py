"""📐 Geometry helpers of the used upright piano: solids with their volume and centre for the mass table, 2D shapes, and the bilingual text helper.

See also author-piano.py (the authoritative script) and ../../brief.md.
"""
import math


def T(en, de):
    """🔤 One bilingual text, English first."""
    return {"en": en, "de": de}


def r6(v):
    """🎯 Rounds a number to the micrometre."""
    return round(v, 6)


def pts(seq):
    """📍 Rounds a list of 2D points."""
    return [[r6(a), r6(b)] for a, b in seq]


def poly_area(p):
    """📐 Shoelace area of a simple polygon."""
    return abs(sum(p[i][0] * p[(i + 1) % len(p)][1] - p[(i + 1) % len(p)][0] * p[i][1] for i in range(len(p)))) / 2


def poly_centroid(p):
    """🎯 Area centroid of a simple polygon."""
    a = 0.0
    cx = cy = 0.0
    for i in range(len(p)):
        x0, y0 = p[i]
        x1, y1 = p[(i + 1) % len(p)]
        c = x0 * y1 - x1 * y0
        a += c
        cx += (x0 + x1) * c
        cy += (y0 + y1) * c
    a /= 2
    return cx / (6 * a), cy / (6 * a)


def arc(cx, cy, r, a0, a1, n):
    """🌈 Points on a circular arc from angle a0 to a1 (degrees), both ends included."""
    return [(cx + r * math.cos(math.radians(a0 + (a1 - a0) * i / (n - 1))), cy + r * math.sin(math.radians(a0 + (a1 - a0) * i / (n - 1)))) for i in range(n)]


def box(sid, bone, mat, tone, x0, x1, y0, y1, z0, z1, role="body", **kw):
    """📦 A box solid from its bone-local extents."""
    s = {"id": sid, "kind": "box", "bone": bone, "material": mat, "role": role, "tone": tone, "size": [r6(x1 - x0), r6(y1 - y0), r6(z1 - z0)], "transform": {"translation": [r6((x0 + x1) / 2), r6((y0 + y1) / 2), r6((z0 + z1) / 2)]}}
    s.update(kw)
    s["_vol"] = (x1 - x0) * (y1 - y0) * (z1 - z0)
    s["_c"] = ((x0 + x1) / 2, (y0 + y1) / 2)
    return s


def extrude(sid, bone, mat, tone, profile, plane, f, t, holes=None, role="body", taper=None, **kw):
    """🧱 An extrude solid; volume and (u, v) centre are kept for the mass table."""
    s = {"id": sid, "kind": "extrude", "bone": bone, "material": mat, "role": role, "tone": tone, "profile": pts(profile), "plane": plane, "from": r6(f), "to": r6(t)}
    area = poly_area(profile)
    ca = poly_centroid(profile)
    mx, my = ca[0] * area, ca[1] * area
    if holes:
        s["holes"] = [pts(h) for h in holes]
        for h in holes:
            ha = poly_area(h)
            hc = poly_centroid(h)
            area -= ha
            mx -= hc[0] * ha
            my -= hc[1] * ha
    c = (mx / area, my / area)
    k = 1.0
    if taper:
        s["taperTo"] = list(taper)
        a, b = taper
        k = (a * a + a * b + b * b) / 3
    s.update(kw)
    s["_vol"] = area * (t - f) * k
    mid = (f + t) / 2
    s["_c"] = {"xy": (c[0], c[1]), "yz": (mid, c[0]), "zx": (c[1], mid)}[plane]
    return s


def revolve(sid, bone, mat, tone, profile, axis, origin, role="body", **kw):
    """🌀 A revolve solid; disk-method volume, centre at its origin in the (u, v) plane plus the axial shift."""
    s = {"id": sid, "kind": "revolve", "bone": bone, "material": mat, "role": role, "tone": tone, "profile": pts(profile), "axis": axis, "origin": [r6(o) for o in origin]}
    s.update(kw)
    vol = 0.0
    mom = 0.0
    for i in range(len(profile) - 1):
        a0, r0 = profile[i]
        a1, r1 = profile[i + 1]
        dv = math.pi * (a1 - a0) * (r0 * r0 + r0 * r1 + r1 * r1) / 3
        vol += dv
        mom += dv * (a0 + a1) / 2
    s["_vol"] = abs(vol)
    sh = mom / vol if vol else 0.0
    s["_c"] = (origin[0] + axis[0] * sh, origin[1] + axis[1] * sh)
    return s


def with_repeat(s, count, step):
    """🔁 Repeats a solid `count` times by the bone-local step; the mass table counts every copy."""
    s["repeat"] = {"count": count, "translation": [r6(v) for v in step]}
    s["_vol"] *= count
    s["_c"] = (s["_c"][0] + step[0] * (count - 1) / 2, s["_c"][1] + step[1] * (count - 1) / 2)
    return s


def clean(lst):
    """🧽 Strips the private helper keys."""
    return [{k: v for k, v in s.items() if not k.startswith("_")} for s in lst]


def sh(sid, bone, kind, role, z, tone, filled=True, region=None, regions=None, material=None, pen=None, solid=None, **geo):
    """✏️ One 2D shape."""
    s = {"id": sid, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone, "kind": kind}
    s.update(geo)
    if region:
        s["region"] = region
    if regions:
        s["regions"] = regions
    if material:
        s["material"] = material
    if pen is not None:
        s["penMm"] = pen
    if solid:
        s["solid"] = solid
    return s


def rect(sid, bone, x0, y0, x1, y1, role="body", z=5, tone=0.5, **kw):
    """⬛ A rectangle shape from two corners."""
    return sh(sid, bone, "rect", role, z, tone, x=r6(min(x0, x1)), y=r6(min(y0, y1)), w=r6(abs(x1 - x0)), h=r6(abs(y1 - y0)), **kw)


def poly(sid, bone, points, role="body", z=5, tone=0.5, closed=True, filled=True, **kw):
    """🔷 A polygon or polyline shape."""
    return sh(sid, bone, "poly", role, z, tone, filled=filled, points=pts(points), closed=closed, **kw)


def circle(sid, bone, cx, cy, r, role="detail", z=7, tone=0.4, **kw):
    """⭕ A circle shape."""
    return sh(sid, bone, "ellipse", role, z, tone, cx=r6(cx), cy=r6(cy), rx=r6(r), ry=r6(r), **kw)


def tex(sid, bone, points, z=7, closed=False, tone=0.3):
    """〰️ A texture stroke (traces of use, grain, seams)."""
    return poly(sid, bone, points, role="texture", z=z, tone=tone, closed=closed, filled=False)


def det(sid, bone, points, z=7, closed=False, tone=0.3):
    """➖ A detail stroke (joints, key gaps, panel frames)."""
    return poly(sid, bone, points, role="detail", z=z, tone=tone, closed=closed, filled=False)


def rect_pts(x0, y0, x1, y1):
    """▭ The four corners of a rectangle, counter-clockwise."""
    return [(x0, y0), (x1, y0), (x1, y1), (x0, y1)]


def swept(sid, bone, profile, z0, z1, role="body", z=5, tone=0.5, **kw):
    """🪚 A front or plan shape of a part that turns with its bone: the bone-local side profile swept laterally from z0 to z1 (rest world z)."""
    return poly(sid, bone, profile, role=role, z=z, tone=tone, solid={"kind": "extruded", "sweep": "w", "z0": r6(z0), "z1": r6(z1)}, **kw)
