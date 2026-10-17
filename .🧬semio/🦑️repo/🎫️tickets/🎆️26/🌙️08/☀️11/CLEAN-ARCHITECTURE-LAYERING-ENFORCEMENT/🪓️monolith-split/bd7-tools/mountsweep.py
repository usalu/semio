import sys,os,re
sys.path.insert(0,os.path.dirname(__file__))
from mountcheck import check
root="/Users/ueli/Documents/semio"
F=os.path.join(root,".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split")
tomls=[l for l in open(F+"/🗑️generated/bd7-cargo-tomls.txt",encoding="utf-8").read().split("\n") if l and not l.startswith(".🧬semio")]
out=[]
n=0
for t in tomls:
    p=os.path.join(root,t)
    try: txt=open(p,encoding="utf-8").read()
    except Exception: continue
    roots=[]
    base=os.path.dirname(p)
    sec=None; haslib=False
    for ln in txt.split("\n"):
        m=re.match(r"\s*\[\[?([\w.-]+)\]\]?",ln)
        if m: sec=m.group(1); continue
        m=re.match(r'\s*path\s*=\s*"([^"]+)"',ln)
        if m and sec in ("lib","bin","test","bench","example"): roots.append(m.group(1)); haslib|=sec=="lib"
    if not haslib and "[package]" in txt and os.path.exists(os.path.join(base,"src/lib.rs")): roots.append("src/lib.rs")
    for r in roots:
        rp=os.path.normpath(os.path.join(base,r))
        if not os.path.exists(rp):
            out.append((t,"ROOT-MISSING",rp)); continue
        n+=1
        for m in check(rp): out.append((t,)+m)
print("roots",n)
for o in out: print("\t".join(x.replace(root+"/","") for x in o))
