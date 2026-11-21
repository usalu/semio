"""🪢 Smooths the limb and abdomen lofts of `humans/pregnant-person` at 1:100 and 1:200.

Round 3 stated these lofts through four or five hand-placed stations and the loft runs straight between them, so the legs read as faceted tubes (thigh, knee, calf and ankle as kinks). The stations stay exactly as stated; between each pair Catmull-Rom sections are inserted (three spans per gap at 1:100, two at 1:200), never thinner than 97 % of the thinner neighbour, so thigh, calf bulge and ankle taper run as continuous curves. Idempotent: it densifies only lofts that still carry the stated station count.

Usage: python3 r5-pregnant-person-smooth.py
@see ./r5-brief.md
"""
import json
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))
SOURCE = os.path.join(HERE, "volumes", "humans", "pregnant-person.json")
PLAN = {"100": (3, ["thigh-l", "thigh-r", "shank-l", "shank-r", "sleeve-l", "sleeve-r", "forearm-l", "forearm-r", "belly-cover", "belly-drape", "leggings-seat"], 7), "200": (2, ["thigh-l", "thigh-r", "shank-l", "shank-r", "sleeve-l", "sleeve-r", "forearm-l", "forearm-r"], 4)}


def cr(p0, p1, p2, p3, t):
    return 0.5 * (2 * p1 + (-p0 + p2) * t + (2 * p0 - 5 * p1 + 4 * p2 - p3) * t * t + (-p0 + 3 * p1 - 3 * p2 + p3) * t ** 3)


def dense(sections, k):
    out = []
    n = len(sections)
    for i in range(n - 1):
        a, b, c, d = sections[max(0, i - 1)], sections[i], sections[i + 1], sections[min(n - 1, i + 2)]
        out.append(b)
        for j in range(1, k):
            t = j / k
            s = {"at": [round(cr(a["at"][m], b["at"][m], c["at"][m], d["at"][m], t), 5) for m in range(3)]}
            for key in ("rx", "ry"):
                s[key] = round(max(min(b[key], c[key]) * 0.97, cr(a[key], b[key], c[key], d[key], t)), 5)
            if "exponent" in b or "exponent" in c:
                s["exponent"] = round(b.get("exponent", 2) + (c.get("exponent", 2) - b.get("exponent", 2)) * t, 4)
            if "rotation" in b and "rotation" in c:
                q = [b["rotation"][m] + (c["rotation"][m] - b["rotation"][m]) * t for m in range(4)]
                norm = math.sqrt(sum(x * x for x in q))
                s["rotation"] = [round(x / norm, 5) for x in q]
            elif "rotation" in b:
                s["rotation"] = b["rotation"]
            out.append(s)
    out.append(sections[-1])
    return out


doc = json.load(open(SOURCE, encoding="utf-8"))
for scale, (k, ids, limit) in PLAN.items():
    for solid in doc["volumes"][scale]["body"]:
        if solid["id"] in ids and solid["kind"] == "loft" and len(solid["sections"]) <= limit:
            before = len(solid["sections"])
            solid["sections"] = dense(solid["sections"], k)
            print(scale, solid["id"], before, "->", len(solid["sections"]))
open(SOURCE, "w", encoding="utf-8").write(json.dumps(doc) + "\n")
