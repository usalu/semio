"""NW-O scratch library: world-space authoring helpers for the new object assets. Shapes are authored by hand in metres in the
default pose (bone rest transforms are inverted to bone-local numbers); thermal/pose/clip helpers keep the bindings exhaustive.
No simplification algorithm: every shape of every scale is written in the per-asset scripts."""
import json, math, os, re

S = '/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage'
T_DIR = '/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE'
OBJ = S + '/🗂️catalog/🪑️objects'


def T(en, de):
    return {'en': en, 'de': de}


def r6(v):
    v = round(v + 0.0, 6)
    return 0.0 if v == 0 else v


def save(asset):
    d = OBJ + '/' + asset['id']
    os.makedirs(d, exist_ok=True)
    doc = {'$schema': '../../../🧬️schema/🔣️.json#/$defs/AssetDocument', 'version': 'semio.staffage/v2', 'asset': asset}
    with open(d + '/🔣️.json', 'w', encoding='utf-8') as f:
        json.dump(doc, f, ensure_ascii=False, indent=2)
        f.write('\n')


# ---------- transforms
def mul(a, b):
    return [[a[0][0] * b[0][0] + a[0][1] * b[1][0], a[0][0] * b[0][1] + a[0][1] * b[1][1], a[0][0] * b[0][2] + a[0][1] * b[1][2] + a[0][2]],
            [a[1][0] * b[0][0] + a[1][1] * b[1][0], a[1][0] * b[0][1] + a[1][1] * b[1][1], a[1][0] * b[0][2] + a[1][1] * b[1][2] + a[1][2]]]


def trs(x, y, deg):
    c, s = math.cos(math.radians(deg)), math.sin(math.radians(deg))
    return [[c, -s, x], [s, c, y]]


def inv(m):
    d = m[0][0] * m[1][1] - m[0][1] * m[1][0]
    a, b, c, e = m[1][1] / d, -m[0][1] / d, -m[1][0] / d, m[0][0] / d
    return [[a, b, -(a * m[0][2] + b * m[1][2])], [c, e, -(c * m[0][2] + e * m[1][2])]]


def apply(m, x, y):
    return (m[0][0] * x + m[0][1] * y + m[0][2], m[1][0] * x + m[1][1] * y + m[1][2])


class Rig:
    """Bones are declared by their WORLD pivot (rest pose, all rest angles 0); local coordinates follow."""

    def __init__(self, kind, root):
        self.kind, self.root, self.bones, self.wp = kind, root, [], {}

    def bone(self, bid, parent, px, py, length, joint, title, mass=None):
        ppx, ppy = self.wp.get(parent, (0, 0)) if parent else (0, 0)
        self.wp[bid] = (px, py)
        b = {'id': bid, 'parent': parent, 'title': title, 'x': r6(px - ppx), 'y': r6(py - ppy), 'angleDeg': 0, 'lengthM': length, 'joint': joint}
        if mass:
            kg, cx, cy, src, ref = mass
            b['mass'] = {'kg': kg, 'comX': r6(cx - px), 'comY': r6(cy - py), 'distribution': 'point', 'source': src, **({'reference': ref} if ref else {})}
        self.bones.append(b)

    def rig(self):
        return {'kind': self.kind, 'root': self.root, 'bones': self.bones}

    def total_mass(self):
        return sum(b.get('mass', {}).get('kg', 0) for b in self.bones)

    def to_local(self, bid, x, y):
        px, py = self.wp[bid]
        return x - px, y - py

    def anchor(self, aid, bone, x, y, kind, title, **kw):
        px, py = self.wp[bone]
        a = {'id': aid, 'bone': bone, 'x': r6(x - px), 'y': r6(y - py), 'kind': kind, 'title': title}
        a.update(kw)
        return a


FIXED = {'type': 'fixed'}


def revolute(lo, hi):
    return {'type': 'revolute', 'minDeg': lo, 'maxDeg': hi}


def prismatic(lo, hi, axis=90):
    return {'type': 'prismatic', 'minM': lo, 'maxM': hi, 'axisDeg': axis}


