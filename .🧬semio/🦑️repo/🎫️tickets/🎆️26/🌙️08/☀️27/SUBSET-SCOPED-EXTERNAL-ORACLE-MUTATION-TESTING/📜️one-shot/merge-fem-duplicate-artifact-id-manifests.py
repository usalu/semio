import json,io,os,sys
APPLY = "--apply" in sys.argv
PAIRS=[("◻2d","s.fem.fem2d","s.fem.2d"),("🧊️3d","s.fem.fem3d","s.fem.3d")]
for art,mine,canon in PAIRS:
    C=f"✏️s/🔌️plugins/🏗️fem/🗿️artifacts/{art}/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️.json"
    d=json.load(open(C,encoding="utf8")); ms=d["mutationManifests"]
    a=next(m for m in ms if m["artifact"]==mine); b=next(m for m in ms if m["artifact"]==canon)
    B={(m.get("id") or m.get("mutationId")):m for m in b["mutations"]}
    merged=[]
    for m in a["mutations"]:
        k=m.get("id") or m.get("mutationId"); other=B.get(k,{})
        reqs=list(m.get("oracleRequirements") or [])
        for orq in (other.get("oracleRequirements") or []):
            if not any(r.get("capability")==orq.get("capability") for r in reqs): reqs.append(orq)
        m=dict(m); m["oracleRequirements"]=reqs
        outs=list(dict.fromkeys(list(m.get("outcomes") or [])+list(other.get("outcomes") or [])))
        if outs: m["outcomes"]=outs
        merged.append(m)
    a=dict(a); a["artifact"]=canon; a["mutations"]=merged
    d["mutationManifests"]=[a]
    caps=sorted({r.get("capability") for m in merged for r in m["oracleRequirements"]})
    print(f"{art}: merged -> {canon}, {len(merged)} mutations, capabilities={caps}")
    if APPLY: io.open(C,"w",encoding="utf8").write(json.dumps(d,ensure_ascii=False,indent=2)+"\n")
