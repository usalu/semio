import json, os, sys, io

sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf8')
BASE = r'C:\git\semio\🎓️teaching\🏛️architecture\🧍️staffage\🗂️catalog\🧱️components'


def r4(v):
    return round(v + 0.0, 5)


def _put(d, key, value):
    if value is not None:
        d[key] = value


def shape(id, bone, role, z, kind, geom, *, filled=True, tone=0.5, region=None, regions=None, material=None, hatch=None, pen=None, follow=False, solid=None):
    s = {'id': id, 'bone': bone, 'role': role, 'z': z, 'filled': filled, 'tone': tone}
    _put(s, 'region', region)
    _put(s, 'regions', regions)
    _put(s, 'material', material)
    _put(s, 'hatch', hatch)
    _put(s, 'penMm', pen)
    s['kind'] = kind
    for k, v in geom.items():
        s[k] = r4(v) if isinstance(v, float) else v
    if solid is not None:
        s['solid'] = solid
    if follow:
        s['followTip'] = True
    return s


def rect(id, bone, role, z, x, y, w, h, tone, rad=None, **kw):
    g = {'x': x, 'y': y, 'w': w, 'h': h}
    if rad is not None:
        g['r'] = rad
    return shape(id, bone, role, z, 'rect', g, tone=tone, **kw)


def poly(id, bone, role, z, points, tone, closed=True, **kw):
    pts = [[r4(a), r4(b)] for a, b in points]
    kw.setdefault('filled', closed)
    return shape(id, bone, role, z, 'poly', {'points': pts, 'closed': closed}, tone=tone, **kw)


def line(id, bone, role, z, points, tone, **kw):
    return poly(id, bone, role, z, points, tone, closed=False, filled=False, **kw)


def ell(id, bone, role, z, cx, cy, rx, ry, tone, **kw):
    return shape(id, bone, role, z, 'ellipse', {'cx': cx, 'cy': cy, 'rx': rx, 'ry': ry}, tone=tone, **kw)


def cap(id, bone, role, z, x0, y0, x1, y1, r0, r1, tone, **kw):
    return shape(id, bone, role, z, 'capsule', {'x0': x0, 'y0': y0, 'x1': x1, 'y1': y1, 'r0': r0, 'r1': r1}, tone=tone, **kw)


def path(id, bone, role, z, d, tone, **kw):
    return shape(id, bone, role, z, 'path', {'d': d}, tone=tone, **kw)


def mirror_x(shapes, fmt):
    out = []
    for s in shapes:
        c = json.loads(json.dumps(s))
        c['id'] = fmt(s['id'])
        if c['kind'] == 'rect':
            c['x'] = r4(-s['x'] - s['w'])
        out.append(c)
    return out


def path_file(i):
    return os.path.join(BASE, i, '🔣️.json')


def load(i):
    p = path_file(i)
    raw = open(p, encoding='utf8', newline='').read()
    indent = 1 if raw.split('\n')[1].startswith(' "') else 2
    return json.loads(raw), indent


def save(i, doc, indent):
    import tempfile
    txt = json.dumps(doc, ensure_ascii=False, indent=indent) + chr(10)
    target = path_file(i)
    fd, tmp = tempfile.mkstemp(dir=os.path.dirname(target), suffix='.tmp')
    try:
        with os.fdopen(fd, 'w', encoding='utf8', newline='') as f:
            f.write(txt)
        os.replace(tmp, target)
    except BaseException:
        if os.path.exists(tmp):
            os.remove(tmp)
        raise


GEN = r'C:\git\semio\.🧬semio\🦑️repo\🎫️tickets\🎆️26\🌙️11\☀️01\TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE\🗑️generated\C1'


def apply(i, views, decisions):
    doc, indent = load(i)
    a = doc['asset']
    backup = os.path.join(GEN, 'orig-decisions-%s.json' % i)
    if os.path.exists(backup):
        orig = json.load(open(backup, encoding='utf8'))
        for sc, dec in orig.items():
            a['representations'][sc]['decisions'] = dec
    else:
        orig = {sc: rep['decisions'] for sc, rep in a['representations'].items() if rep.get('status') == 'authored'}
        json.dump(orig, open(backup, 'w', encoding='utf8'), ensure_ascii=False, indent=1)
    pres = a.get('presentation')
    if pres is not None:
        pres.pop('viewpoints', None)
    for sc, rep in a['representations'].items():
        if rep.get('status') != 'authored':
            continue
        v = views[sc]
        geo = rep['geometry']
        geo['front'] = v['front']
        geo['plan'] = v['plan']
        order = ['elevation', 'front', 'plan']
        rep['geometry'] = {k: geo[k] for k in order}
        for key, (en, de) in decisions[sc].items():
            d = rep['decisions'][key]
            if en.startswith('!'):
                d['en'], d['de'] = en[1:], de[1:]
                continue
            d['en'] = d['en'].rstrip() + ' ' + en
            d['de'] = d['de'].rstrip() + ' ' + de
        for h in v.get('hatches', []):
            if all(x['id'] != h['id'] for x in rep['hatches']):
                rep['hatches'].append(h)
    save(i, doc, indent)
    print('saved', i)
