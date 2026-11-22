"""🌿 Multi-stem birch, crown as a veil: every leafy sleeve and every hanging tress restated about its own hand-placed centre line with the section of a real Betula pendula spray (sleeve of short shoots about 0.3 m across round its bough, pendulous tress a strand of 0.1 to 0.17 m that is fullest in its lower third and ends blunt) instead of 0.7 m caps and 0.4 m carrots. The strands thicken with the scale (0.16 m at 1:50 and 1:100, 0.2 m at 1:200, 0.23 m at 1:500, as far as the agreement between neighbouring scales allows) so that neighbours close into the curtains a coarse drawing can print. The restatement works from the centre lines and the relative taper only, so it can be repeated: `python3 rework-multi-stem-birch-veil.py 200 500` restates those scales; `🗑️generated/rework-multi-stem-birch/source-before-veil.json` is the state before."""
import json
import math
import os
import re
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
SOURCE = os.path.join(HERE, "volumes", "vegetation", "multi-stem-birch.json")
SLEEVE_RADIUS_M = {"50": 0.15, "100": 0.15, "200": 0.18, "500": 0.2}
SLEEVE_START_M = {"50": 0.03, "100": 0.03, "200": 0.035, "500": 0.05}
SIDE_SLEEVE_RADIUS_M = {"50": 0.12, "100": 0.12, "200": 0.145, "500": 0.16}
TRESS_RADIUS_M = {"50": 0.08, "100": 0.08, "200": 0.1, "500": 0.115}
TRESS_SHAPE = [(0.0, 0.3), (0.12, 0.5), (0.35, 0.72), (0.6, 0.92), (0.78, 1.0), (0.92, 0.82), (1.0, 0.42)]
TRESS_FLOOR = {"50": 0.3, "100": 0.3, "200": 0.3, "500": 0.45}
ROUNDNESS = 0.8
TRESS = re.compile(r"^leaf-.*-\d+$")
SIDE = re.compile(r"^leaf-.*-v\d+$")
SLEEVE = re.compile(r"^leaf-.*-s$")


def shape(s, floor):
    for (s0, r0), (s1, r1) in zip(TRESS_SHAPE, TRESS_SHAPE[1:]):
        if s <= s1:
            return max(floor, r0 + (r1 - r0) * (s - s0) / (s1 - s0))
    return max(floor, TRESS_SHAPE[-1][1])


def restate(solid, radius_of):
    vertices, n = solid["vertices"], solid["triangles"][0][2]
    count = (len(vertices) - 2) // n
    assert count * n + 2 == len(vertices) and count >= 3, solid["id"]
    rings = [vertices[i * n:(i + 1) * n] for i in range(count)]
    centres = [[sum(p[j] for p in ring) / n for j in range(3)] for ring in rings]
    along = [0.0]
    for a, b in zip(centres, centres[1:]):
        along.append(along[-1] + math.dist(a, b))
    olds = [[math.dist(p, c) for p in ring] for ring, c in zip(rings, centres)]
    fattest = max(max(r) for r in olds)
    for i, (ring, c, old) in enumerate(zip(rings, centres, olds)):
        target = radius_of(along[i] / along[-1], max(old) / fattest)
        low, high = min(old), max(old)
        for p, r in zip(ring, old):
            new = target * (ROUNDNESS + (1 - ROUNDNESS) * ((r - low) / (high - low) if high - low > 1e-6 else 1))
            for j in range(3):
                p[j] = round(c[j] + (p[j] - c[j]) * new / r, 5)
    return along[-1]


def main():
    source = json.load(open(SOURCE))
    for scale, volume in source["volumes"].items():
        if sys.argv[1:] and scale not in sys.argv[1:]:
            continue
        done = {"sleeve": 0, "side": 0, "tress": 0, "left": []}
        for solid in volume["body"]:
            name = solid["id"]
            if solid.get("kind") != "polyhedron" or solid.get("material") != "birch-leaf":
                continue
            if TRESS.match(name):
                restate(solid, lambda s, k: TRESS_RADIUS_M[scale] * shape(s, TRESS_FLOOR[scale]))
                done["tress"] += 1
            elif SIDE.match(name):
                restate(solid, lambda s, k: max(SLEEVE_START_M[scale], SIDE_SLEEVE_RADIUS_M[scale] * k))
                done["side"] += 1
            elif SLEEVE.match(name):
                restate(solid, lambda s, k: max(SLEEVE_START_M[scale], SLEEVE_RADIUS_M[scale] * k))
                done["sleeve"] += 1
            else:
                done["left"].append(name)
        print("[DEBUG]", scale, done)
    json.dump(source, open(SOURCE, "w"), ensure_ascii=False, separators=(",", ":"))


if __name__ == "__main__":
    main()
