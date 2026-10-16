"""🕸️ Cross-group reference check for the flow-flow carve: which names a lower group takes from a higher one."""
import os,re,sys,collections
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from rlex2 import lex2
from wsan import top_items,classify
FF="/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/"
def group(rel):
    if rel.startswith("🌿️vcs/") or "🧬️mutations/" in rel and rel.startswith("🚪️io/"):return "mutation"
    if rel.startswith("🧵️retained/📑️copy/"):return "copy"
    if rel.startswith("🧵️retained/🧪️tests/") or rel.startswith("🧪️tests/"):return "tests"
    return "document"
files=[]
for dp,dn,fn in os.walk(FF):
    dn[:]=[d for d in dn if d not in("node_modules","🕸️bindings","📦️packages","dist")]
    for f in fn:
        if f.endswith(".rs"):files.append(os.path.join(dp,f))
files.sort()
def specific(n):return (n[:1].isupper() and len(n)>3) or ('_' in n and len(n)>=7) or n.isupper()
defs={};toks={}
for p in files:
    t=open(p,encoding='utf-8').read();L=[l+'\n' for l in t.split('\n')][:-1];I=lex2(L);d={}
    for it in top_items(L,I):
        classify(it)
        if it.get('name') and it['kind'] in('fn','struct','enum','const','static','type','trait','union'):d[it['name']]=it['kind']
    for i,info in enumerate(I):
        if info['depth']==1 and info['state']=='code':
            m=re.match(r'^\s{1,4}(?:pub(?:\([^)]*\))?\s+)?(?:(?:unsafe|async|const)\s+)*fn\s+(\w+)',L[i])
            if m and specific(m.group(1)):d.setdefault(m.group(1),'method')
    defs[p]=d;s=set()
    for info in I:
        for n,c in info['toks']:s.add(n)
    toks[p]=s
owner=collections.defaultdict(set)
for p,d in defs.items():
    for n in d:owner[n].add(p)
order={"document":0,"copy":1,"mutation":1,"tests":2}
for p in files:
    rel=p[len(FF):];g=group(rel)
    if g=="tests":continue
    out=collections.defaultdict(set)
    for n in toks[p]:
        if n in defs[p] or not specific(n):continue
        for q in owner.get(n,()):
            gq=group(q[len(FF):])
            if gq!=g and gq!="tests" and (g=="document" or (g in("copy","mutation") and gq!="document")):
                out[q[len(FF):]].add(n)
    if out:
        print("==",g,rel)
        for q,ns in sorted(out.items()):print("   ->",group(q),q,sorted(ns)[:10])
