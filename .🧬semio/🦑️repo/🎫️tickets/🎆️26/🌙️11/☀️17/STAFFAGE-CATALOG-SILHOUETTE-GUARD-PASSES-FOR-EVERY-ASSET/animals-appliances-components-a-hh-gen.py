import json,sys
ops=json.load(open("🗑️generated/aac-a-hh2.json"))
k=float(sys.argv[1]); views=sys.argv[2].split(",")
a=json.load(open("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🐕️animals/hedgehog/🔣️.json"))["asset"]
g=a["representations"]["50"]["geometry"]
for v in views:
    for s in g[v]:
        if s["id"].startswith("skirt-") and s["kind"]=="poly":
            pts=[[round(-0.0106+(x+0.015)*k,5),y] for x,y in s["points"]]
            ops.append({"kind":"shape","scale":"50","view":v,"id":s["id"],"set":{"points":pts}})
json.dump(ops,open("🗑️generated/aac-a-hh4.json","w"),ensure_ascii=False)
