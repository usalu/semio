import json,os,collections,pickle,sys
R='/Users/Shared/semio/'
ch={}
for line in open('changed14.tsv',encoding='utf8'):
    n,p=line.rstrip('\n').split('\t',1);ch[p.strip('"')]=int(n)
# git quotes non-ascii paths? check
sample=list(ch)[:2];print(sample)
nodes={}
for line in open('nodes.jsonl'):
    d=json.loads(line);nodes[d['name']]=d
nx=json.load(open(R+'nx.json'))['namedInputs']
leafc={}
def leaffiles(tok):
    key=tok[21:]
    if key in leafc:return leafc[key]
    p=R+'.cache/b/g/m/build-input-closures/'+key+'.json'
    fs=json.load(open(p))['files'] if os.path.exists(p) else None
    leafc[key]=fs;return fs
# file index over changed files only
idx={}
def bit(f):
    if f not in ch:return 0
    if f not in idx:idx[f]=len(idx)
    return 1<<idx[f]
leafbits={}
import json as _j
_tbl=_j.load(open(R+'🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🗓️scheduling/🔣️.json'))
_nk=[k for d in _tbl['domains'] for k in d.get('nativeKeyed',[])]
_pref=[p for d in _tbl['domains'] for p in d['prefixes'] if not any(k.startswith(p) or p.startswith(k) for k in _nk)]
WALK=('commandSources','testCommandSources','scriptProvider:','toolClosure:')
REFRESHED=os.environ.get('REFRESHED')=='1'
AFTER=os.environ.get('AFTER')=='1'
_I='🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/'
_L='🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/'
DROP4={_I+'💾️storage/🟨️.mjs',_I+'💾️storage/📒️ledger/🟨️.mjs',_I+'💾️storage/🧬️schema/🔣️.json',_L+'⚡️caching/💾️storage/🔒️enforcement/📦️budget/🔣️policy.json'}
DROP5=DROP4|{_I+'🟨️.mjs'}
PROV='@semio-tech/repo-script-providers'
def drop_for(owner):
    if not AFTER:return set()
    return DROP4 if owner==PROV else DROP5
def lb(tok):
    key=tok[21:]
    if key in leafbits:return leafbits[key]
    fs=leaffiles(tok);b=0
    if fs:
        for f in fs:b|=bit(f)
    leafbits[key]=b;return b
memo={}
def resolve(owner,g,stack=()):
    k=(owner,g)
    if k in memo:return memo[k]
    if k in stack:return 0
    d=nodes.get(owner);gs=(d or {}).get('groups') or {}
    lst=gs.get(g)
    if lst is None:lst=nx.get(g)
    if lst is None:memo[k]=0;return 0
    memo[k]=0;b=items(owner,lst,stack+(k,),g.startswith(WALK));memo[k]=b;return b
def items(owner,lst,stack=(),walk=False):
    b=0
    for x in lst:
        if isinstance(x,str):
            if x.startswith('!'):continue
            if x.startswith('^'):
                for dep in nodes.get(owner,{}).get('deps',[]):b|=resolve(dep,x[1:],stack)
            elif x.startswith('{') or '/' in x or '*' in x or x.endswith(('.json','.toml')):
                y=x[16:] if x.startswith('{workspaceRoot}/') else x
                if y in drop_for(owner):continue
                if not any(c in y for c in '*?{[') and '{projectRoot}' not in y:b|=bit(y)
            else:b|=resolve(owner,x,stack)
        elif isinstance(x,dict):
            if 'env' in x and x['env'].startswith('SEMIO_SOURCE_CLOSURE_'):b|=leafbits_for(x['env'],owner,walk)
            elif 'input' in x:
                for p in x.get('projects',[owner]):b|=resolve(p,x['input'],stack)
    return b
leafw={}
def lbw(tok):
    key=tok[21:]
    if key in leafw:return leafw[key]
    fs=leaffiles(tok);b=0
    if fs:
        for f in fs:
            if not any(f.startswith(p) for p in _pref):b|=bit(f)
    leafw[key]=b;return b
_lcache={}
def leafbits_for(tok,owner,walk):
    drop=drop_for(owner);key=(tok,frozenset(drop),REFRESHED and walk)
    if key in _lcache:return _lcache[key]
    fs=leaffiles(tok);b=0
    if fs:
        for f in fs:
            if f in drop:continue
            if REFRESHED and walk and any(f.startswith(p) for p in _pref):continue
            b|=bit(f)
    _lcache[key]=b;return b
blast=collections.Counter()
per={}
for name,d in nodes.items():
    tb=0
    for t,x in d['targets'].items():
        if not x.get('inputs') or x.get('cache')==False:continue
        tb|=items(name,x['inputs'])
    per[name]=tb
    while tb:
        low=tb&-tb;blast[low.bit_length()-1]+=1;tb^=low
inv={v:k for k,v in idx.items()}
rows=[(inv[i],n,ch[inv[i]]) for i,n in blast.items()]
rows.sort(key=lambda r:-r[1]*r[2])
print('key files touched in 14d',len(idx),'with blast>0',len(rows))
tot=sum(n*c for _,n,c in rows);print('sum blast*touches',tot)
bk=collections.Counter()
for f,n,c in rows:
    bk['>=4000' if n>=4000 else '>=1000' if n>=1000 else '>=100' if n>=100 else '<100']+=n*c
print(dict(bk))
L='🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/'
for f,n,c in rows[:40]:print(str(n).rjust(5),str(c).rjust(3),f.replace(L,'LIB/'))
pickle.dump((rows,),open('blast-%s%s.pkl'%('refreshed' if REFRESHED else 'stored','-after' if AFTER else ''),'wb'))