# ---------- shapes (world metres)
def _path_shift(d, dx, dy):
    toks = re.findall(r'[A-Za-z]|-?\d*\.?\d+(?:e-?\d+)?', d)
    arity = {'M': 2, 'L': 2, 'T': 2, 'C': 6, 'Q': 4, 'S': 4, 'H': 1, 'V': 1, 'Z': 0, 'A': 7}
    out, i = [], 0
    while i < len(toks):
        c = toks[i]
        i += 1
        assert c in arity, c
        out.append(c)
        n = arity[c]
        while n and i < len(toks) and not toks[i].isalpha():
            vals = [float(v) for v in toks[i:i + n]]
            i += n
            if c == 'A':
                vals[5] += dx
                vals[6] += dy
            elif c == 'H':
                vals[0] += dx
            elif c == 'V':
                vals[0] += dy
            else:
                vals = [v + (dx if k % 2 == 0 else dy) for k, v in enumerate(vals)]
            out.append(' '.join(f'{r6(v):g}' if not (c == 'A' and k in (2, 3, 4)) else f'{v:g}' for k, v in enumerate(vals)))
            if n == 0:
                break
    return ' '.join(out)


class Shapes:
    def __init__(self, rig):
        self.rig, self.items, self.ids = rig, [], set()

    def _add(self, sid, bone, role, z, filled, tone, geo, region=None, regions=None, material=None, hatch=None, pen=None):
        assert sid not in self.ids, sid
        assert (region is None) != (regions is None) or (region is None and regions is None), sid
        self.ids.add(sid)
        s = {'id': sid, 'bone': bone, 'role': role, 'z': z, 'filled': filled, 'tone': tone}
        s.update(geo)
        if region:
            s['region'] = region
        if regions:
            s['regions'] = regions
        if material:
            s['material'] = material
        if hatch:
            s['hatch'] = hatch
        if pen is not None:
            s['penMm'] = pen
        self.items.append(s)

    def poly(self, sid, bone, pts, closed=True, role='body', z=3, filled=True, tone=0.5, **kw):
        loc = [[r6(v) for v in self.rig.to_local(bone, x, y)] for x, y in pts]
        self._add(sid, bone, role, z, filled and closed, tone, {'kind': 'poly', 'points': loc, 'closed': closed}, **kw)

    def line(self, sid, bone, pts, role='body', z=4, tone=0.3, **kw):
        self.poly(sid, bone, pts, closed=False, role=role, z=z, filled=False, tone=tone, **kw)

    def path(self, sid, bone, d, role='body', z=3, filled=True, tone=0.5, **kw):
        px, py = self.rig.wp[bone]
        self._add(sid, bone, role, z, filled, tone, {'kind': 'path', 'd': _path_shift(d, -px, -py)}, **kw)

    def rect(self, sid, bone, x0, y0, x1, y1, r=0, role='body', z=3, filled=True, tone=0.5, **kw):
        lx, ly = self.rig.to_local(bone, x0, y0)
        geo = {'kind': 'rect', 'x': r6(lx), 'y': r6(ly), 'w': r6(x1 - x0), 'h': r6(y1 - y0)}
        if r:
            geo['r'] = r6(r)
        self._add(sid, bone, role, z, filled, tone, geo, **kw)

    def ell(self, sid, bone, cx, cy, rx, ry=None, role='body', z=3, filled=True, tone=0.5, rot=0, **kw):
        lx, ly = self.rig.to_local(bone, cx, cy)
        geo = {'kind': 'ellipse', 'cx': r6(lx), 'cy': r6(ly), 'rx': r6(rx), 'ry': r6(ry if ry is not None else rx)}
        if rot:
            geo['rotationDeg'] = rot
        self._add(sid, bone, role, z, filled, tone, geo, **kw)

    def cap(self, sid, bone, x0, y0, x1, y1, r0, r1=None, role='body', z=3, filled=True, tone=0.5, **kw):
        a, b = self.rig.to_local(bone, x0, y0), self.rig.to_local(bone, x1, y1)
        geo = {'kind': 'capsule', 'x0': r6(a[0]), 'y0': r6(a[1]), 'x1': r6(b[0]), 'y1': r6(b[1]), 'r0': r6(r0), 'r1': r6(r0 if r1 is None else r1)}
        self._add(sid, bone, role, z, filled, tone, geo, **kw)


