"""Author 62: city-bus 1:50 body polish (idempotent: works on the source's current lists, ids of the new solids are fixed)."""
import sys; sys.path.insert(0, '.')
from author_62_lib import *

A = 'vehicles/city-bus'
d = load(A)
v = d['volumes']['50']
B = v['body']

XA = (3.45, 9.35)
RW = 0.48
R_ARCH = 0.64
FLOOR = 0.30
BELT = 1.2
HW = 1.275
LEAN0, LEAN1, LEAN_W = 0.3, 2.8, 0.032


def wall(y):
    if y <= LEAN0:
        return HW
    if y <= LEAN1:
        return HW - LEAN_W * (y - LEAN0) / (LEAN1 - LEAN0)
    return None


def lean_phi():
    return math.atan(LEAN_W / (LEAN1 - LEAN0))


def bez(p0, p1, p2, p3, n):
    out = []
    for i in range(n + 1):
        t = i / n
        u = 1 - t
        out.append((u ** 3 * p0[0] + 3 * u * u * t * p1[0] + 3 * u * t * t * p2[0] + t ** 3 * p3[0], u ** 3 * p0[1] + 3 * u * u * t * p1[1] + 3 * u * t * t * p2[1] + t ** 3 * p3[1]))
    return out


def lower_hull():
    secs = []
    for x, bot in bez((0.0, 0.5), (0.0, 0.38), (0.05, 0.3), (0.15, 0.3), 6):
        secs.append(xsec(x, bot, 1.0 if x < 0.001 else BELT, corner(HW, 0.25, x)))
    secs.append(xsec(0.28, FLOOR, BELT, HW))
    for xa in XA:
        x0 = xa - 0.62
        secs.append(xsec(x0 - 0.3, FLOOR, BELT, HW))
        pts = bez((x0, 0.3), (x0, 0.8), (x0 + 0.27, 1.12), (xa, 1.12), 12) + bez((xa, 1.12), (xa + 0.35, 1.12), (xa + 0.62, 0.8), (xa + 0.62, 0.3), 12)[1:]
        for k, (x, y) in enumerate(pts):
            if k == 0:
                secs.append(xsec(x, FLOOR, BELT, HW))
                continue
            if k == len(pts) - 1:
                secs.append(xsec(x - 0.003, y + 0.03, BELT, HW))
                secs.append(xsec(x, FLOOR, BELT, HW))
                continue
            secs.append(xsec(x, y, BELT, HW))
        secs.append(xsec(xa + 0.62 + 0.3, FLOOR, BELT, HW))
    for x, bot, top in ((11.6, FLOOR, BELT), (11.85, FLOOR, BELT), (11.93, 0.31, BELT), (11.97, 0.37, BELT), (11.985, 0.43, 1.1), (11.999, 0.5, 0.95)):
        secs.append(xsec(x, bot, top, corner(HW, 0.25, 12.0 - x)))
    secs.sort(key=lambda s: s['at'][0])
    out = []
    for s in secs:
        if out and s['at'][0] <= out[-1]['at'][0]:
            s['at'][0] = rr(out[-1]['at'][0] + 0.002)
        out.append(s)
    return loft('body-shell', 'body', 'paint', 'body', 0.7, out)


def upper_hull():
    YF = [(1.15, 11.974), (2.7, 11.78), (2.8, 11.76), (2.88, 11.72), (2.93, 11.62), (2.96, 11.4), (2.975, 11.15)]
    YR = [(1.15, 0.0), (2.6, 0.0), (2.7, 0.02), (2.85, 0.06), (2.9, 0.12), (2.94, 0.28), (2.965, 0.55), (2.975, 0.9)]
    def lerp(tab, y):
        for (y0, x0), (y1, x1) in zip(tab, tab[1:]):
            if y0 <= y <= y1:
                return x0 + (x1 - x0) * (y - y0) / (y1 - y0)
        return tab[-1][1]
    ys = [1.15, 1.25, 1.6, 2.0, 2.4, 2.6, 2.7, 2.8, 2.85, 2.9, 2.93, 2.96, 2.975]
    wy = {2.85: 1.236, 2.9: 1.215, 2.93: 1.19, 2.96: 1.13, 2.975: 1.0}
    rings = []
    for y in ys:
        w = wy.get(y) or wall(y) or HW
        xa, xb = lerp(YR, y), lerp(YF, y)
        rings.append((y, (xa + xb) / 2, (xb - xa) / 2, w))
    return vloft('body-upper', 'body', 'paint', 'body', 0.7, rings, e=20)


