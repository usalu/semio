"""🕸️ Module graph v2: top-level (column 0) definitions only; reports ambiguous names."""
import os,re,collections
from modgraph import MODS,files_of,D
from rs import strip,idents
TOP=re.compile(r'^(?:pub(?:\([^)]*\))?\s+)?(?:unsafe\s+)?(?:async\s+)?(?:const\s+)?(fn|struct|enum|trait|type|static|const|union|macro_rules!)\s+([A-Za-z_][A-Za-z0-9_]*)',re.M)
def topdefs(path):
    t=strip(open(path,encoding='utf-8').read())
    return {m.group(2) for m in TOP.finditer(t)}
def load(m,tests):
    return '\n'.join(strip(open(p,encoding='utf-8').read()) for p in files_of(m,tests))
if __name__=='__main__':
    defs={m:set().union(*[topdefs(p) for p in files_of(m,False)]) for m in MODS}
    owners=collections.defaultdict(set)
    for m,s in defs.items():
        for n in s: owners[n].add(m)
    amb={n:o for n,o in owners.items() if len(o)>1}
    print('ambiguous:',amb)
    for label,tests in (('LIB',False),('TEST',True)):
        print('=====',label)
        for m in MODS:
            used=idents(load(m,tests))
            edges=collections.Counter()
            for n in used:
                o=owners.get(n)
                if o and len(o)==1 and m not in o: edges[next(iter(o))]+=1
            print(f'{m:12s} ->',', '.join(f'{k}:{v}' for k,v in sorted(edges.items(),key=lambda x:-x[1])))
