"""Author 27 helpers on top of author-10-lib/-human/-standing (exec'd): tube centre lines from drawn outlines, capsule chains, honest shells, decisions."""
import os, sys, json, math, statistics
HERE = os.path.dirname(os.path.abspath(__file__)) if '__file__' in globals() else os.getcwd()
if 'Fig' not in globals():
    exec(open(os.path.join(HERE, 'author-10-lib.py')).read())
if 'Human' not in globals():
    exec(open(os.path.join(HERE, 'author-10-human.py')).read())
if 'standing_body' not in globals():
    exec(open(os.path.join(HERE, 'author-10-standing.py')).read())


def dp(pts, tol):
    if len(pts) < 3:
        return list(pts)
    a, b = pts[0], pts[-1]
    dx, dy = b[0] - a[0], b[1] - a[1]
    L = math.hypot(dx, dy) or 1e-9
    wi, wd = -1, -1.0
    for i in range(1, len(pts) - 1):
        d = abs((pts[i][0] - a[0]) * dy - (pts[i][1] - a[1]) * dx) / L
        if d > wd:
            wi, wd = i, d
    if wd <= tol:
        return [a, b]
    return dp(pts[:wi + 1], tol)[:-1] + dp(pts[wi:], tol)


def centreline(T, shapes, n=40, trim=0.6, along=None, xr=None, yr=None):
    """Centre points and half widths of a tube-like closed outline, sliced across its dominant direction."""
    pts = [q for s in shapes for r in s['rings'] for q in r]
    xs = [p[0] for p in pts]
    ys = [p[1] for p in pts]
    x0, x1, y0, y1 = min(xs), max(xs), min(ys), max(ys)
    if xr:
        x0, x1 = max(x0, xr[0]), min(x1, xr[1])
    if yr:
        y0, y1 = max(y0, yr[0]), min(y1, yr[1])
    vertical = along == 'y' or (along is None and (y1 - y0) > (x1 - x0))
    out = []
    lo, hi = (y0, y1) if vertical else (x0, x1)
    est = 0.012
    a, b = lo + trim * est, hi - trim * est
    for i in range(n):
        t = a + (b - a) * i / (n - 1)
        if vertical:
            iv = T.intervals(shapes, (0.0, t), (1.0, 0.0))
        else:
            iv = T.intervals(shapes, (t, 0.0), (0.0, 1.0))
        if not iv:
            continue
        m = (iv[0][0] + iv[-1][1]) / 2
        w = iv[-1][1] - iv[0][0]
        out.append(((m, t) if vertical else (t, m), w, vertical))
    cs = [o[0] for o in out]
    ws = []
    for i, o in enumerate(out):
        j0, j1 = max(0, i - 1), min(len(out) - 1, i + 1)
        tx, ty = cs[j1][0] - cs[j0][0], cs[j1][1] - cs[j0][1]
        L = math.hypot(tx, ty) or 1e-9
        c = abs(ty) / L if vertical else abs(tx) / L
        ws.append(o[1] * max(0.2, c))
    return cs, ws


def chain_from(T, shapes, tol=0.006, n=40, r=None, trim=0.6, **kw):
    cs, ws = centreline(T, shapes, n=n, trim=trim, **kw)
    rr = r if r else statistics.median(ws) / 2
    return dp(cs, tol), rr


def capsule_chain(fig, idp, bone, mat, tone, pts_world, w, r, role='body', lst='body', r_end=None, **kw):
    """One capsule per segment of a world-space (x, y) polyline at lateral local w, radius r (round tube)."""
    loc = [fig.to_local(bone, [p[0], p[1], fig.fr(bone)['o'][2]]) for p in pts_world]
    out = []
    for i in range(len(loc) - 1):
        a, b = loc[i], loc[i + 1]
        if math.dist(a, b) < 1e-4:
            continue
        out.append(fig.capsule(idp if len(loc) == 2 else f'{idp}-{i}', bone, mat, tone, [a[0], a[1], w], [b[0], b[1], w], r, r, role=role, lst=lst, **kw))
    return out


def slab_loft(f, idp, bone, mat, tone, e, fr, pl, nst=4, exp=3.4, minr=0.0, role='body', rotvert=False, **kw):
    """A pillow-like slab (cushion, sling) lofted along u: depth from the elevation outline, width from the front/plan."""
    H = Human(f, None)
    return H.loft(idp, bone, mat, tone, e, fr, pl, nst=nst, exp=exp, role=role, planfit=0, minr=minr, roles=('body', 'detail'), **kw)


def ring(fig, idp, bone, mat, tone, zlo, zhi, r_in, r_out, role='body', lst='body', **kw):
    prof = [[zlo, r_in], [zhi, r_in], [zhi, r_out], [zlo, r_out], [zlo, r_in]] if r_in > 0 else [[zlo, 0.0], [zhi, 0.0], [zhi, r_out], [zlo, r_out]]
    return fig.revolve(idp, bone, mat, tone, prof, [0, 0, 1], [0, 0, 0], role=role, lst=lst, **kw)
