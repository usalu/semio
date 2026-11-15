if 'Fig' not in globals():
    exec(open(os.path.join(os.path.dirname(os.path.abspath(__file__)), 'author-10-lib.py')).read())


def lin(n, lo=0.0, hi=1.0, bias=1.0):
    if n == 1:
        return [(lo + hi) / 2]
    return [lo + (hi - lo) * ((i / (n - 1)) ** bias) for i in range(n)]


def fr_stations(u0, u1, n, lo=0.0, hi=1.0, eps=0.012, bias=1.0):
    return [u0 + (u1 - u0) * min(1 - eps, max(eps, x)) for x in lin(n, lo, hi, bias)]


def auto_stations(f, bone, e, fr, pl, ur, n, lo=0.0, hi=1.0, vclamp=None, wclamp=None, m=60, eps=0.012):
    u0 = ur[0] + (ur[1] - ur[0]) * lo
    u1 = ur[0] + (ur[1] - ur[0]) * hi
    if n <= 2:
        return [u0 + (u1 - u0) * eps, u1 - (u1 - u0) * eps][:n]
    xs = [u0 + (u1 - u0) * min(1 - eps, max(eps, i / (m - 1))) for i in range(m)]
    ch = []
    for u in xs:
        r = f.slice(bone, u, e, fr, pl)
        v = r['v'] or (0, 0)
        w = r['w'] or (-(v[1] - v[0]) / 2, (v[1] - v[0]) / 2)
        if vclamp:
            v = (max(v[0], vclamp[0]), min(v[1], vclamp[1]))
        if wclamp:
            w = (max(w[0], wclamp[0]), min(w[1], wclamp[1]))
        ch.append((v[0], v[1], w[0], w[1]))
    keep = [0, m - 1]
    while len(keep) < n:
        keep.sort()
        worst, wi = -1.0, None
        for a, b in zip(keep, keep[1:]):
            for i in range(a + 1, b):
                t = (xs[i] - xs[a]) / (xs[b] - xs[a])
                err = max(abs(ch[i][k] - (ch[a][k] + (ch[b][k] - ch[a][k]) * t)) for k in range(4))
                if err > worst:
                    worst, wi = err, i
        if wi is None:
            break
        keep.append(wi)
    return [xs[i] for i in sorted(keep)]


def plan_fit(f, bone, secs, pl, weight):
    fm = f.fr(bone)
    xs_lo, xs_hi, zs_lo, zs_hi = 9, -9, 9, -9
    for q in secs:
        at = q['at']
        px = fm['o'][0] + at[0] * fm['u'][0] + at[1] * fm['v'][0]
        xs_lo = min(xs_lo, px - q['ry'] * abs(fm['v'][0]))
        xs_hi = max(xs_hi, px + q['ry'] * abs(fm['v'][0]))
        pz = fm['o'][2] + at[2]
        zs_lo = min(zs_lo, pz - q['rx'])
        zs_hi = max(zs_hi, pz + q['rx'])
    ex = f.T.extent(pl, 0)
    ez = f.T.extent(pl, 1)
    if abs(fm['v'][0]) > 0.7 and ex:
        mine = (xs_hi - xs_lo) / 2
        plan = (ex[1] - ex[0]) / 2
        if mine > 0.01:
            k = min(1.2, max(0.55, 1 + weight * (plan / mine - 1)))
            for q in secs:
                q['ry'] = r5(q['ry'] * k)
    if ez:
        mine = (zs_hi - zs_lo) / 2
        plan = (ez[1] - ez[0]) / 2
        if mine > 0.01:
            k = min(1.2, max(0.55, 1 + weight * (plan / mine - 1)))
            for q in secs:
                q['rx'] = r5(q['rx'] * k)


