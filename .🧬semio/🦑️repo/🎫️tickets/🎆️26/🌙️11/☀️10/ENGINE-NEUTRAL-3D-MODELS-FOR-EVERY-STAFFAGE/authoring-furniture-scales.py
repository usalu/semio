import json,sys
slug=sys.argv[1]
a=json.load(open(f"/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🛋️furniture/{slug}/🔣️.json"))["asset"]
for k in ("500","200","100","50"):
    r=a["representations"][k]
    g=r.get("geometry")
    if not g: print(k,"no geometry",list(r.keys())); continue
    print(k,{v:[s["id"] for s in g[v]] for v in ("elevation","front","plan") if v in g})
