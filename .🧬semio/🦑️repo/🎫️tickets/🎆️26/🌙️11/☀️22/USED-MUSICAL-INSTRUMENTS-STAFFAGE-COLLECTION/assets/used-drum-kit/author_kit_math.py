"""🧮️ Frame math of the used drum kit: the product's rest and pose chain (turned, oriented, chain) re-stated in plain Python, vectors, convex hulls and solid surface areas.

@see ../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🦴️rig/🟦️.ts — `turned`, `oriented`, `chain`, `effectiveOffset`
"""
import math


def sincos(deg):
    """📐️ Sine and cosine of an angle in degrees, exact at multiples of 90."""
    r = deg % 360
    exact = {0: (0.0, 1.0), 90: (1.0, 0.0), 180: (0.0, -1.0), 270: (-1.0, 0.0)}
    if r in exact:
        return exact[r]
    return math.sin(math.radians(deg)), math.cos(math.radians(deg))


def add(a, b):
    """➕️ Vector sum."""
    return [a[0] + b[0], a[1] + b[1], a[2] + b[2]]


def sub(a, b):
    """➖️ Vector difference."""
    return [a[0] - b[0], a[1] - b[1], a[2] - b[2]]


def mul(a, s):
    """✖️ Vector times scalar."""
    return [a[0] * s, a[1] * s, a[2] * s]


def dot(a, b):
    """⚫️ Dot product."""
    return a[0] * b[0] + a[1] * b[1] + a[2] * b[2]


def cross(a, b):
    """❎️ Cross product."""
    return [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]


def norm(a):
    """📏️ Length of a vector."""
    return math.sqrt(dot(a, a))


def unit(a):
    """🎯️ Unit vector."""
    n = norm(a)
    return [a[0] / n, a[1] / n, a[2] / n]


def comb(a, s, b, t):
    """🔀️ Linear combination a·s + b·t."""
    return [a[0] * s + b[0] * t, a[1] * s + b[1] * t, a[2] * s + b[2] * t]


def turned(axes, angle, abduction, twist):
    """🔄️ The axes turned by `angle` about w, then `abduction` about the turned v, then `twist` about the turned u (product `turned`)."""
    u, v, w = axes
    ts, tc = sincos(angle)
    u1 = comb(u, tc, v, ts)
    v1 = comb(u, -ts, v, tc)
    ps, pc = sincos(abduction)
    u2 = comb(u1, pc, w, ps)
    w2 = comb(u1, -ps, w, pc)
    ws, wc = sincos(twist)
    return (u2, comb(v1, wc, w2, -ws), comb(v1, ws, w2, wc))


def oriented(parent, bone, off):
    """🧭️ The axes of a bone in its parent: rest placement then offset (product `oriented`)."""
    abd, tw = bone.get("abductionDeg", 0), bone.get("twistDeg", 0)
    a, b, c = off.get("angleDeg", 0), off.get("abductionDeg", 0), off.get("twistDeg", 0)
    if abd == 0 and tw == 0:
        return turned(parent, bone["angleDeg"] + a, b, c)
    if tw == 0 and a == 0:
        return turned(parent, bone["angleDeg"], abd + b, c)
    if a == 0 and b == 0:
        return turned(parent, bone["angleDeg"], abd, tw + c)
    return turned(turned(parent, bone["angleDeg"], abd, tw), a, b, c)


def effective(bone, off):
    """🔩️ The offset a joint realises (product `effectiveOffset`, without clamping to limits)."""
    j = bone["joint"]
    if j["type"] == "revolute":
        return {"angleDeg": off.get("angleDeg", 0), "abductionDeg": off.get("abductionDeg", 0), "twistDeg": off.get("twistDeg", 0), "dx": 0, "dy": 0}
    if j["type"] == "prismatic":
        s_, c_ = sincos(j.get("axisDeg", 0))
        s = off.get("dx", 0) * c_ + off.get("dy", 0) * s_
        return {"angleDeg": 0, "abductionDeg": 0, "twistDeg": 0, "dx": s * c_, "dy": s * s_}
    return {"angleDeg": 0, "abductionDeg": 0, "twistDeg": 0, "dx": 0, "dy": 0}


