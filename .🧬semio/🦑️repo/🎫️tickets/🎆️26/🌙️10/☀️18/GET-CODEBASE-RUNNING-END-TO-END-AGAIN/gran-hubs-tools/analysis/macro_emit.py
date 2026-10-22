"""🧩 Which exported macros (macro_rules! bodies and proc-macro crates) emit paths into the hub crates: a dependent that invokes such a macro needs the hub edge although its own text never names the crate. Scans module trees of crates that have at least one dependent (manifest graph scope). Writes 🗑️generated/gran-hubs/macro-emit.json."""
from hubenv import *
import re,time
IDENTS={h:'semio_framework_'+h.replace('-','_') for h in HUBS}
IDENTS['umbrella']='semio_framework'
cands=[d for d in C if rev.get(d)]
print('crates with dependents',len(cands))
out=collections.defaultdict(list);t0=time.time()
for n,d in enumerate(cands):
    try: fs=module_tree(libfile(d,C[d]))
    except Exception: continue
    for f in fs:
        if not isprod(f): continue
        try: t=open(f,encoding='utf-8',errors='ignore').read()
        except Exception: continue
        if 'macro_rules!' not in t: continue
        if not any(i in t for i in IDENTS.values()): continue
        for m in re.finditer(r'macro_rules!\s+(\w+)\s*\{',t):
            if 'macro_export' not in t[max(0,m.start()-160):m.start()]: continue
            d_=1;j_=m.end()
            while j_<len(t) and d_:
                ch=t[j_]
                if ch=='{': d_+=1
                elif ch=='}': d_-=1
                j_+=1
            seg=t[m.end():j_]
            for h,i in IDENTS.items():
                if re.search(r'(?<![A-Za-z0-9_])'+i+r'(?![A-Za-z0-9_])',seg): out[h].append([C[d]['name'],m.group(1),os.path.relpath(f,ROOT)[-60:]])
    if n%600==0: print(n,round(time.time()-t0),flush=True)
# proc macros
for d,c in C.items():
    if str(c['proc_macro'])!='True': continue
    fs=module_tree(libfile(d,c));txt=''.join(open(f,encoding='utf-8',errors='ignore').read() for f in fs)
    for h,i in IDENTS.items():
        if re.search(r'(?<![A-Za-z0-9_])'+i+r'(?![A-Za-z0-9_])',txt): out[h].append([c['name'],'<proc-macro>',''])
json.dump(out,open(OUT+'/macro-emit.json','w'),ensure_ascii=False)
for h,v in out.items(): print(h,len(v),sorted(set((a,b) for a,b,_ in v))[:12])
