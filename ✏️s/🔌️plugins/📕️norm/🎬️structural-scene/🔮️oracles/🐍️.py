"""📐️ Geometry oracle for the structural-scene tessellation kernel.

Reads {"cases": [...]} on stdin and prints {"failures": [...], "references": {...}}. Every case names a parametric profile and
carries the Rust figures (area, centroid, second moments, mesh positions/indices); this script rebuilds the section from its
parameters with shapely rectangle booleans and textbook rectangle-decomposition moments (independent of the Rust Green's-theorem
code), checks the extruded mesh volume, watertightness and bounds with numpy, and cross-checks with trimesh when it is installed.
"""
import json
import sys

import numpy as np
import shapely
from shapely.geometry import box
from shapely.ops import unary_union

try:
    import trimesh
except ImportError:
    trimesh = None


def rects(profile, q):
    h, b, tw, tf, t, c = (q.get(k, 0.0) for k in ("h", "b", "tw", "tf", "t", "c"))
    table = {
        "ishape": [(0, 0, b, tf, 1), ((b - tw) / 2, tf, (b + tw) / 2, h - tf, 1), (0, h - tf, b, h, 1)],
        "rect": [(0, 0, b, h, 1)],
        "rhs": [(0, 0, b, h, 1), (t, t, b - t, h - t, -1)],
        "channel": [(0, 0, tw, h, 1), (tw, 0, b, tf, 1), (tw, h - tf, b, h, 1)],
        "tee": [(0, h - tf, b, h, 1), ((b - tw) / 2, 0, (b + tw) / 2, h - tf, 1)],
        "angle": [(0, 0, b, t, 1), (0, t, t, h, 1)],
        "cee": [(0, 0, t, h, 1), (t, 0, b, t, 1), (t, h - t, b, h, 1), (b - t, t, b, c, 1), (b - t, h - c, b, h - t, 1)],
        "zed": [(0, 0, t, h, 1), (t, 0, b, t, 1), (t - b, h - t, 0, h, 1), (b - t, t, b, c, 1), (t - b, h - c, 2 * t - b, h - t, 1)],
    }
    return table[profile]


def section(profile, q):
    parts = rects(profile, q)
    solid = unary_union([box(*r[:4]) for r in parts if r[4] > 0])
    voids = [box(*r[:4]) for r in parts if r[4] < 0]
    geometry = solid.difference(unary_union(voids)) if voids else solid
    area = sum(r[4] * (r[2] - r[0]) * (r[3] - r[1]) for r in parts)
    cx = sum(r[4] * (r[2] - r[0]) * (r[3] - r[1]) * (r[0] + r[2]) / 2 for r in parts) / area
    cy = sum(r[4] * (r[2] - r[0]) * (r[3] - r[1]) * (r[1] + r[3]) / 2 for r in parts) / area
    ixx = sum(r[4] * ((r[2] - r[0]) * (r[3] - r[1]) ** 3 / 12 + (r[2] - r[0]) * (r[3] - r[1]) * ((r[1] + r[3]) / 2 - cy) ** 2) for r in parts)
    iyy = sum(r[4] * ((r[3] - r[1]) * (r[2] - r[0]) ** 3 / 12 + (r[2] - r[0]) * (r[3] - r[1]) * ((r[0] + r[2]) / 2 - cx) ** 2) for r in parts)
    ixy = sum(r[4] * (r[2] - r[0]) * (r[3] - r[1]) * ((r[0] + r[2]) / 2 - cx) * ((r[1] + r[3]) / 2 - cy) for r in parts)
    x0, y0, x1, y1 = geometry.bounds
    return geometry, area, cx - (x0 + x1) / 2, cy - (y0 + y1) / 2, ixx, iyy, ixy


def close(a, b, rel, absolute=1e-12):
    return abs(a - b) <= rel * max(abs(a), abs(b)) + absolute


