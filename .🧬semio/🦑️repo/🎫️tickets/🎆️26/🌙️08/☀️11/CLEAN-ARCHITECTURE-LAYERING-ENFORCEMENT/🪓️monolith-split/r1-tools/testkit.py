import sys,os,re,subprocess,shutil
ROOT="/Users/ueli/Documents/semio"
ORIG=ROOT+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/r1-tools/orig-testkit"
def manifests(art):
    out=subprocess.run(["git","-c","core.quotepath=false","-c","core.fsmonitor=false","ls-files","-co","--exclude-standard","-z","--",art],cwd=ROOT,capture_output=True).stdout.decode().split("\0")
    res={}
    for m in out:
        if m.endswith("/Cargo.toml"):
            t=open(os.path.join(ROOT,m),encoding="utf8").read()
            n=re.search(r'^name = "([^"]+)"',t,re.M)
            if n and "[lib]" in t: res[n.group(1)]=os.path.join(ROOT,m)
    return res
def backup(p):
    b=os.path.join(ORIG,str(abs(hash(p)))+"-"+os.path.basename(os.path.dirname(os.path.dirname(os.path.dirname(p)))))
    if not os.path.exists(b): shutil.copy(p,b)
def edit_owner(path):
    backup(path)
    t=open(path,encoding="utf8").read()
    if "testkit" in t: return
    name=re.search(r'^name = "([^"]+)"',t,re.M).group(1)
    assert "\n[dependencies]" in t and "\n[dev-dependencies]" in t
    t=t.replace("\n[dependencies]","\n[features]\ntestkit = []\n\n[dependencies]",1)
    t=t.replace("\n[dev-dependencies]\n","\n[dev-dependencies]\n"+name+' = { path = ".", features = ["testkit"] }\n',1)
    open(path,"w",encoding="utf8").write(t)
def edit_consumer(path,owner):
    backup(path)
    t=open(path,encoding="utf8").read()
    m=re.search(r'^'+re.escape(owner)+r' = \{ path = "([^"]+)" \}$',t,re.M)
    assert m,(path,owner)
    line=owner+' = { path = "'+m.group(1)+'", features = ["testkit"] }\n'
    if line in t: return
    assert "\n[dev-dependencies]\n" in t
    t=t.replace("\n[dev-dependencies]\n","\n[dev-dependencies]\n"+line,1)
    open(path,"w",encoding="utf8").write(t)
art,spec=sys.argv[1],sys.argv[2:]
mf=manifests(art)
for s in spec:
    kind,rest=s.split(":",1)
    if kind=="owner": edit_owner(mf[rest]); print("owner",rest)
    else:
        c,o=rest.split(">"); edit_consumer(mf[c],o); print("consumer",c,"->",o)
