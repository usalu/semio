"""🪛 Chisel helpers of author 17 (building-services appliances): short constructors for the solid kinds, list editing by id and exact text patches of a catalog file. exec() this file from a rework-17-<slug>-*.py script; every number stays in the calling script."""
import json, math, os, glob, re

T = os.path.dirname(os.path.abspath(__file__)) if '__file__' in globals() else os.getcwd()
CATALOG = '/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog'


def catalog_path(slug):
    """📍 The catalog file of an appliance."""
    return glob.glob('%s/*/%s/*.json' % (CATALOG, slug))[0]


def load(slug):
    """📥 The source as it was before author 17 touched it (kept under rework/17-base), so a script can run again."""
    base = '%s/rework/17-base/%s.json' % (T, slug)
    if not os.path.exists(base):
        os.makedirs(os.path.dirname(base), exist_ok=True)
        open(base, 'w').write(open('%s/volumes/appliances/%s.json' % (T, slug)).read())
    return json.load(open(base))['volumes']


def save(slug, volumes):
    """📤 Writes the source."""
    with open('%s/volumes/appliances/%s.json' % (T, slug), 'w') as f:
        json.dump({'volumes': volumes}, f, ensure_ascii=False, separators=(',', ':'))
        f.write('\n')


def r5(x):
    return round(x, 5)


def base(id, bone, mat, role='body', tone=0.5, **kw):
    s = {'id': id, 'bone': bone, 'material': mat, 'role': role, 'tone': tone}
    s.update(kw)
    return s


def box(id, bone, mat, size, at, r=0, **kw):
    """📦 Box of size [u, v, w] centred at `at`."""
    s = base(id, bone, mat, **kw)
    s.update(kind='box', size=list(size))
    if r:
        s['cornerRadius'] = r
    if any(at):
        s.setdefault('transform', {})['translation'] = list(at)
    return s


def rev(id, bone, mat, axis, origin, profile, **kw):
    """🏺 Revolve of [axial, radial] points about `axis` through `origin`."""
    s = base(id, bone, mat, **kw)
    s.update(kind='revolve', profile=[[r5(a), r5(b)] for a, b in profile], axis=list(axis), origin=list(origin))
    return s


def cyl(id, bone, mat, axis, origin, a0, a1, r, **kw):
    """🥫 Solid cylinder from a0 to a1 along the axis."""
    return rev(id, bone, mat, axis, origin, [[a0, 0], [a0, r], [a1, r], [a1, 0]], **kw)


def tube(id, bone, mat, axis, origin, a0, a1, ri, ro, **kw):
    """🧻 Hollow cylinder (closed loop profile)."""
    return rev(id, bone, mat, axis, origin, [[a0, ri], [a0, ro], [a1, ro], [a1, ri], [a0, ri]], **kw)


def torus(id, bone, mat, axis, origin, a, R, r, n=12, flat=1.0, **kw):
    """🍩 Ring of tube radius r (axial radius r*flat) at radius R, axial position a."""
    pts = [[a + r * flat * math.cos(2 * math.pi * k / n), R + r * math.sin(2 * math.pi * k / n)] for k in range(n)]
    return rev(id, bone, mat, axis, origin, pts + [pts[0]], **kw)


def cap(id, bone, mat, p0, p1, r, r1=None, **kw):
    """💊 Capsule."""
    s = base(id, bone, mat, **kw)
    s.update(kind='capsule', p0=list(p0), p1=list(p1), r0=r, r1=r if r1 is None else r1)
    return s


def ext(id, bone, mat, plane, profile, frm, to, holes=None, **kw):
    """🧇 Extrusion of a polygon in `plane` (xy: u,v along w; yz: v,w along u; zx: w,u along v)."""
    s = base(id, bone, mat, **kw)
    s.update(kind='extrude', profile=[[r5(a), r5(b)] for a, b in profile], plane=plane)
    s['from'] = frm
    s['to'] = to
    if holes:
        s['holes'] = [[[r5(a), r5(b)] for a, b in h] for h in holes]
    return s


