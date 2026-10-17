import sys,os,re,subprocess
art=sys.argv[1]               # artifact dir (repo-relative)
needle_def=sys.argv[2]        # regex of a cfg(test)-gated module name defined in some crate, e.g. unit_tests
roots=subprocess.run(["git","-c","core.fsmonitor=false","ls-files","--others","--exclude-standard","--","art"] if False else ["git","-c","core.fsmonitor=false","ls-files","--cached","--others","--exclude-standard","--",art],capture_output=True,text=True,cwd="/Users/ueli/Documents/semio").stdout.split("\n")
roots=[r for r in roots if r.endswith("packages/🦀️rust/🦀️.rs")]
def files_of(root):
    seen=set(); stack=[root]
    while stack:
        f=stack.pop()
        if f in seen or not os.path.exists("/Users/ueli/Documents/semio/"+f): continue
        seen.add(f)
        t=open("/Users/ueli/Documents/semio/"+f,encoding='utf8').read()
        for m in re.finditer(r'#\[path\s*=\s*"([^"]+)"\]',t):
            p=m.group(1)
            if p==".": continue
            g=os.path.normpath(os.path.join(os.path.dirname(f),p))
            if g.endswith(".rs"): stack.append(g)
    return seen
crates={}
for r in roots:
    d=os.path.dirname(r)
    name=re.search(r'name\s*=\s*"([^"]+)"',open("/Users/ueli/Documents/semio/"+d+"/Cargo.toml",encoding='utf8').read()).group(1)
    crates[name]=files_of(r)
# gated modules: files declared under #[cfg(test)] (path attr) or `mod X` marked cfg(test) -> list of (crate, module name)
gated={}
for n,fs in crates.items():
    for f in fs:
        t=open("/Users/ueli/Documents/semio/"+f,encoding='utf8').read()
        for m in re.finditer(r'#\[cfg\(test\)\]\s*(?:#\[[^\]]*\]\s*)*(?:pub(?:\([^)]*\))?\s+)?mod\s+([A-Za-z0-9_]+)',t):
            gated.setdefault(m.group(1),set()).add(n)
# usage: files containing '<gated>::' referenced from crates that do not own the gated module
bad=[]
for g,owners in gated.items():
    if g in ("tests","test","hoisted_tests","schema","mutations","io"): continue
    for n,fs in crates.items():
        if n in owners: continue
        for f in fs:
            t=open("/Users/ueli/Documents/semio/"+f,encoding='utf8').read()
            if re.search(r'\b'+re.escape(g)+r'::',t) and not re.search(r'(pub )?mod '+re.escape(g),t):
                bad.append((n,g,f.split("any/")[-1][:90],sorted(o.replace("semio-s-artifact-","") for o in owners)))
for b in sorted(set((x[0],x[1],tuple(x[3])) for x in bad)): print(b)
print(len(bad))
