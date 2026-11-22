"""🔎 Compares the birch source with the catalog and prints the parts per scale and list."""
import json, collections, sys
CAT = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🌳️vegetation/multi-stem-birch/🔣️.json"
src = json.load(open("volumes/vegetation/multi-stem-birch.json"))["volumes"]
cat = json.load(open(CAT))["asset"]
for s in ("50", "100", "200", "500"):
    a, b = src[s], cat["volumes"][s]
    print(s, "same" if a == b else "DIFF", {k: len(v) if isinstance(v, list) else type(v).__name__ for k, v in a.items()})
    for lst in ("body", "thermal", "structure"):
        if a.get(lst) != b.get(lst):
            ia = {x.get("id"): x for x in a[lst]}; ib = {x.get("id"): x for x in b[lst]}
            print("  ", lst, "only src", len(set(ia) - set(ib)), "only cat", len(set(ib) - set(ia)), "changed", sum(1 for k in ia if k in ib and ia[k] != ib[k]))
        kinds = collections.Counter((x.get("kind"), x.get("bone")) for x in a[lst])
        pre = collections.Counter((x.get("id", "?").split("-")[0], x.get("kind")) for x in a[lst])
        print("  ", lst, dict(pre))
print("rig", json.dumps(cat["rig"])[:3000])
print("poses", [p["id"] for p in cat.get("poses", [])], "clips", [c["id"] for c in cat.get("clips", [])], "default", cat["presentation"].get("defaultPose"))
print("keys", list(cat.keys()))
print("dims", cat.get("dimensions"))
