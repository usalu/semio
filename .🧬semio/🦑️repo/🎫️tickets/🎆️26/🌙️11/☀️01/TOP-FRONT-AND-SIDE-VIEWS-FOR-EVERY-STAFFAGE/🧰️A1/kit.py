import math
from lib import *

def r4(v): return round(v, 4)

class K:
    def __init__(s, a, scale):
        s.d = a; s.sc = scale; s.W = rest(a); s.B = {b['id']: b for b in a['rig']['bones']}
        s.elev = a['representations'][str(scale)]['geometry']['elevation']
        s.tone = {}
        for e in s.elev: s.tone.setdefault(e['bone'], e['tone'])
        s.shapes = []

    def o(s, b): return ap(s.W[b], 0, 0)
    def t(s, b): return ap(s.W[b], s.B[b]['lengthM'], 0)

    def base(s, id, bone, reg, z, role, filled, tone):
        d = {'id': id, 'bone': bone, 'role': role, 'z': z, 'filled': filled, 'tone': s.tone.get(bone, 0.5) if tone is None else tone}
        if isinstance(reg, (list, tuple)): d['regions'] = list(reg)
        elif reg: d['region'] = reg
        return d

    def add(s, d):
        assert all(x['id'] != d['id'] for x in s.shapes), d['id']
        s.shapes.append(d); return d

    def cap(s, id, bone, l0, l1, r0, r1, reg, z, x0=None, x1=None, dx0=0, dx1=0, follow=True, tone=None, role='body'):
        d = s.base(id, bone, reg, z, role, True, tone)
        X0 = (s.o(bone)[0] if x0 is None else x0) + dx0
        X1 = (s.t(bone)[0] if x1 is None else x1) + dx1
        d.update(kind='capsule', x0=r4(X0), y0=r4(l0), x1=r4(X1), y1=r4(l1), r0=r4(r0), r1=r4(r1))
        if follow: d['followTip'] = True
        return s.add(d)

    def col(s, id, bone, l0, l1, w0, w1, reg, z, y0=None, y1=None, dy0=0, dy1=0, follow=True, tone=None, cap0=0, cap1=0, role='body'):
        d = s.base(id, bone, reg, z, role, True, tone)
        Y0 = (s.o(bone)[1] if y0 is None else y0) + dy0
        Y1 = (s.t(bone)[1] if y1 is None else y1) + dy1
        sg = 1 if Y1 >= Y0 else -1
        left = [(l0 - w0, Y0), (l1 - w1, Y1)]; right = [(l1 + w1, Y1), (l0 + w0, Y0)]
        pts = [left[0]]
        pts.append(left[1])
        if cap1 > 0:
            for i in range(1, 6):
                a = math.pi * i / 6
                pts.append((l1 - w1 * math.cos(a), Y1 + sg * cap1 * math.sin(a)))
        pts += right
        if cap0 > 0:
            for i in range(1, 6):
                a = math.pi * i / 6
                pts.append((l0 + w0 * math.cos(a), Y0 - sg * cap0 * math.sin(a)))
        d.update(kind='poly', points=[[r4(x), r4(y)] for x, y in pts], closed=True)
        if follow: d['followTip'] = True
        return s.add(d)

    def poly(s, id, bone, pts, reg, z, closed=True, filled=True, follow=False, tone=None, role='body'):
        d = s.base(id, bone, reg, z, role, filled, tone)
        d.update(kind='poly', points=[[r4(x), r4(y)] for x, y in pts], closed=closed)
        if follow: d['followTip'] = True
        return s.add(d)

    def ell(s, id, bone, cx, cy, rx, ry, reg, z, rot=0, tone=None, role='body', filled=True):
        d = s.base(id, bone, reg, z, role, filled, tone)
        d.update(kind='ellipse', cx=r4(cx), cy=r4(cy), rx=r4(rx), ry=r4(ry))
        if rot: d['rotationDeg'] = rot
        return s.add(d)

    def rect(s, id, bone, x, y, w, h, reg, z, rad=0, follow=False, tone=None, role='body'):
        d = s.base(id, bone, reg, z, role, True, tone)
        d.update(kind='rect', x=r4(x), y=r4(y), w=r4(w), h=r4(h))
        if rad: d['r'] = r4(rad)
        if follow: d['followTip'] = True
        return s.add(d)

def put(a, tier_views, extra):
    pass

def finish(doc, results, texts):
    """results: {scale: {'front': [...], 'plan': [...]}}; texts: {scale: (en, de)}"""
    a = doc['asset']
    for sc, views in results.items():
        rep = a['representations'][str(sc)]
        g = rep['geometry']
        new = {}
        for k, v in g.items():
            new[k] = v
            if k == 'elevation':
                new['front'] = views['front']; new['plan'] = views['plan']
        for k in ('front', 'plan'): new[k] = views[k]
        rep['geometry'] = new
        en, de = texts[sc]
        sil = rep['decisions']['silhouette']
        for lang, add, mark in (('en', en, ' Front and plan:'), ('de', de, ' Vorderansicht und Grundriss:')):
            base = sil[lang].split(mark)[0]
            sil[lang] = base + mark + ' ' + add
    pres = a['presentation']
    pres.pop('viewpoints', None)
    return doc
