"""Python mirror of the Rust profile builders, compared with the sectionproperties library polygons and (via proto.py) with the oracle output."""
import math, sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from math import pi, cos, sin

def arc(c, r, theta, n, ccw):
    if r == 0:
        return [tuple(c)]
    s = 1 if ccw else -1
    return [(c[0] + r * cos(theta + s * i / max(1, n - 1) * pi / 2), c[1] + r * sin(theta + s * i / max(1, n - 1) * pi / 2)) for i in range(n)]

def i_section(d, b, tf, tw, r, n):
    xl, xr = b/2 - tw/2, b/2 + tw/2
    p = [(0, 0), (b, 0), (b, tf)]
    p += arc((xr + r, tf + r), r, 1.5*pi, n, False)
    p += arc((xr + r, d - tf - r), r, pi, n, False)
    p += [(b, d - tf), (b, d), (0, d), (0, d - tf)]
    p += arc((xl - r, d - tf - r), r, pi/2, n, False)
    p += arc((xl - r, tf + r), r, 0, n, False)
    p += [(0, tf)]
    return p, []

def mono_i(d, top, bottom, tw, r, n):
    (bt, tft), (bb, tfb) = top, bottom
    c = max(bt, bb)/2
    xl, xr = c - tw/2, c + tw/2
    p = [(c - bb/2, 0), (c + bb/2, 0), (c + bb/2, tfb)]
    p += arc((xr + r, tfb + r), r, 1.5*pi, n, False)
    p += arc((xr + r, d - tft - r), r, pi, n, False)
    p += [(c + bt/2, d - tft), (c + bt/2, d), (c - bt/2, d), (c - bt/2, d - tft)]
    p += arc((xl - r, d - tft - r), r, pi/2, n, False)
    p += arc((xl - r, tfb + r), r, 0, n, False)
    p += [(c - bb/2, tfb)]
    return p, []

def channel(d, b, tf, tw, r, n):
    p = [(0, 0), (b, 0), (b, tf)]
    p += arc((tw + r, tf + r), r, 1.5*pi, n, False)
    p += arc((tw + r, d - tf - r), r, pi, n, False)
    p += [(b, d - tf), (b, d), (0, d)]
    return p, []

def tee(d, b, tf, tw, r, n):
    xl, xr = b/2 - tw/2, b/2 + tw/2
    p = [(xl, 0), (xr, 0)]
    p += arc((xr + r, d - tf - r), r, pi, n, False)
    p += [(b, d - tf), (b, d), (0, d), (0, d - tf)]
    p += arc((xl - r, d - tf - r), r, pi/2, n, False)
    return p, []

def angle(d, b, t, rr, rt, n):
    full = abs(t - rt) < 1e-12 * abs(t)
    p = [(0, 0)]
    if not full:
        p.append((b, 0))
    p += arc((b - rt, t - rt), rt, 0, n, True)
    p += arc((t + rr, t + rr), rr, 1.5*pi, n, False)
    p += arc((t - rt, d - rt), rt, 0, n, True)
    if not full:
        p.append((0, d))
    return p, []

def rhs(d, b, t, ro, n):
    ri = max(ro - t, 0)
    o = arc((ro, ro), ro, pi, n, True) + arc((b - ro, ro), ro, 1.5*pi, n, True) + arc((b - ro, d - ro), ro, 0, n, True) + arc((ro, d - ro), ro, pi/2, n, True)
    i = arc((t + ri, t + ri), ri, pi, n, True) + arc((b - t - ri, t + ri), ri, 1.5*pi, n, True) + arc((b - t - ri, d - t - ri), ri, 0, n, True) + arc((t + ri, d - t - ri), ri, pi/2, n, True)
    return o, [i]

def ehs(dx, dy, t, n):
    ring = lambda ax, ay: [(ax * cos(2*pi*i/n), ay * sin(2*pi*i/n)) for i in range(n)]
    return ring(dx/2, dy/2), [ring(dx/2 - t, dy/2 - t)]

LIB = {
    "ipe300": lambda: i_section(0.3, 0.15, 0.0107, 0.0071, 0.015, 16),
    "pfc200": lambda: channel(0.2, 0.075, 0.0115, 0.0085, 0.0115, 16),
    "l100x10": lambda: angle(0.1, 0.1, 0.01, 0.012, 0.006, 16),
    "t150": lambda: tee(0.15, 0.15, 0.0107, 0.0071, 0.012, 16),
    "plate-girder": lambda: mono_i(0.8, (0.3, 0.02), (0.4, 0.03), 0.012, 0.0, 1),
    "rhs-200x100x8": lambda: rhs(0.2, 0.1, 0.008, 0.016, 16),
    "ehs-300x200x10": lambda: ehs(0.3, 0.2, 0.01, 128),
}
