from author_37_lib import *
import math

HD = "hdpe-plastic"
BIN = Ctx("body", HD, "bin-hdpe", "body", 0.42)
LID = Ctx("lid", HD, "lid-hdpe", "body", 0.55)
ST = Ctx("body", HD, None, "detail", 0.15)
P0, P1, P2, P3 = (0.2, 0.025), (0.215, 0.3), (0.235, 0.7), (0.248, 0.955)
SHELL_PTS = bezier(P0, P1, P2, P3, 200)


def hw(y):
    """Half width of the drawn shell (x of the bezier at height y)."""
    for (x0, y0), (x1, y1) in zip(SHELL_PTS, SHELL_PTS[1:]):
        if y0 <= y <= y1:
            return x0 + (x1 - x0) * (y - y0) / (y1 - y0)
    return SHELL_PTS[0][0] if y < 0.025 else SHELL_PTS[-1][0]


def rr(hz, hx, r, n=6, cz=0.0, cx=0.0):
    return rrect(cz - hz, cx - hx, cz + hz, cx + hx, r, n)


def tube(sid, ctx, y0, y1, ho0, ho1, hi0, hr0, hr1=None, plan=None, **kw):
    """Hollow rounded-square tube (profile (w, u)) from y0 to y1, outer half ho0 -> ho1, inner half hi0 at y0 and tapering with the outer; corner radius hr0 (scaled)."""
    s = ho1 / ho0
    outer = rr(ho0, ho0, hr0)
    hole = rr(hi0, hi0, max(hr0 - (ho0 - hi0), 0.002))
    return ext(sid, ctx, outer, "zx", y0, y1, holes=[hole], taper=(1, s), **kw)


def crown_path(n=10):
    """Drawn lid outline from the front bottom corner over the crown to the back bottom corner, (u, v)."""
    right = bezier((0.52, 0.006), (0.52, 0.03), (0.45, 0.04), (0.4, 0.038), n)
    left = bezier((0.12, 0.038), (0.05, 0.04), (0.0, 0.03), (0.0, 0.006), n)
    return [(0.52, 0.0)] + right + left + [(0.0, 0.0)]


def crown_profile():
    return [list(p) for p in crown_path()]


def dish_profile(inset=0.004, bottom=0.0):
    """Open-bottom dish: crown outline out, the crown shifted inward by `inset` back."""
    o = crown_path()
    inner = []
    for i in range(1, len(o) - 1):
        a, b = o[i - 1], o[i + 1]
        tx, ty = b[0] - a[0], b[1] - a[1]
        l = math.hypot(tx, ty) or 1
        nx, ny = -ty / l, tx / l
        if nx * (0.26 - o[i][0]) + ny * (0.0 - o[i][1]) < 0:
            nx, ny = -nx, -ny
        inner.append([o[i][0] + nx * inset, o[i][1] + ny * inset])
    inner = [[min(max(x, inset), 0.52 - inset), min(y, 0.038 - inset)] for x, y in inner]
    o = [[x, (bottom if y == 0.0 else y)] for x, y in o]
    if bottom < 0:
        o = [[0.52, bottom], [0.52, 0.0]] + [list(p) for p in o[1:-1]] + [[0.0, 0.0], [0.0, bottom]]
    return [list(p) for p in o] + [[inset, bottom]] + list(reversed(inner)) + [[0.52 - inset, bottom]]


def handle_profile(n=8):
    arc = bezier((0.38, 0.038), (0.39, 0.058), (0.47, 0.058), (0.48, 0.038), n)
    return [[0.38, 0.036]] + [list(p) for p in arc] + [[0.48, 0.036]]
