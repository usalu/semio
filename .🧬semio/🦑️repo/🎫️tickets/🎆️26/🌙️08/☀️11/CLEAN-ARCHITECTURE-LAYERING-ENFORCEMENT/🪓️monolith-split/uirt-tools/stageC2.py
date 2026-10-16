"""🧱️ Stage C2: cfg gating, sibling imports and the primitives lib root."""
import re,sys
sys.path.insert(0,".")
from prim_names import REPO,RT,MODS,top_items
R=f"{REPO}/{RT}"
P=f"{R}/🧱️primitives"
CFG='#[cfg(any(test, feature = "testkit"))]'
# 1. gating + the two manual widenings
for m in MODS:
    p=f"{P}/{m}/🦀️.rs"; t=open(p,encoding="utf-8").read()
    t=t.replace("#[cfg(test)]",CFG)
    if m=="📊️census":
        assert t.count("\nstruct SurfaceSemanticMapPage")==1
        t=t.replace("\nstruct SurfaceSemanticMapPage","\npub struct SurfaceSemanticMapPage")
        t=t.replace("    pub cursor: ui_contract::UiMapCursor","    cursor: ui_contract::UiMapCursor")
    if m=="🚧️limits":
        t=t.replace("\nconst SURFACE_RECONCILE_SEMANTIC_COPIES","\npub const SURFACE_RECONCILE_SEMANTIC_COPIES")
    open(p,"w",encoding="utf-8").write(t)
# 2. definitions per module
defs={}
for m in MODS:
    for n,k,g in top_items(f"{P}/{m}/🦀️.rs"): defs.setdefault(m,[]).append((n,k,g))
tree=f"{R}/♻️retirement/🌲️tree/🦀️.rs"
t=open(tree,encoding="utf-8").read(); t=t.replace("#[cfg(test)]",CFG); open(tree,"w",encoding="utf-8").write(t)
defs["tree_retirement"]=[("SurfaceTreeRetireCursor","struct",False)]
def strip(t):
    from widen import strip_comments
    return strip_comments(t)
# 3. sibling imports
for m in MODS:
    p=f"{P}/{m}/🦀️.rs"; t=open(p,encoding="utf-8").read(); body=strip(t)
    need=[];needg=[]
    for other,items in defs.items():
        if other==m or other=="tree_retirement": continue
        for n,k,g in items:
            if n=="_": continue
            if re.search(r"(?<![A-Za-z0-9_])"+re.escape(n)+r"(?![A-Za-z0-9_])",body): (needg if g else need).append(n)
    lines=[]
    if need: lines.append("use crate::{"+", ".join(sorted(need))+"};")
    if needg: lines.append(CFG+"\nuse crate::{"+", ".join(sorted(needg))+"};")
    if lines:
        # insert after the last leading `use` line of the header block (before first blank line following uses) -> after docs
        idx=t.index("\n\n")+2
        # skip over existing use block
        rest=t[idx:]
        m2=re.match(r"((?:(?:#\[cfg[^\n]*\n)?use [^\n]*\n)+)\n",rest)
        if m2: t=t[:idx]+m2.group(1)+"\n".join(lines)+"\n\n"+rest[m2.end():]
        else: t=t[:idx]+"\n".join(lines)+"\n\n"+rest
        open(p,"w",encoding="utf-8").write(t)
    print(m,"imports",need,needg)
