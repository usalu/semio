"""🧮️ Geometry kit of the used double swing: bone frames of the sagittal rig, solid builders in bone-local metres, surface samples of every solid, convex outlines for the three authored views and a closed heightfield polyhedron for the gravel bed.

Every bone of this rig rests in the sagittal plane at lateral position 0, so a solid's `w` is the world lateral position z and a bone frame is an origin and one angle about the lateral axis.

@see ../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/README.md — section "Volume" (solid kinds) and "Front and plan views"
"""
import math

R5 = 5


def rnd(value):
    """🔢️ Rounds a number or a nested list of numbers to five decimals (0.01 mm)."""
    if isinstance(value, (list, tuple)):
        return [rnd(v) for v in value]
    return round(value + 0.0, R5)


class Frames:
    """🦴️ World placement (origin x, y and angle in degrees) of every bone of a sagittal rig at rest."""

    def __init__(self, bones):
        self.bones = {b["id"]: b for b in bones}
        self.world = {}
        for b in bones:
            self._solve(b["id"])

    def _solve(self, bid):
        if bid in self.world:
            return self.world[bid]
        b = self.bones[bid]
        if b["parent"] is None:
            w = (b["x"], b["y"], b["angleDeg"])
        else:
            px, py, pa = self._solve(b["parent"])
            c, s = math.cos(math.radians(pa)), math.sin(math.radians(pa))
            w = (px + c * b["x"] - s * b["y"], py + s * b["x"] + c * b["y"], pa + b["angleDeg"])
        self.world[bid] = w
        return w

    def to_world(self, bid, u, v):
        ox, oy, a = self.world[bid]
        c, s = math.cos(math.radians(a)), math.sin(math.radians(a))
        return ox + c * u - s * v, oy + s * u + c * v

    def to_local(self, bid, x, y):
        ox, oy, a = self.world[bid]
        c, s = math.cos(math.radians(a)), math.sin(math.radians(a))
        dx, dy = x - ox, y - oy
        return c * dx + s * dy, -s * dx + c * dy

    def local_dir(self, bid, dx, dy):
        a = math.radians(self.world[bid][2])
        c, s = math.cos(a), math.sin(a)
        return c * dx + s * dy, -s * dx + c * dy


def quat_w(deg):
    """🔄️ The unit quaternion [x, y, z, w] of a turn by `deg` about the bone-local lateral axis w."""
    h = math.radians(deg) / 2
    return [0.0, 0.0, round(math.sin(h), 8), round(math.cos(h), 8)]


def solid(sid, kind, bone, material, role, tone, **fields):
    """🧱️ One volume solid with the common fields first."""
    out = {"id": sid, "kind": kind, "bone": bone, "material": material, "role": role, "tone": tone}
    for k, v in fields.items():
        if v is None:
            continue
        out[k] = rnd(v) if k in ("size", "radii", "p0", "p1", "r0", "r1", "profile", "holes", "axis", "origin", "from", "to", "vertices") else v
    return out


def box(sid, bone, material, role, tone, centre, size, angle=0.0, **extra):
    """📦️ A box of `size` [u, v, w] centred at `centre` [u, v, w], turned by `angle` degrees about w."""
    tr = {"translation": rnd(list(centre))}
    if abs(angle) > 1e-9:
        tr["rotation"] = quat_w(angle)
    return solid(sid, "box", bone, material, role, tone, size=list(size), transform=tr, **extra)


def cylinder(sid, bone, material, role, tone, p0, p1, r, r_in=None, **extra):
    """🪵️ A straight cylinder (solid or a tube with bore `r_in`) from `p0` to `p1` [u, v, w] as a revolve about its axis."""
    d = [p1[i] - p0[i] for i in range(3)]
    length = math.sqrt(sum(c * c for c in d))
    axis = [c / length for c in d]
    if r_in is None:
        profile = [[0.0, 0.0], [0.0, r], [length, r], [length, 0.0]]
    else:
        profile = [[0.0, r_in], [0.0, r], [length, r], [length, r_in], [0.0, r_in]]
    return solid(sid, "revolve", bone, material, role, tone, profile=profile, axis=axis, origin=list(p0), **extra)


def torus(sid, bone, material, role, tone, centre, axis, major, minor, segments=12, **extra):
    """🍩️ A ring of wire radius `minor` on a circle of radius `major` about `axis` through `centre`, as a closed revolve loop."""
    loop = [[minor * math.cos(2 * math.pi * k / segments), major + minor * math.sin(2 * math.pi * k / segments)] for k in range(segments)]
    loop.append(loop[0])
    return solid(sid, "revolve", bone, material, role, tone, profile=loop, axis=list(axis), origin=list(centre), **extra)


