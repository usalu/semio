import json,sys,collections
p=json.load(open(sys.argv[1]))
nodes={n['id']:n for n in p['nodes']}
dt=p['timeDeltas'];samples=p['samples']
self_=collections.Counter()
for s,d in zip(samples,dt): self_[s]+=d
parent={}
for n in p['nodes']:
    for c in n.get('children',[]): parent[c]=n['id']
def name(n):
    cf=n['callFrame'];u=cf['url'].split('/')[-3:]
    return f"{cf['functionName'] or '(anon)'} {'/'.join(u)}:{cf['lineNumber']}"
tot=sum(self_.values())
byfn=collections.Counter();incl=collections.Counter()
for nid,t in self_.items():
    byfn[name(nodes[nid])]+=t
    seen=set();x=nid
    while x is not None:
        k=name(nodes[x])
        if k not in seen: incl[k]+=t;seen.add(k)
        x=parent.get(x)
print('total ms',tot/1000)
print('--self');
for k,v in byfn.most_common(25):print(round(v/1000),k[:160])
print('--inclusive')
for k,v in incl.most_common(45):print(round(v/1000),k[:160])
