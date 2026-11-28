import os,sys
R="/Users/ueli/Documents/semio/"
G=R+"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/"
def rd(p): return open(p,encoding="utf8").read()
def wr(p,s):
    t=p+".h9tmp"; open(t,"w",encoding="utf8").write(s); os.replace(t,p)
def sub(s,old,new):
    assert s.count(old)==1,(old[:60],s.count(old)); return s.replace(old,new)
files={}
p=G+"🤝️prefetch/🟦️.ts"; s=rd(p)
s=sub(s,"&&(forced.has(name)||moved&&!input.roots.has(stub.root))),projects=kept","&&(moved||forced.has(name))),projects=kept")
s=sub(s,"also while the environment stays, also under a scanned root: an owner the scan did not infer again is never left behind)","also while the environment stays; a scanned root (`roots`) excuses nothing: an owner the scan did not infer again keeps its edges only by being refreshed or carried)")
files[p]=s
p=G+"🧪️tests/🔗️edge-refresh/🐍️oracle/🐍️.py"; s=rd(p)
s=sub(s,'if stub.get("tag") or name in world["inferred"]:\n            continue\n        if not (name in forced or (moved and stub["root"] not in world["roots"])):\n            continue\n','if stub.get("tag") or name in world["inferred"]:\n            continue\n        if not (moved or name in forced):\n            continue\n')
s=sub(s,"also while the environment stays, also under a scanned root)","also while the environment stays; a scanned root excuses nothing)")
files[p]=s
if "--check" in sys.argv: print("anchors ok",len(files)); sys.exit(0)
for p,s in files.items(): wr(p,s)
print("landed",len(files))
