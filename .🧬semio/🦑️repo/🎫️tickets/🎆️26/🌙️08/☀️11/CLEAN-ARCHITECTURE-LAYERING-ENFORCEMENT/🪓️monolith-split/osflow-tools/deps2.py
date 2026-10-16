"""🕸️ File-to-file reference graph (specific names only) inside one directory of the os-flow tree."""
import os,re,sys,collections
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from rlex2 import lex2
from wsan import top_items,classify
FLOW="/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow"
prefix=sys.argv[1]
SKIP=("node_modules","🗿️artifacts","🫀️core","📚️examples","🏛️ownership","🕸️bindings")
files=[]
for dp,dn,fn in os.walk(FLOW):
    dn[:]=[d for d in dn if d not in SKIP]
    for f in fn:
        if f.endswith(".rs"):files.append(os.path.join(dp,f))
files.sort()
def specific(n):
    return (n[:1].isupper() and len(n)>3) or ('_' in n and len(n)>=7) or n.isupper()
defs={};toks={}
for p in files:
    t=open(p,encoding='utf-8').read()
    L=[l+'\n' for l in t.split('\n')][:-1]
    I=lex2(L)
    d={}
    for it in top_items(L,I):
        classify(it)
        if it.get('name') and it['kind'] in('fn','struct','enum','const','static','type','trait','union'):d[it['name']]=it['kind']
        if it['kind']=='macro':
            m=re.match(r'\s*macro_rules!\s*(\w+)',''.join(L[it['head']:it['end']+1]))
            if m:d[m.group(1)]='macro'
    # methods / assoc fns in impl blocks (name only)
    for i,info in enumerate(I):
        if info['depth']==1 and info['state']=='code':
            m=re.match(r'^\s{1,4}(?:pub(?:\([^)]*\))?\s+)?(?:(?:unsafe|async|const)\s+)*fn\s+(\w+)',L[i])
            if m and specific(m.group(1)):d.setdefault(m.group(1),'method')
    defs[p]=d
    s=set()
    for info in I:
        for n,c in info['toks']:s.add(n)
        for st in info['strs']:
            for m in re.finditer(r'\{([A-Za-z_]\w*)(?::[^}]*)?\}',st):s.add(m.group(1))
    toks[p]=s
owner=collections.defaultdict(set)
for p,d in defs.items():
    for n in d:owner[n].add(p)
sel=[p for p in files if os.path.relpath(p,FLOW).startswith(prefix)]
rel=lambda p:os.path.relpath(p,FLOW)
for p in sel:
    out=collections.defaultdict(set)
    for n in toks[p]:
        if n in defs[p] or not specific(n):continue
        os_=owner.get(n)
        if not os_ or len(os_)>3:continue
        for q in os_:
            if q!=p:out[q].add(n)
    print("==",rel(p))
    for q,ns in sorted(out.items(),key=lambda kv:rel(kv[0])):
        print("   ->",rel(q),len(ns),sorted(ns)[:8])
