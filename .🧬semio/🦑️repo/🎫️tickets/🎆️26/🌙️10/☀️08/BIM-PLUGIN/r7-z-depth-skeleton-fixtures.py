"""🦴️ Wave Z depth: writes the language-agnostic fixtures of the straight skeleton and its JSON schema.

Run from the repo root with the test environment: `.venv/Scripts/python.exe .🧬semio/.../r7-z-depth-skeleton-fixtures.py`.

The oracle logic lives once, in the permanent test `🧰️framework/🔨️modules/📐️geometry/🧪️tests/🦴️skeleton-oracles/🐍️.py`, which this
script imports: expected values come from `py_straight_skeleton` (face areas) and `shapely` (unswept areas), never from our
implementation. This script only chooses the cases and writes the files.
"""
import importlib.util
import json
import math
import random
import sys
from pathlib import Path

sys.stdout.reconfigure(encoding='utf-8')
from shapely.geometry import Polygon

ROOT = Path(__file__).resolve().parents[7]
GEOMETRY = ROOT / '🧰️framework' / '🔨️modules' / '📐️geometry'
ORACLE = GEOMETRY / '🧪️tests' / '🦴️skeleton-oracles' / '🐍️.py'
spec = importlib.util.spec_from_file_location('skeleton_oracles', ORACLE)
oracle = importlib.util.module_from_spec(spec)
spec.loader.exec_module(oracle)
FIXTURE, SCHEMA = oracle.FIXTURE, oracle.SCHEMA


def write(path, text):
    path.parent.mkdir(parents=True, exist_ok=True)
    temp = path.with_name(path.name + '.tmp')
    temp.write_text(text, encoding='utf-8')
    temp.replace(path)


def rounded(levels):
    return [[round(t, 12), round(a, 9)] for t, a in levels]


def case(name, outer, holes=(), speeds=None, horizon=None, allow_single=False):
    holes = [list(h) for h in holes]
    outer = [list(p) for p in outer]
    entry = {'name': name, 'outer': outer, 'holes': holes, 'speeds': speeds, 'peak': None, 'faces': None, 'levels': None, 'oracles': []}
    total = oracle.region(outer, holes).area
    if speeds is not None:
        entry['levels'] = rounded(oracle.half_plane_levels(outer, speeds[0], horizon))
        entry['oracles'] = ['shapely-half-plane-intersection']
        return entry
    faces = oracle.py_faces(outer, holes)
    if faces is not None:
        order = oracle.input_order(outer, holes, faces)
        peak = max(t for _, poly in faces.values() for _, _, t in poly)
        entry['faces'] = [round(faces[key][0], 9) for key in order]
        entry['peak'] = round(peak, 9)
        levels = oracle.levels_from_faces(faces, total, peak)
        entry['levels'] = rounded(levels)
        entry['oracles'] = ['py-straight-skeleton']
        buffered = oracle.buffer_levels(outer, holes, peak)
        if all(abs(a[1] - b[1]) <= 1e-6 * max(1.0, total) for a, b in zip(levels, buffered)):
            entry['oracles'].append('shapely-mitre-buffer')
        return entry
    if not allow_single:
        return None
    low, high = 0.0, max(Polygon(outer).bounds)
    for _ in range(60):
        mid = (low + high) / 2
        if oracle.region(outer, holes).buffer(-mid, join_style=2, mitre_limit=1e9).area > 1e-12:
            low = mid
        else:
            high = mid
    entry['peak'] = round(low, 6)
    entry['levels'] = rounded(oracle.buffer_levels(outer, holes, low))
    entry['oracles'] = ['shapely-mitre-buffer']
    return entry


def star_polygon(rng, n, spread):
    points = []
    for k in range(n):
        angle = (k + 0.05 + 0.9 * rng.random()) / n * math.tau
        radius = 1 + 10 * (spread * rng.random() + (1 - spread) * 0.5)
        points.append([round(radius * math.cos(angle), 6), round(radius * math.sin(angle), 6)])
    return points


def histogram(rng):
    columns = 2 + int(rng.random() * 7)
    heights = [1 + int(rng.random() * 5) for _ in range(columns)]
    points = [[0, 0], [columns, 0]]
    previous = None
    for k in range(columns - 1, -1, -1):
        if previous != heights[k]:
            points.append([k + 1, heights[k]])
        points.append([k, heights[k]])
        previous = heights[k]
    cleaned = []
    for i, p in enumerate(points):
        a, b = points[i - 1], points[(i + 1) % len(points)]
        if (p[0] - a[0]) * (b[1] - p[1]) - (p[1] - a[1]) * (b[0] - p[0]) != 0:
            cleaned.append(p)
    return cleaned


