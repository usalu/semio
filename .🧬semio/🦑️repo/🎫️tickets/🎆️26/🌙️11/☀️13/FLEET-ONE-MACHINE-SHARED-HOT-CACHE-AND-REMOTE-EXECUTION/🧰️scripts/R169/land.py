import hashlib,json,os,shutil,sys
S="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R169"
LIVE="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
mode=sys.argv[1];root=sys.argv[2] if len(sys.argv)>2 else LIVE;backup=sys.argv[3] if len(sys.argv)>3 else S+"/backup"
patches=json.load(open(S+"/patches.json",encoding="utf8"))
MIRROR=S+"/mirror/🏗️build"
new=[]
for dp,dn,fn in os.walk(MIRROR+"/🌡️host-capacity/🧹️disk-recovery"):
    for f in fn: new.append(os.path.relpath(os.path.join(dp,f),MIRROR))
sha=lambda b:hashlib.sha256(b).hexdigest()
def apply():
    os.makedirs(backup,exist_ok=True);plan=[]
    for p in patches:
        path=os.path.join(root,p["file"]);raw=open(path,"rb").read();text=raw.decode("utf8");
        for h in p["hunks"]:
            if text.count(h["old"])!=1: sys.exit("ANCHOR "+p["file"]+": "+str(text.count(h["old"])))
            text=text.replace(h["old"],h["new"],1)
        if sha(raw)==p["baseSha"] and sha(text.encode())!=p["newSha"]: sys.exit("RESULT differs from tested file: "+p["file"])
        plan.append((p["file"],path,raw,text.encode()))
    for f in new:
        path=os.path.join(root,f)
        if os.path.exists(path) and open(path,"rb").read()!=open(os.path.join(MIRROR,f),"rb").read(): sys.exit("EXISTS "+f)
    state={"modified":{},"created":[]}
    for f,path,raw,out in plan:
        b=os.path.join(backup,f);os.makedirs(os.path.dirname(b),exist_ok=True);open(b,"wb").write(raw)
        state["modified"][f]={"before":sha(raw),"after":sha(out)}
    for f,path,raw,out in plan: open(path,"wb").write(out)
    for f in new:
        path=os.path.join(root,f)
        if not os.path.exists(path):
            os.makedirs(os.path.dirname(path),exist_ok=True);shutil.copyfile(os.path.join(MIRROR,f),path);state["created"].append(f)
    json.dump(state,open(os.path.join(backup,"state.json"),"w"),indent=1);print("landed",len(plan),"modified",len(state["created"]),"created")
def restore():
    state=json.load(open(os.path.join(backup,"state.json")))
    for f,s in state["modified"].items():
        path=os.path.join(root,f);cur=sha(open(path,"rb").read())
        if cur!=s["after"]: print("WARN changed since landing (peer edit), restoring by reverse hunks:",f);
        b=open(os.path.join(backup,f),"rb").read()
        if cur==s["after"]: open(path,"wb").write(b)
        else:
            text=open(path,encoding="utf8").read()
            for p in patches:
                if p["file"]==f:
                    for h in reversed(p["hunks"]):
                        if text.count(h["new"])!=1: sys.exit("REVERSE ANCHOR "+f)
                        text=text.replace(h["new"],h["old"],1)
            open(path,"w",encoding="utf8").write(text)
    for f in state["created"]:
        path=os.path.join(root,f)
        if os.path.exists(path): os.remove(path)
    d=os.path.join(root,"🌡️host-capacity/🧹️disk-recovery")
    for dp,dn,fn in list(os.walk(d,topdown=False)):
        if not os.listdir(dp): os.rmdir(dp)
    print("restored")
{"apply":apply,"restore":restore}[mode]()