def roof_panel():
    secs = []
    for x, y, rx, ry in ((0.3, 2.947, 1.2, 0.034), (1.0, 2.947, 1.2, 0.034), (11.0, 2.947, 1.2, 0.034), (11.4, 2.927, 1.2, 0.034), (11.6, 2.897, 1.18, 0.034)):
        secs.append({"at": [x, y, 0], "rx": rx, "ry": ry, "exponent": 3.2, "rotation": [0.70711, 0, 0.70711, 0]})
    return loft('roof-panel', 'body', 'paint', 'detail', 0.8, secs)


def fix_revolves(lst):
    for s in lst:
        if s.get('kind') != 'revolve':
            continue
        p = s['profile']
        if p[0] != p[-1] and len(p) == 4 and p[0][1] == p[-1][1] == 0.1:
            s['profile'] = [p[0], p[1], p[2], p[3], p[0]]


def tyre_new(i, side, s):
    r_out = 0.48
    return {**s, 'profile': tyre_loop(0.30, r_out, 0.38, 0.28)}


def rim_new(s, side):
    h = 0.14
    r = 0.286
    t = 0.012
    a_d = h - 0.07
    a = [(h, r), (-h, r), (-h, r - t), (a_d - 0.015, r - t), (a_d - 0.015, 0.11), (a_d, 0.11), (a_d, r - t), (h, r - t), (h, r)]
    prof = [[rr(sgn * x), rr(y)] for x, y in a for sgn in [1 if side > 0 else -1]]
    return {**s, 'profile': prof}


def hub_new(s, side):
    return s


for i in ('fl', 'rl', 'fr', 'rr'):
    side = 1 if i[1] == 'l' else -1
    replace(B, f'{i}-tyre', tyre_new(i, side, solid_by_id(B, f'{i}-tyre')))
    replace(B, f'{i}-rim', rim_new(solid_by_id(B, f'{i}-rim'), side))

dr = {}
for i in ('fl', 'rl', 'fr', 'rr'):
    side = 1 if i[1] == 'l' else -1
    s = solid_by_id(B, f'{i}-drum')
    a0, a1 = (-0.14, -0.10) if side > 0 else (0.10, 0.14)
    s['profile'] = annulus_loop(a0, a1, 0.11, 0.2)

replace(B, 'body-shell', lower_hull())
for bn, w in (('bumper-front', 2.2), ('bumper-rear', 2.2)):
    solid_by_id(B, bn)['size'][2] = w
B[:] = [x for x in B if x['id'] != 'body-upper']
B.insert(1, upper_hull())
replace(B, 'roof-panel', roof_panel())
for bn in ('battery-pack', 'hvac-unit'):
    q = solid_by_id(B, bn)['sections']
    for z in q:
        z['exponent'] = 12
    q[0]['at'][1] = 0.01
    q[-1]['at'][1] = 0.01
for sd, sg in (('l', 1), ('r', -1)):
    pass
for hn in ('leaf-handrail-m-aft', 'leaf-handrail-m-fwd', 'leaf-handrail-f-aft', 'leaf-handrail-f-fwd'):
    h = solid_by_id(B, hn)
    h['p0'][2] = -1.27
    h['p1'][2] = -1.27

