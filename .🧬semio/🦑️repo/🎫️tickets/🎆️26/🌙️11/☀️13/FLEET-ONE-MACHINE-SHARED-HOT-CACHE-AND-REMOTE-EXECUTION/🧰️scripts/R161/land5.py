import sys,os,json,hashlib
ROOT=os.environ.get("R161_ROOT","/Users/ueli/Documents/semio")
HERE=os.path.dirname(os.path.abspath(__file__))
LA=ROOT+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher"
PK=LA+"/📦️packages/🐹️go"
FILES={"fleet":PK+"/🛰️fleet-store/🐹️.go","prov":PK+"/📥️runtime-provision/🐹️.go"}
NEW={LA+"/📦️packages/🐹️go/🏠️machine-state/🐹️.go":HERE+"/go-stage/machine-state.go",LA+"/🧪️tests/🏠️machine-state/🐹️.go":HERE+"/go-stage/machine-state-test.go",ROOT+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🧪️tests/🔎️installed-watcher/🟦️.ts":HERE+"/go-stage/installed-watcher.ts.txt"}
def read(p): return open(p,encoding="utf8").read()
def write(p,t): open(p,"w",encoding="utf8").write(t)
def sha(t): return hashlib.sha256(t.encode()).hexdigest()
def one(text,old,new):
    assert text.count(old)==1,("anchor count",text.count(old),old[:90])
    return text.replace(old,new)
def fleet(t): return one(t,'filepath.Join(root, ".cache", "f", "machine.json")','MachinePath(env, root, ".cache", "f", "machine.json")')
def prov(t): return one(t,'tools = filepath.Join(append(append([]string{q.Root}, cache.ToolsPartition...), q.Name, row.Sha256)...)','tools = MachinePath(q.Env, q.Root, append(append([]string{}, cache.ToolsPartition...), q.Name, row.Sha256)...)')
PATCH={"fleet":fleet,"prov":prov}
cmd,backup=sys.argv[1],sys.argv[2]
if cmd=="apply":
    os.makedirs(backup,exist_ok=True);manifest={"new":[]};new={}
    for k,p in FILES.items(): new[k]=PATCH[k](read(p))
    for p in NEW:
        assert not os.path.exists(p),("exists",p)
    for i,(k,p) in enumerate(FILES.items()):
        old=read(p);write(backup+"/%d.bak"%i,old);manifest[k]={"i":i,"before":sha(old),"after":sha(new[k])}
    for k,p in FILES.items(): write(p,new[k])
    for p,src in NEW.items():
        os.makedirs(os.path.dirname(p),exist_ok=True);write(p,read(src));manifest["new"].append([p,sha(read(src))])
    json.dump(manifest,open(backup+"/manifest.json","w"));print("applied")
elif cmd=="restore":
    manifest=json.load(open(backup+"/manifest.json"));bad=[]
    for k,p in FILES.items():
        m=manifest[k];now=sha(read(p))
        if now==m["after"]: write(p,read(backup+"/%d.bak"%m["i"]))
        elif now!=m["before"]: bad.append(k)
    for p,h in manifest["new"]:
        if os.path.exists(p):
            if sha(read(p))==h: os.remove(p);
            else: bad.append(p)
            try: os.rmdir(os.path.dirname(p))
            except OSError: pass
    print("CONFLICT" if bad else "restored",bad)
    sys.exit(4 if bad else 0)
