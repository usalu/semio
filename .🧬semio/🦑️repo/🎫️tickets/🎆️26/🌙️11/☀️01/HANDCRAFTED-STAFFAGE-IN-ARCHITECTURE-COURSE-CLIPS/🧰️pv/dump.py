import json,sys,glob
B="🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog"
def find(a):
    return glob.glob(f"{B}/*/{a}/🔣️.json")[0]
for a in sys.argv[1:]:
    d=json.load(open(find(a),encoding="utf-8"))["asset"]
    print("==",a,d["dimensions"],[ (v) for v in d["presentation"]["viewpoints"]])
    for k,R in d["representations"].items():
        if R["status"]!="authored": print(" ",k,"unavailable");continue
        g=R["geometry"]
        print(" ",k,"pens",R["pens"],"views",{v:len(s) for v,s in g.items()},"hatch",[(h["id"],h["spacingMm"],h["penMm"]) for h in R["hatches"]])
        pm=[(v,s["id"],s["penMm"]) for v,ss in g.items() for s in ss if "penMm" in s]
        if pm: print("    penMm",pm)
