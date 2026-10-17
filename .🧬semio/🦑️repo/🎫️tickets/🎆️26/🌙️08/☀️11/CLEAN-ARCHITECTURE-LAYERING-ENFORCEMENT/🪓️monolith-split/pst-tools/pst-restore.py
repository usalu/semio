import sys, os, json, shutil
# usage: pst-restore.py <label> <path-substring>...   restores my own pre-edit snapshot of every matching file
F=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
root="/Users/ueli/Documents/semio/"
snap=root+F+"/🗑️generated/pst-snap/"
man=json.load(open(snap+"manifest.json",encoding="utf8"))
label=sys.argv[1]; subs=sys.argv[2:]
n=0
for k,m in man.items():
    if m["label"]!=label: continue
    if not any(s in m["path"] for s in subs): continue
    shutil.copy2(snap+k+os.path.splitext(m["path"])[1], root+m["path"]); n+=1
    print("restored",m["path"].replace("✏️s/🔌️plugins/","").replace("🗿️artifacts/","")[:130])
print(n,"files")
