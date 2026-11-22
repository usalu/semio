"""🔎 Tram author ruler: compares the volumes source with the catalog and prints rig, poses, clips and dimensions."""
import json, sys
CAT = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🚜️vehicles/tram/🔣️.json"
cat = json.load(open(CAT, encoding="utf-8"))["asset"]
src = json.load(open("volumes/vehicles/tram.json", encoding="utf-8"))
print(list(cat.keys()))
print("same volumes:", cat.get("volumes") == src["volumes"])
for sc in "50", "100", "200", "500":
    a = cat["volumes"][sc]; b = src["volumes"][sc]
    for l in ("body", "thermal", "structure"):
        ia = {s["id"]: s for s in a.get(l, [])}; ib = {s["id"]: s for s in b.get(l, [])}
        diff = [i for i in ib if ia.get(i) != ib[i]] + [i for i in ia if i not in ib]
        kinds = {}
        for s in ib.values(): kinds[s.get("kind", "of")] = kinds.get(s.get("kind", "of"), 0) + 1
        print(sc, l, len(ia), len(ib), kinds, "diff", diff[:10])
    print(" keys", list(b.keys()), "dec same", a.get("decisions") == b.get("decisions"))
if "full" in sys.argv:
    for k in ("rig", "dimensions", "poses", "clips", "presentation", "anchors", "thermal", "structure"):
        print("==", k, json.dumps(cat.get(k), ensure_ascii=False)[:int(sys.argv[2])])
