"""Author 62: city-bus 1:100 / 1:200 / 1:500 hulls anew (lower hull with drawn arches, vertical upper hull), wheels as loops, thermal and structure kept (idempotent). Usage: python3 author-62-bus-coarse.py 100 [200 500]"""
import sys; sys.path.insert(0, '.')
from author_62_lib import *

A = 'vehicles/city-bus'
XA = (3.45, 9.35)
FLOOR, BELT, HW = 0.30, 1.2, 1.275
P = {
    '100': dict(crest=1.15, a=0.62, n=10, lean=0.032, rings=[1.15, 1.6, 2.2, 2.7, 2.85, 2.9, 2.94, 2.965, 2.975], bez_n=5, rt=0.40),
    '200': dict(crest=1.12, a=0.62, n=8, lean=0.0, rings=[1.15, 1.8, 2.7, 2.85, 2.93, 2.965, 2.975], bez_n=4, rt=0.40),
    '500': dict(crest=1.0, a=0.54, n=6, lean=0.0, rings=[1.15, 2.0, 2.7, 2.9, 2.965, 2.975], bez_n=3, rt=0.40),
}


def bez(p0, p1, p2, p3, n):
    out = []
    for i in range(n + 1):
        t = i / n
        u = 1 - t
        out.append((u ** 3 * p0[0] + 3 * u * u * t * p1[0] + 3 * u * t * t * p2[0] + t ** 3 * p3[0], u ** 3 * p0[1] + 3 * u * u * t * p1[1] + 3 * u * t * t * p2[1] + t ** 3 * p3[1]))
    return out


