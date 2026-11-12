"""Writes a scratch copy of a furniture asset whose extruded hints lack `sweep` get sweep "w" (the migration D47 has not landed in the catalog yet). Usage: authoring-furniture-patch.py <slug> -> prints the path."""
import glob, json, os, sys
T = os.path.dirname(os.path.abspath(__file__))
slug = sys.argv[1]
src = glob.glob(f"/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*furniture/{slug}/🔣️.json")[0]
doc = json.load(open(src, encoding="utf-8"))
n = 0
def walk(x):
    global n
    if isinstance(x, dict):
        if x.get("kind") == "extruded" and "z0" in x and "sweep" not in x:
            x["sweep"] = "w"; n += 1
        for v in x.values(): walk(v)
    elif isinstance(x, list):
        for v in x: walk(v)
walk(doc["asset"]["representations"])
out = os.path.join(T, "🗑️generated", "authoring-furniture", "patched", slug + ".json")
os.makedirs(os.path.dirname(out), exist_ok=True)
json.dump(doc, open(out, "w", encoding="utf-8"), ensure_ascii=False)
print(out)
