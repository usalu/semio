import json, sys, os, glob
root = r"C:\git\semio\🎓️teaching\🏛️architecture\🧍️staffage"
asset, scale, views = sys.argv[1], sys.argv[2], sys.argv[3].split(",")
p = glob.glob(os.path.join(root, "🗂️catalog", "*", asset, "🔣️.json"))[0]
d = json.load(open(p, encoding="utf-8"))["asset"]
r = d["representations"][scale]
if "--meta" in sys.argv:
    print(json.dumps({k: v for k, v in r.items() if k != "geometry"}, ensure_ascii=False, indent=1))
    print(json.dumps(d["rig"], ensure_ascii=False)[:6000])
    print("regions/materials", json.dumps(d.get("materials"), ensure_ascii=False)[:1500])
for v in views:
    print("##", v)
    for s in r["geometry"].get(v, []):
        print(json.dumps(s, ensure_ascii=False, separators=(",", ":")))
