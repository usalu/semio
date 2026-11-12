import json,sys
slug=sys.argv[1]; ids=sys.argv[2].split(","); scale=sys.argv[3] if len(sys.argv)>3 else "50"
a=json.load(open(f"/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🛋️furniture/{slug}/🔣️.json"))["asset"]
g=a["representations"][scale]["geometry"]
for v in ("elevation","front","plan"):
    for s in g[v]:
        if s["id"] in ids or "*" in ids:
            d={k:x for k,x in s.items() if k not in("filled","penMm","tone","role","region","z")}
            print(v,json.dumps(d,separators=(",",":"))[:500])
