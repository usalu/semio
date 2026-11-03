import json,glob
B="🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog"
rows=[]
for f in glob.glob(f"{B}/*/*/🔣️.json"):
    d=json.load(open(f,encoding="utf-8"))["asset"]
    R=d["representations"]["500"]
    if R["status"]!="authored": continue
    n=len(R["geometry"]["elevation"]); rows.append((n,d["category"],d["id"]))
rows.sort(reverse=True)
for r in rows: print(r)
