import tomllib,os,sys,collections
ROOT='/home/deck/semio'
cache={}
def load(p):
    p=os.path.abspath(p)
    if p not in cache:
        try: cache[p]=tomllib.load(open(p,'rb'))
        except Exception as e: cache[p]=None; print('PARSE',p,e)
    return cache[p]
def ws_root(mp):
    d=load(mp)
    w=(d.get('package') or {}).get('workspace')
    if isinstance(w,str):
        return os.path.normpath(os.path.join(os.path.dirname(mp),w,'Cargo.toml'))
    cur=os.path.dirname(mp)
    while cur!='/':
        c=os.path.join(cur,'Cargo.toml')
        if os.path.exists(c) and c!=mp:
            dd=load(c)
            if dd and 'workspace' in dd: return c
        cur=os.path.dirname(cur)
def resolve(mp,key,spec):
    """returns (manifest path or None, package name, optional, features, kind)"""
    if isinstance(spec,str): return None,key,False,[],None
    base=spec
    if spec.get('workspace'):
        r=ws_root(mp)
        wd=(load(r) or {}).get('workspace',{}).get('dependencies',{}).get(key) if r else None
        if wd is None: return 'MISSING-WS',key,spec.get('optional',False),spec.get('features',[]),None
        if isinstance(wd,str): return None,key,spec.get('optional',False),spec.get('features',[]),None
        path=wd.get('path'); pkg=wd.get('package',key)
        feats=list(wd.get('features',[]))+list(spec.get('features',[]))
        if path:
            return os.path.normpath(os.path.join(os.path.dirname(r),path,'Cargo.toml')),pkg,spec.get('optional',False),feats,None
        return None,pkg,spec.get('optional',False),feats,None
    pkg=spec.get('package',key)
    if 'path' in spec:
        return os.path.normpath(os.path.join(os.path.dirname(mp),spec['path'],'Cargo.toml')),pkg,spec.get('optional',False),spec.get('features',[]),None
    return None,pkg,spec.get('optional',False),spec.get('features',[]),None
def deps_of(d):
    out={}
    for sec in ('dependencies','dev-dependencies','build-dependencies'):
        for k,v in (d.get(sec) or {}).items(): out.setdefault(k,[]).append((sec,v))
    for t in (d.get('target') or {}).values():
        for sec in ('dependencies','dev-dependencies','build-dependencies'):
            for k,v in (t.get(sec) or {}).items(): out.setdefault(k,[]).append((sec,v))
    return out
def check(mp):
    d=load(mp)
    if not d or 'package' not in d: return
    errs=[]
    feats=d.get('features',{})
    deps=deps_of(d)
    optional=set()
    for k,lst in deps.items():
        for sec,v in lst:
            if sec=='dependencies' or True:
                _,_,opt,_,_=resolve(mp,k,v) if not isinstance(v,str) else (None,k,False,[],None)
                if opt and sec!='dev-dependencies': optional.add(k)
    depused=set()
    for f,items in feats.items():
        for it in items:
            if it.startswith('dep:'): depused.add(it[4:])
    def tfeatures(tm,key=None):
        td=load(tm) if tm and tm!='MISSING-WS' else None
        if not td: return None
        fs=set(td.get('features',{}).keys())
        tdeps=deps_of(td)
        used=set()
        for f,items in td.get('features',{}).items():
            for it in items:
                if it.startswith('dep:'): used.add(it[4:])
        for k,lst in tdeps.items():
            if k in used: continue
            for sec,v in lst:
                if sec=='dev-dependencies': continue
                if not isinstance(v,str):
                    _,_,opt,_,_=resolve(tm,k,v)
                    if opt: fs.add(k)
        return fs
    for f,items in feats.items():
        for it in items:
            if it.startswith('dep:'):
                k=it[4:]
                if k not in optional: errs.append(f'feature {f}: {it} not an optional dependency')
            elif '/' in it:
                k,ft=it.split('/',1); k=k.rstrip('?')
                if k not in deps: errs.append(f'feature {f}: {it}: dependency {k} not declared'); continue
                got=False
                for sec,v in deps[k]:
                    if sec=='dev-dependencies': continue
                    got=True
                    tm,pkg,opt,fl,_=resolve(mp,k,v) if not isinstance(v,str) else (None,k,False,[],None)
                    if tm=='MISSING-WS': errs.append(f'feature {f}: {it}: workspace dependency {k} missing'); continue
                    tf=tfeatures(tm)
                    if tf is not None and ft not in tf: errs.append(f'feature {f}: {it}: {pkg} has no feature {ft}')
                if not got: errs.append(f'feature {f}: {it}: {k} is only a dev-dependency')
            else:
                if it not in feats and it not in optional: errs.append(f'feature {f}: {it} unknown feature/dependency')
    for k,lst in deps.items():
        for sec,v in lst:
            if isinstance(v,str): continue
            tm,pkg,opt,fl,_=resolve(mp,k,v)
            if tm=='MISSING-WS': errs.append(f'{sec} {k}: workspace dependency not in workspace root {ws_root(mp)}'); continue
            if tm and not os.path.exists(tm): errs.append(f'{sec} {k}: path manifest {tm[-80:]} missing'); continue
            if tm:
                td=load(tm)
                if td and td.get('package',{}).get('name')!=pkg: errs.append(f'{sec} {k}: manifest names package {td.get("package",{}).get("name")} not {pkg}')
                tf=tfeatures(tm)
                for ft in fl:
                    if tf is not None and ft not in tf: errs.append(f'{sec} {k}: feature {ft} not defined in {pkg}')
    for e in errs: print(f'{mp.replace(ROOT+"/","")}: {e}')
if __name__=='__main__':
    for a in sys.argv[1:]:
        for dp,dn,fn in os.walk(a):
            dn[:]=[x for x in dn if x not in ('node_modules','target','.git')]
            if 'Cargo.toml' in fn: check(os.path.join(dp,'Cargo.toml'))
