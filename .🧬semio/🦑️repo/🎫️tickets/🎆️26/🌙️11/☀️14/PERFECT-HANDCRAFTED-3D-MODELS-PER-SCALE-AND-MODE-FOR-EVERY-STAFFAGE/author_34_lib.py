import importlib.util, json, math, os
T = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("a17", os.path.join(T, "author-17-lib.py"))
a17 = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(a17)
from importlib.machinery import SourceFileLoader
globals().update({k: getattr(a17, k) for k in dir(a17) if not k.startswith("_")})
CAT = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🧱️components/%s/🔣️.json"


def asset(slug):
    return json.load(open(CAT % slug))["asset"]


def quat_y(deg):
    return qaxis([0, 1, 0], deg)


def helix(r, y0, rise_per_rad, a0, n, da, sign=1):
    pts = []
    for i in range(n):
        a = a0 + sign * i * da
        pts.append([r6(r * math.cos(a)), r6(y0 + rise_per_rad * (i * da)), r6(r * math.sin(a))])
    return pts


def tube_loft(id, bone, mat, tone, pts, r, role="body", slender=False, tags=None, caps=True):
    secs = [{"at": vec(p), "rx": r6(r), "ry": r6(r)} for p in pts]
    d = loft(id, bone, mat, tone, secs, role=role, capStart=caps, capEnd=caps, tags=tags)
    if slender:
        d["slender"] = True
    return d


def signed_area(p):
    return 0.5 * sum(p[i][0] * p[(i + 1) % len(p)][1] - p[(i + 1) % len(p)][0] * p[i][1] for i in range(len(p)))


def inset_convex(poly, t):
    p = poly if signed_area(poly) > 0 else poly[::-1]
    cur = [[-1e3, -1e3], [1e3, -1e3], [1e3, 1e3], [-1e3, 1e3]]
    for i in range(len(p)):
        a, b = p[i], p[(i + 1) % len(p)]
        dx, dy = b[0] - a[0], b[1] - a[1]
        n = math.hypot(dx, dy)
        nx, ny = -dy / n, dx / n
        c = (a[0] + nx * t, a[1] + ny * t)
        side = lambda q: (q[0] - c[0]) * nx + (q[1] - c[1]) * ny
        out = []
        for j in range(len(cur)):
            s, e = cur[j], cur[(j + 1) % len(cur)]
            ds, de = side(s), side(e)
            if ds >= 0:
                out.append(s)
            if (ds >= 0) != (de >= 0):
                k = ds / (ds - de)
                out.append([s[0] + (e[0] - s[0]) * k, s[1] + (e[1] - s[1]) * k])
        cur = out
    return [[r6(x), r6(y)] for x, y in cur]


def sector(k, n_arc, r_out=0.8, r_in=0.081, step=22.5):
    t0, t1 = math.radians((k - 1) * step), math.radians(k * step)
    pts = []
    for i in range(n_arc + 1):
        t = t0 + (t1 - t0) * i / n_arc
        pts.append([r_out * math.sin(t), r_out * math.cos(t)])
    pts.append([r_in * math.sin(t1), r_in * math.cos(t1)])
    pts.append([r_in * math.sin(t0), r_in * math.cos(t0)])
    return [[r6(x), r6(y)] for x, y in pts]
