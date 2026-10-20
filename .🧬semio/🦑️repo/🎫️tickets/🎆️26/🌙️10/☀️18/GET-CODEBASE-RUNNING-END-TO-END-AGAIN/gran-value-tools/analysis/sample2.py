"""🎲 Seeded 240-crate sample of the facade and derive dependents. Run with `cwd` = the generated folder."""
from graph import *
from tree import *
import re,random,collections,json
FD=V+'/📦️packages/🦀️rust'
DV=V+'/✨️derive/📦️packages/🦀️rust'
deps=sorted(rev[FD]); dd=sorted(rev[DV])
print('facade direct dependents',len(deps),'derive direct dependents',len(dd),'union',len(set(deps)|set(dd)),'intersection',len(set(deps)&set(dd)))
random.seed(2026)
smp=random.sample(sorted(set(deps)|set(dd)),240)
DER=['ToValue','FromValue','NativeReceive','RetainedClone','RetainedEquality','RetireOwned','SnapshotProjection','CanonicalJsonTree','FactoryPayloadRetirement','value_codec','owned_json_file']
cls=collections.Counter(); derives=collections.Counter(); kinds=collections.Counter()
rows=[]
for d in smp:
    c=C[d]
    fs=module_tree(libfile(d,c))
    prod='\n'.join(open(f,encoding='utf-8',errors='ignore').read() for f in fs if '🧪️' not in f)
    allt='\n'.join(open(f,encoding='utf-8',errors='ignore').read() for f in fs)
    prod=re.sub(r'//.*','',prod)
    used=set()
    for m in re.finditer(r'derive\(([^)]*)\)',prod):
        for x in m.group(1).split(','):
            x=x.strip().split('::')[-1]
            if x in DER: used.add(x)
    for x in ('value_codec','owned_json_file'):
        if re.search(r'\b'+x+r'!',prod): used.add(x)
    fac=bool(re.search(r'semio_framework_value\b(?!_)',prod))
    facall=bool(re.search(r'semio_framework_value\b(?!_)',allt))
    hasd=d in set(deps); hasdv=d in set(dd)
    for u in used: derives[u]+=1
    k=('derive-only' if used and not fac else 'facade-only' if fac and not used else 'both' if fac and used else 'test-only' if facall else 'none')
    kinds[k]+=1
    rows.append((d,sorted(used),fac))
print(len(smp),kinds)
print({k:round(v/len(smp)*100,1) for k,v in derives.most_common()})
tv=sum(1 for d,u,f in rows if {'ToValue','FromValue','RetireOwned','CanonicalJsonTree'}&set(u))
print('codec-class derive users',tv)
print('retained-class',sum(1 for d,u,f in rows if {'RetainedClone','SnapshotProjection'}&set(u)))
print('owned-class',sum(1 for d,u,f in rows if {'NativeReceive','RetainedEquality'}&set(u)))
print('factory-class',sum(1 for d,u,f in rows if {'FactoryPayloadRetirement'}&set(u)))
json.dump({'n':len(smp),'kinds':kinds,'derives':derives},open('sample2.json','w'))
