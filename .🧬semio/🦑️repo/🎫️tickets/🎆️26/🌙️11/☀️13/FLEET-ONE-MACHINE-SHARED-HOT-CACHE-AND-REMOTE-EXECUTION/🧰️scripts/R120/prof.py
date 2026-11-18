import json,sys,collections
p=json.load(open(sys.argv[1]));nodes={n['id']:n for n in p['nodes']};par={}
for n in p['nodes']:
    for c in n.get('children',[]):par[c]=n['id']
self=collections.Counter();incl=collections.Counter();dt=p['timeDeltas'];tot=0
for i,s in enumerate(p['samples']):
    d=dt[i+1] if i+1<len(dt) else 0;tot+=d;self[s]+=d;seen=set();x=s
    while x is not None:
        f=nodes[x]['callFrame'];k=(f['functionName'] or '?')+'@'+'/'.join(f['url'].split('/')[-2:])+':'+str(f['lineNumber']+1)
        if k not in seen:incl[k]+=d;seen.add(k)
        x=par.get(x)
fs=collections.Counter()
for n,v in self.items():
    f=nodes[n]['callFrame'];fs[(f['functionName'] or '?')+'@'+'/'.join(f['url'].split('/')[-2:])+':'+str(f['lineNumber']+1)]+=v
print('total s',tot/1e6);N=int(sys.argv[2]) if len(sys.argv)>2 else 25
print('SELF');[print(round(v/1e6,2),k) for k,v in fs.most_common(N)]
print('INCL');[print(round(v/1e6,2),k) for k,v in incl.most_common(N+15)]
