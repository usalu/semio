"""👁️ Audit candidate oriented-boundary silhouettes against independent GEOS unions."""
from collections import Counter
import json
from pathlib import Path
from shapely.geometry import Polygon
from shapely.ops import polygonize, unary_union
import shapely

cases = {
    "near-column": [[(2, 0), (4, 0), (4, 2)], [(2, 0), (4, 2), (2, 2)]],
    "far-wall": [[(0, 0), (6, 0), (6, 1)], [(0, 0), (6, 1), (0, 1)]],
    "overlapping-fronts": [[(0, 0), (4, 0), (4, 2)], [(0, 0), (4, 2), (0, 2)], [(2, -1), (6, -1), (6, 1)], [(2, -1), (6, 1), (2, 1)]],
    "coincident-fronts": [[(0, 0), (4, 0), (0, 2)], [(0, 0), (4, 0), (0, 2)]],
    "disjoint-fronts": [[(0, 0), (1, 0), (0, 1)], [(2, 0), (3, 0), (2, 1)]],
    "hole": [[(0, 0), (4, 0), (3, 1)], [(0, 0), (3, 1), (1, 1)], [(4, 0), (4, 4), (3, 3)], [(4, 0), (3, 3), (3, 1)], [(4, 4), (0, 4), (1, 3)], [(4, 4), (1, 3), (3, 3)], [(0, 4), (0, 0), (1, 1)], [(0, 4), (1, 1), (1, 3)]],
}

for name, triangles in cases.items():
    polygons = [Polygon(triangle) for triangle in triangles]
    union = unary_union(polygons)
    edges = Counter()
    for polygon in polygons:
        points = list(polygon.exterior.coords)[:-1]
        if not polygon.exterior.is_ccw:
            points.reverse()
        for a, b in zip(points, points[1:] + points[:1]):
            edges[(a, b)] += 1
    boundary = [(a, b) for (a, b), count in edges.items() if count == 1 and edges[(b, a)] == 0]
    simple = all(count == 1 for count in edges.values()) and len(boundary) >= 3
    candidates = list(polygonize(boundary)) if simple else []
    candidate = candidates[0] if len(candidates) == 1 and len(candidates[0].interiors) == 0 else None
    accepted = candidate is not None and len(boundary) == len(candidate.exterior.coords) - 1
    if accepted:
        assert candidate.symmetric_difference(union).area < 1e-9, name
    print(f"{name}: GEOS area={union.area:g}, components={len(union.geoms) if hasattr(union, 'geoms') else 1}, candidate={'accept' if accepted else 'fallback'}")

near = Polygon([(2, 0), (4, 0), (4, 2), (2, 2)])
far = Polygon([(0, 0), (6, 0), (6, 1), (0, 1)])
assert near.area == 4 and far.area == 6 and near.intersection(far).area == 2
print(f"near/far: coverage={near.intersection(far).area:g}, shapely={shapely.__version__}")

fixture = Path(__file__).with_name("🔺️projected-faces.json")
for case in json.loads(fixture.read_text())["cases"]:
    result = unary_union([Polygon([(x, y) for x, y, _ in triangle]) for triangle in case["triangles"]])
    parts = list(result.geoms) if hasattr(result, "geoms") else [result]
    assert abs(result.area - case["area"]) < 1e-9, case["name"]
    assert len(parts) == case["components"] and sum(len(part.interiors) for part in parts) == case["holes"], case["name"]
    assert all(result.contains(__import__("shapely").geometry.Point(x, y)) == expected for x, y, expected in case["probes"]), case["name"]
    print(f"fixture {case['name']}: area={result.area:g}, components={len(parts)}, holes={sum(len(part.interiors) for part in parts)}")
