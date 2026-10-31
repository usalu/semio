"""CV-FO scratch library: rest-pose bone transforms, world-space shape builders (coordinates are authored in metres in the default pose
and converted to bone-local numbers), thermal builders, representation builder and an SVG/PNG preview. No generic simplification: every
shape is authored by hand in the per-asset scripts."""
import json, math, os, re, subprocess, sys

S = '/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage'
T_DIR = '/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE'
GEN = T_DIR + '/🗑️generated/CV-FO'
os.makedirs(GEN, exist_ok=True)
FOLDER = {'furniture': '🛋️furniture', 'objects': '🪑️objects'}


def T(en, de):
    return {'en': en, 'de': de}


def r6(v):
    v = round(v + 0.0, 6)
    return 0.0 if v == 0 else v


def load(category, asset_id):
    path = f'{S}/🗂️catalog/{FOLDER[category]}/{asset_id}/🔣️.json'
    src = os.path.join(os.path.dirname(os.path.abspath(__file__)), 'v1', asset_id + '.json')
    with open(src if os.path.exists(src) else path, encoding='utf-8') as f:
        return path, json.load(f)['asset']


def save(path, asset):
    asset['thermal'].setdefault('passengers', [])
    for r in asset['thermal']['regions']:
        if r['id'] == 'canopy-underside':
            r['hidden'] = T('The underside of the canopy faces the ground and is hidden behind the upper fabric in every elevation; its state is read numerically.', 'Die Unterseite der Bespannung ist dem Boden zugewandt und in jeder Ansicht hinter dem Oberstoff verdeckt; ihr Zustand wird numerisch gelesen.')
    doc = {'$schema': '../../../🧬️schema/🔣️.json#/$defs/AssetDocument', 'version': 'semio.staffage/v2', 'asset': asset}
    with open(path, 'w', encoding='utf-8') as f:
        json.dump(doc, f, ensure_ascii=False, indent=2)
        f.write('\n')


#region rest-pose transforms
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
    def __init__(self, bones, pose_bones=None):
        self.bones = {b['id']: b for b in bones}
        self.pose = pose_bones or {}
        self.cache = {}

    def world(self, bid):
        if bid in self.cache:
            return self.cache[bid]
        b = self.bones[bid]
        o = self.pose.get(bid, {})
        m = trs(b['x'] + o.get('dx', 0), b['y'] + o.get('dy', 0), b['angleDeg'] + o.get('angleDeg', 0))
        if b['parent']:
            m = mul(self.world(b['parent']), m)
        self.cache[bid] = m
        return m

    def local(self, bid, x, y):
        return apply(inv(self.world(bid)), x, y)

    def rot(self, bid):
        m = self.world(bid)
        return math.degrees(math.atan2(m[1][0], m[0][0]))
#endregion


