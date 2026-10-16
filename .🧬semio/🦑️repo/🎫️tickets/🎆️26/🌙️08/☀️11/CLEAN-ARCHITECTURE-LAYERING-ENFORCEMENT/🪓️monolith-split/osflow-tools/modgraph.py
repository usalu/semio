"""🗺️ Folder-level reference graph of one crate tree with named edges. usage: modgraph.py <rootdir> <depth-grouping-regex-file-list...>"""
import os,re,sys,collections
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from rlex2 import lex2
from wsan import top_items,classify
root=sys.argv[1].rstrip("/")+"/"
def groupof(rel):
    parts=rel.split("/")
    if parts[0]=="🚪️io":return "/".join(parts[:3]) if len(parts)>2 else parts[0]
    if parts[0]=="🌿️vcs":return "/".join(parts[:3]) if len(parts)>3 else parts[0]
    return parts[0] if len(parts)>1 else "ROOT"
files=[]
for dp,dn,fn in os.walk(root):
    dn[:]=[d for d in dn if d not in("node_modules","📦️packages")]
    for f in fn:
        if f.endswith(".rs"):files.append(os.path.join(dp,f))
def specific(n):return (n[:1].isupper() and len(n)>3) or ('_' in n and len(n)>=7) or n.isupper()
defs={};toks={}
for p in files:
    t=open(p,encoding='utf-8').read();L=[l+'\n' for l in t.split('\n')][:-1];I=lex2(L);d={}
    for it in top_items(L,I):
        classify(it)
        if it.get('name') and it['kind'] in('fn','struct','enum','const','static','type','trait','union'):d[it['name']]=1
    for i,info in enumerate(I):
        if info['depth']==1 and info['state']=='code':
            m=re.match(r'^\s{1,4}(?:pub(?:\([^)]*\))?\s+)?(?:(?:unsafe|async|const)\s+)*fn\s+(\w+)',L[i])
            if m and specific(m.group(1)):d.setdefault(m.group(1),1)
    defs[p]=d
    s=set()
    for info in I:
        for n,c in info['toks']:s.add(n)
    toks[p]=s
owner=collections.defaultdict(set)
for p,d in defs.items():
    for n in d:owner[n].add(p)
rel=lambda p:os.path.relpath(p,root)
istest=lambda p:"/🧪️tests/" in p
edges=collections.defaultdict(lambda:collections.defaultdict(set))
for p in files:
    if istest(p):continue
    for n in toks[p]:
        if n in defs[p] or not specific(n):continue
        os_=[q for q in owner.get(n,()) if not istest(q)]
        if not os_ or len(os_)>2:continue
        for q in os_:
            if groupof(rel(p))!=groupof(rel(q)):edges[groupof(rel(p))][groupof(rel(q))].add(n)
sizes=collections.Counter()
for p in files:
    sizes[groupof(rel(p))]+=os.path.getsize(p)
for g in sorted(edges):
    print(g,sizes[g])
    for h in sorted(edges[g]):print("   ->",h,len(edges[g][h]),sorted(edges[g][h])[:6])
print(dict(sizes))
