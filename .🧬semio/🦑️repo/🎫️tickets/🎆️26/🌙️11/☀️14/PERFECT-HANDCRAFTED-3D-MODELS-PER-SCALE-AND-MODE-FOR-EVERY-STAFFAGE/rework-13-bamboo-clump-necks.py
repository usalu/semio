"""🎋 Gives every floating foliage solid of vegetation/bamboo-clump at 1:100, 1:200 and 1:500 its stem: one more ring at the place where it grows from (the branch at 1:100, the node at 1:200, the culm at 1:500), so the mass starts as a branchlet or branch and swells into the leaves.

Usage: python3 rework-13-bamboo-clump-necks.py
Hand-chosen numbers: stem radius 5 mm (1:100, a branchlet with its sheaths drawn at a third of the feature), 15 mm (1:200, half the feature) and 37.5 mm (1:500, half the feature); a stem leaves its axis at 55 degrees (the attach point sits 0.7 of the gap below the nearest point); solids nearer than 1.2 stem radii are left as they are.
"""
import json
import math

SOURCE = 'volumes/vegetation/bamboo-clump.json'
STEM_RADIUS = {'100': 0.005, '200': 0.015, '500': 0.0375}
BACK_OFF = 0.7
TOUCHING = 1.2


def sub(a, b):
    return [a[i] - b[i] for i in range(3)]


def add(a, b, s=1.0):
    return [a[i] + s * b[i] for i in range(3)]


def dot(a, b):
    return sum(a[i] * b[i] for i in range(3))


def cross(a, b):
    return [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]


def unit(a):
    n = math.sqrt(dot(a, a))
    return [c / n for c in a]


def ring_size(solid):
    start = len(solid['vertices']) - 2
    return sum(1 for t in solid['triangles'] if start in t)


def centres(solid):
    k, v = ring_size(solid), solid['vertices']
    return [[sum(p[i] for p in v[r:r + k]) / k for i in range(3)] for r in range(0, len(v) - 2, k)]


def attach(axis, point):
    lengths = [math.dist(axis[i], axis[i + 1]) for i in range(len(axis) - 1)]
    best = None
    for i, length in enumerate(lengths):
        d = sub(axis[i + 1], axis[i])
        t = max(0.0, min(1.0, dot(sub(point, axis[i]), d) / (length * length)))
        gap = math.dist(point, add(axis[i], d, t))
        if best is None or gap < best[0]:
            best = (gap, sum(lengths[:i]) + t * length)
    gap, s = best
    s = max(0.0, s - BACK_OFF * gap)
    for i, length in enumerate(lengths):
        if s <= length or i == len(lengths) - 1:
            return gap, add(axis[i], sub(axis[i + 1], axis[i]), min(1.0, s / length))
        s -= length


def stem(solid, root, radius):
    k, v = ring_size(solid), solid['vertices']
    rings, centre, end = len(v) // k, v[-2], v[-1]
    u = unit(sub(centre, root))
    offsets = [sub(p, centre) for p in v[:k]]
    flat = [add(o, u, -dot(o, u)) for o in offsets]
    m = max(range(k), key=lambda i: dot(flat[i], flat[i]))
    e1 = unit(flat[m])
    e2 = cross(u, e1)
    turn = 1.0 if dot(cross(offsets[m], offsets[(m + 1) % k]), u) > 0 else -1.0
    ring = []
    for i in range(k):
        a = 2 * math.pi * (i - m) / k
        ring.append([round(c, 8) for c in add(add(root, e1, radius * math.cos(a)), e2, turn * radius * math.sin(a))])
    vertices = ring + v[:-2] + [[round(c, 8) for c in root], end]
    rings += 1
    c0, c1 = len(vertices) - 2, len(vertices) - 1
    triangles = []
    for r in range(rings - 1):
        for i in range(k):
            a, b = r * k + i, r * k + (i + 1) % k
            triangles += [[a, b, a + k], [b, b + k, a + k]]
    last = (rings - 1) * k
    triangles += [[c0, (i + 1) % k, i] for i in range(k)]
    triangles += [[c1, last + i, last + (i + 1) % k] for i in range(k)]
    solid['vertices'], solid['triangles'] = vertices, triangles


document = json.load(open(SOURCE))
volumes = document['volumes']
branch_base = {s['id']: (s['bone'], s['vertices'][-2]) for s in volumes['100']['body'] if s.get('slender') and s['id'].startswith('b')}
for scale, radius in STEM_RADIUS.items():
    body = volumes[scale]['body']
    by_id = {s['id']: s for s in body}
    done, gaps = 0, []
    for solid in body:
        if solid['kind'] != 'polyhedron' or solid.get('slender') or solid['material'] != 'bamboo-leaf':
            continue
        start = solid['vertices'][-2]
        parent = by_id.get(solid['id'].rsplit('-s', 1)[0]) if scale == '100' else None
        if parent is None or parent['bone'] != solid['bone'] or not parent.get('slender'):
            parent = by_id[solid['bone']]
        gap, root = attach(centres(parent), start)
        node = branch_base.get(solid['id'].replace('-tuft', '')) if scale == '200' else None
        if node and node[0] == solid['bone']:
            root = node[1]
        if math.dist(root, start) <= TOUCHING * radius:
            continue
        stem(solid, root, radius)
        done += 1
        gaps.append(gap)
    print('[DEBUG] scale', scale, 'stems', done, 'largest gap closed', round(max(gaps), 3) if gaps else 0)
open(SOURCE, 'w').write('{\n  "volumes": ' + json.dumps(volumes, ensure_ascii=False, separators=(',', ':')) + '\n}\n')
