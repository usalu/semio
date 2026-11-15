#!/usr/bin/env python3
"""R5 read-only measurement of the generation store: per wave since the last implementation change, how many re-inferred projects changed their fragment and where. usage: measure.py [first-generation]"""
import json,glob,subprocess,collections,re,random,sys
G='/Users/Shared/semio/.cache/b/g/m/generations/'
random.seed(2)
d={}
for f in glob.glob(G+'m/*.json'):
    x=json.load(open(f))
    d.setdefault(x['implementation'],{})[x['generation']]=x
impl=max(d,key=lambda k:max(d[k]));gens=d[impl]
first=int(sys.argv[1]) if len(sys.argv)>1 else min(gens)
load=lambda h:json.loads(subprocess.run(['brotli','-dc',G+f'o/{h}.json.br'],capture_output=True).stdout)
def walk(x,y,p,out):
    if type(x)!=type(y):out.add(p);return
    if isinstance(x,dict):
        for k in set(x)|set(y):
            if k not in x or k not in y:out.add(p+'/'+k);continue
            walk(x[k],y[k],p+'/'+k,out)
    elif isinstance(x,list):
        if len(x)!=len(y):out.add(p+'(len)');return
        for i,(u,v) in enumerate(zip(x,y)):walk(u,v,p+'[%d]'%i,out)
    elif x!=y:out.add(p)
dirty=changed=0;kinds=collections.Counter()
for g in sorted(gens):
    if g<first:continue
    b=gens[g];base=gens.get(b['built'].get('base'))
    if not base or not isinstance(b['built'].get('dirty'),list):continue
    rows=b['built']['dirty'];dirty+=len(rows)
    ch=[k for k in rows if k in base['owners'] and base['owners'][k]['fragment']!=b['owners'][k]['fragment']];changed+=len(ch)
    for k in random.sample(ch,min(2,len(ch))):
        out=set();walk(load(base['owners'][k]['fragment']),load(b['owners'][k]['fragment']),'',out)
        kinds[tuple(sorted({re.sub(r'\[\d+\]','[]',re.sub(r'/node/(targets|namedInputs)/[^/]+','/node/\\1/*',p)) for p in out}))]+=1
print('implementation',impl[:8],'generations',min(gens),'..',max(gens),'re-inferred',dirty,'fragment changed',changed,'equal result',dirty-changed)
for k,v in kinds.most_common(8):print(v,k)
