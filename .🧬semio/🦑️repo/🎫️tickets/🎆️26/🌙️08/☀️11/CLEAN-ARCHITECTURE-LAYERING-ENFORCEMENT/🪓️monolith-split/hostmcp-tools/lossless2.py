import sys,collections,re
orig=sys.argv[1];files=sys.argv[2:]
def lines(p):return [l.strip() for l in open(p,encoding='utf-8').read().split('\n') if l.strip()]
def norm(l):
    l=l.replace('pub(super) ','',1) if l.startswith('pub(super) ') else l
    l=re.sub(r'^(#\[path\s*=\s*")\.\./',r'\1',l)
    l=re.sub(r'(\bfn|\bstruct|\benum)\b','\\1',l)
    l=l.replace('pub(super) ','')
    return l
A=collections.Counter(lines(orig));B=collections.Counter()
for f in files:
    for l in lines(f):B[norm(l)]+=1
oo=A-B;nn=B-A
for k,v in sorted(oo.items()):print("ONLY-ORIG",v,k[:170])
imp=0
for k,v in sorted(nn.items()):
    tag="IMPORT" if re.match(r'^(#\[cfg.*\]|use |pub use |pub\(crate\) use |mod |//! |//#(end)?region)',k) or k.startswith('#[path') else "OTHER"
    if tag=="IMPORT":imp+=v
    else:print("ONLY-NEW ",v,k[:170])
print("orig",sum(A.values()),"new",sum(B.values()),"added wiring/import/marker/doc lines",imp)
