"""Python mirror of the Rust section kernel (mesh: ear clipping with hole bridges + Rivara 4T-LE refinement; T6 Saint-Venant, warping, shear functions)."""
import math
import sys

import numpy as np
import scipy.sparse as sps
import scipy.sparse.linalg as spl

sys.path.insert(0, __import__("os").path.dirname(__import__("os").path.abspath(__file__)))


def signed_area(r):
    n = len(r)
    return sum(r[i][0] * r[(i + 1) % n][1] - r[(i + 1) % n][0] * r[i][1] for i in range(n)) / 2.0


def clean(ring):
    pts = list(ring)
    scale = max(max(abs(p[0]) for p in pts), max(abs(p[1]) for p in pts), 1e-300)
    changed = True
    while changed and len(pts) > 3:
        changed = False
        n = len(pts)
        for i in range(n):
            a, b, c = pts[i - 1], pts[i], pts[(i + 1) % n]
            cross = (b[0] - a[0]) * (c[1] - b[1]) - (b[1] - a[1]) * (c[0] - b[0])
            same = math.hypot(b[0] - a[0], b[1] - a[1]) <= 1e-12 * scale
            if same or abs(cross) <= 1e-12 * scale * scale:
                del pts[i]
                changed = True
                break
    return pts


def orient(a, b, c):
    return (b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0])


def seg_cross(p, q, r, s):
    """True when open segments pq and rs touch or cross anywhere except at shared endpoints."""
    def on(a, b, c):
        return min(a[0], b[0]) - 1e-15 <= c[0] <= max(a[0], b[0]) + 1e-15 and min(a[1], b[1]) - 1e-15 <= c[1] <= max(a[1], b[1]) + 1e-15
    o1, o2, o3, o4 = orient(p, q, r), orient(p, q, s), orient(r, s, p), orient(r, s, q)
    if ((o1 > 0 and o2 < 0) or (o1 < 0 and o2 > 0)) and ((o3 > 0 and o4 < 0) or (o3 < 0 and o4 > 0)):
        return True
    return False


def touches(p, q, c, tol):
    """True when point c lies on segment pq (within tol) and is not an endpoint."""
    if (abs(c[0] - p[0]) <= tol and abs(c[1] - p[1]) <= tol) or (abs(c[0] - q[0]) <= tol and abs(c[1] - q[1]) <= tol):
        return False
    d = math.hypot(q[0] - p[0], q[1] - p[1])
    if d == 0:
        return False
    if abs(orient(p, q, c)) / d > tol:
        return False
    t = ((c[0] - p[0]) * (q[0] - p[0]) + (c[1] - p[1]) * (q[1] - p[1])) / (d * d)
    return 0.0 < t < 1.0


def triangulate(pts, outer, holes):
    """outer: CCW index list, holes: CW index lists; returns CCW triangles."""
    scale = max(max(abs(p[0]) for p in pts), max(abs(p[1]) for p in pts))
    tol = 1e-12 * scale
    poly = list(outer)
    order = sorted(range(len(holes)), key=lambda k: -max(pts[i][0] for i in holes[k]))
    pending = [holes[k] for k in order]
    for hi, hole in enumerate(pending):
        rest = pending[hi + 1:]
        edges = [(poly[i], poly[(i + 1) % len(poly)]) for i in range(len(poly))]
        for other in rest:
            edges += [(other[i], other[(i + 1) % len(other)]) for i in range(len(other))]
        edges += [(hole[i], hole[(i + 1) % len(hole)]) for i in range(len(hole))]
        best = None
        for hp in range(len(hole)):
            h = hole[hp]
            for vp in range(len(poly)):
                v = poly[vp]
                if pts[v] == pts[h]:
                    continue
                d = math.hypot(pts[v][0] - pts[h][0], pts[v][1] - pts[h][1])
                if best is not None and d >= best[0]:
                    continue
                ok = True
                for (a, b) in edges:
                    if a == h or b == h or a == v or b == v:
                        if seg_cross(pts[h], pts[v], pts[a], pts[b]) and not (a in (h, v) or b in (h, v)):
                            ok = False
                            break
                        continue
                    if seg_cross(pts[h], pts[v], pts[a], pts[b]):
                        ok = False
                        break
                if ok:
                    for k in range(len(poly)):
                        c = poly[k]
                        if c != v and touches(pts[h], pts[v], pts[c], tol):
                            ok = False
                            break
                if ok:
                    for other in [hole] + rest:
                        for c in other:
                            if c != h and touches(pts[h], pts[v], pts[c], tol):
                                ok = False
                                break
                if ok:
                    best = (d, hp, vp)
        if best is None:
            raise ValueError("hole cannot be bridged")
        _, hp, vp = best
        seq = [hole[(hp + k) % len(hole)] for k in range(len(hole))] + [hole[hp]]
        poly = poly[:vp + 1] + seq + poly[vp:]
    return ear_clip(pts, poly, tol)


