"""🧶 Adds the knit that hangs from the bust onto the abdomen of `humans/pregnant-person` at 1:50, 1:100 and 1:200.

Round 3 modelled the stretched knit over the abdomen as a dome that leaves the sweater front almost at a right angle, so a ledge and a hard crease ran under the bust. A knit sweater over a week-36 abdomen falls from the bust apex in a straight tent onto the upper curve of the abdomen. `belly-drape` is that tent: horizontal slices of the standing figure (default pose `hand-on-back`) from just above the bust apex, where it starts inside the sweater, down to where it meets the abdomen tangentially and ends inside the abdomen knit; its back and flanks follow the sweater just inside it, so only the front stands out: the knit front is carried forward from the bust, not a panel laid on it. It is bound to the torso, belongs to the sweater region (`sweater`, layer `knit`: it is the front of the sweater carried forward, and inside the sweater it is buried by its own region, so only its exposed front counts as sweater surface) and gets fewer slices at the coarser scales (1:500 shows no ledge: the step is below its 75 mm feature). Works on the source in place and is idempotent.

Usage: python3 r5-pregnant-person-drape.py
@see ./r5-brief.md
"""
import json
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))
SOURCE = os.path.join(HERE, "volumes", "humans", "pregnant-person.json")
O = (-0.02510472610110416, 0.983927224132304)
U = (-0.08278037812115147, 0.99656781455058)
V = (-0.99656781455058, -0.08278037812115147)
SLICES = {
    50: [(1.27, -0.15, 0.126, 0.19, 2.4), (1.25, -0.148, 0.136, 0.188, 2.45), (1.22, -0.14, 0.159, 0.182, 2.5), (1.19, -0.13, 0.182, 0.17, 2.5), (1.16, -0.11, 0.204, 0.15, 2.5), (1.13, -0.08, 0.222, 0.12, 2.4)],
    100: [(1.27, -0.15, 0.126, 0.19, 2.4), (1.235, -0.145, 0.148, 0.186, 2.5), (1.19, -0.13, 0.182, 0.17, 2.5), (1.13, -0.08, 0.222, 0.12, 2.4)],
    200: [(1.27, -0.15, 0.126, 0.19, 2.4), (1.2, -0.135, 0.174, 0.176, 2.5), (1.13, -0.08, 0.222, 0.12, 2.4)],
}


def r5(x):
    return round(x, 5)


def rotation():
    n = math.atan2(V[1], U[1])
    a = (0.0, 0.0, math.sin(n / 2), math.cos(n / 2))
    b = (0.0, -math.sqrt(0.5), 0.0, math.sqrt(0.5))
    q = (a[3] * b[0] + a[0] * b[3] + a[1] * b[2] - a[2] * b[1], a[3] * b[1] - a[0] * b[2] + a[1] * b[3] + a[2] * b[0], a[3] * b[2] + a[0] * b[1] - a[1] * b[0] + a[2] * b[3], a[3] * b[3] - a[0] * b[0] - a[1] * b[1] - a[2] * b[2])
    return [r5(c) for c in q]


def drape(scale):
    rot = rotation()
    sections = []
    for y, back, front, rw, e in SLICES[scale]:
        x = (front + back) / 2
        d = (x - O[0], y - O[1])
        sections.append({"at": [r5(d[0] * U[0] + d[1] * U[1]), r5(d[0] * V[0] + d[1] * V[1]), 0], "rx": r5(rw), "ry": r5((front - back) / 2), "exponent": e, "rotation": rot})
    sections.reverse()
    return {"id": "belly-drape", "kind": "loft", "bone": "torso", "material": "knit-wool-sweater", "role": "body", "tone": None, "sections": sections, "capStart": True, "capEnd": True}


doc = json.load(open(SOURCE, encoding="utf-8"))
for scale in SLICES:
    vol = doc["volumes"][str(scale)]
    cover = next(s for s in vol["body"] if s["id"] == "belly-cover")
    solid = drape(scale)
    solid["tone"] = cover["tone"]
    solid["material"] = cover["material"]
    vol["body"] = [s for s in vol["body"] if s["id"] != "belly-drape"]
    vol["body"].insert(vol["body"].index(cover) + 1, solid)
    ref = next(s for s in vol["thermal"] if s.get("of") == "sweater")
    vol["thermal"] = [s for s in vol["thermal"] if s.get("id") != "th-belly-drape"]
    vol["thermal"].insert(vol["thermal"].index(ref) + 1, {"id": "th-belly-drape", "of": "belly-drape", "region": ref["region"], **({"layer": ref["layer"]} if "layer" in ref else {})})
open(SOURCE, "w", encoding="utf-8").write(json.dumps(doc) + "\n")
print("belly-drape at", ", ".join("1:%d" % s for s in SLICES))
