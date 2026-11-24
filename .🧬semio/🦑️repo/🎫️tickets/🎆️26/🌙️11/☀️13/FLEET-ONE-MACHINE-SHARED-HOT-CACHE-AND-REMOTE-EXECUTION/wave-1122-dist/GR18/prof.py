import json,sys,collections,re
p=json.load(open(sys.argv[1])); nodes={n['id']:n for n in p['nodes']}
dt=p['timeDeltas']; self_t=collections.Counter()
for sid,d in zip(p['samples'],dt): self_t[sid]+=d
parent={}
for n in p['nodes']:
    for c in n.get('children',[]): parent[c]=n['id']
total=sum(self_t.values()); print('total ms',total/1000)
def name(n):
    cf=n['callFrame']; return f"{cf['functionName'] or '(anon)'} {cf['url'].split('/')[-3:][-3:] and '/'.join(cf['url'].split('/')[-2:])}:{cf['lineNumber']+1}"
# self time by function
byf=collections.Counter()
for i,t in self_t.items(): byf[name(nodes[i])]+=t
print('--- self'); 
for k,v in byf.most_common(25): print(f"{v/1000:8.0f}ms {100*v/total:5.1f}% {k}")
# inclusive
inc=collections.Counter()
for i,t in self_t.items():
    seen=set(); x=i
    while x is not None:
        k=name(nodes[x])
        if k not in seen: inc[k]+=t; seen.add(k)
        x=parent.get(x)
print('--- inclusive')
for k,v in inc.most_common(int(sys.argv[2]) if len(sys.argv)>2 else 45): print(f"{v/1000:8.0f}ms {100*v/total:5.1f}% {k}")