# ---------- thermal
def layer(lid, title, kind, material, t, k, rho, cp, source='authored-estimate', reference=None, vap=None, area_factor=1.0):
    alpha = k / (rho * cp)
    active = min(t, math.sqrt(alpha * 3600))
    d = {'id': lid, 'title': title, 'kind': kind, 'material': material, 'thicknessM': t, 'resistanceM2KW': r6(t / k), 'heatCapacityJM2K': round(rho * cp * active * area_factor), 'source': source}
    if vap:
        d['vapourResistanceM2PaW'] = vap
    if reference:
        d['reference'] = reference
    return d


def region(rid, title, kind, model, material, area, L, geom, svf, layers, source='authored-estimate', reference=None, operating=None, hidden=None, cap=None):
    d = {'id': rid, 'title': title, 'kind': kind, 'model': model, 'material': material, 'areaM2': area, 'characteristicLengthM': L, 'geometry': geom,
         'skyViewFactor': svf, 'layers': layers, 'heatCapacityJM2K': cap if cap is not None else sum(l['heatCapacityJM2K'] for l in layers)}
    if operating:
        d['operating'] = {'key': operating[0], 'heatW': operating[1]}
    if hidden:
        d['hidden'] = hidden
    d['source'] = source
    if reference:
        d['reference'] = reference
    return d


def material(mid, en, de, eps, absorb, rho, k, cp, source='authored-estimate', reference=None, E=None):
    m = {'id': mid, 'title': T(en, de), 'emissivity': eps, 'solarAbsorptivity': absorb, 'densityKgM3': rho, 'conductivityWmK': k, 'specificHeatJKgK': cp}
    if E:
        m['elasticModulusPa'] = E
    m['source'] = source
    if reference:
        m['reference'] = reference
    return m


def bind(regions, sun=0.0, sunmap=None, exposed=None, emitters=None, operating=None, contact=None, wet=None):
    """Exhaustive thermal binding: every region (default exposed 1, sunlit `sun`, per-region overrides), emitters (default 0 for the ids listed in `emitters` dict), operating levels."""
    out = {}
    for rid in regions:
        b = {'exposedFraction': (exposed or {}).get(rid, 1.0), 'sunlitFraction': (sunmap or {}).get(rid, sun)}
        if wet and rid in wet:
            b['wettedness'] = wet[rid]
        if contact and rid in contact:
            b['contact'] = contact[rid]
        out[rid] = b
    t = {'regions': out, 'emitters': dict(emitters or {})}
    if operating is not None:
        t['operating'] = dict(operating)
    return t


def pose(pid, title, desc, bones, contacts, thermal, loads=None, key=False, ground='auto'):
    p = {'id': pid, 'title': title, 'description': desc, 'bones': bones, 'ground': ground, 'contacts': contacts, 'thermal': thermal, 'loads': loads or []}
    if key:
        p['key'] = True
    return p


def load(lid, title, kind, fx, fy, source, reference=None, anchor=None, bone=None, x=None, y=None, moment=None, distributed=None):
    d = {'id': lid, 'kind': kind, 'title': title, 'fxN': fx, 'fyN': fy, 'source': source}
    if anchor:
        d['anchor'] = anchor
    if bone:
        d['bone'] = bone
    if x is not None:
        d['x'] = x
    if y is not None:
        d['y'] = y
    if moment is not None:
        d['momentNm'] = moment
    if distributed:
        d['distributed'] = distributed
    if reference:
        d['reference'] = reference
    return d


def clip(cid, title, desc, dur, fps, loop, keys, contacts, phases, events, tracks, rootMotion='none'):
    return {'id': cid, 'title': title, 'description': desc, 'durationS': dur, 'fps': fps, 'loop': loop, 'rootMotion': rootMotion,
            'keys': [{'t': t, 'pose': p, 'ease': e, **({'bones': b} if b else {})} for (t, p, e, b) in [k + (None,) * (4 - len(k)) for k in keys]],
            'contacts': [{'anchor': a, 'fromS': f, 'toS': t} for a, f, t in contacts],
            'phases': [{'id': i, 'fromS': f, 'toS': t, 'title': ti} for i, f, t, ti in phases],
            'events': [{'t': t, 'kind': k, 'title': ti, **({'target': tg} if tg else {})} for t, k, ti, tg in events],
            'tracks': [{'target': tg, 'ease': e, 'keys': [{'t': t, 'v': v} for t, v in ks]} for tg, e, ks in tracks]}


