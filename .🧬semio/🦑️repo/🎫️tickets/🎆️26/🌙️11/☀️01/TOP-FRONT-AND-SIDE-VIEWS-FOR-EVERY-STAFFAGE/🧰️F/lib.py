import json, math, os, re, sys, shutil
from decimal import Decimal
sys.stdout.reconfigure(encoding='utf8')
S = 'C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🛋️furniture/'
HERE = os.path.dirname(os.path.abspath(__file__))
ORIG = os.path.join(HERE, 'orig')


def r4(v):
    return round(v + 0.0, 4)


def load_original(aid):
    os.makedirs(ORIG, exist_ok=True)
    backup = os.path.join(ORIG, aid + '.json')
    path = S + aid + '/🔣️.json'
    if not os.path.exists(backup):
        shutil.copyfile(path, backup)
    raw = open(backup, encoding='utf8', newline='').read()
    return json.loads(raw), raw.endswith('\n')


def rest_world(asset):
    bones = {b['id']: b for b in asset['rig']['bones']}
    cache = {}

    def solve(i):
        if i in cache:
            return cache[i]
        b = bones[i]
        a = math.radians(b['angleDeg'])
        if b['parent'] is None:
            px, py, pa = 0.0, 0.0, 0.0
        else:
            px, py, pa = solve(b['parent'])
        c, s = math.cos(pa), math.sin(pa)
        x = px + c * b['x'] - s * b['y']
        y = py + s * b['x'] + c * b['y']
        cache[i] = (x, y, pa + a)
        return cache[i]

    return {i: solve(i) for i in bones}


class Scale:
    def __init__(self, asset, scale):
        self.asset = asset
        self.scale = str(scale)
        self.rep = asset['representations'][self.scale]
        self.elev = {s['id']: s for s in self.rep['geometry']['elevation']}
        self.front = []
        self.plan = []
        self.world = rest_world(asset)

    def _make(self, target, kind, sid, like, geo, o):
        base = self.elev.get(like) if like else None
        d = {'id': sid}
        pick = lambda key, default=None: o[key] if key in o else (base.get(key, default) if base else default)
        d['bone'] = o.get('bone') or (base['bone'] if base else None)
        d['role'] = pick('role', 'body')
        d['z'] = pick('z', 1)
        d['filled'] = pick('filled', True)
        d['tone'] = pick('tone', 0.5)
        reg = pick('region')
        regs = pick('regions')
        if 'region' in o:
            regs = None
        if 'regions' in o:
            reg = None
        if reg is not None:
            d['region'] = reg
        if regs is not None:
            d['regions'] = regs
        mat = pick('material')
        if mat is not None:
            d['material'] = mat
        hat = pick('hatch')
        if hat is not None:
            d['hatch'] = hat
        d['kind'] = kind
        d.update(geo)
        if 'solid' in o:
            d['solid'] = o['solid']
        pen = pick('penMm')
        if pen is not None:
            d['penMm'] = pen
        if o.get('followTip'):
            d['followTip'] = True
        target.append(d)
        return d

    def _list(self, view):
        return self.front if view == 'front' else self.plan

    def rect(self, view, sid, x, y, w, h, like=None, r=None, **o):
        geo = {'x': r4(x), 'y': r4(y), 'w': r4(w), 'h': r4(h)}
        if r:
            geo['r'] = r4(r)
        return self._make(self._list(view), 'rect', sid, like, geo, o)

    def poly(self, view, sid, pts, like=None, closed=True, **o):
        geo = {'points': [[r4(a), r4(b)] for a, b in pts], 'closed': closed}
        return self._make(self._list(view), 'poly', sid, like, geo, o)

    def cap(self, view, sid, x0, y0, x1, y1, r0, r1=None, like=None, **o):
        geo = {'x0': r4(x0), 'y0': r4(y0), 'x1': r4(x1), 'y1': r4(y1), 'r0': r4(r0), 'r1': r4(r0 if r1 is None else r1)}
        return self._make(self._list(view), 'capsule', sid, like, geo, o)

    def ell(self, view, sid, cx, cy, rx, ry, like=None, rot=None, **o):
        geo = {'cx': r4(cx), 'cy': r4(cy), 'rx': r4(rx), 'ry': r4(ry)}
        if rot:
            geo['rotationDeg'] = rot
        return self._make(self._list(view), 'ellipse', sid, like, geo, o)

    def solid(self, view, sid, like, z0, z1, kind=None, geo=None, **o):
        base = self.elev[like]
        skip = ('id', 'bone', 'role', 'z', 'filled', 'tone', 'region', 'regions', 'material', 'hatch', 'penMm', 'kind', 'followTip')
        if geo is None:
            geo = {k: v for k, v in base.items() if k not in skip}
        else:
            geo = {k: ([[r4(a), r4(b)] for a, b in v] if k == 'points' else (r4(v) if isinstance(v, float) else v)) for k, v in geo.items()}
        o = dict(o)
        o['solid'] = {'kind': 'extruded', 'z0': r4(z0), 'z1': r4(z1)}
        return self._make(self._list(view), kind or base['kind'], sid, like, geo, o)

    def twin(self, view, sid):
        import copy
        self._list(view).append(copy.deepcopy(self.elev[sid]))

    def follow(self, *bones):
        for sh in self.front:
            if sh['bone'] in bones and sh['kind'] in ('rect', 'poly'):
                sh['followTip'] = True

    def path(self, view, sid, d, like=None, **o):
        return self._make(self._list(view), 'path', sid, like, {'d': d}, o)


