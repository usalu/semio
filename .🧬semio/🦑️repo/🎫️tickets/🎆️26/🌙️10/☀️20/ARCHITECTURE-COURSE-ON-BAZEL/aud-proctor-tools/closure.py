import re,sys,collections
lock=open(sys.argv[1],encoding='utf-8').read()
pkgs=[]
for block in lock.split('[[package]]')[1:]:
    name=re.search(r'^name = "([^"]+)"',block,re.M).group(1)
    ver=re.search(r'^version = "([^"]+)"',block,re.M).group(1)
    src=re.search(r'^source = "([^"]+)"',block,re.M)
    deps=re.search(r'^dependencies = \[(.*?)\]',block,re.M|re.S)
    dl=re.findall(r'"([^"]+)"',deps.group(1)) if deps else []
    pkgs.append((name,ver,src.group(1) if src else None,dl))
byname=collections.defaultdict(list)
for p in pkgs: byname[p[0]].append(p)
def key(dep):
    parts=dep.split(' ')
    return parts[0], (parts[1] if len(parts)>1 else None)
def resolve(dep):
    n,v=key(dep)
    c=byname.get(n,[])
    if v:
        for p in c:
            if p[1]==v: return p
    return c[0] if c else None
roots=[p for p in pkgs if p[0] in sys.argv[2].split(',')]
seen={}
stack=list(roots)
while stack:
    p=stack.pop()
    k=(p[0],p[1])
    if k in seen: continue
    seen[k]=p
    for d in p[3]:
        r=resolve(d)
        if r: stack.append(r)
local=[k for k,p in seen.items() if p[2] is None]
reg=[k for k,p in seen.items() if p[2] is not None]
print("total packages in closure:",len(seen),"path(local):",len(local),"registry:",len(reg))
print("local:",sorted(local))
fw=[k for k in seen if k[0].startswith('semio')]
print("semio* in closure:",sorted(fw))
