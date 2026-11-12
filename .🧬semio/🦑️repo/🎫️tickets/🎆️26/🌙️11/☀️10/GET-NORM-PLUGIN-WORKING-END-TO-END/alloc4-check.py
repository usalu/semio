import tomllib,subprocess,os,sys,collections
G=sys.argv[1]
extra=["semio-framework-3d-testing-support","semio-framework-deflate","semio-framework-io-binary-source","semio-framework-io-sqlite-snapshot-testing-backing","semio-framework-mesh-component-references","semio-framework-trace","semio-ui-scene-mesh","semio-framework-value-resident","semio-framework-artifact-playbook-playbook","semio-os-flow"]
names=set(extra)
for l in open(G+"/🗑️generated/alloc4-owners.txt",encoding="utf-8"):
    p=l.split(" ",2)
    if p[0] in("ok","CONFLICT"):names.add(p[1])
for l in open(G+"/🗑️generated/alloc4-deps-applied.txt",encoding="utf-8"):
    if l.strip():names.add(l.strip())
names-={"semio-framework-value-testing-allocator","semio-framework-trace-memory-testing"}
tomls=[t for t in subprocess.run(["git","-c","core.quotepath=off","ls-files","--cached","--others","--exclude-standard","--","*Cargo.toml"],capture_output=True,text=True).stdout.split("\n") if t and not t.startswith(".🧬semio/")]
by=collections.defaultdict(list)
for t in tomls:
    try:d=tomllib.load(open(t,"rb"))
    except Exception:continue
    if "package" in d and d["package"]["name"] in names:
        w=d["package"].get("workspace")
        if isinstance(w,str):wd=os.path.normpath(os.path.join(os.path.dirname(t),w))
        elif "workspace" in d:wd=os.path.dirname(t) or "."
        else:
            wd=os.path.dirname(t)
            while wd and not (os.path.isfile(os.path.join(wd,"Cargo.toml")) and b"[workspace" in open(os.path.join(wd,"Cargo.toml"),"rb").read()):wd=os.path.dirname(wd)
            wd=wd or "."
        by[wd].append(d["package"]["name"])
for wd,ns in by.items():
    print(wd,len(ns));open(G+"/🗑️generated/alloc4-ws-"+str(abs(hash(wd)))+".txt","w",encoding="utf-8").write(wd+"\n"+"\n".join(sorted(ns)))
