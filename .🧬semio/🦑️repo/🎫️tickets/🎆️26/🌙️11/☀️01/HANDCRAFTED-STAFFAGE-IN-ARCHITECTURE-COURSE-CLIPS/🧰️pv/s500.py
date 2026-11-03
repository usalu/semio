import json,sys,glob
B="🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog"
for a in sys.argv[1:]:
    f=glob.glob(f"{B}/*/{a}/🔣️.json")[0]
    d=json.load(open(f,encoding="utf-8"))["asset"]
    R=d["representations"]["500"]
    print("==",a,d["dimensions"],R["pens"],R["validRange"],{v:len(s) for v,s in R["geometry"].items()})
    for v,ss in R["geometry"].items():
        for s in ss:
            g={k:x for k,x in s.items() if k not in("bone","tone","material","filled")}
            print("  ",v,json.dumps(g,ensure_ascii=False)[:420])
    print("  regions:",[r["id"] for r in d["thermal"]["regions"]])
    print("  decisions:",json.dumps(R["decisions"],ensure_ascii=False)[:1800])
