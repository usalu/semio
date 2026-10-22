import sys,os,json
sys.path.insert(0,os.path.dirname(__file__))
from lib import R
from locks import *
G=R+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/rust-carve-x/"
tag=sys.argv[1];apply=sys.argv[2]=="apply";donors=tuple(sys.argv[3].split(","))
ba={k:tuple(v) for k,v in json.load(open(G+"manifests-"+tag+".json")).items()}
ch=diff_manifests(ba)
json.dump(ch,open(G+tag+"-lock-spec.json","w"),ensure_ascii=False,indent=1)
print(len(ch),"packages changed:",sorted(ch))
n=0
stale=[]
import shutil
bk=G+"lock-backup-"+tag+"/"
for lk in locks():
    try:
        if apply:
            rel=os.path.relpath(lk,R)
            os.makedirs(os.path.dirname(bk+rel),exist_ok=True)
            if not os.path.exists(bk+rel):shutil.copy(lk,bk+rel)
        if apply_to_lock(lk,ch,apply,donors):n+=1
    except Exception as e:
        if e.args and e.args[0][0]=="dep missing":stale.append(lk)
        else:print("ERR",lk.replace(R+"/",""),e)
print(("applied" if apply else "would change"),n,"locks")
print("stale (skipped, lock lacks a dependency of the carved package):",len(stale))
open(G+tag+"-stale-locks.txt","w").write("\n".join(stale))