def pens(body, detail, texture, glazing, ground):
    return {'body': body, 'detail': detail, 'texture': texture, 'glazing': glazing, 'ground': ground}


def rep(denominator, lo, hi, pens_, sh, decisions, hatches=None):
    keys = ['silhouette', 'retained', 'merged', 'omitted', 'thinElements', 'hatching', 'styles', 'overlays']
    assert len(decisions) == 8
    b = bounds(sh.items, sh.rig)
    print(f'  1:{denominator} bounds w={b[1]-b[0]:.3f} h={b[3]-b[2]:.3f} x={b[0]:.3f}..{b[1]:.3f} y={b[2]:.3f}..{b[3]:.3f}')
    return {'status': 'authored', 'denominator': denominator, 'validRange': {'minDenominator': lo, 'maxDenominator': hi}, 'pens': pens_, 'hatches': hatches or [],
            'geometry': {'elevation': sh.items}, 'decisions': {k: T(*v) for k, v in zip(keys, decisions)}}


def unavailable(denominator, reason, feature, size, alts):
    return {'status': 'unavailable', 'denominator': denominator, 'reason': T(*reason), 'limitingFeature': T(*feature), 'limitingSizeM': size, 'alternatives': alts}


def hatch(hid, en, de, pattern, angle, spacing, pen):
    return {'id': hid, 'title': T(en, de), 'pattern': pattern, 'angleDeg': angle, 'spacingMm': spacing, 'penMm': pen}


def ref(rid, citation, url=None):
    d = {'id': rid, 'citation': citation}
    if url:
        d['url'] = url
    return d


ENV_EXPORTS = ['svg', 'pdf', 'dxf', 'dwg', 'png-sequence', 'mp4', 'webm', 'component']


def structure(supports, assumptions):
    return {'gravityMS2': 9.80665, 'dynamics': 'quasi-static', 'supports': supports, 'assumptions': [T(*a) for a in assumptions], 'clinical': False}


def environment(air, mrt, rh, wind, solar, diffuse=0.15, sky=None, ground=None, wdir=0):
    e = {'airTempC': air, 'meanRadiantTempC': mrt, 'relativeHumidity': rh, 'windMS': wind, 'windDirectionDeg': wdir, 'solarWM2': solar, 'diffuseFraction': diffuse, 'pressurePa': 101325}
    if sky is not None:
        e['skyTempC'] = sky
    if ground is not None:
        e['groundTempC'] = ground
    return e


def bounds(items, rig):
    xs, ys = [], []
    for it in items:
        px, py = rig.wp[it['bone']]
        k = it['kind']
        if k == 'rect':
            xs += [it['x'] + px, it['x'] + it['w'] + px]; ys += [it['y'] + py, it['y'] + it['h'] + py]
        elif k == 'ellipse':
            xs += [it['cx'] + px - it['rx'], it['cx'] + px + it['rx']]; ys += [it['cy'] + py - it['ry'], it['cy'] + py + it['ry']]
        elif k == 'poly':
            for x, y in it['points']:
                xs.append(x + px); ys.append(y + py)
        elif k == 'capsule':
            xs += [it['x0'] + px - it['r0'], it['x0'] + px + it['r0'], it['x1'] + px - it['r1'], it['x1'] + px + it['r1']]
            ys += [it['y0'] + py - it['r0'], it['y0'] + py + it['r0'], it['y1'] + py - it['r1'], it['y1'] + py + it['r1']]
        elif k == 'path':
            n = [float(v) for v in re.findall(r'-?\d*\.?\d+(?:e-?\d+)?', it['d'])]
            toks = re.findall(r'[A-Za-z]|-?\d*\.?\d+(?:e-?\d+)?', it['d'])
            cur = None
            vals = []
            for t in toks:
                if t.isalpha():
                    cur = t; continue
                vals.append((cur, float(t)))
            # crude: treat numbers pairwise except H/V
            flat = [v for c, v in vals if c not in 'HVA']
            for i in range(0, len(flat) - 1, 2):
                xs.append(flat[i] + px); ys.append(flat[i + 1] + py)
    return min(xs), max(xs), min(ys), max(ys)
