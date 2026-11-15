import json,os,sys,collections
R='/Users/Shared/semio/'
L='🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/'
I=L+'🏗️build/📥️inputs/'
FAM={
 'helpers':[I+p for p in ["🟨️.mjs","🔏️revision/🟨️.mjs","🔏️revision/🧬️schema/🔣️.json","📋️data/🟨️.mjs","📋️data/📥️load/🟨️.mjs","📋️data/🧬️schema/🔣️.json","🧮️digest/📥️read/🟨️.mjs","🧮️digest/📥️read/🧬️schema/🔣️.json","🔒️lock/🔣️schema.json","💾️storage/🟨️.mjs","💾️storage/📒️ledger/🟨️.mjs","💾️storage/🧬️schema/🔣️.json","🧮️digest/🟨️.mjs","🧮️digest/🔣️schema.json","🔒️lock/📥️read/🟨️.mjs","🔒️lock/🟨️.mjs"]]+[L+'⚡️caching/💾️storage/🔒️enforcement/📦️budget/🔣️policy.json'],
 'bunfig':['bunfig.toml'],'tsconfig':['tsconfig.json'],'rustfmt':['rustfmt.toml'],'shim':[L+'🏃️process/🦀️cargo/🏃️execution/📜️script.ts'],
 'cargocfg':['.cargo/config.toml'],'rtc':['rust-toolchain.toml'],'gowork':['go.work','go.work.sum'],
}
FNAMES=list(FAM)
BIT={f:1<<i for i,f in enumerate(FNAMES)}
PATHBIT={}
for f,ps in FAM.items():
    for p in ps:PATHBIT[p]=PATHBIT.get(p,0)|BIT[f]
def pathbits(p):
    if p.startswith('{workspaceRoot}/'):p=p[16:]
    return PATHBIT.get(p,0)
leafcache={}
def leaf(tok):
    key=tok[len('SEMIO_SOURCE_CLOSURE_'):]
    if key in leafcache:return leafcache[key]
    p=R+'.cache/b/g/m/build-input-closures/'+key+'.json'
    b=None
    if os.path.exists(p):
        b=0
        for n in json.load(open(p))['files']:b|=PATHBIT.get(n,0)
    leafcache[key]=b
    return b
nx=json.load(open(R+'nx.json'))['namedInputs']
nodes={}
for line in open('nodes.jsonl'):
    d=json.loads(line);nodes[d['name']]=d
memo={}
miss=collections.Counter()
def resolve(owner,g,stack=()):
    k=(owner,g)
    if k in memo:return memo[k]
    if k in stack:return 0
    d=nodes.get(owner)
    groups=(d or {}).get('groups') or {}
    lst=groups.get(g)
    if lst is None:lst=nx.get(g)
    if lst is None:miss[(g)]+=1;memo[k]=0;return 0
    memo[k]=0
    b=item_bits(owner,lst,stack+(k,))
    memo[k]=b;return b
def item_bits(owner,lst,stack=()):
    b=0
    for x in lst:
        if isinstance(x,str):
            if x.startswith('!'):continue
            if x.startswith('^'):
                for dep in nodes.get(owner,{}).get('deps',[]):b|=resolve(dep,x[1:],stack)
            elif x.startswith('{') or '/' in x or '*' in x or x.endswith(('.json','.toml')):b|=pathbits(x)
            else:b|=resolve(owner,x,stack)
        elif isinstance(x,dict):
            if 'env' in x and x['env'].startswith('SEMIO_SOURCE_CLOSURE_'):
                lb=leaf(x['env'])
                if lb is None:miss['leaf']+=1
                else:b|=lb
            elif 'input' in x:
                for p in x.get('projects',[owner]):b|=resolve(p,x['input'],stack)
    return b
def stats():
    own={f:0 for f in FNAMES}   # owners where some cached target key contains fam
    anyt={f:0 for f in FNAMES}  # any target (incl uncached)
    ingroup={f:0 for f in FNAMES} # any group (node embedding)
    rows=[]
    for name,d in nodes.items():
        tb=0;ta=0;gb=0
        for g in (d['groups'] or {}):gb|=resolve(name,g)
        for t,x in d['targets'].items():
            if not x.get('inputs'):continue
            b=item_bits(name,x['inputs'])
            ta|=b
            if x.get('cache')!=False:tb|=b
        rows.append((name,gb,ta,tb))
        for f in FNAMES:
            if gb&BIT[f]:ingroup[f]+=1
            if ta&BIT[f]:anyt[f]+=1
            if tb&BIT[f]:own[f]+=1
    return rows,ingroup,anyt,own
if __name__=='__main__':
    rows,ing,anyt,own=stats()
    print('owners',len(rows),'miss',dict(miss.most_common(8)))
    for f in FNAMES:print(f.ljust(9),'in some group',ing[f],'| in some target key',anyt[f],'| in a cached target key',own[f])
    import pickle;pickle.dump(rows,open('rows.pkl','wb'))
