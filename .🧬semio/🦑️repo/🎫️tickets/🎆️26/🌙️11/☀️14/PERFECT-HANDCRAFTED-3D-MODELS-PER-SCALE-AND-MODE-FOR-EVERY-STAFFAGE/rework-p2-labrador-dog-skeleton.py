"""🦴 Labrador skeleton: thirteen curved rib pairs with costal cartilages and radius and ulna as two bones at 1:50, typified curved ribs at 1:100 and 1:200 (hand-chosen numbers).

Usage: python3 rework-p2-labrador-dog-skeleton.py   (rewrites the ribs of the 1:50, 1:100 and 1:200 structure lists and the 1:50 forearm of volumes/animals/labrador-dog.json; repeatable)
Thorax frame: x ahead along the bone from the thoracolumbar junction, y dorsal, z left. Forearm frame: x elbow to carpus, y cranial, z left.
"""
import importlib.util
import json
import math
import os

T = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location('girdle', os.path.join(T, 'rework-p2-labrador-dog-girdle.py'))
girdle = importlib.util.module_from_spec(spec)
spec.loader.exec_module(girdle)
np = girdle.np
SOURCE = girdle.SOURCE
HERE = (np.array([[1, 0, 0], [0, 1, 0], [0, 0, 1]]), np.array([0, 0, 0]))
LIMB = [0.5, 0.5, 0.5, 0.5]

HALF_WIDTH = [0.038, 0.050, 0.062, 0.073, 0.083, 0.092, 0.099, 0.104, 0.107, 0.108, 0.106, 0.100, 0.091]
JUNCTION_Y = [-0.115, -0.130, -0.142, -0.150, -0.155, -0.156, -0.153, -0.146, -0.136, -0.122, -0.106, -0.088, -0.068]
JUNCTION_Z = [0.024, 0.034, 0.045, 0.056, 0.064, 0.072, 0.077, 0.081, 0.084, 0.086, 0.086, 0.083, 0.077]
SWEEP = [-0.030, -0.022, -0.012, -0.002, 0.008, 0.016, 0.024, 0.032, 0.040, 0.046, 0.050, 0.052, 0.052]
STERNAL = [(0.246, -0.128), (0.236, -0.146), (0.223, -0.161), (0.207, -0.174), (0.188, -0.184), (0.166, -0.191), (0.142, -0.1955), (0.116, -0.197), (0.100, -0.1975)]
ARCH = [(0.05, None, 0.40), (0.18, 0.10, 0.78), (0.38, 0.32, 0.97), (0.62, 0.58, 1.0)]


def lerp(a, b, t):
    return tuple(a[i] + (b[i] - a[i]) * t for i in range(3))


def length(points):
    return sum(math.dist(points[i], points[i + 1]) for i in range(len(points) - 1))


def thorax(structure):
    spine = sorted((s['at'][0], s['at'][1]) for s in next(s for s in structure if s['id'] == 'thoracic')['sections'])

    def spine_y(x):
        for (x0, y0), (x1, y1) in zip(spine, spine[1:]):
            if x <= x1:
                return y0 + (y1 - y0) * (x - x0) / (x1 - x0)
        return spine[-1][1]

    ribs, cartilages = {}, {}
    for k in range(1, 14):
        i = k - 1
        x = 0.203 - i * 0.01625
        head = (x, spine_y(x) - 0.004, 0.010)
        drop = head[1] - JUNCTION_Y[i]
        junction = (x - SWEEP[i], JUNCTION_Y[i], JUNCTION_Z[i])
        path = [head] + [(x - SWEEP[i] * s, head[1] + 0.006 if d is None else head[1] - drop * d, HALF_WIDTH[i] * w) for s, d, w in ARCH]
        path += [(x - SWEEP[i] * 0.84, head[1] - drop * 0.82, JUNCTION_Z[i] + (HALF_WIDTH[i] - JUNCTION_Z[i]) * 0.55), junction]
        ribs[k] = path
        if k <= 9:
            end = (STERNAL[i][0], STERNAL[i][1], 0.009)
            knee = (junction[0] + (end[0] - junction[0]) * 0.3, junction[1] + (end[1] - junction[1]) * 0.55, junction[2] * 0.8)
            fall = (junction[0] + (end[0] - junction[0]) * 0.7, junction[1] + (end[1] - junction[1]) * 0.9, junction[2] * 0.4)
            cartilages[k] = [junction, knee, fall, end]
        elif k <= 12:
            end = cartilages[k - 1][1]
            mid = lerp(junction, end, 0.5)
            cartilages[k] = [junction, (mid[0], mid[1] - 0.010, mid[2]), end]
        else:
            cartilages[k] = [junction, (junction[0] + 0.012, junction[1] - 0.012, junction[2] - 0.004), (junction[0] + 0.024, junction[1] - 0.018, junction[2] - 0.010)]
    return ribs, cartilages


