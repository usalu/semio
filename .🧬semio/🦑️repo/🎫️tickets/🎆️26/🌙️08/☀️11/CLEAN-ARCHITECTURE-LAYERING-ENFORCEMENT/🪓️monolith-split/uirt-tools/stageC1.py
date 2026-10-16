"""🔓️ Stage C1: widens the primitives definitions that the reconcile crate and the laws reference."""
import sys,re
sys.path.insert(0,".")
from widen import *
R=f"{REPO}/{RT}"
prim_mods=("🗃️containers","🚧️limits","📊️census","🧩️records")
prim=[f"{R}/🧱️primitives/{m}/🦀️.rs" for m in prim_mods]+[f"{R}/♻️retirement/🌲️tree/🦀️.rs"]
others=[f"{R}/♻️reconcile/🦀️.rs",f"{R}/📤️output/🦀️.rs",f"{R}/🩹️patch/🧪️tests/🩹️patch/🦀️.rs",f"{R}/🚪️handback/🧪️tests/🚪️handback/🦀️.rs",f"{R}/📤️output/🧪️tests/🔬️unit/🦀️.rs",
 f"{R}/🧪️tests/🔬️reconcile-unit/🦀️.rs",f"{R}/📏️ownership/🧪️tests/📏️ownership/🦀️.rs",f"{R}/📃️document/🧪️tests/📃️document/🦀️.rs",f"{R}/📤️output/🧪️tests/📤️output/🦀️.rs",f"{R}/♻️retirement/🌲️tree/🧪️tests/🌲️tree/🦀️.rs"]
oth=Others(others)
for p in prim: widen_file(p,oth,apply=True)
revert={
 f"{R}/♻️retirement/🌲️tree/🦀️.rs":["    pub owner: Option<ui_contract::BuiltTreeRetirement>,","    pub native_resources: Option<UiNativeResources>,"],
 f"{R}/🧱️primitives/📊️census/🦀️.rs":["    pub cursor: ui_contract::UiMapCursor,","    pub field: u8,","    pub depth: usize,"],
}
for p,ls in revert.items():
    t=open(p,encoding="utf-8").read()
    for l in ls:
        assert t.count(l)==1,(p,l,t.count(l))
        t=t.replace(l,l.replace("    pub ","    ",1))
    open(p,"w",encoding="utf-8").write(t)
# type closure: private top-level types named in the signature lines of pub definitions
def closure():
    changed=True
    names={}
    for p in prim:
        t=open(p,encoding="utf-8").read()
        for m in re.finditer(r"^(?:pub(?:\([^)]*\))?\s+)?(struct|enum|type)\s+([A-Za-z_]\w*)",t,re.M):
            names.setdefault(m.group(2),p)
    while changed:
        changed=False
        pubtext="\n".join(l for p in prim for l in open(p,encoding="utf-8").read().split("\n") if l.lstrip().startswith("pub "))
        for n,p in names.items():
            t=open(p,encoding="utf-8").read()
            m=re.search(r"^(?:pub\([^)]*\)\s+)?(struct|enum|type)\s+"+n+r"\b",t,re.M)
            if m and not m.group(0).startswith("pub ") and re.search(r"(?<![A-Za-z0-9_])"+n+r"(?![A-Za-z0-9_])",pubtext):
                t=t[:m.start()]+"pub "+re.sub(r"^pub\([^)]*\)\s+","",m.group(0))+t[m.end():]
                open(p,"w",encoding="utf-8").write(t); changed=True; print("closure widened",n)
closure()
