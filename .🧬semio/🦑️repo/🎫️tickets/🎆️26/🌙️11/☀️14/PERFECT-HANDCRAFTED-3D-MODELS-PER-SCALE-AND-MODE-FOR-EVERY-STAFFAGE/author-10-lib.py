import json, math, fnmatch, os, copy

TICKET = os.path.dirname(os.path.abspath(__file__))
GEN = os.path.join(TICKET, '🗑️generated', 'author10')
SCALES = (50, 100, 200, 500)
FEATURE = {50: 0.0075, 100: 0.015, 200: 0.03, 500: 0.075}
SQ2 = math.sqrt(0.5)


def r5(x):
    return round(x, 5)


def line_intervals(rings, p, d):
    ts = []
    for ring in rings:
        n = len(ring)
        for i in range(n):
            a = ring[i]
            b = ring[(i + 1) % n]
            ex, ey = b[0] - a[0], b[1] - a[1]
            den = d[0] * ey - d[1] * ex
            if abs(den) < 1e-12:
                continue
            t = ((a[0] - p[0]) * ey - (a[1] - p[1]) * ex) / den
            s = ((a[0] - p[0]) * d[1] - (a[1] - p[1]) * d[0]) / den
            if 0 <= s < 1:
                ts.append(t)
    ts.sort()
    return [(ts[i], ts[i + 1]) for i in range(0, len(ts) - 1, 2)]


def merge(iv):
    iv = sorted(iv)
    out = []
    for lo, hi in iv:
        if out and lo <= out[-1][1] + 1e-9:
            out[-1] = (out[-1][0], max(out[-1][1], hi))
        else:
            out.append((lo, hi))
    return out


def quat_cols(xl, yl, zl):
    m = [[xl[0], yl[0], zl[0]], [xl[1], yl[1], zl[1]], [xl[2], yl[2], zl[2]]]
    tr = m[0][0] + m[1][1] + m[2][2]
    if tr > 0:
        s = math.sqrt(tr + 1) * 2
        return [(m[2][1] - m[1][2]) / s, (m[0][2] - m[2][0]) / s, (m[1][0] - m[0][1]) / s, 0.25 * s]
    if m[0][0] > m[1][1] and m[0][0] > m[2][2]:
        s = math.sqrt(1 + m[0][0] - m[1][1] - m[2][2]) * 2
        return [0.25 * s, (m[0][1] + m[1][0]) / s, (m[0][2] + m[2][0]) / s, (m[2][1] - m[1][2]) / s]
    if m[1][1] > m[2][2]:
        s = math.sqrt(1 + m[1][1] - m[0][0] - m[2][2]) * 2
        return [(m[0][1] + m[1][0]) / s, 0.25 * s, (m[1][2] + m[2][1]) / s, (m[0][2] - m[2][0]) / s]
    s = math.sqrt(1 + m[2][2] - m[0][0] - m[1][1]) * 2
    return [(m[0][2] + m[2][0]) / s, (m[1][2] + m[2][1]) / s, 0.25 * s, (m[1][0] - m[0][1]) / s]


QU = [r5(x) for x in quat_cols((0, 0, 1), (0, -1, 0), (1, 0, 0))]
QV = [r5(x) for x in quat_cols((1, 0, 0), (0, 0, 1), (0, -1, 0))]
QW = [r5(x) for x in quat_cols((0, 1, 0), (1, 0, 0), (0, 0, -1))]


class Truth:
    def __init__(self, slug):
        self.slug = slug
        self.d = json.load(open(os.path.join(GEN, slug + '.json')))
        self.dims = self.d['dims']
        self.bones = {b['id']: b for b in self.d['bones']}
        self.thermal = self.d['thermal']
        self.supports = self.d['supports']
        self.rest = self.d['frames']

    def status(self, scale):
        return self.d['scales'][str(scale)]['status']

    def frames(self, scale):
        return self.d['scales'][f'{scale}frames']

    def shapes(self, scale, view):
        return self.d['scales'][str(scale)]['views'][view]

    def sel(self, scale, view, pats, roles=('body', 'detail', 'glazing'), closed=True):
        out = []
        pats = [pats] if isinstance(pats, str) else pats
        for s in self.shapes(scale, view):
            if not s['rings'] or (closed and not s['closed']) or s['role'] not in roles:
                continue
            if any(fnmatch.fnmatch(s['id'], p) or (p.startswith('bone:') and s['bone'] == p[5:]) for p in pats):
                out.append(s)
        return out

    def rings_of(self, shapes):
        return [s['rings'] for s in shapes]

    def intervals(self, shapes, p, d):
        iv = []
        for s in shapes:
            iv += line_intervals(s['rings'], p, d)
        return merge(iv)

    def extent(self, shapes, axis):
        pts = [q for s in shapes for r in s['rings'] for q in r]
        if not pts:
            return None
        v = [q[axis] for q in pts]
        return min(v), max(v)


