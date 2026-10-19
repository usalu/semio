import os,re,sys,subprocess,collections
R="/Users/ueli/Documents/semio"
FEM=R+"/✏️s/🔌️plugins/🏗️fem"
tomls=[]
for dp,dn,fn in os.walk(FEM):
    dn[:]=[d for d in dn if d not in("node_modules","target",".🧬semio")]
    if "Cargo.toml" in fn: tomls.append(dp)
pathre=re.compile(r'path\s*=\s*"([^"]+\.rs)"')
crate_of={}
for d in tomls:
    t=open(d+"/Cargo.toml",encoding="utf-8").read()
    for p in pathre.findall(t):
        crate_of.setdefault(os.path.normpath(os.path.join(d,p)),d)
print("crates",len(tomls),"mounted files",len(crate_of))
lit=re.compile(r'(fixture!|include_str!|include_bytes!)\(\s*"([^"]*🧫️fixtures[^"]*)"')
miss=[];n=0;nocrate=[]
for dp,dn,fn in os.walk(FEM):
    dn[:]=[d for d in dn if d not in("node_modules","target",".🧬semio")]
    for f in fn:
        if not f.endswith(".rs"): continue
        fa=dp+"/"+f
        txt=open(fa,encoding="utf-8",errors="ignore").read()
        for m in lit.finditer(txt):
            kind,s=m.groups()
            if kind=="fixture!":
                c=crate_of.get(fa)
                if not c: nocrate.append(fa); continue
                base=c
            else: base=dp
            n+=1
            if not os.path.exists(os.path.normpath(os.path.join(base,s))): miss.append((fa[len(FEM):],kind,s))
print("checked",n,"missing",len(miss),"fixture! without crate",len(set(nocrate)))
for x in miss[:40]: print(x)
for x in sorted(set(nocrate))[:10]: print("NOCRATE",x[len(FEM):])
print("---- nocrate resolution")
bad=[];by=collections.Counter()
for fa in sorted(set(nocrate)):
    txt=open(fa,encoding="utf-8",errors="ignore").read()
    lits=[m.group(2) for m in lit.finditer(txt) if m.group(1)=="fixture!"]
    ok=[d for d in tomls if all(os.path.exists(os.path.normpath(os.path.join(d,s))) for s in lits)]
    if not ok: bad.append(fa)
    else: by[ok[0][len(FEM):]]+=1
print("unresolvable",len(bad))
for b in bad[:15]: print("BAD",b[len(FEM):])
print(by.most_common(8))