def fine(structure):
    ribs, cartilages = thorax(structure)
    out = []
    for name, s in (('n', -1), ('f', 1)):
        for k in range(1, 14):
            flat = length(ribs[k]) >= 0.19
            stations = [(p, 0.0045 if flat else 0.004, 0.003 if flat else 0.0038) for p in ribs[k]]
            out.append(girdle.world_loft(HERE, f'rib-{k}-{name}', 'thorax', stations, (1, 0, 0), s, 2, {'slender': True} if flat else None))
            out.append(girdle.world_loft(HERE, f'costal-cartilage-{k}-{name}', 'thorax', [(p, 0.0038, 0.0038) for p in cartilages[k]], (0.3, 1, 0.2), s, 2, {'tone': 0.75}))
    return out


def coarse(structure, numbers, r):
    """🩻 Typified ribs of a coarse scale: every named rib as one rod of the smallest feature from the vertebra over the costal cartilage to the sternum, drawn in by its own radius so that it stays under the skin."""
    ribs, cartilages = thorax(structure)
    out = []
    for name, s in (('n', -1), ('f', 1)):
        for k in numbers:
            path = ribs[k] + (cartilages[k][1:] if k <= 9 else [])
            widest = max(p[2] for p in path)
            pull = (widest - (r - 0.003)) / widest
            stations = [((p[0], p[1] + (r - 0.004 if 0 < i < len(path) - 1 else 0), max(p[2] * pull, 0.009)), r, r) for i, p in enumerate(path)]
            out.append(girdle.world_loft(HERE, f'rib-{k}-{name}', 'thorax', stations, (1, 0, 0), s, 2))
    return out


def forearm(tag, name, s):
    bone = 'forearm-' + tag
    radius = [((0.002, 0.004, 0), 0.007, 0.011), ((0.012, 0.005, 0), 0.006, 0.009), ((0.045, 0.007, 0), 0.0048, 0.0075), ((0.095, 0.008, 0), 0.0046, 0.0072), ((0.140, 0.006, 0), 0.005, 0.0085), ((0.165, 0.004, 0), 0.0065, 0.0115), ((0.178, 0.003, 0), 0.0075, 0.0125)]
    ulna = [((-0.036, -0.014, 0), 0.006, 0.0045), ((-0.028, -0.015, 0), 0.009, 0.005), ((-0.012, -0.013, 0), 0.0085, 0.005), ((0.002, -0.011, 0), 0.006, 0.0055), ((0.015, -0.008, s * 0.002), 0.0065, 0.005), ((0.050, -0.005, s * 0.005), 0.0045, 0.0042), ((0.100, -0.003, s * 0.008), 0.004, 0.004), ((0.150, -0.002, s * 0.010), 0.0038, 0.0038), ((0.176, -0.002, s * 0.011), 0.0038, 0.0038)]
    out = []
    for ident, stations in (('radius-' + name, radius), ('ulna-' + name, ulna)):
        out.append({'id': ident, 'kind': 'loft', 'bone': bone, 'material': 'bone', 'role': 'body', 'tone': 0.9, 'sections': [{'at': list(at), 'rx': rx, 'ry': ry, 'exponent': 2.2, 'rotation': LIMB} for at, rx, ry in stations], 'capStart': True, 'capEnd': True, 'member': bone, 'part': 'member'})
    return out


def main():
    source = json.load(open(SOURCE, encoding='utf-8'))
    structure = source['volumes']['50']['structure']
    old = [s for s in structure if s.get('bone', '').startswith('forearm-') and s.get('part') == 'member']
    print('[DEBUG] forearm members replaced:', [(s['id'], s['kind'], [(x['at'], x['rx'], x['ry']) for x in s.get('sections', [])][::4]) for s in old])
    gone = {s['id'] for s in old}
    kept = [s for s in structure if s['id'] not in gone and not s['id'].startswith(('rib-', 'costal-cartilage-', 'radius-', 'ulna-'))]
    fresh = fine(structure) + forearm('fn', 'n', -1) + forearm('ff', 'f', 1)
    cut = next(i for i, s in enumerate(kept) if s.get('part') != 'member')
    source['volumes']['50']['structure'] = kept[:cut] + fresh + kept[cut:]
    for scale, numbers, r in (('100', (1, 3, 5, 7, 9, 11, 13), 0.0077), ('200', (2, 6, 10), 0.0152)):
        rest = [s for s in source['volumes'][scale]['structure'] if not s['id'].startswith('rib-')]
        print('[DEBUG] 1:' + scale, 'hoops replaced:', len(source['volumes'][scale]['structure']) - len(rest), 'thorax members kept:', [s['id'] for s in rest if s.get('bone') == 'thorax'])
        cut = next(i for i, s in enumerate(rest) if s.get('part') != 'member')
        source['volumes'][scale]['structure'] = rest[:cut] + coarse(structure, numbers, r) + rest[cut:]
    json.dump(source, open(SOURCE, 'w', encoding='utf-8'), ensure_ascii=False, separators=(',', ':'))
    print('[DEBUG] 1:50 structure solids:', len(source['volumes']['50']['structure']))


if __name__ == '__main__':
    main()
