import json,sys
a=json.load(open("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🐕️animals/"+sys.argv[1]+"/🔣️.json"))["asset"]
for sc in sys.argv[2:]:
    print("== scale",sc)
    print(" VOL", [(s["id"],s["kind"][0],s.get("bone")) for s in a["volumes"][sc]["body"]])
    for v,l in a["representations"][sc]["geometry"].items():
        print(" ",v,[(s["id"],s["kind"][:3],s.get("bone"),s.get("role","")[:1]) for s in l])
