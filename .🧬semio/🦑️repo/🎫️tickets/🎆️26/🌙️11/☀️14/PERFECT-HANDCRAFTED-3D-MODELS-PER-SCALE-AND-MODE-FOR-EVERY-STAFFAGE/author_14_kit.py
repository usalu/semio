"""Vehicle kit of author 14: vertical lofts (horizontal slices), fitted hulls, glass panes, wheel sets per level of detail."""
import math

from author_14_lib import *  # noqa: F401,F403
from author_14_lib import L, V, C, B, E, X, qbasis, r

HSLICE = [r(c) for c in qbasis([1, 0, 0], [0, 0, -1], [0, 1, 0])]
XSLICE = [r(c) for c in qbasis([0, 0, 1], [0, -1, 0], [1, 0, 0])]


def vloft(id, bone, material, role, tone, slices, e=6.0, caps=(True, True), **kw):
    """Loft through horizontal slices (y, x0, x1, halfwidth[, exponent[, zc]]): rx along x, ry along z, so elevation and front outline are exact."""
    secs = []
    for s in slices:
        y, x0, x1, hw = s[0], s[1], s[2], s[3]
        ex = s[4] if len(s) > 4 and s[4] is not None else e
        zc = s[5] if len(s) > 5 else 0.0
        secs.append({"at": [r((x0 + x1) / 2), r(y), r(zc)], "rx": r((x1 - x0) / 2), "ry": r(hw), "exponent": ex, "rotation": HSLICE})
    return base_solid_loft(id, bone, material, role, tone, secs, caps, **kw)


def base_solid_loft(id, bone, material, role, tone, secs, caps, **kw):
    from author_14_base import solid
    return solid(id, "loft", bone, material, role, tone, None, sections=secs, capStart=caps[0], capEnd=caps[1], **kw)


def xloft(id, bone, material, role, tone, sts, e=6.0, **kw):
    """Loft along x through vertical slices (x, y0, y1, halfwidth[, exponent[, zc]])."""
    secs = []
    for s in sts:
        x, y0, y1, hw = s[0], s[1], s[2], s[3]
        ex = s[4] if len(s) > 4 and s[4] is not None else e
        zc = s[5] if len(s) > 5 else 0.0
        secs.append({"at": [r(x), r((y0 + y1) / 2), r(zc)], "rx": r(hw), "ry": r((y1 - y0) / 2), "exponent": ex, "rotation": [r(c) for c in qbasis([0, 0, 1], [0, -1, 0], [1, 0, 0])]})
    return base_solid_loft(id, bone, material, role, tone, secs, (True, True), **kw)


def fit_x(S, scale, xs, esel, psel, ytop=None, ybot=None, hw_cap=None, hw_k=1.0):
    """Stations (x, y0, y1, halfwidth) read from the drawing at the given x: elevation union (esel) and plan half-width (psel); ytop/ybot override per x via callables."""
    out = []
    for x in xs:
        e = S.col(scale, "elevation", x, esel)
        p = S.col(scale, "plan", x, psel)
        if not e or not p:
            continue
        y0 = min(a for a, b in e)
        y1 = max(b for a, b in e)
        if ybot:
            y0 = ybot(x, y0)
        if ytop:
            y1 = ytop(x, y1)
        hw = max(max(abs(a), abs(b)) for a, b in p) * hw_k
        if hw_cap:
            hw = min(hw, hw_cap)
        out.append((round(x, 4), round(y0, 4), round(y1, 4), round(hw, 4)))
    return out


def fit_y(S, scale, ys, esel, fsel, x_override=None, hw_k=1.0):
    """Slices (y, x0, x1, halfwidth) read from the drawing at the given y: elevation row (esel) and front half-width (fsel)."""
    out = []
    for y in ys:
        e = S.row(scale, "elevation", y, esel)
        f = S.row(scale, "front", y, fsel)
        if not e or not f:
            continue
        x0 = min(a for a, b in e)
        x1 = max(b for a, b in e)
        if x_override:
            x0, x1 = x_override(y, x0, x1)
        hw = max(max(abs(a), abs(b)) for a, b in f) * hw_k
        out.append((round(y, 4), round(x0, 4), round(x1, 4), round(hw, 4)))
    return out


def rdp(rows, tol, keys):
    """Douglas-Peucker over rows (first element is the abscissa): keeps rows whose interpolated error on any of `keys` exceeds tol."""
    n = len(rows)
    keep = [False] * n
    keep[0] = keep[-1] = True
    stack = [(0, n - 1)]
    while stack:
        a, b = stack.pop()
        worst, wi = 0.0, -1
        for i in range(a + 1, b):
            t = (rows[i][0] - rows[a][0]) / ((rows[b][0] - rows[a][0]) or 1)
            err = max(abs(rows[i][k] - (rows[a][k] + t * (rows[b][k] - rows[a][k]))) for k in keys)
            if err > worst:
                worst, wi = err, i
        if worst > tol and wi >= 0:
            keep[wi] = True
            stack.append((a, wi))
            stack.append((wi, b))
    return [row for row, k in zip(rows, keep) if k]


def auto_x(S, scale, x0, x1, esel, psel, tol, step=0.01, ytop=None, ybot=None, hw_cap=None, hw_k=1.0):
    n = int(round((x1 - x0) / step))
    xs = [x0 + (x1 - x0) * i / n for i in range(n + 1)]
    rows = fit_x(S, scale, xs, esel, psel, ytop=ytop, ybot=ybot, hw_cap=hw_cap, hw_k=hw_k)
    return rdp(rows, tol, (1, 2, 3))


def auto_y(S, scale, y0, y1, esel, fsel, tol, step=0.01, hw_k=1.0):
    n = int(round((y1 - y0) / step))
    ys = [y0 + (y1 - y0) * i / n for i in range(n + 1)]
    rows = fit_y(S, scale, ys, esel, fsel, hw_k=hw_k)
    return rdp(rows, tol, (1, 2, 3))


def panel_loft(id, S, scale, shape_id, bone, us, width, material, role, tone, e=12, min_ry=0.01, view="elevation", **kw):
    """Loft along the bone axis u through the drawn outline `shape_id` (world ring of the elevation, read in the bone frame): section at each u spans the outline's v-range, `width(u)` is the half-width on w."""
    from author_14_base import loc
    ring = [loc(bone, (p[0], p[1], 0)) for p in S.shapes(scale, view, lambda s: s["id"] == shape_id)[0]["rings"][0]]
    secs = []
    for u in us:
        vs = []
        n = len(ring)
        for i in range(n):
            a, b = ring[i - 1], ring[i]
            if (a[0] <= u) != (b[0] <= u):
                t = (u - a[0]) / (b[0] - a[0])
                vs.append(a[1] + t * (b[1] - a[1]))
        if len(vs) < 2:
            continue
        lo, hi = min(vs), max(vs)
        if hi - lo < 2 * min_ry:
            c = (lo + hi) / 2
            lo, hi = c - min_ry, c + min_ry
        secs.append(((round(u, 4), round((lo + hi) / 2, 4), 0), width(u), round((hi - lo) / 2, 4), e))
    return L(id, bone, material, role, tone, secs, **kw)
