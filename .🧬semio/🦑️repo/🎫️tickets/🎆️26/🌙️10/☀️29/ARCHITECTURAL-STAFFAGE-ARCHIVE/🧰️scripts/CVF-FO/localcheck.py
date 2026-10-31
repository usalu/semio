"""Local approximation of the official suitability measurement (pure Python, no dependencies): visible-outline gaps (hidden lines removed behind higher-z occluding filled shapes),
feature width of filled shapes (EDT opening, same criterion as the representation module) and white slivers. Used only to iterate quickly while the bun capacity queue is busy;
the official check CLI remains authoritative.  usage: localcheck.py asset-id [scales] [--poses]"""
import json, math, sys, glob
sys.path.insert(0, __file__.rsplit('/', 1)[0])
from lib import S, Rig, apply, outline

MIN_STROKE, MIN_GAP, MIN_FEAT = 0.1, 0.15, 0.2
OCC = ('body', 'detail', 'texture')
INF = 1e20


def find(asset_id):
    for f in glob.glob(S + '/🗂️catalog/*/' + asset_id + '/🔣️.json'):
        return f


def paper_shapes(a, rep, rig, k):
    out = []
    pens = rep['pens']
    for order, s in enumerate(sorted(rep['geometry']['elevation'], key=lambda s: s['z'])):
        pen = s.get('penMm', pens.get(s['role']))
        if pen is None or s['role'] == 'ground':
            continue
        m = rig.world(s['bone'])
        rings = []
        closed = None
        for pts, c in outline(s, 24):
            rings.append([(apply(m, x, y)[0] * k, -apply(m, x, y)[1] * k) for x, y in pts])
            closed = c if closed is None else closed
        out.append(dict(s=s, rings=rings, closed=bool(closed), pen=pen, order=order, fill=bool(s['filled']) and bool(closed) and s['role'] in OCC))
    return out


def inside(pt, rings):
    x, y = pt
    c = False
    for ring in rings:
        n = len(ring)
        for i in range(n):
            a, b = ring[i], ring[i - 1]
            if (a[1] > y) != (b[1] > y) and x < a[0] + (y - a[1]) / (b[1] - a[1]) * (b[0] - a[0]):
                c = not c
    return c


def bbox(rings):
    xs = [p[0] for r in rings for p in r]
    ys = [p[1] for r in rings for p in r]
    return min(xs), min(ys), max(xs), max(ys)


def joined(p, q, parents):
    sp, sq = p['s'], q['s']
    r1 = sp.get('region')
    return r1 is not None and r1 == sq.get('region') and (sp['bone'] == sq['bone'] or parents.get(sp['bone']) == sq['bone'] or parents.get(sq['bone']) == sp['bone'])


def visible_segments(shapes, parents, step=0.06):
    segs = {}
    occ = [(q, bbox(q['rings'])) for q in shapes if q['fill']]
    for p in shapes:
        res = []
        hiders = [(q, b) for q, b in occ if q is not p and (q['order'] > p['order'] or joined(p, q, parents))]
        for ring in p['rings']:
            n = len(ring)
            cnt = n if p['closed'] else n - 1
            for i in range(cnt):
                a, b = ring[i], ring[(i + 1) % n]
                L = math.hypot(b[0] - a[0], b[1] - a[1])
                m = max(1, int(L / step) + 1)
                for j in range(m):
                    p0 = (a[0] + (b[0] - a[0]) * j / m, a[1] + (b[1] - a[1]) * j / m)
                    p1 = (a[0] + (b[0] - a[0]) * (j + 1) / m, a[1] + (b[1] - a[1]) * (j + 1) / m)
                    mid = ((p0[0] + p1[0]) / 2, (p0[1] + p1[1]) / 2)
                    hid = False
                    for q, bb in hiders:
                        if bb[0] + 0.004 < mid[0] < bb[2] - 0.004 and bb[1] + 0.004 < mid[1] < bb[3] - 0.004 and inside(mid, q['rings']):
                            hid = True
                            break
                    if not hid:
                        res.append((p0, p1))
        segs[id(p)] = res
    return segs


