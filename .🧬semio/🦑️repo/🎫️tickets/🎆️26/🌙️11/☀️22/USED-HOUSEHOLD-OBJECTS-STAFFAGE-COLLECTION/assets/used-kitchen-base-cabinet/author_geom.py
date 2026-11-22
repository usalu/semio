"""🧰 Geometry kit of the used kitchen base cabinet author: bone frames, world boxes to bone-local solids and to the three 2D views.

World rest frame of the staffage: x ahead (the cabinet front), y up, z left. Two frame kinds occur in this rig:
`S` sagittal bones (u = x, v = y, w = z) and `U` upright door bones (angleDeg 90: u = y, v = -x, w = z).
Front shapes are drawn in world (z, y), plan shapes in world (x, z), elevation shapes in bone-local (u, v);
door shapes carry an extruded `solid` swept along w so all three views follow the door swing.

@see ../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/README.md — "Solid placements", "Volume"
"""
import math

R6 = lambda v: round(v + 0.0, 6)


def T(en, de):
    """🗣 A bilingual text, English first."""
    return {"en": en, "de": de}


class Frame:
    """🧭 The rest frame of one bone: kind `S` or `U` and its world origin."""

    def __init__(self, kind, ox, oy, oz):
        self.kind, self.o = kind, (ox, oy, oz)

    def local(self, x, y, z):
        ox, oy, oz = self.o
        return (x - ox, y - oy, z - oz) if self.kind == "S" else (y - oy, -(x - ox), z - oz)

    def box_local(self, lo, hi):
        a = self.local(*lo)
        b = self.local(*hi)
        mn = tuple(min(p, q) for p, q in zip(a, b))
        mx = tuple(max(p, q) for p, q in zip(a, b))
        return mn, mx

    def uv(self, x, y):
        ox, oy, _ = self.o
        return (x - ox, y - oy) if self.kind == "S" else (y - oy, -(x - ox))


def solid_box(sid, bone, frame, lo, hi, material, tone, role="body", **extra):
    """📦 A `box` solid from a world AABB, expressed in the bone frame."""
    mn, mx = frame.box_local(lo, hi)
    size = [R6(mx[i] - mn[i]) for i in range(3)]
    c = [R6((mx[i] + mn[i]) / 2) for i in range(3)]
    s = {"id": sid, "kind": "box", "bone": bone, "material": material, "role": role, "tone": tone, "size": size, "transform": {"translation": c}}
    s.update(extra)
    return s


def rect_ring(x0, y0, x1, y1):
    return [[R6(x0), R6(y0)], [R6(x1), R6(y0)], [R6(x1), R6(y1)], [R6(x0), R6(y1)]]


def solid_extrude(sid, bone, plane, profile, frm, to, material, tone, role="body", holes=None, **extra):
    """📐 An `extrude` solid on `plane` (xy: (u, v) along w; yz: (v, w) along u; zx: (w, u) along v) in bone-local metres."""
    s = {"id": sid, "kind": "extrude", "bone": bone, "material": material, "role": role, "tone": tone, "plane": plane, "profile": [[R6(a), R6(b)] for a, b in profile], "from": R6(frm), "to": R6(to)}
    if holes:
        s["holes"] = [[[R6(a), R6(b)] for a, b in h] for h in holes]
    s.update(extra)
    return s


def solid_revolve(sid, bone, profile, axis, origin, material, tone, role="body", **extra):
    """🏺 A `revolve` solid: profile [axial, radial] about `axis` through `origin` (bone-local)."""
    s = {"id": sid, "kind": "revolve", "bone": bone, "material": material, "role": role, "tone": tone, "profile": [[R6(a), R6(b)] for a, b in profile], "axis": list(axis), "origin": [R6(c) for c in origin]}
    s.update(extra)
    return s


def solid_capsule(sid, bone, p0, p1, r0, r1, material, tone, role="body", **extra):
    """💊 A `capsule` solid between two bone-local points."""
    s = {"id": sid, "kind": "capsule", "bone": bone, "material": material, "role": role, "tone": tone, "p0": [R6(c) for c in p0], "p1": [R6(c) for c in p1], "r0": R6(r0), "r1": R6(r1)}
    s.update(extra)
    return s