def capsule(sid, bone, material, role, tone, p0, p1, r, **extra):
    """💊️ A capsule of radius `r` between `p0` and `p1` [u, v, w]."""
    return solid(sid, "capsule", bone, material, role, tone, p0=list(p0), p1=list(p1), r0=r, r1=r, **extra)


def prism(sid, bone, material, role, tone, profile, w0, w1, **extra):
    """🔷️ A profile in the bone's (u, v) plane extruded laterally from w0 to w1."""
    return solid(sid, "extrude", bone, material, role, tone, profile=[list(p) for p in profile], plane="xy", **{"from": w0, "to": w1}, **extra)


def prism_u(sid, bone, material, role, tone, profile_vw, u0, u1, **extra):
    """➡️ A profile in the bone's (v, w) plane extruded along the bone from u0 to u1."""
    return solid(sid, "extrude", bone, material, role, tone, profile=[list(p) for p in profile_vw], plane="yz", **{"from": u0, "to": u1}, **extra)


def ellipsoid(sid, bone, material, role, tone, centre, radii, **extra):
    """🥚️ An ellipsoid of semi-axes `radii` at `centre`."""
    return solid(sid, "ellipsoid", bone, material, role, tone, radii=list(radii), transform={"translation": rnd(list(centre))}, **extra)


def _rot(q, p):
    x, y, z, w = q
    ux, uy, uz = x, y, z
    t = [2 * (uy * p[2] - uz * p[1]), 2 * (uz * p[0] - ux * p[2]), 2 * (ux * p[1] - uy * p[0])]
    return [p[0] + w * t[0] + (uy * t[2] - uz * t[1]), p[1] + w * t[1] + (uz * t[0] - ux * t[2]), p[2] + w * t[2] + (ux * t[1] - uy * t[0])]


def _place(s, pts):
    tr = s.get("transform") or {}
    out = []
    for p in pts:
        q = list(p)
        if "scale" in tr:
            q = [q[i] * tr["scale"][i] for i in range(3)]
        if "rotation" in tr:
            q = _rot(tr["rotation"], q)
        if "translation" in tr:
            q = [q[i] + tr["translation"][i] for i in range(3)]
        out.append(q)
    rep = s.get("repeat")
    if rep:
        base = list(out)
        for k in range(1, rep["count"]):
            step = rep.get("translation", [0, 0, 0])
            out += [[p[i] + k * step[i] for i in range(3)] for p in base]
    return out


def samples(s, n=24):
    """🎯️ Points on the surface of a solid in its bone-local frame, enough for its convex outline in any view."""
    k = s["kind"]
    pts = []
    if k == "box":
        a, b, c = [x / 2 for x in s["size"]]
        pts = [[i * a, j * b, l * c] for i in (-1, 1) for j in (-1, 1) for l in (-1, 1)]
    elif k == "ellipsoid":
        a, b, c = s["radii"]
        for i in range(n):
            t = 2 * math.pi * i / n
            pts += [[a * math.cos(t), b * math.sin(t), 0], [a * math.cos(t), 0, c * math.sin(t)], [0, b * math.cos(t), c * math.sin(t)]]
    elif k == "capsule":
        for p, r in ((s["p0"], s["r0"]), (s["p1"], s["r1"])):
            for i in range(n):
                t = 2 * math.pi * i / n
                for d in ([math.cos(t), math.sin(t), 0], [math.cos(t), 0, math.sin(t)], [0, math.cos(t), math.sin(t)]):
                    pts.append([p[j] + r * d[j] for j in range(3)])
    elif k == "revolve":
        ax = s["axis"]
        o = s["origin"]
        h = [1.0, 0.0, 0.0] if abs(ax[0]) < 0.9 else [0.0, 1.0, 0.0]
        e1 = [ax[1] * h[2] - ax[2] * h[1], ax[2] * h[0] - ax[0] * h[2], ax[0] * h[1] - ax[1] * h[0]]
        m = math.sqrt(sum(c * c for c in e1))
        e1 = [c / m for c in e1]
        e2 = [ax[1] * e1[2] - ax[2] * e1[1], ax[2] * e1[0] - ax[0] * e1[2], ax[0] * e1[1] - ax[1] * e1[0]]
        for a, r in s["profile"]:
            for i in range(n):
                t = 2 * math.pi * i / n
                pts.append([o[j] + a * ax[j] + r * (math.cos(t) * e1[j] + math.sin(t) * e2[j]) for j in range(3)])
    elif k == "extrude":
        for p in s["profile"]:
            for e in (s["from"], s["to"]):
                if s["plane"] == "xy":
                    pts.append([p[0], p[1], e])
                elif s["plane"] == "yz":
                    pts.append([e, p[0], p[1]])
                else:
                    pts.append([p[1], e, p[0]])
    elif k == "polyhedron":
        pts = [list(v) for v in s["vertices"]]
    return _place(s, pts)