def seg_dist(a, b, c, d):
    def pt(p, q, r):
        dx, dy = r[0] - q[0], r[1] - q[1]
        l2 = dx * dx + dy * dy
        t = 0 if l2 == 0 else max(0, min(1, ((p[0] - q[0]) * dx + (p[1] - q[1]) * dy) / l2))
        return math.hypot(p[0] - q[0] - t * dx, p[1] - q[1] - t * dy)
    d1x, d1y, d2x, d2y = b[0] - a[0], b[1] - a[1], d[0] - c[0], d[1] - c[1]
    den = d1x * d2y - d1y * d2x
    if abs(den) > 1e-14:
        t = ((c[0] - a[0]) * d2y - (c[1] - a[1]) * d2x) / den
        u = ((c[0] - a[0]) * d1y - (c[1] - a[1]) * d1x) / den
        if 0 <= t <= 1 and 0 <= u <= 1:
            return 0.0
    return min(pt(a, c, d), pt(b, c, d), pt(c, a, b), pt(d, a, b))


def gaps(shapes, parents):
    segs = visible_segments(shapes, parents)
    boxes = {}
    for p in shapes:
        ss = segs[id(p)]
        if ss:
            xs = [q[0] for s in ss for q in s]
            ys = [q[1] for s in ss for q in s]
            boxes[id(p)] = (min(xs), min(ys), max(xs), max(ys))
    res = []
    for i, p in enumerate(shapes):
        if id(p) not in boxes:
            continue
        for q in shapes[i + 1:]:
            if id(q) not in boxes:
                continue
            reach = 1.0 + (p['pen'] + q['pen']) / 2
            bp, bq = boxes[id(p)], boxes[id(q)]
            if bp[0] > bq[2] + reach or bq[0] > bp[2] + reach or bp[1] > bq[3] + reach or bq[1] > bp[3] + reach:
                continue
            best, at = 1e9, None
            qs = [s for s in segs[id(q)]]
            for a, b in segs[id(p)]:
                ax0, ax1, ay0, ay1 = min(a[0], b[0]) - reach, max(a[0], b[0]) + reach, min(a[1], b[1]) - reach, max(a[1], b[1]) + reach
                for c, d in qs:
                    if max(c[0], d[0]) < ax0 or min(c[0], d[0]) > ax1 or max(c[1], d[1]) < ay0 or min(c[1], d[1]) > ay1:
                        continue
                    dd = seg_dist(a, b, c, d)
                    if dd < best:
                        best, at = dd, ((a[0] + c[0]) / 2, (a[1] + c[1]) / 2)
                if best <= 1e-4:
                    break
            if best <= 1e-4 or best > reach:
                continue
            ink = best - (p['pen'] + q['pen']) / 2
            if ink > 0:
                res.append((ink, p['s']['id'], q['s']['id'], at))
    return sorted(res)


def edt(src, nx, ny):
    def one(f, n):
        d = [0.0] * n
        v = [0] * n
        z = [0.0] * (n + 1)
        k = 0
        z[0], z[1] = -INF, INF
        for q in range(1, n):
            while True:
                s = ((f[q] + q * q) - (f[v[k]] + v[k] * v[k])) / (2 * q - 2 * v[k])
                if s <= z[k]:
                    k -= 1
                else:
                    break
            k += 1
            v[k] = q
            z[k] = s
            z[k + 1] = INF
        k = 0
        for q in range(n):
            while z[k + 1] < q:
                k += 1
            d[q] = (q - v[k]) ** 2 + f[v[k]]
        return d
    g = [[0.0 if src[y][x] else INF for x in range(nx)] for y in range(ny)]
    for x in range(nx):
        col = one([g[y][x] for y in range(ny)], ny)
        for y in range(ny):
            g[y][x] = col[y]
    for y in range(ny):
        g[y] = one(g[y], nx)
    return g


def raster(rings, pitch, x0, y0, nx, ny):
    m = [[False] * nx for _ in range(ny)]
    for row in range(ny):
        y = y0 + (row + 0.5) * pitch
        xs = []
        for ring in rings:
            for i in range(len(ring)):
                a, b = ring[i], ring[i - 1]
                if (a[1] > y) != (b[1] > y):
                    xs.append(a[0] + (y - a[1]) / (b[1] - a[1]) * (b[0] - a[0]))
        xs.sort()
        for j in range(0, len(xs) - 1, 2):
            lo = max(0, math.ceil((xs[j] - x0) / pitch - 0.5))
            hi = min(nx - 1, math.floor((xs[j + 1] - x0) / pitch - 0.5))
            for c in range(lo, hi + 1):
                m[row][c] = True
    return m


