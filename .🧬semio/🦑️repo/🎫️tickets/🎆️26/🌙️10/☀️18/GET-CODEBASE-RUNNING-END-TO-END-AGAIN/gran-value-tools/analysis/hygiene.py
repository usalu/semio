"""🧹 Classifies every normal edge from outside the value family into it as prod-used, test-only or unused. Run with `cwd` = the generated folder."""
from graph import *
from tree import *
import re,json,collections,time
famset=set(fam)
ds=sorted({a for f in fam for a in rev[f] if a not in famset})
print('non-family direct dependents of family crates',len(ds))
res={}  # (dependent, family crate) -> 'prod'|'test'|'none'
t0=time.time()
IMPLICIT={ 'semio-framework-value-native-owned':('NativeReceive',), 'semio-framework-value-persistent':('NativeReceive','RetainedEquality'), 'semio-framework-value':('ToValue','FromValue','RetireOwned','CanonicalJsonTree','RetainedClone','FactoryPayloadRetirement','value_codec','SnapshotProjection','NativeReceive','RetainedEquality'),'semio-framework-value-derive':('ToValue','FromValue','RetireOwned','CanonicalJsonTree','RetainedClone','FactoryPayloadRetirement','value_codec','SnapshotProjection','NativeReceive','RetainedEquality','owned_json_file')}
for n,d in enumerate(ds):
    c=C[d]
    fs=module_tree(libfile(d,c))
    P=[];T=[]
    for f in fs:
        try:t=open(f,encoding='utf-8',errors='ignore').read()
        except Exception: continue
        t=re.sub(r'//[^\n]*','',t)
        (T if '🧪️' in f else P).append(t)
    P='\n'.join(P);T='\n'.join(T)
    for kind in ('normal','build'):
        for dn,t in c['deps'][kind]:
            if t not in famset: continue
            nm=C[t]['name'];i=nm.replace('-','_')
            if kind=='build': res[(d,t,kind)]='build';continue
            if re.search(r'\b'+i+r'\b',P): u='prod'
            elif any(re.search(r'derive\([^)]*\b'+x+r'\b',P) for x in IMPLICIT.get(nm,())): u='prod'
            elif any(re.search(r'\b'+x+r'!',P) for x in ('value_codec','owned_json_file') if x in IMPLICIT.get(nm,())): u='prod'
            elif re.search(r'\b'+i+r'\b',T): u='test'
            else: u='none'
            res[(d,t,kind)]=u
    if n%500==0: print(n,round(time.time()-t0),flush=True)
out=collections.defaultdict(collections.Counter)
for (d,t,k),u in res.items(): out[C[t]['name'].replace('semio-framework-','')][u]+=1
for nm,c in sorted(out.items(),key=lambda x:-sum(x[1].values())): print('%-34s'%nm,dict(c))
json.dump([[d,t,k,u] for (d,t,k),u in res.items()],open('hygiene.json','w'),ensure_ascii=False)
tot=collections.Counter(u for u in res.values());print(tot)