phi = lean_phi()
newB = []
for s in B:
    i = s['id']
    if i.startswith(('side-window', 'upper-window')) and not i.startswith('side-window-door-gap'):
        if 'repeat' in s:
            parts = []
            for suf in ('l', 'r'):
                t = copy.deepcopy(s)
                del t['repeat']
                t['id'] = i + '-' + suf
                parts.append(t)
        else:
            parts = [s]
        for t in parts:
            sg = 1 if t['id'].endswith('-l') else -1
            yc = t['transform']['translation'][1]
            t['transform'] = {'translation': [t['transform']['translation'][0], yc, rr(sg * (wall(yc) + 0.008 - 0.03))], 'rotation': quat_x(-sg * phi)}
            newB.append(t)
    elif i.startswith('side-window-door-gap'):
        yc = s['transform']['translation'][1]
        s['transform'] = {'translation': [s['transform']['translation'][0], yc, rr(wall(yc) + 0.008 - 0.03)], 'rotation': quat_x(-phi)}
        newB.append(s)
    elif i.startswith('door-opening'):
        s['transform'] = {'translation': [s['transform']['translation'][0], 1.4, rr(-(wall(1.4) + 0.005))], 'rotation': quat_x(phi)}
        newB.append(s)
    elif i.startswith(('leaf-m-', 'leaf-f-')) and not i.startswith(('leaf-handrail')):
        if i.endswith('glass'):
            s['transform'] = {'translation': [0.275, 1.25, rr(-(wall(1.59) - 0.03))], 'rotation': quat_x(phi)}
        else:
            s['transform'] = {'translation': [0.275, 1.06, rr(-(wall(1.4) + 0.013 - 0.045))], 'rotation': quat_x(phi)}
        newB.append(s)
    else:
        newB.append(s)
v['body'] = newB
B = v['body']
fix_revolves(v['thermal'])
ea = solid_by_id(B, 'e-axle-casting')
ea['radii'] = [0.45, 0.17, 0.72]
ab = solid_by_id(B, 'axle-beam-r')
ab['r0'] = ab['r1'] = 0.08
for t in v['thermal']:
    if t['id'].startswith('th-drum-'):
        t['profile'] = annulus_loop(-0.02, 0.02, 0.1, 0.215)


def thermal_walls():
    T_ = [t for t in v['thermal'] if not (t.get('region') in ('body-panels', 'roof', 'doors', 'glazing'))]
    return T_


T_ = thermal_walls()
glz = []
bodypar = ['body-shell', 'body-upper', 'bumper-front', 'bumper-rear', 'roof-panel']
for bid in bodypar:
    reg = 'roof' if bid == 'roof-panel' else 'body-panels'
    sol = solid_by_id(B, bid)
    nm = bid.replace('bumper-front', 'bumper-f').replace('bumper-rear', 'bumper-r')
    T_.append(ref(f'th-{nm}-skin', bid, reg, 'skin', ['+v'] if bid == 'body-upper' else (['-v'] if bid == 'roof-panel' else None)))
    T_.append(shell(sol, 0.0015, f'th-{nm}-insulation', reg, 'insulation', 0.0008))
for sd in ('l', 'r'):
    for n in ('r', 'f'):
        T_.append(ref(f'th-wheelhouse-{n}-{sd}-insulation', f'wheelhouse-{n}-{sd}', 'body-panels', 'insulation'))
for n in ('m-aft', 'm-fwd', 'f-aft', 'f-fwd'):
    sol = solid_by_id(B, f'leaf-{n}')
    T_.append(ref(f'th-leaf-{n}-skin', f'leaf-{n}', 'doors', 'skin', ['+w']))
    T_.append(shell(sol, 0.0015, f'th-leaf-{n}-insulation', 'doors', 'insulation', 0.0008))
for s in B:
    if s['role'] == 'glazing' and (s['id'].startswith(('side-window', 'upper-window')) or s['id'] == 'windscreen'):
        i = s['id']
        T_.append(ref(f'th-{i}-pane-in', i, 'glazing', 'pane-in', ['-w', '-u', '+u', '-v', '+v'] if i.endswith('-l') else (['+w', '-u', '+u', '-v', '+v'] if i.endswith('-r') else (['-u', '+v', '-v', '-w', '+w'] if i == 'windscreen' else ['-w', '-u', '+u', '-v', '+v']))))
        T_.append(shell(s, 0.004, f'th-{i}-gap', 'glazing', 'gap', 0.004))
        T_.append(shell(s, 0.016, f'th-{i}-pane-out', 'glazing', 'pane-out', 0.004))
v['thermal'] = T_

for s in v['structure']:
    pass
save(A, d)
print('bodies', len(v['body']), 'thermal', len(v['thermal']), 'structure', len(v['structure']))