class Human:
    def __init__(self, fig, cfg):
        self.f = fig
        self.c = cfg
        self.sc = fig.scale
        self.T = fig.T

    def pk(self, view, pats, roles=('body',)):
        return self.T.sel(self.sc, view, pats, roles=roles)

    def loft(self, id, bone, mat, tone, pe, pf, pp=None, nst=4, exp=2.0, ur=None, lo=0.0, hi=1.0, role='body', stations=None, bias=1.0, roles=('body',), cap=True, tags=None, trim=False, planfit=None, **kw):
        f = self.f
        e = self.pk('elevation', pe, roles)
        fr = self.pk('front', pf, roles) if pf else []
        pl = self.pk('plan', pp, roles) if pp else []
        f.vconst = None
        f.wconst = None
        if not e and pl and abs(f.fr(bone)['v'][0]) > 0.7:
            fm = f.fr(bone)
            ex = f.T.extent(pl, 0)
            vlo, vhi = ex[0] - fm['o'][0], ex[1] - fm['o'][0]
            f.vconst = (vlo, vhi) if fm['v'][0] > 0 else (-vhi, -vlo)
        if not e and pl and not f.vconst:
            ez0 = f.T.extent(pl, 1)
            hw = (ez0[1] - ez0[0]) / 2
            f.vconst = (-hw, hw)
        if not fr and pl:
            ez = f.T.extent(pl, 1)
            f.wconst = (ez[0] - f.fr(bone)['o'][2], ez[1] - f.fr(bone)['o'][2])
        if not e and not pl and fr:
            zf = f.T.extent(fr, 0)
            hw0 = (zf[1] - zf[0]) / 2
            f.vconst = (-hw0, hw0)
        if not e and not fr and not pl:
            return None
        if not e and not f.vconst:
            return None
        if ur is None:
            ur = f.urange(bone, e) if e else (0.0, f.T.bones[bone]['lengthM'])
            if trim and fr:
                fm = f.fr(bone)
                ey = [q[1] for sh in fr for r in sh['rings'] for q in r]
                if abs(fm['u'][1]) > 0.7:
                    us = [(y - fm['o'][1]) * fm['u'][1] for y in ey]
                    lo_f, hi_f = min(us), max(us)
                    a0, a1 = max(ur[0], lo_f), min(ur[1], hi_f)
                    if a1 - a0 > 0.7 * (ur[1] - ur[0]):
                        ur = (a0, a1)
        if stations is None:
            stations = auto_stations(f, bone, e, fr, pl, ur, nst, lo, hi, kw.get('vclamp'), kw.get('wclamp'))
        secs = f.sections(bone, stations, e, fr, pl, exp=exp, **kw)
        if len(secs) < 2:
            return None
        if planfit is None:
            planfit = {50: 0.5, 100: 0.6, 200: 0.8, 500: 1.0}[self.sc]
        if planfit and pl:
            plan_fit(f, bone, secs, pl, planfit)
        mr = kw.get('minr', 0.0)
        if mr:
            for q in secs:
                q['rx'] = r5(max(q['rx'], mr))
                q['ry'] = r5(max(q['ry'], mr))
        s = f.loft(id, bone, mat, tone, secs, role=role, cap=cap)
        if tags:
            s['tags'] = tags
        return s

    def blob(self, id, bone, mat, tone, pe, pf, pp=None, role='detail', depth=None, shift=(0, 0, 0), grow=1.0, minr=0.0):
        f = self.f
        e = self.pk('elevation', pe, ('body', 'detail')) if pe else []
        fr = self.pk('front', pf, ('body', 'detail')) if pf else []
        pl = self.pk('plan', pp, ('body', 'detail')) if pp else []
        if not (e or fr or pl):
            return None
        ex = self.T.extent(e, 0) if e else None
        ey = self.T.extent(e, 1) if e else None
        fz = self.T.extent(fr, 0) if fr else None
        fy = self.T.extent(fr, 1) if fr else None
        px = self.T.extent(pl, 0) if pl else None
        pz = self.T.extent(pl, 1) if pl else None
        xr = ex or px
        yr = ey or fy
        zr = fz or pz
        if xr is None or yr is None or zr is None:
            return None
        c = [(xr[0] + xr[1]) / 2, (yr[0] + yr[1]) / 2, (zr[0] + zr[1]) / 2]
        h = [(xr[1] - xr[0]) / 2, (yr[1] - yr[0]) / 2, (zr[1] - zr[0]) / 2]
        q = f.to_local(bone, c)
        fm = f.fr(bone)
        rad = [abs(h[0] * fm['u'][0]) + abs(h[1] * fm['u'][1]), abs(h[0] * fm['v'][0]) + abs(h[1] * fm['v'][1]), h[2]]
        if depth is not None:
            rad[1] = depth
        rad = [max(minr, r * grow) for r in rad]
        q = [q[i] + shift[i] for i in range(3)]
        return f.ellipsoid(id, bone, mat, tone, rad, q, role=role)


def ids2(pat, s):
    return [p.format(s=s) for p in pat] if isinstance(pat, list) else pat.format(s=s)


def _pca2(pts):
    n = len(pts)
    mx = sum(p[0] for p in pts) / n
    my = sum(p[1] for p in pts) / n
    sxx = sum((p[0] - mx) ** 2 for p in pts) / n
    syy = sum((p[1] - my) ** 2 for p in pts) / n
    sxy = sum((p[0] - mx) * (p[1] - my) for p in pts) / n
    th = 0.5 * math.atan2(2 * sxy, sxx - syy)
    return (mx, my), (math.cos(th), math.sin(th))


