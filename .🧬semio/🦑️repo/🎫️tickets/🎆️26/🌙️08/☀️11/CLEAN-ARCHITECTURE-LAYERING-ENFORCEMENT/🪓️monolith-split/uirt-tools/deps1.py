import re
from items import items,R
lines,full=items(R+'♻️reconcile/🦀️.rs')
def name_of(t):
    m=re.search(r'\b(?:fn|struct|enum|const|static|type|trait|mod)\s+([A-Za-z_][A-Za-z0-9_]*)',t)
    if m: return m.group(1)
    m=re.search(r'\bimpl\b.*?\b([A-Z][A-Za-z0-9_]*)',t)
    return 'impl '+(m.group(1) if m else '?')
# group by line range
def grp(s):
    if 29<=s<=237: return 'F'   # identity
    if 599<=s<=656: return 'F'
    if 746<=s<=1380 and not (s==749): return 'F'
    if s==749: return 'F'
    if s in (2404,2405,2406,2407,2408,2409,2410): return 'F?'
    return 'C'
G={}
defs={}
for k,s,e,t in full:
    if k!='item': continue
    g=grp(s)
    if 658<=s<=744: g='C'
    G[s]=(g,e,t)
    n=name_of(t)
    if not n.startswith('impl'): defs.setdefault(n,[]).append((s,g))
idents=set(defs)
for s,(g,e,t) in sorted(G.items()):
    if g!='F': continue
    body='\n'.join(lines[s-1:e])
    refs=set(re.findall(r'[A-Za-z_][A-Za-z0-9_]*',body))
    for r in sorted(refs&idents):
        dg=[x for x in defs[r] if x[1]!='F']
        if dg and not all(x[0]==s for x in defs[r]): print('F item line',s,name_of(t),'-> uses core item',r,defs[r])
