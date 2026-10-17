import sys, os, json, hashlib, shutil
# usage: pst-snap.py <label> <file-with-repo-relative-paths>
F=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
root="/Users/ueli/Documents/semio"
label=sys.argv[1]
snap=os.path.join(root,F,"🗑️generated","pst-snap")
os.makedirs(snap,exist_ok=True)
manifest_path=os.path.join(snap,"manifest.json")
manifest=json.load(open(manifest_path,encoding="utf8")) if os.path.exists(manifest_path) else {}
n=0
for line in open(sys.argv[2],encoding="utf8"):
    rel=line.strip()
    if not rel: continue
    src=os.path.join(root,rel)
    if not os.path.exists(src): continue
    key=hashlib.sha1((label+"|"+rel).encode()).hexdigest()[:14]
    dst=os.path.join(snap,key+os.path.splitext(rel)[1])
    if key in manifest: continue
    shutil.copy2(src,dst)
    manifest[key]={"label":label,"path":rel}
    n+=1
json.dump(manifest,open(manifest_path,"w",encoding="utf8"),ensure_ascii=False,indent=0)
print("snapshotted",n)
