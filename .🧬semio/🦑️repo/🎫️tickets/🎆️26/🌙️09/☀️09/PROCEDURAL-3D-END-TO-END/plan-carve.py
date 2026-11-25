import json,os,re,subprocess,sys
R="/Users/ueli/Documents/semio"
os.chdir(R)
SKIP={'node_modules','.git','target','.cache','🎫️tickets','dist'}
def index():
    idx={}
    for dp,dn,fn in os.walk('.'):
        dn[:]=[d for d in dn if d not in SKIP]
        if 'Cargo.toml' in fn:
            p=os.path.join(dp,'Cargo.toml')
            m=re.search(r'^\[package\][^\[]*?^name\s*=\s*"([^"]+)"',open(p,encoding='utf8').read(),re.S|re.M)
            if m: idx[m.group(1)]=p[2:]
    return idx
MOD=re.compile(r'#\[path\s*=\s*"([^"]+)"\]\s*(?:#\[[^\]]*\]\s*)*(?:pub(?:\([a-z]+\))?\s+)?mod\s+(\w+)\s*;')
def leaves(entry,seen=None):
    seen=seen if seen is not None else {}
    if entry in seen or not os.path.exists(entry): return seen
    seen[entry]=[]
    t=open(entry,encoding='utf8').read()
    for rel,name in MOD.findall(t):
        p=os.path.normpath(os.path.join(os.path.dirname(entry),rel))
        if os.path.isdir(p): continue
        seen[entry].append((name,p)); leaves(p,seen)
    return seen
def scc(g):
    idx={};low={};st=[];on=set();out=[];c=[0]
    def f(v):
        idx[v]=low[v]=c[0];c[0]+=1;st.append(v);on.add(v)
        for w in g.get(v,()):
            if w not in idx: f(w);low[v]=min(low[v],low[w])
            elif w in on: low[v]=min(low[v],idx[w])
        if low[v]==idx[v]:
            comp=[]
            while True:
                w=st.pop();on.discard(w);comp.append(w)
                if w==v:break
            out.append(sorted(comp))
    for v in list(g):
        if v not in idx:f(v)
    return out
def plan(pkg,idx):
    mf=idx[pkg];t=open(mf,encoding='utf8').read()
    lib=re.search(r'^\[lib\][^\[]*?^path\s*=\s*"([^"]+)"',t,re.S|re.M)
    entry=os.path.normpath(os.path.join(os.path.dirname(mf),lib.group(1) if lib else 'src/lib.rs'))
    tree=leaves(entry)
    files={f:os.path.getsize(f) for f in tree}
    children={}
    for f,ms in tree.items():
        for n,p in ms: children.setdefault(f,{})[n]=p
    names={}
    for f,ms in children.items():
        for n,p in ms.items(): names[p]=n
    byname={}
    for p,n in names.items(): byname.setdefault(n,[]).append(p)
    edges={}
    for p,n in names.items():
        s=open(p,encoding='utf8').read();deps=set()
        for m in set(re.findall(r'\b(\w+)::',s)):
            for q in byname.get(m,[]):
                if q!=p: deps.add(q)
        edges[p]=sorted(deps)
    comps=scc({p:edges[p] for p in edges})
    locks=[l for l in subprocess.run(['git','ls-files','-z','--','*Cargo.lock'],capture_output=True,text=True).stdout.split('\0') if l and '🎫️tickets' not in l and f'\nname = "{pkg}"\n' in open(l,encoding='utf8').read()]
    ws=[l for l in subprocess.run(['git','ls-files','-z','--','*Cargo.toml'],capture_output=True,text=True).stdout.split('\0') if l and '🎫️tickets' not in l and (f'[workspace.dependencies.{pkg}]' in open(l,encoding='utf8').read() or re.search(r'^%s\s*=\s*\{ path'%re.escape(pkg),open(l,encoding='utf8').read(),re.M))]
    users=[l for l in subprocess.run(['git','ls-files','-z','--','*Cargo.toml'],capture_output=True,text=True).stdout.split('\0') if l and '🎫️tickets' not in l and l!=mf and re.search(r'^\s*[\w-]+\s*=\s*\{[^}]*(workspace = true|package = "%s")'%re.escape(pkg),open(l,encoding='utf8').read(),re.M) and re.search(r'^%s\b'%re.escape(pkg),open(l,encoding='utf8').read(),re.M)]
    owners=json.load(open('📋️build-owners.json'))['owners']
    carve=re.search(r'\[package\.metadata\.semio\.carve\]',t) is not None
    return dict(package=pkg,manifest=mf,entry=entry,carveMetadata=carve,modules=[dict(name=names[p],file=p,bytes=files.get(p,0),dependsOn=[names[q] for q in edges[p]]) for p in sorted(names)],
      sccGroups=[[names[x] for x in c] for c in sorted(comps,key=lambda c:-len(c))],totalBytes=sum(files.values()),
      lockFiles=locks,workspaceManifests=ws,directDependents=users,buildOwnersRow=[o['name'] for o in owners if o['name']==pkg])
if __name__=="__main__":
    idx=index();out=sys.argv[2] if len(sys.argv)>2 else None
    res=plan(sys.argv[1],idx)
    s=json.dumps(res,ensure_ascii=False,indent=1)
    if out: open(out,'w',encoding='utf8').write(s)
    print(json.dumps({k:(v if not isinstance(v,list) else len(v)) for k,v in res.items()},ensure_ascii=False))
