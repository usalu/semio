import json,sys
B="🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/"
a=sys.argv[1]; cat=sys.argv[2]
d=json.load(open(B+cat+"/"+a+"/🔣️.json",encoding="utf-8"))["asset"]
print(d["dimensions"],d["presentation"]["viewpoints"])
for k in ["200","100","50"]:
    R=d["representations"][k]
    if "plan" in R["geometry"]:
        print(k, R["validRange"])
        for s in R["geometry"]["plan"]: print("  ",json.dumps(s,ensure_ascii=False))
        print({x:R["decisions"][x] for x in ("silhouette","overlays")})