def in_tri(a, b, c, p):
    return orient(a, b, p) >= 0 and orient(b, c, p) >= 0 and orient(c, a, p) >= 0


def ear_clip(pts, poly, tol):
    tris = []
    ring = list(poly)
    guard = 0
    while len(ring) > 3:
        n = len(ring)
        found = False
        for i in range(n):
            ia, ib, ic = ring[i - 1], ring[i], ring[(i + 1) % n]
            a, b, c = pts[ia], pts[ib], pts[ic]
            if orient(a, b, c) <= tol * tol * 0 + 1e-300:
                continue
            if orient(a, b, c) <= 0:
                continue
            ok = True
            for k in range(n):
                p = pts[ring[k]]
                if p == a or p == b or p == c:
                    continue
                if in_tri(a, b, c, p):
                    ok = False
                    break
            if ok:
                tris.append((ia, ib, ic))
                del ring[i]
                found = True
                break
        if not found:
            raise ValueError("no ear")
        guard += 1
    if orient(pts[ring[0]], pts[ring[1]], pts[ring[2]]) > 0:
        tris.append((ring[0], ring[1], ring[2]))
    return tris


def edge_len(pts, a, b):
    return math.hypot(pts[a][0] - pts[b][0], pts[a][1] - pts[b][1])


def longest(pts, t):
    best = None
    for k in range(3):
        a, b = t[k], t[(k + 1) % 3]
        key = (edge_len(pts, a, b), min(a, b), max(a, b))
        if best is None or key > best[0]:
            best = (key, k)
    return best[1]


def refine(pts, tris, h, max_tris):
    pts = list(pts)
    tris = list(tris)
    while True:
        marked = set()
        work = []
        for t in tris:
            k = longest(pts, t)
            a, b = t[k], t[(k + 1) % 3]
            if edge_len(pts, a, b) > h:
                e = (min(a, b), max(a, b))
                if e not in marked:
                    marked.add(e)
                    work.append(e)
        if not marked:
            return pts, tris
        edge_tris = {}
        for ti, t in enumerate(tris):
            for k in range(3):
                a, b = t[k], t[(k + 1) % 3]
                edge_tris.setdefault((min(a, b), max(a, b)), []).append(ti)
        while work:
            e = work.pop()
            for ti in edge_tris[e]:
                t = tris[ti]
                k = longest(pts, t)
                a, b = t[k], t[(k + 1) % 3]
                le = (min(a, b), max(a, b))
                if le not in marked:
                    marked.add(le)
                    work.append(le)
        mid = {}
        for e in sorted(marked):
            mid[e] = len(pts)
            pts.append(((pts[e[0]][0] + pts[e[1]][0]) / 2, (pts[e[0]][1] + pts[e[1]][1]) / 2))
        out = []
        for t in tris:
            k = longest(pts, t)
            # rotate so that edge 0 (v0-v1) is the longest, v2 opposite
            v = [t[k], t[(k + 1) % 3], t[(k + 2) % 3]]
            def m(a, b):
                return mid.get((min(a, b), max(a, b)))
            m01, m12, m20 = m(v[0], v[1]), m(v[1], v[2]), m(v[2], v[0])
            if m01 is None:
                out.append(tuple(t))
                continue
            # split v2-m01
            parts = [(v[0], m01, v[2]), (m01, v[1], v[2])]
            res = []
            # first part (v0, m01, v2): edge v2-v0 is its edge 2 (m20)
            if m20 is not None:
                res += [(v[0], m01, m20), (m01, v[2], m20)]
            else:
                res.append(parts[0])
            if m12 is not None:
                res += [(m01, v[1], m12), (m01, m12, v[2])]
            else:
                res.append(parts[1])
            out += res
        tris = out
        if len(tris) > max_tris:
            return pts, tris


def incircle(a, b, c, d):
    ax, ay, bx, by, cx, cy = a[0]-d[0], a[1]-d[1], b[0]-d[0], b[1]-d[1], c[0]-d[0], c[1]-d[1]
    return ((ax*ax+ay*ay)*(bx*cy-cx*by) - (bx*bx+by*by)*(ax*cy-cx*ay) + (cx*cx+cy*cy)*(ax*by-bx*ay))


