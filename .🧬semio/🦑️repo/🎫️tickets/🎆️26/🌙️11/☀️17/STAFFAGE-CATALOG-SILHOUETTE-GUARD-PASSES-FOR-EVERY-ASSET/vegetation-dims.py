"""Intersection of the per-scale admissible declared dimensions: python3 vegetation-dims.py <bbox.jsonl>"""
import json,sys,collections
rows=[json.loads(l) for l in open(sys.argv[1]) if l.startswith("{")]
by=collections.defaultdict(list)
for r in rows: by[r["asset"]].append(r)
for a,rs in by.items():
    dims=rs[0]["dims"]; out=[]
    for k,n in enumerate(("width","height","depth")):
        lo=max(r["ext"][k]-r["f"] for r in rs); hi=min(r["ext"][k]+r["f"] for r in rs)
        ok=lo<=dims[k]<=hi
        out.append(f"{n} {dims[k]} {'ok' if ok else 'FAIL'} [{lo:.4f},{hi:.4f}]{'' if lo<=hi else ' EMPTY'}")
    print(a.ljust(24)," | ".join(out))
