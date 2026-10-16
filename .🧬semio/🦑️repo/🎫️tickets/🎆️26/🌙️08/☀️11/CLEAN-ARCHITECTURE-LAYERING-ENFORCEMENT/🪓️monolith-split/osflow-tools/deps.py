"""🕸️ File-level reference graph of the os-flow module tree (analysis only)."""
import os,re,sys,collections
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from rlex2 import lex2
from wsan import top_items,classify,parse_use
FLOW=sys.argv[1] if len(sys.argv)>1 else "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow"
SKIP=("node_modules","🗿️artifacts","🫀️core","📚️examples","🏛️ownership")
def rsfiles():
    out=[]
    for dp,dn,fn in os.walk(FLOW):
        dn[:]=[d for d in dn if d not in SKIP and not d.startswith("🕸️bindings")]
        for f in fn:
            if f.endswith(".rs"):out.append(os.path.join(dp,f))
    return sorted(out)
files=rsfiles()
def unit(p):
    r=os.path.relpath(p,FLOW)
    return r
defs={};toks={};tests={}
for p in files:
    t=open(p,encoding='utf-8').read()
    L=[l+'\n' for l in t.split('\n')][:-1]
    I=lex2(L)
    its=top_items(L,I)
    d={}
    for it in its:
        classify(it)
        if it.get('name') and it['kind'] in('fn','struct','enum','const','static','type','trait','mod','union'):
            d[it['name']]=it['kind']
        elif it['kind']=='macro':
            txt=''.join(L[it['head']:it['end']+1])
            m=re.match(r'\s*macro_rules!\s*(\w+)',txt)
            if m:d[m.group(1)]='macro'
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
def top(p):
    r=os.path.relpath(p,FLOW).split(os.sep)
    return r[0] if r[0] not in("🧩️extensions","📦️packages") else "/".join(r[:2])
if __name__=="__main__":
    edges=collections.defaultdict(lambda:collections.defaultdict(set))
    for p in files:
        for n in toks[p]:
            if n in defs[p]:continue
            os_=owner.get(n)
            if not os_ or len(os_)>2:continue
            for q in os_:
                if q!=p:edges[p][q].add(n)
    # module level
    mod=collections.defaultdict(lambda:collections.defaultdict(set))
    for p in edges:
        for q,ns in edges[p].items():
            if top(p)!=top(q):mod[top(p)][top(q)]|=ns
    for a in sorted(mod):
        for b in sorted(mod[a]):
            print(f"{a} -> {b}: {len(mod[a][b])} {sorted(mod[a][b])[:12]}")