def frames(bones, offsets=None, root=(0.0, 0.0)):
    """🦴️ World frames {id: (o, u, v, w)} of every bone, parents first, for the given offsets (product `chain`)."""
    offsets = offsets or {}
    out = {}
    base = ([root[0], root[1], 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0])
    for b in bones:
        po, pu, pv, pw = out[b["parent"]] if b["parent"] else base
        e = effective(b, offsets.get(b["id"], {}))
        along = comb(pu, b["x"] + e["dx"], pv, b["y"] + e["dy"])
        o = add(add(po, along), mul(pw, b.get("z", 0)))
        u, v, w = oriented((pu, pv, pw), b, e)
        out[b["id"]] = (o, u, v, w)
    return out


def to_local(frame, p):
    """📍️ A world point in the (u, v, w) coordinates of a frame."""
    o, u, v, w = frame
    d = sub(p, o)
    return [dot(d, u), dot(d, v), dot(d, w)]


def to_world(frame, q):
    """🌍️ A local (u, v, w) point of a frame in world coordinates."""
    o, u, v, w = frame
    return [o[0] + q[0] * u[0] + q[1] * v[0] + q[2] * w[0], o[1] + q[0] * u[1] + q[1] * v[1] + q[2] * w[1], o[2] + q[0] * u[2] + q[1] * v[2] + q[2] * w[2]]


def dir_local(frame, d):
    """🧭️ A world direction in the axes of a frame."""
    _, u, v, w = frame
    return [dot(d, u), dot(d, v), dot(d, w)]


def hull(points):
    """🪢️ Convex hull of 2D points, counter-clockwise (monotone chain)."""
    pts = sorted(set((round(p[0], 6), round(p[1], 6)) for p in points))
    if len(pts) < 3:
        return pts

    def turn(o, a, b):
        return (a[0] - o[0]) * (b[1] - o[1]) - (a[1] - o[1]) * (b[0] - o[0])

    lower, upper = [], []
    for p in pts:
        while len(lower) >= 2 and turn(lower[-2], lower[-1], p) <= 1e-12:
            lower.pop()
        lower.append(p)
    for p in reversed(pts):
        while len(upper) >= 2 and turn(upper[-2], upper[-1], p) <= 1e-12:
            upper.pop()
        upper.append(p)
    return lower[:-1] + upper[:-1]


def perp_basis(axis):
    """⊥️ Two unit vectors perpendicular to an axis."""
    a = unit(axis)
    ref = [0.0, 0.0, 1.0] if abs(a[2]) < 0.9 else [1.0, 0.0, 0.0]
    e1 = unit(cross(a, ref))
    e2 = cross(a, e1)
    return e1, e2


def circle_points(centre, axis, r, n=48):
    """⭕️ Points of a circle of radius r about an axis through centre."""
    e1, e2 = perp_basis(axis)
    return [add(centre, comb(e1, r * math.cos(2 * math.pi * k / n), e2, r * math.sin(2 * math.pi * k / n))) for k in range(n)]


def area_cyl(r, length, ends=2):
    """🧴️ Surface of a solid cylinder with `ends` discs."""
    return 2 * math.pi * r * length + ends * math.pi * r * r


def area_tube(ro, ri, length):
    """🌀️ Outer surface of a tube whose bore is closed off: outer side plus the two annular rims."""
    return 2 * math.pi * ro * length + 2 * math.pi * (ro * ro - ri * ri)


def area_box(s):
    """📦️ Surface of a box."""
    return 2 * (s[0] * s[1] + s[1] * s[2] + s[0] * s[2])


def area_revolve(profile):
    """🏺️ Surface of a body revolved from an open profile closed through the axis (lateral surface plus end discs)."""
    total = 0.0
    for (a0, r0), (a1, r1) in zip(profile, profile[1:]):
        total += math.pi * (r0 + r1) * math.hypot(a1 - a0, r1 - r0)
    total += math.pi * profile[0][1] ** 2 + math.pi * profile[-1][1] ** 2
    return total


def volume_revolve(profile):
    """🫙️ Volume of a body revolved from an open profile closed through the axis (frusta)."""
    total = 0.0
    for (a0, r0), (a1, r1) in zip(profile, profile[1:]):
        total += math.pi * abs(a1 - a0) * (r0 * r0 + r0 * r1 + r1 * r1) / 3
    return total
