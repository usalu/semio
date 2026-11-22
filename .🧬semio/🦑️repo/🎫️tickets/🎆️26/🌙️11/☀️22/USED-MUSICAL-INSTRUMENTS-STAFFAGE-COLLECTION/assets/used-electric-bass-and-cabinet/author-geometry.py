"""🧰 Geometry kernel of the used electric bass and cabinet: bone rest frames, primitives that write one body solid and its three 2D views from the same numbers, and the bass prisms whose drawings the evaluator places by their `solid` sweep.

Imported by `author-bass.py`; never writes a file itself.
"""
import math

R3 = lambda x: round(x, 5)


def p2(points):
    """📍 Rounds a list of 2D points for the document."""
    return [[R3(a), R3(b)] for a, b in points]


class Frame:
    """🦴 The rest frame of a bone: origin (x, y, z) in world metres and the rest angle about the lateral axis (every bone of this asset rests with w = +z)."""

    def __init__(self, ox, oy, oz, deg):
        self.o = (ox, oy, oz)
        self.deg = deg
        self.c = math.cos(math.radians(deg))
        self.s = math.sin(math.radians(deg))

    def world(self, u, v, w):
        """🌍 A bone-local point in world metres."""
        return (self.o[0] + u * self.c - v * self.s, self.o[1] + u * self.s + v * self.c, self.o[2] + w)

    def local(self, x, y, z):
        """🏠 A world point in bone-local metres."""
        dx, dy = x - self.o[0], y - self.o[1]
        return (dx * self.c + dy * self.s, -dx * self.s + dy * self.c, z - self.o[2])

    def axis(self, name):
        """🧭 A bone axis as a world vector."""
        if name == "u":
            return (self.c, self.s, 0.0)
        if name == "v":
            return (-self.s, self.c, 0.0)
        return (0.0, 0.0, 1.0)


def view2(view, p):
    """🖼️ The paper coordinates of a world point in the front (z, y) or plan (x, z) view."""
    return (p[2], p[1]) if view == "front" else (p[0], p[2])


def rect(x0, x1, y0, y1, r=None):
    """⬛ A rect shape geometry from two ranges."""
    g = {"kind": "rect", "x": R3(min(x0, x1)), "y": R3(min(y0, y1)), "w": R3(abs(x1 - x0)), "h": R3(abs(y1 - y0))}
    if r:
        g["r"] = R3(min(r, abs(x1 - x0) / 2, abs(y1 - y0) / 2))
    return g


def poly(points, closed=True):
    """🔷 A poly shape geometry."""
    return {"kind": "poly", "points": p2(points), "closed": closed}


def ellipse(cx, cy, rx, ry):
    """⭕ An ellipse shape geometry."""
    return {"kind": "ellipse", "cx": R3(cx), "cy": R3(cy), "rx": R3(rx), "ry": R3(ry)}


def capsule(a, b, r):
    """💊 A capsule shape geometry between two paper points."""
    return {"kind": "capsule", "x0": R3(a[0]), "y0": R3(a[1]), "x1": R3(b[0]), "y1": R3(b[1]), "r0": R3(r), "r1": R3(r)}


def path(rings):
    """〰️ A path of closed rings."""
    return {"kind": "path", "d": " ".join("M " + " L ".join(f"{R3(a)} {R3(b)}" for a, b in ring) + " Z" for ring in rings)}


def bbox(points):
    """📦 The extents of a list of 2D points."""
    xs = [p[0] for p in points]
    ys = [p[1] for p in points]
    return min(xs), max(xs), min(ys), max(ys)


def circle_points(cx, cy, r, n=16, phase=0.0):
    """🔘 A polygon approximating a circle."""
    return [(cx + r * math.cos(phase + 2 * math.pi * k / n), cy + r * math.sin(phase + 2 * math.pi * k / n)) for k in range(n)]


def catmull(control, per=6):
    """🪡 A closed Catmull-Rom curve through the control points, `per` points per span."""
    n = len(control)
    out = []
    for i in range(n):
        p0, p1, p2_, p3 = control[(i - 1) % n], control[i], control[(i + 1) % n], control[(i + 2) % n]
        for k in range(per):
            t = k / per
            t2, t3 = t * t, t * t * t
            out.append(tuple(0.5 * ((2 * p1[j]) + (-p0[j] + p2_[j]) * t + (2 * p0[j] - 5 * p1[j] + 4 * p2_[j] - p3[j]) * t2 + (-p0[j] + 3 * p1[j] - 3 * p2_[j] + p3[j]) * t3) for j in range(2)))
    return out


def area(ring):
    """📐 The signed area of a ring."""
    return 0.5 * sum(ring[i][0] * ring[(i + 1) % len(ring)][1] - ring[(i + 1) % len(ring)][0] * ring[i][1] for i in range(len(ring)))


