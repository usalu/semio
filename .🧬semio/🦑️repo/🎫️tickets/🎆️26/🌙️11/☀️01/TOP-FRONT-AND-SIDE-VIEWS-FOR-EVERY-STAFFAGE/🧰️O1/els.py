import json,sys
base=r"C:\git\semio\🎓️teaching\🏛️architecture\🧍️staffage\🗂️catalog\🪑️objects"
a=sys.argv[1]
d=json.load(open(base+"\\"+a+"\🔣️.json",encoding="utf-8"))["asset"]
for sc in sys.argv[2:]:
    r=d["representations"][sc]
    print("#### scale",sc,r["status"],"pens",r.get("pens"),"hatches",[(h["id"],h["spacingMm"]) for h in r.get("hatches",[])],"range",r.get("validRange"))
    for s in r["geometry"]["elevation"]:
        k={x:v for x,v in s.items() if x not in("filled","role","bone") or x=="bone"}
        print(json.dumps(k,ensure_ascii=False,separators=(",",":")))