def finish(aid, asset, scales, notes, has_trailing_newline):
    """scales: {scale: Scale}; notes: {scale: (en, de)} appended to decisions.silhouette."""
    for sc, sobj in scales.items():
        rep = asset['representations'][sc]
        rep['geometry']['front'] = sobj.front
        rep['geometry']['plan'] = sobj.plan
        en, de = notes[sc]
        dec = rep['decisions']
        dec['silhouette']['en'] = dec['silhouette']['en'].rstrip() + ' ' + en
        dec['silhouette']['de'] = dec['silhouette']['de'].rstrip() + ' ' + de
    pres = asset.get('presentation', {})
    pres.pop('viewpoints', None)
    return asset


def write(aid, doc, trailing):
    path = S + aid + '/🔣️.json'
    raw = open(os.path.join(ORIG, aid + '.json'), encoding='utf8', newline='').read()
    indent = 1 if raw.startswith('{\n "') else 2
    text = json.dumps(doc, ensure_ascii=False, indent=indent) + ('\n' if trailing else '')
    text = re.sub(r'(?m)^(\s*(?:"[^"]+": )?)(-?\d+(?:\.\d+)?e-\d+)(,?)$', lambda m: m.group(1) + format(Decimal(m.group(2)), 'f') + m.group(3), text)
    atomic_write(path, text)


def atomic_write(path, text):
    import tempfile
    fd, tmp = tempfile.mkstemp(dir=os.path.dirname(path), suffix='.tmp')
    try:
        with os.fdopen(fd, 'w', encoding='utf8', newline='') as h:
            h.write(text)
        os.replace(tmp, path)
    except BaseException:
        if os.path.exists(tmp):
            os.remove(tmp)
        raise


BUMP = {'park-bench': {'l-arm-scroll': 5.3, 'r-arm-scroll': 5.3}, 'three-seat-sofa': {'arm-top-piping-l': 8.1, 'arm-top-piping-r': 8.1}}


def run(aid, build, pre=None):
    doc, trailing = load_original(aid)
    asset = doc['asset']
    if pre:
        pre(asset)
    scales = {}
    notes = {}
    for sc in ('500', '200', '100', '50'):
        if asset['representations'][sc]['status'] != 'authored':
            continue
        sobj = Scale(asset, sc)
        notes[sc] = build(sobj, sc)
        scales[sc] = sobj
    finish(aid, asset, scales, notes, trailing)
    for rep in asset['representations'].values():
        if rep['status'] != 'authored':
            continue
        for view in ('elevation', 'front', 'plan'):
            for sh in rep['geometry'][view]:
                z = BUMP.get(aid, {}).get(sh['id'])
                if z is not None and not sh.get('filled', True):
                    sh['z'] = z
    write(aid, doc, trailing)
    for sc, sobj in scales.items():
        print(aid, sc, 'front', len(sobj.front), 'plan', len(sobj.plan))


def turn_point(c, d=0):
    return lambda x, y: (y + d, c - x)


def turn_shape(sh, c, d=0):
    f = turn_point(c, d)
    k = sh['kind']
    if k == 'rect':
        x, y, w, h = sh['x'], sh['y'], sh['w'], sh['h']
        sh.update(x=r4(y + d), y=r4(c - x - w), w=r4(h), h=r4(w))
    elif k == 'poly':
        sh['points'] = [[r4(a) for a in f(px, py)] for px, py in sh['points']]
    elif k == 'capsule':
        sh['x0'], sh['y0'] = (r4(a) for a in f(sh['x0'], sh['y0']))
        sh['x1'], sh['y1'] = (r4(a) for a in f(sh['x1'], sh['y1']))
    elif k == 'ellipse':
        sh['cx'], sh['cy'] = (r4(a) for a in f(sh['cx'], sh['cy']))
        sh['rx'], sh['ry'] = sh['ry'], sh['rx']
    elif k == 'path':
        toks = re.findall(r'[A-Za-z]|-?\d+(?:\.\d+)?(?:e-?\d+)?', sh['d'])
        out, i = [], 0
        while i < len(toks):
            t = toks[i]
            if t.isalpha():
                out.append(t)
                i += 1
            else:
                a, b = f(float(toks[i]), float(toks[i + 1]))
                out += [format(r4(a), 'g'), format(r4(b), 'g')]
                i += 2
        sh['d'] = ' '.join(out)
    else:
        raise ValueError(k)


def move_shape(sh, dx, dy):
    k = sh['kind']
    if k == 'rect':
        sh['x'], sh['y'] = r4(sh['x'] + dx), r4(sh['y'] + dy)
    elif k == 'poly':
        sh['points'] = [[r4(a + dx), r4(b + dy)] for a, b in sh['points']]
    elif k == 'capsule':
        sh['x0'], sh['y0'], sh['x1'], sh['y1'] = r4(sh['x0'] + dx), r4(sh['y0'] + dy), r4(sh['x1'] + dx), r4(sh['y1'] + dy)
    elif k == 'ellipse':
        sh['cx'], sh['cy'] = r4(sh['cx'] + dx), r4(sh['cy'] + dy)
    elif k == 'path':
        toks = re.findall(r'[A-Za-z]|-?\d+(?:\.\d+)?(?:e-?\d+)?', sh['d'])
        out, i = [], 0
        while i < len(toks):
            if toks[i].isalpha():
                out.append(toks[i])
                i += 1
            else:
                out += [format(r4(float(toks[i]) + dx), 'g'), format(r4(float(toks[i + 1]) + dy), 'g')]
                i += 2
        sh['d'] = ' '.join(out)