def centroid(ring):
    """🎯 The centroid of a simple ring."""
    a = area(ring)
    cx = sum((ring[i][0] + ring[(i + 1) % len(ring)][0]) * (ring[i][0] * ring[(i + 1) % len(ring)][1] - ring[(i + 1) % len(ring)][0] * ring[i][1]) for i in range(len(ring))) / (6 * a)
    cy = sum((ring[i][1] + ring[(i + 1) % len(ring)][1]) * (ring[i][0] * ring[(i + 1) % len(ring)][1] - ring[(i + 1) % len(ring)][0] * ring[i][1]) for i in range(len(ring))) / (6 * a)
    return cx, cy


def ccw(ring):
    """🔄 The ring wound counter-clockwise."""
    return ring if area(ring) > 0 else list(reversed(ring))


class Part:
    """🧩 One physical part at one scale: its body solid, the region and material it belongs to, and its three 2D shape geometries (or None where a view does not draw it)."""

    def __init__(self, pid, bone, material, region, role, tone, solid, views, corners, extra=None):
        self.id = pid
        self.bone = bone
        self.material = material
        self.region = region
        self.role = role
        self.tone = tone
        self.solid = solid
        self.views = views
        self.corners = corners
        self.extra = extra or {}
        self.flags = {}


def box(frames, pid, bone, material, region, role, tone, lo, hi, cr=0.0, views=("elevation", "front", "plan")):
    """⬛ An axis-aligned box from local corner `lo` to `hi` on a bone resting at a multiple of 90 degrees; its drawings are the rects of its world extents."""
    f = frames[bone]
    size = [hi[i] - lo[i] for i in range(3)]
    centre = [(hi[i] + lo[i]) / 2 for i in range(3)]
    solid = {"kind": "box", "size": [R3(s) for s in size]}
    if cr:
        solid["cornerRadius"] = R3(cr)
    solid["transform"] = {"translation": [R3(c) for c in centre]}
    corners = [f.world(a, b, c) for a in (lo[0], hi[0]) for b in (lo[1], hi[1]) for c in (lo[2], hi[2])]
    out = {}
    if "elevation" in views:
        out["elevation"] = rect(lo[0], hi[0], lo[1], hi[1], cr)
    for view in ("front", "plan"):
        if view in views:
            x0, x1, y0, y1 = bbox([view2(view, p) for p in corners])
            out[view] = rect(x0, x1, y0, y1, cr)
    return Part(pid, bone, material, region, role, tone, solid, out, corners)


def cylinder(frames, pid, bone, material, region, role, tone, centre, axis, radius, length, views=("elevation", "front", "plan")):
    """🛢️ A turned cylinder about the local axis `u`, `v` or `w` through `centre`, as a revolve; its drawings are a circle where the view looks along the axis and a rect elsewhere."""
    f = frames[bone]
    vec = {"u": [1, 0, 0], "v": [0, 1, 0], "w": [0, 0, 1]}[axis]
    solid = {"kind": "revolve", "profile": p2([(-length / 2, 0), (-length / 2, radius), (length / 2, radius), (length / 2, 0)]), "axis": vec, "origin": [R3(c) for c in centre]}
    corners = []
    for sa in (-1, 1):
        for sb in (-1, 1):
            for sc in (-1, 1):
                d = [radius * sb, radius * sc]
                lp = list(centre)
                k = "uvw".index(axis)
                others = [i for i in range(3) if i != k]
                lp[k] += sa * length / 2
                lp[others[0]] += d[0]
                lp[others[1]] += d[1]
                corners.append(f.world(*lp))
    out = {}
    world_axis = f.axis(axis)
    if "elevation" in views:
        if axis == "w":
            out["elevation"] = ellipse(centre[0], centre[1], radius, radius)
        else:
            lo = [centre[i] - (length / 2 if "uvw"[i] == axis else radius) for i in range(2)]
            hi = [centre[i] + (length / 2 if "uvw"[i] == axis else radius) for i in range(2)]
            out["elevation"] = rect(lo[0], hi[0], lo[1], hi[1])
    for view in ("front", "plan"):
        if view not in views:
            continue
        normal = (1, 0, 0) if view == "front" else (0, 1, 0)
        if abs(sum(world_axis[i] * normal[i] for i in range(3))) > 0.999:
            c = view2(view, f.world(*centre))
            out[view] = ellipse(c[0], c[1], radius, radius)
        else:
            x0, x1, y0, y1 = bbox([view2(view, p) for p in corners])
            out[view] = rect(x0, x1, y0, y1)
    return Part(pid, bone, material, region, role, tone, solid, out, corners)