def flip(pts, tris, passes=200):
    tris = [tuple(t) for t in tris]
    scale = max(max(abs(p[0]) for p in pts), max(abs(p[1]) for p in pts))
    eps = 1e-12 * scale ** 4
    for _ in range(passes):
        owner = {}
        for ti, t in enumerate(tris):
            for k in range(3):
                owner.setdefault((min(t[k], t[(k+1) % 3]), max(t[k], t[(k+1) % 3])), []).append((ti, k))
        changed = False
        dead = set()
        for e in sorted(owner):
            o = owner[e]
            if len(o) != 2 or o[0][0] in dead or o[1][0] in dead:
                continue
            (t1, k1), (t2, k2) = o
            T1, T2 = tris[t1], tris[t2]
            a, b, c = T1[k1], T1[(k1+1) % 3], T1[(k1+2) % 3]
            d = T2[(k2+2) % 3]
            if incircle(pts[a], pts[b], pts[c], pts[d]) > eps and orient(pts[c], pts[d], pts[b]) > 0 and orient(pts[d], pts[c], pts[a]) > 0:
                tris[t1] = (c, a, d)
                tris[t2] = (d, b, c)
                dead.add(t1); dead.add(t2)
                changed = True
        if not changed:
            break
    return tris


def mesh(outer, holes, h_factor=4.0, h=None, max_tris=60000):
    rings = [clean(outer)] + [clean(r) for r in holes]
    if signed_area(rings[0]) < 0:
        rings[0] = rings[0][::-1]
    for i in range(1, len(rings)):
        if signed_area(rings[i]) > 0:
            rings[i] = rings[i][::-1]
    pts = []
    idx = []
    for r in rings:
        idx.append(list(range(len(pts), len(pts) + len(r))))
        pts += r
    area = sum(signed_area(r) for r in rings)
    per = sum(math.hypot(r[(i + 1) % len(r)][0] - r[i][0], r[(i + 1) % len(r)][1] - r[i][1]) for r in rings for i in range(len(r)))
    t_est = 2 * area / per
    hh = h if h else t_est / h_factor
    tris = triangulate(pts, idx[0], idx[1:])
    tris = flip(pts, tris)
    pts, tris = refine(pts, tris, hh, max_tris)
    tris = flip(pts, tris)
    return pts, tris, rings


GP = []
for a, b, w in [(0.445948490915965, 0.108103018168070, 0.223381589678011), (0.091576213509771, 0.816847572980459, 0.109951743655322)]:
    for L in [(a, a, b), (a, b, a), (b, a, a)]:
        GP.append((L, w))


def shape(L):
    l0, l1, l2 = L
    N = np.array([l0 * (2 * l0 - 1), l1 * (2 * l1 - 1), l2 * (2 * l2 - 1), 4 * l0 * l1, 4 * l1 * l2, 4 * l2 * l0])
    dN = np.array([  # d/dL0, d/dL1, d/dL2
        [4 * l0 - 1, 0, 0], [0, 4 * l1 - 1, 0], [0, 0, 4 * l2 - 1],
        [4 * l1, 4 * l0, 0], [0, 4 * l2, 4 * l1], [4 * l2, 0, 4 * l0]])
    return N, dN


