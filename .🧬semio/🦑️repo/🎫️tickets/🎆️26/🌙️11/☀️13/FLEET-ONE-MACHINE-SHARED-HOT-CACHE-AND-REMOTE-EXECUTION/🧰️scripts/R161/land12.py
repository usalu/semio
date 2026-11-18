import sys,os,json,hashlib
ROOT=os.environ.get("R161_ROOT","/Users/ueli/Documents/semio");MODE=os.environ.get("R161_MODE","live");HERE=os.path.dirname(os.path.abspath(__file__))
LA=ROOT+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher";PK=LA+"/📦️packages/🐹️go"
MEMBER=PK+"/🏠️machine-state/🐹️.go";TEST=LA+"/🧪️tests/🏠️machine-state/🐹️.go"
def read(p): return open(p,encoding="utf8").read()
def write(p,t):
    os.makedirs(os.path.dirname(p),exist_ok=True);open(p,"w",encoding="utf8").write(t)
def sha(t): return hashlib.sha256(t.encode()).hexdigest()
def one(text,old,new):
    assert text.count(old)==1,("anchor count",text.count(old),old[:90])
    return text.replace(old,new)
APPEND=read(HERE+"/go-stage/root-append.txt")
EDITS={"ns":(PK+"/📦️workspace-namespace/🐹️.go",lambda t:one(t,'contract.generationRoot = filepath.Join(root, filepath.FromSlash(directory), mount, manager.GenerationDirectory)','contract.generationRoot = MachinePath(os.Getenv, root, append(strings.Split(directory, "/"), mount, manager.GenerationDirectory)...)')),
"root":(PK+"/🧭️workspace-root/🐹️.go",lambda t:t.rstrip("\n")+"\n"+APPEND)}
if MODE=="full":
    EDITS["fleet"]=(PK+"/🛰️fleet-store/🐹️.go",lambda t:one(t,'filepath.Join(root, ".cache", "f", "machine.json")','MachinePath(env, root, ".cache", "f", "machine.json")'))
    EDITS["prov"]=(PK+"/📥️runtime-provision/🐹️.go",lambda t:one(t,'tools = filepath.Join(append(append([]string{q.Root}, cache.ToolsPartition...), q.Name, row.Sha256)...)','tools = MachinePath(q.Env, q.Root, append(append([]string{}, cache.ToolsPartition...), q.Name, row.Sha256)...)'))
cmd,backup=sys.argv[1],sys.argv[2]
if cmd=="apply":
    os.makedirs(backup,exist_ok=True);manifest={"edits":{}}
    new={k:f(read(p)) for k,(p,f) in EDITS.items()}
    for i,(k,(p,f)) in enumerate(EDITS.items()):
        old=read(p);write(backup+"/%d.bak"%i,old);manifest["edits"][k]={"i":i,"before":sha(old),"after":sha(new[k])}
    removed=None
    if MODE=="live":
        old=read(MEMBER);write(backup+"/member.bak",old);removed=sha(old)
    manifest["removed"]=removed
    testold=read(TEST) if os.path.exists(TEST) else None
    if testold is not None: write(backup+"/test.bak",testold)
    testnew=read(HERE+"/go-stage/machine-state-test.go");manifest["test"]={"before":None if testold is None else sha(testold),"after":sha(testnew)}
    json.dump(manifest,open(backup+"/manifest.json","w"))
    for k,(p,f) in EDITS.items(): write(p,new[k])
    if MODE=="live":
        os.remove(MEMBER);os.rmdir(os.path.dirname(MEMBER))
    write(TEST,testnew);print("applied")
elif cmd=="restore":
    manifest=json.load(open(backup+"/manifest.json"));bad=[]
    for k,(p,f) in EDITS.items():
        m=manifest["edits"][k];now=sha(read(p))
        if now==m["after"]: write(p,read(backup+"/%d.bak"%m["i"]))
        elif now!=m["before"]: bad.append(k)
    if manifest["removed"] is not None: write(MEMBER,read(backup+"/member.bak"))
    t=manifest["test"]
    if os.path.exists(TEST) and sha(read(TEST))==t["after"]:
        if t["before"] is None: os.remove(TEST);os.rmdir(os.path.dirname(TEST))
        else: write(TEST,read(backup+"/test.bak"))
    print("CONFLICT" if bad else "restored",bad);sys.exit(4 if bad else 0)