def tube(frames, pid, bone, material, region, role, tone, p0, p1, radius, views=("elevation", "front", "plan")):
    """💊 A capsule between two local points: tubes, sleeves and rubber caps; its drawings are the capsules of the projected end points."""
    f = frames[bone]
    solid = {"kind": "capsule", "p0": [R3(c) for c in p0], "p1": [R3(c) for c in p1], "r0": R3(radius), "r1": R3(radius)}
    w0, w1 = f.world(*p0), f.world(*p1)
    corners = [(w[0] + dx, w[1] + dy, w[2] + dz) for w in (w0, w1) for dx in (-radius, radius) for dy in (-radius, radius) for dz in (-radius, radius)]
    out = {}
    if "elevation" in views:
        out["elevation"] = capsule((p0[0], p0[1]), (p1[0], p1[1]), radius)
    for view in ("front", "plan"):
        if view in views:
            out[view] = capsule(view2(view, w0), view2(view, w1), radius)
    return Part(pid, bone, material, region, role, tone, solid, out, corners)


def extrude_xy(frames, pid, bone, material, region, role, tone, profile, w0, w1, holes=None, views=("elevation", "front", "plan")):
    """🪚 A prism of a side profile (u, v) pushed along w from `w0` to `w1`: the elevation is the profile, front and plan the rects of its world extents."""
    f = frames[bone]
    solid = {"kind": "extrude", "profile": p2(profile), "plane": "xy", "from": R3(w0), "to": R3(w1)}
    if holes:
        solid["holes"] = [p2(h) for h in holes]
    corners = [f.world(a, b, c) for a, b in profile for c in (w0, w1)]
    out = {}
    if "elevation" in views:
        out["elevation"] = path([profile] + list(holes)) if holes else poly(profile)
    for view in ("front", "plan"):
        if view in views:
            x0, x1, y0, y1 = bbox([view2(view, p) for p in corners])
            out[view] = rect(x0, x1, y0, y1)
    return Part(pid, bone, material, region, role, tone, solid, out, corners)


def extrude_yz(frames, pid, bone, material, region, role, tone, profile, u0, u1, holes=None, views=("elevation", "front", "plan")):
    """🪟 A prism of a front profile (v, w) pushed along u from `u0` to `u1` on an unrotated bone: the front view is the profile, elevation and plan the rects of its extents."""
    f = frames[bone]
    solid = {"kind": "extrude", "profile": p2(profile), "plane": "yz", "from": R3(u0), "to": R3(u1)}
    if holes:
        solid["holes"] = [p2(h) for h in holes]
    corners = [f.world(a, b, c) for b, c in profile for a in (u0, u1)]
    out = {}
    vs = [p[0] for p in profile]
    if "elevation" in views:
        out["elevation"] = rect(u0, u1, min(vs), max(vs))
    if "front" in views:
        def fw(ring):
            return [view2("front", f.world(u0, b, c)) for b, c in ring]
        out["front"] = path([fw(profile)] + [fw(h) for h in holes]) if holes else poly(fw(profile))
    if "plan" in views:
        x0, x1, y0, y1 = bbox([view2("plan", p) for p in corners])
        out["plan"] = rect(x0, x1, y0, y1)
    return Part(pid, bone, material, region, role, tone, solid, out, corners)


def extrude_zx(frames, pid, bone, material, region, role, tone, profile, v0, v1, views=("elevation", "front", "plan")):
    """🗺️ A prism of a plan profile (w, u) pushed along v from `v0` to `v1` on an unrotated bone: the plan is the profile, elevation and front the rects of its extents."""
    f = frames[bone]
    solid = {"kind": "extrude", "profile": p2(profile), "plane": "zx", "from": R3(v0), "to": R3(v1)}
    corners = [f.world(a, b, c) for c, a in profile for b in (v0, v1)]
    out = {}
    us = [p[1] for p in profile]
    if "elevation" in views:
        out["elevation"] = rect(min(us), max(us), v0, v1)
    if "front" in views:
        x0, x1, y0, y1 = bbox([view2("front", p) for p in corners])
        out["front"] = rect(x0, x1, y0, y1)
    if "plan" in views:
        out["plan"] = poly([view2("plan", f.world(a, v0, c)) for c, a in profile])
    return Part(pid, bone, material, region, role, tone, solid, out, corners)


def prism_v(frames, pid, bone, material, region, role, tone, profile, v0, v1, views=("elevation", "front", "plan"), filled=True):
    """🎸 A bass part: the face profile (s along the bone, w lateral) pushed along the bone normal from `v0` to `v1` (an `extrude` on plane zx), drawn in every view by the same profile with a `solid` sweep `v`, so the evaluator places it in every pose."""
    f = frames[bone]
    solid = {"kind": "extrude", "profile": p2([(w, s) for s, w in profile]), "plane": "zx", "from": R3(v0), "to": R3(v1)}
    corners = [f.world(s, b, w) for s, w in profile for b in (v0, v1)]
    out = {}
    for view in views:
        g = poly(profile)
        g["solid"] = {"kind": "extruded", "sweep": "v", "z0": R3(v0), "z1": R3(v1)}
        out[view] = g
    part = Part(pid, bone, material, region, role, tone, solid, out, corners)
    part.flags["filled"] = filled
    return part

