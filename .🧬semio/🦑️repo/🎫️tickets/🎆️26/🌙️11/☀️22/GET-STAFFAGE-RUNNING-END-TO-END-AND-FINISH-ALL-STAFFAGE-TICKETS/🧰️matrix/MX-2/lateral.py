"""🥚️ Trial copies of the catalog documents that author `lateralScale` on a revolved drawing solid, with that member removed (what the engine drew before D54): `python3 lateral.py <out folder>` writes `<id>.json` per asset and prints the ids."""
import glob, json, os, sys
OUT = sys.argv[1]
os.makedirs(OUT, exist_ok=True)
ids = []
for path in sorted(glob.glob("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*/*/🔣️.json")):
    text = open(path, encoding="utf8").read()
    if '"lateralScale"' not in text:
        continue
    doc = json.loads(text)
    found = 0
    for representation in doc["asset"]["representations"].values():
        for shapes in (representation.get("geometry") or {}).values():
            for shape in shapes:
                solid = shape.get("solid") or {}
                if solid.get("kind") == "revolved" and solid.get("lateralScale", 1) != 1:
                    del solid["lateralScale"]
                    found += 1
    if found:
        json.dump(doc, open(f"{OUT}/{doc['asset']['id']}.json", "w", encoding="utf8"), ensure_ascii=False)
        ids.append(doc["asset"]["id"])
print(" ".join(ids))
