"""Chooses the declared dimensions and the volume extent shifts: python3 vegetation-bbox-plan.py <bbox.jsonl> [slug...]  -> prints dims per axis and lines `SHIFT <slug> <scale> <axis> <min|max> <shift>`"""
import json,sys,collections,os
rows=[json.loads(l) for l in open(sys.argv[1]) if l.startswith("{")]
only=set(sys.argv[2:])
by=collections.defaultdict(list)
for r in rows: by[r["asset"]].append(r)
MARGIN=0.8; TARGET=0.25
for a,rs in by.items():
    if only and a not in only: continue
    dims=rs[0]["dims"]; new=[]
    for k,n in enumerate(("width","height","depth")):
        lo=min(r["ext"][k] for r in rs)-0.06; hi=max(r["ext"][k] for r in rs)+0.06
        cands=[round(lo+i*0.005,3) for i in range(int((hi-lo)/0.005)+1)]+[dims[k]]
        if a in os.environ.get("KEEPDIMS","").split(","): cands=[dims[k]]
        def cost(d): return sum((abs(r["ext"][k]-d)-TARGET*r["f"]) for r in rs if abs(r["ext"][k]-d)>MARGIN*r["f"])
        best=min(cands,key=lambda d:(round(cost(d),6),abs(d-dims[k])))
        if cost(dims[k])==0 or a in os.environ.get("KEEPDIMS","").split(","): best=dims[k]
        new.append(best)
        print(f"DIMS {a} {n} {dims[k]} -> {best}")
        for r in rs:
            e=r["ext"][k]; diff=e-best
            if abs(diff)>MARGIN*r["f"]:
                dext=-(abs(diff)-TARGET*r["f"])*(1 if diff>0 else -1)
                side="max" if k==1 or abs(r["max"][k])>=abs(r["min"][k]) else "min"
                shift=dext if side=="max" else -dext
                print(f"SHIFT {a} {r['scale']} {k} {side} {shift:+.4f} (ext {e:.4f} vs {best}, f {r['f']})")
