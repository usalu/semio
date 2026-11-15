#!/usr/bin/env python3
import hashlib,os,shutil,subprocess,sys
LIVE="/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree"
S=os.path.dirname(os.path.abspath(__file__))
MINE=S+"/tree/🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree"
BASE={"🔁️replica/🟦️.ts":"replica.ts","🔁️replica/🧪️tests/🛟️boot.ts":"boot.ts","🗜️compaction/🟦️.ts":"compaction.ts","🗜️compaction/🧪️tests/🟦️.ts":"compaction-tests.ts"}
NEW=["🚦️canary/🧬️schema/🔣️.json","🚦️canary/🟦️.ts","🚦️canary/🧪️tests/🟦️.ts","🚦️canary/🧪️tests/🐍️.py","🚦️canary/🧪️tests/🧫️fixtures/🔣️.json","🔁️replica/🧪️tests/🚦️canary.ts"]
sha=lambda p:hashlib.sha256(open(p,"rb").read()).hexdigest()[:16] if os.path.exists(p) else "-"
dry=len(sys.argv)>1 and sys.argv[1]=="dry"
out=[]
merged={}
for rel,base in BASE.items():
    live=LIVE+"/"+rel; mine=MINE+"/"+rel; b=S+"/base/"+base
    tmp=S+"/merge."+base
    shutil.copyfile(live,tmp)
    r=subprocess.run(["git","merge-file",tmp,b,mine])
    if r.returncode!=0: print("CONFLICT",rel,r.returncode); sys.exit(1)
    merged[rel]=tmp
before={rel:sha(LIVE+"/"+rel) for rel in list(BASE)+NEW+["README.md"]}
if dry:
    for rel,tmp in merged.items(): print("merged ok",rel,sha(tmp))
    sys.exit(0)
for rel in NEW:
    dst=LIVE+"/"+rel; os.makedirs(os.path.dirname(dst),exist_ok=True); shutil.copyfile(MINE+"/"+rel,dst)
for rel in ["🗜️compaction/🟦️.ts","🗜️compaction/🧪️tests/🟦️.ts","🔁️replica/🧪️tests/🛟️boot.ts","🔁️replica/🟦️.ts"]:
    shutil.copyfile(merged[rel],LIVE+"/"+rel)
r=subprocess.run(["python3",S+"/readme-edit.py",LIVE+"/README.md"])
if r.returncode!=0: print("README edit failed"); sys.exit(1)
for rel in before: print(f"{before[rel]} -> {sha(LIVE+'/'+rel)}  {rel}")
