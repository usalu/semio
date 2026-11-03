import json,sys
B="🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/"
a=sys.argv[1]; cat=sys.argv[2]
d=json.load(open(B+cat+"/"+a+"/🔣️.json",encoding="utf-8"))["asset"]
print(json.dumps({"dim":d["dimensions"],"pres":d["presentation"],"bones":d["rig"],"anchors":d["anchors"]},ensure_ascii=False)[:6000])
for k in ["50","100"]:
    R=d["representations"][k]
    print(k,R["validRange"],R["pens"])
    for v,ss in R["geometry"].items():
        for s in ss: print("  ",v,json.dumps(s,ensure_ascii=False))
print("regions",[(r["id"],r.get("areaM2"),r["model"]) for r in d["thermal"]["regions"]])
print("poses",[(p["id"]) for p in d["poses"]])
for c in d["clips"]: print("clip",json.dumps(c,ensure_ascii=False)[:1500])