def shape(sid, bone, role, z, filled, tone, geom, **extra):
    """✏️ One 2D shape."""
    s = {"id": sid, "bone": bone, "role": role, "z": round(z, 3), "filled": filled, "tone": tone}
    s.update(geom)
    s.update({k: v for k, v in extra.items() if v is not None})
    return s


def g_rect(x, y, w, h, r=None):
    g = {"kind": "rect", "x": R6(x), "y": R6(y), "w": R6(w), "h": R6(h)}
    if r:
        g["r"] = R6(r)
    return g


def g_poly(points, closed=True):
    return {"kind": "poly", "points": [[R6(a), R6(b)] for a, b in points], "closed": closed}


def g_ellipse(cx, cy, rx, ry):
    return {"kind": "ellipse", "cx": R6(cx), "cy": R6(cy), "rx": R6(rx), "ry": R6(ry)}


def g_capsule(x0, y0, x1, y1, r0, r1):
    return {"kind": "capsule", "x0": R6(x0), "y0": R6(y0), "x1": R6(x1), "y1": R6(y1), "r0": R6(r0), "r1": R6(r1)}


def g_path_rings(rings):
    d = " ".join("M " + " L ".join(f"{R6(a):.6f} {R6(b):.6f}" for a, b in ring) + " Z" for ring in rings)
    return {"kind": "path", "d": d}


def box_views(frame, bone, lo, hi):
    """🪟 The three 2D geometries of a world AABB on its bone: elevation (bone-local u, v), front (world z, y), plan (world x, z);
    for an upright door bone every view is the (u, v) profile with an extruded solid swept along w over the world z span."""
    x0, y0, z0 = lo
    x1, y1, z1 = hi
    if frame.kind == "S":
        u0, v0 = frame.uv(x0, y0)
        return {
            "elevation": (g_rect(u0, v0, x1 - x0, y1 - y0), None),
            "front": (g_rect(z0, y0, z1 - z0, y1 - y0), None),
            "plan": (g_rect(x0, z0, x1 - x0, z1 - z0), None),
        }
    ua, va = frame.uv(x0, y0)
    ub, vb = frame.uv(x1, y1)
    g = g_rect(min(ua, ub), min(va, vb), abs(ub - ua), abs(vb - va))
    sol = {"kind": "extruded", "sweep": "w", "z0": R6(z0), "z1": R6(z1)}
    return {"elevation": (g, sol), "front": (g, sol), "plan": (g, sol)}


def depth_z(view, lo, hi, base=0.0):
    """🎚 Paint order from depth: elevation nearer = smaller world z, front nearer = larger x, plan nearer = larger y."""
    if view == "elevation":
        return base + 5.0 - 4.0 * lo[2]
    if view == "front":
        return base + 4.0 * hi[0]
    return base + 4.0 * hi[1]


def box_volume(lo, hi):
    return (hi[0] - lo[0]) * (hi[1] - lo[1]) * (hi[2] - lo[2])


def poly_area(pts):
    a = 0.0
    for i in range(len(pts)):
        x0, y0 = pts[i]
        x1, y1 = pts[(i + 1) % len(pts)]
        a += x0 * y1 - x1 * y0
    return abs(a) / 2


def poly_centroid(pts):
    a = 0.0
    cx = cy = 0.0
    for i in range(len(pts)):
        x0, y0 = pts[i]
        x1, y1 = pts[(i + 1) % len(pts)]
        f = x0 * y1 - x1 * y0
        a += f
        cx += (x0 + x1) * f
        cy += (y0 + y1) * f
    a /= 2
    return cx / (6 * a), cy / (6 * a)


def revolve_volume(profile):
    """🧮 Volume of a profile closed through the axis, revolved a full turn (Pappus over the closed ring with the axis points)."""
    ring = [(0.0, profile[0][0])] + [(r, ax) for ax, r in profile] + [(0.0, profile[-1][0])]
    ring = [(r, ax) for r, ax in ring]
    a = 0.0
    m = 0.0
    for i in range(len(ring)):
        r0, z0 = ring[i]
        r1, z1 = ring[(i + 1) % len(ring)]
        f = r0 * z1 - r1 * z0
        a += f
        m += (r0 + r1) * f
    a /= 2
    rc = m / (6 * a) if a else 0
    return abs(2 * math.pi * rc * a)


def circle_pts(cx, cy, r, n=24):
    return [(cx + r * math.cos(2 * math.pi * k / n), cy + r * math.sin(2 * math.pi * k / n)) for k in range(n)]
