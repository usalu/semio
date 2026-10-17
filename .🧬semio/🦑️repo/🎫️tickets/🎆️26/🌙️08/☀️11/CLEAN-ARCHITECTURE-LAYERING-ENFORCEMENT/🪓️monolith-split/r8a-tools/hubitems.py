import re,sys,os,subprocess,collections
# usage: hubitems.py <hub-file> <subtree-dir> : lists top-level items of the hub file and which files under the subtree name them
hub,tree=sys.argv[1],sys.argv[2]
src=open(hub,errors='ignore').read()
items=collections.OrderedDict()
for m in re.finditer(r'^(?:pub(?:\([^)]*\))?\s+)?(?:async\s+)?(?:unsafe\s+)?(fn|struct|enum|const|static|type|trait|union)\s+([A-Za-z_][A-Za-z0-9_]*)',src,re.M):
    items[m.group(2)]=m.group(1)
files=[]
for dp,dn,fn in os.walk(tree):
    for f in fn:
        if f.endswith('.rs'): files.append(os.path.join(dp,f))
hubabs=os.path.abspath(hub)
use=collections.defaultdict(set)
for f in files:
    if os.path.abspath(f)==hubabs: continue
    t=open(f,errors='ignore').read()
    for n in items:
        if re.search(r'\b'+re.escape(n)+r'\b',t): use[n].add(os.path.relpath(f,tree))
print('hub items',len(items),'lines',src.count('\n'))
for n,k in items.items():
    print(f'{k:6} {n:60} used-by {len(use[n])}')
