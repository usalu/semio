"""🔎️ Top-level item names defined by the primitives crate files (with cfg-gated flag)."""
import re
REPO="/Users/ueli/Documents/semio"
RT="🧰️framework/🔨️modules/🖱️ui/🧠️runtime"
MODS=["🗃️containers","🚧️limits","📊️census","🧩️records"]
def top_items(path):
    lines=open(path,encoding="utf-8").read().split("\n")
    out=[];gated=False
    for i,l in enumerate(lines):
        if l.startswith("#[cfg("): gated=("test" in l); continue
        if l.startswith("use "): gated=False; continue
        m=re.match(r"^(?:pub(?:\([^)]*\))?\s+)?(?:unsafe\s+)?(fn|struct|enum|const|static|type|trait)\s+([A-Za-z_][A-Za-z0-9_]*)",l)
        if m and m.group(2)!="_":
            out.append((m.group(2),m.group(1),gated)); gated=False
        elif l and not l.startswith(("///","//","#[","}","use ","impl")) and not l.startswith(" "):
            gated=False
    return out
def prim_items():
    res={}
    for mod in MODS:
        for n,k,g in top_items(f"{REPO}/{RT}/🧱️primitives/{mod}/🦀️.rs"): res[n]=(mod,k,g)
    res["SurfaceTreeRetireCursor"]=("tree_retirement","struct",False)
    return res
if __name__=="__main__":
    for n,(m,k,g) in prim_items().items(): print(m,k,n,"GATED" if g else "")