HAND = [
    ('rectangle 10x4', [[0, 0], [10, 0], [10, 4], [0, 4]], []),
    ('square 4x4', [[0, 0], [4, 0], [4, 4], [0, 4]], []),
    ('right triangle 6x8', [[0, 0], [6, 0], [0, 8]], []),
    ('clockwise rectangle', [[0, 4], [10, 4], [10, 0], [0, 0]], []),
    ('L shape', [[0, 0], [6, 0], [6, 2], [2, 2], [2, 6], [0, 6]], []),
    ('U shape', [[0, 0], [9, 0], [9, 6], [6, 6], [6, 2], [3, 2], [3, 6], [0, 6]], []),
    ('T shape', [[0, 4], [3, 4], [3, 0], [5, 0], [5, 4], [8, 4], [8, 6], [0, 6]], []),
    ('plus shape', [[2, 0], [4, 0], [4, 2], [6, 2], [6, 4], [4, 4], [4, 6], [2, 6], [2, 4], [0, 4], [0, 2], [2, 2]], []),
    ('H shape', [[0, 0], [2, 0], [2, 3], [5, 3], [5, 0], [7, 0], [7, 8], [5, 8], [5, 5], [2, 5], [2, 8], [0, 8]], []),
    ('comb with four teeth', [[0, 0], [11, 0], [11, 5], [9, 5], [9, 2], [7, 2], [7, 5], [5, 5], [5, 2], [3, 2], [3, 5], [1, 5], [1, 2], [0, 2]], []),
    ('arrow head', [[0, 0], [8, 3], [0, 6], [2, 3]], []),
    ('frame with square hole', [[0, 0], [10, 0], [10, 10], [0, 10]], [[[3, 3], [3, 7], [7, 7], [7, 3]]]),
    ('rectangle with off-centre hole', [[0, 0], [12, 0], [12, 8], [0, 8]], [[[2, 2], [2, 4], [5, 4], [5, 2]]]),
    ('L shape with hole', [[0, 0], [12, 0], [12, 4], [6, 4], [6, 12], [0, 12]], [[[1.5, 1.5], [1.5, 3], [3.5, 3], [3.5, 1.5]]]),
    ('square with two holes', [[0, 0], [14, 0], [14, 8], [0, 8]], [[[2, 2], [2, 6], [5, 6], [5, 2]], [[8, 2], [8, 6], [12, 6], [12, 2]]]),
]

WEIGHTED = [
    ('hip rectangle with a steep and a shallow side', [[0, 0], [10, 0], [10, 6], [0, 6]], [[1.0, 1.0, 2.0, 1.0]], 1.9),
    ('convex pentagon with five speeds', [[0, 0], [9, 0], [11, 5], [4, 8], [-2, 4]], [[1.0, 0.7, 1.3, 0.5, 1.0]], 1.0),
    ('convex quadrilateral with one vertical edge', [[0, 0], [10, 0], [8, 6], [0, 6]], [[1.2, 0.8, 1.2, 0.0]], 2.0),
]


def main():
    rng = random.Random(0x5EED)
    cases = []
    for name, outer, holes in HAND:
        entry = case(name, outer, holes, allow_single=True)
        assert entry is not None, name
        cases.append(entry)
    for name, outer, speeds, horizon in WEIGHTED:
        cases.append(case(name, outer, speeds=speeds, horizon=horizon))
    kept = 0
    attempts = 0
    while kept < 24 and attempts < 200:
        attempts += 1
        polygon = star_polygon(rng, 5 + int(rng.random() * 14), rng.random())
        if not Polygon(polygon).is_valid:
            continue
        entry = case(f'random star {kept}', polygon)
        if entry is not None:
            cases.append(entry)
            kept += 1
    kept = 0
    while kept < 12:
        entry = case(f'random histogram {kept}', histogram(rng))
        if entry is not None:
            cases.append(entry)
            kept += 1
    write(FIXTURE, json.dumps(cases, indent=1, ensure_ascii=False) + '\n')
    point = {'type': 'array', 'minItems': 2, 'maxItems': 2, 'items': {'type': 'number'}}
    ring = {'type': 'array', 'minItems': 3, 'items': point}
    schema = {
        '$schema': 'http://json-schema.org/draft-07/schema#',
        '$id': 'https://semio.tech/framework/geometry/skeleton',
        'type': 'array',
        'items': {
            'type': 'object',
            'required': ['name', 'outer', 'holes', 'speeds', 'peak', 'faces', 'levels', 'oracles'],
            'additionalProperties': False,
            'properties': {
                'name': {'type': 'string', 'minLength': 1},
                'outer': ring,
                'holes': {'type': 'array', 'items': ring},
                'speeds': {'type': ['array', 'null'], 'items': {'type': 'array', 'items': {'type': 'number', 'minimum': 0}}},
                'peak': {'type': ['number', 'null'], 'minimum': 0},
                'faces': {'type': ['array', 'null'], 'items': {'type': 'number', 'minimum': 0}},
                'levels': {'type': ['array', 'null'], 'items': {'type': 'array', 'minItems': 2, 'maxItems': 2, 'items': {'type': 'number'}}},
                'oracles': {'type': 'array', 'items': {'enum': ['py-straight-skeleton', 'shapely-mitre-buffer', 'shapely-half-plane-intersection']}},
            },
        },
    }
    write(SCHEMA, json.dumps(schema, indent=2, ensure_ascii=False) + '\n')
    print('cases', len(cases), 'with py', sum('py-straight-skeleton' in c['oracles'] for c in cases), 'both', sum(len(c['oracles']) == 2 for c in cases))


main()
