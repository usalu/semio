"""👥 Scoped census of the direct dependents of one hub: per dependent, the public paths it names (prod vs test roots), resolved to the hub's defining file. `python3 census.py <hub> [--ident semio_framework_x]`. Writes 🗑️generated/gran-hubs/census-<hub>.json. Reads only the hub's dependents' module trees (manifest graph), no repo-wide scan."""
from hubenv import *
import re,time,pickle
from rs import *
from cratemodel import Model,World
hub=sys.argv[1]
HD=hubdir(hub) if hub!='umbrella' else byname['semio-framework']
hname=C[HD]['name']
def target_roots(d):
    t=open(os.path.join(ROOT,d,'Cargo.toml'),encoding='utf-8').read()
    prod=[];test=[]
    for m in re.finditer(r'^\[\[(bin|test|example|bench)\]\](.*?)(?=^\[|\Z)',t,re.S|re.M):
        pm=re.search(r'^\s*path\s*=\s*"([^"]+)"',m.group(2),re.M)
        if pm: ((prod if m.group(1)=='bin' else test)).append(os.path.normpath(os.path.join(ROOT,d,pm.group(1))))
    bm=re.search(r'^\s*build\s*=\s*"([^"]+)"',t,re.M)
    if bm: prod.append(os.path.normpath(os.path.join(ROOT,d,bm.group(1))))
    for sub in ('src/main.rs','build.rs'):
        p=os.path.join(ROOT,d,sub)
        if os.path.exists(p): prod.append(p)
    return prod,test
def parse_tree(s,i):
    n=len(s)
    def ws(i):
        while i<n and s[i].isspace(): i+=1
        return i
    i=ws(i)
    if i<n and s[i]=='{':
        out=[];i+=1
        while i<n:
            i=ws(i)
            if s[i]=='}': i+=1;break
            r,i=parse_tree(s,i);out+=r
            i=ws(i)
            if i<n and s[i]==',': i+=1
        return out,i
    if i<n and s[i]=='*': return [('*',)],i+1
    m=re.compile(IDENT).match(s,i)
    if not m: return [()],i
    seg=m.group(0);j=ws(m.end())
    if s.startswith('::',j):
        sub,k=parse_tree(s,j+2)
        return [(seg,)+x for x in sub],k
    am=re.compile(r'as\s+('+IDENT+')').match(s,j)
    if am: j=ws(am.end())
    return [(seg,)],j
def refs(sk,idents):
    """all paths after `ident::` for any ident; also `ident` alone (crate used as value/alias)."""
    out=[];aliases=set()
    for I in idents:
        for m in re.finditer(r'(?<![A-Za-z0-9_])'+re.escape(I)+r'(?![A-Za-z0-9_])',sk):
            e=m.end();k=e
            while k<len(sk) and sk[k].isspace(): k+=1
            if sk.startswith('::',k):
                r,_=parse_tree(sk,k+2);out+=r
            else:
                am=re.match(r'\s+as\s+('+IDENT+')',sk[e:e+80])
                if am: aliases.add(am.group(1))
                out.append(())
    return out,aliases
MAC=json.load(open(OUT+'/macro-emit.json')).get(hub,[]) if os.path.exists(OUT+'/macro-emit.json') else []
MACN=sorted({m for _,m,_ in MAC if m!='<proc-macro>'})
MACRE=re.compile(r'(?<![A-Za-z0-9_])('+'|'.join(map(re.escape,MACN))+r')\s*!') if MACN else None
deps=sorted(rev[HD])
world,I2L,I2D,LIB=make_world()
HID=LIB[HD]
model=world.model_for(HID)
res={}
t0=time.time()
for n,d in enumerate(deps):
    c=C[d]
    idents=set(dn.replace('-','_') for k in('normal','build') for dn,t in c['deps'][k] if t==HD)
    try: lib=module_tree(libfile(d,c))
    except Exception: lib=[]
    pr,te=target_roots(d)
    pf=set(lib)
    for r in pr: pf|=set(module_tree(r))
    tf=set()
    for r in te: tf|=set(module_tree(r))
    prodp=[];testp=[];mh=set()
    texts={}
    for f in sorted(pf|tf):
        try: texts[f]=open(f,encoding='utf-8',errors='ignore').read()
        except Exception: pass
    aliases=set();sks={}
    for f,t in texts.items():
        if MACRE:
            for mm in MACRE.finditer(t): mh.add(mm.group(1))
        if any(I in t for I in idents):
            sks[f]=skeleton(t)
            for I in idents:
                for am in re.finditer(r'(?<![A-Za-z0-9_])'+re.escape(I)+r'(?![A-Za-z0-9_])\s+as\s+('+IDENT+')',sks[f]): aliases.add(am.group(1))
    aliases.discard('_')
    names=set(idents)|aliases
    for f,t in texts.items():
        if f not in sks:
            if not aliases or not any(a in t for a in aliases): continue
            sks[f]=skeleton(t)
        r,_=refs(sks[f],names)
        ist=('🧪️' in f) or (f in tf and f not in pf)
        (testp if ist else prodp).extend(r)
    def resolve(p):
        p=[x for x in p]
        while p and p[0]=='self': p=p[1:]
        q=list(p)
        while q:
            r=model.resolve_in(model.root,q,True)
            if r:
                if r[0]=='def': return ('def',os.path.relpath(r[1].file,ROOT),r[2]['name'],r[2]['kw'],model.lines(r[1],r[2]))
                if r[0]=='mod': return ('mod',os.path.relpath(r[1].file,ROOT),q[-1],'mod',0)
                return ('ext',list(r[1]),'','',0)
            q.pop()
        return ('crate','','','',0)
    def summarize(ps):
        cnt=collections.Counter(tuple(p[:4]) for p in ps)
        return {'/'.join(p):v for p,v in cnt.items()}
    res[d]={'name':c['name'],'prod':summarize(prodp),'test':summarize(testp),'idents':sorted(idents),'macros':sorted(mh)}
    if n%300==0: print(hub,n,len(deps),round(time.time()-t0),flush=True)
# resolve distinct paths once
distinct=set()
for d,v in res.items():
    for k in ('prod','test'): distinct|=set(v[k])
RES={}
for p in distinct:
    q=[x for x in p.split('/') if x]
    while q and q[0]=='self': q=q[1:]
    r=None;qq=list(q)
    while qq:
        r=world.resolve(model,qq)
        if r: break
        qq.pop()
    RES[p]=r or {'kind':'unresolved'}
json.dump({'hub':hname,'dir':HD,'ident':HID,'deps':res,'resolved':RES},open(OUT+'/census-%s.json'%hub,'w'),ensure_ascii=False)
print('done',hub,len(deps),round(time.time()-t0))