def circular(profile, q, n):
    d = q["d"]
    outer = n / 2 * (d / 2) ** 2 * np.sin(2 * np.pi / n)
    inner = 0.0 if profile == "circle" else n / 2 * (d / 2 - q["t"]) ** 2 * np.sin(2 * np.pi / n)
    exact = np.pi / 4 * (d ** 2 - (0 if profile == "circle" else (d - 2 * q["t"]) ** 2))
    ring = np.pi / 64 * (d ** 4 - (0 if profile == "circle" else (d - 2 * q["t"]) ** 4))
    return outer - inner, exact, ring


def mesh_checks(case, area, failures):
    pos = np.asarray(case["positions"], dtype=float).reshape(-1, 3)
    idx = np.asarray(case["indices"], dtype=int).reshape(-1, 3)
    a, b, c = pos[idx[:, 0]], pos[idx[:, 1]], pos[idx[:, 2]]
    volume = float(np.einsum("ij,ij->i", a, np.cross(b, c)).sum() / 6.0)
    if not close(volume, area * case["length"], 2e-5):
        failures.append(f"{case['name']}: mesh volume {volume} != area*length {area * case['length']}")
    key = np.round(pos * 1e5).astype(np.int64)
    ids = {}
    for row in map(tuple, key):
        ids.setdefault(row, len(ids))
    face = np.array([[ids[tuple(key[i])] for i in tri] for tri in idx])
    edges = {}
    for tri in face:
        for k in range(3):
            e = (tri[k], tri[(k + 1) % 3])
            edges[e] = edges.get(e, 0) + 1
    if not all(n == 1 and edges.get((e[1], e[0])) == 1 for e, n in edges.items()):
        failures.append(f"{case['name']}: mesh is not watertight")
    lo, hi = pos.min(axis=0), pos.max(axis=0)
    if not (np.allclose(lo, case["aabb"][:3], atol=1e-6) and np.allclose(hi, case["aabb"][3:], atol=1e-6)):
        failures.append(f"{case['name']}: reported bounds differ from the vertices")
    if trimesh is not None:
        tm = trimesh.Trimesh(vertices=pos, faces=idx, process=True)
        if not close(float(tm.volume), area * case["length"], 2e-5) or not tm.is_watertight:
            failures.append(f"{case['name']}: trimesh volume {tm.volume} watertight {tm.is_watertight}")


def run(cases):
    failures = []
    for case in cases:
        name, profile, q, r = case["name"], case["profile"], case["params"], case["rust"]
        if profile in ("chs", "circle"):
            n = case["segments"]
            polygon, exact, ring = circular(profile, q, n)
            bound = 1.5 * (2 * np.pi / n) ** 2 / 6
            if not close(r["area"], polygon, 1e-9) or not close(r["area"], exact, bound):
                failures.append(f"{name}: area {r['area']} vs polygon {polygon} / circle {exact}")
            if not close(r["ixx"], ring, 3 * bound) or not close(r["ixx"], r["iyy"], 1e-6):
                failures.append(f"{name}: second moment {r['ixx']} vs circle {ring}")
            area = r["area"]
        else:
            geometry, area, cu, cv, ixx, iyy, ixy = section(profile, q)
            if not close(shapely.area(geometry), area, 1e-9):
                failures.append(f"{name}: shapely union area {shapely.area(geometry)} != decomposition {area}")
            fillets = case.get("fillets", 0)
            area += fillets * (1 - np.pi / 4) * case.get("radius", 0.0) ** 2
            if not close(r["area"], area, 2e-3 if fillets else 1e-9):
                failures.append(f"{name}: area {r['area']} != {area}")
            if not fillets:
                for key, want in (("cu", cu), ("cv", cv), ("ixx", ixx), ("iyy", iyy), ("ixy", ixy)):
                    if not close(r[key], want, 1e-8, 1e-14):
                        failures.append(f"{name}: {key} {r[key]} != {want}")
        if "positions" in case:
            mesh_checks(case, area, failures)
    return failures


def main():
    cases = json.load(sys.stdin)["cases"]
    refs = {"shapely": shapely.__version__, "numpy": np.__version__, "trimesh": getattr(trimesh, "__version__", None)}
    print(json.dumps({"failures": run(cases), "references": refs, "cases": len(cases)}))


if __name__ == "__main__":
    main()
