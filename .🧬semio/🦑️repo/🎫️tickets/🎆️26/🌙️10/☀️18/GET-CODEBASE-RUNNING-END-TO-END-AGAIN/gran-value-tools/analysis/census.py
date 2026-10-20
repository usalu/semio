"""👥 Census of the direct dependents of the value facade and derive crates: which names and derives each one uses in production and in test sources. Run with `cwd` = the generated folder."""
from graph import *
from tree import *
import re,json,collections,time
FD=V+'/📦️packages/🦀️rust'
DV=V+'/✨️derive/📦️packages/🦀️rust'
cons=sorted(set(rev[FD])|set(rev[DV]))
DER=['ToValue','FromValue','NativeReceive','RetainedClone','RetainedEquality','RetireOwned','SnapshotProjection','CanonicalJsonTree','FactoryPayloadRetirement']
def groups(s):
    out=[];depth=0;cur=''
    for ch in s:
        if ch=='{':depth+=1
        if ch=='}':depth-=1
        if ch==',' and depth==0: out.append(cur.strip());cur=''
        else: cur+=ch
    if cur.strip(): out.append(cur.strip())
    return out
USE=re.compile(r'semio_framework_value\b(?!_)\s*(?:::\s*(\{.*?\}\s*;|[A-Za-z_][A-Za-z0-9_]*)|(?:as\s+\w+)?\s*;)',re.S)
res={}
t0=time.time()
for n,d in enumerate(cons):
    c=C[d]
    try: fs=module_tree(libfile(d,c))
    except Exception as e: fs=[]
    prod=[];tst=[]
    for f in fs:
        try: t=open(f,encoding='utf-8',errors='ignore').read()
        except Exception: continue
        t=re.sub(r'//[^\n]*','',t)
        (tst if '🧪️' in f else prod).append(t)
    P='\n'.join(prod);T='\n'.join(tst)
    def names(txt):
        s=set()
        for m in USE.finditer(txt):
            g=m.group(1)
            if g is None: s.add('<crate>');continue
            if g.startswith('{'):
                inner=g.rstrip().rstrip(';').rstrip()[1:-1]
                for it in groups(inner):
                    nm=re.match(r'[A-Za-z_][A-Za-z0-9_]*',it)
                    if nm: s.add(nm.group(0))
            else: s.add(g)
        for m in re.finditer(r'semio_framework_value\b(?!_)\s+as\s+(\w+)',txt): s.add('<alias:'+m.group(1)+'>')
        return s
    def ders(txt):
        s=set()
        for m in re.finditer(r'derive\(([^)]*)\)',txt):
            for x in m.group(1).split(','):
                x=x.strip().split('::')[-1]
                if x in DER: s.add(x)
        for x in ('value_codec','owned_json_file'):
            if re.search(r'\b'+x+r'!',txt): s.add(x)
        return s
    res[d]={'prod_names':sorted(names(P)),'test_names':sorted(names(T)),'prod_derives':sorted(ders(P)),'test_derives':sorted(ders(T)),'nfiles':len(fs),
            'kinds':{k:(any(t==FD for _,t in c['deps'][k]),any(t==DV for _,t in c['deps'][k])) for k in ('normal','build','dev')}}
    if n%400==0: print(n,len(cons),round(time.time()-t0),flush=True)
json.dump(res,open('census.json','w'),ensure_ascii=False)
print('done',round(time.time()-t0))
