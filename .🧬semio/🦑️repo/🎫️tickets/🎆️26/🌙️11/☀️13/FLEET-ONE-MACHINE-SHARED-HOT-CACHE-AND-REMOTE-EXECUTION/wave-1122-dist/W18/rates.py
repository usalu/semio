"""📈 W18: per-machine growth of compiled units and skip reasons between two samples of W18/🗑️generated/samples.jsonl (default: first and last); args: [index_a index_b]."""
import json,sys,os,datetime
rows=[json.loads(l) for l in open(os.path.join(os.path.dirname(__file__),"🗑️generated","samples.jsonl"))]
a,b=(rows[int(sys.argv[1])],rows[int(sys.argv[2])]) if len(sys.argv)>2 else (rows[0],rows[-1])
minutes=(b["atMs"]-a["atMs"])/60000
f=lambda ms:datetime.datetime.fromtimestamp(ms/1000).strftime("%H:%M")
print(f"window {f(a['atMs'])}-{f(b['atMs'])} = {minutes:.1f} min")
old={m["origin"]:m for m in a["machines"]}
tot={}
for m in b["machines"]:
    o=old.get(m["origin"]);
    if not o:continue
    miss=m["miss"]-o["miss"];hit=m["hit"]-o["hit"]
    d={k:v-o["reasons"].get(k,0) for k,v in m["reasons"].items() if v-o["reasons"].get(k,0)}
    for k,v in d.items():tot[k]=tot.get(k,0)+v
    tot["_compiled"]=tot.get("_compiled",0)+miss
    print(f'{m["origin"]:10} compiled {miss:6} hits {hit:6} skipped {m["skipped"]-o["skipped"]:5} {d}')
print("total",dict(sorted(tot.items(),key=lambda kv:-kv[1])))