def vadd(a, b):
    return [a[i] + b[i] for i in range(3)]


def vscale(a, k):
    return [a[i] * k for i in range(3)]


def vdot(a, b):
    return sum(a[i] * b[i] for i in range(3))


class Fig:
    def __init__(self, T, scale):
        self.T = T
        self.scale = scale
        self.F = T.frames(scale)
        self.feature = FEATURE[scale]
        self.body = []
        self.thermal = []
        self.structure = []

    def fr(self, bone):
        return self.F[bone]

    def to_local(self, bone, p):
        f = self.F[bone]
        d = [p[i] - f['o'][i] for i in range(3)]
        return [vdot(d, f['u']), vdot(d, f['v']), vdot(d, f['w'])]

    def to_world(self, bone, q):
        f = self.F[bone]
        return [f['o'][i] + q[0] * f['u'][i] + q[1] * f['v'][i] + q[2] * f['w'][i] for i in range(3)]

    def urange(self, bone, shapes_e):
        f = self.F[bone]
        pts = [q for s in shapes_e for r in s['rings'] for q in r]
        us = [(q[0] - f['o'][0]) * f['u'][0] + (q[1] - f['o'][1]) * f['u'][1] for q in pts]
        return min(us), max(us)

    def slice(self, bone, s, elev, front, plan=None):
        f = self.F[bone]
        P = [f['o'][i] + s * f['u'][i] for i in range(3)]
        res = {}
        iv = self.T.intervals(elev, (P[0], P[1]), (f['v'][0], f['v'][1])) if elev else []
        if elev and not iv:
            ey = self.T.extent(elev, 1)
            ex = self.T.extent(elev, 0)
            if abs(f['u'][1]) >= abs(f['u'][0]):
                yy = min(max(P[1], ey[0] + 0.004), ey[1] - 0.004)
                iv = self.T.intervals(elev, (P[0] + (yy - P[1]) * f['u'][0] / (f['u'][1] or 1e-9), yy), (f['v'][0], f['v'][1]))
            else:
                xx = min(max(P[0], ex[0] + 0.004), ex[1] - 0.004)
                iv = self.T.intervals(elev, (xx, P[1] + (xx - P[0]) * f['u'][1] / (f['u'][0] or 1e-9)), (f['v'][0], f['v'][1]))
        res['v'] = (iv[0][0], iv[-1][1]) if iv else None
        if res['v'] is None and getattr(self, 'vconst', None):
            res['v'] = self.vconst
        w = None
        horizontal = abs(f['u'][1]) < abs(f['u'][0])
        if horizontal and plan:
            ivp = self.T.intervals(plan, (P[0], P[2]), (0, 1))
            if ivp:
                w = (ivp[0][0] + P[2] - f['o'][2], ivp[-1][1] + P[2] - f['o'][2])
        elif front:
            ivf = self.T.intervals(front, (P[2], P[1]), (1, 0))
            if not ivf:
                ey = self.T.extent(front, 1)
                yy = min(max(P[1], ey[0] + 0.004), ey[1] - 0.004)
                ivf = self.T.intervals(front, (P[2], yy), (1, 0))
            if ivf:
                w = (ivf[0][0] + P[2] - f['o'][2], ivf[-1][1] + P[2] - f['o'][2])
        if w is None and getattr(self, 'wconst', None):
            w = self.wconst
        res['w'] = w
        return res

    def sections(self, bone, stations, elev, front, plan=None, exp=2.0, dk=1.0, wk=1.0, vshift=0.0, wshift=0.0, minr=0.0, ends=0.012, vclamp=None, wclamp=None, grow=0.0):
        out = []
        n = len(stations)
        for i, s in enumerate(stations):
            r = self.slice(bone, s, elev, front, plan)
            if r['v'] is None and r['w'] is None:
                continue
            v = r['v'] or (-0.03, 0.03)
            w = r['w'] or (-(v[1] - v[0]) / 2, (v[1] - v[0]) / 2)
            if vclamp:
                v = (max(v[0], vclamp[0]), min(v[1], vclamp[1]))
            if wclamp:
                w = (max(w[0], wclamp[0]), min(w[1], wclamp[1]))
            if grow:
                v = (v[0] - grow, v[1] + grow)
                w = (w[0] - grow, w[1] + grow)
            ex = exp[i] if isinstance(exp, (list, tuple)) else exp
            out.append({'at': [r5(s), r5((v[0] + v[1]) / 2 + vshift), r5((w[0] + w[1]) / 2 + wshift)], 'rx': r5(max(minr, (w[1] - w[0]) / 2 * wk)), 'ry': r5(max(minr, (v[1] - v[0]) / 2 * dk)), 'exponent': ex, 'rotation': QU})
        return out

    def add(self, lst, s):
        getattr(self, lst).append(s)
        return s

    def solid(self, lst, id, bone, material, tone, kind, role='body', **kw):
        s = {'id': id, 'bone': bone, 'material': material, 'role': role, 'tone': tone, 'kind': kind}
        s.update(kw)
        return self.add(lst, s)

    def loft(self, id, bone, material, tone, secs, role='body', lst='body', cap=True, **kw):
        s = {'id': id, 'bone': bone, 'material': material, 'role': role, 'tone': tone, 'kind': 'loft', 'sections': secs, 'capStart': cap, 'capEnd': cap}
        s.update(kw)
        return self.add(lst, s)

    def capsule(self, id, bone, material, tone, p0, p1, r0, r1=None, role='body', lst='body', **kw):
        s = {'id': id, 'bone': bone, 'material': material, 'role': role, 'tone': tone, 'kind': 'capsule', 'p0': [r5(x) for x in p0], 'p1': [r5(x) for x in p1], 'r0': r5(r0), 'r1': r5(r0 if r1 is None else r1)}
        s.update(kw)
        return self.add(lst, s)

    def ellipsoid(self, id, bone, material, tone, radii, at, role='body', lst='body', **kw):
        s = {'id': id, 'bone': bone, 'material': material, 'role': role, 'tone': tone, 'kind': 'ellipsoid', 'radii': [r5(x) for x in radii], 'transform': {'translation': [r5(x) for x in at]}}
        s.update(kw)
        return self.add(lst, s)

    def box(self, id, bone, material, tone, size, at, role='body', lst='body', rot=None, corner=None, **kw):
        s = {'id': id, 'bone': bone, 'material': material, 'role': role, 'tone': tone, 'kind': 'box', 'size': [r5(x) for x in size]}
        tr = {'translation': [r5(x) for x in at]}
        if rot:
            tr['rotation'] = rot
        s['transform'] = tr
        if corner:
            s['cornerRadius'] = corner
        s.update(kw)
        return self.add(lst, s)

    def revolve(self, id, bone, material, tone, profile, axis, origin, role='body', lst='body', **kw):
        s = {'id': id, 'bone': bone, 'material': material, 'role': role, 'tone': tone, 'kind': 'revolve', 'profile': [[r5(a), r5(b)] for a, b in profile], 'axis': axis, 'origin': [r5(x) for x in origin]}
        s.update(kw)
        return self.add(lst, s)

    def extrude(self, id, bone, material, tone, profile, plane, a, b, role='body', lst='body', **kw):
        s = {'id': id, 'bone': bone, 'material': material, 'role': role, 'tone': tone, 'kind': 'extrude', 'profile': [[r5(x), r5(y)] for x, y in profile], 'plane': plane, 'from': r5(a), 'to': r5(b)}
        s.update(kw)
        return self.add(lst, s)

    def ref(self, id, of, region, layer=None, tags=None, lst='thermal', member=None, part=None):
        s = {'id': id, 'of': of}
        if lst == 'thermal':
            s['region'] = region
            if layer:
                s['layer'] = layer
            if tags:
                s['tags'] = tags
        return self.add(lst, s)

    def find(self, id):
        for s in self.body:
            if s['id'] == id:
                return s
        raise KeyError(id)


