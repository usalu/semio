import json,sys,collections
CAT="/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🌳️vegetation/"
for slug in sys.argv[1:]:
    a=json.load(open(CAT+slug+"/🔣️.json"))["asset"]
    print("==",slug,a["dimensions"],"bones",len(a["rig"]["bones"]),"pose",a["presentation"]["defaultPose"])
    for sc in ("50","100","200","500"):
        v=a["volumes"][sc]; r=a["representations"][sc]
        if "body" not in v: print(" ",sc,v.get("status")); continue
        kinds=collections.Counter(s["kind"] for s in v["body"])
        bones=len({s["bone"] for s in v["body"]})
        print(f" {sc}: solids {len(v['body'])} bones {bones} {dict(kinds)} pens {r.get('pens')}")
        for view,l in r["geometry"].items():
            c=collections.Counter((s["kind"],s["role"],bool(s.get("filled"))) for s in l)
            print("    ",view,len(l),dict(c))
