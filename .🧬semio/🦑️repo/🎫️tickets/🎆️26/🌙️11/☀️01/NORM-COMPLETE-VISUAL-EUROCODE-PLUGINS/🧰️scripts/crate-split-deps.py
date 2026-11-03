import sys,re,os
root=sys.argv[1]
files=[]
for d,_,fs in os.walk(root):
    if '📦️packages' in d: continue
    for f in fs:
        if f.endswith('.rs'): files.append(os.path.join(d,f))
defs={};src={}
pat=re.compile(r'^(?:pub(?:\([a-z]+\))?\s+)?(?:async\s+)?(?:const\s+)?(?:unsafe\s+)?(?:fn|struct|enum|const|static|type|trait|macro_rules!)\s+([A-Za-z_][A-Za-z0-9_]*)',re.M)
for f in files:
    s=open(f).read();src[f]=s
    defs[f]=set(m.group(1) for m in pat.finditer(s))
owner={}
for f,ds in defs.items():
    for n in ds: owner.setdefault(n,set()).add(f)
rel=lambda f:os.path.relpath(f,root)
for f in sorted(files):
    s=src[f];deps={}
    toks=set(re.findall(r'(?<![.\w])([A-Za-z_][A-Za-z0-9_]*)\s*(?:\(|::|\{|<|,|\)|;|\s)',s))
    for t in toks:
        if t in defs[f] or t not in owner or len(t)<4: continue
        for o in owner[t]:
            if o!=f: deps.setdefault(rel(o),[]).append(t)
    print(rel(f),len(s),len(s.splitlines()))
    for o,t in sorted(deps.items()): print('   ->',o,len(t),sorted(t)[:3])
