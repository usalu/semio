import json,os,re,sys
R='/Users/ueli/Documents/semio/'
sc='🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🎬️scopes/'
d=json.load(open(R+sc+'🔣️.json'));row=[r for r in d['scopes'] if r['id']=='executor-command'][0]
corpus=json.load(open(R+sc+'🧪️tests/🔣️.json'))
entry=os.path.normpath(os.path.join(R+sc+'🧪️tests',corpus['entryFromTests']))
print('entry',entry[len(R):])
imp=re.compile(r'''(?:from|import)\s*\(?\s*["'](\.{1,2}/[^"']+)["']|["'](\.{1,2}/[^"'\\\r\n]+\.json)["']''')
par={};pend=[entry]+[R+x for x in row['roots']];seen=set()
for p in pend: par[p]=None
while pend:
    p=pend.pop()
    if p in seen or not os.path.isfile(p): continue
    seen.add(p)
    if p.endswith('.json'): continue
    s=open(p,encoding='utf8').read()
    for m in imp.finditer(s):
        t=os.path.normpath(os.path.join(os.path.dirname(p),m.group(1) or m.group(2)))
        if t not in par: par[t]=p
        pend.append(t)
tgt=R+sys.argv[1]
chain=[];p=tgt
while p: chain.append(p[len(R):]);p=par.get(p)
print('\n'.join(chain))