def ell(id, bone, mat, radii, at, **kw):
    """🥚 Ellipsoid."""
    s = base(id, bone, mat, **kw)
    s.update(kind='ellipsoid', radii=list(radii))
    if any(at):
        s['transform'] = {'translation': list(at)}
    return s


def loft(id, bone, mat, sections, **kw):
    """🪵 Loft through sections (at, rx, ry, exponent)."""
    s = base(id, bone, mat, **kw)
    s.update(kind='loft', sections=[dict(at=list(a), rx=rx, ry=ry, exponent=e) for a, rx, ry, e in sections], capStart=True, capEnd=True)
    return s


def circle(cx, cy, r, n=24, start=0.0):
    """⭕ Polygon of a circle."""
    return [[cx + r * math.cos(start + 2 * math.pi * k / n), cy + r * math.sin(start + 2 * math.pi * k / n)] for k in range(n)]


def arc(cx, cy, r, a0, a1, n):
    """🌙 Points of an arc from a0 to a1 degrees, both ends included."""
    return [[cx + r * math.cos(math.radians(a0 + (a1 - a0) * k / n)), cy + r * math.sin(math.radians(a0 + (a1 - a0) * k / n))] for k in range(n + 1)]


def rrect(x0, y0, x1, y1, r, n=4):
    """▢ Rounded rectangle, counter-clockwise from the lower right corner."""
    return arc(x1 - r, y0 + r, r, -90, 0, n) + arc(x1 - r, y1 - r, r, 0, 90, n) + arc(x0 + r, y1 - r, r, 90, 180, n) + arc(x0 + r, y0 + r, r, 180, 270, n)


def hexagon(cx, cy, af, start=0.0):
    """⬡ Hexagon across flats af."""
    return circle(cx, cy, af / math.sqrt(3), 6, start)


def th(solid, region, layer=None, **kw):
    """🌡️ Thermal entry: a solid or the id of a body solid."""
    s = {'id': 'th-' + solid, 'of': solid} if isinstance(solid, str) else dict(solid)
    s['region'] = region
    if layer:
        s['layer'] = layer
    s.update(kw)
    return s


def st(solid, member, part='member', **kw):
    """🏗️ Structural entry: a solid or the id of a body solid."""
    s = {'id': 'st-' + solid, 'of': solid} if isinstance(solid, str) else dict(solid)
    s['member'] = member
    s['part'] = part
    s.update(kw)
    return s


class Lists:
    """🗂️ One scale of a source: keep, drop, put by id."""

    def __init__(self, volumes, scale):
        self.v = volumes[str(scale)]

    def get(self, lst, sid):
        return next(s for s in self.v[lst] if s['id'] == sid)

    def drop(self, lst, *ids):
        missing = set(ids) - {s['id'] for s in self.v[lst]}
        assert not missing, missing
        self.v[lst] = [s for s in self.v[lst] if s['id'] not in ids]

    def put(self, lst, *solids, after=None):
        ids = {s['id'] for s in solids}
        first = next((k for k, s in enumerate(self.v[lst]) if s['id'] in ids), None)
        arr = [s for s in self.v[lst] if s['id'] not in ids]
        if first is not None and (after is None or after in ids):
            i = len([s for s in self.v[lst][:first] if s['id'] not in ids])
        else:
            i = len(arr) if after is None else next(k for k, s in enumerate(arr) if s['id'] == after) + 1
        arr[i:i] = list(solids)
        self.v[lst] = arr

    def set(self, lst, solids):
        self.v[lst] = list(solids)

    def decide(self, prefix, en, de):
        d = self.v['decisions']
        for i, x in enumerate(d):
            if x['en'].startswith(prefix):
                d[i] = {'en': en, 'de': de}
                return
        d.append({'en': en, 'de': de})


def patch_catalog(slug, pairs):
    """🩹 Exact, unique text replacements in the catalog file, re-read right before writing; pairs already applied are skipped."""
    p = catalog_path(slug)
    text = open(p).read()
    for old, new in pairs:
        if old not in text and new in text:
            continue
        assert text.count(old) == 1, (text.count(old), old[:80])
        text = text.replace(old, new)
    json.loads(text)
    open(p, 'w').write(text)


