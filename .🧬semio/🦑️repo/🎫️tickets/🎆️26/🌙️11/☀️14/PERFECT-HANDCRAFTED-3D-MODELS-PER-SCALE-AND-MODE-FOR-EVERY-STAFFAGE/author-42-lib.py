import os, math
exec(open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "author-20-lib.py")).read())

REAR = (-0.13, 1.2)


def sub(a, b):
    return [a[i] - b[i] for i in range(3)]


def dot(a, b):
    return sum(a[i] * b[i] for i in range(3))


def cross(a, b):
    return [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]


def unit(a):
    n = math.sqrt(dot(a, a))
    return [c / n for c in a]


def quat_of(e1, e2, e3):
    m = [[e1[0], e2[0], e3[0]], [e1[1], e2[1], e3[1]], [e1[2], e2[2], e3[2]]]
    t = m[0][0] + m[1][1] + m[2][2]
    if t > 0:
        s = math.sqrt(t + 1) * 2
        q = [(m[2][1] - m[1][2]) / s, (m[0][2] - m[2][0]) / s, (m[1][0] - m[0][1]) / s, s / 4]
    elif m[0][0] > m[1][1] and m[0][0] > m[2][2]:
        s = math.sqrt(1 + m[0][0] - m[1][1] - m[2][2]) * 2
        q = [s / 4, (m[0][1] + m[1][0]) / s, (m[0][2] + m[2][0]) / s, (m[2][1] - m[1][2]) / s]
    elif m[1][1] > m[2][2]:
        s = math.sqrt(1 + m[1][1] - m[0][0] - m[2][2]) * 2
        q = [(m[0][1] + m[1][0]) / s, s / 4, (m[1][2] + m[2][1]) / s, (m[0][2] - m[2][0]) / s]
    else:
        s = math.sqrt(1 + m[2][2] - m[0][0] - m[1][1]) * 2
        q = [(m[0][2] + m[2][0]) / s, (m[1][2] + m[2][1]) / s, s / 4, (m[1][0] - m[0][1]) / s]
    return [r6(c) for c in q]


def loc(p, off):
    return [p[0] - off[0], p[1] - off[1], p[2]]


def rrect2(w, h, c):
    return rrect(0, 0, w, h, c)


def tube(id, bone, mat, tone, p0, p1, w1, w2, hint=None, c=0.0, hole=None, role="body", off=(0, 0), tags=None):
    """Prismatic bar from p0 to p1 (world) with cross section w1 x w2; w1 lies along `hint` (default: perpendicular to the axis in the xy plane)."""
    a = unit(sub(p1, p0))
    h = hint if hint is not None else [a[1], -a[0], 0]
    h = unit([h[i] - dot(h, a) * a[i] for i in range(3)])
    e2 = cross(a, h)
    length = math.sqrt(dot(sub(p1, p0), sub(p1, p0)))
    holes = [rrect(0, 0, hole[0], hole[1], max(0, c - (w1 - hole[0]) / 2))] if hole else None
    prof = rrect(0, 0, w1, w2, c) if c else rect(0, 0, w1, w2)
    if hole and not c:
        holes = [rect(0, 0, hole[0], hole[1])]
    at = [p0[0] - off[0], p0[1] - off[1], p0[2]]
    return ext(id, bone, mat, tone, "xy", prof, 0, length, holes=holes, role=role, at=at, rot=quat_of(h, e2, a), tags=tags)


def lerp(p, q, t):
    return [p[i] + (q[i] - p[i]) * t for i in range(3)]


def circle_pts(cx, cy, r, n=24, a0=0.0):
    return [[cx + r * math.cos(a0 + 2 * math.pi * k / n), cy + r * math.sin(a0 + 2 * math.pi * k / n)] for k in range(n)]


def rod(id, bone, mat, tone, p0, p1, r, n=16, hole=None, role="body", off=(0, 0), tags=None):
    """Round bar (optionally a tube with inner radius `hole`) from p0 to p1 (world)."""
    a = unit(sub(p1, p0))
    h = unit([a[1], -a[0], 0]) if abs(a[2]) < 0.99 else [1, 0, 0]
    e2 = cross(a, h)
    length = math.sqrt(dot(sub(p1, p0), sub(p1, p0)))
    return ext(id, bone, mat, tone, "xy", circle_pts(0, 0, r, n), 0, length, holes=[circle_pts(0, 0, hole, n)] if hole else None, role=role, at=[p0[0] - off[0], p0[1] - off[1], p0[2]], rot=quat_of(h, e2, a), tags=tags)
