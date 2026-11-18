import os,json,hashlib
S=os.environ["R143_SCRATCH"];st=json.load(open(S+"/land-state.json"))
cur=open(st["path"],encoding="utf8").read()
if hashlib.sha256(cur.encode()).hexdigest()==st["after"]:
    tmp=st["path"]+".r143tmp";open(tmp,"w",encoding="utf8").write(open(S+"/backup/project-nodes.mjs",encoding="utf8").read());os.replace(tmp,st["path"]);print("restored")
else:print("SKIP: changed by a peer after the landing; reverse the anchored edit by hand")
