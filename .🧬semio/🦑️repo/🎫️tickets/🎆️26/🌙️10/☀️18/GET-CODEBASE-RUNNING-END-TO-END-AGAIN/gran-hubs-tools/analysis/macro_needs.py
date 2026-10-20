"""🧩 Crates a facade's exported macros expand into (`$crate::path` resolved through the facade model to the first non-facade crate, plus explicit `::semio_framework_x::` crate paths). `python3 macro_needs.py <hub>`; library: macro_needs(hub) -> (set(dir), {macro: set(ident)})."""
from rewire import *
import re
def macro_needs(hub):
    d=json.load(open(OUT+'/census-%s.json'%hub));ID=d['ident']
    m=WORLD.model_for(ID);per=collections.defaultdict(set)
    for f in ptree(hub):
        if not isprod(f): continue
        t=open(f,encoding='utf-8',errors='ignore').read()
        if 'macro_rules!' not in t: continue
        ms=list(re.finditer(r'macro_rules!\s+(\w+)\s*\{',t))
        for i,mm in enumerate(ms):
            end=ms[i+1].start() if i+1<len(ms) else len(t)
            body=t[mm.end():end]
            for x in re.finditer(r'\$crate::([A-Za-z0-9_]+(?:::[A-Za-z0-9_]+)*)',body):
                segs=x.group(1).split('::');q=list(segs);r=None
                while q:
                    r=WORLD.resolve(m,q)
                    if r: break
                    q.pop()
                if r and r['kind']!='external':
                    ch=[c for c in r.get('chain',[]) if c not in FAC and c!=ID] or ([r['owner']] if r['owner']!=ID else [])
                    if ch: per[mm.group(1)].add(ch[0])
                    else: per[mm.group(1)].add('<self>')
                else: per[mm.group(1)].add('<unresolved:%s>'%'::'.join(segs[:2]))
            for x in re.finditer(r'(?<![A-Za-z0-9_])::?(semio_framework_[a-z0-9_]+)::',body):
                per[mm.group(1)].add(x.group(1))
    dirs={I2D[i] for s in per.values() for i in s if i in I2D}
    return dirs,per
if __name__=='__main__':
    dirs,per=macro_needs(sys.argv[1])
    print(sys.argv[1],'macros',len(per),'needed crates',len(dirs))
    print(sorted(R0.C[x]['name'].replace('semio-framework-','') for x in dirs))
    for k,v in per.items(): print('  ',k,sorted(i.replace('semio_framework_','') for i in v)[:12])
