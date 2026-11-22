"""✏️ Draws the stems that `rework-13-bamboo-clump-necks.py` gave the foliage of vegetation/bamboo-clump into the 2D drawings of 1:100, 1:200 and 1:500 (elevation, front, plan): one stroke per stem from its root on the branch or culm into the mass, collected per bone in the shape `<bone>-branches`.

Usage: python3 rework-13-bamboo-clump-stems2d.py <source before the stems> --candidate <file> | --write
1:100 appends to the existing branch strokes (detail pen 0.13 mm = 13 mm), 1:200 adds the shape in the detail pen (0.13 mm = 26 mm for the 30 mm stem), 1:500 adds it as a body stroke of 0.13 mm like the culms (65 mm for the 75 mm stem; the scale has no detail pen).
Frames: elevation is bone-local (x, y); front is (world z, world height) and plan (world x, world z) of the rest pose.
"""
import json
import math
import re
import sys

CATALOG = '/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🌳️vegetation/bamboo-clump/🔣️.json'
SOURCE = 'volumes/vegetation/bamboo-clump.json'
before = {scale: {s['id']: s for s in v['body']} for scale, v in json.load(open(sys.argv[1]))['volumes'].items()}
volumes = json.load(open(SOURCE))['volumes']
text = open(CATALOG, encoding='utf-8').read()
document = json.loads(text)
asset = document['asset']
bones = {b['id']: b for b in asset['rig']['bones']}


def frame(bone_id):
    chain, b = [], bones[bone_id]
    while b:
        chain.append(b)
        b = bones.get(b['parent'])
    ox = oy = oz = angle = 0.0
    for b in reversed(chain):
        c, s = math.cos(math.radians(angle)), math.sin(math.radians(angle))
        ox, oy, oz = ox + c * b['x'] - s * b['y'], oy + s * b['x'] + c * b['y'], oz + b.get('z', 0)
        angle += b['angleDeg']
    return ox, oy, oz, angle


def views(bone_id, p):
    ox, oy, oz, angle = frame(bone_id)
    c, s = math.cos(math.radians(angle)), math.sin(math.radians(angle))
    wx, wy, wz = ox + c * p[0] - s * p[1], oy + s * p[0] + c * p[1], oz + p[2]
    return {'elevation': (p[0], p[1]), 'front': (wz, wy), 'plan': (wx, wz)}


def stroke(a, b):
    return 'M%s %sL%s %s' % tuple(format(round(c, 4), 'g') for c in (*a, *b))


for scale in ('100', '200', '500'):
    strokes = {}
    for solid in volumes[scale]['body']:
        old = before[scale].get(solid['id'])
        if solid['kind'] != 'polyhedron' or old is None or len(old['vertices']) == len(solid['vertices']):
            continue
        v = solid['vertices']
        k = len(v) - len(old['vertices'])
        root, tip = v[-2], [sum(p[i] for p in v[k:2 * k]) / k for i in range(3)]
        a, b = views(solid['bone'], root), views(solid['bone'], tip)
        for view in a:
            strokes.setdefault((solid['bone'], view), []).append(stroke(a[view], b[view]))
    geometry = asset['representations'][scale]['geometry']
    for (bone, view), parts in sorted(strokes.items()):
        shapes = geometry[view]
        name = bone + '-branches'
        shape = next((s for s in shapes if s['id'] == name), None)
        if shape is not None:
            shape['d'] += ''.join(parts)
            continue
        culm = next((i for i, s in enumerate(shapes) if s['id'] == bone), None)
        if culm is None:
            culm = next(i for i, s in enumerate(shapes) if s['bone'] == bone)
        line = next(s for s in shapes if s['id'] in bones and not s.get('filled'))
        model = next((s for s in asset['representations']['100']['geometry'][view] if s['id'] == name), None)
        if scale == '200' and model is not None:
            shape = {**model, 'd': ''.join(parts)}
        else:
            shape = {**line, 'id': name, 'bone': bone, 'z': round(shapes[culm]['z'] + 0.0001, 4), 'd': ''.join(parts)}
            if scale == '200':
                shape.update(role='detail', tone=0.3)
                for key in ('penMm', 'region', 'material'):
                    shape.pop(key, None)
        if view == 'plan':
            shape['z'] = round(min(s['z'] for s in shapes if s['bone'] == bone) - 0.0001, 4)
        shapes.insert(culm + 1, shape)
    print('[DEBUG] scale', scale, 'stems drawn', sum(len(p) for p in strokes.values()) // 3, 'bones', len(strokes) // 3)

import importlib.util
spec = importlib.util.spec_from_file_location('texts', 'rework-13-bamboo-clump-texts.py')
texts = importlib.util.module_from_spec(spec)
spec.loader.exec_module(texts)
texts.apply(asset['representations'], texts.DRAWING)

if sys.argv[2] == '--candidate':
    asset['volumes'] = volumes
    json.dump(document, open(sys.argv[3], 'w', encoding='utf-8'), ensure_ascii=False)
else:
    found = re.search(r'\n    "representations"\s*:\s*', text)
    depth, index, quoted = 0, found.end(), False
    while True:
        char = text[index]
        if quoted:
            if char == '\\':
                index += 1
            elif char == '"':
                quoted = False
        elif char == '"':
            quoted = True
        elif char in '{[':
            depth += 1
        elif char in '}]':
            depth -= 1
            if depth == 0:
                break
        index += 1
    result = text[:found.end()] + json.dumps(asset['representations'], ensure_ascii=False, separators=(',', ':')) + text[index + 1:]
    check = json.loads(result)
    if check['asset']['representations'] != asset['representations'] or {k: v for k, v in check['asset'].items() if k != 'representations'} != {k: v for k, v in json.loads(text)['asset'].items() if k != 'representations'}:
        raise SystemExit('splice did not round-trip')
    if open(CATALOG, encoding='utf-8').read() != text:
        raise SystemExit('catalog changed while splicing; retry')
    open(CATALOG, 'w', encoding='utf-8').write(result)
    print('representations written')
