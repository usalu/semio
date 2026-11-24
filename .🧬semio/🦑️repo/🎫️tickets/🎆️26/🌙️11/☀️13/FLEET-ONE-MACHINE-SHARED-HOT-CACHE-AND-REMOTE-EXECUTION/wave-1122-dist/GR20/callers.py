import json,sys,collections
p=json.load(open(sys.argv[1])); nodes={n['id']:n for n in p['nodes']}
parent={}
for n in p['nodes']:
    for c in n.get('children',[]): parent[c]=n['id']
self_t=collections.Counter()
for sid,d in zip(p['samples'],p['timeDeltas']): self_t[sid]+=d
def nm(n):
    cf=n['callFrame']; u=cf['url'].split('/'); return (cf['functionName'] or '(anon)')+' '+(u[-1][-16:] if u else '')+':'+str(cf['lineNumber']+1)
for target in sys.argv[2:]:
    agg=collections.Counter()
    for i,t in self_t.items():
        if nodes[i]['callFrame']['functionName']==target:
            chain=[];x=i
            for _ in range(7):
                x=parent.get(x)
                if x is None:break
                chain.append(nm(nodes[x]))
            agg[' < '.join(chain[:6])]+=t
    print(target)
    for k,v in agg.most_common(5): print(f"  {v/1000:.0f}ms {k[:500]}")