def shrink_loft(s, t, newid, extra=None):
    c = copy.deepcopy(s)
    c['id'] = newid
    for q in c['sections']:
        q['rx'] = r5(max(0.002, q['rx'] - t))
        q['ry'] = r5(max(0.002, q['ry'] - t))
    if extra:
        c.update(extra)
    return c


def shrink_solid(s, t, newid):
    c = copy.deepcopy(s)
    c['id'] = newid
    k = s['kind']
    if k == 'loft':
        return shrink_loft(s, t, newid)
    if k == 'capsule':
        c['r0'] = r5(max(0.002, s['r0'] - t))
        c['r1'] = r5(max(0.002, s['r1'] - t))
    elif k == 'ellipsoid':
        c['radii'] = [r5(max(0.002, x - t)) for x in s['radii']]
    elif k == 'box':
        c['size'] = [r5(max(0.004, x - 2 * t)) for x in s['size']]
    elif k == 'revolve':
        c['profile'] = [[a, r5(max(0.002, b - t))] for a, b in s['profile']]
    else:
        raise ValueError('shrink ' + k)
    return c


def text(en, de):
    return {'en': en, 'de': de}


def dump(slug, cat, volumes):
    path = os.path.join(TICKET, 'volumes', cat, slug + '.json')
    os.makedirs(os.path.dirname(path), exist_ok=True)
    json.dump({'volumes': volumes}, open(path, 'w'), ensure_ascii=False, separators=(',', ':'))
    return path
