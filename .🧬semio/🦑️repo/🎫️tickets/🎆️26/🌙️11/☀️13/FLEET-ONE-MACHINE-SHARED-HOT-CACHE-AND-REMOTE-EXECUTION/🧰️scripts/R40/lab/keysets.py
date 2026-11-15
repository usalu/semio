import json,sys,os,pickle,resource,fnmatch,collections,re
R='/Users/Shared/semio/'
C=R+'.cache/b/g/m/build-input-closures/'
I=sys.intern
O={}
for line in open('model.jsonl'):
    d=json.loads(line)
    g={k:tuple(I(x) if isinstance(x,str) else (I(x['t']) if 't' in x else (I(x['input']),tuple(I(p) for p in (x.get('projects') or [])) if x.get('projects') is not None else None)) for x in v) for k,v in d['groups'].items()}
    t={k:tuple(I(x) if isinstance(x,str) else (I(x['t']) if 't' in x else (I(x['input']),tuple(I(p) for p in (x.get('projects') or [])) if x.get('projects') is not None else None)) for x in v) for k,v in d['targets'].items()}
    O[I(d['name'])]={'root':d['root'],'g':g,'t':t,'e':tuple(I(x) for x in d['edges'])}
print('owners',len(O),'maxrss MB',resource.getrusage(resource.RUSAGE_SELF).ru_maxrss/1e6)
memo={}
def res_group(o,g,stack=()):
    k=(o,g)
    if k in memo:return memo[k]
    if k in stack:return (frozenset(),frozenset(),frozenset())
    d=O.get(o)
    if d is None or g not in d['g']:
        memo[k]=(frozenset(),frozenset(),frozenset());return memo[k]
    memo[k]=(frozenset(),frozenset(),frozenset())
    r=items(o,d['g'][g],stack+(k,));memo[k]=r;return r
def items(o,lst,stack=()):
    toks=set();paths=set();globs=set()
    for x in lst:
        if isinstance(x,str):
            if x.startswith('!'):continue
            if x.startswith('^'):
                for dep in O[o]['e']:
                    a,b,c=res_group(dep,x[1:],stack);toks|=a;paths|=b;globs|=c
            elif x in O[o]['g']:
                a,b,c=res_group(o,x,stack);toks|=a;paths|=b;globs|=c
            elif x.startswith('{') or '/' in x or '*' in x:
                y=x.replace('{workspaceRoot}/','').replace('{projectRoot}',O[o]['root'])
                if any(ch in y for ch in '*?[{'):globs.add(y)
                else:paths.add(y)
        elif isinstance(x,tuple):
            inp,projs=x
            for p in (projs if projs is not None else (o,)):
                a,b,c=res_group(p,inp,stack);toks|=a;paths|=b;globs|=c
        else:
            toks.add(x)
    return (frozenset(toks),frozenset(paths),frozenset(globs))
# note: token entries are bare str (interned) in both cases -> distinguish: token keys are 64 hex
TC={}
def tclosure(o):
    if o in TC:return TC[o]
    seen=set();st=list(O[o]['e'] if o in O else ())
    while st:
        x=st.pop()
        if x in seen or x not in O:continue
        seen.add(x);st+=list(O[x]['e'])
    TC[o]=seen;return seen
HEX=re.compile(r'^[0-9a-f]{64}$')
def items2(o,lst,stack=()):
    toks=set();paths=set();globs=set()
    for x in lst:
        if isinstance(x,str):
            if HEX.match(x):toks.add(x);continue
            if x.startswith('!'):continue
            if x.startswith('^'):
                for dep in tclosure(o):
                    a,b,c=res_group(dep,x[1:],stack);toks|=a;paths|=b;globs|=c
            elif x in O[o]['g']:
                a,b,c=res_group(o,x,stack);toks|=a;paths|=b;globs|=c
            elif x.startswith('{') or '/' in x or '*' in x:
                y=x.replace('{workspaceRoot}/','').replace('{projectRoot}',O[o]['root'])
                if any(ch in y for ch in '*?[{'):globs.add(y)
                else:paths.add(y)
        else:
            inp,projs=x
            for p in (projs if projs is not None else (o,)):
                a,b,c=res_group(p,inp,stack);toks|=a;paths|=b;globs|=c
    return (frozenset(toks),frozenset(paths),frozenset(globs))
items=items2
keys={}
for o,d in O.items():
    for t,lst in d['t'].items():
        keys[(o,t)]=items(o,lst)
print('keys',len(keys),'maxrss MB',resource.getrusage(resource.RUSAGE_SELF).ru_maxrss/1e6)
toks=set()
for a,b,c in keys.values():toks|=a
print('tokens reached',len(toks))
tf={}
miss=0
for k in toks:
    p=C+k+'.json'
    if os.path.exists(p):tf[k]=tuple(I(f) for f in json.load(open(p))['files'])
    else:miss+=1
print('token files loaded',len(tf),'missing',miss,'maxrss MB',resource.getrusage(resource.RUSAGE_SELF).ru_maxrss/1e6)
pickle.dump((O and {o:{'root':d['root'],'e':d['e']} for o,d in O.items()},keys,tf),open('keys.pkl','wb'))

ns_tok={o:set(res_group(o,'nativeSources')[0]) for o in O}
chk_dep={};chk_own={}
for o,d in O.items():
    lst=d['t'].get('check')
    if lst is None:continue
    deps=set();rest=[]
    for x in lst:
        if isinstance(x,tuple) and x[0]=='nativeSources' and x[1] is not None:deps|=set(x[1])
        else:rest.append(x)
    chk_dep[o]=deps;chk_own[o]=items(o,tuple(rest))
pickle.dump((ns_tok,chk_dep,chk_own),open('keys2.pkl','wb'))
print('keys2 saved',len(chk_dep))