#region shape builders (world metres, default pose)
class Shapes:
    def __init__(self, rig):
        self.rig = rig
        self.items = []
        self.ids = set()

    def _add(self, sid, bone, role, z, filled, tone, geo, region=None, regions=None, material=None, hatch=None, penMm=None):
        assert sid not in self.ids, sid
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
        if penMm is not None:
            s['penMm'] = penMm
        self.items.append(s)
        return s

    def poly(self, sid, bone, pts, closed=True, role='body', z=3, filled=True, tone=0.5, **kw):
        loc = [[r6(v) for v in self.rig.local(bone, x, y)] for x, y in pts]
        return self._add(sid, bone, role, z, filled and closed, tone, {'kind': 'poly', 'points': loc, 'closed': closed}, **kw)

    def line(self, sid, bone, pts, role='detail', z=4, tone=0.3, **kw):
        return self.poly(sid, bone, pts, closed=False, role=role, z=z, filled=False, tone=tone, **kw)

    def path(self, sid, bone, cmds, role='body', z=3, filled=True, tone=0.5, **kw):
        out = []
        for c in cmds:
            op, vals = c[0], c[1:]
            if op == 'Z':
                out.append('Z')
                continue
            pts = []
            for i in range(0, len(vals), 2):
                lx, ly = self.rig.local(bone, vals[i], vals[i + 1])
                pts.append(f'{r6(lx):g} {r6(ly):g}')
            out.append(op + ' ' + ' '.join(pts))
        return self._add(sid, bone, role, z, filled, tone, {'kind': 'path', 'd': ' '.join(out)}, **kw)

    def rect(self, sid, bone, x0, y0, x1, y1, r=0, role='body', z=3, filled=True, tone=0.5, **kw):
        ang = self.rig.rot(bone) % 360
        if min(ang, 360 - ang) < 1e-6:
            lx, ly = self.rig.local(bone, x0, y0)
            geo = {'kind': 'rect', 'x': r6(lx), 'y': r6(ly), 'w': r6(x1 - x0), 'h': r6(y1 - y0)}
            if r:
                geo['r'] = r6(r)
            return self._add(sid, bone, role, z, filled, tone, geo, **kw)
        return self.poly(sid, bone, [(x0, y0), (x1, y0), (x1, y1), (x0, y1)], role=role, z=z, filled=filled, tone=tone, **kw)

    def ell(self, sid, bone, cx, cy, rx, ry=None, role='body', z=3, filled=True, tone=0.5, rot=0, **kw):
        lx, ly = self.rig.local(bone, cx, cy)
        geo = {'kind': 'ellipse', 'cx': r6(lx), 'cy': r6(ly), 'rx': r6(rx), 'ry': r6(ry if ry is not None else rx)}
        if rot:
            geo['rotationDeg'] = rot - self.rig.rot(bone)
        return self._add(sid, bone, role, z, filled, tone, geo, **kw)

    def cap(self, sid, bone, x0, y0, x1, y1, r0, r1=None, role='body', z=3, filled=True, tone=0.5, **kw):
        a = self.rig.local(bone, x0, y0)
        b = self.rig.local(bone, x1, y1)
        geo = {'kind': 'capsule', 'x0': r6(a[0]), 'y0': r6(a[1]), 'x1': r6(b[0]), 'y1': r6(b[1]), 'r0': r6(r0), 'r1': r6(r0 if r1 is None else r1)}
        return self._add(sid, bone, role, z, filled, tone, geo, **kw)
#endregion


#region thermal helpers
def layer(lid, title, kind, material, t, k, cap=None, vap=None, source='literature', reference=None):
    d = {'id': lid, 'title': title, 'kind': kind, 'material': material, 'thicknessM': t, 'resistanceM2KW': r6(t / k) if t else 0, 'source': source}
    if cap is not None:
        d['heatCapacityJM2K'] = cap
    if vap is not None:
        d['vapourResistanceM2PaW'] = vap
    if reference:
        d['reference'] = reference
    return d


def region(rid, title, kind, model, material, area, L, geom, svf, layers, cap, source='authored-estimate', reference=None, operating=None):
    d = {'id': rid, 'title': title, 'kind': kind, 'model': model, 'material': material, 'areaM2': area, 'characteristicLengthM': L, 'geometry': geom,
         'skyViewFactor': svf, 'layers': layers, 'heatCapacityJM2K': cap}
    if operating:
        d['operating'] = operating
    d['source'] = source
    if reference:
        d['reference'] = reference
    return d


def binding(regions, emitters=None, operating=None, base=None, over=None):
    """regions: dict rid -> (exposed, sunlit) or full dict; over: dict rid -> partial overrides."""
    out = {}
    for rid, v in regions.items():
        out[rid] = {'exposedFraction': v[0], 'sunlitFraction': v[1]} if isinstance(v, tuple) else dict(v)
    for rid, o in (over or {}).items():
        out[rid].update(o)
    t = {'regions': out, 'emitters': emitters or {}}
    if operating:
        t['operating'] = operating
    return t
#endregion


#region representations
PENS = {
    '500': dict(body=0.18, detail=None, texture=None, glazing=None, ground=0.25),
    '200': dict(body=0.25, detail=0.13, texture=None, glazing=0.13, ground=0.35),
    '100': dict(body=0.35, detail=0.18, texture=0.13, glazing=0.13, ground=0.5),
    '50': dict(body=0.5, detail=0.25, texture=0.13, glazing=0.18, ground=0.7),
}
RANGES = {'500': (350, 1000), '200': (150, 350), '100': (75, 150), '50': (20, 75)}


