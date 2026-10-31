"""Hand-parameterised shape forms shared by the plant scripts (blobs, fans, scalloped masses)."""
import sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib import *


def blob(cx, cy, rx, ry, rot, m, a, ph, n=None):
    n = n or max(14, 4 * m)
    out = []
    cr, sr = cos(radians(rot)), sin(radians(rot))
    for k in range(n):
        t = 2 * pi * k / n
        r = 1 + a * cos(m * t + radians(ph))
        x, y = rx * r * cos(t), ry * r * sin(t)
        out.append((cx + x * cr - y * sr, cy + x * sr + y * cr))
    return out


def fan(base, ang, L, n, spread, w, taper=0.2):
    """Union outline of n lanceolate leaves radiating from base (closed loop, sharp tips and valleys)."""
    bx, by = base
    mid = (n - 1) / 2
    loop = [(bx, by, "s")]
    leaves = []
    for i in range(n):
        th = ang + (spread * (i / (n - 1) - 0.5) if n > 1 else 0)
        Li = L * (1 - taper * abs(i - mid) / max(mid, 1))
        leaves.append((th, Li))
    delta = spread / (n - 1) if n > 1 else 0
    for i, (th, Li) in enumerate(leaves):
        c, s = cos(radians(th)), sin(radians(th))
        nx, ny = -s, c
        r = (bx + c * Li * 0.45 + nx * (-w / 2), by + s * Li * 0.45 + ny * (-w / 2))
        l = (bx + c * Li * 0.45 + nx * (w / 2), by + s * Li * 0.45 + ny * (w / 2))
        tip = (bx + c * Li, by + s * Li)
        loop += [r, (tip[0], tip[1], "s"), l]
        if i < n - 1:
            thv = th + delta / 2
            rv = max(0.32 * Li, (w * 0.8) / (2 * sin(radians(max(delta, 8)) / 2)))
            loop.append((bx + cos(radians(thv)) * rv, by + sin(radians(thv)) * rv, "s"))
    return loop


def pol(c, ang, d):
    return (c[0] + cos(radians(ang)) * d, c[1] + sin(radians(ang)) * d)


def ragged(cx, cy, rx, ry, rot, m, depth, ph, droop=0.3):
    """Star-like foliage mass: m tips (sharp) and m valleys; lower tips longer (hanging leaves)."""
    out = []
    cr, sr = cos(radians(rot)), sin(radians(rot))
    for k in range(2 * m):
        t = 2 * pi * k / (2 * m) + radians(ph)
        tip = k % 2 == 0
        f = (1 + droop * max(0, -sin(t - radians(rot)))) if tip else (1 - depth)
        x, y = rx * f * cos(t), ry * f * sin(t)
        out.append((cx + x * cr - y * sr, cy + x * sr + y * cr, "s") if tip else (cx + x * cr - y * sr, cy + x * sr + y * cr, "s"))
    return out


def plume(base, tip, width, n=8, widest=0.45, bend=0.0, teeth=0.22, ph=0):
    """Serrated lanceolate mass from base to tip: leaf bunch whose margin is cut by leaf tips (n segments per side)."""
    bx, by = base
    tx, ty = tip
    L = hypot(tx - bx, ty - by)
    ux, uy = (tx - bx) / L, (ty - by) / L
    nx, ny = -uy, ux
    p = math.log(0.5) / math.log(widest)
    left, right = [], []
    for i in range(1, n):
        t = i / n
        hw = width / 2 * (sin(pi * t ** p)) ** 0.8
        off = bend * L * sin(pi * t)
        tooth = 1 + teeth * (1 if (i + ph) % 2 == 0 else -0.6)
        cx, cy = bx + ux * L * t + nx * off, by + uy * L * t + ny * off
        left.append((cx + nx * hw * tooth, cy + ny * hw * tooth, "s"))
        tooth_r = 1 + teeth * (1 if (i + ph + 1) % 2 == 0 else -0.6)
        right.append((cx - nx * hw * tooth_r, cy - ny * hw * tooth_r, "s"))
    return [(bx, by, "s")] + left + [(tx, ty, "s")] + right[::-1]


def fan_leaves(base, ang, L, n, spread, w, taper=0.2, bend=0.06):
    """The n separate lanceolate leaf outlines of a fan (to be merged by the union step)."""
    mid = (n - 1) / 2
    out = []
    for i in range(n):
        th = ang + (spread * (i / (n - 1) - 0.5) if n > 1 else 0)
        Li = L * (1 - taper * abs(i - mid) / max(mid, 1))
        tip = (base[0] + cos(radians(th)) * Li, base[1] + sin(radians(th)) * Li)
        side = 1 if (i - mid) >= 0 else -1
        out.append(lancet(base, tip, w, 6, 0.4, bend * side))
    return out


def spine_pt(spine, t):
    """Point and direction (deg) at arclength fraction t of a polyline spine."""
    p, a = along(spine, t)
    return p, a


def leaf_outline(spine, t0, t1, hw_fn, n=10, teeth=0.0, flat_ends=True, tooth_every=1):
    """Closed polygon of the part [t0,t1] of a leaf whose centre line is `spine`; hw_fn(t) = half width in metres."""
    left, right = [], []
    for i in range(n + 1):
        t = t0 + (t1 - t0) * i / n
        p, a = along(spine, t)
        nx, ny = -sin(radians(a)), cos(radians(a))
        h = hw_fn(t)
        if teeth and 0 < i < n and h > 0.01:
            k = 1 + (teeth if i % 2 == 0 else -teeth)
            h *= k
        left.append((p[0] + nx * h, p[1] + ny * h))
        right.append((p[0] - nx * h, p[1] - ny * h))
    return left + right[::-1]
