import json,os,sys,collections,pickle
R='/Users/Shared/semio/'
L='🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/'
I=L+'🏗️build/📥️inputs/'
HELP=[I+p for p in ["🟨️.mjs","🔏️revision/🟨️.mjs","🔏️revision/🧬️schema/🔣️.json","📋️data/🟨️.mjs","📋️data/📥️load/🟨️.mjs","📋️data/🧬️schema/🔣️.json","🧮️digest/📥️read/🟨️.mjs","🧮️digest/📥️read/🧬️schema/🔣️.json","🔒️lock/🔣️schema.json","💾️storage/🟨️.mjs","💾️storage/📒️ledger/🟨️.mjs","💾️storage/🧬️schema/🔣️.json","🧮️digest/🟨️.mjs","🧮️digest/🔣️schema.json","🔒️lock/📥️read/🟨️.mjs","🔒️lock/🟨️.mjs"]]+[L+'⚡️caching/💾️storage/🔒️enforcement/📦️budget/🔣️policy.json']
BIT={p:1<<i for i,p in enumerate(HELP)}
OTHER={'bunfig':'bunfig.toml','tsconfig':'tsconfig.json','rustfmt':'rustfmt.toml','shim':L+'🏃️process/🦀️cargo/🏃️execution/📜️script.ts'}
for i,(k,p) in enumerate(OTHER.items()):BIT[p]=1<<(len(HELP)+i)
PROV='@semio-tech/repo-script-providers'
CONFIG_OWNER_NEUTRAL=[I+"💾️storage/🟨️.mjs",I+"💾️storage/📒️ledger/🟨️.mjs",I+"💾️storage/🧬️schema/🔣️.json",L+'⚡️caching/💾️storage/🔒️enforcement/📦️budget/🔣️policy.json']
COMPACT_DROP=CONFIG_OWNER_NEUTRAL+[I+"🟨️.mjs"]
def mask(paths):
    m=0
    for p in paths:m|=BIT[p]
    return m
nx=json.load(open(R+'nx.json'))['namedInputs']
nodes={}
for line in open('nodes.jsonl'):
    d=json.loads(line);nodes[d['name']]=d
leafc={}
def leaf(tok):
    key=tok[21:]
    if key in leafc:return leafc[key]
    p=R+'.cache/b/g/m/build-input-closures/'+key+'.json'
    b=None
    if os.path.exists(p):
        b=0
        for n in json.load(open(p))['files']:b|=BIT.get(n,0)
    leafc[key]=b;return b
def pb(x):
    if x.startswith('{workspaceRoot}/'):x=x[16:]
    return BIT.get(x,0)
def make(compact_mask,prov_mask):
    memo={}
    def resolve(owner,g,stack=()):
        k=(owner,g)
        if k in memo:return memo[k]
        if k in stack:return 0
        d=nodes.get(owner);gs=(d or {}).get('groups') or {}
        lst=gs.get(g)
        if lst is None:lst=nx.get(g)
        if lst is None:memo[k]=0;return 0
        memo[k]=0;b=items(owner,lst,stack+(k,));memo[k]=b;return b
    def items(owner,lst,stack=()):
        b=0;m=prov_mask if owner==PROV else compact_mask
        for x in lst:
            if isinstance(x,str):
                if x.startswith('!'):continue
                if x.startswith('^'):
                    for dep in nodes.get(owner,{}).get('deps',[]):b|=resolve(dep,x[1:],stack)
                elif x.startswith('{') or '/' in x or '*' in x or x.endswith(('.json','.toml')):b|=pb(x)&~m
                else:b|=resolve(owner,x,stack)
            elif isinstance(x,dict):
                if 'env' in x and x['env'].startswith('SEMIO_SOURCE_CLOSURE_'):
                    lb=leaf(x['env'])
                    if lb is not None:b|=lb&~m
                elif 'input' in x:
                    for p in x.get('projects',[owner]):b|=resolve(p,x['input'],stack)
        return b
    return items
def count(items):
    c=collections.Counter();per={}
    for name,d in nodes.items():
        tb=0
        for t,x in d['targets'].items():
            if not x.get('inputs') or x.get('cache')==False:continue
            tb|=items(name,x['inputs'])
        per[name]=tb
        for p,b in BIT.items():
            if tb&b:c[p]+=1
    return c,per
if __name__=='__main__':
    before,pb0=count(make(0,0))
    after,pb1=count(make(mask(COMPACT_DROP),mask(CONFIG_OWNER_NEUTRAL)))
    for p in BIT:print(str(before[p]).rjust(5),str(after[p]).rjust(5),p.replace(L,'LIB/'))
    pickle.dump((pb0,pb1),open('per3.pkl','wb'))