def lower_hull(p):
    secs = []
    for x, bot in bez((0.0, 0.5), (0.0, 0.38), (0.05, 0.3), (0.15, 0.3), p['bez_n']):
        secs.append(xsec(x, bot, 1.0 if x < 0.001 else BELT, corner(HW, 0.25, x)))
    secs.append(xsec(0.28, FLOOR, BELT, HW))
    a, c = p['a'], p['crest']
    for xa in XA:
        x0 = xa - a
        secs.append(xsec(x0 - 0.3, FLOOR, BELT, HW))
        h = 0.3 + 0.61 * (c - 0.3)
        pts = bez((x0, 0.3), (x0, h), (x0 + 0.435 * a, c), (xa, c), p['n'] // 2) + bez((xa, c), (xa + 0.565 * a, c), (xa + a, h), (xa + a, 0.3), p['n'] // 2)[1:]
        for k, (x, y) in enumerate(pts):
            if k == 0:
                secs.append(xsec(x, FLOOR, BELT, HW))
            elif k == len(pts) - 1:
                secs.append(xsec(x - 0.004, y + 0.04, BELT, HW))
                secs.append(xsec(x, FLOOR, BELT, HW))
            else:
                secs.append(xsec(x, y, BELT, HW))
        secs.append(xsec(xa + a + 0.3, FLOOR, BELT, HW))
    for x, bot, top in ((11.6, FLOOR, BELT), (11.85, FLOOR, BELT), (11.93, 0.31, BELT), (11.97, 0.37, BELT), (11.985, 0.43, 1.1), (11.999, 0.5, 0.95)):
        secs.append(xsec(x, bot, top, corner(HW, 0.25, 12.0 - x)))
    secs.sort(key=lambda s: s['at'][0])
    out = []
    for s in secs:
        if out and s['at'][0] <= out[-1]['at'][0]:
            s['at'][0] = rr(out[-1]['at'][0] + 0.002)
        out.append(s)
    return loft('body-shell', 'body', 'paint', 'body', 0.7, out)


def wall_of(p):
    lean = p['lean']
    def wall(y):
        return HW - lean * (y - 0.3) / 2.5 if y <= 2.8 else HW - lean
    return wall


def upper_hull(p):
    YF = [(1.15, 11.974), (2.7, 11.78), (2.8, 11.76), (2.88, 11.72), (2.93, 11.62), (2.96, 11.4), (2.975, 11.15)]
    YR = [(1.15, 0.0), (2.6, 0.0), (2.7, 0.02), (2.85, 0.06), (2.9, 0.12), (2.94, 0.28), (2.965, 0.55), (2.975, 0.9)]
    def lerp(tab, y):
        for (y0, x0), (y1, x1) in zip(tab, tab[1:]):
            if y0 <= y <= y1:
                return x0 + (x1 - x0) * (y - y0) / (y1 - y0)
        return tab[-1][1]
    wall = wall_of(p)
    wy = {2.85: 1.236, 2.9: 1.215, 2.93: 1.19, 2.94: 1.19, 2.96: 1.13, 2.965: 1.12, 2.975: 1.0}
    rings = []
    for y in p['rings']:
        w = wy[y] if y in wy else wall(y)
        if p['lean'] == 0 and y in wy and y < 2.95:
            w = wy[y] + 0.032
        xa, xb = lerp(YR, y), lerp(YF, y)
        rings.append((y, (xa + xb) / 2, (xb - xa) / 2, w))
    return vloft('body-upper', 'body', 'paint', 'body', 0.7, rings, e=20)


def tyre_new(s, tr):
    return {**s, 'profile': tyre_loop(0.30, 0.48, 0.38, 0.28)}


def main(sc):
    p = P[sc]
    d = load(A)
    v = d['volumes'][sc]
    B = v['body']
    wall = wall_of(p)
    phi = math.atan(p['lean'] / 2.5) if p['lean'] else 0.0
    replace(B, 'body-shell', lower_hull(p))
    B[:] = [x for x in B if x['id'] != 'body-upper']
    B.insert(1, upper_hull(p))
    for i in ('fl', 'rl', 'fr', 'rr'):
        if sc != '500':
            replace(B, f'{i}-tyre', tyre_new(solid_by_id(B, f'{i}-tyre'), sc))
        else:
            w = solid_by_id(B, f'{i}-wheel')
            w['profile'] = tyre_loop(0.30, 0.48, 0.38, 0.0)[:-4] + [[0.15, 0.1], [-0.15, 0.1], [-0.15, 0.38]]
            w['profile'] = [[-0.15, 0.1], [-0.15, 0.4], [-0.1, 0.48], [0.1, 0.48], [0.15, 0.4], [0.15, 0.1], [-0.15, 0.1]]
    for s in B:
        i = s['id']
        if p['lean'] and i.startswith('glass-') and s['kind'] == 'box':
            yc = s['transform']['translation'][1]
            sg = 1 if s['transform']['translation'][2] > 0 else -1
            t = s['size'][2]
            s['transform'] = {'translation': [s['transform']['translation'][0], yc, rr(sg * (wall(yc) + 0.0083 - t / 2))], 'rotation': quat_x(-sg * phi)}
        elif p['lean'] and i.startswith('leaf-m-') or p['lean'] and i.startswith('leaf-f-'):
            t = s['size'][2]
            s['transform'] = {'translation': [0.275, 1.06, rr(-(wall(1.4) + 0.013 - t / 2))], 'rotation': quat_x(phi)}
    qs = {'e-axle-casting': [0.45, 0.17, 0.72]}
    if sc != '500':
        solid_by_id(B, 'e-axle-casting')['radii'] = qs['e-axle-casting']
        for bn in ('axle-beam-r', 'axle-beam-f'):
            b = solid_by_id(B, bn)
            b['r0'] = b['r1'] = 0.08
    else:
        solid_by_id(B, 'e-axle-casting')['radii'] = qs['e-axle-casting']
    for bn in ('battery-pack', 'hvac-unit'):
        q = solid_by_id(B, bn)['sections']
        for z in q:
            z['exponent'] = 12
    rp = solid_by_id(B, 'roof-panel')['sections']
    for q in rp:
        q['at'][1] = {2.935: 2.947, 2.915: 2.927, 2.89: 2.897, 2.895: 2.897, 2.947: 2.947, 2.927: 2.927, 2.897: 2.897}[q['at'][1]]
        q['ry'] = 0.034 if sc != '500' else 0.0385
        q['exponent'] = 3.2 if sc != '500' else 3.2

    T_ = [t for t in v['thermal'] if not (t['id'] in ('th-shell', 'th-body-shell') or t['id'].startswith(('th-upper', 'th-body-upper')))]
    T_.insert(0, ref('th-body-shell', 'body-shell', 'body-panels'))
    T_.insert(1, ref('th-body-upper', 'body-upper', 'body-panels', None, ['+v']))
    for t in T_:
        if t['id'] == 'th-roof':
            t['sectionFaces'] = ['-v']
        elif t['id'].startswith('th-glass-') or t['id'] == 'th-windscreen':
            tid = t['of']
            if t['id'] == 'th-windscreen':
                t['sectionFaces'] = ['-u', '+v', '-v', '-w', '+w']
            else:
                left = solid_by_id(B, tid)['transform']['translation'][2] > 0
                t['sectionFaces'] = ['-w' if left else '+w', '-u', '+u', '-v', '+v']
        elif t['id'].startswith('th-leaf-'):
            t['sectionFaces'] = ['+w']
        elif t['id'].startswith('th-drum-'):
            t['profile'] = annulus_loop(-0.025, 0.025, 0.1, 0.205) if sc == '100' else ([[-0.025, 0.215], [0.025, 0.215]] if sc == '200' else [[-0.04, 0.2], [0.04, 0.2]])
    v['thermal'] = T_
    save(A, d)
    print(sc, 'bodies', len(v['body']), 'thermal', len(v['thermal']), 'structure', len(v['structure']))


for sc in sys.argv[1:]:
    main(sc)
