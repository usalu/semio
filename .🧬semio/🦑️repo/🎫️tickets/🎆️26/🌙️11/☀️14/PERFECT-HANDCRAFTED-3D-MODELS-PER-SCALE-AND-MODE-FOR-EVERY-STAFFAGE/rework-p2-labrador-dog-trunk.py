"""🐕 Labrador trunk: an oval instead of a round barrel (chest 0.25 m wide at the ninth rib, 0.20 m between the shoulders, 0.20 m at the loin), so that shoulders and upper arms stand outside the rib cage.

Usage: python3 rework-p2-labrador-dog-trunk.py   (once; refuses a second run)
The lateral half width of the chest and hindbody lofts of every scale is multiplied by NARROW(world x at rest, hip joint at 0); the plan shapes of both get the same factor point by point, the front shapes the ratio of the widest section.
"""
import importlib.util
import json
import os

T = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location('girdle', os.path.join(T, 'rework-p2-labrador-dog-girdle.py'))
girdle = importlib.util.module_from_spec(spec)
spec.loader.exec_module(girdle)

NARROW = [(0.00, 1.0), (0.10, 0.86), (0.25, 0.90), (0.33, 0.90), (0.38, 0.86), (0.42, 0.80), (0.46, 0.80), (0.50, 0.84), (0.53, 0.90), (0.555, 1.0)]
PARTS = {'chest': 'thorax', 'hindbody': 'lumbar'}


def narrow(x):
    if x <= NARROW[0][0]:
        return NARROW[0][1]
    for (x0, f0), (x1, f1) in zip(NARROW, NARROW[1:]):
        if x <= x1:
            return f0 + (f1 - f0) * (x - x0) / (x1 - x0)
    return NARROW[-1][1]


def main():
    text = open(girdle.CATALOG, encoding='utf-8').read()
    asset = json.loads(text)['asset']
    source = json.load(open(girdle.SOURCE, encoding='utf-8'))
    frames = girdle.frames(asset['rig']['bones'])
    if max(s['rx'] for s in next(s for s in source['volumes']['50']['body'] if s['id'] == 'chest')['sections']) < 0.135:
        raise SystemExit('the trunk is already narrowed')
    for scale, volume in source['volumes'].items():
        ratio = {}
        for ident, bone in PARTS.items():
            basis, origin = frames[bone]
            loft = next(s for s in volume['body'] if s['id'] == ident and s['bone'] == bone)
            before = max(s['rx'] for s in loft['sections'])
            for section in loft['sections']:
                if abs(section['rotation'][1]) < 0.6:
                    raise SystemExit('unexpected section frame in ' + ident)
                world = origin + basis @ girdle.np.array(section['at'])
                section['rx'] = round(section['rx'] * narrow(world[0]), 6)
            ratio[ident] = max(s['rx'] for s in loft['sections']) / before
            print('[DEBUG] 1:' + scale, ident, 'widest half width', round(before, 4), '->', round(before * ratio[ident], 4))
        geometry = asset['representations'][scale]['geometry']
        for shape in geometry['front']:
            if shape['id'] in PARTS and shape['bone'] == PARTS[shape['id']]:
                shape['points'] = [[round(p[0] * ratio[shape['id']], 5), p[1]] for p in shape['points']]
        for shape in geometry['plan']:
            if shape['id'] in PARTS and shape['bone'] == PARTS[shape['id']]:
                shape['points'] = [[p[0], round(p[1] * narrow(p[0]), 5)] for p in shape['points']]
    json.dump(source, open(girdle.SOURCE, 'w', encoding='utf-8'), ensure_ascii=False, separators=(',', ':'))
    fresh = open(girdle.CATALOG, encoding='utf-8').read()
    if fresh != text:
        raise SystemExit('catalog changed while working, run again')
    open(girdle.CATALOG, 'w', encoding='utf-8').write(girdle.splice(text, 'representations', asset['representations'], girdle.insert().dump))


if __name__ == '__main__':
    main()
