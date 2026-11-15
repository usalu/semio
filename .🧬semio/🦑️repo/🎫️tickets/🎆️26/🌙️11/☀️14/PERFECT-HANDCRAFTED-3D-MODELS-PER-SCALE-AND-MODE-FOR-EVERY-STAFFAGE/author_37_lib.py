from author_11_lib import *
import math

QTOP = [0.70711, 0, 0.70711, 0]


def qzr(phi_deg):
    a = math.radians(phi_deg) / 2
    return qmul([0, 0, math.sin(a), math.cos(a)], QTOP)


def psec(x, y, phi, rw, rn, e=None, z=0.0):
    s = {"at": [x, y, z], "rx": rw, "ry": rn, "rotation": qzr(phi)}
    if e:
        s["exponent"] = e
    return s


def ploft(sid, ctx, stations, cs=True, ce=True, e=None, **kw):
    return loft(sid, ctx, [psec(*st, e=e) for st in stations], cs, ce, **kw)


def bezier(p0, c1, c2, p3, n):
    out = []
    for k in range(n + 1):
        t = k / n
        out.append(tuple((1 - t) ** 3 * p0[i] + 3 * (1 - t) ** 2 * t * c1[i] + 3 * (1 - t) * t ** 2 * c2[i] + t ** 3 * p3[i] for i in range(2)))
    return out


def bezier_d(p0, c1, c2, p3, t):
    return tuple(3 * (1 - t) ** 2 * (c1[i] - p0[i]) + 6 * (1 - t) * t * (c2[i] - c1[i]) + 3 * t ** 2 * (p3[i] - c2[i]) for i in range(2))


def path_stations(p0, c1, c2, p3, n, rw, rn=None, wscale=None):
    out = []
    for k in range(n + 1):
        t = k / n
        x, y = [(1 - t) ** 3 * p0[i] + 3 * (1 - t) ** 2 * t * c1[i] + 3 * (1 - t) * t ** 2 * c2[i] + t ** 3 * p3[i] for i in range(2)]
        dx, dy = bezier_d(p0, c1, c2, p3, t)
        phi = math.degrees(math.atan2(dy, dx))
        r = rw(t) if callable(rw) else rw
        out.append((x, y, phi, r, r))
    return out


def ellipse_pts(cy, cz, ry, rz, n=24):
    return [[cy + ry * math.cos(2 * math.pi * k / n), cz + rz * math.sin(2 * math.pi * k / n)] for k in range(n)]


def ring(sid, ctx, profile, axis="v", origin=(0, 0, 0), **kw):
    """Revolve of a profile that does not touch the axis: closed explicitly (the tessellator closes an open profile through the axis)."""
    return rev(sid, ctx, [list(p) for p in profile] + [list(profile[0])], axis, origin, **kw)
