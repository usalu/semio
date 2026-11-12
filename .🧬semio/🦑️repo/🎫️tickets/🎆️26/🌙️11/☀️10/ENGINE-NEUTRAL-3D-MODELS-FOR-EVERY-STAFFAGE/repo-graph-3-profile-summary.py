#!/usr/bin/env python3
"""Summarizes a V8/Bun .cpuprofile: top self-time functions and top inclusive (total) functions. usage: repo-graph-3-profile-summary.py FILE [N]"""
import json,sys,collections
p=json.load(open(sys.argv[1]));n=int(sys.argv[2]) if len(sys.argv)>2 else 25
nodes={x['id']:x for x in p['nodes']};dt=p['timeDeltas'];samples=p['samples']
self_t=collections.Counter();
for s,d in zip(samples,dt):self_t[s]+=d
parent={}
for x in p['nodes']:
    for c in x.get('children',[]):parent[c]=x['id']
def name(i):
    f=nodes[i]['callFrame'];return f"{f['functionName'] or '(anon)'} {f['url'].split('/')[-1] if f['url'] else ''}:{f['lineNumber']}"
fn_self=collections.Counter();fn_tot=collections.Counter()
for i,t in self_t.items():
    fn_self[name(i)]+=t
    seen=set();j=i
    while j is not None:
        k=name(j)
        if k not in seen:fn_tot[k]+=t;seen.add(k)
        j=parent.get(j)
total=sum(self_t.values())
print("total ms",total/1000)
print("--- self");[print(f"{v/1000:10.0f} ms {100*v/total:5.1f}% {k[:150]}") for k,v in fn_self.most_common(n)]
print("--- inclusive");[print(f"{v/1000:10.0f} ms {100*v/total:5.1f}% {k[:150]}") for k,v in fn_tot.most_common(n)]
