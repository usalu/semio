import tomllib,os,sys,collections
root='.';ws=tomllib.load(open('Cargo.toml','rb'))['workspace']
wdeps=ws.get('dependencies',{})
pkgs={}  # name -> (dir, deps names)
def alias_pkg(k,v):
    return v.get('package',k) if isinstance(v,dict) else k
for m in ws['members']:
    c=tomllib.load(open(os.path.join(m,'Cargo.toml'),'rb'));n=c['package']['name'];deps={}
    def add(tbl):
        for k,v in tbl.items():
            deps[alias_pkg(k,v) if not (isinstance(v,dict) and v.get('workspace')) else alias_pkg(k,wdeps.get(k,{}))]=1
    for key in ('dependencies','dev-dependencies','build-dependencies'): add(c.get(key,{}))
    for t in c.get('target',{}).values():
        for key in ('dependencies','dev-dependencies','build-dependencies'): add(t.get(key,{}))
    pkgs[n]=(m,list(deps))
sel=sys.argv[1:]
par={s:None for s in sel};q=collections.deque(sel)
while q:
    x=q.popleft()
    for y in pkgs.get(x,('',[]))[1]:
        if y in pkgs and y not in par: par[y]=x;q.append(y)
print(len(par),'workspace packages reachable')
for p in sorted(par):
    if p.startswith('semio-framework-plugin') or 'wgpu' in p or 'pixels' in p: 
        x=p;ch=[]
        while x:ch.append(x);x=par[x]
        print(' <- '.join(ch));break
if os.path.exists('compiled.txt'):
    comp=set(open('compiled.txt').read().split())
    extra=comp-set(par)
    dependents=collections.defaultdict(set)
    for n in comp:
        for d in pkgs.get(n,('',[]))[1]:
            if d in comp: dependents[d].add(n)
    tops=[n for n in extra if not dependents[n]]
    print('compiled',len(comp),'extra',len(extra),'tops',sorted(tops)[:20])
