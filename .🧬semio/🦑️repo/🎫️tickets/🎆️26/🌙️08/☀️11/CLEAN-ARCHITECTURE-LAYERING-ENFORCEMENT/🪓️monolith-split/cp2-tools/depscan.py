import csv,re,os,sys
F=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
rows=list(csv.DictReader(open(F+"/🗑️generated/crate-census.csv")))
lib_by_pkg={}
def sections(t):
    cur=None;out={}
    for line in t.split("\n"):
        m=re.match(r'\s*\[([^\]]+)\]\s*$',line)
        if m: cur=m.group(1); out.setdefault(cur,[]); continue
        if cur is not None: out[cur].append(line)
    return out
info={}
for r in rows:
    mp=r['manifest']; d=os.path.dirname(mp)
    t=open(mp,encoding='utf-8').read()
    s=sections(t)
    def keys(sec):
        ks=set()
        for l in s.get(sec,[]):
            m=re.match(r'\s*([A-Za-z0-9_\-]+)\s*(=|\.)',l)
            if m: ks.add(m.group(1))
        return ks
    for l in s.get('dependencies',[]):
        pass
    deps=keys('dependencies'); dev=keys('dev-dependencies'); build=keys('build-dependencies')
    # renamed deps: package = "x"
    lib=None
    for l in s.get('lib',[]):
        m=re.match(r'\s*path\s*=\s*"([^"]+)"',l)
        if m: lib=m.group(1)
        m=re.match(r'\s*name\s*=\s*"([^"]+)"',l)
        if m: info.setdefault(r['name'],{})['libname']=m.group(1)
    info.setdefault(r['name'],{}).update(dict(dir=d,deps=deps,dev=dev,lib=lib or 'src/lib.rs',row=r))
def files(root):
    seen=set();stack=[root]
    while stack:
        f=stack.pop()
        f=os.path.normpath(f)
        if f in seen or not os.path.isfile(f): continue
        seen.add(f)
        try: t=open(f,encoding='utf-8').read()
        except: continue
        base=os.path.dirname(f)
        for m in re.finditer(r'#\[path\s*=\s*"([^"]+)"\s*\]',t):
            stack.append(os.path.join(base,m.group(1)))
        for m in re.finditer(r'(?m)^\s*(?:pub(?:\([^)]*\))?\s+)?mod\s+([a-z0-9_]+)\s*;',t):
            pre=t[max(0,m.start()-200):m.start()]
            if '#[path' in pre.split('\n')[-2] if len(pre.split('\n'))>1 else False: continue
            stack.append(os.path.join(base,m.group(1)+'.rs'));stack.append(os.path.join(base,m.group(1),'mod.rs'))
    return seen
pat=re.compile(r'\b(semio_[a-z0-9_]+)\b')
res=[]
pkgnames={n.replace('-','_'):n for n in info}
for n,i in info.items():
    r=i['row']
    if r['closureCompilable']!='true': continue
    root=os.path.join(i['dir'],i['lib'])
    fs=files(root)
    used={}
    for f in fs:
        isTest='tests' in f.split('/') or f.endswith('tests.rs')
        t=open(f,encoding='utf-8').read()
        t=re.sub(r'(?m)//.*$','',t)
        for m in pat.finditer(t):
            used.setdefault(m.group(1),set()).add((f,isTest))
    own=i.get('libname') or n.replace('-','_')
    for u,locs in used.items():
        if u==own: continue
        if u not in pkgnames: continue
        pkg=pkgnames[u]
        if pkg in i['deps'] or pkg in i['dev']: 
            if pkg not in i['deps'] and any(not t for f,t in locs):
                res.append((n,'DEVONLY-used-in-nontest',pkg,sorted(f for f,t in locs if not t)[0]))
            continue
        res.append((n,'UNDECLARED',pkg,sorted(f for f,t in locs)[0]))
for x in sorted(res): print("\t".join(x))
