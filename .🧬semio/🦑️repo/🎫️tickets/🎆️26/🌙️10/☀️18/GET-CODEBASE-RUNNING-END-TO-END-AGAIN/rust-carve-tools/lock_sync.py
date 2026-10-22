import sys,os,json,subprocess
sys.path.insert(0,os.path.dirname(__file__))
from lib import R
from locks import *
NEW={"semio-framework-os-store-presence","semio-framework-os-store-replay-operation"}
G=R+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/rust-carve-x/"
apply=sys.argv[1]=="apply"
mans=subprocess.run(["rg","-l","--glob","Cargo.toml","--glob","!node_modules","--glob","!.🧬semio","--glob","!target","semio-framework-os-store-(presence|replay-operation)|os-presence-native-snapshot",R+"/🧰️framework",R+"/Cargo.toml"],capture_output=True,text=True).stdout.split()
mans.append(R+"/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🫧️ephemeral/📦️packages/🦀️rust/Cargo.toml")
ch={}
for m in mans:
    t=open(m,encoding="utf8").read()
    name,ws,normal,dev=parse_manifest(t)
    if not name:continue
    wsdir=os.path.normpath(os.path.join(os.path.dirname(m),ws)) if ws else None
    created=name in NEW
    c={"path":m,"workspace":wsdir,"created":created,"addN":sorted(normal&NEW) if not created else [],"addD":sorted(dev&NEW) if not created else [],"remN":[],"remD":[],"normal":sorted(normal),"dev":sorted(dev)}
    if name=="semio-framework-os-store-ephemeral":c["remN"]=["semio-framework-os-presence-native-snapshot"]
    if c["created"] or c["addN"] or c["addD"] or c["remN"]:ch[name]=c
print(sorted(ch))
json.dump(ch,open(G+"eprep-lock-spec.json","w"),ensure_ascii=False,indent=1)
import shutil
n=0;stale=[]
bk=G+"lock-backup-eprep/"
for lk in locks():
    try:
        if apply:
            rel=os.path.relpath(lk,R);os.makedirs(os.path.dirname(bk+rel),exist_ok=True)
            if not os.path.exists(bk+rel):shutil.copy(lk,bk+rel)
        if apply_to_lock(lk,ch,apply,("semio-framework-os-store-ephemeral","semio-framework-os-store-replay")):n+=1
    except Exception as e:
        if e.args and e.args[0][0]=="dep missing":stale.append(lk)
        else:print("ERR",lk.replace(R+"/",""),e)
print(("applied" if apply else "would change"),n,"stale",len(stale))
open(G+"eprep-stale-locks.txt","w").write("\n".join(stale))
