import sys,collections,re
lock=sys.argv[1]; root=sys.argv[2]; target=sys.argv[3] if len(sys.argv)>3 else None
def load(f):
    pk=[];cur=None;indeps=False
    for l in open(f,encoding='utf8'):
        l=l.rstrip()
        if l=='[[package]]': cur={'dependencies':[]};pk.append(cur);indeps=False;continue
        if cur is None: continue
        m=re.match(r'(name|version|source) = "(.*)"',l)
        if m: cur[m.group(1)]=m.group(2);continue
        if l.startswith('dependencies = ['): indeps=True;continue
        if indeps:
            if l==']': indeps=False
            else: cur['dependencies'].append(l.strip().strip(',').strip('"'))
    return {'package':pk}
d=load(lock)
pk={}
byname=collections.defaultdict(list)
for p in d['package']:
    k=(p['name'],p['version']); pk[k]=p; byname[p['name']].append(k)
def res(dep):
    n=dep.split(' ')
    if len(n)==1: return byname[n[0]][0] if n[0] in byname else None
    return (n[0],n[1])
rk=byname[root][0]
seen={rk:None}; q=[rk]
while q:
    k=q.pop(0)
    for dd in pk[k].get('dependencies',[]):
        r=res(dd)
        if r and r in pk and r not in seen: seen[r]=k; q.append(r)
print(len(seen),'crates from',root, 'ws:',sum(1 for k in seen if 'source' not in pk[k]))
if target:
    for k in seen:
        if target in k[0]:
            path=[];x=k
            while x: path.append(x[0]); x=seen[x]
            print(' <- '.join(path))
