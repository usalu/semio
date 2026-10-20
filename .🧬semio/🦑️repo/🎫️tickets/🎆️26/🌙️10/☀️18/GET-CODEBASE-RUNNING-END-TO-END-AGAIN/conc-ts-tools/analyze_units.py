#!/usr/bin/env python3
"""Approximate unit/package import graph from the working-tree file list (regex imports, own resolver). Scratch analysis only."""
import os,re,json,sys,collections
R='/Users/ueli/Documents/semio'
SPLIT=os.environ.get('SPLIT')=='1'
os.chdir(R)
fs=[x for x in open('/private/tmp/conc-ts/files2.z','rb').read().decode().split('\0') if x]
tsf=[f for f in fs if f.endswith(('.ts','.tsx','.mts','.cts')) and '/node_modules/' not in f and '/🗑️generated/' not in f]
fset=set(fs)
pkgdirs={}
for f in fs:
    if f.endswith('package.json') and os.path.basename(f)=='package.json':
        d=os.path.dirname(f)
        try: j=json.load(open(f))
        except: continue
        pkgdirs[d]=j
names={j.get('name'):d for d,j in pkgdirs.items() if j.get('name')}
# unit roots
roots=set()
for d in pkgdirs:
    if '/📦️packages/' not in d+'/': roots.add(d)
for f in tsf:
    parts=f.split('/')
    for i,p in enumerate(parts[:-1]):
        if p=='📦️packages': roots.add('/'.join(parts[:i]))
        if p=='🔌️plugins' and i+1<len(parts)-1: roots.add('/'.join(parts[:i+2]))
roots.add('')
def unit(f):
    d=os.path.dirname(f)
    while d and d not in roots: d=os.path.dirname(d)
    rel=f[len(d)+1:] if d else f
    return d+('#tests' if SPLIT and '/🧪️tests/' in '/'+rel else '')
EXT=['','.ts','.tsx','.d.ts','/🟦️.ts','/index.ts']
def exists(p): return p in fset
def resolve_rel(base,spec):
    p=os.path.normpath(os.path.join(os.path.dirname(base),spec))
    if p.endswith('.js'): c=[p[:-3]+'.ts',p[:-3]+'.tsx']
    else: c=[p+e for e in EXT]
    for x in c:
        if exists(x): return x
    return None
def exports_target(j,sub):
    ex=j.get('exports')
    if ex is None: return None
    if isinstance(ex,str): ex={'.':ex}
    key='.' if sub=='' else './'+sub
    def pick(v):
        if isinstance(v,str): return v
        if isinstance(v,dict):
            for k in ('types','import','default','bun','node','require'):
                if k in v:
                    r=pick(v[k])
                    if r: return r
        if isinstance(v,list):
            for x in v:
                r=pick(x)
                if r: return r
        return None
    if key in ex: return pick(ex[key])
    for k,v in ex.items():
        if '*' in k:
            pre,post=k.split('*',1)
            if key.startswith(pre) and key.endswith(post) and len(key)>=len(pre)+len(post):
                mid=key[len(pre):len(key)-len(post)]
                t=pick(v)
                if t: return t.replace('*',mid)
    return None
def resolve_bare(spec):
    parts=spec.split('/')
    nm='/'.join(parts[:2]) if spec.startswith('@') else parts[0]
    sub='/'.join(parts[2:] if spec.startswith('@') else parts[1:])
    d=names.get(nm)
    if d is None: return None,'ext'
    t=exports_target(pkgdirs[d],sub)
    if t is None: return None,'noexport:'+spec
    p=os.path.normpath(os.path.join(d,t))
    return (p if p in fset else None),('missing:'+p if p not in fset else '')
IMP=re.compile(r'''(?:^|[\s;}])(?:import|export)\s[^'"`;]*?from\s*['"]([^'"]+)['"]|(?:^|[\s;(=,])import\s*['"]([^'"]+)['"]|import\(\s*['"]([^'"]+)['"]\s*\)|require\(\s*['"]([^'"]+)['"]\s*\)''',re.M)
edges=collections.defaultdict(collections.Counter); funit={}; fedges=[]
unresolved=collections.Counter(); nfiles=0; nimp=0
for f in tsf:
    try: src=open(f,encoding='utf8',errors='replace').read()
    except: continue
    nfiles+=1; u=unit(f)
    for m in IMP.finditer(src):
        spec=next(g for g in m.groups() if g)
        nimp+=1
        if spec.startswith('.'):
            t=resolve_rel(f,spec)
            if t is None:
                if not spec.endswith(('.json','.css','.svg','.wasm','.js','.md','.txt','.mjs')): unresolved['rel']+=1
                continue
        elif spec.startswith(('node:','bun:')): continue
        else:
            t,why=resolve_bare(spec)
            if t is None:
                if why.startswith(('noexport','missing')): unresolved[why[:60]]+=1
                continue
        if not t.endswith(('.ts','.tsx','.mts','.cts')): continue
        v=unit(t)
        if v!=u: edges[u][v]+=1
        fedges.append((f,t))
units=collections.Counter(unit(f) for f in tsf)
# SCC
sys.setrecursionlimit(100000)
idx={};low={};on=set();st=[];sccs=[];c=[0]
def sc(v):
    idx[v]=low[v]=c[0];c[0]+=1;st.append(v);on.add(v)
    for w in edges.get(v,()):
        if w not in idx: sc(w);low[v]=min(low[v],low[w])
        elif w in on: low[v]=min(low[v],idx[w])
    if low[v]==idx[v]:
        comp=[]
        while True:
            w=st.pop();on.discard(w);comp.append(w)
            if w==v:break
        sccs.append(comp)
allu=set(units)|set(edges)|{w for v in edges for w in edges[v]}
for v in sorted(allu):
    if v not in idx: sc(v)
sz=sorted((len(s),sum(units[u] for u in s)) for s in sccs)
big=sorted(sccs,key=lambda s:-sum(units[u] for u in s))
print('files',nfiles,'imports',nimp,'units',len(allu),'with ts',len(units),'edges',sum(len(v) for v in edges.values()))
print('SCCs',len(sccs),'nontrivial',sum(1 for s in sccs if len(s)>1),'largest by files:')
for s in big[:6]: print(' size',len(s),'files',sum(units[u] for u in s),'e.g.',sorted(s)[:3])
print('unresolved',unresolved.most_common(8))
json.dump({'units':units,'edges':{k:dict(v) for k,v in edges.items()},'sccs':[sorted(s) for s in sccs]},open('/private/tmp/conc-ts/units.json','w'),ensure_ascii=False)
print('top units',[(u,c) for u,c in units.most_common(10)])

json.dump({'files':tsf,'fedges':fedges},open('/private/tmp/conc-ts/fedges.json','w'),ensure_ascii=False)
