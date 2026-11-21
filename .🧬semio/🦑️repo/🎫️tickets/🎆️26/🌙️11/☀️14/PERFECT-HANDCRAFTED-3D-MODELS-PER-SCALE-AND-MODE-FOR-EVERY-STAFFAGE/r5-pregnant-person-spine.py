"""🦴 Replaces the one-piece thoracolumbar spine of `humans/pregnant-person` at 1:50 by its 17 vertebrae (L5 to T1).

Each vertebral body is a short column on the spine curve of the round-3 skeleton (lumbar lordosis deepened by the pregnancy, thoracic kyphosis), lumbar bodies 1.35 times as high as thoracic ones, with the radius of the old column at that height and the discs closed so the column carries without a gap; each has its spinous process pointing back and down (lumbar processes short and level, thoracic ones long and steep). Works on the source `volumes/humans/pregnant-person.json` in place; running it twice gives the same result.

Usage: python3 r5-pregnant-person-spine.py
@see ./r5-brief.md
"""
import json
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))
SOURCE = os.path.join(HERE, "volumes", "humans", "pregnant-person.json")
SPINE = [(-0.01, 0.002), (0.04, 0.016), (0.09, 0.022), (0.14, 0.014), (0.19, -0.004), (0.24, -0.02), (0.29, -0.031), (0.34, -0.034), (0.39, -0.026), (0.43, -0.012), (0.458, 0)]
RADII = [0.022, 0.023, 0.023, 0.022, 0.02, 0.018, 0.016, 0.015, 0.014, 0.0125, 0.012]
NAMES = ["l5", "l4", "l3", "l2", "l1", "t12", "t11", "t10", "t9", "t8", "t7", "t6", "t5", "t4", "t3", "t2", "t1"]


def at(a):
    i = 0
    while i + 2 < len(SPINE) and SPINE[i + 1][0] < a:
        i += 1
    (a0, f0), (a1, f1) = SPINE[i], SPINE[i + 1]
    t = (a - a0) / (a1 - a0)
    return [a, -(f0 + (f1 - f0) * t), 0.0], RADII[i] + (RADII[i + 1] - RADII[i]) * t


def r5(x):
    return round(x, 5)


def vertebrae():
    weights = [1.35 if n.startswith("l") else 1.0 for n in NAMES]
    a, a1 = SPINE[0][0] + 0.004, SPINE[-1][0] - 0.004
    out = []
    for name, w in zip(NAMES, weights):
        h = (a1 - SPINE[0][0] - 0.004) * w / sum(weights)
        p, _ = at(a)
        q, _ = at(a + h)
        c, r = at(a + h / 2)
        length = math.dist(p, q)
        t = [(q[0] - p[0]) / length, (q[1] - p[1]) / length]
        g = max(0.001, length / 2 - r * 0.85)
        n = [-t[1], t[0]] if t[0] * 1 > 0 else [t[1], -t[0]]
        if n[1] < 0:
            n = [-n[0], -n[1]]
        body = {"id": "vertebra-" + name, "kind": "capsule", "bone": "torso", "material": "bone", "role": "body", "tone": 0.92, "p0": [r5(c[0] - t[0] * g), r5(c[1] - t[1] * g), 0], "p1": [r5(c[0] + t[0] * g), r5(c[1] + t[1] * g), 0], "r0": r5(r), "r1": r5(r), "member": "torso", "part": "member"}
        steep = 0.35 if name.startswith("l") else 0.95
        reach = 2.1 if name.startswith("l") else 2.5
        s0 = [c[0] + n[0] * r * 0.8, c[1] + n[1] * r * 0.8]
        s1 = [c[0] + n[0] * r * reach - t[0] * r * steep, c[1] + n[1] * r * reach - t[1] * r * steep]
        proc = {"id": "spinous-" + name, "kind": "capsule", "bone": "torso", "material": "bone", "role": "body", "tone": 0.92, "p0": [r5(s0[0]), r5(s0[1]), 0], "p1": [r5(s1[0]), r5(s1[1]), 0], "r0": 0.0055 if name.startswith("l") else 0.0045, "r1": 0.0045 if name.startswith("l") else 0.004, "member": "torso", "part": "member"}
        out += [body, proc]
        a += h
    return out


doc = json.load(open(SOURCE, encoding="utf-8"))
lst = doc["volumes"]["50"]["structure"]
first = next(i for i, s in enumerate(lst) if s["id"] in ("spine", "vertebra-l5"))
kept = [s for s in lst if not (s["id"] == "spine" or s["id"].startswith("spinous-process-") or s["id"].startswith("vertebra-") or s["id"].startswith("spinous-"))]
doc["volumes"]["50"]["structure"] = kept[:first] + vertebrae() + kept[first:]
open(SOURCE, "w", encoding="utf-8").write(json.dumps(doc) + "\n")
print("1:50 structure", len(doc["volumes"]["50"]["structure"]), "solids")
