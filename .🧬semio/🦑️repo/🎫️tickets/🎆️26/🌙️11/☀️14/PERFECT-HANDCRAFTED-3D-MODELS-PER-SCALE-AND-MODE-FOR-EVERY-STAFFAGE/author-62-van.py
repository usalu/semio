"""Author 62: delivery-van hulls (lower hull with drawn arches, vertical upper hull with tumblehome and crowned roof), wheels as loops, thermal and exhaust, all scales. Usage: python3 author-62-van.py 50 [100 200 500] (idempotent)"""
import sys; sys.path.insert(0, '.')
from author_62_lib import *

A = 'vehicles/delivery-van'
FLOOR, BELT, HW = 0.356, 1.1, 0.91
XA = (-1.6, 1.7)
LEAN = 0.0429
P = {
    '50': dict(rings=[1.0, 1.1, 1.45, 1.8, 1.9, 1.95, 1.97, 1.99], lean=True, e=30, arch_keep=True),
    '100': dict(rings=[1.0, 1.1, 1.6, 1.9, 1.95, 1.97, 1.99], lean=True, e=30, arch_keep=True),
    '200': dict(rings=[1.0, 1.1, 1.8, 1.95, 1.99], lean=False, e=30, arch_keep=True),
    '500': dict(rings=[1.0, 1.1, 1.95, 1.99], lean=False, e=30, arch_keep=True),
}


def wall(y, lean):
    if not lean:
        return HW
    return HW if y <= 1.1 else HW - LEAN * (y - 1.1)


def lower_hull(old, pr):
    secs = []
    for q in old['sections']:
        x = q['at'][0]
        bot = q['at'][1] - q['ry']
        top = min(q['at'][1] + q['ry'], BELT)
        if top - bot < 0.02:
            continue
        secs.append(xsec(x, bot, top, q['rx']))
    secs.sort(key=lambda s: s['at'][0])
    out = []
    for s in secs:
        if out and s['at'][0] <= out[-1]['at'][0]:
            s['at'][0] = rr(out[-1]['at'][0] + 0.002)
        out.append(s)
    return loft('body-shell', 'chassis', 'paint', 'body', 0.7, out)


def upper_hull(pr):
    YF = [(1.0, 1.9), (1.1, 1.82), (1.95, 1.2), (1.99, 1.12)]
    YR = [(1.0, -2.67), (1.93, -2.67), (1.97, -2.63), (1.99, -2.58)]
    wy = {1.9: 0.862, 1.95: 0.84, 1.97: 0.83, 1.99: 0.78}
    def lerp(tab, y):
        for (y0, x0), (y1, x1) in zip(tab, tab[1:]):
            if y0 <= y <= y1:
                return x0 + (x1 - x0) * (y - y0) / (y1 - y0)
        return tab[-1][1]
    rings = []
    for y in pr['rings']:
        w = wy.get(y) if y in wy and pr['lean'] else (wall(y, pr['lean']) if y < 1.95 else wy[y] + (0 if pr['lean'] else 0.03))
        if w is None:
            w = wall(y, pr['lean'])
        xa, xb = lerp(YR, y), lerp(YF, y)
        rings.append((y, (xa + xb) / 2, (xb - xa) / 2, w))
    return vloft('body-upper', 'chassis', 'paint', 'body', 0.7, rings, e=pr['e'])


def roof_lens(rx, ry=0.03):
    secs = [{"at": [x, 1.96, 0], "rx": rx, "ry": ry, "exponent": 3.2, "rotation": [0.70711, 0, 0.70711, 0]} for x in (-2.55, 1.12)]
    return loft('roof-cap', 'chassis', 'paint', 'body', 0.8, secs)


def tyre(s, r_out, w, seat):
    return {**s, 'profile': tyre_loop(w, r_out, 0, seat)}


def rim(s, side, r=0.2, h=0.09, recess=0.05, hub=0.055, t=0.008):
    a_d = h - recess
    pts = [(h, r), (-h, r), (-h, r - t), (a_d - 0.012, r - t), (a_d - 0.012, hub), (a_d, hub), (a_d, r - t), (h, r - t), (h, r)]
    return {**s, 'profile': [[rr(side * x * -1 if False else (x if side > 0 else -x)), rr(y)] for x, y in pts]}