def rep(den, shapes, decisions, pens=None, hatches=None, valid=None):
    k = str(den)
    d = decisions
    keys = ['silhouette', 'retained', 'merged', 'omitted', 'thinElements', 'hatching', 'styles', 'overlays']
    assert set(d) == set(keys), set(keys) ^ set(d)
    lo, hi = valid or RANGES[k]
    return {'status': 'authored', 'denominator': int(den), 'validRange': {'minDenominator': lo, 'maxDenominator': hi},
            'pens': pens or PENS[k], 'hatches': hatches or [], 'geometry': {'elevation': shapes.items if isinstance(shapes, Shapes) else shapes},
            'decisions': d}


def unavailable(den, reason, feature, size, alternatives):
    return {'status': 'unavailable', 'denominator': int(den), 'reason': reason, 'limitingFeature': feature, 'limitingSizeM': size, 'alternatives': alternatives}
#endregion


#region preview
def sample_path(d, n=12):
    toks = re.findall(r'[MLCQZ]|-?\d*\.?\d+(?:e-?\d+)?', d)
    i, subs, cur = 0, [], []
    pos = (0, 0)
    op = None
    while i < len(toks):
        t = toks[i]
        if t in 'MLCQZ':
            op = t
            i += 1
            if op == 'Z':
                if cur:
                    cur.append(cur[0])
                    subs.append((cur, True))
                    cur = []
                continue
        nargs = {'M': 2, 'L': 2, 'C': 6, 'Q': 4}[op]
        v = [float(x) for x in toks[i:i + nargs]]
        i += nargs
        if op == 'M':
            if cur:
                subs.append((cur, False))
            cur = [(v[0], v[1])]
            pos = cur[0]
            op = 'L'
        elif op == 'L':
            cur.append((v[0], v[1]))
            pos = cur[-1]
        elif op == 'C':
            for k in range(1, n + 1):
                u = k / n
                x = (1 - u) ** 3 * pos[0] + 3 * (1 - u) ** 2 * u * v[0] + 3 * (1 - u) * u * u * v[2] + u ** 3 * v[4]
                y = (1 - u) ** 3 * pos[1] + 3 * (1 - u) ** 2 * u * v[1] + 3 * (1 - u) * u * u * v[3] + u ** 3 * v[5]
                cur.append((x, y))
            pos = (v[4], v[5])
        elif op == 'Q':
            for k in range(1, n + 1):
                u = k / n
                x = (1 - u) ** 2 * pos[0] + 2 * (1 - u) * u * v[0] + u * u * v[2]
                y = (1 - u) ** 2 * pos[1] + 2 * (1 - u) * u * v[1] + u * u * v[3]
                cur.append((x, y))
            pos = (v[2], v[3])
    if cur:
        subs.append((cur, False))
    return subs


def outline(s, n=14):
    """Local polylines of a shape: list of (points, closed)."""
    k = s['kind']
    if k == 'poly':
        return [([tuple(p) for p in s['points']], s['closed'])]
    if k == 'path':
        return sample_path(s['d'], n)
    if k == 'rect':
        x, y, w, h = s['x'], s['y'], s['w'], s['h']
        return [([(x, y), (x + w, y), (x + w, y + h), (x, y + h)], True)]
    if k == 'ellipse':
        rot = math.radians(s.get('rotationDeg', 0))
        pts = []
        for i in range(36):
            a = 2 * math.pi * i / 36
            ex, ey = s['rx'] * math.cos(a), s['ry'] * math.sin(a)
            pts.append((s['cx'] + ex * math.cos(rot) - ey * math.sin(rot), s['cy'] + ex * math.sin(rot) + ey * math.cos(rot)))
        return [(pts, True)]
    if k == 'capsule':
        x0, y0, x1, y1, r0, r1 = s['x0'], s['y0'], s['x1'], s['y1'], s['r0'], s['r1']
        a = math.atan2(y1 - y0, x1 - x0)
        pts = []
        for i in range(19):
            t = a + math.pi / 2 + math.pi * i / 18
            pts.append((x1 + r1 * math.cos(t - math.pi), y1 + r1 * math.sin(t - math.pi)))
        pts = []
        for i in range(0, 19):
            t = a - math.pi / 2 + math.pi * i / 18
            pts.append((x1 + r1 * math.cos(t), y1 + r1 * math.sin(t)))
        for i in range(0, 19):
            t = a + math.pi / 2 + math.pi * i / 18
            pts.append((x0 + r0 * math.cos(t), y0 + r0 * math.sin(t)))
        return [(pts, True)]
    raise ValueError(k)


def world_outline(rig, s):
    m = rig.world(s['bone'])
    return [([apply(m, x, y) for x, y in pts], closed) for pts, closed in outline(s)]


