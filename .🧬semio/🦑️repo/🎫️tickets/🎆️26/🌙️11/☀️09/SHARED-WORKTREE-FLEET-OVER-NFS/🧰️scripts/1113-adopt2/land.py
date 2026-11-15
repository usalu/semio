import hashlib,os,sys
S="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/adopt2/"
REL="🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/"
LIVE="/Users/Shared/semio/"+REL
COPY=S+"tree/"+REL
base={}
for line in open(S+"base.sha",encoding="utf8"):
    h,_,f=line.rstrip("\n").partition("  ")
    base[f]=h
sha=lambda p:hashlib.sha256(open(p,"rb").read()).hexdigest()
changed=[]
for root,_,names in os.walk(COPY):
    for n in names:
        p=os.path.join(root,n); f="./"+os.path.relpath(p,COPY)
        if sha(p)!=base.get(f): changed.append(f)
changed.sort()
bad=[f for f in changed if not os.path.exists(LIVE+f) or sha(LIVE+f)!=base[f]]
if bad: print("LIVE FILE MOVED, nothing written:",bad); sys.exit(1)
if "--check" in sys.argv: print("\n".join(changed)); sys.exit(0)
rows=[]
for f in changed:
    data=open(COPY+f,"rb").read(); before=sha(LIVE+f)
    tmp=LIVE+f+".adopt2.tmp"
    open(tmp,"wb").write(data); os.replace(tmp,LIVE+f)
    rows.append((f,before[:16],sha(LIVE+f)[:16]))
for r in rows: print("| %s | %s | %s |"%r)