def feature_width(rings, minf=MIN_FEAT):
    x0, y0, x1, y1 = bbox(rings)
    if x1 - x0 <= 0 or y1 - y0 <= 0:
        return 0
    pitch = max(minf / 5, 0.01)
    nx, ny = math.ceil((x1 - x0) / pitch) + 4, math.ceil((y1 - y0) / pitch) + 4
    if nx * ny > 400000:
        pitch *= math.sqrt(nx * ny / 400000)
        nx, ny = math.ceil((x1 - x0) / pitch) + 4, math.ceil((y1 - y0) / pitch) + 4
    ox, oy = x0 - 2 * pitch, y0 - 2 * pitch
    m = raster(rings, pitch, ox, oy, nx, ny)
    inside_n = sum(1 for r in m for c in r if c)
    if not inside_n:
        return 0
    d2 = edt([[not c for c in r] for r in m], nx, ny)
    rad = [[max(0.0, (math.sqrt(d2[y][x]) - 0.5) * pitch) if m[y][x] else 0.0 for x in range(nx)] for y in range(ny)]
    rmax = max(max(r) for r in rad)
    tol = max(0.01 * inside_n * pitch * pitch, (minf / 2) ** 2)

    def thin(w):
        r = w / 2
        core = [[m[y][x] and rad[y][x] >= r for x in range(nx)] for y in range(ny)]
        reach = edt(core, nx, ny)
        lim = ((r + pitch * 0.5) / pitch) ** 2
        return sum(1 for y in range(ny) for x in range(nx) if m[y][x] and reach[y][x] > lim) * pitch * pitch

    allowed = lambda w: tol + 4 * (1 - math.pi / 4) * (w / 2) ** 2
    top = 2 * rmax
    if thin(top) <= allowed(top):
        return top
    lo, hi = 0.0, top
    for _ in range(9):
        mid = (lo + hi) / 2
        if thin(mid) > allowed(mid):
            hi = mid
        else:
            lo = mid
    return hi


