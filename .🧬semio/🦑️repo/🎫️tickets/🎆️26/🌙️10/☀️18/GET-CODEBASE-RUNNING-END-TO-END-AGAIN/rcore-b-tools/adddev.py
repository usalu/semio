import re,subprocess,sys
root=sys.argv[1] if sys.argv[1].endswith(".toml") else "Cargo.toml"
def manifest(name):
    out=subprocess.run(["rg","-l","--glob","Cargo.toml",'^name = "%s"$'%name,"🧰️framework"],capture_output=True,text=True).stdout.split("\n")
    return [o for o in out if o][0]
ws=open(root).read()
def wsdep(dep):
    return bool(re.search(r'^(%s|%s)\s*=.*$'%(re.escape(dep),re.escape(dep.replace("-","_"))),ws,re.M))
for pair in sys.argv[2:]:
    crate,dep=pair.split(":")
    dep=dep.replace("_","-")
    p=manifest(crate); s=open(p).read()
    if re.search(r'^%s\s*='%re.escape(dep),s,re.M): print("have",crate,dep); continue
    import os
    if wsdep(dep):
        key=dep if re.search(r'^%s\s*='%re.escape(dep),ws,re.M) else dep.replace("-","_")
        line='%s = { workspace = true }\n'%key
    else:
        try: q=manifest(dep)
        except Exception: print("NOT-FOUND",crate,dep); continue
        rel=os.path.relpath(os.path.dirname(q),os.path.dirname(p))
        line='%s = { path = "%s" }\n'%(dep,rel)
    if "[dev-dependencies]" in s: s=s.replace("[dev-dependencies]\n","[dev-dependencies]\n"+line,1)
    else: s=s.rstrip("\n")+"\n\n[dev-dependencies]\n"+line
    open(p,"w").write(s); print("added",crate,dep,p[-50:])