def hull(points):
    """🪢️ Convex hull of 2D points (monotone chain), counter-clockwise, without repeated end point."""
    pts = sorted(set((round(p[0], 6), round(p[1], 6)) for p in points))
    if len(pts) <= 2:
        return [list(p) for p in pts]

    def cross(o, a, b):
        return (a[0] - o[0]) * (b[1] - o[1]) - (a[1] - o[1]) * (b[0] - o[0])

    lower, upper = [], []
    for p in pts:
        while len(lower) >= 2 and cross(lower[-2], lower[-1], p) <= 1e-12:
            lower.pop()
        lower.append(p)
    for p in reversed(pts):
        while len(upper) >= 2 and cross(upper[-2], upper[-1], p) <= 1e-12:
            upper.pop()
        upper.append(p)
    return [list(p) for p in lower[:-1] + upper[:-1]]


def simplify(ring, tol):
    """✂️ Drops hull vertices that deviate less than `tol` metres from the chord of their neighbours."""
    out = list(ring)
    changed = True
    while changed and len(out) > 4:
        changed = False
        for i in range(len(out)):
            a, b, c = out[i - 1], out[i], out[(i + 1) % len(out)]
            ab = math.hypot(c[0] - a[0], c[1] - a[1])
            if ab < 1e-12:
                continue
            d = abs((c[0] - a[0]) * (a[1] - b[1]) - (a[0] - b[0]) * (c[1] - a[1])) / ab
            if d < tol:
                out.pop(i)
                changed = True
                break
    return out


def views_of(frames, s, n=24):
    """👁️ The convex outlines of a solid in the three authored views: elevation in the bone's (u, v), front in world (z, y), plan in world (x, z)."""
    pts = samples(s, n)
    elev = hull([[p[0], p[1]] for p in pts])
    world = [frames.to_world(s["bone"], p[0], p[1]) + (p[2],) for p in pts]
    front = hull([[p[2], p[1]] for p in world])
    plan = hull([[p[0], p[2]] for p in world])
    return elev, front, plan


def heightfield(sid, bone, material, role, tone, xs, zs, top, bottom):
    """⛰️ A closed polyhedron over the rectilinear grid `xs` × `zs`: its top is the surface `top(x, z)`, its floor the plane `bottom`, every face wound outward."""
    nx, nz = len(xs), len(zs)
    verts = []
    for i, x in enumerate(xs):
        for j, z in enumerate(zs):
            verts.append([x, top(x, z), z])
    base = len(verts)
    for i, x in enumerate(xs):
        for j, z in enumerate(zs):
            verts.append([x, bottom, z])

    def t(i, j):
        return i * nz + j

    def b(i, j):
        return base + i * nz + j

    tris = []
    for i in range(nx - 1):
        for j in range(nz - 1):
            tris.append([t(i, j), t(i, j + 1), t(i + 1, j + 1)])
            tris.append([t(i, j), t(i + 1, j + 1), t(i + 1, j)])
            tris.append([b(i, j), b(i + 1, j + 1), b(i, j + 1)])
            tris.append([b(i, j), b(i + 1, j), b(i + 1, j + 1)])
    for i in range(nx - 1):
        tris.append([t(i, 0), t(i + 1, 0), b(i + 1, 0)])
        tris.append([t(i, 0), b(i + 1, 0), b(i, 0)])
        tris.append([t(i, nz - 1), b(i + 1, nz - 1), t(i + 1, nz - 1)])
        tris.append([t(i, nz - 1), b(i, nz - 1), b(i + 1, nz - 1)])
    for j in range(nz - 1):
        tris.append([t(0, j), b(0, j + 1), t(0, j + 1)])
        tris.append([t(0, j), b(0, j), b(0, j + 1)])
        tris.append([t(nx - 1, j), t(nx - 1, j + 1), b(nx - 1, j + 1)])
        tris.append([t(nx - 1, j), b(nx - 1, j + 1), b(nx - 1, j)])
    return solid(sid, "polyhedron", bone, material, role, tone, vertices=verts, triangles=tris)


def signed_volume(s):
    """🧪️ Signed volume of a polyhedron (positive when every face is wound outward)."""
    v = s["vertices"]
    total = 0.0
    for a, b, c in s["triangles"]:
        p, q, r = v[a], v[b], v[c]
        total += (p[0] * (q[1] * r[2] - q[2] * r[1]) - p[1] * (q[0] * r[2] - q[2] * r[0]) + p[2] * (q[0] * r[1] - q[1] * r[0])) / 6
    return total