def bbox_world(rig, shapes):
    xs, ys = [], []
    for s in shapes:
        for pts, _ in world_outline(rig, s):
            for x, y in pts:
                xs.append(x)
                ys.append(y)
    return min(xs), min(ys), max(xs), max(ys)


def svg(rep_, rig, den, zoom=1.0, shade=None, margin_mm=2.0, clip=None):
    """SVG in paper millimetres at drawing scale 1:den; rep_ is an authored representation."""
    k = 1000.0 / den
    shapes = rep_['geometry']['elevation']
    x0, y0, x1, y1 = bbox_world(rig, shapes)
    if clip:
        x0, y0, x1, y1 = clip
    pens = rep_['pens']
    hatches = {h['id']: h for h in rep_['hatches']}
    W, H = (x1 - x0) * k + 2 * margin_mm, (y1 - min(y0, 0)) * k + 2 * margin_mm
    out = [f'<svg xmlns="http://www.w3.org/2000/svg" width="{W * zoom:.3f}mm" height="{H * zoom:.3f}mm" viewBox="0 0 {W:.3f} {H:.3f}">',
           '<rect width="100%" height="100%" fill="white"/>']
    for h in hatches.values():
        out.append(f'<pattern id="h-{h["id"]}" patternUnits="userSpaceOnUse" width="{h["spacingMm"]}" height="{h["spacingMm"]}" patternTransform="rotate({-h["angleDeg"]})">'
                   f'<line x1="0" y1="0" x2="{h["spacingMm"]}" y2="0" stroke="black" stroke-width="{h["penMm"]}"/></pattern>')

    def P(x, y):
        return (margin_mm + (x - x0) * k, H - margin_mm - (y - min(y0, 0)) * k)
    gy = P(0, 0)[1]
    out.append(f'<line x1="0" y1="{gy:.3f}" x2="{W:.3f}" y2="{gy:.3f}" stroke="#888" stroke-width="{pens["ground"]}"/>')
    for s in sorted(shapes, key=lambda s: s['z']):
        role = s['role']
        pen = s.get('penMm', pens.get(role))
        if pen is None:
            continue
        tone = int(255 * s['tone'])
        fill = f'rgb({tone},{tone},{tone})' if s['filled'] else 'none'
        if role == 'glazing':
            fill = 'rgb(225,235,245)' if s['filled'] else 'none'
        d = ''
        for pts, closed in world_outline(rig, s):
            pp = [P(x, y) for x, y in pts]
            d += 'M ' + ' L '.join(f'{x:.3f} {y:.3f}' for x, y in pp) + (' Z' if closed else '') + ' '
        out.append(f'<path d="{d}" fill="{fill}" fill-rule="nonzero" stroke="black" stroke-width="{pen}" stroke-linejoin="round" stroke-linecap="round"/>')
        if s.get('hatch') and s['hatch'] in hatches and s['filled']:
            out.append(f'<path d="{d}" fill="url(#h-{s["hatch"]})" stroke="none"/>')
    out.append('</svg>')
    return '\n'.join(out)


def render(rep_, rig, den, name, dpi=600, zoom=1.0, clip=None):
    s = svg(rep_, rig, den, zoom=zoom, clip=clip)
    p = f'{GEN}/{name}.svg'
    with open(p, 'w', encoding='utf-8') as f:
        f.write(s)
    q = f'{GEN}/{name}.png'
    subprocess.run(['rsvg-convert', '-d', str(dpi), '-p', str(dpi), '-o', q, p], check=True)
    return q
#endregion


def lw(rig, bone):
    m = rig.world(bone)
    return lambda x, y: apply(m, x, y)


def v1_shapes(A, keep=None, skip=()):
    """The 1:50 raw material: the v1 shapes minus `lod`; optional id filter."""
    out = []
    for s in A['geometry']['elevation']:
        if s['id'] in skip:
            continue
        s = {k: v for k, v in s.items() if k != 'lod'}
        out.append(s)
    return out


def sorted_keys(A):
    order = ['id', 'category', 'title', 'summary', 'tags', 'provenance', 'dimensions', 'materials', 'rig', 'anchors', 'representations', 'symbols', 'poses', 'clips', 'thermal', 'structure', 'presentation', 'exports', 'completion']
    return {k: A[k] for k in order}
