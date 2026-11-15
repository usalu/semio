import json,sys
slug,scales=sys.argv[1],sys.argv[2].split(",")
views=sys.argv[3].split(",") if len(sys.argv)>3 else ["elevation","front","plan"]
cat={"patio-heater":"objects","playground-swing":"objects"}
p="/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🪑️objects/%s/🔣️.json"%slug
a=json.load(open(p))["asset"]
for sc in scales:
    r=a["representations"][sc]
    print("=====",sc,r["status"])
    for v in views:
        print("--",v)
        for s in r["geometry"][v]:
            d={k:x for k,x in s.items() if k not in("z","tone","material","hatch","region","filled") }
            if "filled" in s and not s["filled"]: d["F"]=0
            if "d" in d: d["d"]=d["d"][:260]
            if "points" in d: d["points"]=[[round(c,3) for c in q] for q in d["points"]][:12]
            print("  ",json.dumps(d,ensure_ascii=False))
