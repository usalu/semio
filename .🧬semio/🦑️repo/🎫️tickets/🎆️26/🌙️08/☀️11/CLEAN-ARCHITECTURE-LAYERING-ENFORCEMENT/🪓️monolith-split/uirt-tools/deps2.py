import re
from items import items,R
lines,full=items(R+'♻️reconcile/🦀️.rs')
def name_of(t):
    m=re.search(r'\b(?:fn|struct|enum|const|static|type|trait|mod)\s+([A-Za-z_][A-Za-z0-9_]*)',t)
    if m: return m.group(1)
    m=re.search(r'\bimpl\b.*?\b([A-Z][A-Za-z0-9_]*)',t)
    return 'impl '+(m.group(1) if m else '?')
F_RANGES=[(29,237),(554,576),(599,656),(746,747),(749,766),(768,1380),(2404,2411)]
REC_RANGES=[(658,744),(3851,3945),(4137,4165)]
def grp(s):
    for a,b in F_RANGES:
        if a<=s<=b: return 'P'
    for a,b in REC_RANGES:
        if a<=s<=b: return 'P'
    return 'C'
G={};defs={}
for k,s,e,t in full:
    if k!='item': continue
    g=grp(s); G[s]=(g,e,t); n=name_of(t)
    if not n.startswith('impl') and n not in ('N','_'): defs.setdefault(n,[]).append((s,g))
idents=set(defs)
import sys
print("P items using C-defined names:")
for s,(g,e,t) in sorted(G.items()):
    if g!='P': continue
    body='\n'.join(lines[s-1:e]); refs=set(re.findall(r'[A-Za-z_][A-Za-z0-9_]*',body))
    for r in sorted(refs&idents):
        if all(x[1]=='C' for x in defs[r]): print('  ',s,name_of(t),'->',r,defs[r])
print("C items using P-defined names (these need `use`):")
need=set()
for s,(g,e,t) in sorted(G.items()):
    if g!='C': continue
    body='\n'.join(lines[s-1:e]); refs=set(re.findall(r'[A-Za-z_][A-Za-z0-9_]*',body))
    for r in refs&idents:
        if all(x[1]=='P' for x in defs[r]): need.add(r)
print(sorted(need))
