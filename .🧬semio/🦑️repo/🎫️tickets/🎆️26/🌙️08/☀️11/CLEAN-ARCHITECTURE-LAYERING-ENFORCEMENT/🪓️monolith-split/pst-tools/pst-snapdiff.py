import sys, os, json, difflib
# usage: pst-snapdiff.py <label> [substring-of-path] ; prints stats and (with substring) the diff
F=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
root="/Users/ueli/Documents/semio"
snap=os.path.join(root,F,"🗑️generated","pst-snap")
manifest=json.load(open(os.path.join(snap,"manifest.json"),encoding="utf8"))
label=sys.argv[1]; sub=sys.argv[2] if len(sys.argv)>2 else None
add=rem=files=0
for key,m in manifest.items():
    if m["label"]!=label: continue
    old=open(os.path.join(snap,key+os.path.splitext(m["path"])[1]),encoding="utf8").read().splitlines()
    new=open(os.path.join(root,m["path"]),encoding="utf8").read().splitlines()
    d=list(difflib.unified_diff(old,new,lineterm="",n=0))
    a=sum(1 for l in d if l.startswith("+") and not l.startswith("+++")); r=sum(1 for l in d if l.startswith("-") and not l.startswith("---"))
    if a or r: files+=1
    add+=a; rem+=r
    if sub and sub in m["path"]:
        print("\n".join(l[:300] for l in d))
print("label",label,"files changed",files,"added",add,"removed",rem)
