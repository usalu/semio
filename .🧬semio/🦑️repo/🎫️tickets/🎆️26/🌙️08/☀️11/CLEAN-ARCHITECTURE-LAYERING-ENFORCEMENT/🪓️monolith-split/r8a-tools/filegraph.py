import re,os,sys,collections,json
# usage: filegraph.py <dir> [--tests]  : approximate item-level reference graph between the .rs files below <dir>
root=sys.argv[1]; with_tests='--tests' in sys.argv
files=[]
for dp,dn,fn in os.walk(root):
    for f in fn:
        p=os.path.join(dp,f)
        if f.endswith('.rs') and (with_tests or '🧪️tests' not in p): files.append(p)
rel=lambda p:p[len(root)+1:]
defs={}  # name -> file
text={}
pat=re.compile(r'^\s*(?:pub(?:\([^)]*\))?\s+)?(?:unsafe\s+)?(?:async\s+)?(?:const\s+)?(fn|struct|enum|const|static|type|trait|union)\s+([A-Za-z_][A-Za-z0-9_]*)',re.M)
for p in files:
    t=open(p,encoding='utf8').read(); text[p]=t
    # only top-level (column 0) items count as module items
    for m in re.finditer(r'^(?:pub(?:\([^)]*\))?\s+)?(?:unsafe\s+)?(?:async\s+)?(?:const\s+)?(fn|struct|enum|const|static|type|trait|union)\s+([A-Za-z_][A-Za-z0-9_]*)',t,re.M):
        n=m.group(2)
        if n=='_' : continue
        defs.setdefault(n,[]).append(p)
ign=set(next((a[9:].split(',') for a in sys.argv if a.startswith('--ignore=')),[]))
uniq={n:ps[0] for n,ps in defs.items() if n not in ign and 0 or True}
uniq={n:ps[0] for n,ps in defs.items() if len(ps)==1 and re.search(r'[A-Z]|_',n) and len(n)>8 and n not in ign}
edges=collections.defaultdict(lambda:collections.defaultdict(set))
for p,t in text.items():
    ids=set(re.findall(r'[A-Za-z_][A-Za-z0-9_]*',t))
    for n,q in uniq.items():
        if q!=p and n in ids: edges[p][q].add(n)
G={p:set(edges[p].keys()) for p in files}
idx={};low={};st=[];on=set();res=[];c=[0]
sys.setrecursionlimit(100000)
def v(n):
    idx[n]=low[n]=c[0];c[0]+=1;st.append(n);on.add(n)
    for m in G[n]:
        if m not in idx: v(m);low[n]=min(low[n],low[m])
        elif m in on: low[n]=min(low[n],idx[m])
    if low[n]==idx[n]:
        comp=[]
        while True:
            x=st.pop();on.discard(x);comp.append(x)
            if x==n:break
        res.append(comp)
for n in files:
    if n not in idx:v(n)
lines={p:text[p].count('\n')+1 for p in files}
big=[c for c in res if len(c)>1]
print('files',len(files),'SCCs>1:',[ (len(c),sum(lines[x] for x in c)) for c in big])
if '--scc' in sys.argv:
  for c in big:
    print('--- SCC',len(c))
    for x in sorted(c): print('  ',lines[x],rel(x)[:100])
json.dump({rel(p):{rel(q):sorted(ns) for q,ns in edges[p].items()} for p in files},open('/tmp/r8a/filegraph.json','w'),ensure_ascii=False)
