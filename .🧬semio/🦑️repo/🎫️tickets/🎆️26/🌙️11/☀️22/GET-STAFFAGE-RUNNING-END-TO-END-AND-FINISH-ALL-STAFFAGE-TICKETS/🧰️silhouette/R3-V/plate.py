"""🦋️ Turns a shape that outline.ts traced in the paper coordinates of the front view (`world=1`) into a thin plate on its bone: the outline in the plane of the bone axis u and the lateral axis w, swept through `z0`..`z1` along the bone normal v, so the part turns with its bone in every view. The bone lies in the sagittal plane at rest at `angle` degrees with its origin at height `y0`; paper x of the front view is w, paper y the height: plate.py <id> <scale> <shape id> <y0> <angle deg> <z0> <z1>"""
import json, math, os, sys
HERE = os.path.dirname(os.path.abspath(__file__))
slug, scale, sid = sys.argv[1:4]
y0, angle, z0, z1 = (float(x) for x in sys.argv[4:8])
path = f"{HERE}/ops/{slug}.json"
ops = json.load(open(path))
o = next(o for o in ops if o["op"] == "shape-add" and o["scale"] == scale and o["view"] == "front" and o["shape"]["id"] == sid)
s, c = math.sin(math.radians(angle)), math.cos(math.radians(angle))
v = (z0 + z1) / 2
o["shape"]["points"] = [[round((y - y0 - v * c) / s, 5), x] for x, y in o["shape"]["points"]]
o["shape"]["solid"] = {"kind": "extruded", "sweep": "v", "z0": z0, "z1": z1}
open(path, "w").write("[\n" + ",\n".join(json.dumps(x, ensure_ascii=False) for x in ops) + "\n]\n")
print(f"{slug} {scale} front {sid}: plate of {len(o['shape']['points'])} points")
