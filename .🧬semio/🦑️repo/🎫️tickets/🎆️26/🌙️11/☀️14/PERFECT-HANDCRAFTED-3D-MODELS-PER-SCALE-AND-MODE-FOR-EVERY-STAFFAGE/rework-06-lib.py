"""🪚 Chisel helpers of author 06: load and save a volumes source, find solids, mirror left to right, quaternions. exec() this file from a rework-06-<slug>-*.py script."""
import json, math, copy, sys

CATEGORY = {'humans': '👤️humans', 'vehicles': '🚜️vehicles'}
CATALOG = '/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog'

def catalog_path(asset):
    c, slug = asset.split('/')
    return '%s/%s/%s/🔣️.json' % (CATALOG, CATEGORY[c], slug)

def load(asset):
    """Always starts from the baseline kept in the ticket (the source as it was before author 06 touched it), so a script can be run again."""
    import os
    b = 'rework/06-base/%s.json' % asset.replace('/', '-')
    if not os.path.exists(b):
        os.makedirs('rework/06-base', exist_ok=True)
        open(b, 'w').write(open('volumes/%s.json' % asset).read())
    return json.load(open(b))

def save(asset, doc):
    with open('volumes/%s.json' % asset, 'w') as f:
        json.dump(doc, f, ensure_ascii=False, separators=(',', ':'))
        f.write('\n')

def find(doc, scale, lst, sid):
    for s in doc['volumes'][str(scale)][lst]:
        if s.get('id') == sid:
            return s
    return None

def replace(doc, scale, lst, sid, new):
    arr = doc['volumes'][str(scale)][lst]
    for i, s in enumerate(arr):
        if s.get('id') == sid:
            arr[i] = new
            return True
    return False

def insert_after(doc, scale, lst, sid, new):
    arr = doc['volumes'][str(scale)][lst]
    for i, s in enumerate(arr):
        if s.get('id') == sid:
            arr.insert(i + 1, new)
            return
    arr.append(new)

def remove(doc, scale, lst, sid):
    arr = doc['volumes'][str(scale)][lst]
    doc['volumes'][str(scale)][lst] = [s for s in arr if s.get('id') != sid]

def qmul(a, b):
    ax, ay, az, aw = a; bx, by, bz, bw = b
    return [aw*bx + ax*bw + ay*bz - az*by, aw*by - ax*bz + ay*bw + az*bx, aw*bz + ax*by - ay*bx + az*bw, aw*bw - ax*bx - ay*by - az*bz]

def qaxis(axis, deg):
    h = math.radians(deg) / 2
    q = [0, 0, 0, math.cos(h)]
    q['uvw'.index(axis)] = math.sin(h)
    return q

def qrot(q, p):
    x, y, z, w = q
    px, py, pz = p
    tx, ty, tz = 2*(y*pz - z*py), 2*(z*px - x*pz), 2*(x*py - y*px)
    return [px + w*tx + (y*tz - z*ty), py + w*ty + (z*tx - x*tz), pz + w*tz + (x*ty - y*tx)]

def rq(q):
    return [round(c, 5) for c in q]

def qmirror(q):
    return [-q[0], -q[1], q[2], q[3]]

def loft_sections(rows, rotation, exponent=2):
    """rows: (u, v, w, rx, ry[, exponent])"""
    return [{'at': [r[0], r[1], r[2]], 'rx': r[3], 'ry': r[4], 'exponent': r[5] if len(r) > 5 else exponent, 'rotation': rotation} for r in rows]

def set_loft(solid, rows, exponent=None):
    rot = solid['sections'][0]['rotation']
    ex = exponent if exponent is not None else solid['sections'][0].get('exponent', 2)
    solid['sections'] = loft_sections(rows, rot, ex)

def mirror_rows(rows):
    return [(r[0], r[1], -r[2]) + tuple(r[3:]) for r in rows]

def lerp_table(table, x):
    if x <= table[0][0]: return table[0][1]
    for (x0, y0), (x1, y1) in zip(table, table[1:]):
        if x <= x1:
            return y0 + (y1 - y0) * (x - x0) / (x1 - x0)
    return table[-1][1]

