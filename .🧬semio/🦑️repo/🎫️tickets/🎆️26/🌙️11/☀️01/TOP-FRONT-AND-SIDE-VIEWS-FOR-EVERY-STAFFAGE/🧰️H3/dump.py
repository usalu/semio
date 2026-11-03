import json,sys,os
ROOT=r"C:\git\semio\🎓️teaching\🏛️architecture\🧍️staffage\🗂️catalog\👤️humans"
OUT=r"C:\git\semio\.🧬semio\🦑️repo\🎫️tickets\🎆️26\🌙️11\☀️01\TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE\🗑️generated\H3"
def short(s):
    return json.dumps(s,ensure_ascii=False,separators=(",",":"))
for a in sys.argv[1:]:
    d=json.load(open(f"{ROOT}\\{a}\\🔣️.json",encoding="utf8"))["asset"]
    L=[]
    L.append("DIMS "+short(d["dimensions"]))
    L.append("RIG "+short(d["rig"]))
    L.append("POSES "+short(d["poses"]))
    L.append("CLIPS "+short(d["clips"]))
    L.append("MATERIALS "+short([ (m.get("id"),m.get("name")) if isinstance(m,dict) else m for m in d["materials"]]))
    reps=d["representations"]
    for sc,r in (reps.items() if isinstance(reps,dict) else enumerate(reps)):
        L.append(f"=== scale {sc} {r.get('status')} keys={list(r.keys())}")
        g=r.get("geometry",{})
        for k,v in g.items():
            if isinstance(v,list):
                L.append(f"--- {k}")
                for s in v: L.append(short(s))
            else: L.append(f"--- {k} {short(v)}")
        for k in r:
            if k not in ("geometry",): L.append(f"[{k}] "+short(r[k])[:1500])
    open(f"{OUT}\\dump-{a}.txt","w",encoding="utf8").write("\n".join(L))
    print(a,"ok",len("\n".join(L)))
