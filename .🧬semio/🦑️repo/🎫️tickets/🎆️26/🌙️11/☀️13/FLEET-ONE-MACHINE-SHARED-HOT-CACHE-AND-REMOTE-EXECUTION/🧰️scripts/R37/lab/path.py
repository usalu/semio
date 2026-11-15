import re,os,json,sys,collections
R='/Users/Shared/semio/'
L=R+'🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/'
SPEC=re.compile(r'''(?:\bfrom\s*|\bimport\s*\(\s*|\bimport\s+|\brequire\s*\(\s*)["'](\.{1,2}/[^"']+)["']''')
def cands(p):
    yield p
    for e in('.ts','.mjs','.cjs','.mts'):yield p+e
    if p.endswith('.js'):yield p[:-3]+'.ts'
cache={}
def deps(f):
    if f in cache:return cache[f]
    out=[]
    if re.search(r'\.(ts|mjs|js|cjs|mts|tsx)$',f):
        try:src=open(f,encoding='utf8').read()
        except Exception:src=''
        for m in SPEC.finditer(src):
            p=os.path.normpath(os.path.join(os.path.dirname(f),m.group(1)))
            for c in cands(p):
                if os.path.isfile(c):out.append(c);break
    cache[f]=out;return out
start=L+'⚡️caching/🦀️cargo/📜️script.ts'
targets={'script-inputs':L+'🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs','generations/load':L+'🏗️build/🕸️graph/🕰️generations/🔌️load/🟨️.mjs','memory':L+'🏗️build/🧠️memory/🟨️.mjs','observation':L+'🏗️build/👁️observation/🟨️.mjs','inputs':L+'🏗️build/📥️inputs/🟨️.mjs'}
prev={start:None};q=collections.deque([start])
while q:
    f=q.popleft()
    for d in deps(f):
        if d not in prev:prev[d]=f;q.append(d)
print('closure files',len(prev))
for k,t in targets.items():
    if t not in prev:print(k,'NOT reachable');continue
    p=[];x=t
    while x:p.append(x.replace(L,'LIB/').replace(R,''));x=prev[x]
    print(k,len(p)-1,'hops');print('   '+'\n   '.join(reversed(p)))
