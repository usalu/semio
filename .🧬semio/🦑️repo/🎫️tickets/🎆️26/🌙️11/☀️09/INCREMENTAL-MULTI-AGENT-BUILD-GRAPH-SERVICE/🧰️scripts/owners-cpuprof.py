"""Top functions of a .cpuprofile by self time and by inclusive time (build code only for inclusive); optional window start/end in seconds from the profile start."""
import json,sys,collections
p=json.load(open(sys.argv[1]));nodes={n["id"]:n for n in p["nodes"]};parent={}
for n in p["nodes"]:
    for c in n.get("children",[]):parent[c]=n["id"]
t0=p["startTime"];lo=float(sys.argv[2])*1e6 if len(sys.argv)>2 else 0;hi=float(sys.argv[3])*1e6 if len(sys.argv)>3 else 1e18
selfs=collections.Counter();incl=collections.Counter()
ts=p["timeDeltas"];t=0
for s,d in zip(p["samples"],ts):
    t+=d
    if not(lo<=t<=hi):continue
    selfs[s]+=d;seen=set();x=s
    while x in nodes:
        cf=nodes[x]["callFrame"];k=(cf["functionName"],cf["url"].split("🏗️build/")[-1][-60:],cf["lineNumber"])
        if k not in seen:incl[k]+=d;seen.add(k)
        x=parent.get(x)
        if x is None:break
def key(i):cf=nodes[i]["callFrame"];return (cf["functionName"],cf["url"].split("🏗️build/")[-1][-60:],cf["lineNumber"])
agg=collections.Counter()
for i,v in selfs.items():agg[key(i)]+=v
print("SELF");[print(round(v/1e6,2),k) for k,v in agg.most_common(15)]
print("INCLUSIVE");[print(round(v/1e6,2),k) for k,v in incl.most_common(45) if k[1]]