def set_area(slug, areas):
    """📐 Sets thermal.regions[id].areaM2 by exact text patch."""
    p = catalog_path(slug)
    doc = json.load(open(p))['asset']
    pairs = []
    text = open(p).read()
    for r in doc['thermal']['regions']:
        if r['id'] in areas and r['areaM2'] != areas[r['id']]:
            m = re.search(r'"id": "%s",(?:(?!"id": ).)*?"areaM2": ([0-9.e-]+),' % re.escape(r['id']), text[text.index('"thermal": {\n      "environment"'):], re.S)
            assert m, r['id']
            start = text.index('"thermal": {\n      "environment"')
            a, b = m.span(1)
            text = text[:start + a] + repr(areas[r['id']]) + text[start + b:]
    json.loads(text)
    open(p, 'w').write(text)


def _member_end(text, start):
    depth, index, quoted = 0, start, False
    while index < len(text):
        ch = text[index]
        if quoted:
            if ch == '\\':
                index += 1
            elif ch == '"':
                quoted = False
        elif ch == '"':
            quoted = True
        elif ch in '{[':
            depth += 1
        elif ch in '}]':
            depth -= 1
            if depth == 0:
                return index + 1
        index += 1
    raise SystemExit('unbalanced')


def _scalar(v):
    return not isinstance(v, (dict, list))


def _inline(v):
    return all(_scalar(i) for i in v) or (all(isinstance(i, list) and all(_scalar(x) for x in i) for i in v) and len(json.dumps(v)) <= 160)


def _dump_inline(value, depth):
    pad, inner = '  ' * depth, '  ' * (depth + 1)
    if isinstance(value, dict):
        if not value:
            return '{}'
        return '{\n' + ',\n'.join(inner + json.dumps(k, ensure_ascii=False) + ': ' + _dump_inline(v, depth + 1) for k, v in value.items()) + '\n' + pad + '}'
    if isinstance(value, list):
        if not value:
            return '[]'
        if _inline(value):
            return json.dumps(value, ensure_ascii=False)
        return '[\n' + ',\n'.join(inner + _dump_inline(v, depth + 1) for v in value) + '\n' + pad + ']'
    return json.dumps(value, ensure_ascii=False)


def _dump_member(value, style=0):
    return json.dumps(value, indent=2, ensure_ascii=False).replace('\n', '\n    ') if style == 0 else _dump_inline(value, 2)


def edit_catalog(slug, members, fn, dry=False):
    """✍️ Re-reads the catalog file, hands the asset to fn(asset) and rewrites only the named top-level members of the asset (standard two-space layout, byte-identical when unchanged)."""
    p = catalog_path(slug)
    text = open(p).read()
    asset = json.loads(text)['asset']
    styles = {}
    for m in members:
        found = re.search(r'\n    "' + m + r'": ', text)
        s, e = found.end(), _member_end(text, found.end())
        styles[m] = next((k for k in (0, 1) if text[s:e] == _dump_member(asset[m], k)), None)
        if styles[m] is None:
            styles[m] = 1
    fn(asset)
    for m in members:
        found = re.search(r'\n    "' + m + r'": ', text)
        s, e = found.end(), _member_end(text, found.end())
        text = text[:s] + _dump_member(asset[m], styles[m]) + text[e:]
    json.loads(text)
    if not dry:
        open(p, 'w').write(text)
    return asset


def shapes(asset, scale, view):
    """🖊️ The 2D shapes of a view."""
    return asset['representations'][str(scale)]['geometry'][view]


def shape(asset, scale, view, sid):
    return next(s for s in shapes(asset, scale, view) if s['id'] == sid)


def own(solid, region, layer=None, **kw):
    """🧬 Thermal entry with its own geometry: the solid copied under the id th-<id> (hidden regions may not reference body solids)."""
    s = json.loads(json.dumps(solid))
    s['id'] = 'th-' + s['id']
    return th(s, region, layer, **kw)
