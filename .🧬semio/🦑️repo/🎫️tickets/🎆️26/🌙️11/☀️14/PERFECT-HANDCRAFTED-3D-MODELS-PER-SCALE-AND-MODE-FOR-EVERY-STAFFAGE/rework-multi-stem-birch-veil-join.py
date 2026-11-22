"""🪢 Multi-stem birch, after the veil: side-shoot sleeves start on the centre line of the sleeve that bears them, four tightly curved top sleeves of 1:50 are restated with sections square to a smoothed centre line (their old oblique sections crossed each other once round), and the foliage mass markers sit on the centre line of their sleeve at a radius inside it."""
import json
import math
import os
import re

HERE = os.path.dirname(os.path.abspath(__file__))
SOURCE = os.path.join(HERE, "volumes", "vegetation", "multi-stem-birch.json")
SQUARE_50 = ["leaf-tl4-s", "leaf-tl5-s", "leaf-tl6-s", "leaf-tr4-s"]
SQUARE_RINGS = 7
MASS_RADIUS_M = 0.085
SIDE = re.compile(r"^(leaf-.*)-v\d+$")


def sub(a, b): return [a[i] - b[i] for i in range(3)]
def add(a, b, k=1.0): return [a[i] + b[i] * k for i in range(3)]
def dot(a, b): return sum(a[i] * b[i] for i in range(3))
def cross(a, b): return [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
def unit(a):
    n = math.sqrt(dot(a, a))
    return [x / n for x in a]


def rings_of(solid):
    n = solid["triangles"][0][2]
    v = solid["vertices"]
    count = (len(v) - 2) // n
    rings = [v[i * n:(i + 1) * n] for i in range(count)]
    centres = [[sum(p[j] for p in ring) / n for j in range(3)] for ring in rings]
    radii = [max(math.dist(p, c) for p in ring) for ring, c in zip(rings, centres)]
    return n, rings, centres, radii


def nearest(point, line):
    best = None
    for a, b in zip(line, line[1:]):
        d = sub(b, a)
        t = max(0.0, min(1.0, dot(sub(point, a), d) / dot(d, d)))
        q = add(a, d, t)
        if best is None or math.dist(q, point) < math.dist(best, point):
            best = q
    return best


def square(solid):
    n, _, centres, radii = rings_of(solid)
    line = [centres[0]] + [[(a[j] + 2 * b[j] + c[j]) / 4 for j in range(3)] for a, b, c in zip(centres, centres[1:], centres[2:])] + [centres[-1]]
    along = [0.0]
    for a, b in zip(line, line[1:]):
        along.append(along[-1] + math.dist(a, b))
    def at(s, values):
        for i in range(len(along) - 1):
            if s <= along[i + 1] + 1e-9:
                t = (s - along[i]) / (along[i + 1] - along[i])
                return [values[i][j] + (values[i + 1][j] - values[i][j]) * t for j in range(len(values[i]))]
    stations = [along[-1] * k / (SQUARE_RINGS - 1) for k in range(SQUARE_RINGS)]
    points = [at(s, line) for s in stations]
    sizes = [at(s, [[r] for r in radii])[0] for s in stations]
    side = None
    vertices = []
    for k, (c, r) in enumerate(zip(points, sizes)):
        tangent = unit(sub(points[min(k + 1, SQUARE_RINGS - 1)], points[max(k - 1, 0)]))
        seed = side if side is not None else ([0, 1, 0] if abs(tangent[1]) < 0.9 else [1, 0, 0])
        side = unit(sub(seed, [tangent[j] * dot(seed, tangent) for j in range(3)]))
        other = cross(tangent, side)
        for i in range(n):
            a = 2 * math.pi * i / n
            vertices.append([round(c[j] + r * (math.cos(a) * side[j] + 0.85 * math.sin(a) * other[j]), 5) for j in range(3)])
    first, last = len(vertices), len(vertices) + 1
    vertices += [[round(x, 5) for x in points[0]], [round(x, 5) for x in points[-1]]]
    triangles = []
    for k in range(SQUARE_RINGS - 1):
        for i in range(n):
            a, b = k * n + i, k * n + (i + 1) % n
            triangles += [[a, b, a + n], [b, b + n, a + n]]
    base = (SQUARE_RINGS - 1) * n
    for i in range(n):
        triangles += [[first, (i + 1) % n, i], [last, base + i, base + (i + 1) % n]]
    volume = sum(dot(vertices[a], cross(vertices[b], vertices[c])) for a, b, c in triangles) / 6
    if volume < 0:
        triangles = [[a, c, b] for a, b, c in triangles]
    solid["vertices"], solid["triangles"] = vertices, triangles


def main():
    source = json.load(open(SOURCE))
    for scale, volume in source["volumes"].items():
        body = {s["id"]: s for s in volume["body"]}
        if scale == "50":
            for name in SQUARE_50:
                if (len(body[name]["vertices"]) - 2) // body[name]["triangles"][0][2] != SQUARE_RINGS:
                    square(body[name])
        moved = 0
        for name, solid in body.items():
            match = SIDE.match(name)
            if not match or solid.get("kind") != "polyhedron":
                continue
            parent = body.get(match.group(1) + "-s")
            n, rings, centres, _ = rings_of(solid)
            target = nearest(centres[0], rings_of(parent)[2])
            delta = sub(target, centres[0])
            for share, ring in ((1.0, rings[0]), (0.4, rings[1])):
                for p in ring:
                    for j in range(3):
                        p[j] = round(p[j] + delta[j] * share, 5)
            start = min((len(solid["vertices"]) - 2, len(solid["vertices"]) - 1), key=lambda i: math.dist(solid["vertices"][i], centres[0]))
            solid["vertices"][start] = [round(x, 5) for x in target]
            moved += 1
        placed = 0
        for mark in volume["structure"]:
            if not mark["id"].startswith("mass-fol") or "vertices" not in mark:
                continue
            sleeves = [s for s in volume["body"] if s.get("bone") == mark["bone"] and s["id"].endswith("-s") and s.get("kind") == "polyhedron"]
            at = mark["transform"]["translation"]
            home = min((nearest(at, rings_of(s)[2]) for s in sleeves), key=lambda q: math.dist(q, at))
            v = mark["vertices"]
            size = max(math.hypot(*p) for p in v)
            mark["vertices"] = [[round(x * MASS_RADIUS_M / size, 5) for x in p] for p in v]
            mark["transform"]["translation"] = [round(x, 5) for x in home]
            placed += 1
        print("[DEBUG]", scale, "side sleeves joined", moved, "mass markers placed", placed)
    json.dump(source, open(SOURCE, "w"), ensure_ascii=False, separators=(",", ":"))


if __name__ == "__main__":
    main()
