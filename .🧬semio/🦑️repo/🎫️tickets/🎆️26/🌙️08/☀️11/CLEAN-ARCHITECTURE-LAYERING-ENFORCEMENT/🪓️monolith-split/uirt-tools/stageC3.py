"""🧱️ Stage C3: writes the primitives lib root with explicit re-exports and normalises single-name use braces."""
import re,sys
sys.path.insert(0,".")
from prim_names import REPO,RT,MODS,top_items
R=f"{REPO}/{RT}"
P=f"{R}/🧱️primitives"
CFG='#[cfg(any(test, feature = "testkit"))]'
for m in MODS+["TREE"]:
    p=f"{R}/♻️retirement/🌲️tree/🦀️.rs" if m=="TREE" else f"{P}/{m}/🦀️.rs"
    t=open(p,encoding="utf-8").read()
    t=re.sub(r"^use ([A-Za-z0-9_:]+)::\{([A-Za-z0-9_]+)\};$",r"use \1::\2;",t,flags=re.M)
    open(p,"w",encoding="utf-8").write(t)
mods=[]
for m in MODS:
    p=f"{P}/{m}/🦀️.rs"; t=open(p,encoding="utf-8").read()
    plain=[];gated=[]
    for n,k,g in top_items(p):
        if n=="_": continue
        if not re.search(r"^pub\s+(?:unsafe\s+)?"+k+r"\s+"+n+r"\b",t,re.M):
            print("NOT PUB",m,n); continue
        (gated if g else plain).append(n)
    snake=m[2:].replace("-","_")
    mods.append((m,snake,plain,gated))
def lst(names): 
    names=sorted(names,key=lambda s:(not s.isupper() and not s.upper()==s,s))
    return ", ".join(names)
doc="//! 🧱️ The reconcile primitives — bounded containers, admission limits and censuses, record-diff cursors and the tree-retirement cursor every surface reconciliation is built from. Pure bounded-state building blocks: no registry, no job, no renderer."
out=[doc,""]
for m,snake,plain,gated in mods:
    out.append(f'#[path = "{m}/🦀️.rs"]\nmod {snake};')
out.append('#[path = "../♻️retirement/🌲️tree/🦀️.rs"]\nmod tree_retirement;\n')
for m,snake,plain,gated in mods:
    out.append(f"pub use {snake}::{{{lst(plain)}}};")
    if gated: out.append(f"{CFG}\npub use {snake}::{{{lst(gated)}}};")
out.append("pub use tree_retirement::SurfaceTreeRetireCursor;")
open(f"{P}/🦀️.rs","w",encoding="utf-8").write("\n".join(out)+"\n")
print("\n".join(out))