def main(sc):
    pr = P[sc]
    d = load(A)
    v = d['volumes'][sc]
    B = v['body']
    old = solid_by_id(B, 'body-shell')
    base = json.load(open(os.path.join(G, f'van-orig-{sc}.json'))) if os.path.exists(os.path.join(G, f'van-orig-{sc}.json')) else None
    if base is None:
        base = copy.deepcopy(old)
        json.dump(base, open(os.path.join(G, f'van-orig-{sc}.json'), 'w'))
    replace(B, 'body-shell', lower_hull(base, pr))
    B[:] = [x for x in B if x['id'] != 'body-upper']
    B.insert(1, upper_hull(pr))
    rx = 0.84 if sc in ('50', '100') else 0.87
    ro = solid_by_id(B, 'roof-cap')
    replace(B, 'roof-cap', roof_lens(rx, 0.0385 if sc == '500' else 0.03))
    phi = LEAN if pr['lean'] else 0.0
    for s in B:
        i = s['id']
        if i in ('door-window-l', 'door-window-r') and s['kind'] == 'extrude':
            sg = 1 if i.endswith('-l') else -1
            if phi:
                s['transform'] = {'translation': [0, 0, rr(sg * 0.0469)], 'rotation': quat_x(-sg * phi)}
            else:
                s.pop('transform', None)
        if i == 'slider-panel' and phi:
            s['transform']['rotation'] = quat_x(phi)
    for i in ('fl', 'rl', 'fr', 'rr'):
        side = 1 if i[1] == 'l' else -1
        if sc in ('50', '100', '200'):
            replace(B, f'{i}-tyre', tyre(solid_by_id(B, f'{i}-tyre'), 0.356, 0.2, 0.195))
        if sc in ('50',):
            replace(B, f'{i}-rim', rim(solid_by_id(B, f'{i}-rim'), side))
    for bn in ('axle-f-beam', 'axle-r-beam'):
        for s in B:
            if s['id'] == bn:
                s['p0'][1] = 0.06
                s['p1'][1] = 0.06
    for s in v['structure']:
        if s['id'].endswith(('-beam-member',)):
            s['p0'][1] = 0.06
            s['p1'][1] = 0.06
        if s['id'] in ('axle-f-mass', 'axle-r-mass'):
            s['transform']['translation'][1] = 0.06
    T_ = v['thermal']
    for t in T_:
        if t.get('kind') == 'revolve' and t['id'].startswith('th-brake-'):
            p = t['profile']
            if len(p) in (4, 5) and p[0][1] > 0.01:
                a = p[0][0]
                ro = 0.17
                ri = p[0][1]
                t['profile'] = [[a, ri], [a, ro], [-a, ro], [-a, ri], [a, ri]]
    fine = sc == '50'
    pre = lambda n: (n + '-paint') if fine else n
    drop = {'th-shell-wall', 'th-upper-paint', 'th-upper-wall', 'th-upper', 'th-roof-wall', 'th-dpf-can'}
    T_[:] = [t for t in T_ if t['id'] not in drop]
    sh = solid_by_id(B, 'body-shell')
    up = solid_by_id(B, 'body-upper')
    rf = solid_by_id(B, 'roof-cap')
    k = next(i for i, t in enumerate(T_) if t['id'] == pre('th-shell'))
    if fine:
        T_.insert(k + 1, shell(sh, 0.001, 'th-shell-wall', 'body-panels', 'wall', 0.0008))
        T_.insert(k + 2, ref('th-upper-paint', 'body-upper', 'body-panels', 'paint', ['+v']))
        T_.insert(k + 3, shell(up, 0.001, 'th-upper-wall', 'body-panels', 'wall', 0.0008))
        kr = next(i for i, t in enumerate(T_) if t['id'] == 'th-roof-paint')
        T_.insert(kr + 1, shell(rf, 0.001, 'th-roof-wall', 'roof', 'wall', 0.0008))
    else:
        T_.insert(k + 1, ref('th-upper', 'body-upper', 'body-panels', None, ['+v']))
    sf = {'th-roof': ['-v'], 'th-roof-paint': ['-v'], 'th-hood': ['-v'], 'th-hood-paint': ['-v'], 'th-slider': ['+w'], 'th-slider-paint': ['+w'],
          'th-rear-l': ['-w'], 'th-rear-r': ['-w'], 'th-rear-l-paint': ['-w'], 'th-rear-r-paint': ['-w'],
          'th-windscreen': ['-u', '+v', '-v', '-w', '+w'], 'th-door-window-l': ['-w', '-u', '+u', '-v', '+v'], 'th-door-window-r': ['+w', '-u', '+u', '-v', '+v']}
    for t in T_:
        if t['id'] in sf:
            t['sectionFaces'] = sf[t['id']]
    ex = next((t for t in T_ if t['id'] == 'th-exhaust-line'), None)
    if ex is not None:
        ex['p0'] = [1.2, 0.42, -0.31]
        ex['p1'] = [-2.3, 0.42, -0.31]
        for t in T_:
            if t['id'] == 'th-silencer-can':
                t['r0'] = t['r1'] = 0.085
        k = T_.index(ex)
        T_.insert(k + 1, {**copy.deepcopy(ex), 'id': 'th-dpf-can', 'p0': [-0.2, 0.42, -0.31], 'p1': [0.2, 0.42, -0.31], 'r0': 0.08, 'r1': 0.08})
        if sc != '50':
            T_[k + 1].pop('layer', None)
    save(A, d)
    print(sc, 'bodies', len(B), 'thermal', len(T_))


for sc in sys.argv[1:]:
    main(sc)
