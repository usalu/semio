"""🌾 Writes the shape ops of `ops/miscanthus-grass.json` (anchor ops in the file stay): the culms are open strokes at every scale; in the plan each shared its id with its elevation twin, so the engine carried the whole stroke at one depth and the plan culms stayed 49 to 71 mm behind the side elevation in the gale (tolerance 43 mm). Each such plan stroke takes the suffix `-dp`: its points are then carried at the depth of the elevation silhouette of its bone, and the plan follows the swing. (The same strokes as `extruded` solids were tried and rejected: the silhouette guard then fails the plan at 1:50, 1:100 and 1:200, 45 to 113 mm. The same suffix on the racemes of the plumes at 1:50 clears the last finding there, plan 48 mm short in the gale, but the silhouette guard then measures the plan at 57 mm instead of 13 mm, so the racemes stay as they are.) Run: python3 ops-miscanthus-grass.py"""
import json, os, re

S = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage"
HERE = os.path.dirname(os.path.abspath(__file__))
asset = json.load(open(f"{S}/🗂️catalog/🌳️vegetation/miscanthus-grass/🔣️.json"))["asset"]
out = os.path.join(HERE, "ops", "miscanthus-grass.json")
ops = [op for op in (json.load(open(out)) if os.path.exists(out) else []) if op["op"] == "anchor"]
for scale, representation in asset["representations"].items():
    for shape in representation.get("geometry", {}).get("plan", []):
        base = re.sub(r"-(dp|mvs)$", "", shape["id"])
        wanted = shape["bone"].startswith("culm-") and base.startswith("culm-")
        if wanted and "solid" not in shape and shape.get("filled") is not True and not base.startswith("mvc-"):
            ops.append({"op": "rename", "scale": scale, "view": "plan", "id": base, "bone": shape["bone"], "to": base + "-dp"})
open(out, "w").write("[\n" + ",\n".join("  " + json.dumps(op, ensure_ascii=False) for op in ops) + "\n]\n")
print(len(ops), "ops")
