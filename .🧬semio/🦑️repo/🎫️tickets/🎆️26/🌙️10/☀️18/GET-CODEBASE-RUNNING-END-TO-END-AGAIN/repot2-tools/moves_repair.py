#!/usr/bin/env python3
# usage: moves_repair.py <fixture.json> [--apply]   rows: {source,owner,kind,anchor,consumers[]}; proposes new owner by unique anchor match among files with the owner's basename
import json,os,subprocess,sys
f=sys.argv[1]; apply="--apply" in sys.argv
raw=open(f,encoding="utf-8").read(); d=json.loads(raw)
changes=[]
for i,r in enumerate(d["moves"]):
    o=r["owner"]; ok=os.path.isfile(o) and r["anchor"] in open(o,encoding="utf-8",errors="replace").read()
    miss=[c for c in r.get("consumers",[]) if not os.path.exists(c)]
    if ok and not miss: continue
    base=os.path.basename(o)
    if not ok:
        scope=os.path.dirname(o).split("/")[0:2]
        res=subprocess.run(["git","grep","-l","-F","--",r["anchor"],"--","*"+base],capture_output=True,text=True).stdout.split("\n")
        res=[x for x in res if x and os.path.basename(x)==base and not x.startswith(".🧬semio/")]
        print(("FIX  " if len(res)==1 else "AMBIG" if res else "NONE ")+"[%d] owner=%s -> %s"%(i,o[-70:],[x[-70:] for x in res[:3]]))
        if len(res)==1: changes.append((o,res[0]))
    for c in miss: print("CONS-MISSING [%d] %s"%(i,c[-100:]))
if apply:
    for old,new in changes:
        a=json.dumps(old,ensure_ascii=False)
        assert raw.count(a)>=1
        raw=raw.replace(a,json.dumps(new,ensure_ascii=False))
    json.loads(raw); open(f,"w",encoding="utf-8").write(raw); print("applied",len(changes))
