"""🔔 Hand edit of `objects/street-lamp` (second round, pass 1): the flared base shroud belongs to the column.

Usage: python3 rework-street-lamp-shroud.py [--write]
The shroud is welded to the shaft and only stands on the base plate, so it sways with the `pole` bone (in `lit-in-wind` the shaft broke through a shroud left on the foundation). Its solids and 2D shapes move to the pole frame (u = height above the plate) and its skirt reaches 5 mm into the plate so the 1.8 degree sway (rim travel 3.8 mm) opens no joint. Without `--write` only the candidate `🗑️generated/rework-street-lamp/cand/🔣️.json` is written.
"""
import difflib
import json
import os
import subprocess
import sys
import types

T = os.path.dirname(os.path.abspath(__file__))
SOURCE = os.path.join(T, "volumes/objects/street-lamp.json")
CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🪑️objects/street-lamp/🔣️.json"
PLATE_TOP = 0.12
SKIRT = 0.005
layout = open(os.path.join(T, "volumes-insert.py")).read()
insert = types.SimpleNamespace()
exec("import json\n" + layout[layout.index("def scalar"):layout.index("def member_end")], insert.__dict__)
earlier = open(os.path.join(T, "rework-street-lamp-edit.py")).read()
roots = types.SimpleNamespace()
exec("import json\n" + earlier[earlier.index("def layout_root"):earlier.index("def find")], roots.__dict__)


def find(rows, identity):
    return next((row for row in rows if row.get("id") == identity), None)


def along(profile):
    return [[round(-SKIRT if abs(y - PLATE_TOP) < 1e-9 else y - PLATE_TOP, 6), r] for y, r in profile]


source = json.load(open(SOURCE))
for scale in ("50", "100"):
    model = source["volumes"][scale]
    for rows, identity in ((model["body"], "base-cover"), (model["thermal"], "th-base-cover")):
        solid = find(rows, identity)
        if solid and solid.get("bone") == "foundation":
            solid.update(bone="pole", axis=[1, 0, 0], origin=[0, 0, 0], profile=along(solid["profile"]))
    for decision in model["decisions"]:
        if "shroud" in decision["en"] and "sways with the shaft" not in decision["en"] and decision["en"].startswith("Body"):
            decision["en"] += " The shroud is welded to the shaft, sways with it and reaches 5 mm into the plate."
            decision["de"] += " Die Manschette ist mit dem Schaft verschweißt, schwingt mit ihm und greift 5 mm in die Platte."

text = open(CATALOG).read()
document = json.loads(text)
asset = document["asset"]
for scale in ("50", "100"):
    geometry = asset["representations"][scale]["geometry"]
    for view in ("elevation", "front", "plan"):
        shape = find(geometry[view], "base-cover")
        if shape["bone"] == "foundation":
            shape["bone"] = "pole"
            if view == "elevation":
                half = [[round(y - PLATE_TOP, 6), -abs(x)] for x, y in shape["points"] if x < 0]
                half[0][0] = -SKIRT
                shape["points"] = half + [[u, -v] for u, v in reversed(half)]
            elif view == "front":
                shape["points"] = [[x, round(PLATE_TOP - SKIRT, 6) if abs(y - PLATE_TOP) < 1e-9 else y] for x, y in shape["points"]]
candidate = json.loads(json.dumps(document))
candidate["asset"]["volumes"] = source["volumes"]
folder = os.path.join(T, "🗑️generated", "rework-street-lamp", "cand")
os.makedirs(folder, exist_ok=True)
json.dump(candidate, open(os.path.join(folder, "🔣️.json"), "w"), ensure_ascii=False)
if "--write" in sys.argv:
    assert open(CATALOG).read() == text, "catalog changed while editing, run again"
    open(SOURCE, "w").write(insert.dump(source, 0) + "\n")
    open(CATALOG, "w").write(roots.layout_root(document, 0) + "\n")
    subprocess.run([sys.executable, os.path.join(T, "volumes-insert.py"), SOURCE, CATALOG], check=True)
    fresh = open(CATALOG).read().splitlines()
    print("written,", sum(1 for line in difflib.unified_diff(text.splitlines(), fresh, lineterm="", n=0) if line[:1] in "+-" and line[:3] not in ("+++", "---")), "changed lines")
else:
    print(os.path.join(folder, "🔣️.json"))
