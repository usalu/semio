import os,sys,json,hashlib,shutil
S=os.environ["R143_SCRATCH"]
P=os.environ.get("R143_TARGET") or "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs"
OLD='let fleetPort;if(!options?.ownerAdmission){try{const records=await import('
NEW='let fleetPort;{try{const records=await import('
s=open(P,encoding="utf8").read()
assert s.count(OLD)==1,"anchor count %d"%s.count(OLD)
os.makedirs(S+"/backup",exist_ok=True)
open(S+"/backup/project-nodes.mjs","w",encoding="utf8").write(s)
t=s.replace(OLD,NEW)
tmp=P+".r143tmp";open(tmp,"w",encoding="utf8").write(t);os.replace(tmp,P)
sha=lambda x:hashlib.sha256(x.encode()).hexdigest()
json.dump({"path":P,"before":sha(s),"after":sha(t)},open(S+"/land-state.json","w"))
print("landed",sha(s)[:12],"->",sha(t)[:12])