def root_edit(asset, key, fn, out=None):
    """Re-reads the catalog file, lets fn change asset[key] in place and splices only that member back (same writer as volumes-insert.py); out = candidate path instead of the catalog file."""
    import importlib.util, os
    spec = importlib.util.spec_from_file_location('vi', 'volumes-insert.py'); vi = importlib.util.module_from_spec(spec)
    argv = sys.argv; sys.argv = ['x']
    try:
        spec.loader.exec_module(vi)
    except SystemExit:
        pass
    sys.argv = argv
    path = catalog_path(asset)
    text = open(path).read()
    doc = json.loads(text)
    fn(doc['asset'][key], doc['asset'])
    marker = '\n    %s: ' % json.dumps(key)
    at = text.index(marker) + len(marker)
    end = vi.member_end(text, at)
    new = text[:at] + vi.dump(doc['asset'][key], 2) + text[end:]
    json.loads(new)
    if out:
        os.makedirs(os.path.dirname(out), exist_ok=True)
    open(out or path, 'w').write(new)

def shape(geometry, view, sid):
    for s in geometry[view]:
        if s['id'] == sid:
            return s
    raise KeyError(sid)

def sym(points_right):
    """Closed outline from its right half listed bottom to top: adds the mirrored left half top to bottom."""
    return [list(p) for p in points_right] + [[-p[0], p[1]] for p in reversed(points_right)]

def recap(doc, scale, lst, sid, cap_rows, inset=0.0, keep_from=0.0, side=1):
    """Replaces the sections before keep_from (the shoulder end of a sleeve) by hand-stated cap rows (u, v, w, rx, ry), shrunk by inset for an inner shell; side -1 mirrors w."""
    s = find(doc, scale, lst, sid)
    rot = s['sections'][0]['rotation']; ex = s['sections'][0].get('exponent', 2)
    keep = [q for q in s['sections'] if q['at'][0] >= keep_from]
    rows = [(r[0] + (inset if i == 0 else 0), r[1], side * r[2], round(r[3] - inset, 4), round(r[4] - inset, 4)) for i, r in enumerate(cap_rows)]
    s['sections'] = loft_sections(rows, rot, ex) + keep

def widen(doc, scale, lst, sid, table, cap=None, lo=None):
    """Adds the hand-stated half-width delta(u) to the lateral radius of a trunk loft (and its nested shells), never beyond cap."""
    s = find(doc, scale, lst, sid)
    for q in s['sections']:
        u = q['at'][0]
        if lo is not None and u <= lo: continue
        new = q['rx'] + lerp_table(table, u)
        if cap is not None: new = min(new, max(q['rx'], cap))
        q['rx'] = round(new, 4)

def repoint(points, mapping):
    """Redraws single vertices of a 2D outline: mapping = [(old point, [new points])]; an old point must exist."""
    out = []
    todo = {tuple(o): n for o, n in mapping}
    for p in points:
        if tuple(p) in todo:
            out.extend([list(q) for q in todo.pop(tuple(p))])
        else:
            out.append(p)
    if todo: raise SystemExit('repoint: not found %s' % list(todo))
    return out

NOTE_SHOULDER = {'en': 'Rework 2026-11-22: the garment carries the shoulder girdle out to the acromion (0.20 m from the midline, about 35 mm above the shoulder joint) and slopes along the trapezius to the collar, so the deltoid cap of the sleeve grows out of the trunk instead of sitting beside it.', 'de': 'Überarbeitung 2026-11-22: Das Kleidungsstück führt den Schultergürtel bis zum Akromion (0,20 m von der Mitte, etwa 35 mm über dem Schultergelenk) und fällt entlang des Trapezmuskels zum Kragen ab, sodass die Deltakappe des Ärmels aus dem Rumpf wächst, statt daneben zu sitzen.'}
NOTE_BONES = {'en': 'Rework 2026-11-22: each hip bone is two plates with the drawn outline of the ilium (anterior superior and inferior spine, crest with tubercle, posterior spines, greater sciatic notch), the ala flared 14 degrees and the posterior wing turned 42 degrees onto the sacrum; each shoulder blade is a triangular plate with superior, inferior and glenoid angle lying on the ribs.', 'de': 'Überarbeitung 2026-11-22: Jedes Hüftbein besteht aus zwei Platten mit dem gezeichneten Umriss des Darmbeins (vorderer oberer und unterer Darmbeinstachel, Kamm mit Höcker, hintere Stachel, große Sitzbeinkerbe), die Schaufel um 14 Grad ausgestellt, der hintere Flügel um 42 Grad zum Kreuzbein gedreht; jedes Schulterblatt ist eine dreieckige Platte mit oberem, unterem und Gelenkwinkel auf den Rippen.'}