def analyse(outer, holes, nu=0.0, h_factor=4.0, h=None):
    pts, tris, rings = mesh(outer, holes, h_factor, h)
    nv = len(pts)
    edge_node = {}
    coords = [list(p) for p in pts]
    elems = []
    for t in tris:
        nodes = list(t)
        for (a, b) in [(t[0], t[1]), (t[1], t[2]), (t[2], t[0])]:
            e = (min(a, b), max(a, b))
            if e not in edge_node:
                edge_node[e] = len(coords)
                coords.append([(pts[a][0] + pts[b][0]) / 2, (pts[a][1] + pts[b][1]) / 2])
            nodes.append(edge_node[e])
        elems.append(nodes)
    xy = np.array(coords)
    n = len(coords)
    # geometry (exact)
    A = 0.0
    sx = sy = 0.0
    for r in rings:
        m = len(r)
        for i in range(m):
            x0, y0 = r[i]
            x1, y1 = r[(i + 1) % m]
            c = x0 * y1 - x1 * y0
            A += c / 2
            sx += (x0 + x1) * c / 6
            sy += (y0 + y1) * c / 6
    cx, cy = sx / A, sy / A
    xy = xy - np.array([cx, cy])
    # element integrals (area-weighted second moments)
    ixx = iyy = ixy = 0.0
    data = []
    for nodes in elems:
        X = xy[nodes[:3]]
        d = (X[1][0] - X[0][0]) * (X[2][1] - X[0][1]) - (X[2][0] - X[0][0]) * (X[1][1] - X[0][1])
        area = d / 2
        b = np.array([X[1][1] - X[2][1], X[2][1] - X[0][1], X[0][1] - X[1][1]]) / d
        c = np.array([X[2][0] - X[1][0], X[0][0] - X[2][0], X[1][0] - X[0][0]]) / d
        pg = []
        for L, w in GP:
            N, dN = shape(L)
            gx = np.array(L) @ X
            B = np.vstack([dN @ b, dN @ c])
            pg.append((w * area, N, B, gx[0], gx[1]))
            ixx += w * area * gx[1] ** 2
            iyy += w * area * gx[0] ** 2
            ixy += w * area * gx[0] * gx[1]
        data.append(pg)
    rows, cols, vals = [], [], []
    f = np.zeros(n)
    fpsi = np.zeros(n)
    fphi = np.zeros(n)
    for nodes, pg in zip(elems, data):
        ke = np.zeros((6, 6))
        fe = np.zeros(6)
        fp = np.zeros(6)
        fh = np.zeros(6)
        for wt, N, B, x, y in pg:
            ke += wt * B.T @ B
            fe += wt * B.T @ np.array([y, -x])
            r = x * x - y * y
            q = 2 * x * y
            d1, d2 = ixx * r - ixy * q, ixy * r + ixx * q
            h1, h2 = -ixy * r + iyy * q, -iyy * r - ixy * q
            fp += wt * (nu / 2 * B.T @ np.array([d1, d2]) + 2 * (1 + nu) * N * (ixx * x - ixy * y))
            fh += wt * (nu / 2 * B.T @ np.array([h1, h2]) + 2 * (1 + nu) * N * (iyy * y - ixy * x))
        for i in range(6):
            f[nodes[i]] += fe[i]
            fpsi[nodes[i]] += fp[i]
            fphi[nodes[i]] += fh[i]
            for j in range(6):
                rows.append(nodes[i]); cols.append(nodes[j]); vals.append(ke[i, j])
    K = sps.csr_matrix((vals, (rows, cols)), shape=(n, n)).tolil()
    K[0, :] = 0; K[:, 0] = 0; K[0, 0] = 1.0
    K = K.tocsc()
    lu = spl.splu(K)
    def solve(rhs):
        r = rhs.copy(); r[0] = 0.0
        return lu.solve(r)
    om = solve(f)
    psi = solve(fpsi)
    phi = solve(fphi)
    J = ixx + iyy - om @ f
    s_xint = s_yint = q_om = i_om = i_xom = i_yom = 0.0
    kx = ky = kxy = 0.0
    for nodes, pg in zip(elems, data):
        for wt, N, B, x, y in pg:
            o = N @ om[nodes]
            s_xint += wt * (iyy * x + ixy * y) * (x * x + y * y)
            s_yint += wt * (ixx * y + ixy * x) * (x * x + y * y)
            q_om += wt * o
            i_om += wt * o * o
            i_xom += wt * x * o
            i_yom += wt * y * o
            r = x * x - y * y
            q = 2 * x * y
            d1, d2 = ixx * r - ixy * q, ixy * r + ixx * q
            h1, h2 = -ixy * r + iyy * q, -iyy * r - ixy * q
            bp = B @ psi[nodes] - nu / 2 * np.array([d1, d2])
            bh = B @ phi[nodes] - nu / 2 * np.array([h1, h2])
            kx += wt * bp @ bp
            ky += wt * bh @ bh
            kxy += wt * bp @ bh
    ds = 2 * (1 + nu) * (ixx * iyy - ixy ** 2)
    x_se = (nu / 2 * s_xint - f @ phi) / ds
    y_se = (nu / 2 * s_yint + f @ psi) / ds
    gamma = i_om - q_om ** 2 / A - y_se * i_xom + x_se * i_yom
    return dict(area=A, c=(cx, cy), ic=(ixx, iyy, ixy), j=J, sc=(x_se + cx, y_se + cy), gamma=gamma, a_s=(ds ** 2 / kx, ds ** 2 / ky), tris=len(tris), nodes=n)
