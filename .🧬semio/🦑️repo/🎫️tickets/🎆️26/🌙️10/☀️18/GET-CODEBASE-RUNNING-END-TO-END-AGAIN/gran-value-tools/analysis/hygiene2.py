"""🧹 Re-classifies unused and test-only edges including bin, test, example, bench and build roots. Run with `cwd` = the generated folder."""
from graph import *
from tree import *
import json,re,os,collections
H=json.load(open('hygiene.json'))
cand=[(d,t,u) for d,t,k,u in H if k=='normal' and u in('none','test')]
bydep=collections.defaultdict(list)
for d,t,u in cand: bydep[d].append((t,u))
IMPL={'semio-framework-value':('ToValue','FromValue','RetireOwned','CanonicalJsonTree','RetainedClone','FactoryPayloadRetirement','value_codec','SnapshotProjection','NativeReceive','RetainedEquality'),'semio-framework-value-derive':('ToValue','FromValue','RetireOwned','CanonicalJsonTree','RetainedClone','FactoryPayloadRetirement','value_codec','SnapshotProjection','NativeReceive','RetainedEquality','owned_json_file'),'semio-framework-value-native-owned':('NativeReceive',),'semio-framework-value-persistent':('NativeReceive','RetainedEquality')}
def target_roots(d):
    t=open(os.path.join(ROOT,d,'Cargo.toml'),encoding='utf-8').read()
    prod=[];test=[]
    for m in re.finditer(r'^\[\[(bin|test|example|bench)\]\](.*?)(?=^\[|\Z)',t,re.S|re.M):
        pm=re.search(r'^\s*path\s*=\s*"([^"]+)"',m.group(2),re.M)
        if pm: ((prod if m.group(1)=='bin' else test)).append(os.path.normpath(os.path.join(ROOT,d,pm.group(1))))
    bm=re.search(r'^\s*build\s*=\s*"([^"]+)"',t,re.M)
    if bm: prod.append(os.path.normpath(os.path.join(ROOT,d,bm.group(1))))
    # default bins
    for sub in ('src/main.rs','build.rs'):
        p=os.path.join(ROOT,d,sub)
        if os.path.exists(p): prod.append(p)
    return prod,test
def read(fs):
    out=[]
    for f in fs:
        try: out.append((f,re.sub(r'//[^\n]*','',open(f,encoding='utf-8',errors='ignore').read())))
        except Exception: pass
    return out
final=collections.defaultdict(dict);flipped=0
for d,lst in bydep.items():
    c=C[d]
    lib=module_tree(libfile(d,c))
    pr,te=target_roots(d)
    pf=set(lib)
    for r in pr: pf|=set(module_tree(r))
    tf=set()
    for r in te: tf|=set(module_tree(r))
    P='\n'.join(t for f,t in read(sorted(pf)) if '🧪️' not in f)
    P2='\n'.join(t for f,t in read(sorted(pf)) if '🧪️' in f)
    T='\n'.join(t for f,t in read(sorted(tf)))+'\n'+P2
    for t,u0 in lst:
        nm=C[t]['name'];i=nm.replace('-','_')
        if re.search(r'\b'+i+r'\b',P) or any(re.search(r'derive\([^)]*\b'+x+r'\b',P) for x in IMPL.get(nm,())) : u='prod'
        elif re.search(r'\b'+i+r'\b',T) or any(re.search(r'derive\([^)]*\b'+x+r'\b',T) for x in IMPL.get(nm,())): u='test'
        else: u='none'
        if u!=u0: flipped+=1
        final[d][t]=u
cnt=collections.Counter(u for d in final for u in final[d].values())
print('candidates',len(cand),'after target-root refinement',cnt,'flipped',flipped,'crates',len(final))
json.dump({d:{t:u for t,u in v.items()} for d,v in final.items()},open('hygiene-final.json','w'),ensure_ascii=False)