def capsule_from(H, id, bone, mat, tone, pats, w=0.0, role='body', rscale=0.9, lst='body', views=('elevation',)):
    f = H.f
    sh = H.pk('elevation', pats, ('body', 'detail'))
    if not sh:
        return None
    pts = []
    for s in sh:
        for r in s['rings']:
            for q in r:
                l = f.to_local(bone, [q[0], q[1], f.fr(bone)['o'][2]])
                pts.append((l[0], l[1]))
    c, d = _pca2(pts)
    ax = [(p[0] - c[0]) * d[0] + (p[1] - c[1]) * d[1] for p in pts]
    nm = (-d[1], d[0])
    ay = [(p[0] - c[0]) * nm[0] + (p[1] - c[1]) * nm[1] for p in pts]
    a0, a1 = min(ax), max(ax)
    rr = (max(ay) - min(ay)) / 2 * rscale
    cm = (max(ay) + min(ay)) / 2
    cx = c[0] + nm[0] * cm
    cy = c[1] + nm[1] * cm
    p0 = (cx + d[0] * (a0 + rr), cy + d[1] * (a0 + rr), w)
    p1 = (cx + d[0] * (a1 - rr), cy + d[1] * (a1 - rr), w)
    return f.capsule(id, bone, mat, tone, p0, p1, rr, rr * 0.8, role=role, lst=lst)


def region_layers(T, rid):
    for r in T.thermal['regions']:
        if r['id'] == rid:
            return r['layers']
    raise KeyError(rid)


def centre_of(f, solid, k=0):
    q = solid['sections'][k]['at']
    return q


def axis_pts(solid, a=0.1, b=0.9):
    secs = solid['sections']
    def at(t):
        n = len(secs) - 1
        x = t * n
        i = min(int(x), n - 1)
        fr = x - i
        p, q = secs[i], secs[i + 1]
        return [p['at'][k] + (q['at'][k] - p['at'][k]) * fr for k in range(3)], p['rx'] + (q['rx'] - p['rx']) * fr, p['ry'] + (q['ry'] - p['ry']) * fr
    return at(a), at(b)


def thermal_layers(fig, region, solids, layers):
    f = fig
    if not layers:
        for sid in solids:
            f.ref(f'th-{sid}', sid, region)
        return
    last = len(layers) - 1
    for k, layer in enumerate(layers):
        if f.scale > 50 and k != last:
            continue
        outside = sum(l.get('thicknessM') or 0.002 for l in layers[k + 1:])
        for sid in solids:
            if k == last:
                f.ref(f'th-{sid}' if last == 0 else f'th-{sid}-{layer["id"]}', sid, region, layer=layer['id'] if last else None)
            else:
                b = f.find(sid)
                c = shrink_solid(b, outside, f'th-{sid}-{layer["id"]}')
                c['region'] = region
                c['layer'] = layer['id']
                f.thermal.append(c)


def box_from(H, id, bone, mat, tone, pe, pf, pp, role='body', corner=0.01, lst='body', grow=0.0, tmin=0.0):
    f = H.f
    e = H.pk('elevation', pe, ('body', 'detail')) if pe else []
    fr = H.pk('front', pf, ('body', 'detail')) if pf else []
    pl = H.pk('plan', pp, ('body', 'detail')) if pp else []
    fm = f.fr(bone)
    zr = H.T.extent(fr, 0) if fr else (H.T.extent(pl, 1) if pl else None)
    if e:
        pts = [f.to_local(bone, [q[0], q[1], fm['o'][2]]) for sh in e for r in sh['rings'] for q in r]
        us = [p[0] for p in pts]
        vs = [p[1] for p in pts]
        ur, vr = (min(us), max(us)), (min(vs), max(vs))
    else:
        xr = H.T.extent(pl, 0) if pl else None
        yr = H.T.extent(fr, 1) if fr else None
        if not (xr and yr and zr):
            return None
        c = f.to_local(bone, [(xr[0] + xr[1]) / 2, (yr[0] + yr[1]) / 2, fm['o'][2]])
        h = [(xr[1] - xr[0]) / 2, (yr[1] - yr[0]) / 2]
        szu = 2 * (abs(fm['u'][0] * h[0]) + abs(fm['u'][1] * h[1]))
        szv = 2 * (abs(fm['v'][0] * h[0]) + abs(fm['v'][1] * h[1]))
        ur, vr = (c[0] - szu / 2, c[0] + szu / 2), (c[1] - szv / 2, c[1] + szv / 2)
    if zr is None:
        return None
    wc = (zr[0] + zr[1]) / 2 - fm['o'][2]
    size = [ur[1] - ur[0] + 2 * grow, vr[1] - vr[0] + 2 * grow, zr[1] - zr[0] + 2 * grow]
    at = [(ur[0] + ur[1]) / 2, (vr[0] + vr[1]) / 2, wc]
    for k in range(3):
        if size[k] < tmin:
            size[k] = tmin
    corner = min(corner, min(size) / 2 - 0.0005)
    return f.box(id, bone, mat, tone, size, at, role=role, lst=lst, corner=max(corner, 0.0005))
