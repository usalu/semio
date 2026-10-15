import sys,collections,os,re,glob
orig=sys.argv[1];parent=sys.argv[2];kids=sys.argv[3:]
def lines(p):
    return [l.strip() for l in open(p,encoding='utf-8').read().split('\n') if l.strip()]
def norm(l):
    l=l.replace('"../🧪️tests/','"🧪️tests/').replace('"../🧬️schema/','"🧬️schema/')
    return l
A=collections.Counter(lines(orig))
B=collections.Counter()
for f in [parent]+kids:
    for l in lines(f):B[norm(l)]+=1
for k,v in sorted((A-B).items()):print("ONLY-ORIG",v,k[:200])
for k,v in sorted((B-A).items()):print("ONLY-NEW ",v,k[:200])
print("orig",sum(A.values()),"new",sum(B.values()))
