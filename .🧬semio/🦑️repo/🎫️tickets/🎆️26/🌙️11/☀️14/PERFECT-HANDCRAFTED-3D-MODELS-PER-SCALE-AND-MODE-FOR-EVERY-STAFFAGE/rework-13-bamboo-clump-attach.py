"""🔗 Measures, per bone, which body polyhedra of vegetation/bamboo-clump are not connected to the culm solid of their bone through any chain of solids (sampled surface points sharing a grid cell of the tolerance or a neighbouring one).

Usage: python3 rework-13-bamboo-clump-attach.py <scale> [toleranceM]
"""
import itertools
import json
import sys

scale, tol = sys.argv[1], float(sys.argv[2]) if len(sys.argv) > 2 else 0.006
body = json.load(open('volumes/vegetation/bamboo-clump.json'))['volumes'][scale]['body']
NEAR = list(itertools.product((-1, 0, 1), repeat=3))


def cells(solid):
    v, out = solid['vertices'], set()
    for i, j, k in solid['triangles']:
        a, b, c = v[i], v[j], v[k]
        n = max(1, int(max(abs(a[m] - b[m]) + abs(a[m] - c[m]) for m in range(3)) / tol))
        for p in range(n + 1):
            for q in range(n + 1 - p):
                s, t = p / n, q / n
                out.add(tuple(int((a[m] + s * (b[m] - a[m]) + t * (c[m] - a[m])) // tol) for m in range(3)))
    return out


groups = {}
for solid in body:
    if solid['kind'] == 'polyhedron':
        groups.setdefault(solid['bone'], []).append(solid)
loose, total = [], 0
for bone, solids in groups.items():
    total += len(solids)
    own = [cells(s) for s in solids]
    grid = {}
    for index, cs in enumerate(own):
        for c in cs:
            grid.setdefault(c, set()).add(index)
    reached = {i for i, s in enumerate(solids) if s['id'] == bone}
    front = list(reached)
    while front:
        i = front.pop()
        for c in own[i]:
            for d in NEAR:
                for j in grid.get((c[0] + d[0], c[1] + d[1], c[2] + d[2]), ()):
                    if j not in reached:
                        reached.add(j)
                        front.append(j)
    loose += [(s['id'], bone) for j, s in enumerate(solids) if j not in reached]
print('scale', scale, 'tolerance', tol, 'polyhedra', total, 'loose', len(loose))
print(' ', loose[:30])