def sliver(shapes, parents, maxcells=300000):
    fills = [p for p in shapes if p['fill']]
    segs = visible_segments(shapes, parents, 0.2)
    pts = [q for p in shapes for s in segs[id(p)] for q in s] + [q for p in fills for r in p['rings'] for q in r]
    if not pts:
        return None
    x0, y0 = min(q[0] for q in pts), min(q[1] for q in pts)
    x1, y1 = max(q[0] for q in pts), max(q[1] for q in pts)
    pen = max(p['pen'] for p in shapes)
    margin = 2 * MIN_GAP + pen
    pitch = MIN_GAP / 6
    while ((x1 - x0 + 2 * margin) / pitch) * ((y1 - y0 + 2 * margin) / pitch) > maxcells:
        pitch *= 1.25
    x0 -= margin
    y0 -= margin
    nx, ny = math.ceil((x1 - x0 + margin) / pitch) + 1, math.ceil((y1 - y0 + margin) / pitch) + 1
    ink = [[False] * nx for _ in range(ny)]
    for p in fills:
        m = raster(p['rings'], pitch, x0, y0, nx, ny)
        for y in range(ny):
            for x in range(nx):
                if m[y][x]:
                    ink[y][x] = True
    for p in shapes:
        half = p['pen'] / 2
        for a, b in segs[id(p)]:
            cx0 = max(0, int((min(a[0], b[0]) - half - x0) / pitch - 0.5))
            cx1 = min(nx - 1, int((max(a[0], b[0]) + half - x0) / pitch + 1.5))
            cy0 = max(0, int((min(a[1], b[1]) - half - y0) / pitch - 0.5))
            cy1 = min(ny - 1, int((max(a[1], b[1]) + half - y0) / pitch + 1.5))
            dx, dy = b[0] - a[0], b[1] - a[1]
            l2 = dx * dx + dy * dy
            for cy in range(cy0, cy1 + 1):
                py = y0 + (cy + 0.5) * pitch
                for cx in range(cx0, cx1 + 1):
                    px = x0 + (cx + 0.5) * pitch
                    t = 0 if l2 == 0 else max(0, min(1, ((px - a[0]) * dx + (py - a[1]) * dy) / l2))
                    if math.hypot(px - a[0] - t * dx, py - a[1] - t * dy) <= half:
                        ink[cy][cx] = True
    d2 = edt(ink, nx, ny)
    rad = [[max(0.0, (math.sqrt(d2[y][x]) - 0.5) * pitch) if not ink[y][x] else 0.0 for x in range(nx)] for y in range(ny)]
    rmax = max(max(r) for r in rad)

    def unc(w):
        r = w / 2
        core = [[(not ink[y][x]) and rad[y][x] >= r for x in range(nx)] for y in range(ny)]
        reach = edt(core, nx, ny)
        lim = ((r + pitch * 0.5) / pitch) ** 2
        cnt, sx, sy = 0, 0.0, 0.0
        for y in range(ny):
            for x in range(nx):
                if not ink[y][x] and reach[y][x] > lim:
                    cnt += 1
                    sx += x0 + (x + 0.5) * pitch
                    sy += y0 + (y + 0.5) * pitch
        return cnt * pitch * pitch, (sx / cnt, sy / cnt) if cnt else (0, 0)

    allowed = lambda w: 3 * w * w
    if rmax <= 0 or unc(MIN_GAP)[0] <= allowed(MIN_GAP):
        return None
    lo, hi = 0.0, MIN_GAP
    for _ in range(8):
        mid = (lo + hi) / 2
        if unc(mid)[0] > allowed(mid):
            hi = mid
        else:
            lo = mid
    return hi, unc(MIN_GAP)[1]


def run(asset_id, scales=(500, 200, 100, 50), all_poses=False, slivers=True, features=True):
    path = find(asset_id)
    a = json.load(open(path))['asset']
    parents = {b['id']: b['parent'] for b in a['rig']['bones']}
    poses = [p for p in a['poses'] if not p.get('key')] if all_poses else [next(p for p in a['poses'] if p['id'] == a['presentation']['defaultPose'])]
    for den in scales:
        rep = a['representations'][str(den)]
        if rep['status'] != 'authored':
            print(f'{asset_id} 1:{den} unavailable')
            continue
        k = 1000 / den
        issues = []
        for pose in poses:
            rig = Rig(a['rig']['bones'], pose.get('bones', {}))
            shapes = paper_shapes(a, rep, rig, k)
            for g in gaps(shapes, parents):
                if g[0] < MIN_GAP - 1e-9:
                    issues.append(f'gap {g[1]}+{g[2]} {g[0]:.3f} @{g[3][0]:.2f},{g[3][1]:.2f} [{pose["id"]}]')
            if features:
                for p in shapes:
                    if p['fill']:
                        w = feature_width(p['rings'])
                        if w < MIN_FEAT - 1e-9:
                            issues.append(f'feature {p["s"]["id"]} {w:.3f} [{pose["id"]}]')
            if slivers:
                sv = sliver(shapes, parents)
                if sv:
                    issues.append(f'sliver {sv[0]:.3f} @{sv[1][0]:.2f},{sv[1][1]:.2f} [{pose["id"]}]')
            for p in shapes:
                if p['pen'] < MIN_STROKE - 1e-9:
                    issues.append(f'stroke {p["s"]["id"]} {p["pen"]}')
        seen = []
        for i in issues:
            if i not in seen:
                seen.append(i)
        print(f'{asset_id} 1:{den} {"OK" if not seen else str(len(seen)) + " issues"}')
        for i in seen[:40]:
            print('   ', i)


if __name__ == '__main__':
    args = [x for x in sys.argv[1:] if not x.startswith('--')]
    scales = tuple(int(v) for v in args[1].split(',')) if len(args) > 1 else (500, 200, 100, 50)
    run(args[0], scales, '--poses' in sys.argv, '--no-sliver' not in sys.argv, '--no-feature' not in sys.argv)
